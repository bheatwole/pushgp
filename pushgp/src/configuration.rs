use crate::{Code, GeneticOperation, LiteralEnum, LiteralEnumHasLiteralValue, Name};
use fnv::FnvHashMap;
use rand::{prelude::SliceRandom, rngs::SmallRng, Rng, SeedableRng};

/// A Configuration is a Vec of u8 where each u8 represents the weight of one of the possible randomly generated items
/// for Code<L>. The first u8 is the likelihood of picking an already defined name. The next set of u8s is the chance of
/// picking a new random literal value. The last set of u8s is the chance of picking each of the instructions.  Any
/// weight set to zero means the random code generator will not pick that item. An item with a weight of '2' is twice as
/// likely to be picked as an item with a weight of '1'.
///
/// A Vec<u8> is used to allow for running an island where the random code generator itself is optimized by genetic
/// programming. Crossover, mutation, etc are applied to the Configurations, new populations are generated and run for
/// a few generations on the main island. The Configuration that produces the most fit population is the winner.
#[derive(Debug, PartialEq)]
pub struct Configuration<L: LiteralEnum<L>> {
    rng: SmallRng,
    max_points_in_random_expressions: usize,

    crossover_rate: u8,
    mutation_rate: u8,

    defined_name_weight: usize,

    ephemeral_total: usize,
    ephemeral_weights: Vec<EphemeralEntry<L>>,
    ephemeral_by_type: FnvHashMap<String, LiteralConstructor<L>>,

    instruction_total: usize,
    instruction_weights: Vec<InstructionEntry>,
}

#[derive(Clone)]
pub struct LiteralConstructor<L>(pub fn(&mut SmallRng) -> L);

impl<L> std::fmt::Debug for LiteralConstructor<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LiteralConstructor(0x{:x})", self.0 as usize)
    }
}

impl<L> PartialEq for LiteralConstructor<L> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 as usize == rhs.0 as usize
    }
}

pub type RandomLiteralFunction<RealLiteralType> = fn(rng: &mut SmallRng) -> RealLiteralType;

#[derive(Debug, PartialEq)]
struct EphemeralEntry<L: LiteralEnum<L>> {
    pub weight: usize,
    pub call: LiteralConstructor<L>,
}

#[derive(Debug, PartialEq)]
struct InstructionEntry {
    pub weight: usize,
    pub instruction: String,
}

pub trait EphemeralConfiguration<L: LiteralEnum<L>> {
    fn get_all_literal_types() -> Vec<String>;
    fn make_literal_constructor_for_type(literal_type: &str) -> LiteralConstructor<L>;
}

pub trait InstructionConfiguration {
    fn get_all_instructions() -> Vec<String>;
}

