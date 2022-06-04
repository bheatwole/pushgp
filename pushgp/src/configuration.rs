use crate::{Code, GeneticOperation, Context, VirtualTable};
use rand::{prelude::SliceRandom, rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;
use std::ops::DerefMut;

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
pub struct Configuration {
    rng: RefCell<SmallRng>,
    max_points_in_random_expressions: usize,

    crossover_rate: u8,
    mutation_rate: u8,

    defined_name_weight: usize,

    instruction_total: usize,
    instruction_weights: Vec<InstructionEntry>,
}

#[derive(Debug, PartialEq)]
struct InstructionEntry {
    pub weight: usize,
    pub instruction: usize,
}

impl Configuration {
    pub fn new(
        rng_seed: Option<u64>,
        max_points_in_random_expressions: usize,
        virtual_table: &VirtualTable,
        weights: &[u8],
    ) -> Configuration {
        let (crossover_rate, weights) = pop_front_weight_or_one_and_return_rest(weights);
        let (mutation_rate, weights) = pop_front_weight_or_one_and_return_rest(weights);
        let (defined_name_weight, weights) = pop_front_weight_or_one_and_return_rest(weights);
        let (instruction_total, instruction_weights, _) = Self::make_instruction_weights(virtual_table, weights);

        Configuration {
            rng: RefCell::new(small_rng_from_optional_seed(rng_seed)),
            max_points_in_random_expressions,
            crossover_rate,
            mutation_rate,
            defined_name_weight: defined_name_weight as usize,
            instruction_total,
            instruction_weights,
        }
    }

    fn make_instruction_weights<'a, 'b>(
        virtual_table: &'a VirtualTable,
        mut weights: &'b [u8],
    ) -> (usize, Vec<InstructionEntry>, &'b [u8]) {
        let mut total = 0;
        let mut entries = vec![];

        for id in 0..virtual_table.len() {
            let (instruction_weight, rest_weights) = pop_front_weight_or_one_and_return_rest(weights);
            weights = rest_weights;

            total += instruction_weight as usize;
            entries.push(InstructionEntry { weight: total, instruction: id });
        }

        (total, entries, weights)
    }

    /// Seeds the random number with a specific value so that you may get repeatable results. Passing `None` will seed
    /// the generator with a truly random value ensuring unique results.
    pub fn set_seed(&self, seed: Option<u64>) {
        self.rng.replace(small_rng_from_optional_seed(seed));
    }

    pub fn run_random_literal_function<F, RealLiteralType>(&self, func: F) -> RealLiteralType
    where
        F: Fn(&mut SmallRng) -> RealLiteralType,
    {
        let mut rng = self.rng.borrow_mut();
        func(rng.deref_mut())
    }

    /// Returns a list of all instructions that have a weight > 0
    pub fn allowed_instructions(&self) -> Vec<usize> {
        self.instruction_weights
            .iter()
            .filter(|entry| entry.weight != 0)
            .map(|entry| entry.instruction.clone())
            .collect()
    }

    /// Returns a random genetic operation
    pub fn random_genetic_operation(&self) -> GeneticOperation {
        let total: usize = self.mutation_rate as usize + self.crossover_rate as usize;
        let pick = self.rng.borrow_mut().gen_range(0..total);

        if pick < self.mutation_rate as usize {
            GeneticOperation::Mutation
        } else {
            GeneticOperation::Crossover
        }
    }

    /// Returns one random atom
    pub fn random_atom(&self, context: &Context) -> Code {
        // Determine how many total possibilities there are. This shifts depending upon how many defined_names we have.
        let defined_names_total = context.defined_names_len();
        let random_total = defined_names_total + self.instruction_total;

        // Pick one
        let mut pick = self.rng.borrow_mut().gen_range(0..random_total);

        // Is it a defined name?
        if pick < defined_names_total {
            return self.random_defined_name(context);
        }
        pick -= defined_names_total;

        // Must be an instruction
        self.random_instruction(context, pick)
    }

    /// Returns one random defined name
    pub fn random_defined_name(&self, context: &Context) -> Code {
        let defined_names = context.all_names();
        let pick = self.rng.borrow_mut().gen_range(0..defined_names.len());
        context.definition_for_name(&defined_names[pick]).unwrap()
    }

    /// Returns a new random instruction
    pub fn random_instruction(&self, context: &Context, pick: usize) -> Code {
        let index = self.instruction_weights.partition_point(|entry| entry.weight < pick);
        let mut rng = self.rng.borrow_mut();
        let data = context.get_virtual_table().call_random_value(index, rng.deref_mut());
        Code::InstructionWithData(index, data)
    }

    /// Generates some random code using the context parameters for how often random bool, ints, floats and names are
    /// chosen. You may also pass in pre-defined names that could be selected randomly as well. The weights table for
    /// all instructions will be considered as well.
    ///
    /// The generated code will have at least one code point and as many as `self.max_points_in_random_expressions`.
    /// The generated code will be in a general tree-like shape using lists of lists as the trunks and individual
    /// atoms as the leaves. The shape is neither balanced nor linear, but somewhat in between.
    pub fn generate_random_code(&self, points: Option<usize>, context: &Context) -> Code {
        let max_points = if let Some(maybe_huge_max) = points {
            let max = maybe_huge_max % self.max_points_in_random_expressions;
            if max > 0 {
                max
            } else {
                1
            }
        } else {
            self.max_points_in_random_expressions
        };
        let actual_points = self.rng.borrow_mut().gen_range(1..=max_points);
        self.random_code_with_size(actual_points, context)
    }

    fn random_code_with_size(&self, points: usize, context: &Context) -> Code {
        if 1 == points {
            // We need a leaf, so pick one of the atoms
            self.random_atom(context)
        } else {
            // Break this level down into a list of lists, or possibly specific leaf atoms.
            let mut sizes_this_level = self.decompose(points - 1, points - 1);
            {
                let mut rng = self.rng.borrow_mut();
                sizes_this_level.shuffle(rng.deref_mut());
            }
            let mut list = vec![];
            for size in sizes_this_level {
                list.push(self.random_code_with_size(size, context));
            }
            Code::List(list)
        }
    }

    fn decompose(&self, number: usize, max_parts: usize) -> Vec<usize> {
        if 1 == number || 1 == max_parts {
            return vec![1];
        }
        let this_part = self.rng.borrow_mut().gen_range(1..=(number - 1));
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
