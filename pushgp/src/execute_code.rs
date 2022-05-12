use crate::code::Extraction;
use crate::{Code, Context};

// pub fn execute_codeappend(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let src = context.code_stack.pop().unwrap().to_list();
//         let mut dst = context.code_stack.pop().unwrap().to_list();
//         dst.extend_from_slice(&src[..]);
//         context.code_stack.push(Code::List(dst));
//     }
// }

// pub fn execute_codeatom(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let c = context.code_stack.last().unwrap();
//         context.bool_stack.push(!c.is_list());
//     }
// }

// pub fn execute_codecar(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let c = context.code_stack.pop().unwrap();
//         context.code_stack.push(match c {
//             Code::List(list) => {
//                 if list.len() > 0 {
//                     list[0].clone()
//                 } else {
//                     Code::List(vec![])
//                 }
//             }
//             x => x.clone(),
//         });
//     }
// }

// pub fn execute_codecdr(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let c = context.code_stack.pop().unwrap();
//         context.code_stack.push(match c {
//             Code::List(mut list) => {
//                 if list.len() > 0 {
//                     list.remove(0);
//                 }
//                 Code::List(list)
//             }
//             _ => Code::List(vec![]),
//         })
//     }
// }

// pub fn execute_codecons(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let top = context.code_stack.pop().unwrap();
//         let c = context.code_stack.pop().unwrap();
//         context.code_stack.push(match top {
//             Code::List(mut list) => {
//                 list.insert(0, c);
//                 Code::List(list)
//             }
//             x => Code::List(vec![c, x]),
//         })
//     }
// }
// pub fn execute_codecontainer(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let look_for = context.code_stack.pop().unwrap();
//         let look_in = context.code_stack.pop().unwrap();
//         if let Some(code) = look_in.container(&look_for) {
//             context.code_stack.push(code);
//         }
//     }
// }

// pub fn execute_codecontains(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let look_for = context.code_stack.pop().unwrap();
//         let look_in = context.code_stack.pop().unwrap();
//         context.bool_stack.push(look_in.contains(&look_for));
//     }
// }

// pub fn execute_codedefine(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.name_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         let n = context.name_stack.pop().unwrap();
//         context.defined_names.insert(n, code);
//     }
// }

// pub fn execute_codedefinition(context: &mut Context) {
//     if context.name_stack.len() >= 1 {
//         let name = context.name_stack.pop().unwrap();
//         if let Some(code) = context.defined_names.get(&name) {
//             context.code_stack.push(code.clone());
//         }
//     }
// }

// pub fn execute_codediscrepancy(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let a = context.code_stack.pop().unwrap();
//         let b = context.code_stack.pop().unwrap();

//         // Determine all the unique code items along with the count that each appears
//         let a_items = a.discrepancy_items();
//         let b_items = b.discrepancy_items();

//         // Count up all the difference from a to b
//         let mut discrepancy = 0;
//         for (key, &a_count) in a_items.iter() {
//             let b_count = *b_items.get(&key).unwrap_or(&0);
//             discrepancy += (a_count - b_count).abs();
//         }

//         // Count up the difference from b to a for only the keys we didn't use already
//         for (key, &b_count) in b_items.iter() {
//             if a_items.get(&key).is_none() {
//                 discrepancy += b_count;
//             }
//         }

//         // Push that value
//         context.int_stack.push(discrepancy);
//     }
// }

// pub fn execute_codedo(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         context.exec_stack.push(Code::Instruction(Instruction::CodePop));
//         context.exec_stack.push(code.clone());
//         context.code_stack.push(code);
//     }
// }

// pub fn execute_codedon(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         context.exec_stack.push(code.clone());
//         context.code_stack.push(code);
//     }
// }

// pub fn execute_codedoncount(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         let count = context.int_stack.pop().unwrap();
//         // NOOP if count <= 0
//         if count <= 0 {
//             context.code_stack.push(code);
//             context.int_stack.push(count);
//         } else {
//             // Turn into DoNRange with (Count - 1) as destination
//             let next = Code::List(vec![
//                 Code::LiteralInteger(0),
//                 Code::LiteralInteger(count - 1),
//                 Code::Instruction(Instruction::CodeQuote),
//                 code,
//                 Code::Instruction(Instruction::CodeDoNRange),
//             ]);
//             context.exec_stack.push(next);
//         }
//     }
// }

