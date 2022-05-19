use crate::*;
use std::marker::PhantomData;

pub type Bool = bool;

// Our version of Bool needs to display with uppercase TRUE and FALSE instead of the default
impl Literal<Bool> for Bool {
    fn parse(input: &str) -> nom::IResult<&str, Bool> {
        crate::parse::parse_code_bool(input)
    }

    fn nom_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self { "TRUE" } else { "FALSE" })
    }

    fn random_value<R: rand::Rng>(rng: &mut R) -> Bool {
        if 0 == rng.gen_range(0..=1) {
            false
        } else {
            true
        }
    }
}

pub trait ContextHasBoolStack<L: LiteralEnum<L>> {
    fn bool(&self) -> &Stack<Bool>;
    fn make_literal_bool(value: Bool) -> Code<L>;
}

/// Pushes the logical AND of the top two BOOLEANs onto the EXEC stack
pub struct BoolAnd<C, L> {
    c: PhantomData<C>,
    l: PhantomData<L>,
}

impl<C, L> InstructionTrait<C> for BoolAnd<C, L>
where
    C: Context + ContextHasBoolStack<L>,
    L: LiteralEnum<L>,
{
    fn name() -> &'static str {
        "BOOL.AND"
    }

    fn execute(context: &mut C) {
        if context.bool().len() >= 2 {
            let a = context.bool().pop().unwrap();
            let b = context.bool().pop().unwrap();
            context.bool().push(a && b);
        }
    }
}

/// Defines the name on top of the NAME stack as an instruction that will push the top item of the BOOLEAN stack
pub struct BoolDefine<C, L> {
    c: PhantomData<C>,
    l: PhantomData<L>,
}

impl<C, L> InstructionTrait<C> for BoolDefine<C, L>
where
    C: Context + ContextHasBoolStack<L> + ContextHasNameStack<L>,
    L: LiteralEnum<L>,
{
    fn name() -> &'static str {
        "BOOL.DEFINE"
    }

    fn execute(context: &mut C) {
        if context.bool().len() >= 1 && context.name().len() >= 1 {
            let value = context.bool().pop().unwrap();
            let name = context.name().pop().unwrap();
            context.name().define_name(name, C::make_literal_bool(value));
        }
    }
}

/// Duplicates the top item on the BOOLEAN stack. Does not pop its argument (which, if it did, would negate the
/// effect of the duplication!)
pub struct BoolDup<C, L> {
    c: PhantomData<C>,
    l: PhantomData<L>,
}

impl<C, L> InstructionTrait<C> for BoolDup<C, L>
where
    C: Context + ContextHasBoolStack<L>,
    L: LiteralEnum<L>,
{
    fn name() -> &'static str {
        "BOOL.DUP"
    }

    fn execute(context: &mut C) {
        context.bool().duplicate_top_item();
    }
}

// pub fn execute_booldup(context: &mut Context) {
//     if context.bool_stack.len() >= 1 {
//         let &b = context.bool_stack.last().unwrap();
//         context.bool_stack.push(b);
//     }
// }

// pub fn execute_boolequal(context: &mut Context) {
//     if context.bool_stack.len() >= 2 {
//         let a = context.bool_stack.pop().unwrap();
//         let b = context.bool_stack.pop().unwrap();
//         context.bool_stack.push(a == b);
//     }
// }

// pub fn execute_boolflush(context: &mut Context) {
//     context.bool_stack.clear();
// }

// pub fn execute_boolfromfloat(context: &mut Context) {
//     if context.float_stack.len() >= 1 {
//         let f = context.float_stack.pop().unwrap();
//         context.bool_stack.push(!f.is_zero());
//     }
// }

// pub fn execute_boolfromint(context: &mut Context) {
//     if context.int_stack.len() >= 1 {
//         let i = context.int_stack.pop().unwrap();
//         context.bool_stack.push(i != 0);
//     }
// }

// pub fn execute_boolnot(context: &mut Context) {
//     if context.bool_stack.len() >= 1 {
//         let b = context.bool_stack.pop().unwrap();
//         context.bool_stack.push(!b);
//     }
// }

// pub fn execute_boolor(context: &mut Context) {
//     if context.bool_stack.len() >= 2 {
//         let a = context.bool_stack.pop().unwrap();
//         let b = context.bool_stack.pop().unwrap();
//         context.bool_stack.push(a || b);
//     }
// }

// pub fn execute_boolpop(context: &mut Context) {
//     context.bool_stack.pop();
// }

// pub fn execute_boolrand(context: &mut Context) {
//     context.bool_stack.push(context.config.random_bool())
// }

// pub fn execute_boolrot(context: &mut Context) {
//     let a = context.bool_stack.pop().unwrap();
//     let b = context.bool_stack.pop().unwrap();
//     let c = context.bool_stack.pop().unwrap();
//     context.bool_stack.push(b);
//     context.bool_stack.push(a);
//     context.bool_stack.push(c);
// }

// pub fn execute_boolshove(context: &mut Context) {
//     if context.bool_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.bool_stack.len());
//         let b = context.bool_stack.pop().unwrap();
//         context.bool_stack.insert(vec_index, b);
//     }
// }

// pub fn execute_boolstackdepth(context: &mut Context) {
//     context.int_stack.push(context.bool_stack.len() as i64);
// }

// pub fn execute_boolswap(context: &mut Context) {
//     if context.bool_stack.len() >= 2 {
//         let a = context.bool_stack.pop().unwrap();
//         let b = context.bool_stack.pop().unwrap();
//         context.bool_stack.push(a);
//         context.bool_stack.push(b);
//     }
// }

// pub fn execute_boolyank(context: &mut Context) {
//     if context.bool_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.bool_stack.len());
//         let b = context.bool_stack.remove(vec_index);
//         context.bool_stack.push(b);
//     }
// }

// pub fn execute_boolyankdup(context: &mut Context) {
//     if context.bool_stack.len() >= 1 && context.int_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.bool_stack.len());
//         let &b = context.bool_stack.get(vec_index).unwrap();
//         context.bool_stack.push(b);
//     }
// }