impl<
        L: LiteralEnum<L> + EphemeralConfiguration<L> + InstructionConfiguration + LiteralEnumHasLiteralValue<L, Name>,
    > Configuration<L>
{
    pub fn new(rng_seed: Option<u64>, max_points_in_random_expressions: usize, weights: &[u8]) -> Configuration<L> {
        let (crossover_rate, weights) = pop_front_weight_or_one_and_return_rest(weights);
        let (mutation_rate, weights) = pop_front_weight_or_one_and_return_rest(weights);
        let (defined_name_weight, weights) = pop_front_weight_or_one_and_return_rest(weights);
        let (ephemeral_total, ephemeral_weights, ephemeral_by_type, weights) = Self::make_ephemeral_weights(weights);
        let (instruction_total, instruction_weights, _) = Self::make_instruction_weights(weights);

        Configuration {
            rng: small_rng_from_optional_seed(rng_seed),
            max_points_in_random_expressions,
            crossover_rate,
            mutation_rate,
            defined_name_weight: defined_name_weight as usize,
            ephemeral_total,
            ephemeral_weights,
            ephemeral_by_type,
            instruction_total,
            instruction_weights,
        }
    }

    fn make_ephemeral_weights(mut weights: &[u8]) -> (usize, Vec<EphemeralEntry<L>>, FnvHashMap<String, LiteralConstructor<L>>, &[u8]) {
        let mut ephemeral_types = L::get_all_literal_types();
        let mut total = 0;
        let mut entries = vec![];
        let mut mapped = FnvHashMap::default();

        for literal_type in ephemeral_types.drain(..) {
            let (literal_weight, rest_weights) = pop_front_weight_or_one_and_return_rest(weights);
            weights = rest_weights;

            total += literal_weight as usize;
            let call = L::make_literal_constructor_for_type(literal_type.as_str());
            entries.push(EphemeralEntry {
                weight: total,
                call: call.clone(),
            });
            mapped.insert(literal_type, call);
        }

        (total, entries, mapped, weights)
    }

    fn make_instruction_weights(mut weights: &[u8]) -> (usize, Vec<InstructionEntry>, &[u8]) {
        let mut instructions = L::get_all_instructions();
        let mut total = 0;
        let mut entries = vec![];

        for instruction in instructions.drain(..) {
            let (instruction_weight, rest_weights) = pop_front_weight_or_one_and_return_rest(weights);
            weights = rest_weights;

            total += instruction_weight as usize;
            entries.push(InstructionEntry { weight: total, instruction: instruction });
        }

        (total, entries, weights)
    }

    /// Returns a list of all instructions that have a weight > 0
    pub fn allowed_instructions(&self) -> Vec<String> {
        self.instruction_weights
            .iter()
            .filter(|entry| entry.weight != 0)
            .map(|entry| entry.instruction.clone())
            .collect()
    }

    /// Seeds the random number with a specific value so that you may get repeatable results. Passing `None` will seed
    /// the generator with a truly random value ensuring unique results.
    pub fn set_seed(&mut self, seed: Option<u64>) {
        self.rng = small_rng_from_optional_seed(seed);
    }

    pub fn run_random_literal_function<RealLiteralType>(&mut self, func: RandomLiteralFunction<RealLiteralType>) -> RealLiteralType {
        func(&mut self.rng)
    }

    /// Returns a random value of the type requested or None if the configuration does not know how to construct a
    /// random value of that type
    pub fn random_literal_of_type(&mut self, literal_type: &String) -> Option<Code<L>> {
        if let Some(constructor) = self.ephemeral_by_type.get(literal_type) {
            Some(Code::Literal(constructor.0(&mut self.rng)))
        } else {
            None
        }
    }

    /// Returns a random genetic operation
    pub fn random_genetic_operation(&mut self) -> GeneticOperation {
        let total: usize = self.mutation_rate as usize + self.crossover_rate as usize;
        let pick = self.rng.gen_range(0..total);

        if pick < self.mutation_rate as usize {
            GeneticOperation::Mutation
        } else {
            GeneticOperation::Crossover
        }
    }

    /// Returns one random atom
    pub fn random_atom(&mut self, defined_names: &[String]) -> Code<L> {
        // Determine how many total possibilities there are. This shifts depending upon how many defined_names we have.
        let defined_names_total = if <L as LiteralEnumHasLiteralValue<L, Name>>::supports_literal_type() {
            self.defined_name_weight * defined_names.len()
        } else {
            0
        };
        let random_total = defined_names_total + self.ephemeral_total + self.instruction_total;

        // Pick one
        let mut pick = self.rng.gen_range(0..random_total);

        // Is it a defined name?
        if pick < defined_names_total {
            return self.random_defined_name(defined_names);
        }
        pick -= defined_names_total;

        // Is it a new literal value?
        if pick < self.ephemeral_total {
            return self.random_literal_from_ephemeral(pick);
        }
        pick -= self.ephemeral_total;

        // Must be an instruction
        self.random_instruction(pick)
    }

    /// Returns one random defined name
    pub fn random_defined_name(&mut self, defined_names: &[Name]) -> Code<L> {
        let pick = self.rng.gen_range(0..defined_names.len());
        Code::Literal(<L as LiteralEnumHasLiteralValue<L, Name>>::make_from_value(defined_names[pick].clone()))
    }

    /// Returns a new random literal based upon one of the ephemeral random constructors
    pub fn random_literal_from_ephemeral(&mut self, pick: usize) -> Code<L> {
        let index = self.ephemeral_weights.partition_point(|entry| entry.weight < pick);
        let entry = self.ephemeral_weights.get(index).unwrap();
        Code::Literal((entry.call.0)(&mut self.rng))
    }

    /// Returns a new random instruction
    pub fn random_instruction(&mut self, pick: usize) -> Code<L> {
        let index = self.instruction_weights.partition_point(|entry| entry.weight < pick);
        let entry = self.instruction_weights.get(index).unwrap();
        Code::Instruction(entry.instruction.clone())
    }

    /// Generates some random code using the context parameters for how often random bool, ints, floats and names are
    /// chosen. You may also pass in pre-defined names that could be selected randomly as well. The weights table for
    /// all instructions will be considered as well.
    ///
    /// The generated code will have at least one code point and as many as `self.max_points_in_random_expressions`.
    /// The generated code will be in a general tree-like shape using lists of lists as the trunks and individual
    /// atoms as the leaves. The shape is neither balanced nor linear, but somewhat in between.
    pub fn generate_random_code(&mut self, defined_names: &[Name]) -> Code<L> {
        let actual_points = self.rng.gen_range(1..=self.max_points_in_random_expressions);
        self.random_code_with_size(actual_points, defined_names)
    }

    fn random_code_with_size(&mut self, points: usize, defined_names: &[Name]) -> Code<L> {
        if 1 == points {
            // We need a leaf, so pick one of the atoms
            self.random_atom(defined_names)
        } else {
            // Break this level down into a list of lists, or possibly specific leaf atoms.
            let mut sizes_this_level = self.decompose(points - 1, points - 1);
            sizes_this_level.shuffle(&mut self.rng);
            let mut list = vec![];
            for size in sizes_this_level {
                list.push(self.random_code_with_size(size, defined_names));
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

fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}

fn pop_front_weight_or_one_and_return_rest(weights: &[u8]) -> (u8, &[u8]) {
    if weights.len() == 0 {
        (1, weights)
    } else {
        (weights[0], &weights[1..])
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_partition_point_function() {
        // Both the ephemeral entries and instruction entries table depend upon the following behavior from
        // partition_point. If it ever stops working like this, we need to know. Specifically only the first of a series
        // of identical values is ever returned
        let entries = [1, 5, 5, 5, 10];
        assert_eq!(0, entries.partition_point(|&x| x < 1));
        assert_eq!(1, entries.partition_point(|&x| x < 2));
        assert_eq!(1, entries.partition_point(|&x| x < 3));
        assert_eq!(1, entries.partition_point(|&x| x < 4));
        assert_eq!(1, entries.partition_point(|&x| x < 5));
        assert_eq!(4, entries.partition_point(|&x| x < 6));
    }
}
