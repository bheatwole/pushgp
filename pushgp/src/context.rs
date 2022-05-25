use crate::configuration::RandomLiteralFunction;
use log::*;
use std::fmt::Debug;

pub trait Context: Debug {
    fn clear(&mut self);
    fn next(&mut self) -> Option<usize>;
    fn all_instruction_names(&self) -> Vec<String>;
    fn run_random_literal_function<RealLiteralType>(&mut self, func: RandomLiteralFunction<RealLiteralType>) -> RealLiteralType;

    /// Seeds the random number with a specific value so that you may get repeatable results. Passing `None` will seed
    /// the generator with a truly random value ensuring unique results.
    fn set_seed(&mut self, seed: Option<u64>);

    fn run(&mut self, max: usize) -> usize {
        trace!("{:?}", self);
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

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::default_code_gen::{BaseContext, BaseLiteral, BaseLiteralParser};

    fn load_and_run(src: &str) -> BaseContext {
        let weights = vec![];
        let config = Configuration::<BaseLiteral>::new(Some(1), 100, &weights[..]);
        let mut context = BaseContext::new(config, new_instruction_table_with_all_instructions());
        let code = BaseLiteralParser::parse(src);
        context.exec().push(code);
        context.run(1000);

        // Reset the random seed after every run
        context.set_seed(Some(1));

        context
    }

    macro_rules! context_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected, mut expected_definitions): (&str, &str, Vec<(&str, &str)>) = $value;
                let input_run = load_and_run(input);
                let expected_run = load_and_run(expected);

