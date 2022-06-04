use crate::{InstructionData, InstructionTable, VirtualTable};

pub trait Instruction {
    /// Every instruction must have a name
    fn name() -> &'static str;

    /// All instructions must be parsable by 'nom' from a string. Parsing an instruction will either return an error to
    /// indicate the instruction was not found, or the optional data, indicating the instruction was found and parsing
    /// should cease.
    fn parse(input: &str) -> nom::IResult<&str, Option<InstructionData>> {
        let (rest, _) = nom::bytes::complete::tag(Self::name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, None))
    }

    /// All instructions must also be able to write to a string that can later be parsed by nom.
    fn nom_fmt(_data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::name())
    }

    /// If the instruction makes use of InstructionData, it must be able to generate a random value for code generation.
    /// If it does not use InstructionData, it just returns None
    fn random_value(_rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        None
    }

    /// Instructions are pure functions on a Context and optional InstructionData. All parameters are read from the
    /// Context and/or data and all outputs are updates to the Context.
    fn execute(context: &crate::context::NewContext, data: Option<InstructionData>);

    fn add_to_virtual_table(table: &mut VirtualTable) {
        table.add_entry(Self::name(), Self::parse, Self::nom_fmt, Self::random_value, Self::execute);
    }
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
