use crate::Instruction;

// Code is the basic building block of a PushGP program. It's the translation between human readable and machine
// readable strings.
#[derive(Clone, Debug, PartialEq)]
pub enum Code {
    // A list is just a list containing other code (which can be lists) and may also be empty (len() == 0)
    List(Vec<Code>),

    // Code can be literal values
    LiteralBool(bool),
    LiteralFloat(f64),
    LiteralInteger(i64),
    LiteralName(u64),

    // Code can be an instruction
    Instruction(Instruction),
}
