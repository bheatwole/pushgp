use crate::*;

#[derive(Debug)]
pub struct PushList {}

impl StaticName for PushList {
    fn static_name() -> &'static str {
        "__PUSH.LIST"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> Instruction<Vm> for PushList {
    // The PushList cannot be parsed this way because it requires recursive parsing (and thus access to the parser). See
    // parse.rs for the implementation of recursive parsing
    fn parse<'a>(input: &'a str, _opcode: u32) -> nom::IResult<&'a str, Code> {
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }

    fn fmt(f: &mut std::fmt::Formatter<'_>, code: &Code, vtable: &InstructionTable<Vm>) -> std::fmt::Result {
        write!(f, "(")?;
        if let Some(iter) = code.get_data().code_iter() {
            for c in iter {
                write!(f, " ")?;
                vtable.fmt(f, c)?;
            }
        } else {
            panic!("fmt called for PushList with data that is not a CodeList")
        }
        write!(f, " )")
    }

    // A PushList should typically have its weight set to zero and never called for a random value. The tree of
    // Code values is created in the random code generation.
    fn random_value(_engine: &mut VirtualMachineEngine<Vm>) -> Code {
        Code::new(0, Data::CodeList(vec![]))
    }

    fn execute(mut code: Code, vm: &mut Vm) -> Result<(), ExecutionError> {
        match code.get_data_mut() {
            Data::CodeList(list) => {
                while let Some(item) = list.pop() {
                    vm.exec().push(item)?;
                }
                Ok(())
            }
            _ => Err(ExecutionError::IllegalOperation),
        }
    }
}
