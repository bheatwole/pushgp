use crate::*;
use pushgp_macros::*;

pub type Exec<L> = Code<L>;

pub trait ContextHasExecStack<L: LiteralEnum<L>> {
    fn exec(&self) -> &Stack<Exec<L>>;
}

instruction! {
    /// Defines the name on top of the NAME stack as an instruction that will push the top item of the EXEC stack back
    /// onto the EXEC stack.
    #[stack(Exec)]
    fn define(context: &mut Context) {}
}

instruction! {
    /// An iteration instruction that performs a loop (the body of which is taken from the EXEC stack) the number of
    /// times indicated by the INTEGER argument, pushing an index (which runs from zero to one less than the number of
    /// iterations) onto the INTEGER stack prior to each execution of the loop body. This is similar to CODE.DO*COUNT
    /// except that it takes its code argument from the EXEC stack. This should be implemented as a macro that expands
    /// into a call to EXEC.DO*RANGE. EXEC.DO*COUNT takes a single INTEGER argument (the number of times that the loop
    /// will be executed) and a single EXEC argument (the body of the loop). If the provided INTEGER argument is
    /// negative or zero then this becomes a NOOP. Otherwise it expands into:
    ///   ( 0 <1 - IntegerArg> EXEC.DO*RANGE <ExecArg> )
    #[stack(Exec)]
    fn do_n_count(context: &mut Context) {}
}

instruction! {
    /// An iteration instruction that executes the top item on the EXEC stack a number of times that depends on the top
    /// two integers, while also pushing the loop counter onto the INTEGER stack for possible access during the
    /// execution of the body of the loop. This is similar to CODE.DO*COUNT except that it takes its code argument from
    /// the EXEC stack. The top integer is the "destination index" and the second integer is the "current index."
    /// First the code and the integer arguments are saved locally and popped. Then the integers are compared. If the
    /// integers are equal then the current index is pushed onto the INTEGER stack and the code (which is the "body" of
    /// the loop) is pushed onto the EXEC stack for subsequent execution. If the integers are not equal then the current
    /// index will still be pushed onto the INTEGER stack but two items will be pushed onto the EXEC stack -- first a
    /// recursive call to EXEC.DO*RANGE (with the same code and destination index, but with a current index that has
    /// been either incremented or decremented by 1 to be closer to the destination index) and then the body code. Note
    /// that the range is inclusive of both endpoints; a call with integer arguments 3 and 5 will cause its body to be
    /// executed 3 times, with the loop counter having the values 3, 4, and 5. Note also that one can specify a loop
    /// that "counts down" by providing a destination index that is less than the specified current index.
    #[stack(Exec)]
    fn do_n_range(context: &mut Context) {}
}

instruction! {
    /// Like EXEC.DO*COUNT but does not push the loop counter. This should be implemented as a macro that expands into
    /// EXEC.DO*RANGE, similarly to the implementation of EXEC.DO*COUNT, except that a call to INTEGER.POP should be
    /// tacked on to the front of the loop body code in the call to EXEC.DO*RANGE. This call to INTEGER.POP will remove
    /// the loop counter, which will have been pushed by EXEC.DO*RANGE, prior to the execution of the loop body.
    #[stack(Exec)]
    fn do_n_times(context: &mut Context) {}
}

instruction! {
    /// Duplicates the top item on the EXEC stack. Does not pop its argument (which, if it did, would negate the effect
    /// of the duplication!). This may be thought of as a "DO TWICE" instruction.
    #[stack(Exec)]
    fn dup(context: &mut Context) {}
}

instruction! {
    /// Pushes TRUE if the top two items on the EXEC stack are equal, or FALSE otherwise.
    #[stack(Exec)]
    fn equal(context: &mut Context) {}
}

instruction! {
    /// Empties the EXEC stack. This may be thought of as a "HALT" instruction.
    #[stack(Exec)]
    fn flush(context: &mut Context) {}
}

instruction! {
    /// If the top item of the BOOLEAN stack is TRUE then this removes the second item on the EXEC stack, leaving the
    /// first item to be executed. If it is false then it removes the first item, leaving the second to be executed.
    /// This is similar to CODE.IF except that it operates on the EXEC stack. This acts as a NOOP unless there are at
    /// least two items on the EXEC stack and one item on the BOOLEAN stack.
    #[stack(Exec)]
    fn if(context: &mut Context) {}
}

