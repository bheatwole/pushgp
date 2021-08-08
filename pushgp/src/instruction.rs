use crate::{Code, InstructionType};
use nom::{branch::alt, bytes::complete::tag, character::complete::space0, IResult};
use std::fmt::Display;

trait NomTag {
    fn nom_tag(input: &str) -> IResult<&str, Instruction>;
}

#[derive(Clone, Copy, Debug, Display, Eq, Hash, NomTag, PartialEq)]
pub enum Instruction {
    // Pushes the logical AND of the top two BOOLEANs.
    BoolAnd,
    // Defines the name on top of the NAME stack as an instruction that will push the top item of the BOOLEAN stack onto
    // the EXEC stack
    BoolDefine,
    // Duplicates the top item on the BOOLEAN stack. Does not pop its argument (which, if it did, would negate the
    // effect of the duplication!)
    BoolDup,
    // Pushes TRUE if the top two BOOLEANs are equal, or FALSE otherwise
    BoolEqual,
    // Empties the BOOLEAN stack
    BoolFlush,
    // Pushes FALSE if the top FLOAT is 0.0, or TRUE otherwise
    BoolFromFloat,
    // Pushes FALSE if the top INTEGER is 0, or TRUE otherwise
    BoolFromInt,
    // Pushes the logical NOT of the top BOOLEAN
    BoolNot,
    // Pushes the logical OR of the top two BOOLEANs
    BoolOr,
    // Pops the BOOLEAN stack
    BoolPop,
    // Pushes a random BOOLEAN
    BoolRand,
    // Rotates the top three items on the BOOLEAN stack, pulling the third item out and pushing it on top. This is
    // equivalent to "2 BOOLEAN.YANK"
    BoolRot,
    // Inserts the top BOOLEAN "deep" in the stack, at the position indexed by the top INTEGER
    BoolShove,
    // Pushes the stack depth onto the INTEGER stack
    BoolStackDepth,
    // Swaps the top two BOOLEANs
    BoolSwap,
    // Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
    // INTEGER stack
    BoolYank,
    // Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    // The index is taken from the INTEGER stack
    BoolYankDup,
    // Pushes the result of appending the top two pieces of code. If one of the pieces of code is a single instruction
    // or literal (that is, something not surrounded by parentheses) then it is surrounded by parentheses first.
    CodeAppend,
    // Pushes TRUE onto the BOOLEAN stack if the top piece of code is a single instruction or a literal, and FALSE
    // otherwise (that is, if it is something surrounded by parentheses).
    CodeAtom,
    // Pushes the first item of the list on top of the CODE stack. For example, if the top piece of code is "( A B )"
    // then this pushes "A" (after popping the argument). If the code on top of the stack is not a list then this has
    // no effect. The name derives from the similar Lisp function; a more generic name would be "FIRST".
    CodeCar,
    // Pushes a version of the list from the top of the CODE stack without its first element. For example, if the top
    // piece of code is "( A B )" then this pushes "( B )" (after popping the argument). If the code on top of the stack
    // is not a list then this pushes the empty list ("( )"). The name derives from the similar Lisp function; a more
    // generic name would be "REST".
    CodeCdr,
    // Pushes the result of "consing" (in the Lisp sense) the second stack item onto the first stack item (which is
    // coerced to a list if necessary). For example, if the top piece of code is "( A B )" and the second piece of code
    // is "X" then this pushes "( X A B )" (after popping the argument).
    CodeCons,
    // Pushes the "container" of the second CODE stack item within the first CODE stack item onto the CODE stack. If
    // second item contains the first anywhere (i.e. in any nested list) then the container is the smallest sub-list
    // that contains but is not equal to the first instance. For example, if the top piece of code is
    // "( B ( C ( A ) ) ( D ( A ) ) )" and the second piece of code is "( A )" then this pushes ( C ( A ) ). Pushes an
    // empty list if there is no such container.
    CodeContainer,
    // Pushes TRUE on the BOOLEAN stack if the second CODE stack item contains the first CODE stack item anywhere (e.g. in a sub-list).
    CodeContains,
    // Defines the name on top of the NAME stack as an instruction that will push the top item of the CODE stack onto the EXEC stack.
    CodeDefine,
    // Pushes the definition associated with the top NAME on the NAME stack (if any) onto the CODE stack. This extracts the definition for inspection/manipulation, rather than for immediate execution (although it may then be executed with a call to CODE.DO or a similar instruction).
    CodeDefinition,
    // Pushes a measure of the discrepancy between the top two CODE stack items onto the INTEGER stack. This will be zero if the top two items are equivalent, and will be higher the 'more different' the items are from one another. The calculation is as follows: 1. Construct a list of all of the unique items in both of the lists (where uniqueness is determined by equalp). Sub-lists and atoms all count as items. 2. Initialize the result to zero. 3. For each unique item increment the result by the difference between the number of occurrences of the item in the two pieces of code. 4. Push the result.
    CodeDiscrepancy,
    // Recursively invokes the interpreter on the program on top of the CODE stack. After evaluation the CODE stack is popped; normally this pops the program that was just executed, but if the expression itself manipulates the stack then this final pop may end up popping something else.
    CodeDo,
    // Like CODE.DO but pops the stack before, rather than after, the recursive execution.
    CodeDoN,
    // An iteration instruction that performs a loop (the body of which is taken from the CODE stack) the number of times indicated by the INTEGER argument, pushing an index (which runs from zero to one less than the number of iterations) onto the INTEGER stack prior to each execution of the loop body. This should be implemented as a macro that expands into a call to CODE.DO*RANGE. CODE.DO*COUNT takes a single INTEGER argument (the number of times that the loop will be executed) and a single CODE argument (the body of the loop). If the provided INTEGER argument is negative or zero then this becomes a NOOP. Otherwise it expands into: ( 0 <1 - IntegerArg> CODE.QUOTE <CodeArg> CODE.DO*RANGE )
    CodeDoNCount,
    // An iteration instruction that executes the top item on the CODE stack a number of times that depends on the top two integers, while also pushing the loop counter onto the INTEGER stack for possible access during the execution of the body of the loop. The top integer is the "destination index" and the second integer is the "current index." First the code and the integer arguments are saved locally and popped. Then the integers are compared. If the integers are equal then the current index is pushed onto the INTEGER stack and the code (which is the "body" of the loop) is pushed onto the EXEC stack for subsequent execution. If the integers are not equal then the current index will still be pushed onto the INTEGER stack but two items will be pushed onto the EXEC stack -- first a recursive call to CODE.DO*RANGE (with the same code and destination index, but with a current index that has been either incremented or decremented by 1 to be closer to the destination index) and then the body code. Note that the range is inclusive of both endpoints; a call with integer arguments 3 and 5 will cause its body to be executed 3 times, with the loop counter having the values 3, 4, and 5. Note also that one can specify a loop that "counts down" by providing a destination index that is less than the specified current index.
    CodeDoNRange,
    // Like CODE.DO*COUNT but does not push the loop counter. This should be implemented as a macro that expands into CODE.DO*RANGE, similarly to the implementation of CODE.DO*COUNT, except that a call to INTEGER.POP should be tacked on to the front of the loop body code in the call to CODE.DO*RANGE. This call to INTEGER.POP will remove the loop counter, which will have been pushed by CODE.DO*RANGE, prior to the execution of the loop body.
    CodeDoNTimes,
    // Duplicates the top item on the CODE stack. Does not pop its argument (which, if it did, would negate the effect of the duplication!).
    CodeDup,
    // Pushes TRUE if the top two pieces of CODE are equal, or FALSE otherwise.
    CodeEqual,
    // Pushes the sub-expression of the top item of the CODE stack that is indexed by the top item of the INTEGER stack. The indexing here counts "points," where each parenthesized expression and each literal/instruction is considered a point, and it proceeds in depth first order. The entire piece of code is at index 0; if it is a list then the first item in the list is at index 1, etc. The integer used as the index is taken modulo the number of points in the overall expression (and its absolute value is taken in case it is negative) to ensure that it is within the meaningful range.
    CodeExtract,
    // Empties the CODE stack.
    CodeFlush,
    // Pops the BOOLEAN stack and pushes the popped item (TRUE or FALSE) onto the CODE stack.
    CodeFromBoolean,
    // Pops the FLOAT stack and pushes the popped item onto the CODE stack.
    CodeFromFloat,
    // Pops the INTEGER stack and pushes the popped integer onto the CODE stack.
    CodeFromInteger,
    // Pops the NAME stack and pushes the popped item onto the CODE stack.
    CodeFromName,
    // If the top item of the BOOLEAN stack is TRUE this recursively executes the second item of the CODE stack; otherwise it recursively executes the first item of the CODE stack. Either way both elements of the CODE stack (and the BOOLEAN value upon which the decision was made) are popped.
    CodeIf,
    // Pushes the result of inserting the second item of the CODE stack into the first item, at the position indexed by the top item of the INTEGER stack (and replacing whatever was there formerly). The indexing is computed as in CODE.EXTRACT.
    CodeInsert,
    // Pushes a list of all active instructions in the interpreter's current configuration.
    CodeInstructions,
    // Pushes the length of the top item on the CODE stack onto the INTEGER stack. If the top item is not a list then this pushes a 1. If the top item is a list then this pushes the number of items in the top level of the list; that is, nested lists contribute only 1 to this count, no matter what they contain.
    CodeLength,
    // Pushes a list of the top two items of the CODE stack onto the CODE stack.
    CodeList,
    // Pushes TRUE onto the BOOLEAN stack if the second item of the CODE stack is a member of the first item (which is coerced to a list if necessary). Pushes FALSE onto the BOOLEAN stack otherwise.
    CodeMember,
    // Does nothing.
    CodeNoop,
    // Pushes the nth element of the expression on top of the CODE stack (which is coerced to a list first if necessary). If the expression is an empty list then the result is an empty list. N is taken from the INTEGER stack and is taken modulo the length of the expression into which it is indexing.
    CodeNth,
    // Pushes the nth "CDR" (in the Lisp sense) of the expression on top of the CODE stack (which is coerced to a list first if necessary). If the expression is an empty list then the result is an empty list. N is taken from the INTEGER stack and is taken modulo the length of the expression into which it is indexing. A "CDR" of a list is the list without its first element.
    CodeNthcdr,
    // Pushes TRUE onto the BOOLEAN stack if the top item of the CODE stack is an empty list, or FALSE otherwise.
    CodeNull,
    // Pops the CODE stack.
    CodePop,
    // Pushes onto the INTEGER stack the position of the second item on the CODE stack within the first item (which is coerced to a list if necessary). Pushes -1 if no match is found.
    CodePosition,
    // Specifies that the next expression submitted for execution will instead be pushed literally onto the CODE stack. This can be implemented by moving the top item on the EXEC stack onto the CODE stack.
    CodeQuote,
    // Pushes a newly-generated random program onto the CODE stack. The limit for the size of the expression is taken from the INTEGER stack; to ensure that it is in the appropriate range this is taken modulo the value of the MAX-POINTS-IN-RANDOM-EXPRESSIONS parameter and the absolute value of the result is used.
    CodeRand,
    // Rotates the top three items on the CODE stack, pulling the third item out and pushing it on top. This is equivalent to "2 CODE.YANK".
    CodeRot,
    // Inserts the top piece of CODE "deep" in the stack, at the position indexed by the top INTEGER.
    CodeShove,
    // Pushes the number of "points" in the top piece of CODE onto the INTEGER stack. Each instruction, literal, and pair of parentheses counts as a point.
    CodeSize,
    // Pushes the stack depth onto the INTEGER stack.
    CodeStackdepth,
    // Pushes the result of substituting the third item on the code stack for the second item in the first item. As of this writing this is implemented only in the Lisp implementation, within which it relies on the Lisp "subst" function. As such, there are several problematic possibilities; for example "dotted-lists" can result in certain cases with empty-list arguments. If any of these problematic possibilities occurs the stack is left unchanged.
    CodeSubstitute,
    // Swaps the top two pieces of CODE.
    CodeSwap,
    // Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the INTEGER stack.
    CodeYank,
    // Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item. The index is taken from the INTEGER stack.
    CodeYankdup,
}

