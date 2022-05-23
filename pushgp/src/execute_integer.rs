use crate::*;
use pushgp_macros::*;

pub type Integer = i64;

impl Literal<Integer> for Integer {
    fn parse(input: &str) -> nom::IResult<&str, Integer> {
        crate::parse::parse_code_integer(input)
    }

    fn random_value<R: rand::Rng>(rng: &mut R) -> Integer {
        rng.gen_range(i64::MIN..=i64::MAX)
    }
}

pub trait ContextHasIntegerStack<L: LiteralEnum<L>> {
    fn integer(&self) -> &Stack<Integer>;
    fn make_literal_integer(value: Integer) -> Code<L>;
}

instruction! {
    /// Duplicates the top item on the INTEGER stack. Does not pop its argument (which, if it did, would negate the
    /// effect of the duplication!).
    #[stack(Integer)]
    fn dup(context: &mut Context) {
        context.integer().duplicate_top_item();
    }
}

instruction! {
    /// Pushes TRUE if the top two items on the INTEGER stack are equal, or FALSE otherwise.
    #[stack(Integer)]
    fn equal(context: &mut Context, a: Integer, b: Integer) {
        context.bool().push(a == b);
    }
}

instruction! {
    /// Pops the INTEGER stack.
    #[stack(Integer)]
    fn pop(context: &mut Context, _popped: Integer) {
    }
}

instruction! {
    /// Pushes the sum of the top two items.
    #[stack(Integer)]
    fn sum(context: &mut Context, a: Integer, b: Integer) {
        context.integer().push(a + b);
    }
}

// pub fn execute_integerdefine(context: &mut Context) {
//     if context.name_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let name = context.name_stack.pop().unwrap();
//         let value = context.int_stack.pop().unwrap();
//         context.defined_names.insert(name, Code::LiteralInteger(value));
//     }
// }

// pub fn execute_integerdifference(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let right = context.int_stack.pop().unwrap();
//         let left = context.int_stack.pop().unwrap();
//         context.int_stack.push(left - right);
//     }
// }

// pub fn execute_integerdup(context: &mut Context) {
//     if context.int_stack.len() >= 1 {
//         let value = context.int_stack.last().unwrap().clone();
//         context.int_stack.push(value);
//     }
// }

// pub fn execute_integerequal(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let a = context.int_stack.pop().unwrap();
//         let b = context.int_stack.pop().unwrap();
//         context.bool_stack.push(a == b);
//     }
// }

// pub fn execute_integerflush(context: &mut Context) {
//     context.int_stack.clear();
// }

// pub fn execute_integerfromboolean(context: &mut Context) {
//     if context.bool_stack.len() >= 1 {
//         context.int_stack.push(if context.bool_stack.pop().unwrap() { 1 } else { 0 });
//     }
// }

// pub fn execute_integerfromfloat(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         context.int_stack.push(context.float_stack.pop().unwrap().to_i64().unwrap());
//     }
// }

// pub fn execute_integergreater(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let right = context.int_stack.pop().unwrap();
//         let left = context.int_stack.pop().unwrap();
//         context.bool_stack.push(left > right);
//     }
// }

// pub fn execute_integerless(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let right = context.int_stack.pop().unwrap();
//         let left = context.int_stack.pop().unwrap();
//         context.bool_stack.push(left < right);
//     }
// }

// pub fn execute_integermax(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let a = context.int_stack.pop().unwrap();
//         let b = context.int_stack.pop().unwrap();
//         context.int_stack.push(if a < b { b } else { a });
//     }
// }

// pub fn execute_integermin(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let a = context.int_stack.pop().unwrap();
//         let b = context.int_stack.pop().unwrap();
//         context.int_stack.push(if a < b { a } else { b });
//     }
// }

// pub fn execute_integermodulo(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let bottom = context.int_stack.pop().unwrap();
//         let top = context.int_stack.pop().unwrap();
//         if bottom != 0 {
//             context.int_stack.push(top % bottom);
//         }
//     }
// }

// pub fn execute_integerpop(context: &mut Context) {
//     if context.int_stack.len() >= 1 {
//         context.int_stack.pop();
//     }
// }

// pub fn execute_integerproduct(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let right = context.int_stack.pop().unwrap();
//         let left = context.int_stack.pop().unwrap();
//         context.int_stack.push(left * right);
//     }
// }

// pub fn execute_integerquotient(context: &mut Context) {
//     let bottom = context.int_stack.pop().unwrap();
//     let top = context.int_stack.pop().unwrap();
//     if bottom != 0 {
//         context.int_stack.push(top / bottom);
//     }
// }

// pub fn execute_integerrand(context: &mut Context) {
//     context.int_stack.push(context.config.random_int())
// }

// pub fn execute_integerrot(context: &mut Context) {
//     let a = context.int_stack.pop().unwrap();
//     let b = context.int_stack.pop().unwrap();
//     let c = context.int_stack.pop().unwrap();
//     context.int_stack.push(b);
//     context.int_stack.push(a);
//     context.int_stack.push(c);
// }

// pub fn execute_integershove(context: &mut Context) {
//     if context.int_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.int_stack.len());
//         let b = context.int_stack.pop().unwrap();
//         context.int_stack.insert(vec_index, b);
//     }
// }

// pub fn execute_integerstackdepth(context: &mut Context) {
//     context.int_stack.push(context.int_stack.len() as i64);
// }

// pub fn execute_integersum(context: &mut Context) {
//     if context.int_stack.len() >= 2 {
//         let a = context.int_stack.pop().unwrap();
//         let b = context.int_stack.pop().unwrap();
//         context.int_stack.push(a + b);
//     }
// }

// pub fn execute_integerswap(context: &mut Context) {
//     let a = context.int_stack.pop().unwrap();
//     let b = context.int_stack.pop().unwrap();
//     context.int_stack.push(a);
//     context.int_stack.push(b);
// }

// pub fn execute_integeryankdup(context: &mut Context) {
//     if context.int_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.int_stack.len());
//         let &b = context.int_stack.get(vec_index).unwrap();
//         context.int_stack.push(b);
//     }
// }

// pub fn execute_integeryank(context: &mut Context) {
//     if context.int_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.int_stack.len());
//         let b = context.int_stack.remove(vec_index);
//         context.int_stack.push(b);
//     }
// }