instruction! {
    /// The Push implementation of the "K combinator". Removes the second item on the EXEC stack.
    #[stack(Exec)]
    fn k(context: &mut Context) {}
}

instruction! {
    /// Pops the EXEC stack. This may be thought of as a "DONT" instruction.
    #[stack(Exec)]
    fn pop(context: &mut Context) {}
}

instruction! {
    /// Rotates the top three items on the EXEC stack, pulling the third item out and pushing it on top. This is
    /// equivalent to "2 EXEC.YANK".
    #[stack(Exec)]
    fn rot(context: &mut Context) {}
}

instruction! {
    /// Inserts the top EXEC item "deep" in the stack, at the position indexed by the top INTEGER. This may be thought
    /// of as a "DO LATER" instruction.
    #[stack(Exec)]
    fn shove(context: &mut Context) {}
}

instruction! {
    /// Pushes the stack depth onto the INTEGER stack.
    #[stack(Exec)]
    fn stack_depth(context: &mut Context) {}
}

instruction! {
    /// Swaps the top two items on the EXEC stack.
    #[stack(Exec)]
    fn swap(context: &mut Context) {}
}

instruction! {
    /// The Push implementation of the "S combinator". Pops 3 items from the EXEC stack, which we will call A, B, and C
    /// (with A being the first one popped). Then pushes a list containing B and C back onto the EXEC stack, followed by
    /// another instance of C, followed by another instance of A.
    #[stack(Exec)]
    fn s(context: &mut Context) {}
}

instruction! {
    /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    /// The index is taken from the INTEGER stack.
    #[stack(Exec)]
    fn yank_dup(context: &mut Context) {}
}

instruction! {
    /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
    /// INTEGER stack. This may be thought of as a "DO SOONER" instruction.
    #[stack(Exec)]
    fn yank(context: &mut Context) {}
}

instruction! {
    /// The Push implementation of the "Y combinator". Inserts beneath the top item of the EXEC stack a new item of the
    /// form "( EXEC.Y <TopItem> )".
    #[stack(Exec)]
    fn y(context: &mut Context) {}
}

// pub fn execute_execdefine(context: &mut Context) {
//     if context.name_stack.len() >= 1 && context.exec_stack.len() >= 1 {
//         let name = context.name_stack.pop().unwrap();
//         let code = context.exec_stack.pop().unwrap();
//         context.defined_names.insert(name, code);
//     }
// }

// pub fn execute_execdoncount(context: &mut Context) {
//     if context.exec_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let code = context.exec_stack.pop().unwrap();
//         let count = context.int_stack.pop().unwrap();
//         // NOOP if count <= 0
//         if count <= 0 {
//             context.exec_stack.push(code);
//             context.int_stack.push(count);
//         } else {
//             // Turn into DoNRange with (Count - 1) as destination
//             let next = Code::List(vec![
//                 Code::LiteralInteger(0),
//                 Code::LiteralInteger(count - 1),
//                 Code::Instruction(Instruction::ExecDoNRange),
//                 code,
//             ]);
//             context.exec_stack.push(next);
//         }
//     }
// }

// pub fn execute_execdonrange(context: &mut Context) {
//     if context.exec_stack.len() >= 1 && context.int_stack.len() >= 2 {
//         let code = context.exec_stack.pop().unwrap();
//         let dest = context.int_stack.pop().unwrap();
//         let cur = context.int_stack.pop().unwrap();

//         // If we haven't reached the destination yet, push the next iteration onto the stack first.
//         if cur != dest {
//             let increment = if cur < dest { 1 } else { -1 };
//             let next = Code::List(vec![
//                 Code::LiteralInteger(cur + increment),
//                 Code::LiteralInteger(dest),
//                 Code::Instruction(Instruction::ExecDoNRange),
//                 code.clone(),
//             ]);
//             context.exec_stack.push(next);
//         }

//         // Push the current index onto the int stack so its accessible in the loop
//         context.int_stack.push(cur);

//         // Push the code to run onto the exec stack
//         context.exec_stack.push(code);
//     }
// }

// pub fn execute_execdontimes(context: &mut Context) {
//     if context.exec_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let code = context.exec_stack.pop().unwrap();
//         let count = context.int_stack.pop().unwrap();

