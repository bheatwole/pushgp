use crate::code::Extraction;
use crate::{Code, Configuration, Instruction, RandomType};
use fnv::FnvHashMap;
use log::*;
use rand::{thread_rng, RngCore};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

#[derive(Debug, PartialEq)]
pub struct Context {
    bool_stack: Vec<bool>,
    code_stack: Vec<Code>,
    exec_stack: Vec<Code>,
    float_stack: Vec<Decimal>,
    int_stack: Vec<i64>,
    name_stack: Vec<u64>,
    defined_names: FnvHashMap<u64, Code>,
    config: Configuration,
}

impl Iterator for Context {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Turn on 'trace' log level to debug execution of a context
        trace!("{:?}", self);

        // Pop the top piece of code from the exec stack and execute it.
        if let Some(code) = self.exec_stack.pop() {
            match code {
                Code::List(mut list) => {
                    // Push the code in reverse order so the first item of the list is the top of stack
                    while let Some(item) = list.pop() {
                        self.exec_stack.push(item);
                    }
                }
                Code::LiteralBool(v) => self.bool_stack.push(v),
                Code::LiteralFloat(v) => self.float_stack.push(v),
                Code::LiteralInteger(v) => self.int_stack.push(v),
                Code::LiteralName(v) => match self.defined_names.get(&v) {
                    None => self.name_stack.push(v),
                    Some(code) => self.exec_stack.push(code.clone()),
                },
                Code::Instruction(inst) => self.execute_instruction(inst),
            }

            // Return the number of points required to perform that action
            return Some(1);
        }

        // No action was found
        None
    }
}

