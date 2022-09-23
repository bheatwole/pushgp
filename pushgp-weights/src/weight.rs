use pushgp::{
    parse_code_integer, Code, Instruction, InstructionTable, Opcode, OpcodeConvertor, Stack,
    StaticName, VirtualMachine, VirtualMachineEngine, ExecutionError,
};

pub type Weight = u8;

pub trait VirtualMachineMustHaveWeight<Vm> {
    fn weight(&mut self) -> &mut Stack<Weight>;
}

#[derive(Clone)]
pub struct WeightLiteralValue {}

impl StaticName for WeightLiteralValue {
    fn static_name() -> &'static str {
        "WEIGHT.LITERALVALUE"
    }
}

impl WeightLiteralValue {
    pub fn new_code<Oc: OpcodeConvertor>(oc: &Oc, value: Weight) -> Code {
        let opcode = oc.opcode_for_name(Self::static_name()).unwrap();
        Code::new(opcode, value.into())
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveWeight<Vm>> Instruction<Vm> for WeightLiteralValue {
    fn parse<'a>(input: &'a str, opcode: Opcode) -> nom::IResult<&'a str, Code> {
        let (rest, value) = parse_code_integer(input)?;
        Ok((rest, Code::new(opcode, value.into())))
    }

    fn fmt(
        f: &mut std::fmt::Formatter<'_>,
        code: &Code,
        _vtable: &InstructionTable<Vm>,
    ) -> std::fmt::Result {
        if let Some(value) = code.get_data().integer_value() {
            write!(f, "{}", value)
        } else {
            panic!("fmt called for WeightLiteralValue with Code that does not have a integer value stored")
        }
    }

    fn random_value(engine: &mut VirtualMachineEngine<Vm>) -> Code {
        use rand::Rng;
        let value: u8 = engine.get_rng().gen_range(0..=255);
        WeightLiteralValue::new_code(engine, value)
    }

    /// Executing a WeightLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(code: Code, vm: &mut Vm) -> Result<(), ExecutionError> {
        if let Some(value) = code.get_data().integer_value() {
            vm.weight().push(value as u8)?;
        }

        Ok(())
    }
}
