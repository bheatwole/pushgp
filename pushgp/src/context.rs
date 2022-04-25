use crate::{Code, Configuration, InstructionTrait, Stack};
use crate::execute_bool::{BoolAnd, BoolStack};
use crate::instruction::execute_instruction;
use crate::instruction_table::InstructionTable;
use fnv::FnvHashMap;
use log::*;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub struct Context {
    pub(crate) bool_stack: Vec<bool>,
    pub(crate) bool_stack_two: Stack<bool>,
    pub(crate) code_stack: Vec<Code>,
    pub(crate) exec_stack: Vec<Code>,
    pub(crate) float_stack: Vec<Decimal>,
    pub(crate) int_stack: Vec<i64>,
    pub(crate) name_stack: Vec<u64>,
    pub(crate) quote_next_name: bool,
    pub(crate) defined_names: FnvHashMap<u64, Code>,
    pub(crate) config: Configuration,

    pub(crate) vtable: InstructionTable<Context>,
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
                Code::LiteralName(v) => {
                    if self.quote_next_name {
                        self.name_stack.push(v);
                        self.quote_next_name = false;
                    } else {
                        match self.defined_names.get(&v) {
                            None => self.name_stack.push(v),
                            Some(code) => self.exec_stack.push(code.clone()),
                        }
                    }
                }
                Code::Instruction(inst) => execute_instruction(self, inst),
                Code::InstructionByName(_name) => {
                    // TODO: Lookup 'name' in a vtable and execute that instruction
                }
            }

            // Return the number of points required to perform that action
            return Some(1);
        }

        // No action was found
        None
    }
}

impl Context {
    pub fn new(config: Configuration) -> Context {
        let mut context = Context {
            bool_stack: vec![],
            bool_stack_two: Stack::new(),
            code_stack: vec![],
            exec_stack: vec![],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            quote_next_name: false,
            defined_names: FnvHashMap::default(),
            config: config,
            vtable: InstructionTable::new(),
        };
        context.vtable.set("BOOL.AND", BoolAnd::execute);
        context
    }

    pub fn clear(&mut self) {
        self.bool_stack.clear();
        self.code_stack.clear();
        self.exec_stack.clear();
        self.float_stack.clear();
        self.int_stack.clear();
        self.name_stack.clear();
        self.quote_next_name = false;
        self.defined_names.clear();
    }

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
}

impl BoolStack for Context {
    fn bool_stack_len(&self) -> usize {
        self.bool_stack_two.len()
    }

    fn bool_stack_pop(&mut self) -> Option<bool> {
        self.bool_stack_two.pop()
    }
    fn bool_stack_push(&mut self, value: bool) {
        self.bool_stack_two.push(value)
    }
    fn get_bool_stack(&mut self) -> &mut Stack<bool> {
        &mut self.bool_stack_two
    }

}

#[cfg(test)]
mod tests {
    use crate::{Code, Configuration, Context, Instruction, Stack};
    use crate::instruction_table::InstructionTable;
    use fnv::FnvHashMap;