impl Context {
    pub fn run(&mut self, max: usize) -> usize {
        let mut total_count = 0;
        while let Some(count) = self.next() {
            total_count += count;
            if total_count >= max {
                break;
            }
        }
        total_count
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::BoolAnd => {
                if self.bool_stack.len() >= 2 {
                    let a = self.bool_stack.pop().unwrap();
                    let b = self.bool_stack.pop().unwrap();
                    self.bool_stack.push(a && b);
                }
            }
            Instruction::BoolDefine => {
                if self.bool_stack.len() >= 1 && self.name_stack.len() >= 1 {
                    let b = self.bool_stack.pop().unwrap();
                    let n = self.name_stack.pop().unwrap();
                    self.defined_names.insert(n, Code::LiteralBool(b));
                }
            }
            Instruction::BoolDup => {
                if self.bool_stack.len() >= 1 {
                    let &b = self.bool_stack.last().unwrap();
                    self.bool_stack.push(b);
                }
            }
            Instruction::BoolEqual => {
                if self.bool_stack.len() >= 2 {
                    let a = self.bool_stack.pop().unwrap();
                    let b = self.bool_stack.pop().unwrap();
                    self.bool_stack.push(a == b);
                }
            }
            Instruction::BoolFlush => {
                self.bool_stack.clear();
            }
            Instruction::BoolFromFloat => {
                if self.float_stack.len() >= 1 {
                    let f = self.float_stack.pop().unwrap();
                    self.bool_stack.push(!f.is_zero());
                }
            }
            Instruction::BoolFromInt => {
                if self.int_stack.len() >= 1 {
                    let i = self.int_stack.pop().unwrap();
                    self.bool_stack.push(i != 0);
                }
            }
            Instruction::BoolNot => {
                if self.bool_stack.len() >= 1 {
                    let b = self.bool_stack.pop().unwrap();
                    self.bool_stack.push(!b);
                }
            }
            Instruction::BoolOr => {
                if self.bool_stack.len() >= 2 {
                    let a = self.bool_stack.pop().unwrap();
                    let b = self.bool_stack.pop().unwrap();
                    self.bool_stack.push(a || b);
                }
            }
            Instruction::BoolPop => {
                self.bool_stack.pop();
            }
            Instruction::BoolRand => {
                self.bool_stack.push(thread_rng().next_u32() & 1 == 1);
            }
            Instruction::BoolRot => {
                let a = self.bool_stack.pop().unwrap();
                let b = self.bool_stack.pop().unwrap();
                let c = self.bool_stack.pop().unwrap();
                self.bool_stack.push(b);
                self.bool_stack.push(a);
                self.bool_stack.push(c);
            }
            Instruction::BoolShove => {
                if self.bool_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.bool_stack.len());
                    let b = self.bool_stack.pop().unwrap();
                    self.bool_stack.insert(vec_index, b);
                }
            }
            Instruction::BoolStackDepth => {
                self.int_stack.push(self.bool_stack.len() as i64);
            }
            Instruction::BoolSwap => {
                let a = self.bool_stack.pop().unwrap();
                let b = self.bool_stack.pop().unwrap();
                self.bool_stack.push(a);
                self.bool_stack.push(b);
            }
            Instruction::BoolYank => {
                if self.bool_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.bool_stack.len());
                    let b = self.bool_stack.remove(vec_index);
                    self.bool_stack.push(b);
                }
            }
            Instruction::BoolYankDup => {
                if self.bool_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.bool_stack.len());
                    let &b = self.bool_stack.get(vec_index).unwrap();
                    self.bool_stack.push(b);
                }
            }
            Instruction::CodeAppend => {
                if self.code_stack.len() >= 2 {
                    let src = self.code_stack.pop().unwrap().to_list();
                    let mut dst = self.code_stack.pop().unwrap().to_list();
                    dst.extend_from_slice(&src[..]);
                    self.code_stack.push(Code::List(dst));
                }
            }
            Instruction::CodeAtom => {
                if self.code_stack.len() >= 1 {
                    let c = self.code_stack.last().unwrap();
                    self.bool_stack.push(!c.is_list());
                }
            }
            Instruction::CodeCar => {
                if self.code_stack.len() >= 1 {
                    let c = self.code_stack.pop().unwrap();
                    self.code_stack.push(match c {
                        Code::List(list) => {
                            if list.len() > 0 {
                                list[0].clone()
                            } else {
                                Code::List(vec![])
                            }
                        }
                        x => x.clone(),
                    });
                }
            }
            Instruction::CodeCdr => {
                if self.code_stack.len() >= 1 {
                    let c = self.code_stack.pop().unwrap();
                    self.code_stack.push(match c {
                        Code::List(mut list) => {
                            if list.len() > 0 {
                                list.remove(0);
                            }
                            Code::List(list)
                        }
                        _ => Code::List(vec![]),
                    })
                }
            }
            Instruction::CodeCons => {
                if self.code_stack.len() >= 2 {
                    let top = self.code_stack.pop().unwrap();
                    let c = self.code_stack.pop().unwrap();
                    self.code_stack.push(match top {
                        Code::List(mut list) => {
                            list.insert(0, c);
                            Code::List(list)
                        }
                        x => Code::List(vec![c, x]),
                    })
                }
            }
            Instruction::CodeContainer => {
                if self.code_stack.len() >= 2 {
                    let look_for = self.code_stack.pop().unwrap();
                    let look_in = self.code_stack.pop().unwrap();
                    if let Some(code) = look_in.container(&look_for) {
                        self.code_stack.push(code);
                    }
                }
            }
            Instruction::CodeContains => {
                if self.code_stack.len() >= 2 {
                    let look_for = self.code_stack.pop().unwrap();
                    let look_in = self.code_stack.pop().unwrap();
                    self.bool_stack.push(look_in.contains(&look_for));
                }
            }
            Instruction::CodeDefine => {
                if self.code_stack.len() >= 1 && self.name_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    let n = self.name_stack.pop().unwrap();
                    self.defined_names.insert(n, code);
                }
            }
            Instruction::CodeDefinition => {
                if self.name_stack.len() >= 1 {
                    let name = self.name_stack.pop().unwrap();
                    if let Some(code) = self.defined_names.get(&name) {
                        self.code_stack.push(code.clone());
                    }
                }
            }
            Instruction::CodeDiscrepancy => {
                if self.code_stack.len() >= 2 {
                    let a = self.code_stack.pop().unwrap();
                    let b = self.code_stack.pop().unwrap();

                    // Determine all the unique code items along with the count that each appears
                    let a_items = a.discrepancy_items();
                    let b_items = b.discrepancy_items();

                    // Count up all the difference from a to b
                    let mut discrepancy = 0;
                    for (key, &a_count) in a_items.iter() {
                        let b_count = *b_items.get(&key).unwrap_or(&0);
                        discrepancy += (a_count - b_count).abs();
                    }

                    // Count up the difference from b to a for only the keys we didn't use already
                    for (key, &b_count) in b_items.iter() {
                        if a_items.get(&key).is_none() {
                            discrepancy += b_count;
                        }
                    }

                    // Push that value
                    self.int_stack.push(discrepancy);
                }
            }
            Instruction::CodeDo => {
                if self.code_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    self.exec_stack.push(Code::Instruction(Instruction::CodePop));
                    self.exec_stack.push(code.clone());
                    self.code_stack.push(code);
                }
            }
            Instruction::CodeDoN => {
                if self.code_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    self.exec_stack.push(code.clone());
                    self.code_stack.push(code);
                }
            }
            Instruction::CodeDoNCount => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    let count = self.int_stack.pop().unwrap();
                    // NOOP if count <= 0
                    if count <= 0 {
                        self.code_stack.push(code);
                        self.int_stack.push(count);
                    } else {
                        // Turn into DoNRange with (Count - 1) as destination
                        let next = Code::List(vec![
                            Code::LiteralInteger(0),
                            Code::LiteralInteger(count - 1),
                            Code::Instruction(Instruction::CodeQuote),
                            code,
                            Code::Instruction(Instruction::CodeDoNRange),
                        ]);
                        self.exec_stack.push(next);
                    }
                }
            }
            Instruction::CodeDoNRange => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 2 {
                    let code = self.code_stack.pop().unwrap();
                    let dest = self.int_stack.pop().unwrap();
                    let cur = self.int_stack.pop().unwrap();

                    // If we haven't reached the destination yet, push the next iteration onto the stack first.
                    if cur != dest {
                        let increment = if cur < dest { 1 } else { -1 };
                        let next = Code::List(vec![
                            Code::LiteralInteger(cur + increment),
                            Code::LiteralInteger(dest),
                            Code::Instruction(Instruction::CodeQuote),
                            code.clone(),
                            Code::Instruction(Instruction::CodeDoNRange),
                        ]);
                        self.exec_stack.push(next);
                    }

                    // Push the current index onto the int stack so its accessible in the loop
                    self.int_stack.push(cur);

                    // Push the code to run onto the exec stack
                    self.exec_stack.push(code);
                }
            }
            Instruction::CodeDoNTimes => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    let count = self.int_stack.pop().unwrap();

                    // NOOP if count <= 0
                    if count <= 0 {
                        self.code_stack.push(code);
                        self.int_stack.push(count);
                    } else {
                        // The difference between Count and Times is that the 'current index' is not available to
                        // the loop body. Pop that value first
                        let code = Code::List(vec![Code::Instruction(Instruction::IntegerPop), code]);

                        // Turn into DoNRange with (Count - 1) as destination
                        let next = Code::List(vec![
                            Code::LiteralInteger(0),
                            Code::LiteralInteger(count - 1),
                            Code::Instruction(Instruction::CodeQuote),
                            code,
                            Code::Instruction(Instruction::CodeDoNRange),
                        ]);
                        self.exec_stack.push(next);
                    }
                }
            }
            Instruction::CodeDup => {
                if self.code_stack.len() >= 1 {
                    let code = self.code_stack.last().unwrap().clone();
                    self.code_stack.push(code);
                }
            }
            Instruction::CodeEqual => {
                if self.code_stack.len() >= 2 {
                    let a = self.code_stack.pop().unwrap();
                    let b = self.code_stack.pop().unwrap();
                    self.bool_stack.push(a == b);
                }
            }
            Instruction::CodeExtract => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    let total_points = code.points();
                    let point = self.int_stack.pop().unwrap().abs() % total_points;
                    match code.extract_point(point) {
                        Extraction::Extracted(code) => self.code_stack.push(code),
                        Extraction::Used(_) => {
                            panic!("should always be able to extract some code because of abs() and modulo")
                        }
                    }
                }
            }
            Instruction::CodeFlush => {
                self.code_stack.clear();
            }
            Instruction::CodeFromBoolean => {
                if self.bool_stack.len() >= 1 {
                    let value = self.bool_stack.pop().unwrap();
                    self.code_stack.push(Code::LiteralBool(value));
                }
            }
            Instruction::CodeFromFloat => {
                if self.float_stack.len() >= 1 {
                    let value = self.float_stack.pop().unwrap();
                    self.code_stack.push(Code::LiteralFloat(value));
                }
            }
            Instruction::CodeFromInteger => {
                if self.int_stack.len() >= 1 {
                    let value = self.int_stack.pop().unwrap();
                    self.code_stack.push(Code::LiteralInteger(value));
                }
            }
            Instruction::CodeFromName => {
                if self.name_stack.len() >= 1 {
                    let value = self.name_stack.pop().unwrap();
                    self.code_stack.push(Code::LiteralName(value));
                }
            }
            Instruction::CodeIf => {
                if self.code_stack.len() >= 2 && self.bool_stack.len() >= 1 {
                    let false_branch = self.code_stack.pop().unwrap();
                    let true_branch = self.code_stack.pop().unwrap();
                    self.exec_stack.push(if self.bool_stack.pop().unwrap() { true_branch } else { false_branch });
                }
            }
            Instruction::CodeInsert => {
                if self.code_stack.len() >= 2 && self.int_stack.len() >= 1 {
                    let search_in = self.code_stack.pop().unwrap();
                    let replace_with = self.code_stack.pop().unwrap();
                    let total_points = search_in.points();
                    let point = self.int_stack.pop().unwrap().abs() % total_points;
                    self.code_stack.push(search_in.replace_point(point, &replace_with).0);
                }
            }
            Instruction::CodeInstructions => {
                for inst in self.config.allowed_instructions() {
                    self.code_stack.push(Code::Instruction(inst));
                }
            }
            Instruction::CodeLength => {
                if self.code_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    self.int_stack.push(code.len() as i64);
                }
            }
            Instruction::CodeList => {
                if self.code_stack.len() >= 2 {
                    let a = self.code_stack.pop().unwrap();
                    let b = self.code_stack.pop().unwrap();
                    self.code_stack.push(Code::List(vec![b, a]));
                }
            }
            Instruction::CodeMember => {
                if self.code_stack.len() >= 2 {
                    let look_in = self.code_stack.pop().unwrap();
                    let look_for = self.code_stack.pop().unwrap();
                    self.bool_stack.push(look_in.has_member(&look_for));
                }
            }
            Instruction::CodeNoop => {
                // Intentionally blank
            }
            Instruction::CodeNth => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let index = self.int_stack.pop().unwrap().abs() as usize;
                    let mut list = self.code_stack.pop().unwrap().to_list();
                    if 0 == list.len() {
                        self.code_stack.push(Code::List(list));
                    } else {
                        let index = index % list.len();
                        list.truncate(index + 1);
                        self.code_stack.push(list.pop().unwrap());
                    }
                }
            }
            Instruction::CodeNthCdr => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let index = self.int_stack.pop().unwrap().abs() as usize;
                    let mut list = self.code_stack.pop().unwrap().to_list();
                    if 0 == list.len() {
                        self.code_stack.push(Code::List(list));
                    } else {
                        let index = index % list.len();
                        list.remove(index);
                        self.code_stack.push(Code::List(list));
                    }
                }
            }
            Instruction::CodeNull => {
                if self.code_stack.len() >= 1 {
                    // This relies on the behavior that code.len() returns 1 for atoms
                    let code = self.code_stack.pop().unwrap();
                    self.bool_stack.push(0 == code.len());
                }
            }
            Instruction::CodePop => {
                if self.code_stack.len() >= 1 {
                    self.code_stack.pop();
                }
            }
            Instruction::CodePosition => {
                if self.code_stack.len() >= 2 {
                    let look_in = self.code_stack.pop().unwrap();
                    let look_for = self.code_stack.pop().unwrap();
                    match look_in.position_of(&look_for) {
                        Some(index) => self.int_stack.push(index as i64),
                        None => self.int_stack.push(-1),
                    }
                }
            }
            Instruction::CodeQuote => {
                if self.exec_stack.len() >= 1 {
                    self.code_stack.push(self.exec_stack.pop().unwrap());
                }
            }
            Instruction::CodeRand => {
                let names: Vec<u64> = self.defined_names.keys().map(|n| *n).collect();
                self.code_stack.push(self.config.generate_random_code(&names[..]));
            }
            Instruction::CodeRot => {
                let a = self.code_stack.pop().unwrap();
                let b = self.code_stack.pop().unwrap();
                let c = self.code_stack.pop().unwrap();
                self.code_stack.push(b);
                self.code_stack.push(a);
                self.code_stack.push(c);
            }
            Instruction::CodeShove => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.code_stack.len());
                    let b = self.code_stack.pop().unwrap();
                    self.code_stack.insert(vec_index, b);
                }
            }
            Instruction::CodeSize => {
                if self.code_stack.len() >= 1 {
                    let code = self.code_stack.pop().unwrap();
                    self.int_stack.push(code.points());
                }
            }
            Instruction::CodeStackdepth => {
                self.int_stack.push(self.code_stack.len() as i64);
            }
            Instruction::CodeSubstitute => {
                if self.code_stack.len() >= 3 {
                    let look_in = self.code_stack.pop().unwrap();
                    let look_for = self.code_stack.pop().unwrap();
                    let replace_with = self.code_stack.pop().unwrap();
                    self.code_stack.push(look_in.replace(&look_for, &replace_with));
                }
            }
            Instruction::CodeSwap => {
                let a = self.code_stack.pop().unwrap();
                let b = self.code_stack.pop().unwrap();
                self.code_stack.push(a);
                self.code_stack.push(b);
            }
            Instruction::CodeYank => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.code_stack.len());
                    let b = self.code_stack.remove(vec_index);
                    self.code_stack.push(b);
                }
            }
            Instruction::CodeYankDup => {
                if self.code_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.code_stack.len());
                    let b = self.code_stack.get(vec_index).unwrap().clone();
                    self.code_stack.push(b);
                }
            }
            Instruction::ExecDefine => {
                if self.name_stack.len() >= 1 && self.exec_stack.len() >= 1 {
                    let name = self.name_stack.pop().unwrap();
                    let code = self.exec_stack.pop().unwrap();
                    self.defined_names.insert(name, code);
                }
            }
            Instruction::ExecDoNCount => {
                if self.exec_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let code = self.exec_stack.pop().unwrap();
                    let count = self.int_stack.pop().unwrap();
                    // NOOP if count <= 0
                    if count <= 0 {
                        self.exec_stack.push(code);
                        self.int_stack.push(count);
                    } else {
                        // Turn into DoNRange with (Count - 1) as destination
                        let next = Code::List(vec![
                            Code::LiteralInteger(0),
                            Code::LiteralInteger(count - 1),
                            Code::Instruction(Instruction::ExecDoNRange),
                            code,
                        ]);
                        self.exec_stack.push(next);
                    }
                }
            }
            Instruction::ExecDoNRange => {
                if self.exec_stack.len() >= 1 && self.int_stack.len() >= 2 {
                    let code = self.exec_stack.pop().unwrap();
                    let dest = self.int_stack.pop().unwrap();
                    let cur = self.int_stack.pop().unwrap();

                    // If we haven't reached the destination yet, push the next iteration onto the stack first.
                    if cur != dest {
                        let increment = if cur < dest { 1 } else { -1 };
                        let next = Code::List(vec![
                            Code::LiteralInteger(cur + increment),
                            Code::LiteralInteger(dest),
                            Code::Instruction(Instruction::ExecDoNRange),
                            code.clone(),
                        ]);
                        self.exec_stack.push(next);
                    }

                    // Push the current index onto the int stack so its accessible in the loop
                    self.int_stack.push(cur);

                    // Push the code to run onto the exec stack
                    self.exec_stack.push(code);
                }
            }
            Instruction::ExecDoNTimes => {
                if self.exec_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let code = self.exec_stack.pop().unwrap();
                    let count = self.int_stack.pop().unwrap();

                    // NOOP if count <= 0
                    if count <= 0 {
                        self.exec_stack.push(code);
                        self.int_stack.push(count);
                    } else {
                        // The difference between Count and Times is that the 'current index' is not available to
                        // the loop body. Pop that value first
                        let code = Code::List(vec![Code::Instruction(Instruction::IntegerPop), code]);

                        // Turn into DoNRange with (Count - 1) as destination
                        let next = Code::List(vec![
                            Code::LiteralInteger(0),
                            Code::LiteralInteger(count - 1),
                            Code::Instruction(Instruction::ExecDoNRange),
                            code,
                        ]);
                        self.exec_stack.push(next);
                    }
                }
            }
            Instruction::ExecDup => {
                if self.exec_stack.len() >= 1 {
                    let value = self.exec_stack.last().unwrap().clone();
                    self.exec_stack.push(value);
                }
            }
            Instruction::ExecEqual => {
                if self.exec_stack.len() >= 2 {
                    let a = self.exec_stack.pop().unwrap();
                    let b = self.exec_stack.pop().unwrap();
                    self.bool_stack.push(a == b);
                }
            }
            Instruction::ExecFlush => {
                self.exec_stack.clear();
            }
            Instruction::ExecIf => {
                if self.exec_stack.len() >= 2 && self.bool_stack.len() >= 1 {
                    let true_branch = self.exec_stack.pop().unwrap();
                    let false_branch = self.exec_stack.pop().unwrap();
                    self.exec_stack.push(if self.bool_stack.pop().unwrap() { true_branch } else { false_branch });
                }
            }
            Instruction::ExecK => {
                if self.exec_stack.len() >= 2 {
                    let keep = self.exec_stack.pop().unwrap();
                    let _discard = self.exec_stack.pop().unwrap();
                    self.exec_stack.push(keep);
                }
            }
            Instruction::ExecPop => {
                if self.exec_stack.len() >= 1 {
                    let _discard = self.exec_stack.pop().unwrap();
                }
            }
            Instruction::ExecRot => {
                let a = self.exec_stack.pop().unwrap();
                let b = self.exec_stack.pop().unwrap();
                let c = self.exec_stack.pop().unwrap();
                self.exec_stack.push(b);
                self.exec_stack.push(a);
                self.exec_stack.push(c);
            }
            Instruction::ExecShove => {
                if self.exec_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.exec_stack.len());
                    let b = self.exec_stack.pop().unwrap();
                    self.exec_stack.insert(vec_index, b);
                }
            }
            Instruction::ExecStackdepth => {
                self.int_stack.push(self.exec_stack.len() as i64);
            }
            Instruction::ExecSwap => {
                let a = self.exec_stack.pop().unwrap();
                let b = self.exec_stack.pop().unwrap();
                self.exec_stack.push(a);
                self.exec_stack.push(b);
            }
            Instruction::ExecS => {
                if self.exec_stack.len() >= 3 {
                    let a = self.exec_stack.pop().unwrap();
                    let b = self.exec_stack.pop().unwrap();
                    let c = self.exec_stack.pop().unwrap();
                    self.exec_stack.push(Code::List(vec![b, c.clone()]));
                    self.exec_stack.push(c);
                    self.exec_stack.push(a);
                }
            }
            Instruction::ExecYankDup => {
                if self.exec_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.exec_stack.len());
                    let b = self.exec_stack.get(vec_index).unwrap().clone();
                    self.exec_stack.push(b);
                }
            }
            Instruction::ExecYank => {
                if self.exec_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.exec_stack.len());
                    let b = self.exec_stack.remove(vec_index);
                    self.exec_stack.push(b);
                }
            }
            Instruction::ExecY => {
                if self.exec_stack.len() >= 1 {
                    // Get the code we will run on a loop
                    let repeat = self.exec_stack.pop().unwrap();
                    // Construct the looping code
                    let next_exec = Code::List(vec![Code::Instruction(Instruction::ExecY), repeat.clone()]);
                    // Push them back so that we DO and the DO AGAIN
                    self.exec_stack.push(next_exec);
                    self.exec_stack.push(repeat);
                }
            }
            Instruction::FloatCos => {
                if self.float_stack.len() >= 1 {
                    let value = self.float_stack.pop().unwrap();
                    self.float_stack.push(Decimal::from_f64(value.to_f64().unwrap().cos()).unwrap());
                }
            }
            Instruction::FloatDefine => {
                if self.name_stack.len() >= 1 && self.float_stack.len() >= 1 {
                    let name = self.name_stack.pop().unwrap();
                    let value = self.float_stack.pop().unwrap();
                    self.defined_names.insert(name, Code::LiteralFloat(value));
                }
            }
            Instruction::FloatDifference => {
                if self.float_stack.len() >= 2 {
                    let right = self.float_stack.pop().unwrap();
                    let left = self.float_stack.pop().unwrap();
                    self.float_stack.push(left - right);
                }
            }
            Instruction::FloatDup => {
                if self.float_stack.len() >= 1 {
                    let value = self.float_stack.pop().unwrap();
                    self.float_stack.push(value);
                    self.float_stack.push(value);
                }
            }
            Instruction::FloatEqual => {
                if self.float_stack.len() >= 2 {
                    let a = self.float_stack.pop().unwrap();
                    let b = self.float_stack.pop().unwrap();
                    self.bool_stack.push(a == b);
                }
            }
            Instruction::FloatFlush => {
                self.float_stack.clear();
            }
            Instruction::FloatFromBoolean => {
                if self.bool_stack.len() >= 1 {
                    self.float_stack.push(if self.bool_stack.pop().unwrap() {
                        Decimal::new(1, 0)
                    } else {
                        Decimal::new(0, 0)
                    });
                }
            }
            Instruction::FloatFromInteger => {
                if self.int_stack.len() >= 1 {
                    self.float_stack.push(Decimal::new(self.int_stack.pop().unwrap(), 0));
                }
            }
            Instruction::FloatGreater => {
                if self.float_stack.len() >= 2 {
                    let right = self.float_stack.pop().unwrap();
                    let left = self.float_stack.pop().unwrap();
                    self.bool_stack.push(left > right);
                }
            }
            Instruction::FloatLess => {
                if self.float_stack.len() >= 2 {
                    let right = self.float_stack.pop().unwrap();
                    let left = self.float_stack.pop().unwrap();
                    self.bool_stack.push(left < right);
                }
            }
            Instruction::FloatMax => {
                if self.float_stack.len() >= 2 {
                    let a = self.float_stack.pop().unwrap();
                    let b = self.float_stack.pop().unwrap();
                    self.float_stack.push(if a < b { b } else { a });
                }
            }
            Instruction::FloatMin => {
                if self.float_stack.len() >= 2 {
                    let a = self.float_stack.pop().unwrap();
                    let b = self.float_stack.pop().unwrap();
                    self.float_stack.push(if a < b { a } else { b });
                }
            }
            Instruction::FloatModulo => {
                if self.float_stack.len() >= 2 {
                    let bottom = self.float_stack.pop().unwrap();
                    let top = self.float_stack.pop().unwrap();
                    if bottom != Decimal::ZERO {
                        self.float_stack.push(top % bottom);
                    }
                }
            }
            Instruction::FloatPop => {
                self.float_stack.pop();
            }
            Instruction::FloatProduct => {
                if self.float_stack.len() >= 2 {
                    let right = self.float_stack.pop().unwrap();
                    let left = self.float_stack.pop().unwrap();
                    self.float_stack.push(left * right);
                }
            }
            Instruction::FloatQuotient => {
                let bottom = self.float_stack.pop().unwrap();
                let top = self.float_stack.pop().unwrap();
                if bottom != Decimal::ZERO {
                    self.float_stack.push(top / bottom);
                }
            }
            Instruction::FloatRand => {
                self.float_stack.push(match self.config.random_atom_of_type(RandomType::EphemeralFloat) {
                    Code::LiteralFloat(value) => value,
                    _ => panic!("shouldn't ever get anything else"),
                })
            }
            Instruction::FloatRot => {
                let a = self.float_stack.pop().unwrap();
                let b = self.float_stack.pop().unwrap();
                let c = self.float_stack.pop().unwrap();
                self.float_stack.push(b);
                self.float_stack.push(a);
                self.float_stack.push(c);
            }
            Instruction::FloatShove => {
                if self.float_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.float_stack.len());
                    let b = self.float_stack.pop().unwrap();
                    self.float_stack.insert(vec_index, b);
                }
            }
            Instruction::FloatSin => {
                if self.float_stack.len() >= 1 {
                    let value = self.float_stack.pop().unwrap();
                    self.float_stack.push(Decimal::from_f64(value.to_f64().unwrap().sin()).unwrap());
                }
            }
            Instruction::FloatStackdepth => {
                self.int_stack.push(self.float_stack.len() as i64);
            }
            Instruction::FloatSum => {
                if self.float_stack.len() >= 2 {
                    let right = self.float_stack.pop().unwrap();
                    let left = self.float_stack.pop().unwrap();
                    self.float_stack.push(left + right);
                }
            }
            Instruction::FloatSwap => {
                let a = self.float_stack.pop().unwrap();
                let b = self.float_stack.pop().unwrap();
                self.float_stack.push(a);
                self.float_stack.push(b);
            }
            Instruction::FloatTan => {
                if self.float_stack.len() >= 1 {
                    let value = self.float_stack.pop().unwrap();
                    self.float_stack.push(Decimal::from_f64(value.to_f64().unwrap().tan()).unwrap());
                }
            }
            Instruction::FloatYankDup => {
                if self.float_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.float_stack.len());
                    let &b = self.float_stack.get(vec_index).unwrap();
                    self.float_stack.push(b);
                }
            }
            Instruction::FloatYank => {
                if self.float_stack.len() >= 1 && self.int_stack.len() >= 1 {
                    let stack_index = self.int_stack.pop().unwrap();
                    let vec_index = crate::util::stack_to_vec(stack_index, self.float_stack.len());
                    let b = self.float_stack.remove(vec_index);
                    self.float_stack.push(b);
                }
            }
            Instruction::IntegerDefine => {}
            Instruction::IntegerDifference => {}
            Instruction::IntegerDup => {
                if self.int_stack.len() >= 1 {
                    let value = self.int_stack.last().unwrap().clone();
                    self.int_stack.push(value);
                }
            }
            Instruction::IntegerEqual => {
                if self.int_stack.len() >= 2 {
                    let a = self.int_stack.pop().unwrap();
                    let b = self.int_stack.pop().unwrap();
                    self.bool_stack.push(a == b);
                }
            }
            Instruction::IntegerFlush => {}
            Instruction::IntegerFromBoolean => {}
            Instruction::IntegerFromFloat => {}
            Instruction::IntegerGreater => {}
            Instruction::IntegerLess => {}
            Instruction::IntegerMax => {}
            Instruction::IntegerMin => {}
            Instruction::IntegerModulo => {}
            Instruction::IntegerPop => {
                if self.int_stack.len() >= 1 {
                    self.int_stack.pop();
                }
            }
            Instruction::IntegerProduct => {}
            Instruction::IntegerQuotient => {}
            Instruction::IntegerRand => {}
            Instruction::IntegerRot => {}
            Instruction::IntegerShove => {}
            Instruction::IntegerStackdepth => {}
            Instruction::IntegerSum => {
                if self.int_stack.len() >= 2 {
                    let a = self.int_stack.pop().unwrap();
                    let b = self.int_stack.pop().unwrap();
                    self.int_stack.push(a + b);
                }
            }
            Instruction::IntegerSwap => {}
            Instruction::IntegerYankDup => {}
            Instruction::IntegerYank => {}
            Instruction::NameDup => {}
            Instruction::NameEqual => {}
            Instruction::NameFlush => {}
            Instruction::NamePop => {}
            Instruction::NameQuote => {}
            Instruction::NameRandBoundName => {}
            Instruction::NameRand => {}
            Instruction::NameRot => {}
            Instruction::NameShove => {}
            Instruction::NameStackdepth => {}
            Instruction::NameSwap => {}
            Instruction::NameYankDup => {}
            Instruction::NameYank => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Configuration, Context, Instruction};
    use fnv::FnvHashMap;

    fn load_and_run(src: &str) -> Context {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::new(src)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };
        context.config.set_seed(1);
        context.run(1000);
        context
    }

    macro_rules! context_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let mut input_run = load_and_run(input);
                let expected_run = load_and_run(expected);
                // Clear the defined names so that the comparison will work
                input_run.defined_names.clear();
                assert_eq!(input_run, expected_run);
            }
        )*
        }
    }

    context_tests! {
        test_bool_and: ("( TRUE FALSE BOOLAND )", "( FALSE )"),
        test_bool_define: ("( KMu7 TRUE BOOLDEFINE KMu7 )", "( TRUE )"),
        test_bool_dup: ("( TRUE BOOLDUP )", "( TRUE TRUE )"),
        test_bool_equal: ("( TRUE FALSE BOOLEQUAL )", "( FALSE )"),
        test_bool_flush: ("( TRUE FALSE BOOLFLUSH )", "( )"),
        test_bool_fromfloat: ("( 0.0 0.00001 BOOLFROMFLOAT BOOLFROMFLOAT )", "( TRUE FALSE )"),
        test_bool_fromint: ("( 0 1 BOOLFROMINT BOOLFROMINT )", "( TRUE FALSE )"),
        test_bool_not: ("( TRUE BOOLNOT )", "( FALSE )"),
        test_bool_or: ("( TRUE FALSE BOOLOR )", "( TRUE )"),
        test_bool_pop: ("( TRUE FALSE BOOLPOP )", "( TRUE )"),
        test_bool_rot: ("( TRUE FALSE FALSE BOOLROT )", "( FALSE FALSE TRUE )"),
        test_bool_shove: ("( TRUE TRUE FALSE 2 BOOLSHOVE )", "( FALSE TRUE TRUE )"),
        test_bool_shove_zero: ("( TRUE TRUE FALSE 0 BOOLSHOVE )", "( TRUE TRUE FALSE )"),
        test_bool_shove_wrap: ("( TRUE TRUE FALSE 3 BOOLSHOVE )", "( TRUE TRUE FALSE )"),
        test_bool_stack_depth: ("( TRUE FALSE BOOLSTACKDEPTH )", "( TRUE FALSE 2 )"),
        test_bool_swap: ("( FALSE TRUE FALSE BOOLSWAP )", "( FALSE FALSE TRUE )"),
        test_bool_yank: ("( FALSE TRUE FALSE FALSE 2 BOOLYANK )", "( FALSE FALSE FALSE TRUE )"),
        test_bool_yank_dup: ("( FALSE TRUE FALSE FALSE 2 BOOLYANKDUP )", "( FALSE TRUE FALSE FALSE TRUE )"),
        test_code_append: ("( CODEQUOTE 1 CODEQUOTE 2 CODEAPPEND )", "( CODEQUOTE ( 1 2 ) )"),
        test_code_atom_true: ("( CODEQUOTE -12 CODEATOM )", "( CODEQUOTE -12 TRUE )"),
        test_code_atom_false: ("( CODEQUOTE ( ) CODEATOM )", "( CODEQUOTE ( ) FALSE )"),
        test_code_car: ("( CODEQUOTE ( -12 2 ) CODECAR )", "( CODEQUOTE -12 )"),
        test_code_cdr: ("( CODEQUOTE ( -12 2 ) CODECDR )", "( CODEQUOTE ( 2 ) )"),
        test_code_cdr_atom: ("( CODEQUOTE A CODECDR )", "( CODEQUOTE ( ) )"),
        test_code_cons: ("( CODEQUOTE TRUE CODEQUOTE ( 1 2 ) CODECONS )", "( CODEQUOTE ( TRUE 1 2 ) )"),
        test_code_container: ("( CODEQUOTE ( B ( C ( A ) ) ( D ( A ) ) ) CODEQUOTE ( A ) CODECONTAINER )", "( CODEQUOTE ( C ( A ) ) )"),
        test_code_contains_true: ("( CODEQUOTE ( 4 ( 3 ( 2 ) ) ) CODEQUOTE 3 CODECONTAINS )", "( TRUE )"),
        test_code_contains_false: ("( CODEQUOTE ( 4 ( 3 ( 2 ) ) ) CODEQUOTE 1 CODECONTAINS )", "( FALSE )"),
        test_code_contains_list: ("( CODEQUOTE ( 4 ( 3 ( 2 ) ) ) CODEQUOTE ( 2 ) CODECONTAINS )", "( TRUE )"),
        test_code_definition: ("( CODEQUOTE TRUE ANAME ANAME CODEDEFINE CODEDEFINITION )", "( CODEQUOTE TRUE )"),
        test_code_discrepancy_zero: ("( CODEQUOTE ( ANAME ( 3 ( 1 ) ) 1 ( 1 ) ) CODEQUOTE ( ANAME ( 3 ( 1 ) ) 1 ( 1 ) ) CODEDISCREPANCY )", "( 0 )"),
        test_code_discrepancy_multi: ("( CODEQUOTE ( ANAME ( 3 ( 1 ) ) 1 ( 1 ) ) CODEQUOTE 1 CODEDISCREPANCY )", "( 7 )"),
        test_code_do: ("( CODEQUOTE ( FALSE 1 ) CODEDO )", "( FALSE 1 )"),
        test_code_do_pops_last: ("( CODEQUOTE ( CODEQUOTE FALSE ) CODEDO )", "( CODEQUOTE ( CODEQUOTE FALSE ) )"),
        test_code_do_n_count: ("( 4 CODEQUOTE BOOLFROMINT CODEDONCOUNT )", "( FALSE TRUE TRUE TRUE )"),
        test_code_do_n_range_countup: ("( 0 3 CODEQUOTE BOOLFROMINT CODEDONRANGE )", "( FALSE TRUE TRUE TRUE )"),
        test_code_do_n_range_countdown: ("( 3 0 CODEQUOTE BOOLFROMINT CODEDONRANGE )", "( TRUE TRUE TRUE FALSE )"),
        test_code_do_n_times: ("( FALSE TRUE TRUE 2 CODEQUOTE BOOLROT CODEDONTIMES )", "( TRUE FALSE TRUE )"),
        test_code_dup: ("( CODEQUOTE BOOLFROMINT CODEDUP )", "( CODEQUOTE BOOLFROMINT CODEQUOTE BOOLFROMINT )"),
        test_code_equal_true: ("( CODEQUOTE BOOLFROMINT CODEQUOTE BOOLFROMINT CODEEQUAL )", "( TRUE )"),
        test_code_equal_false: ("( CODEQUOTE BOOLFROMINT CODEQUOTE BOOLFROMFLOAT CODEEQUAL )", "( FALSE )"),
        test_code_extract_0: ("( CODEQUOTE ( 1 ( 2 ) ) 0 CODEEXTRACT )", "( CODEQUOTE ( 1 ( 2 ) ) )"),
        test_code_extract_1: ("( CODEQUOTE ( 1 ( 2 ) ) 1 CODEEXTRACT )", "( CODEQUOTE 1 )"),
        test_code_extract_2: ("( CODEQUOTE ( 1 ( 2 ) ) 2 CODEEXTRACT )", "( CODEQUOTE ( 2 ) )"),
        test_code_extract_3: ("( CODEQUOTE ( 1 ( 2 ) ) 3 CODEEXTRACT )", "( CODEQUOTE 2 )"),
        test_code_extract_modulo: ("( CODEQUOTE ( 1 ( 2 ) ) 4 CODEEXTRACT )", "( CODEQUOTE ( 1 ( 2 ) ) )"),
        test_code_flush: ("( CODEQUOTE ( 1 ( 2 ) ) CODEFLUSH )", "( )"),
        test_code_from_boolean: ("( TRUE CODEFROMBOOLEAN )", "( CODEQUOTE TRUE )"),
        test_code_from_float: ("( 1.5 CODEFROMFLOAT )", "( CODEQUOTE 1.5 )"),
        test_code_from_integer: ("( 42 CODEFROMINTEGER )", "( CODEQUOTE 42 )"),
        test_code_from_name: ("( KmU7 CODEFROMNAME )", "( CODEQUOTE KmU7 )"),
        test_code_if_true: ("( TRUE CODEQUOTE TRUENAME CODEQUOTE FALSENAME CODEIF )", "( TRUENAME )"),
        test_code_if_false: ("( FALSE CODEQUOTE TRUENAME CODEQUOTE FALSENAME CODEIF )", "( FALSENAME )"),
        test_code_insert: ("( CODEQUOTE C CODEQUOTE ( A ( B ) ) 2 CODEINSERT )", "( CODEQUOTE ( A C ) )"),
        test_code_length: ("( CODEQUOTE ( A B ( C 1 2 3 ) ) CODELENGTH )", "( 3 )"),
        test_code_list: ("( CODEQUOTE A CODEQUOTE ( B ) CODELIST )", "( CODEQUOTE ( A ( B ) ) )"),
        test_code_member_true: ("( CODEQUOTE A CODEQUOTE ( A ( B ) ) CODEMEMBER )", "( TRUE )"),
        test_code_member_false: ("( CODEQUOTE B CODEQUOTE ( A ( B ) ) CODEMEMBER )", "( FALSE )"),
        test_code_nth: ("( CODEQUOTE ( A ( B ) C ) 2 CODENTH )", "( CODEQUOTE C )"),
        test_code_nth_modulo: ("( CODEQUOTE ( A ( B ) C ) 4 CODENTH )", "( CODEQUOTE ( B ) )"),
        test_code_nth_empty: ("( CODEQUOTE ( ) 3 CODENTH )", "( CODEQUOTE ( ) )"),
        test_code_nth_coerce: ("( CODEQUOTE A 3 CODENTH )", "( CODEQUOTE A )"),
        test_code_nthcdr: ("( CODEQUOTE ( A ( B ) C ) 2 CODENTHCDR )", "( CODEQUOTE ( A ( B ) ) )"),
        test_code_nthcdr_modulo: ("( CODEQUOTE ( A ( B ) C ) 4 CODENTHCDR )", "( CODEQUOTE ( A C ) )"),
        test_code_nthcdr_empty: ("( CODEQUOTE ( ) 3 CODENTHCDR )", "( CODEQUOTE ( ) )"),
        test_code_nthcdr_coerce: ("( CODEQUOTE A 3 CODENTHCDR )", "( CODEQUOTE ( ) )"),
        test_code_null_false: ("( CODEQUOTE ( A ) CODENULL )", "( FALSE )"),
        test_code_null_atom: ("( CODEQUOTE A CODENULL )", "( FALSE )"),
        test_code_null_true: ("( CODEQUOTE ( ) CODENULL )", "( TRUE )"),
        test_code_pop: ("( CODEQUOTE TRUE CODEPOP )", "( )"),
        test_code_position: ("( CODEQUOTE ( B ) CODEQUOTE ( A ( B ) ) CODEPOSITION )", "( 1 )"),
        test_code_position_not_found: ("( CODEQUOTE B CODEQUOTE ( A ( B ) ) CODEPOSITION )", "( -1 )"),
        test_code_position_self: ("( CODEQUOTE B CODEQUOTE B CODEPOSITION )", "( 0 )"),
        test_code_rot: ("( CODEQUOTE A CODEQUOTE B CODEQUOTE C CODEROT )", "( CODEQUOTE B CODEQUOTE C CODEQUOTE A )"),
        test_code_shove: ("( CODEQUOTE A CODEQUOTE B CODEQUOTE C 2 CODESHOVE )", "( CODEQUOTE C CODEQUOTE A CODEQUOTE B )"),
        test_code_shove_zero: ("( CODEQUOTE A CODEQUOTE B CODEQUOTE C 0 CODESHOVE )", "( CODEQUOTE A CODEQUOTE B CODEQUOTE C )"),
        test_code_shove_wrap: ("( CODEQUOTE A CODEQUOTE B CODEQUOTE C 3 CODESHOVE )", "( CODEQUOTE A CODEQUOTE B CODEQUOTE C )"),
        test_code_size: ("( CODEQUOTE ( A ( B ) C ) CODESIZE )", "( 5 )"),
        test_code_stack_depth: ("( CODEQUOTE A CODEQUOTE B CODESTACKDEPTH )", "( CODEQUOTE A CODEQUOTE B 2 )"),
        test_code_substitute: ("( CODEQUOTE A CODEQUOTE ( B ) CODEQUOTE ( A ( B ) ( A ( B ) ) ) CODESUBSTITUTE )", "( CODEQUOTE ( A A ( A A ) ) )"),
        test_code_swap: ("( CODEQUOTE A CODEQUOTE B CODESWAP )", "( CODEQUOTE B CODEQUOTE A )"),
        test_code_yank: ("( CODEQUOTE A CODEQUOTE B CODEQUOTE C CODEQUOTE D 2 CODEYANK )", "( CODEQUOTE A CODEQUOTE C CODEQUOTE D CODEQUOTE B )"),
        test_code_yank_dup: ("( CODEQUOTE A CODEQUOTE B CODEQUOTE C CODEQUOTE D 2 CODEYANKDUP )", "( CODEQUOTE A CODEQUOTE B CODEQUOTE C CODEQUOTE D CODEQUOTE B )"),
        test_exec_define: ("( A TRUE EXECDEFINE A )", "( TRUE )"),
        test_exec_do_n_count: ("( 4 EXECDONCOUNT BOOLFROMINT )", "( FALSE TRUE TRUE TRUE )"),
        test_exec_do_n_range_countup: ("( 0 3 EXECDONRANGE BOOLFROMINT )", "( FALSE TRUE TRUE TRUE )"),
        test_exec_do_n_range_countdown: ("( 3 0 EXECDONRANGE BOOLFROMINT )", "( TRUE TRUE TRUE FALSE )"),
        test_exec_do_n_times: ("( FALSE TRUE TRUE 2 EXECDONTIMES BOOLROT )", "( TRUE FALSE TRUE )"),
        test_exec_dup: ("( EXECDUP 5 )", "( 5 5 )"),
        test_exec_equal: ("( EXECEQUAL 5 5 )", "( TRUE )"),
        test_exec_flush: ("( EXECFLUSH 5 5 )", "( )"),
        test_exec_if_true: ("( TRUE EXECIF TRUENAME FALSENAME )", "( TRUENAME )"),
        test_exec_if_false: ("( FALSE EXECIF TRUENAME FALSENAME )", "( FALSENAME )"),
        test_exec_k: ("( EXECK TRUENAME FALSENAME )", "( TRUENAME )"),
        test_exec_pop: ("( EXECPOP 5 )", "( )"),
        test_exec_rot: ("( EXECROT A B C )", "( C A B )"),
        test_exec_shove: ("( 2 EXECSHOVE A B C )", "( B C A )"),
        test_exec_shove_zero: ("( 0 EXECSHOVE A B C )", "( A B C )"),
        test_exec_shove_wrap: ("( 3 EXECSHOVE A B C )", "( A B C )"),
        test_exec_stack_depth: ("( EXECSTACKDEPTH A B )", "( A B 2 )"),
        test_exec_swap: ("( EXECSWAP A B )", "( B A )"),
        test_exec_s: ("( EXECS A B C )", "( A C ( B C ) )"),
        test_exec_yank: ("( 2 EXECYANK A B C D )", "( C A B D )"),
        test_exec_yank_dup: ("( 2 EXECYANKDUP A B C D )", "( C A B C D )"),
        test_exec_y: ("( 0 EXECY ( INTEGERDUP 2 INTEGEREQUAL EXECIF EXECPOP ( INTEGERDUP 1 INTEGERSUM ) ) )", "( 0 1 2 )"),
        test_float_cos: ("( 1.0 FLOATCOS )", "( 0.54030230586814 )"),
        test_float_define: ("( A 1.0 FLOATDEFINE A )", "( 1.0 )"),
        test_float_difference: ("( 3.0 1.0 FLOATDIFFERENCE )", "( 2.0 )"),
        test_float_dup: ("( 1.0 FLOATDUP )", "( 1.0 1.0 )"),
        test_float_equal: ("( 1.0 1.0 FLOATEQUAL )", "( TRUE )"),
        test_float_flush: ("( 1.0 1.0 FLOATFLUSH )", "( )"),
        test_float_fromboolean: ("( TRUE FLOATFROMBOOLEAN FALSE FLOATFROMBOOLEAN )", "( 1.0 0.0 )"),
        test_float_frominteger: ("( 5 FLOATFROMINTEGER )", "( 5.0 )"),
        test_float_greater: ("( 5.0 3.0 FLOATGREATER )", "( TRUE )"),
        test_float_less: ("( 5.0 3.0 FLOATLESS )", "( FALSE )"),
        test_float_max: ("( 5.0 3.0 FLOATMAX )", "( 5.0 )"),
        test_float_min: ("( -5.0 3.0 FLOATMIN )", "( -5.0 )"),
        test_float_modulo: ("( -5.0 3.0 FLOATMODULO )", "( -2.0 )"),
        test_float_modulo_zero: ("( -5.0 0.0 FLOATMODULO )", "( )"),
        test_float_product: ("( -5.0 3.0 FLOATPRODUCT )", "( -15.0 )"),
        test_float_quotient: ("( 15.0 3.0 FLOATQUOTIENT )", "( 5.0 )"),
        test_float_quotient_zero: ("( 15.0 0.0 FLOATQUOTIENT )", "( )"),
        test_float_pop: ("( 5.0 FLOATPOP )", "( )"),
        test_float_rot: ("( 0.0 1.0 2.0 FLOATROT )", "( 1.0 2.0 0.0 )"),
        test_float_shove: ("( 1.0 2.0 3.0 2 FLOATSHOVE )", "( 3.0 1.0 2.0 )"),
        test_float_shove_zero: ("( 1.0 2.0 3.0 0 FLOATSHOVE )", "( 1.0 2.0 3.0 )"),
        test_float_shove_wrap: ("( 1.0 2.0 3.0 3 FLOATSHOVE )", "( 1.0 2.0 3.0 )"),
        test_float_sin: ("( 1.0 FLOATSIN )", "( 0.841470984807897 )"),
        test_float_stack_depth: ("( 1.0 2.0 FLOATSTACKDEPTH )", "( 1.0 2.0 2 )"),
        test_float_sum: ("( 1.5 2.5 FLOATSUM )", "( 4.0 )"),
        test_float_swap: ("( 1.0 2.0 3.0 FLOATSWAP )", "( 1.0 3.0 2.0 )"),
        test_float_tan: ("( 1.0 FLOATTAN )", "( 1.557407724654902 )"),
        test_float_yank: ("( 1.0 2.0 3.0 4.0 2 FLOATYANK )", "( 1.0 3.0 4.0 2.0 )"),
        test_float_yank_dup: ("( 1.0 2.0 3.0 4.0 2 FLOATYANKDUP )", "( 1.0 2.0 3.0 4.0 2.0 )"),
        test_int_dup: ("( 42 INTEGERDUP )", "( 42 42 )"),
        test_int_equal: ("( 42 0 INTEGEREQUAL )", "( FALSE )"),
        test_int_pop: ("( 42 INTEGERPOP )", "( )"),
        test_int_sum: ("( 42 7 INTEGERSUM )", "( 49 )"),
    }

    #[test]
    fn bool_rand() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolRand)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(1, context.bool_stack.len());
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn float_rand() {
        let context = load_and_run("( FLOATRAND )");

        assert_eq!(1, context.float_stack.len());
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn code_quote() {
        let to_run = load_and_run("( CODEQUOTE TRUE )");
        assert_eq!(0, to_run.exec_stack.len());
        assert_eq!(0, to_run.bool_stack.len());
        assert_eq!(vec![Code::LiteralBool(true)], to_run.code_stack);
    }

    #[test]
    fn code_instructions() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::new("CODEINSTRUCTIONS")],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        // These instructions should appear in the output in any order
        context.config.set_instruction_weight(Instruction::BoolAnd, 1);
        context.config.set_instruction_weight(Instruction::CodeAppend, 5);

        // This instruction should not appear because it's weight is zero
        context.config.set_instruction_weight(Instruction::CodeCdr, 0);

        context.run(1000);
        assert_eq!(2, context.code_stack.len());
        assert!(context.code_stack.contains(&Code::Instruction(Instruction::BoolAnd)));
        assert!(context.code_stack.contains(&Code::Instruction(Instruction::CodeAppend)));
    }
}
