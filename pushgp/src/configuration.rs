// use crate::{Code, Context, GeneticOperation, VirtualTable};
// use rand::{prelude::SliceRandom, rngs::SmallRng, Rng, SeedableRng};
// use std::cell::RefCell;
// use std::ops::DerefMut;

use crate::GeneticOperation;
use fnv::FnvHashMap;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    // TODO: a random program running long enough can use more memory than the real hardware has. Implement a way to
    // determine usage per stack element and track total usage compared to this number.
    max_memory_size: usize,

    max_points_in_random_expressions: usize,

    crossover_rate: u8,
    mutation_rate: u8,

    defined_name_weight: u8,

    instruction_weights: FnvHashMap<&'static str, u8>,
}

impl Configuration {
    pub fn new(
        max_memory_size: usize,
        max_points_in_random_expressions: usize,
        crossover_rate: u8,
        mutation_rate: u8,
        defined_name_weight: u8,
        instruction_weights: FnvHashMap<&'static str, u8>,
    ) -> Configuration {
        Configuration {
            max_memory_size,
            max_points_in_random_expressions,
            crossover_rate,
            mutation_rate,
            defined_name_weight,
            instruction_weights,
        }
    }

    pub fn new_simple() -> Configuration {
        Configuration {
            max_memory_size: 65536,
            max_points_in_random_expressions: 100,
            crossover_rate: 99,
            mutation_rate: 1,
            defined_name_weight: 1,
            instruction_weights: FnvHashMap::default(),
        }
    }

    pub fn get_max_points_in_random_expressions(&self) -> usize {
        self.max_points_in_random_expressions
    }

    pub fn get_crossover_rate(&self) -> u8 {
        self.crossover_rate
    }

    pub fn get_mutation_rate(&self) -> u8 {
        self.mutation_rate
    }

    pub fn get_defined_name_weight(&self) -> u8 {
        self.defined_name_weight
    }

    /// Returns the weight of the specified instruction. If a weight the instruction was not specified earlier, a '1' is
    /// always returned. To turn off random generation of an instruction, you must specify it with a '0' weight.
    pub fn get_instruction_weight(&self, instruction_name: &'static str) -> u8 {
        if let Some(weight) = self.instruction_weights.get(&instruction_name) {
            *weight
        } else {
            1
        }
    }

    /// Returns a random genetic operation
    pub fn random_genetic_operation<R: rand::Rng>(&self, rng: &mut R) -> GeneticOperation {
        let total: usize = self.mutation_rate as usize + self.crossover_rate as usize;
        let pick = rng.gen_range(0..total);

        if pick < self.mutation_rate as usize {
            GeneticOperation::Mutation
        } else {
            GeneticOperation::Crossover
        }
    }
}

// /// A Configuration is a Vec of u8 where each u8 represents the weight of one of the possible randomly generated items
// /// for Code. The first u8 is the likelihood of picking an already defined name. The last set of u8s is the chance of
// /// picking each of the instructions.  Any weight set to zero means the random code generator will not pick that item.
// /// An item with a weight of '2' is twice as likely to be picked as an item with a weight of '1'.
// ///
// /// A Vec<u8> is used to allow for running an island where the random code generator itself is optimized by genetic
// /// programming. Crossover, mutation, etc are applied to the Configurations, new populations are generated and run for
// /// a few generations on the main island. The Configuration that produces the most fit population is the winner.
// #[derive(Clone, Debug, PartialEq)]
// pub struct Configuration<State: std::fmt::Debug + Clone> {
//     rng: RefCell<SmallRng>,
//     virtual_table: VirtualTable<State>,

//     max_points_in_random_expressions: usize,

//     crossover_rate: u8,
//     mutation_rate: u8,

//     defined_name_weight: usize,

//     instruction_total: usize,
//     instruction_weights: Vec<InstructionEntry>,
// }

// #[derive(Clone, Copy, Debug, PartialEq)]
// struct InstructionEntry {
//     pub weight: usize,
//     pub instruction: usize,
// }

// impl<State: std::fmt::Debug + Clone> Configuration<State> {
//     pub fn new(
//         rng_seed: Option<u64>,
//         max_points_in_random_expressions: usize,
//         virtual_table: &VirtualTable<State>,
//         weights: &[u8],
//     ) -> Configuration<State> {
//         let (crossover_rate, weights) = pop_front_weight_or_one_and_return_rest(weights);
//         let (mutation_rate, weights) = pop_front_weight_or_one_and_return_rest(weights);
//         let (defined_name_weight, weights) = pop_front_weight_or_one_and_return_rest(weights);
//         let (instruction_total, instruction_weights, _) = Self::make_instruction_weights(virtual_table, weights);

//         Configuration {
//             rng: RefCell::new(small_rng_from_optional_seed(rng_seed)),
//             virtual_table: virtual_table.clone(),
//             max_points_in_random_expressions,
//             crossover_rate,
//             mutation_rate,
//             defined_name_weight: defined_name_weight as usize,
//             instruction_total,
//             instruction_weights,
//         }
//     }

//     fn make_instruction_weights<'a, 'b>(
//         virtual_table: &'a VirtualTable<State>,
//         mut weights: &'b [u8],
//     ) -> (usize, Vec<InstructionEntry>, &'b [u8]) {
//         let mut total = 0;
//         let mut entries = vec![];

