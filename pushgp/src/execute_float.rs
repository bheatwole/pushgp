use crate::*;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use rand::Rng;

pub type Float = Decimal;

impl Literal<Float> for Float {
    fn parse(input: &str) -> nom::IResult<&str, Float> {
        crate::parse::parse_code_float(input)
    }

    fn random_value(rng: &mut rand::rngs::SmallRng) -> Float {
        let float: f64 = rng.gen_range(-1f64..1f64);
        Decimal::from_f64(float).unwrap()
    }
}

pub trait ContextHasFloatStack<L: LiteralEnum<L>> {
    fn float(&self) -> &Stack<Float>;
    fn make_literal_float(value: Float) -> Code<L>;
}

// pub fn execute_floatcos(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         let value = context.float_stack.pop().unwrap();
//         context.float_stack.push(Decimal::from_f64(value.to_f64().unwrap().cos()).unwrap());
//     }
// }

// pub fn execute_floatdefine(context: &mut Context) {
//     if context.name_stack.len() >= 1 && context.float_stack.len() >= 1 {
//         let name = context.name_stack.pop().unwrap();
//         let value = context.float_stack.pop().unwrap();
//         context.defined_names.insert(name, Code::LiteralFloat(value));
//     }
// }

// pub fn execute_floatdifference(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let right = context.float_stack.pop().unwrap();
//         let left = context.float_stack.pop().unwrap();
//         context.float_stack.push(left - right);
//     }
// }

// pub fn execute_floatdup(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         let value = context.float_stack.pop().unwrap();
//         context.float_stack.push(value);
//         context.float_stack.push(value);
//     }
// }

// pub fn execute_floatequal(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let a = context.float_stack.pop().unwrap();
//         let b = context.float_stack.pop().unwrap();
//         context.bool_stack.push(a == b);
//     }
// }

// pub fn execute_floatflush(context: &mut Context) {
//     context.float_stack.clear();
// }

// pub fn execute_floatfromboolean(context: &mut Context) {
//     if context.bool_stack.len() >= 1 {
//         context.float_stack.push(if context.bool_stack.pop().unwrap() {
//             Decimal::new(1, 0)
//         } else {
//             Decimal::new(0, 0)
//         });
//     }
// }

// pub fn execute_floatfrominteger(context: &mut Context) {
//     if context.int_stack.len() >= 1 {
//         context.float_stack.push(Decimal::new(context.int_stack.pop().unwrap(), 0));
//     }
// }

// pub fn execute_floatgreater(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let right = context.float_stack.pop().unwrap();
//         let left = context.float_stack.pop().unwrap();
//         context.bool_stack.push(left > right);
//     }
// }

// pub fn execute_floatless(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let right = context.float_stack.pop().unwrap();
//         let left = context.float_stack.pop().unwrap();
//         context.bool_stack.push(left < right);
//     }
// }

// pub fn execute_floatmax(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let a = context.float_stack.pop().unwrap();
//         let b = context.float_stack.pop().unwrap();
//         context.float_stack.push(if a < b { b } else { a });
//     }
// }

// pub fn execute_floatmin(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let a = context.float_stack.pop().unwrap();
//         let b = context.float_stack.pop().unwrap();
//         context.float_stack.push(if a < b { a } else { b });
//     }
// }

// pub fn execute_floatmodulo(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let bottom = context.float_stack.pop().unwrap();
//         let top = context.float_stack.pop().unwrap();
//         if bottom != Decimal::ZERO {
//             context.float_stack.push(top % bottom);
//         }
//     }
// }

// pub fn execute_floatpop(context: &mut Context) {
//     context.float_stack.pop();
// }

// pub fn execute_floatproduct(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let right = context.float_stack.pop().unwrap();
//         let left = context.float_stack.pop().unwrap();
//         context.float_stack.push(left * right);
//     }
// }

// pub fn execute_floatquotient(context: &mut Context) {
//     let bottom = context.float_stack.pop().unwrap();
//     let top = context.float_stack.pop().unwrap();
//     if bottom != Decimal::ZERO {
//         context.float_stack.push(top / bottom);
//     }
// }

// pub fn execute_floatrand(context: &mut Context) {
//     context.float_stack.push(context.config.random_float())
// }

// pub fn execute_floatrot(context: &mut Context) {
//     let a = context.float_stack.pop().unwrap();
//     let b = context.float_stack.pop().unwrap();
//     let c = context.float_stack.pop().unwrap();
//     context.float_stack.push(b);
//     context.float_stack.push(a);
//     context.float_stack.push(c);
// }

// pub fn execute_floatshove(context: &mut Context) {
//     if context.float_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.float_stack.len());
//         let b = context.float_stack.pop().unwrap();
//         context.float_stack.insert(vec_index, b);
//     }
// }

// pub fn execute_floatsin(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         let value = context.float_stack.pop().unwrap();
//         context.float_stack.push(Decimal::from_f64(value.to_f64().unwrap().sin()).unwrap());
//     }
// }

// pub fn execute_floatstackdepth(context: &mut Context) {
//     context.int_stack.push(context.float_stack.len() as i64);
// }

// pub fn execute_floatsum(context: &mut Context) {
//     if context.float_stack.len() >= 2 {
//         let right = context.float_stack.pop().unwrap();
//         let left = context.float_stack.pop().unwrap();
//         context.float_stack.push(left + right);
//     }
// }

// pub fn execute_floatswap(context: &mut Context) {
//     let a = context.float_stack.pop().unwrap();
//     let b = context.float_stack.pop().unwrap();
//     context.float_stack.push(a);
//     context.float_stack.push(b);
// }

// pub fn execute_floattan(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         let value = context.float_stack.pop().unwrap();
//         context.float_stack.push(Decimal::from_f64(value.to_f64().unwrap().tan()).unwrap());
//     }
// }

// pub fn execute_floatyankdup(context: &mut Context) {
//     if context.float_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.float_stack.len());
//         let &b = context.float_stack.get(vec_index).unwrap();
//         context.float_stack.push(b);
//     }
// }

// pub fn execute_floatyank(context: &mut Context) {
//     if context.float_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.float_stack.len());
//         let b = context.float_stack.remove(vec_index);
//         context.float_stack.push(b);
//     }
// }
