use crate::{Instruction, InstructionType};
use fnv::FnvHashSet;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    allowed_instructions: FnvHashSet<Instruction>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            allowed_instructions: FnvHashSet::default(),
        }
    }

    pub fn disallow_type(&mut self, disallowed: InstructionType) {
        // From the instructions we currently allow, make a list of the ones that don't use the specified type in any
        // way.
        let mut remaining: Vec<Instruction> = self
            .allowed_instructions
            .iter()
            .filter(|inst| !inst.types().contains(&disallowed))
            .map(|inst| *inst)
            .collect();

        // Set our list of allowed instructions to just the ones that are left.
        self.allowed_instructions.clear();
        for inst in remaining.drain(..) {
            self.allowed_instructions.insert(inst);
        }
    }

    pub fn disallow_instruction(&mut self, disallow: Instruction) {
        self.allowed_instructions.remove(&disallow);
    }

    pub fn allow_instruction(&mut self, allow: Instruction) {
        self.allowed_instructions.insert(allow);
    }

    pub fn get_allowed_instructions(&self) -> Vec<Instruction> {
        self.allowed_instructions
            .iter()
            .map(|i| i.clone())
            .collect()
    }
}