//         for id in 0..virtual_table.len() {
//             let (instruction_weight, rest_weights) = pop_front_weight_or_one_and_return_rest(weights);
//             weights = rest_weights;

//             total += instruction_weight as usize;
//             entries.push(InstructionEntry { weight: total, instruction: id });
//         }

//         (total, entries, weights)
//     }

//     /// Seeds the random number with a specific value so that you may get repeatable results. Passing `None` will seed
//     /// the generator with a truly random value ensuring unique results.
//     pub fn set_seed(&self, seed: Option<u64>) {
//         self.rng.replace(small_rng_from_optional_seed(seed));
//     }

//     /// Runs the specified function with the random number generator
//     pub fn run_random_function<F, ResultType>(&self, func: F) -> ResultType
//     where
//         F: Fn(&mut SmallRng) -> ResultType,
//     {
//         let mut rng = self.rng.borrow_mut();
//         func(rng.deref_mut())
//     }

//     /// Returns a list of all instructions that have a weight > 0
//     pub fn allowed_instructions(&self) -> Vec<usize> {
//         self.instruction_weights
//             .iter()
//             .filter(|entry| entry.weight != 0)
//             .map(|entry| entry.instruction.clone())
//             .collect()
//     }

//     /// Returns a random genetic operation
//     pub fn random_genetic_operation(&self) -> GeneticOperation {
//         let total: usize = self.mutation_rate as usize + self.crossover_rate as usize;
//         let pick = self.rng.borrow_mut().gen_range(0..total);

//         if pick < self.mutation_rate as usize {
//             GeneticOperation::Mutation
//         } else {
//             GeneticOperation::Crossover
//         }
//     }

//     /// Returns one random atom
//     pub fn random_atom(&self, context: Option<&Context<State>>) -> Code {
//         // Determine how many total possibilities there are. This shifts depending upon how many defined_names we have.
//         let defined_names_total = if let Some(context) = context { context.defined_names_len() } else { 0 };
//         let random_total = defined_names_total + self.instruction_total;

//         // Pick one
//         let mut pick = self.rng.borrow_mut().gen_range(0..random_total);

//         // Is it a defined name?
//         if pick < defined_names_total {
//             return self.random_defined_name(context.unwrap());
//         }
//         pick -= defined_names_total;

//         // Must be an instruction
//         self.random_instruction(pick)
//     }

//     /// Returns one random defined name
//     pub fn random_defined_name(&self, context: &Context<State>) -> Code {
//         let defined_names = context.all_defined_names();
//         let pick = self.rng.borrow_mut().gen_range(0..defined_names.len());
//         context.definition_for_name(&defined_names[pick]).unwrap()
//     }

//     /// Returns a new random instruction
//     pub fn random_instruction(&self, pick: usize) -> Code {
//         let index = self.instruction_weights.partition_point(|entry| entry.weight < pick);
//         let mut rng = self.rng.borrow_mut();
//         let data = self.virtual_table.call_random_value(index, rng.deref_mut());
//         Code::InstructionWithData(index, data)
//     }

//     /// Generates some random code using the configured weight parameters.
//     ///
//     /// The generated code will have at least one code point and as many as `self.max_points_in_random_expressions`.
//     /// The generated code will be in a general tree-like shape using lists of lists as the trunks and individual
//     /// atoms as the leaves. The shape is neither balanced nor linear, but somewhat in between.
//     pub fn generate_random_code(&self, points: Option<usize>, context: Option<&Context<State>>) -> Code {
//         let max_points = if let Some(maybe_huge_max) = points {
//             let max = maybe_huge_max % self.max_points_in_random_expressions;
//             if max > 0 {
//                 max
//             } else {
//                 1
//             }
//         } else {
//             self.max_points_in_random_expressions
//         };
//         let actual_points = self.rng.borrow_mut().gen_range(1..=max_points);
//         self.random_code_with_size(actual_points, context)
//     }

//     fn random_code_with_size(&self, points: usize, context: Option<&Context<State>>) -> Code {
//         if 1 == points {
//             // We need a leaf, so pick one of the atoms
//             self.random_atom(context)
//         } else {
//             // Break this level down into a list of lists, or possibly specific leaf atoms.
//             let mut sizes_this_level = self.decompose(points - 1, points - 1);
//             {
//                 let mut rng = self.rng.borrow_mut();
//                 sizes_this_level.shuffle(rng.deref_mut());
//             }
//             let mut list = vec![];
//             for size in sizes_this_level {
//                 list.push(self.random_code_with_size(size, context));
//             }
//             Code::List(list)
//         }
//     }

//     fn decompose(&self, number: usize, max_parts: usize) -> Vec<usize> {
//         if 1 == number || 1 == max_parts {
//             return vec![1];
//         }
//         let this_part = self.rng.borrow_mut().gen_range(1..=(number - 1));
//         let mut result = vec![this_part];
//         result.extend_from_slice(&self.decompose(number - this_part, max_parts - 1));
//         result
//     }
// }

// fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
//     if let Some(seed) = rng_seed {
//         SmallRng::seed_from_u64(seed)
//     } else {
//         SmallRng::from_entropy()
//     }
// }

// fn pop_front_weight_or_one_and_return_rest(weights: &[u8]) -> (u8, &[u8]) {
//     if weights.len() == 0 {
//         (1, weights)
//     } else {
//         (weights[0], &weights[1..])
//     }
// }
