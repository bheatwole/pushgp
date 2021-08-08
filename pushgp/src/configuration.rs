use crate::{Instruction, InstructionType};
use fnv::FnvHashSet;

#[derive(Debug, PartialEq)]
pub struct Configuration {
    allowed_instructions: FnvHashSet<Instruction>,
}

impl Configuration {
    pub fn new() -> Configuration {
        let mut config = Configuration {
            allowed_instructions: FnvHashSet::default(),
        };
        config.allowed_instructions.insert(Instruction::BoolAnd);
        config.allowed_instructions.insert(Instruction::BoolDefine);
        config.allowed_instructions.insert(Instruction::BoolDup);
        config.allowed_instructions.insert(Instruction::BoolEqual);
        config.allowed_instructions.insert(Instruction::BoolFlush);
        config
            .allowed_instructions
            .insert(Instruction::BoolFromFloat);
        config.allowed_instructions.insert(Instruction::BoolFromInt);
        config.allowed_instructions.insert(Instruction::BoolNot);
        config.allowed_instructions.insert(Instruction::BoolOr);
        config.allowed_instructions.insert(Instruction::BoolPop);
        config.allowed_instructions.insert(Instruction::BoolRand);
        config.allowed_instructions.insert(Instruction::BoolRot);
        config.allowed_instructions.insert(Instruction::BoolShove);
        config
            .allowed_instructions
            .insert(Instruction::BoolStackDepth);
        config.allowed_instructions.insert(Instruction::BoolSwap);
        config.allowed_instructions.insert(Instruction::BoolYank);
        config.allowed_instructions.insert(Instruction::BoolYankDup);

        config
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
}
