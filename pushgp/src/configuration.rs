use crate::{Code, Instruction, InstructionType, RandomCodeGenerator, RandomType};
use fnv::FnvHashMap;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    instruction_weights: FnvHashMap<Instruction, u8>,
    ephemeral_bool_weight: u8,
    ephemeral_float_weight: u8,
    ephemeral_int_weight: u8,
    ephemeral_name_weight: u8,
    defined_name_weight: u8,
    random_seed: Option<u64>,
    runtime_instruction_limit: usize,
    code_generator: RandomCodeGenerator,
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
            code_generator: RandomCodeGenerator::new(),
        }
    }

    pub fn disallow_type(&mut self, disallowed: InstructionType) {
        // Loop through all instructions and set the weight to zero for any that use the specified type
        for (inst, weight) in self.instruction_weights.iter_mut() {
            if inst.types().contains(&disallowed) {
                *weight = 0;
                self.code_generator.clear_types();
            }
        }
    }

    /// Sets the weight of the specified instruction. Set to zero to prevent the instruction from being used in random
    /// code generation. The weight will be the relative likelihood of the instruction being selected.
    pub fn set_instruction_weight(&mut self, allow: Instruction, weight: u8) {
        self.instruction_weights.insert(allow, weight);
        self.code_generator.clear_types();
    }

    /// Returns a list of all instructions that have a weight > 0
    pub fn allowed_instructions(&self) -> Vec<Instruction> {
        self.instruction_weights.iter().filter(|pair| *pair.1 != 0).map(|pair| *pair.0).collect()
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.random_seed = Some(seed);
        self.code_generator.set_seed(self.random_seed);
    }

    pub fn generate_random_code(&mut self, defined_names: &[u64]) -> Code {
        if !self.code_generator.are_types_defined() {
            // Define the ephemal constants types
            self.code_generator.append_type(RandomType::EphemeralBool, self.ephemeral_bool_weight);
            self.code_generator.append_type(RandomType::EphemeralFloat, self.ephemeral_float_weight);
            self.code_generator.append_type(RandomType::EphemeralInt, self.ephemeral_int_weight);
            self.code_generator.append_type(RandomType::EphemeralName, self.ephemeral_name_weight);

            // Define all the names we know about
            for &name in defined_names {
                self.code_generator.append_type(RandomType::DefinedName(name), self.defined_name_weight);
            }

            // Define all the instructions we know about
            for (&inst, &weight) in self.instruction_weights.iter() {
                self.code_generator.append_type(RandomType::Instruction(inst), weight);
            }
        }

        // Generate some code!
        self.code_generator.generate()
    }
}