// pub fn execute_codedonrange(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 2 {
//         let code = context.code_stack.pop().unwrap();
//         let dest = context.int_stack.pop().unwrap();
//         let cur = context.int_stack.pop().unwrap();

//         // If we haven't reached the destination yet, push the next iteration onto the stack first.
//         if cur != dest {
//             let increment = if cur < dest { 1 } else { -1 };
//             let next = Code::List(vec![
//                 Code::LiteralInteger(cur + increment),
//                 Code::LiteralInteger(dest),
//                 Code::Instruction(Instruction::CodeQuote),
//                 code.clone(),
//                 Code::Instruction(Instruction::CodeDoNRange),
//             ]);
//             context.exec_stack.push(next);
//         }

//         // Push the current index onto the int stack so its accessible in the loop
//         context.int_stack.push(cur);

//         // Push the code to run onto the exec stack
//         context.exec_stack.push(code);
//     }
// }

// pub fn execute_codedontimes(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         let count = context.int_stack.pop().unwrap();

//         // NOOP if count <= 0
//         if count <= 0 {
//             context.code_stack.push(code);
//             context.int_stack.push(count);
//         } else {
//             // The difference between Count and Times is that the 'current index' is not available to
//             // the loop body. Pop that value first
//             let code = Code::List(vec![Code::Instruction(Instruction::IntegerPop), code]);

//             // Turn into DoNRange with (Count - 1) as destination
//             let next = Code::List(vec![
//                 Code::LiteralInteger(0),
//                 Code::LiteralInteger(count - 1),
//                 Code::Instruction(Instruction::CodeQuote),
//                 code,
//                 Code::Instruction(Instruction::CodeDoNRange),
//             ]);
//             context.exec_stack.push(next);
//         }
//     }
// }

// pub fn execute_codedup(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let code = context.code_stack.last().unwrap().clone();
//         context.code_stack.push(code);
//     }
// }

// pub fn execute_codeequal(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let a = context.code_stack.pop().unwrap();
//         let b = context.code_stack.pop().unwrap();
//         context.bool_stack.push(a == b);
//     }
// }

// pub fn execute_codeextract(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         let total_points = code.points();
//         let point = context.int_stack.pop().unwrap().abs() % total_points;
//         match code.extract_point(point) {
//             Extraction::Extracted(code) => context.code_stack.push(code),
//             Extraction::Used(_) => {
//                 panic!("should always be able to extract some code because of abs() and modulo")
//             }
//         }
//     }
// }

// pub fn execute_codeflush(context: &mut Context) {
//     context.code_stack.clear();
// }

// pub fn execute_codefromboolean(context: &mut Context) {
//     if context.bool_stack.len() >= 1 {
//         let value = context.bool_stack.pop().unwrap();
//         context.code_stack.push(Code::LiteralBool(value));
//     }
// }

// pub fn execute_codefromfloat(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         let value = context.float_stack.pop().unwrap();
//         context.code_stack.push(Code::LiteralFloat(value));
//     }
// }

// pub fn execute_codefrominteger(context: &mut Context) {
//     if context.int_stack.len() >= 1 {
//         let value = context.int_stack.pop().unwrap();
//         context.code_stack.push(Code::LiteralInteger(value));
//     }
// }

// pub fn execute_codefromname(context: &mut Context) {
//     if context.name_stack.len() >= 1 {
//         let value = context.name_stack.pop().unwrap();
//         context.code_stack.push(Code::LiteralName(value));
//     }
// }

// pub fn execute_codeif(context: &mut Context) {
//     if context.code_stack.len() >= 2 && context.bool_stack.len() >= 1 {
//         let false_branch = context.code_stack.pop().unwrap();
//         let true_branch = context.code_stack.pop().unwrap();
//         context.exec_stack.push(if context.bool_stack.pop().unwrap() { true_branch } else { false_branch });
//     }
// }

// pub fn execute_codeinsert(context: &mut Context) {
//     if context.code_stack.len() >= 2 && context.int_stack.len() >= 1 {
//         let search_in = context.code_stack.pop().unwrap();
//         let replace_with = context.code_stack.pop().unwrap();
//         let total_points = search_in.points();
//         let point = context.int_stack.pop().unwrap().abs() % total_points;
//         context.code_stack.push(search_in.replace_point(point, &replace_with).0);
//     }
// }

// pub fn execute_codeinstructions(context: &mut Context) {
//     for inst in context.config.allowed_instructions() {
//         context.code_stack.push(Code::Instruction(inst));
//     }
// }

