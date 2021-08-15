use crate::{Code, Configuration, Instruction};
use fnv::FnvHashMap;
use log::*;
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
                    self.exec_stack
                        .push(Code::Instruction(Instruction::CodePop));
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
                        let code =
                            Code::List(vec![Code::Instruction(Instruction::IntegerPop), code]);

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
            Instruction::CodePop => {
                if self.code_stack.len() >= 1 {
                    self.code_stack.pop();
                }
            }
            Instruction::CodePosition => {}
            Instruction::CodeQuote => {
                if self.exec_stack.len() >= 1 {
                    self.code_stack.push(self.exec_stack.pop().unwrap());
                }
            }
            Instruction::CodeRand => {}
            Instruction::CodeRot => {}
            Instruction::CodeShove => {}
            Instruction::CodeSize => {}
            Instruction::CodeStackdepth => {}
            Instruction::CodeSubstitute => {}
            Instruction::CodeSwap => {}
            Instruction::CodeYank => {}
            Instruction::CodeYankdup => {}
            Instruction::IntegerPop => {
                if self.int_stack.len() >= 1 {
                    self.int_stack.pop();
                }
            }
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
        context.run(9999999);
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
        test_code_cons: ("( CODEQUOTE TRUE CODEQUOTE ( 1 2 ) CODECONS )", "( CODEQUOTE ( TRUE 1 2 ) )"),
        test_code_container: ("( CODEQUOTE ( 2 ( 3 ( 1 ) ) ( 4 ( 1 ) ) ) CODEQUOTE ( 1 ) CODECONTAINER )", "( CODEQUOTE ( 3 ( 1 ) ) )"),
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
        test_code_pop: ("( CODEQUOTE TRUE CODEPOP )", "( )"),
        test_int_pop: ("( 42 INTEGERPOP )", "( )"),
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
    fn code_quote() {
        let to_run = load_and_run("( CODEQUOTE TRUE )");
        assert_eq!(0, to_run.exec_stack.len());
        assert_eq!(0, to_run.bool_stack.len());
        assert_eq!(vec![Code::LiteralBool(true)], to_run.code_stack);
    }
}
