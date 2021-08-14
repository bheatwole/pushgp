use crate::{Code, Configuration, Instruction};
use fnv::FnvHashMap;
use rand::{thread_rng, RngCore};
use rust_decimal::Decimal;

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
                Code::LiteralName(v) => self.name_stack.push(v),
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
    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::BoolAnd => {
                if self.bool_stack.len() >= 2 {
                    let &a = self.bool_stack.get(self.bool_stack.len() - 1).unwrap();
                    let &b = self.bool_stack.get(self.bool_stack.len() - 2).unwrap();
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
                    let &a = self.bool_stack.get(self.bool_stack.len() - 1).unwrap();
                    let &b = self.bool_stack.get(self.bool_stack.len() - 2).unwrap();
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
                    let &i = self.int_stack.last().unwrap();
                    self.bool_stack.push(i != 0);
                }
            }
            Instruction::BoolNot => {
                if self.bool_stack.len() >= 1 {
                    let &b = self.bool_stack.last().unwrap();
                    self.bool_stack.push(!b);
                }
            }
            Instruction::BoolOr => {
                if self.bool_stack.len() >= 2 {
                    let &a = self.bool_stack.get(self.bool_stack.len() - 1).unwrap();
                    let &b = self.bool_stack.get(self.bool_stack.len() - 2).unwrap();
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
                    let to_append = self.code_stack.pop().unwrap().to_list();
                    let append_to = self.code_stack.pop().unwrap().to_list();
                    let combined = match (append_to, to_append) {
                        (Code::List(mut dst), Code::List(src)) => {
                            dst.extend_from_slice(&src[..]);
                            Code::List(dst)
                        }
                        _ => panic!("should never get here"),
                    };
                    self.code_stack.push(combined);
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
            Instruction::CodeContainer => {}
            Instruction::CodeContains => {}
            Instruction::CodeDefine => {}
            Instruction::CodeDefinition => {}
            Instruction::CodeDiscrepancy => {}
            Instruction::CodeDo => {}
            Instruction::CodeDoN => {}
            Instruction::CodeDoNCount => {}
            Instruction::CodeDoNRange => {}
            Instruction::CodeDoNTimes => {}
            Instruction::CodeDup => {}
            Instruction::CodeEqual => {}
            Instruction::CodeExtract => {}
            Instruction::CodeFlush => {}
            Instruction::CodeFromBoolean => {}
            Instruction::CodeFromFloat => {}
            Instruction::CodeFromInteger => {}
            Instruction::CodeFromName => {}
            Instruction::CodeIf => {}
            Instruction::CodeInsert => {}
            Instruction::CodeInstructions => {}
            Instruction::CodeLength => {}
            Instruction::CodeList => {}
            Instruction::CodeMember => {}
            Instruction::CodeNoop => {}
            Instruction::CodeNth => {}
            Instruction::CodeNthcdr => {}
            Instruction::CodeNull => {}
            Instruction::CodePop => {}
            Instruction::CodePosition => {}
            Instruction::CodeQuote => {}
            Instruction::CodeRand => {}
            Instruction::CodeRot => {}
            Instruction::CodeShove => {}
            Instruction::CodeSize => {}
            Instruction::CodeStackdepth => {}
            Instruction::CodeSubstitute => {}
            Instruction::CodeSwap => {}
            Instruction::CodeYank => {}
            Instruction::CodeYankdup => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Configuration, Context, Instruction};
    use fnv::FnvHashMap;

    #[test]
    fn bool_and() {
        let mut context = Context {
            bool_stack: vec![true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolAnd)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, false, false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_define() {
        let mut context = Context {
            bool_stack: vec![true],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolDefine)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![1234],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.bool_stack.len());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.name_stack.len());
        assert_eq!(1, context.defined_names.len());
        assert_eq!(
            Code::LiteralBool(true),
            *context.defined_names.get(&1234).unwrap()
        );
    }

    #[test]
    fn bool_dup() {
        let mut context = Context {
            bool_stack: vec![true],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolDup)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![1234],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_equal() {
        let mut context = Context {
            bool_stack: vec![true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolEqual)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, false, false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_flush() {
        let mut context = Context {
            bool_stack: vec![true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolFlush)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.bool_stack.len());
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_from_float() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolFromFloat)],
            float_stack: vec![0.0],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![0.0], context.float_stack);

        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolFromFloat)],
            float_stack: vec![0.1],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![0.1], context.float_stack);
    }

    #[test]
    fn bool_from_int() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolFromInt)],
            float_stack: vec![],
            int_stack: vec![0],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![0], context.int_stack);
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolFromInt)],
            float_stack: vec![],
            int_stack: vec![-1],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![-1], context.int_stack);
    }

    #[test]
    fn bool_not() {
        let mut context = Context {
            bool_stack: vec![true],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolNot)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_or() {
        let mut context = Context {
            bool_stack: vec![true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolOr)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, false, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_pop() {
        let mut context = Context {
            bool_stack: vec![true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolPop)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
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
    fn bool_rot() {
        let mut context = Context {
            bool_stack: vec![true, false, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolRot)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false, false, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_shove() {
        let mut context = Context {
            bool_stack: vec![true, false, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolShove)],
            float_stack: vec![],
            int_stack: vec![2],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false, true, false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.int_stack.len());

        let mut context = Context {
            bool_stack: vec![true, true, true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolShove)],
            float_stack: vec![],
            int_stack: vec![2],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, false, true, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.int_stack.len());

        let mut context = Context {
            bool_stack: vec![true, true, true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolShove)],
            float_stack: vec![],
            int_stack: vec![0],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, true, true, false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.int_stack.len());

        let mut context = Context {
            bool_stack: vec![true],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolShove)],
            float_stack: vec![],
            int_stack: vec![1],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.int_stack.len());
    }

    #[test]
    fn bool_stack_depth() {
        let mut context = Context {
            bool_stack: vec![true, false, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolStackDepth)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![true, false, false], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![3], context.int_stack);
    }

    #[test]
    fn bool_swap() {
        let mut context = Context {
            bool_stack: vec![true, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolSwap)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn bool_yank() {
        let mut context = Context {
            bool_stack: vec![false, true, false, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolYank)],
            float_stack: vec![],
            int_stack: vec![2],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false, false, false, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.int_stack.len());
    }

    #[test]
    fn bool_yank_dup() {
        let mut context = Context {
            bool_stack: vec![false, true, false, false],
            code_stack: vec![],
            exec_stack: vec![Code::Instruction(Instruction::BoolYankDup)],
            float_stack: vec![],
            int_stack: vec![2],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(vec![false, true, false, false, true], context.bool_stack);
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(0, context.int_stack.len());
    }

    #[test]
    fn code_append() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![
                Code::List(vec![Code::LiteralInteger(1)]),
                Code::List(vec![Code::LiteralInteger(2)]),
            ],
            exec_stack: vec![Code::Instruction(Instruction::CodeAppend)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(
            vec![Code::List(vec![
                Code::LiteralInteger(1),
                Code::LiteralInteger(2)
            ])],
            context.code_stack
        );
    }

    #[test]
    fn code_atom() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::LiteralInteger(-12)],
            exec_stack: vec![Code::Instruction(Instruction::CodeAtom)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![true], context.bool_stack);
        assert_eq!(vec![Code::LiteralInteger(-12)], context.code_stack);
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::List(vec![])],
            exec_stack: vec![Code::Instruction(Instruction::CodeAtom)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![false], context.bool_stack);
        assert_eq!(vec![Code::List(vec![])], context.code_stack);
    }

    #[test]
    fn code_car() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::LiteralInteger(-12)],
            exec_stack: vec![Code::Instruction(Instruction::CodeCar)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![Code::LiteralInteger(-12)], context.code_stack);
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::List(vec![
                Code::LiteralInteger(-12),
                Code::LiteralInteger(2),
            ])],
            exec_stack: vec![Code::Instruction(Instruction::CodeCar)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![Code::LiteralInteger(-12)], context.code_stack);
    }

    #[test]
    fn code_cdr() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::LiteralInteger(-12)],
            exec_stack: vec![Code::Instruction(Instruction::CodeCdr)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(vec![Code::List(vec![])], context.code_stack);
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::List(vec![
                Code::LiteralInteger(-12),
                Code::LiteralInteger(2),
            ])],
            exec_stack: vec![Code::Instruction(Instruction::CodeCdr)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(
            vec![Code::List(vec![Code::LiteralInteger(2)])],
            context.code_stack
        );
    }

    #[test]
    fn code_cons() {
        let mut context = Context {
            bool_stack: vec![],
            code_stack: vec![Code::LiteralInteger(-12), Code::LiteralBool(true)],
            exec_stack: vec![Code::Instruction(Instruction::CodeCons)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
        };

        assert_eq!(Some(1), context.next());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(
            vec![Code::List(vec![
                Code::LiteralInteger(-12),
                Code::LiteralBool(true),
            ])],
            context.code_stack
        );
    }
}