//         // NOOP if count <= 0
//         if count <= 0 {
//             context.exec_stack.push(code);
//             context.int_stack.push(count);
//         } else {
//             // The difference between Count and Times is that the 'current index' is not available to
//             // the loop body. Pop that value first
//             let code = Code::List(vec![Code::Instruction(Instruction::IntegerPop), code]);

//             // Turn into DoNRange with (Count - 1) as destination
//             let next = Code::List(vec![
//                 Code::LiteralInteger(0),
//                 Code::LiteralInteger(count - 1),
//                 Code::Instruction(Instruction::ExecDoNRange),
//                 code,
//             ]);
//             context.exec_stack.push(next);
//         }
//     }
// }

// pub fn execute_execdup(context: &mut Context) {
//     if context.exec_stack.len() >= 1 {
//         let value = context.exec_stack.last().unwrap().clone();
//         context.exec_stack.push(value);
//     }
// }

// pub fn execute_execequal(context: &mut Context) {
//     if context.exec_stack.len() >= 2 {
//         let a = context.exec_stack.pop().unwrap();
//         let b = context.exec_stack.pop().unwrap();
//         context.bool_stack.push(a == b);
//     }
// }

// pub fn execute_execflush(context: &mut Context) {
//     context.exec_stack.clear();
// }

// pub fn execute_execif(context: &mut Context) {
//     if context.exec_stack.len() >= 2 && context.bool_stack.len() >= 1 {
//         let true_branch = context.exec_stack.pop().unwrap();
//         let false_branch = context.exec_stack.pop().unwrap();
//         context.exec_stack.push(if context.bool_stack.pop().unwrap() { true_branch } else { false_branch });
//     }
// }

// pub fn execute_execk(context: &mut Context) {
//     if context.exec_stack.len() >= 2 {
//         let keep = context.exec_stack.pop().unwrap();
//         let _discard = context.exec_stack.pop().unwrap();
//         context.exec_stack.push(keep);
//     }
// }

// pub fn execute_execpop(context: &mut Context) {
//     if context.exec_stack.len() >= 1 {
//         let _discard = context.exec_stack.pop().unwrap();
//     }
// }

// pub fn execute_execrot(context: &mut Context) {
//     let a = context.exec_stack.pop().unwrap();
//     let b = context.exec_stack.pop().unwrap();
//     let c = context.exec_stack.pop().unwrap();
//     context.exec_stack.push(b);
//     context.exec_stack.push(a);
//     context.exec_stack.push(c);
// }

// pub fn execute_execshove(context: &mut Context) {
//     if context.exec_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.exec_stack.len());
//         let b = context.exec_stack.pop().unwrap();
//         context.exec_stack.insert(vec_index, b);
//     }
// }

// pub fn execute_execstackdepth(context: &mut Context) {
//     context.int_stack.push(context.exec_stack.len() as i64);
// }

// pub fn execute_execswap(context: &mut Context) {
//     let a = context.exec_stack.pop().unwrap();
//     let b = context.exec_stack.pop().unwrap();
//     context.exec_stack.push(a);
//     context.exec_stack.push(b);
// }

// pub fn execute_execs(context: &mut Context) {
//     if context.exec_stack.len() >= 3 {
//         let a = context.exec_stack.pop().unwrap();
//         let b = context.exec_stack.pop().unwrap();
//         let c = context.exec_stack.pop().unwrap();
//         context.exec_stack.push(Code::List(vec![b, c.clone()]));
//         context.exec_stack.push(c);
//         context.exec_stack.push(a);
//     }
// }

// pub fn execute_execyankdup(context: &mut Context) {
//     if context.exec_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.exec_stack.len());
//         let b = context.exec_stack.get(vec_index).unwrap().clone();
//         context.exec_stack.push(b);
//     }
// }

// pub fn execute_execyank(context: &mut Context) {
//     if context.exec_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.exec_stack.len());
//         let b = context.exec_stack.remove(vec_index);
//         context.exec_stack.push(b);
//     }
// }

// pub fn execute_execy(context: &mut Context) {
//     if context.exec_stack.len() >= 1 {
//         // Get the code we will run on a loop
//         let repeat = context.exec_stack.pop().unwrap();
//         // Construct the looping code
//         let next_exec = Code::List(vec![Code::Instruction(Instruction::ExecY), repeat.clone()]);
//         // Push them back so that we DO and the DO AGAIN
//         context.exec_stack.push(next_exec);
//         context.exec_stack.push(repeat);
//     }
// }
