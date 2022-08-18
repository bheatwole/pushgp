use fnv::FnvHashMap;
use rand::{
    rngs::SmallRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};

use crate::*;

#[derive(Debug, PartialEq)]
pub struct VirtualMachineEngine<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> {
    rng: SmallRng,
    exec_stack: Stack<Exec<Vm>>,
    parser: Parser<Vm>,
    config: Configuration,
    weights: InstructionWeights<Vm>,
    defined_names: FnvHashMap<Name, Code<Vm>>,
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> VirtualMachineEngine<Vm> {
    pub fn new(seed: Option<u64>, config: Configuration) -> VirtualMachineEngine<Vm> {
        let vm = VirtualMachineEngine {
            rng: small_rng_from_optional_seed(seed),
            exec_stack: Stack::new(),
            parser: Parser::new(),
            config,
            weights: InstructionWeights::new(),
            defined_names: FnvHashMap::default(),
        };

        vm
    }

    pub fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        &mut self.rng
    }

    pub fn set_rng_seed(&mut self, seed: Option<u64>) {
        self.rng = small_rng_from_optional_seed(seed);
    }

    pub fn exec(&mut self) -> &mut Stack<Code<Vm>> {
        &mut self.exec_stack
    }

    pub fn get_weights(&self) -> &InstructionWeights<Vm> {
        &self.weights
    }

    pub fn clear(&mut self) {
        self.exec_stack.clear();
        self.defined_names.clear();
    }

    pub fn add_instruction<C: StaticInstruction<Vm>>(&mut self) {
        self.parser.add_instruction::<C>();
        self.weights.add_instruction::<C>(self.config.get_instruction_weight(C::static_name()));
    }

    pub fn get_configuration(&self) -> &Configuration {
        &self.config
    }

    pub fn reset_configuration(&mut self, config: Configuration) {
        self.config = config;

        // Iterate through all instruction names and re-assign the weights for the instructions
        self.weights.reset_weights_from_configuration(&self.config);
    }

    pub fn get_instruction_weights(&self) -> &InstructionWeights<Vm> {
        &self.weights
    }

    /// Creates a new random instruction
    fn generate_random_instruction(&mut self) -> Code<Vm> {
        let generator = self.weights.pick_random_instruction_generator(&mut self.rng);
        generator(self)
    }

    pub fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code<Vm>> {
        self.parser.parse(input)
    }