impl Instruction {
    pub fn types(&self) -> Vec<InstructionType> {
        match self {
            Instruction::BoolAnd => vec![InstructionType::Bool],
            Instruction::BoolDefine => vec![InstructionType::Bool, InstructionType::Name],
            Instruction::BoolDup => vec![InstructionType::Bool],
            Instruction::BoolEqual => vec![InstructionType::Bool],
            Instruction::BoolFlush => vec![InstructionType::Bool],
            Instruction::BoolFromFloat => vec![InstructionType::Bool, InstructionType::Float],
            Instruction::BoolFromInt => vec![InstructionType::Bool, InstructionType::Int],
            Instruction::BoolNot => vec![InstructionType::Bool],
            Instruction::BoolOr => vec![InstructionType::Bool],
            Instruction::BoolPop => vec![InstructionType::Bool],
            Instruction::BoolRand => vec![InstructionType::Bool],
            Instruction::BoolRot => vec![InstructionType::Bool],
            Instruction::BoolShove => vec![InstructionType::Bool, InstructionType::Int],
            Instruction::BoolStackDepth => vec![InstructionType::Bool, InstructionType::Int],
            Instruction::BoolSwap => vec![InstructionType::Bool],
            Instruction::BoolYank => vec![InstructionType::Bool, InstructionType::Int],
            Instruction::BoolYankDup => vec![InstructionType::Bool, InstructionType::Int],
            Instruction::CodeAppend => vec![InstructionType::Code],
            Instruction::CodeAtom => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodeCar => vec![InstructionType::Code],
            Instruction::CodeCdr => vec![InstructionType::Code],
            Instruction::CodeCons => vec![InstructionType::Code],
            Instruction::CodeContainer => vec![InstructionType::Code],
            Instruction::CodeContains => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodeDefine => vec![InstructionType::Code, InstructionType::Name],
            Instruction::CodeDefinition => vec![InstructionType::Code, InstructionType::Name],
            Instruction::CodeDiscrepancy => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeDo => vec![InstructionType::Code],
            Instruction::CodeDoN => vec![InstructionType::Code],
            Instruction::CodeDoNCount => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeDoNRange => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeDoNTimes => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeDup => vec![InstructionType::Code],
            Instruction::CodeEqual => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodeExtract => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeFlush => vec![InstructionType::Code],
            Instruction::CodeFromBoolean => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodeFromFloat => vec![InstructionType::Code, InstructionType::Float],
            Instruction::CodeFromInteger => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeFromName => vec![InstructionType::Code, InstructionType::Name],
            Instruction::CodeIf => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodeInsert => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeInstructions => vec![InstructionType::Code],
            Instruction::CodeLength => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeList => vec![InstructionType::Code],
            Instruction::CodeMember => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodeNoop => vec![InstructionType::Code],
            Instruction::CodeNth => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeNthcdr => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeNull => vec![InstructionType::Bool, InstructionType::Code],
            Instruction::CodePop => vec![InstructionType::Code],
            Instruction::CodePosition => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeQuote => vec![InstructionType::Code],
            Instruction::CodeRand => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeRot => vec![InstructionType::Code],
            Instruction::CodeShove => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeSize => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeStackdepth => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeSubstitute => vec![InstructionType::Code],
            Instruction::CodeSwap => vec![InstructionType::Code],
            Instruction::CodeYank => vec![InstructionType::Code, InstructionType::Int],
            Instruction::CodeYankdup => vec![InstructionType::Code, InstructionType::Int],
        }
    }
}

pub fn parse_code_instruction(input: &str) -> IResult<&str, Code> {
    let (input, inst) = Instruction::nom_tag(input)?;
    Ok((input, Code::Instruction(inst)))
}

#[cfg(test)]
mod tests {
    use crate::Instruction;

    #[test]
    fn instruction_display() {
        assert_eq!("CODENTH", format!("{}", Instruction::CodeNth));
    }
}
