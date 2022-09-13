use fnv::FnvHashMap;

use crate::{Instruction, Opcode, Code, VirtualMachine, PushList};

/// The instruction table allows a single point of entry for the lookup of the main function that every instruction has.
/// This is used to convert from opcode to executation and back.
/// 
/// It's okay to use a boxed trait object here because these are constructed once during the virtual machine setup and
/// then only referenced. Its use is similar to a compiled virtual table.
/// 
/// The first entry in every InstructionTable is for PushList, which fixes the 'zero' opcode to reference PushList. All
/// other instructions have opcodes in the order in which they are added to the table
pub struct InstructionTable<Vm: VirtualMachine> {
    instructions_by_opcode: Vec<Box<dyn Instruction<Vm>>>,
    lookup_opcode_by_name: FnvHashMap<&'static str, Opcode>
}

pub trait OpcodeConvertor {
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str>;
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode>;
}

impl<Vm: VirtualMachine> InstructionTable<Vm> {
    pub fn new() -> InstructionTable<Vm> {
        let mut instructions = InstructionTable {
            instructions_by_opcode: vec![],
            lookup_opcode_by_name: FnvHashMap::default(),
        };

        instructions.add_instruction(Box::new(PushList{}));

        instructions
    }

    pub fn add_instruction(&mut self, instruction: Box<dyn Instruction<Vm>>) -> Opcode {
        assert!(self.instructions_by_opcode.len() < u32::MAX as usize, "Added too many instructions. Please reconsider why you really need 4 billion instructions");
        let opcode = self.instructions_by_opcode.len() as Opcode;
        let name = instruction.name();
        self.instructions_by_opcode.push(instruction);
        self.lookup_opcode_by_name.insert(name, opcode);

        opcode
    }

    /// Returns a reference to the instruction for the specified opcode, or None
    pub fn instruction(&self, opcode: Opcode) -> Option<&Box<dyn Instruction<Vm>>> {
        let index = opcode as usize;
        self.instructions_by_opcode.get(index)
    }

    /// Loop through all instructions to find one that can parse the input
    pub fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code> {
        // Loop through the instructions to see if any can successfully parse the input. Skip the first one which is
        // always PushList. The opcode is the index
        for (index, instruction) in self.instructions_by_opcode.iter().enumerate().skip(1) {
            let opcode = index as Opcode;
            match instruction.parse(input, opcode) {
                Ok((rest, code)) => return Ok((rest, code)),
                Err(_) => {
                    // Continue searching
                }
            }
        }

        // Return an error if none of our parsers could parse the string
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }

    /// Using the opcode of the Code object, call the appropriate format function. This may need to recursively call
    /// format for child objects (PushList does this), so also provide a reference to the table
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, code: &Code) -> std::fmt::Result {
        if let Some(instruction) = self.instruction(code.get_opcode()) {
            instruction.fmt(f, code, &self)
        } else {
            panic!("UNKNOWN_OPCODE {}", code.get_opcode());
        }
    }
}

impl<Vm: VirtualMachine> OpcodeConvertor for InstructionTable<Vm> {

    /// Returns the name for the specified opcode, or None if the opcode does not exist
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str> {
        self.instructions_by_opcode.get(opcode as usize).map(|instruction| instruction.name())
    }

    /// Returns the opcode for the specified name, or None if the named instruction has not been registered
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode> {
        self.lookup_opcode_by_name.get(name).map(|o| *o)
    }
}

impl<Vm: VirtualMachine> std::fmt::Debug for InstructionTable<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionTable with {} instructions", self.instructions_by_opcode.len())
    }
}

impl<Vm: VirtualMachine> std::cmp::PartialEq for InstructionTable<Vm> {
    fn eq(&self, other: &InstructionTable<Vm>) -> bool {
        if self.instructions_by_opcode.len() != other.instructions_by_opcode.len() {
            return false;
        }
        for i in 0..self.instructions_by_opcode.len() {
            if self.instructions_by_opcode[i].name() != other.instructions_by_opcode[i].name() {
                return false;
            }
        }

        true
    }
}