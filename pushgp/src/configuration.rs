use crate::{Code, Instruction, InstructionType, RandomType};
use fnv::FnvHashMap;
use rand::{prelude::SliceRandom, rngs::SmallRng, Rng, SeedableRng};
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[derive(Debug, PartialEq)]
pub struct Configuration {
    instruction_weights: FnvHashMap<Instruction, u8>,
    ephemeral_bool_weight: u8,
    ephemeral_float_weight: u8,
    min_random_float: f64,
    max_random_float: f64,
    ephemeral_int_weight: u8,
    min_random_int: i64,
    max_random_int: i64,
    ephemeral_name_weight: u8,
    defined_name_weight: u8,
    random_seed: Option<u64>,
    max_points_in_random_expressions: usize,
    runtime_instruction_limit: usize,
    rng: SmallRng,
    types: Vec<RandomType>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            instruction_weights: FnvHashMap::default(),
            ephemeral_bool_weight: 1,
            ephemeral_float_weight: 1,
            ephemeral_int_weight: 1,
            ephemeral_name_weight: 1,
            defined_name_weight: 1,
            random_seed: None,
            runtime_instruction_limit: 10000,
            rng: SmallRng::from_entropy(),
            types: vec![],
            min_random_float: -1.0,
            max_random_float: 1.0,
            min_random_int: std::i64::MIN,
            max_random_int: std::i64::MAX,
            max_points_in_random_expressions: 1000,
        }
    }

    /// Sets to zero the weights of all instructions that use the specified type as an operand or as a destination. This
    /// will prevent that type of instruction from appearing in randomly generated code.
    /// 
    /// For example if you didn't want your program to define any variables, you could call
    /// context.disallow_type(InstructionType::Name)
    pub fn disallow_type(&mut self, disallowed: InstructionType) {
        // Loop through all instructions and set the weight to zero for any that use the specified type
        for (inst, weight) in self.instruction_weights.iter_mut() {
            if inst.types().contains(&disallowed) {
                *weight = 0;
                self.types.clear();
            }
        }
    }

    /// Sets the weight of the specified instruction. Set to zero to prevent the instruction from being used in random
    /// code generation. The weight will be the relative likelihood of the instruction being selected.
    pub fn set_instruction_weight(&mut self, allow: Instruction, weight: u8) {
        self.instruction_weights.insert(allow, weight);
        self.types.clear();
    }

    /// Returns a list of all instructions that have a weight > 0
    pub fn allowed_instructions(&self) -> Vec<Instruction> {
        self.instruction_weights.iter().filter(|pair| *pair.1 != 0).map(|pair| *pair.0).collect()
    }

    /// Seeds the random number with a specific value so that you may get repeatable results. Passing `None` will seed
    /// the generator with a truly random value ensuring unique results.
    pub fn set_seed(&mut self, seed: Option<u64>) {
        self.random_seed = seed;
        self.rng = if let Some(seed) = seed { SmallRng::seed_from_u64(seed) } else { SmallRng::from_entropy() }
    }

    fn append_type(&mut self, rand_type: RandomType, weight: u8) {
        for _ in 0..weight {
            self.types.push(rand_type);
        }
    }

    /// Returns a random boolean value
    pub fn random_bool(&mut self) -> bool {
        if 0 == self.rng.gen_range(0..=1) {
            false
        } else {
            true
        }
    }

    /// Returns a random float value in the range (context.min_random_float..context.max_random_float)
    pub fn random_float(&mut self) -> Decimal {
        let float: f64 = self.rng.gen_range(self.min_random_float..self.max_random_float);
        Decimal::from_f64(float).unwrap()
    }

    /// Returns a random int value in the range (context.min_random_int..context.max_random_int)
    pub fn random_int(&mut self) -> i64 {
        self.rng.gen_range(self.min_random_int..=self.max_random_int)
    }

    /// Returns a random int value using the range passed int
    pub fn random_int_in_range(&mut self, range: std::ops::Range<i64>) -> i64 {
        self.rng.gen_range(range)
    }

    /// Returns a random name
    pub fn random_name(&mut self) -> u64 {
        self.rng.gen_range(0..=u64::MAX)
    }

    /// Generates some random code using the context parameters for how often random bool, ints, floats and names are
    /// choosen. You may also pass in pre-defined names that could be selected randomly as well. The weights table for
    /// all instructions will be considered as well.
    /// 
    /// The generated code will have at least one code point and as many as `context.max_points_in_random_expressions`.
    /// The generated code will be in a general tree-like shape using lists of lists as the trunks and individual
    /// atoms as the leaves. The shape is neither balanced nor linear, but somewhat in between.
    pub fn generate_random_code(&mut self, defined_names: &[u64]) -> Code {
        if 0 == self.types.len() {
            // Define the ephemal constants types
            self.append_type(RandomType::EphemeralBool, self.ephemeral_bool_weight);
            self.append_type(RandomType::EphemeralFloat, self.ephemeral_float_weight);
            self.append_type(RandomType::EphemeralInt, self.ephemeral_int_weight);
            self.append_type(RandomType::EphemeralName, self.ephemeral_name_weight);

            // Define all the names we know about
            for &name in defined_names {
                self.append_type(RandomType::DefinedName(name), self.defined_name_weight);
            }

            // Define all the instructions we know about. We need to copy the weights for immutable/mutable borrowing
            let weights: Vec<(Instruction, u8)> =
                self.instruction_weights.iter().map(|(&key, &weight)| (key, weight)).collect();
            for (inst, weight) in weights {
                self.append_type(RandomType::Instruction(inst), weight);
            }
        }

        // Generate some code!
        self.generate()
    }

    fn generate(&mut self) -> Code {
        let actual_points = self.rng.gen_range(1..=self.max_points_in_random_expressions);
        self.random_code_with_size(actual_points)
    }

    fn random_code_with_size(&mut self, points: usize) -> Code {
        if 1 == points {
            // We need a leaf, so pick one of the atoms
            let index = self.rng.gen_range(0..self.types.len());
            match self.types.get(index).unwrap() {
                RandomType::EphemeralBool => Code::LiteralBool(self.random_bool()),
                RandomType::EphemeralFloat => Code::LiteralFloat(self.random_float()),
                RandomType::EphemeralInt => Code::LiteralInteger(self.random_int()),
                RandomType::EphemeralName => Code::LiteralName(self.random_name()),
                RandomType::DefinedName(name) => Code::LiteralName(*name),
                RandomType::Instruction(inst) => Code::Instruction(*inst),
            }
        } else {
            // Break this level down into a list of lists, or possibly specific leaf atoms.
            let mut sizes_this_level = self.decompose(points - 1, points - 1);
            sizes_this_level.shuffle(&mut self.rng);
            let mut list = vec![];
            for size in sizes_this_level {
                list.push(self.random_code_with_size(size));
            }
            Code::List(list)
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
