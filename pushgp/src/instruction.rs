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
//     /// Defines the name on top of the NAME stack as an instruction that will push the top item of the EXEC stack back
//     /// onto the EXEC stack.
//     ExecDefine,
//     /// An iteration instruction that performs a loop (the body of which is taken from the EXEC stack) the number of
//     /// times indicated by the INTEGER argument, pushing an index (which runs from zero to one less than the number of
//     /// iterations) onto the INTEGER stack prior to each execution of the loop body. This is similar to CODE.DO*COUNT
//     /// except that it takes its code argument from the EXEC stack. This should be implemented as a macro that expands
//     /// into a call to EXEC.DO*RANGE. EXEC.DO*COUNT takes a single INTEGER argument (the number of times that the loop
//     /// will be executed) and a single EXEC argument (the body of the loop). If the provided INTEGER argument is
//     /// negative or zero then this becomes a NOOP. Otherwise it expands into:
//     ///   ( 0 <1 - IntegerArg> EXEC.DO*RANGE <ExecArg> )
//     ExecDoNCount,
//     /// An iteration instruction that executes the top item on the EXEC stack a number of times that depends on the top
//     /// two integers, while also pushing the loop counter onto the INTEGER stack for possible access during the
//     /// execution of the body of the loop. This is similar to CODE.DO*COUNT except that it takes its code argument from
//     /// the EXEC stack. The top integer is the "destination index" and the second integer is the "current index."
//     /// First the code and the integer arguments are saved locally and popped. Then the integers are compared. If the
//     /// integers are equal then the current index is pushed onto the INTEGER stack and the code (which is the "body" of
//     /// the loop) is pushed onto the EXEC stack for subsequent execution. If the integers are not equal then the current
//     /// index will still be pushed onto the INTEGER stack but two items will be pushed onto the EXEC stack -- first a
//     /// recursive call to EXEC.DO*RANGE (with the same code and destination index, but with a current index that has
//     /// been either incremented or decremented by 1 to be closer to the destination index) and then the body code. Note
//     /// that the range is inclusive of both endpoints; a call with integer arguments 3 and 5 will cause its body to be
//     /// executed 3 times, with the loop counter having the values 3, 4, and 5. Note also that one can specify a loop
//     /// that "counts down" by providing a destination index that is less than the specified current index.
//     ExecDoNRange,
//     /// Like EXEC.DO*COUNT but does not push the loop counter. This should be implemented as a macro that expands into
//     /// EXEC.DO*RANGE, similarly to the implementation of EXEC.DO*COUNT, except that a call to INTEGER.POP should be
//     /// tacked on to the front of the loop body code in the call to EXEC.DO*RANGE. This call to INTEGER.POP will remove
//     /// the loop counter, which will have been pushed by EXEC.DO*RANGE, prior to the execution of the loop body.
//     ExecDoNTimes,
//     /// Duplicates the top item on the EXEC stack. Does not pop its argument (which, if it did, would negate the effect
//     /// of the duplication!). This may be thought of as a "DO TWICE" instruction.
//     ExecDup,
//     /// Pushes TRUE if the top two items on the EXEC stack are equal, or FALSE otherwise.
//     ExecEqual,
//     /// Empties the EXEC stack. This may be thought of as a "HALT" instruction.
//     ExecFlush,
//     /// If the top item of the BOOLEAN stack is TRUE then this removes the second item on the EXEC stack, leaving the
//     /// first item to be executed. If it is false then it removes the first item, leaving the second to be executed.
//     /// This is similar to CODE.IF except that it operates on the EXEC stack. This acts as a NOOP unless there are at
//     /// least two items on the EXEC stack and one item on the BOOLEAN stack.
//     ExecIf,
//     /// The Push implementation of the "K combinator". Removes the second item on the EXEC stack.
//     ExecK,
//     /// Pops the EXEC stack. This may be thought of as a "DONT" instruction.
//     ExecPop,
//     /// Rotates the top three items on the EXEC stack, pulling the third item out and pushing it on top. This is
//     /// equivalent to "2 EXEC.YANK".
//     ExecRot,
//     /// Inserts the top EXEC item "deep" in the stack, at the position indexed by the top INTEGER. This may be thought
//     /// of as a "DO LATER" instruction.
//     ExecShove,
//     /// Pushes the stack depth onto the INTEGER stack.
//     ExecStackdepth,
//     /// Swaps the top two items on the EXEC stack.
//     ExecSwap,
//     /// The Push implementation of the "S combinator". Pops 3 items from the EXEC stack, which we will call A, B, and C
//     /// (with A being the first one popped). Then pushes a list containing B and C back onto the EXEC stack, followed by
//     /// another instance of C, followed by another instance of A.
//     ExecS,
//     /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
//     /// The index is taken from the INTEGER stack.
//     ExecYankDup,
//     /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
//     /// INTEGER stack. This may be thought of as a "DO SOONER" instruction.
//     ExecYank,
//     /// The Push implementation of the "Y combinator". Inserts beneath the top item of the EXEC stack a new item of the
//     /// form "( EXEC.Y <TopItem> )".
//     ExecY,
//     /// Pushes the cosine of the top item.
//     FloatCos,
//     /// Defines the name on top of the NAME stack as an instruction that will push the top item of the FLOAT stack onto
//     /// the EXEC stack.
//     FloatDefine,
//     /// Pushes the difference of the top two items; that is, the second item minus the top item.
//     FloatDifference,
//     /// Duplicates the top item on the FLOAT stack. Does not pop its argument (which, if it did, would negate the effect
//     /// of the duplication!).
//     FloatDup,
//     /// Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE otherwise.
//     FloatEqual,
//     /// Empties the FLOAT stack.
//     FloatFlush,
//     /// Pushes 1.0 if the top BOOLEAN is TRUE, or 0.0 if the top BOOLEAN is FALSE.
//     FloatFromBoolean,
//     /// Pushes a floating point version of the top INTEGER.
//     FloatFromInteger,
//     /// Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or FALSE otherwise.
//     FloatGreater,
//     /// Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or FALSE otherwise.
//     FloatLess,
//     /// Pushes the maximum of the top two items.
//     FloatMax,
//     /// Pushes the minimum of the top two items.
//     FloatMin,
//     /// Pushes the second stack item modulo the top stack item. If the top item is zero this acts as a NOOP. The modulus
//     /// is computed as the remainder of the quotient, where the quotient has first been truncated toward negative
//     /// infinity. (This is taken from the definition for the generic MOD function in Common Lisp, which is described for
//     /// example at http://www.lispworks.com/reference/HyperSpec/Body/f_mod_r.htm.)
//     FloatModulo,
//     /// Pops the FLOAT stack.
//     FloatPop,
//     /// Pushes the product of the top two items.
//     FloatProduct,
//     /// Pushes the quotient of the top two items; that is, the second item divided by the top item. If the top item is
//     /// zero this acts as a NOOP.
//     FloatQuotient,
//     /// Pushes a newly generated random FLOAT that is greater than or equal to MIN-RANDOM-FLOAT and less than or equal
//     /// to MAX-RANDOM-FLOAT.
//     FloatRand,
//     /// Rotates the top three items on the FLOAT stack, pulling the third item out and pushing it on top. This is
//     /// equivalent to "2 FLOAT.YANK".
//     FloatRot,
//     /// Inserts the top FLOAT "deep" in the stack, at the position indexed by the top INTEGER.
//     FloatShove,
//     /// Pushes the sine of the top item.
//     FloatSin,
//     /// Pushes the stack depth onto the INTEGER stack.
//     FloatStackdepth,
//     /// Pushes the sum of the top two items.
//     FloatSum,
//     /// Swaps the top two BOOLEANs.
//     FloatSwap,
//     /// Pushes the tangent of the top item.
//     FloatTan,
//     /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
//     /// The index is taken from the INTEGER stack.
//     FloatYankDup,
//     /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
//     /// INTEGER stack.
//     FloatYank,
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
