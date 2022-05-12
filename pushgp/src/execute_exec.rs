use crate::{Code, Context, Literal};

#[derive(Clone, Debug, PartialEq)]
pub struct Exec<L: Literal<L>> {
    code: Code<L>,
}

impl<L: Literal<L>> From<Code<L>> for Exec<L> {
    fn from(code: Code<L>) -> Exec<L> {
        Exec {
            code,
        }
    }
}

impl<L: Literal<L>> Into<Code<L>> for Exec<L> {
    fn into(self) -> Code<L> {
        self.code
    }
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
