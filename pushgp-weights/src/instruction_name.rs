use pushgp::{
    Code, Instruction, InstructionTable, Opcode, OpcodeConvertor, Stack,
    StaticName, VirtualMachine, VirtualMachineEngine,
};

pub type InstructionName = &'static str;

pub trait VirtualMachineMustHaveInstructionName<Vm> {
    fn instruction_name(&mut self) -> &mut Stack<InstructionName>;
}

#[derive(Clone)]
pub struct InstructionNameLiteralValue {}

impl StaticName for InstructionNameLiteralValue {
    fn static_name() -> &'static str {
        "INSTRUCTIONNAME.LITERALVALUE"
    }
}

impl InstructionNameLiteralValue {
    pub fn new_code<Oc: OpcodeConvertor>(oc: &Oc, value: InstructionName) -> Code {
        let opcode = oc.opcode_for_name(Self::static_name()).unwrap();
        Code::new(opcode, pushgp::Data::StaticString(value))
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveInstructionName<Vm>> Instruction<Vm> for InstructionNameLiteralValue {
    fn parse<'a>(_input: &'a str, _opcode: Opcode) -> nom::IResult<&'a str, Code> {
        panic!("The only way to construct an InstructionNameLiteralValue is by calling `random_value`")
    }

    fn fmt(
        f: &mut std::fmt::Formatter<'_>,
        code: &Code,
        _vtable: &InstructionTable<Vm>,
    ) -> std::fmt::Result {
        if let Some(value) = code.get_data().static_string_value() {
            write!(f, "{}", value)
        } else {
            panic!("fmt called for InstructionNameLiteralValue with Code that does not have a static string value stored")
        }
    }

    fn random_value(engine: &mut VirtualMachineEngine<Vm>) -> Code {
        use rand::Rng;
        let names = engine.get_weights().get_instruction_names();
        let pick: usize = engine.get_rng().gen_range(0..names.len());
        InstructionNameLiteralValue::new_code(engine, names[pick])
    }

    /// Executing a InstructionNameLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(code: Code, vm: &mut Vm) {
        if let Some(value) = code.get_data().static_string_value() {
            vm.instruction_name().push(value)
        }
    }
}