// pub fn execute_codelength(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         context.int_stack.push(code.len() as i64);
//     }
// }

// pub fn execute_codelist(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let a = context.code_stack.pop().unwrap();
//         let b = context.code_stack.pop().unwrap();
//         context.code_stack.push(Code::List(vec![b, a]));
//     }
// }

// pub fn execute_codemember(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let look_in = context.code_stack.pop().unwrap();
//         let look_for = context.code_stack.pop().unwrap();
//         context.bool_stack.push(look_in.has_member(&look_for));
//     }
// }

// pub fn execute_codenoop(_context: &mut Context) {
//     // Intentionally blank
// }

// pub fn execute_codenth(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let index = context.int_stack.pop().unwrap().abs() as usize;
//         let mut list = context.code_stack.pop().unwrap().to_list();
//         if 0 == list.len() {
//             context.code_stack.push(Code::List(list));
//         } else {
//             let index = index % list.len();
//             list.truncate(index + 1);
//             context.code_stack.push(list.pop().unwrap());
//         }
//     }
// }

// pub fn execute_codenthcdr(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let index = context.int_stack.pop().unwrap().abs() as usize;
//         let mut list = context.code_stack.pop().unwrap().to_list();
//         if 0 == list.len() {
//             context.code_stack.push(Code::List(list));
//         } else {
//             let index = index % list.len();
//             list.remove(index);
//             context.code_stack.push(Code::List(list));
//         }
//     }
// }

// pub fn execute_codenull(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         // This relies on the behavior that code.len() returns 1 for atoms
//         let code = context.code_stack.pop().unwrap();
//         context.bool_stack.push(0 == code.len());
//     }
// }

// pub fn execute_codepop(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         context.code_stack.pop();
//     }
// }

// pub fn execute_codeposition(context: &mut Context) {
//     if context.code_stack.len() >= 2 {
//         let look_in = context.code_stack.pop().unwrap();
//         let look_for = context.code_stack.pop().unwrap();
//         match look_in.position_of(&look_for) {
//             Some(index) => context.int_stack.push(index as i64),
//             None => context.int_stack.push(-1),
//         }
//     }
// }

// pub fn execute_codequote(context: &mut Context) {
//     if context.exec_stack.len() >= 1 {
//         context.code_stack.push(context.exec_stack.pop().unwrap());
//     }
// }

// pub fn execute_coderand(context: &mut Context) {
//     let names: Vec<u64> = context.defined_names.keys().map(|n| *n).collect();
//     context.code_stack.push(context.config.generate_random_code(&names[..]));
// }

// pub fn execute_coderot(context: &mut Context) {
//     let a = context.code_stack.pop().unwrap();
//     let b = context.code_stack.pop().unwrap();
//     let c = context.code_stack.pop().unwrap();
//     context.code_stack.push(b);
//     context.code_stack.push(a);
//     context.code_stack.push(c);
// }

// pub fn execute_codeshove(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.code_stack.len());
//         let b = context.code_stack.pop().unwrap();
//         context.code_stack.insert(vec_index, b);
//     }
// }

// pub fn execute_codesize(context: &mut Context) {
//     if context.code_stack.len() >= 1 {
//         let code = context.code_stack.pop().unwrap();
//         context.int_stack.push(code.points());
//     }
// }

// pub fn execute_codestackdepth(context: &mut Context) {
//     context.int_stack.push(context.code_stack.len() as i64);
// }

// pub fn execute_codesubstitute(context: &mut Context) {
//     if context.code_stack.len() >= 3 {
//         let look_in = context.code_stack.pop().unwrap();
//         let look_for = context.code_stack.pop().unwrap();
//         let replace_with = context.code_stack.pop().unwrap();
//         context.code_stack.push(look_in.replace(&look_for, &replace_with));
//     }
// }

// pub fn execute_codeswap(context: &mut Context) {
//     let a = context.code_stack.pop().unwrap();
//     let b = context.code_stack.pop().unwrap();
//     context.code_stack.push(a);
//     context.code_stack.push(b);
// }

// pub fn execute_codeyank(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.code_stack.len());
//         let b = context.code_stack.remove(vec_index);
//         context.code_stack.push(b);
//     }
// }

// pub fn execute_codeyankdup(context: &mut Context) {
//     if context.code_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.code_stack.len());
//         let b = context.code_stack.get(vec_index).unwrap().clone();
//         context.code_stack.push(b);
//     }
// }
