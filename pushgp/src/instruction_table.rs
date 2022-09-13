use fnv::FnvHashMap;

use crate::{Code, Instruction, Opcode, PushList, VirtualMachine, VirtualMachineEngine, CodeParser};

pub type NameFn = fn() -> &'static str;
pub type ParseFn = fn(input: &str, opcode: Opcode) -> nom::IResult<&str, Code>;
pub type FmtFn<Vm> =
    fn(f: &mut std::fmt::Formatter<'_>, code: &Code, vtable: &InstructionTable<Vm>) -> std::fmt::Result;
pub type RandomValueFn<Vm> = fn(engine: &mut VirtualMachineEngine<Vm>) -> Code;
pub type ExecuteFn<Vm> = fn(code: Code, vm: &mut Vm);

/// The instruction table allows a single point of entry for the lookup of the main function that every instruction has.
/// This is used to convert from opcode to executation and back.
///
/// It's okay to use a boxed trait object here because these are constructed once during the virtual machine setup and
/// then only referenced. Its use is similar to a compiled virtual table.
///
/// The first entry in every InstructionTable is for PushList, which fixes the 'zero' opcode to reference PushList. All
/// other instructions have opcodes in the order in which they are added to the table
pub struct InstructionTable<Vm: VirtualMachine> {
    name_functions: Vec<NameFn>,
    parse_functions: Vec<ParseFn>,
    fmt_functions: Vec<FmtFn<Vm>>,
    random_value_functions: Vec<RandomValueFn<Vm>>,
    execute_functions: Vec<ExecuteFn<Vm>>,
    lookup_opcode_by_name: FnvHashMap<&'static str, Opcode>,
}

pub trait OpcodeConvertor {
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str>;
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode>;
}

impl<Vm: VirtualMachine> InstructionTable<Vm> {
    pub fn new() -> InstructionTable<Vm> {
        let mut instructions = InstructionTable {
            name_functions: vec![],
            parse_functions: vec![],
            fmt_functions: vec![],
            random_value_functions: vec![],
            execute_functions: vec![],
            lookup_opcode_by_name: FnvHashMap::default(),
        };

        instructions.add_instruction::<PushList>();

        instructions
    }

    pub fn add_instruction<I: Instruction<Vm>>(&mut self) -> Opcode {
        assert!(
            self.name_functions.len() < u32::MAX as usize,
            "Added too many instructions. Please reconsider why you really need 4 billion instructions"
        );
        let opcode = self.name_functions.len() as Opcode;
        let name = I::static_name();
        self.name_functions.push(I::static_name);
        self.parse_functions.push(I::parse);
        self.fmt_functions.push(I::fmt);
        self.random_value_functions.push(I::random_value);
        self.execute_functions.push(I::execute);
        self.lookup_opcode_by_name.insert(name, opcode);

        opcode
    }

    /// Using the opcode of the Code object, call the appropriate format function. This may need to recursively call
    /// format for child objects (PushList does this), so also provide a reference to the table
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, code: &Code) -> std::fmt::Result {
        if let Some(fmt_fn) = self.fmt_functions.get(code.get_opcode() as usize) {
            fmt_fn(f, code, &self)
        } else {
            panic!("UNKNOWN_OPCODE {}", code.get_opcode());
        }
    }

    /// Returns the random value fn pointer for the specified opcode or None
    pub fn random_value_fn(&self, opcode: Opcode) -> Option<RandomValueFn<Vm>> {
        self.random_value_functions.get(opcode as usize).map(|f| *f)
    }

    /// Returns the execute fn pointer for the specified opcode or None
    pub fn execute_fn(&self, opcode: Opcode) -> Option<ExecuteFn<Vm>> {
        self.execute_functions.get(opcode as usize).map(|f| *f)
    }
}

impl<Vm: VirtualMachine> CodeParser for InstructionTable<Vm> {
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code> {
        // Loop through the instructions to see if any can successfully parse the input. Skip the first one which is
        // always PushList. The opcode is the index
        for (index, parse_fn) in self.parse_functions.iter().enumerate().skip(1) {
            let opcode = index as Opcode;
            match parse_fn(input, opcode) {
                Ok((rest, code)) => return Ok((rest, code)),
                Err(_) => {
                    // Continue searching
                }
            }
        }

        // Return an error if none of our parsers could parse the string
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }
}

impl<Vm: VirtualMachine> OpcodeConvertor for InstructionTable<Vm> {
    /// Returns the name for the specified opcode, or None if the opcode does not exist
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str> {
        self.name_functions.get(opcode as usize).map(|name_fn| name_fn())
    }

    /// Returns the opcode for the specified name, or None if the named instruction has not been registered
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode> {
        self.lookup_opcode_by_name.get(name).map(|o| *o)
    }
}

impl<Vm: VirtualMachine> std::fmt::Debug for InstructionTable<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionTable with {} instructions", self.name_functions.len())
    }
}

impl<Vm: VirtualMachine> std::cmp::PartialEq for InstructionTable<Vm> {
    fn eq(&self, other: &InstructionTable<Vm>) -> bool {
        if self.name_functions.len() != other.name_functions.len() {
            return false;
        }
        for i in 0..self.name_functions.len() {
            if self.name_functions[i] != other.name_functions[i] {
                return false;
            }
        }

        true
    }
}