    fn load_and_run(src: &str) -> Context {
        let mut context = Context {
            bool_stack: vec![],
            bool_stack_two: Stack::new(),
            code_stack: vec![],
            exec_stack: vec![Code::new(src)],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            quote_next_name: false,
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
            vtable: InstructionTable::new(),
        };
        context.config.set_seed(Some(1));
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
        test_bool_dot_and: ("( TRUE FALSE BOOL.AND )", "( FALSE )"),
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
        test_bool_swap_not_enough: ("( FALSE BOOLSWAP )", "( FALSE )"),
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
        test_float_pop: ("( 5.0 FLOATPOP )", "( )"),
        test_float_product: ("( -5.0 3.0 FLOATPRODUCT )", "( -15.0 )"),
        test_float_quotient: ("( 15.0 3.0 FLOATQUOTIENT )", "( 5.0 )"),
        test_float_quotient_zero: ("( 15.0 0.0 FLOATQUOTIENT )", "( )"),
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
        test_int_define: ("( A 1 INTEGERDEFINE A )", "( 1 )"),
        test_int_difference: ("( 3 1 INTEGERDIFFERENCE )", "( 2 )"),
        test_int_dup: ("( 42 INTEGERDUP )", "( 42 42 )"),
        test_int_equal: ("( 42 0 INTEGEREQUAL )", "( FALSE )"),
        test_int_flush: ("( 1 1 INTEGERFLUSH )", "( )"),
        test_int_fromboolean: ("( TRUE INTEGERFROMBOOLEAN FALSE INTEGERFROMBOOLEAN )", "( 1 0 )"),
        test_int_fromfloat: ("( 5.0 INTEGERFROMFLOAT )", "( 5 )"),
        test_int_greater: ("( 5 3 INTEGERGREATER )", "( TRUE )"),
        test_int_less: ("( 5 3 INTEGERLESS )", "( FALSE )"),
        test_int_max: ("( 5 3 INTEGERMAX )", "( 5 )"),
        test_int_min: ("( -5 3 INTEGERMIN )", "( -5 )"),
        test_int_modulo: ("( -5 3 INTEGERMODULO )", "( -2 )"),
        test_int_modulo_zero: ("( -5 0 INTEGERMODULO )", "( )"),
        test_int_pop: ("( 42 INTEGERPOP )", "( )"),
        test_int_product: ("( -5 3 INTEGERPRODUCT )", "( -15 )"),
        test_int_quotient: ("( 15 3 INTEGERQUOTIENT )", "( 5 )"),
        test_int_quotient_zero: ("( 15 0 INTEGERQUOTIENT )", "( )"),
        test_int_rot: ("( 0 1 2 INTEGERROT )", "( 1 2 0 )"),
        test_int_shove: ("( 1 2 3 2 INTEGERSHOVE )", "( 3 1 2 )"),
        test_int_shove_zero: ("( 1 2 3 0 INTEGERSHOVE )", "( 1 2 3 )"),
        test_int_shove_wrap: ("( 1 2 3 3 INTEGERSHOVE )", "( 1 2 3 )"),
        test_int_stack_depth: ("( 1 2 INTEGERSTACKDEPTH )", "( 1 2 2 )"),
        test_int_sum: ("( 42 7 INTEGERSUM )", "( 49 )"),
        test_int_swap: ("( 1 2 3 INTEGERSWAP )", "( 1 3 2 )"),
        test_int_yank: ("( 1 2 3 4 2 INTEGERYANK )", "( 1 3 4 2 )"),
        test_int_yank_dup: ("( 1 2 3 4 2 INTEGERYANKDUP )", "( 1 2 3 4 2 )"),
        test_name_dup: ("( A NAMEDUP )", "( A A )"),
        test_name_equal: ("( A B NAMEEQUAL )", "( FALSE )"),
        test_name_flush: ("( A B NAMEFLUSH )", "( )"),
        test_name_pop: ("( A NAMEPOP )", "( )"),
        test_name_quote: ("( A 1.0 FLOATDEFINE NAMEQUOTE A )", "( A )"),
        test_name_rot: ("( A B C NAMEROT )", "( B C A )"),
        test_name_shove: ("( A B C 2 NAMESHOVE )", "( C A B )"),
        test_name_shove_zero: ("( A B C 0 NAMESHOVE )", "( A B C )"),
        test_name_shove_wrap: ("( A B C 3 NAMESHOVE )", "( A B C )"),
        test_name_stack_depth: ("( A B NAMESTACKDEPTH )", "( A B 2 )"),
        test_name_swap: ("( A B C NAMESWAP )", "( A C B )"),
        test_name_yank: ("( A B C D 2 NAMEYANK )", "( A C D B )"),
        test_name_yank_dup: ("( A B C D 2 NAMEYANKDUP )", "( A B C D B )"),

    }

    #[test]
    fn bool_rand() {
        let context = load_and_run("( BOOLRAND )");

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
    fn int_rand() {
        let context = load_and_run("( INTEGERRAND )");

        assert_eq!(1, context.int_stack.len());
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn name_rand() {
        let context = load_and_run("( NAMERAND )");

        assert_eq!(1, context.name_stack.len());
        assert_eq!(0, context.exec_stack.len());
    }

    #[test]
    fn name_rand_bound_name() {
        let mut context = load_and_run("( A 1.0 FLOATDEFINE NAMERANDBOUNDNAME )");
        let mut expected = load_and_run("( A )");

        assert_eq!(1, context.name_stack.len());
        assert_eq!(0, context.exec_stack.len());
        assert_eq!(context.name_stack.pop(), expected.name_stack.pop());
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
            bool_stack_two: Stack::new(),
            code_stack: vec![],
            exec_stack: vec![Code::new("CODEINSTRUCTIONS")],
            float_stack: vec![],
            int_stack: vec![],
            name_stack: vec![],
            quote_next_name: false,
            defined_names: FnvHashMap::default(),
            config: Configuration::new(),
            vtable: InstructionTable::new(),
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
