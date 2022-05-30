use crate::{InstructionData, InstructionTable};

pub trait Instruction {
    /// All instructions must be parsable by 'nom' from a string. Parsing an instruction will either return an error to
    /// indicate the instruction was not found, or the optional data, indicating the instruction was found and parsing
    /// should cease.
    fn parse(input: &str) -> nom::IResult<&str, Option<InstructionData>>;

    /// All instructions must also be able to write to a string that can later be parsed by nom.
    fn nom_fmt(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;

    /// If the instruction makes use of InstructionData, it must be able to generate a random value for code generation.
    /// If it does not use InstructionData, it just returns None
    fn random_value(_rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        None
    }

    /// Instructions are pure functions on a Context and optional InstructionData. All parameters are read from the
    /// Context and/or data and all outputs are updates to the Context.
    fn execute(context: &mut crate::context::NewContext, data: &Option<InstructionData>);
}

pub trait InstructionTrait<Context> {
    fn name() -> &'static str;

    // Instructions are pure functions on a Context. All parameters are read from the Context and all outputs are
    // updates to the Context.
    fn execute(context: &mut Context);

    fn add_to_table(table: &mut InstructionTable<Context>) {
        table.set(Self::name(), Self::execute)
    }
}