    pub fn must_parse<'a>(&self, input: &'a str) -> Code<Vm> {
        let (rest, code) = self.parse(input).unwrap();
        assert_eq!(rest.len(), 0);
        code
    }

    pub fn parse_and_set_code(&mut self, input: &str) -> Result<(), ParseError> {
        self.clear();
        let (rest, code) = self.parse(input).map_err(|e| ParseError::new(e))?;
        if rest.len() == 0 {
            self.exec_stack.push(code);
            Ok(())
        } else {
            return Err(ParseError::new_with_message("the code did not finish parsing"));
        }
    }

    pub fn set_code(&mut self, code: Code<Vm>) {
        self.clear();
        self.exec_stack.push(code);
    }

    /// Returns the code for the specified name, or None if the name is not defined
    pub fn definition_for_name(&self, name: &String) -> Option<Code<Vm>> {
        self.defined_names.get(name).map(|c| c.clone())
    }

    /// Returns a list of all the names that are defined
    pub fn all_defined_names(&self) -> Vec<String> {
        self.defined_names.keys().map(|k| k.clone()).collect()
    }

    /// Returns one random defined name, or None if there are no defined names
    pub fn random_defined_name(&mut self) -> Option<Code<Vm>> {
        if 0 == self.defined_names.len() {
            return None;
        }

        Some(self.defined_names.values().choose(&mut self.rng).unwrap().clone())
    }

    /// Randomly selects either a crossover or mutation as the genetic operation to perform.
    pub fn select_genetic_operation(&mut self) -> GeneticOperation {
        let mutation_rate = self.config.get_mutation_rate() as usize;
        let total = self.config.get_crossover_rate() as usize + mutation_rate;
        let pick = self.rng.gen_range(0..total);
        if pick < mutation_rate as usize {
            GeneticOperation::Mutation
        } else {
            GeneticOperation::Crossover
        }
    }

    /// Creates a newly-generated random chunk of code. The limit for the size of the expression is taken is the points
    /// parameters; to ensure that it is in the appropriate range this is taken modulo the value of the
    /// MAX-POINTS-IN-RANDOM-EXPRESSIONS parameter and the absolute value of the result is used.
    pub fn rand_code(&mut self, points: Option<usize>) -> Code<Vm> {
        let shape = self.generate_random_code_shape(points);
        self.fill_code_shape(shape)
    }

    /// Produces a random child of the two individuals that is either a mutation of the left individual, or the genetic
    /// crossover of both.
    ///
    /// The defined_names of the child will only include the code that is specifically named in the child's code. If
    /// both parents have the same defined_name, the value for that will come from the left individual.
    pub fn rand_child<RunResult: std::fmt::Debug + Clone>(
        &mut self,
        left: &Individual<RunResult, Vm>,
        right: &Individual<RunResult, Vm>,
    ) -> Individual<RunResult, Vm> {
        match self.select_genetic_operation() {
            GeneticOperation::Mutation => self.mutate(left),
            GeneticOperation::Crossover => self.crossover(left, right),
        }
    }

    /// Mutates the parent by randomly selecting a point in the code, generating a new random code item of the same
    /// size, and replacing the selected point with the new code.
    ///
    /// The defined_names of the child will only include the code that is specifically named in the child's code.
    pub fn mutate<RunResult: std::fmt::Debug + Clone>(
        &mut self,
        parent: &Individual<RunResult, Vm>,
    ) -> Individual<RunResult, Vm> {
        let (selected_point, replace_shape) = self.select_operation_point_and_shape(parent.get_code());
        let replacement_code = self.fill_code_shape(replace_shape);
        let (child_code, _) = parent.get_code().replace_point(selected_point, replacement_code.as_ref());

        // TODO: Ensure the individuals defined_names are correct

        Individual::new(child_code, FnvHashMap::default(), None)
    }

    /// Produces a random child that is a crossover of both parents. A random point from the left tree will be selected
    /// and child create that has the selected point from that parent replaced with the code tree of a selected point of
    /// the right parent.
    ///
    /// The defined_names of the child will only include the code that is specifically named in the child's code. If
    /// both parents have the same defined_name, the value for that will come from the left individual.
    pub fn crossover<RunResult: std::fmt::Debug + Clone>(
        &mut self,
        left: &Individual<RunResult, Vm>,
        right: &Individual<RunResult, Vm>,
    ) -> Individual<RunResult, Vm> {
        let left_selected_point = self.select_random_point(left.get_code());
        let left_code = extract_known_point(left.get_code(), left_selected_point);
        let right_selected_point = self.select_random_point(right.get_code());

        let (child_code, _) = right.get_code().replace_point(right_selected_point, left_code.as_ref());

        // TODO: Ensure the individuals defined_names are correct

        Individual::new(child_code, FnvHashMap::default(), None)
    }

    fn select_random_point(&mut self, code: &Code<Vm>) -> i64 {
        let total_points = code.points();
        self.rng.gen_range(0..total_points)
    }

    fn select_operation_point_and_shape(&mut self, parent: &Code<Vm>) -> (i64, CodeShape) {
        let selected_point = self.select_random_point(parent);
        let replace_size = match parent.extract_point(selected_point) {
            Extraction::Used(_) => 1,
            Extraction::Extracted(sub) => sub.points(),
        };
        let replace_shape = self.random_code_shape_with_size(replace_size as usize);

        (selected_point, replace_shape)
    }

    // Returns one random atom
    fn fill_code_shape(&mut self, shape: CodeShape) -> Code<Vm> {
        match shape {
            CodeShape::Atom => {
                // Determine how many total possibilities there are. This shifts depending upon how many defined_names we have.
                let defined_names_total = if Vm::HAS_NAME {
                    self.defined_names.len() * self.config.get_defined_name_weight() as usize
                } else {
                    0
                };
                let random_total = defined_names_total + self.weights.get_sum_of_weights();

                // Pick one
                let pick = self.rng.gen_range(0..random_total);

                // Is it a defined name? For VMs that do not use the name stack, this always be zero
                if pick < defined_names_total {
                    self.random_defined_name().unwrap()
                } else {
                    // Must be an instruction
                    self.generate_random_instruction()
                }
            }
            CodeShape::List(mut list) => {
                let mut code = vec![];
                for s in list.drain(..) {
                    code.push(self.fill_code_shape(s));
                }
                Box::new(PushList::new(code))
            }
        }
    }

    // The generated shape will have at least one code point and as many as `self.max_points_in_random_expressions`.
    // The generated shape will be in a general tree-like using lists of lists as the trunks and individual atoms as
    // the leaves. The shape is neither balanced nor linear, but somewhat in between.
    fn generate_random_code_shape(&mut self, points: Option<usize>) -> CodeShape {
        let max_points = if let Some(maybe_huge_max) = points {
            let max = maybe_huge_max % self.config.get_max_points_in_random_expressions();
            if max > 0 {
                max
            } else {
                1
            }
        } else {
            self.config.get_max_points_in_random_expressions()
        };
        let actual_points = self.rng.gen_range(1..=max_points);
        self.random_code_shape_with_size(actual_points)
    }

    fn random_code_shape_with_size(&mut self, points: usize) -> CodeShape {
        if 1 == points {
            CodeShape::Atom
        } else {
            // Break this level down into a list of lists, or possibly specific leaf atoms.
            let mut sizes_this_level = self.decompose(points - 1, points - 1);
            {
                sizes_this_level.shuffle(&mut self.rng);
            }
            let mut list = vec![];
            for size in sizes_this_level {
                list.push(self.random_code_shape_with_size(size));
            }
            CodeShape::List(list)
        }
    }

    fn decompose(&mut self, number: usize, max_parts: usize) -> Vec<usize> {
        if 1 == number || 1 == max_parts {
            return vec![1];
        }
        let this_part = self.rng.gen_range(1..=(number - 1));
        let mut result = vec![this_part];
        result.extend_from_slice(&self.decompose(number - this_part, max_parts - 1));
        result
    }
}

fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}

// Returns the sub-tree of code from a larger piece of code where 'point' is known to be less than `code.points()`
fn extract_known_point<Vm: VirtualMachine>(code: &Code<Vm>, point: i64) -> Code<Vm> {
    match code.extract_point(point) {
        Extraction::Used(_) => {
            panic!("do not call extract_known_point unless point is known to be less than code.points()")
        }
        Extraction::Extracted(sub) => sub,
    }
}

#[derive(Clone, Debug)]
enum CodeShape {
    Atom,
    List(Vec<CodeShape>),
}