use crate::instruction_table::InstructionTable;

pub trait InstructionTrait<Context> {
    fn name() -> &'static str;

    // Instructions are pure functions on a Context. All parameters are read from the Context and all outputs are
    // updates to the Context.
    fn execute(context: &mut Context);

    fn add_to_table(table: &mut InstructionTable<Context>) {
        table.set(Self::name(), Self::execute)
    }
}