                // Add the expected definitions to the expected run
                for (name, src) in expected_definitions.drain(..) {
                    let code = BaseLiteralParser::parse(src);
                    expected_run.name().define_name(name.to_owned(), code);
                }
                assert_eq!(input_run, expected_run);
            }
        )*
        }
    }

    context_tests! {
        test_bool_and: ("( TRUE FALSE BOOL.AND )", "( FALSE )", vec![]),
        test_bool_define: ("( KMu7 TRUE BOOL.DEFINE KMu7 )", "( TRUE )", vec![("KMu7", "TRUE")]),
        test_bool_dup: ("( TRUE BOOL.DUP )", "( TRUE TRUE )", vec![]),
        test_bool_equal: ("( TRUE FALSE BOOL.EQUAL )", "( FALSE )", vec![]),
        test_bool_flush: ("( TRUE FALSE BOOL.FLUSH )", "( )", vec![]),
        test_bool_fromfloat: ("( 0.0 0.00001 BOOL.FROMFLOAT BOOL.FROMFLOAT )", "( TRUE FALSE )", vec![]),
        test_bool_fromint: ("( 0 1 BOOL.FROMINT BOOL.FROMINT )", "( TRUE FALSE )", vec![]),
        test_bool_not: ("( TRUE BOOL.NOT )", "( FALSE )", vec![]),
        test_bool_or: ("( TRUE FALSE BOOL.OR )", "( TRUE )", vec![]),
        test_bool_pop: ("( TRUE FALSE BOOL.POP )", "( TRUE )", vec![]),
        test_bool_rand: ("( BOOL.RAND )", "( TRUE )", vec![]),
        test_bool_rot: ("( TRUE FALSE FALSE BOOL.ROT )", "( FALSE FALSE TRUE )", vec![]),
        test_bool_shove: ("( TRUE TRUE FALSE 2 BOOL.SHOVE )", "( FALSE TRUE TRUE )", vec![]),
        test_bool_shove_zero: ("( TRUE TRUE FALSE 0 BOOL.SHOVE )", "( TRUE TRUE FALSE )", vec![]),
        test_bool_shove_wrap: ("( TRUE TRUE FALSE 3 BOOL.SHOVE )", "( TRUE TRUE FALSE )", vec![]),
        test_bool_stack_depth: ("( TRUE FALSE BOOL.STACKDEPTH )", "( TRUE FALSE 2 )", vec![]),
        test_bool_swap: ("( FALSE TRUE FALSE BOOL.SWAP )", "( FALSE FALSE TRUE )", vec![]),
        test_bool_swap_not_enough: ("( FALSE BOOL.SWAP )", "( FALSE )", vec![]),
        test_bool_yank: ("( FALSE TRUE FALSE FALSE 2 BOOL.YANK )", "( FALSE FALSE FALSE TRUE )", vec![]),
        test_bool_yank_dup: ("( FALSE TRUE FALSE FALSE 2 BOOL.YANKDUP )", "( FALSE TRUE FALSE FALSE TRUE )", vec![]),
        test_code_append: ("( CODE.QUOTE 1 CODE.QUOTE 2 CODE.APPEND )", "( CODE.QUOTE ( 1 2 ) )", vec![]),
        test_code_atom_true: ("( CODE.QUOTE -12 CODE.ATOM )", "( CODE.QUOTE -12 TRUE )", vec![]),
        test_code_atom_false: ("( CODE.QUOTE ( ) CODE.ATOM )", "( CODE.QUOTE ( ) FALSE )", vec![]),
        test_code_car: ("( CODE.QUOTE ( -12 2 ) CODE.CAR )", "( CODE.QUOTE -12 )", vec![]),
        test_code_cdr: ("( CODE.QUOTE ( -12 2 ) CODE.CDR )", "( CODE.QUOTE ( 2 ) )", vec![]),
        test_code_cdr_atom: ("( CODE.QUOTE A CODE.CDR )", "( CODE.QUOTE ( ) )", vec![]),
        test_code_cons: ("( CODE.QUOTE TRUE CODE.QUOTE ( 1 2 ) CODE.CONS )", "( CODE.QUOTE ( TRUE 1 2 ) )", vec![]),
        test_code_container: ("( CODE.QUOTE ( B ( C ( A ) ) ( D ( A ) ) ) CODE.QUOTE ( A ) CODE.CONTAINER )", "( CODE.QUOTE ( C ( A ) ) )", vec![]),
        test_code_contains_true: ("( CODE.QUOTE ( 4 ( 3 ( 2 ) ) ) CODE.QUOTE 3 CODE.CONTAINS )", "( TRUE )", vec![]),
        test_code_contains_false: ("( CODE.QUOTE ( 4 ( 3 ( 2 ) ) ) CODE.QUOTE 1 CODE.CONTAINS )", "( FALSE )", vec![]),
        test_code_contains_list: ("( CODE.QUOTE ( 4 ( 3 ( 2 ) ) ) CODE.QUOTE ( 2 ) CODE.CONTAINS )", "( TRUE )", vec![]),
        test_code_define: ("( SOMENAME CODE.QUOTE TRUE CODE.DEFINE )", "( )", vec![("SOMENAME", "TRUE")]),
        test_code_definition: ("( CODE.QUOTE TRUE ANAME ANAME CODE.DEFINE CODE.DEFINITION )", "( CODE.QUOTE TRUE )", vec![("ANAME", "TRUE")]),
        test_code_discrepancy_zero: ("( CODE.QUOTE ( ANAME ( 3 ( 1 ) ) 1 ( 1 ) ) CODE.QUOTE ( ANAME ( 3 ( 1 ) ) 1 ( 1 ) ) CODE.DISCREPANCY )", "( 0 )", vec![]),
        test_code_discrepancy_multi: ("( CODE.QUOTE ( ANAME ( 3 ( 1 ) ) 1 ( 1 ) ) CODE.QUOTE 1 CODE.DISCREPANCY )", "( 7 )", vec![]),
        test_code_do: ("( CODE.QUOTE ( FALSE 1 ) CODE.DO )", "( FALSE 1 )", vec![]),
        test_code_do_pops_last: ("( CODE.QUOTE ( CODE.QUOTE FALSE ) CODE.DO )", "( CODE.QUOTE ( CODE.QUOTE FALSE ) )", vec![]),
        test_code_do_n_count: ("( 4 CODE.QUOTE BOOL.FROMINT CODE.DONCOUNT )", "( FALSE TRUE TRUE TRUE )", vec![]),
        test_code_do_n_range_countup: ("( 0 3 CODE.QUOTE BOOL.FROMINT CODE.DONRANGE )", "( FALSE TRUE TRUE TRUE )", vec![]),
        test_code_do_n_range_countdown: ("( 3 0 CODE.QUOTE BOOL.FROMINT CODE.DONRANGE )", "( TRUE TRUE TRUE FALSE )", vec![]),
        test_code_do_n_times: ("( FALSE TRUE TRUE 2 CODE.QUOTE BOOL.ROT CODE.DONTIMES )", "( TRUE FALSE TRUE )", vec![]),
        test_code_dup: ("( CODE.QUOTE BOOL.FROMINT CODE.DUP )", "( CODE.QUOTE BOOL.FROMINT CODE.QUOTE BOOL.FROMINT )", vec![]),
        test_code_equal_true: ("( CODE.QUOTE BOOL.FROMINT CODE.QUOTE BOOL.FROMINT CODE.EQUAL )", "( TRUE )", vec![]),
        test_code_equal_false: ("( CODE.QUOTE BOOL.FROMINT CODE.QUOTE BOOL.FROMFLOAT CODE.EQUAL )", "( FALSE )", vec![]),
        test_code_extract_0: ("( CODE.QUOTE ( 1 ( 2 ) ) 0 CODE.EXTRACT )", "( CODE.QUOTE ( 1 ( 2 ) ) )", vec![]),
        test_code_extract_1: ("( CODE.QUOTE ( 1 ( 2 ) ) 1 CODE.EXTRACT )", "( CODE.QUOTE 1 )", vec![]),
        test_code_extract_2: ("( CODE.QUOTE ( 1 ( 2 ) ) 2 CODE.EXTRACT )", "( CODE.QUOTE ( 2 ) )", vec![]),
        test_code_extract_3: ("( CODE.QUOTE ( 1 ( 2 ) ) 3 CODE.EXTRACT )", "( CODE.QUOTE 2 )", vec![]),
        test_code_extract_modulo: ("( CODE.QUOTE ( 1 ( 2 ) ) 4 CODE.EXTRACT )", "( CODE.QUOTE ( 1 ( 2 ) ) )", vec![]),
        test_code_flush: ("( CODE.QUOTE ( 1 ( 2 ) ) CODE.FLUSH )", "( )", vec![]),
        test_code_from_boolean: ("( TRUE CODE.FROMBOOLEAN )", "( CODE.QUOTE TRUE )", vec![]),
        test_code_from_float: ("( 1.5 CODE.FROMFLOAT )", "( CODE.QUOTE 1.5 )", vec![]),
        test_code_from_integer: ("( 42 CODE.FROMINTEGER )", "( CODE.QUOTE 42 )", vec![]),
        test_code_from_name: ("( KmU7 CODE.FROMNAME )", "( CODE.QUOTE KmU7 )", vec![]),
        test_code_if_true: ("( TRUE CODE.QUOTE TRUENAME CODE.QUOTE FALSENAME CODE.IF )", "( TRUENAME )", vec![]),
        test_code_if_false: ("( FALSE CODE.QUOTE TRUENAME CODE.QUOTE FALSENAME CODE.IF )", "( FALSENAME )", vec![]),
        test_code_insert: ("( CODE.QUOTE C CODE.QUOTE ( A ( B ) ) 2 CODE.INSERT )", "( CODE.QUOTE ( A C ) )", vec![]),
        test_code_length: ("( CODE.QUOTE ( A B ( C 1 2 3 ) ) CODE.LENGTH )", "( 3 )", vec![]),
        test_code_list: ("( CODE.QUOTE A CODE.QUOTE ( B ) CODE.LIST )", "( CODE.QUOTE ( A ( B ) ) )", vec![]),
        test_code_member_true: ("( CODE.QUOTE A CODE.QUOTE ( A ( B ) ) CODE.MEMBER )", "( TRUE )", vec![]),
        test_code_member_false: ("( CODE.QUOTE B CODE.QUOTE ( A ( B ) ) CODE.MEMBER )", "( FALSE )", vec![]),
        test_code_nth: ("( CODE.QUOTE ( A ( B ) C ) 2 CODE.NTH )", "( CODE.QUOTE C )", vec![]),
        test_code_nth_modulo: ("( CODE.QUOTE ( A ( B ) C ) 4 CODE.NTH )", "( CODE.QUOTE ( B ) )", vec![]),
        test_code_nth_empty: ("( CODE.QUOTE ( ) 3 CODE.NTH )", "( CODE.QUOTE ( ) )", vec![]),
        test_code_nth_coerce: ("( CODE.QUOTE A 3 CODE.NTH )", "( CODE.QUOTE A )", vec![]),
        test_code_nthcdr: ("( CODE.QUOTE ( A ( B ) C ) 2 CODE.NTHCDR )", "( CODE.QUOTE ( A ( B ) ) )", vec![]),
        test_code_nthcdr_modulo: ("( CODE.QUOTE ( A ( B ) C ) 4 CODE.NTHCDR )", "( CODE.QUOTE ( A C ) )", vec![]),
        test_code_nthcdr_empty: ("( CODE.QUOTE ( ) 3 CODE.NTHCDR )", "( CODE.QUOTE ( ) )", vec![]),
        test_code_nthcdr_coerce: ("( CODE.QUOTE A 3 CODE.NTHCDR )", "( CODE.QUOTE ( ) )", vec![]),
        test_code_null_false: ("( CODE.QUOTE ( A ) CODE.NULL )", "( FALSE )", vec![]),
        test_code_null_atom: ("( CODE.QUOTE A CODE.NULL )", "( FALSE )", vec![]),
        test_code_null_true: ("( CODE.QUOTE ( ) CODE.NULL )", "( TRUE )", vec![]),
        test_code_pop: ("( CODE.QUOTE TRUE CODE.POP )", "( )", vec![]),
        test_code_position: ("( CODE.QUOTE ( B ) CODE.QUOTE ( A ( B ) ) CODE.POSITION )", "( 1 )", vec![]),
        test_code_position_not_found: ("( CODE.QUOTE B CODE.QUOTE ( A ( B ) ) CODE.POSITION )", "( -1 )", vec![]),
        test_code_position_self: ("( CODE.QUOTE B CODE.QUOTE B CODE.POSITION )", "( 0 )", vec![]),
        test_code_rot: ("( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C CODE.ROT )", "( CODE.QUOTE B CODE.QUOTE C CODE.QUOTE A )", vec![]),
        test_code_shove: ("( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C 2 CODE.SHOVE )", "( CODE.QUOTE C CODE.QUOTE A CODE.QUOTE B )", vec![]),
        test_code_shove_zero: ("( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C 0 CODE.SHOVE )", "( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C )", vec![]),
        test_code_shove_wrap: ("( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C 3 CODE.SHOVE )", "( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C )", vec![]),
        test_code_size: ("( CODE.QUOTE ( A ( B ) C ) CODE.SIZE )", "( 5 )", vec![]),
        test_code_stack_depth: ("( CODE.QUOTE A CODE.QUOTE B CODE.STACKDEPTH )", "( CODE.QUOTE A CODE.QUOTE B 2 )", vec![]),
        test_code_substitute: ("( CODE.QUOTE A CODE.QUOTE ( B ) CODE.QUOTE ( A ( B ) ( A ( B ) ) ) CODE.SUBSTITUTE )", "( CODE.QUOTE ( A A ( A A ) ) )", vec![]),
        test_code_swap: ("( CODE.QUOTE A CODE.QUOTE B CODE.SWAP )", "( CODE.QUOTE B CODE.QUOTE A )", vec![]),
        test_code_yank: ("( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C CODE.QUOTE D 2 CODE.YANK )", "( CODE.QUOTE A CODE.QUOTE C CODE.QUOTE D CODE.QUOTE B )", vec![]),
        test_code_yank_dup: ("( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C CODE.QUOTE D 2 CODE.YANKDUP )", "( CODE.QUOTE A CODE.QUOTE B CODE.QUOTE C CODE.QUOTE D CODE.QUOTE B )", vec![]),
        test_exec_define: ("( A EXEC.DEFINE TRUE A )", "( TRUE )", vec![("A", "TRUE")]),
        test_exec_do_n_count: ("( 4 EXEC.DONCOUNT BOOL.FROMINT )", "( FALSE TRUE TRUE TRUE )", vec![]),
        test_exec_do_n_range_countup: ("( 0 3 EXEC.DONRANGE BOOL.FROMINT )", "( FALSE TRUE TRUE TRUE )", vec![]),
        test_exec_do_n_range_countdown: ("( 3 0 EXEC.DONRANGE BOOL.FROMINT )", "( TRUE TRUE TRUE FALSE )", vec![]),
        test_exec_do_n_times: ("( FALSE TRUE TRUE 2 EXEC.DONTIMES BOOL.ROT )", "( TRUE FALSE TRUE )", vec![]),
        test_exec_dup: ("( EXEC.DUP 5 )", "( 5 5 )", vec![]),
        test_exec_equal: ("( EXEC.EQUAL 5 5 )", "( TRUE )", vec![]),
        test_exec_flush: ("( EXEC.FLUSH 5 5 )", "( )", vec![]),
        test_exec_if_true: ("( TRUE EXEC.IF TRUENAME FALSENAME )", "( TRUENAME )", vec![]),
        test_exec_if_false: ("( FALSE EXEC.IF TRUENAME FALSENAME )", "( FALSENAME )", vec![]),
        test_exec_k: ("( EXEC.K TRUENAME FALSENAME )", "( TRUENAME )", vec![]),
        test_exec_pop: ("( EXEC.POP 5 )", "( )", vec![]),
        test_exec_rot: ("( EXEC.ROT A B C )", "( C A B )", vec![]),
        test_exec_shove: ("( 2 EXEC.SHOVE A B C )", "( B C A )", vec![]),
        test_exec_shove_zero: ("( 0 EXEC.SHOVE A B C )", "( A B C )", vec![]),
        test_exec_shove_wrap: ("( 3 EXEC.SHOVE A B C )", "( A B C )", vec![]),
        test_exec_stack_depth: ("( EXEC.STACKDEPTH A B )", "( A B 2 )", vec![]),
        test_exec_swap: ("( EXEC.SWAP A B )", "( B A )", vec![]),
        test_exec_s: ("( EXEC.S A B C )", "( A C ( B C ) )", vec![]),
        test_exec_yank: ("( 2 EXEC.YANK A B C D )", "( C A B D )", vec![]),
        test_exec_yank_dup: ("( 2 EXEC.YANKDUP A B C D )", "( C A B C D )", vec![]),
        test_exec_y: ("( 0 EXEC.Y ( INTEGER.DUP 2 INTEGER.EQUAL EXEC.IF EXEC.POP ( INTEGER.DUP 1 INTEGER.SUM ) ) )", "( 0 1 2 )", vec![]),
        test_float_cos: ("( 1.0 FLOAT.COS )", "( 0.54030230586814 )", vec![]),
        test_float_define: ("( A 1.0 FLOAT.DEFINE A )", "( 1.0 )", vec![("A", "1.0")]),
        test_float_difference: ("( 3.0 1.0 FLOAT.DIFFERENCE )", "( 2.0 )", vec![]),
        test_float_dup: ("( 1.0 FLOAT.DUP )", "( 1.0 1.0 )", vec![]),
        test_float_equal: ("( 1.0 1.0 FLOAT.EQUAL )", "( TRUE )", vec![]),
        test_float_flush: ("( 1.0 1.0 FLOAT.FLUSH )", "( )", vec![]),
        test_float_fromboolean: ("( TRUE FLOAT.FROMBOOLEAN FALSE FLOAT.FROMBOOLEAN )", "( 1.0 0.0 )", vec![]),
        test_float_frominteger: ("( 5 FLOAT.FROMINTEGER )", "( 5.0 )", vec![]),
        test_float_greater: ("( 5.0 3.0 FLOAT.GREATER )", "( TRUE )", vec![]),
        test_float_less: ("( 5.0 3.0 FLOAT.LESS )", "( FALSE )", vec![]),
        test_float_max: ("( 5.0 3.0 FLOAT.MAX )", "( 5.0 )", vec![]),
        test_float_min: ("( -5.0 3.0 FLOAT.MIN )", "( -5.0 )", vec![]),
        test_float_modulo: ("( -5.0 3.0 FLOAT.MODULO )", "( -2.0 )", vec![]),
        test_float_modulo_zero: ("( -5.0 0.0 FLOAT.MODULO )", "( )", vec![]),
        test_float_pop: ("( 5.0 FLOAT.POP )", "( )", vec![]),
        test_float_product: ("( -5.0 3.0 FLOAT.PRODUCT )", "( -15.0 )", vec![]),
        test_float_quotient: ("( 15.0 3.0 FLOAT.QUOTIENT )", "( 5.0 )", vec![]),
        test_float_quotient_zero: ("( 15.0 0.0 FLOAT.QUOTIENT )", "( )", vec![]),
        test_float_rand: ("( FLOAT.RAND )", "( 0.426738773909753 )", vec![]),
        test_float_rot: ("( 0.0 1.0 2.0 FLOAT.ROT )", "( 1.0 2.0 0.0 )", vec![]),
        test_float_shove: ("( 1.0 2.0 3.0 2 FLOAT.SHOVE )", "( 3.0 1.0 2.0 )", vec![]),
        test_float_shove_zero: ("( 1.0 2.0 3.0 0 FLOAT.SHOVE )", "( 1.0 2.0 3.0 )", vec![]),
        test_float_shove_wrap: ("( 1.0 2.0 3.0 3 FLOAT.SHOVE )", "( 1.0 2.0 3.0 )", vec![]),
        test_float_sin: ("( 1.0 FLOAT.SIN )", "( 0.841470984807897 )", vec![]),
        test_float_stack_depth: ("( 1.0 2.0 FLOAT.STACKDEPTH )", "( 1.0 2.0 2 )", vec![]),
        test_float_sum: ("( 1.5 2.5 FLOAT.SUM )", "( 4.0 )", vec![]),
        test_float_swap: ("( 1.0 2.0 3.0 FLOAT.SWAP )", "( 1.0 3.0 2.0 )", vec![]),
        test_float_tan: ("( 1.0 FLOAT.TAN )", "( 1.557407724654902 )", vec![]),
        test_float_yank: ("( 1.0 2.0 3.0 4.0 2 FLOAT.YANK )", "( 1.0 3.0 4.0 2.0 )", vec![]),
        test_float_yank_dup: ("( 1.0 2.0 3.0 4.0 2 FLOAT.YANKDUP )", "( 1.0 2.0 3.0 4.0 2.0 )", vec![]),
        // test_integer_define: ("( A 1 INTEGER.DEFINE A )", "( 1 )", vec![]),
        // test_integer_difference: ("( 3 1 INTEGER.DIFFERENCE )", "( 2 )", vec![]),
        test_integer_dup: ("( 42 INTEGER.DUP )", "( 42 42 )", vec![]),
        test_integer_equal: ("( 42 0 INTEGER.EQUAL )", "( FALSE )", vec![]),
        // test_integer_flush: ("( 1 1 INTEGER.FLUSH )", "( )", vec![]),
        // test_integer_fromboolean: ("( TRUE INTEGER.FROMBOOLEAN FALSE INTEGER.FROMBOOLEAN )", "( 1 0 )", vec![]),
        // test_integer_fromfloat: ("( 5.0 INTEGER.FROMFLOAT )", "( 5 )", vec![]),
        // test_integer_greater: ("( 5 3 INTEGER.GREATER )", "( TRUE )", vec![]),
        // test_integer_less: ("( 5 3 INTEGER.LESS )", "( FALSE )", vec![]),
        // test_integer_max: ("( 5 3 INTEGER.MAX )", "( 5 )", vec![]),
        // test_integer_min: ("( -5 3 INTEGER.MIN )", "( -5 )", vec![]),
        // test_integer_modulo: ("( -5 3 INTEGER.MODULO )", "( -2 )", vec![]),
        // test_integer_modulo_zero: ("( -5 0 INTEGER.MODULO )", "( )", vec![]),
        test_integer_pop: ("( 42 INTEGER.POP )", "( )", vec![]),
        // test_integer_product: ("( -5 3 INTEGER.PRODUCT )", "( -15 )", vec![]),
        // test_integer_quotient: ("( 15 3 INTEGER.QUOTIENT )", "( 5 )", vec![]),
        // test_integer_quotient_zero: ("( 15 0 INTEGER.QUOTIENT )", "( )", vec![]),
        // test_integer_rand: ("( INTEGER.RAND )", "( 1 )", vec![]),
        // test_integer_rot: ("( 0 1 2 INTEGER.ROT )", "( 1 2 0 )", vec![]),
        // test_integer_shove: ("( 1 2 3 2 INTEGER.SHOVE )", "( 3 1 2 )", vec![]),
        // test_integer_shove_zero: ("( 1 2 3 0 INTEGER.SHOVE )", "( 1 2 3 )", vec![]),
        // test_integer_shove_wrap: ("( 1 2 3 3 INTEGER.SHOVE )", "( 1 2 3 )", vec![]),
        // test_integer_stack_depth: ("( 1 2 INTEGER.STACKDEPTH )", "( 1 2 2 )", vec![]),
        test_integer_sum: ("( 42 7 INTEGER.SUM )", "( 49 )", vec![]),
        // test_integer_swap: ("( 1 2 3 INTEGER.SWAP )", "( 1 3 2 )", vec![]),
        // test_integer_yank: ("( 1 2 3 4 2 INTEGER.YANK )", "( 1 3 4 2 )", vec![]),
        // test_integer_yank_dup: ("( 1 2 3 4 2 INTEGER.YANKDUP )", "( 1 2 3 4 2 )", vec![]),
        // test_name_dup: ("( A NAME.DUP )", "( A A )", vec![]),
        // test_name_equal: ("( A B NAME.EQUAL )", "( FALSE )", vec![]),
        // test_name_flush: ("( A B NAME.FLUSH )", "( )", vec![]),
        // test_name_pop: ("( A NAME.POP )", "( )", vec![]),
        // test_name_quote: ("( A 1.0 FLOAT.DEFINE NAME.QUOTE A )", "( A )", vec![]),
        // test_name_rand: ("( NAME.RAND )", "( ABCDEFG )", vec![]),
        // test_name_rand_bound: ("( A 1.0 FLOAT.DEFINE NAME.RANDBOUNDNAME A )", "( A )", vec![]),
        // test_name_rot: ("( A B C NAME.ROT )", "( B C A )", vec![]),
        // test_name_shove: ("( A B C 2 NAME.SHOVE )", "( C A B )", vec![]),
        // test_name_shove_zero: ("( A B C 0 NAME.SHOVE )", "( A B C )", vec![]),
        // test_name_shove_wrap: ("( A B C 3 NAME.SHOVE )", "( A B C )", vec![]),
        // test_name_stack_depth: ("( A B NAME.STACKDEPTH )", "( A B 2 )", vec![]),
        // test_name_swap: ("( A B C NAME.SWAP )", "( A C B )", vec![]),
        // test_name_yank: ("( A B C D 2 NAME.YANK )", "( A C D B )", vec![]),
        // test_name_yank_dup: ("( A B C D 2 NAME.YANKDUP )", "( A B C D B )", vec![]),
    }

    #[test]
    fn code_quote() {
        let to_run = load_and_run("( CODE.QUOTE TRUE )");
        assert_eq!(0, to_run.exec().len());
        assert_eq!(0, to_run.bool().len());
        assert_eq!(Some(Code::Literal(BaseLiteral::Bool(true))), to_run.code().pop());
    }

    #[test]
    fn code_instructions() {
        use crate::StackTrait;

        let to_run = load_and_run("( CODE.INSTRUCTIONS )");
        assert!(to_run.code().len() > 100);

        let mut all_entries = vec![];
        while let Some(c) = to_run.code().pop() {
            all_entries.push(c);
        }
        assert!(all_entries.contains(&Code::Instruction("BOOL.AND".to_owned())));
        assert!(all_entries.contains(&Code::Instruction("CODE.APPEND".to_owned())));
    }
}
