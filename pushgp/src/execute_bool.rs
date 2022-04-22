use crate::{Code, Context};

pub fn execute_booland(context: &mut Context) {
    if context.bool_stack.len() >= 2 {
        let a = context.bool_stack.pop().unwrap();
        let b = context.bool_stack.pop().unwrap();
        context.bool_stack.push(a && b);
    }
}

pub fn execute_booldefine(context: &mut Context) {
    if context.bool_stack.len() >= 1 && context.name_stack.len() >= 1 {
        let b = context.bool_stack.pop().unwrap();
        let n = context.name_stack.pop().unwrap();
        context.defined_names.insert(n, Code::LiteralBool(b));
    }
}

pub fn execute_booldup(context: &mut Context) {
    if context.bool_stack.len() >= 1 {
        let &b = context.bool_stack.last().unwrap();
        context.bool_stack.push(b);
    }
}

pub fn execute_boolequal(context: &mut Context) {
    if context.bool_stack.len() >= 2 {
        let a = context.bool_stack.pop().unwrap();
        let b = context.bool_stack.pop().unwrap();
        context.bool_stack.push(a == b);
    }
}

pub fn execute_boolflush(context: &mut Context) {
    context.bool_stack.clear();
}

pub fn execute_boolfromfloat(context: &mut Context) {
    if context.float_stack.len() >= 1 {
        let f = context.float_stack.pop().unwrap();
        context.bool_stack.push(!f.is_zero());
    }
}

pub fn execute_boolfromint(context: &mut Context) {
    if context.int_stack.len() >= 1 {
        let i = context.int_stack.pop().unwrap();
        context.bool_stack.push(i != 0);
    }
}

pub fn execute_boolnot(context: &mut Context) {
    if context.bool_stack.len() >= 1 {
        let b = context.bool_stack.pop().unwrap();
        context.bool_stack.push(!b);
    }
}

pub fn execute_boolor(context: &mut Context) {
    if context.bool_stack.len() >= 2 {
        let a = context.bool_stack.pop().unwrap();
        let b = context.bool_stack.pop().unwrap();
        context.bool_stack.push(a || b);
    }
}

pub fn execute_boolpop(context: &mut Context) {
    context.bool_stack.pop();
}

pub fn execute_boolrand(context: &mut Context) {
    context.bool_stack.push(context.config.random_bool())
}

pub fn execute_boolrot(context: &mut Context) {
    let a = context.bool_stack.pop().unwrap();
    let b = context.bool_stack.pop().unwrap();
    let c = context.bool_stack.pop().unwrap();
    context.bool_stack.push(b);
    context.bool_stack.push(a);
    context.bool_stack.push(c);
}

pub fn execute_boolshove(context: &mut Context) {
    if context.bool_stack.len() >= 1 && context.int_stack.len() >= 1 {
        let stack_index = context.int_stack.pop().unwrap();
        let vec_index = crate::util::stack_to_vec(stack_index, context.bool_stack.len());
        let b = context.bool_stack.pop().unwrap();
        context.bool_stack.insert(vec_index, b);
    }
}

pub fn execute_boolstackdepth(context: &mut Context) {
    context.int_stack.push(context.bool_stack.len() as i64);
}

pub fn execute_boolswap(context: &mut Context) {
    if context.bool_stack.len() >= 2 {
        let a = context.bool_stack.pop().unwrap();
        let b = context.bool_stack.pop().unwrap();
        context.bool_stack.push(a);
        context.bool_stack.push(b);
    }
}

pub fn execute_boolyank(context: &mut Context) {
    if context.bool_stack.len() >= 1 && context.int_stack.len() >= 1 {
        let stack_index = context.int_stack.pop().unwrap();
        let vec_index = crate::util::stack_to_vec(stack_index, context.bool_stack.len());
        let b = context.bool_stack.remove(vec_index);
        context.bool_stack.push(b);
    }
}

pub fn execute_boolyankdup(context: &mut Context) {
    if context.bool_stack.len() >= 1 && context.int_stack.len() >= 1 {
        let stack_index = context.int_stack.pop().unwrap();
        let vec_index = crate::util::stack_to_vec(stack_index, context.bool_stack.len());
        let &b = context.bool_stack.get(vec_index).unwrap();
        context.bool_stack.push(b);
    }
}
