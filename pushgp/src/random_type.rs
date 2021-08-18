use crate::Instruction;

/// Defines the different kinds of code that can be selected randomly when building code
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RandomType {
    /// One of the instructions may be selected
    Instruction(Instruction),

    /// A name that's already defined may be selected
    DefinedName(u64),

    /// A new random bool may be selected
    EphemeralBool,

    /// A new random float may be selected
    EphemeralFloat,

    /// A new random integer may be selected
    EphemeralInt,

    /// A new random name may be selected
    EphemeralName,
}
