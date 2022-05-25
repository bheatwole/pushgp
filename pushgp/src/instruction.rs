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

// /// Each Instruction performs a different operation to the stacks of a Context. The bulk of the repeative definitions
// /// are handled by macros.
// #[derive(Clone, ConfigureAllInstructions, Copy, Debug, Display, Eq, ExecuteInstruction, Hash, NomTag, PartialEq)]
// pub enum Instruction {
//     /// Defines the name on top of the NAME stack as an instruction that will push the top item of the INTEGER stack
//     /// onto the EXEC stack.
//     IntegerDefine,
//     /// Pushes the difference of the top two items; that is, the second item minus the top item.
//     IntegerDifference,
//     /// Duplicates the top item on the INTEGER stack. Does not pop its argument (which, if it did, would negate the
//     /// effect of the duplication!).
//     IntegerDup,
//     /// Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE otherwise.
//     IntegerEqual,
//     /// Empties the INTEGER stack.
//     IntegerFlush,
//     /// Pushes 1 if the top BOOLEAN is TRUE, or 0 if the top BOOLEAN is FALSE.
//     IntegerFromBoolean,
//     /// Pushes the result of truncating the top FLOAT.
//     IntegerFromFloat,
//     /// Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or FALSE otherwise.
//     IntegerGreater,
//     /// Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or FALSE otherwise.
//     IntegerLess,
//     /// Pushes the maximum of the top two items.
//     IntegerMax,
//     /// Pushes the minimum of the top two items.
//     IntegerMin,
//     /// Pushes the second stack item modulo the top stack item. If the top item is zero this acts as a NOOP. The modulus
//     /// is computed as the remainder of the quotient, where the quotient has first been truncated toward negative
//     /// infinity. (This is taken from the definition for the generic MOD function in Common Lisp, which is described for
//     /// example at http://www.lispworks.com/reference/HyperSpec/Body/f_mod_r.htm.)
//     IntegerModulo,
//     /// Pops the INTEGER stack.
//     IntegerPop,
//     /// Pushes the product of the top two items.
//     IntegerProduct,
//     /// Pushes the quotient of the top two items; that is, the second item divided by the top item. If the top item is
//     /// zero this acts as a NOOP.
//     IntegerQuotient,
//     /// Pushes a newly generated random INTEGER that is greater than or equal to MIN-RANDOM-INTEGER and less than or
//     /// equal to MAX-RANDOM-INTEGER.
//     IntegerRand,
//     /// Rotates the top three items on the INTEGER stack, pulling the third item out and pushing it on top. This is
//     /// equivalent to "2 INTEGER.YANK".
//     IntegerRot,
//     /// Inserts the second INTEGER "deep" in the stack, at the position indexed by the top INTEGER. The index position
//     /// is calculated after the index is removed.
//     IntegerShove,
//     /// Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
//     IntegerStackdepth,
//     /// Pushes the sum of the top two items.
//     IntegerSum,
//     /// Swaps the top two INTEGERs.
//     IntegerSwap,
//     /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
//     /// The index is taken from the INTEGER stack, and the indexing is done after the index is removed.
//     IntegerYankDup,
//     /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
//     /// INTEGER stack, and the indexing is done after the index is removed.
//     IntegerYank,
//     /// Duplicates the top item on the NAME stack. Does not pop its argument (which, if it did, would negate the effect
//     /// of the duplication!).
//     NameDup,
//     /// Pushes TRUE if the top two NAMEs are equal, or FALSE otherwise.
//     NameEqual,
//     /// Empties the NAME stack.
//     NameFlush,
//     /// Pops the NAME stack.
//     NamePop,
//     /// Sets a flag indicating that the next name encountered will be pushed onto the NAME stack (and not have its
//     /// associated value pushed onto the EXEC stack), regardless of whether or not it has a definition. Upon
//     /// encountering such a name and pushing it onto the NAME stack the flag will be cleared (whether or not the pushed
//     /// name had a definition).
//     NameQuote,
//     /// Pushes a randomly selected NAME that already has a definition.
//     NameRandBoundName,
//     /// Pushes a newly generated random NAME.
//     NameRand,
//     /// Rotates the top three items on the NAME stack, pulling the third item out and pushing it on top. This is
//     /// equivalent to "2 NAME.YANK".
//     NameRot,
//     /// Inserts the top NAME "deep" in the stack, at the position indexed by the top INTEGER.
//     NameShove,
//     /// Pushes the stack depth onto the INTEGER stack.
//     NameStackdepth,
//     /// Swaps the top two NAMEs.
//     NameSwap,
//     /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
//     /// The index is taken from the INTEGER stack.
//     NameYankDup,
//     /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
//     /// INTEGER stack.
//     NameYank,
// }
