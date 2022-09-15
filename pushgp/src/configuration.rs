use crate::GeneticOperation;
use fnv::FnvHashMap;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    // A random program running long enough can use more memory than the real hardware has. Implement a way to
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

    pub fn get_max_memory_size(&self) -> usize {
        self.max_memory_size
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
