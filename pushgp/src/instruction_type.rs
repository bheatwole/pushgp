/// Represents which stack an instruction may interact with.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum InstructionType {
    Bool,
    Code,
    Float,
    Int,
    Name,
}
