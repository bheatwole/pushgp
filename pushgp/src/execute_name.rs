use crate::Context;

pub fn execute_namedup(context: &mut Context) {
    if context.name_stack.len() >= 1 {
        let value = context.name_stack.last().unwrap().clone();
        context.name_stack.push(value);
    }
}

pub fn execute_nameequal(context: &mut Context) {
    if context.name_stack.len() >= 2 {
        let a = context.name_stack.pop().unwrap();
        let b = context.name_stack.pop().unwrap();
        context.bool_stack.push(a == b);
    }
}

pub fn execute_nameflush(context: &mut Context) {
    context.name_stack.clear();
}

pub fn execute_namepop(context: &mut Context) {
    if context.name_stack.len() >= 1 {
        context.name_stack.pop();
    }
}

pub fn execute_namequote(context: &mut Context) {
    context.quote_next_name = true;
}

pub fn execute_namerandboundname(context: &mut Context) {
    let len = context.defined_names.len() as i64;
    if len > 0 {
        let index = context.config.random_int_in_range(0..len);
        if let Some(name) = context.defined_names.keys().skip(index as usize).next() {
            context.name_stack.push(*name);
        }
    }
}

pub fn execute_namerand(context: &mut Context) {
    context.name_stack.push(context.config.random_name())
}

pub fn execute_namerot(context: &mut Context) {
    let a = context.name_stack.pop().unwrap();
    let b = context.name_stack.pop().unwrap();
    let c = context.name_stack.pop().unwrap();
    context.name_stack.push(b);
    context.name_stack.push(a);
    context.name_stack.push(c);
}

pub fn execute_nameshove(context: &mut Context) {
    if context.name_stack.len() >= 1 && context.name_stack.len() >= 1 {
        let stack_index = context.int_stack.pop().unwrap();
        let vec_index = crate::util::stack_to_vec(stack_index, context.name_stack.len());
        let b = context.name_stack.pop().unwrap();
        context.name_stack.insert(vec_index, b);
    }
}

pub fn execute_namestackdepth(context: &mut Context) {
    context.int_stack.push(context.name_stack.len() as i64);
}

pub fn execute_nameswap(context: &mut Context) {
    let a = context.name_stack.pop().unwrap();
    let b = context.name_stack.pop().unwrap();
    context.name_stack.push(a);
    context.name_stack.push(b);
}

pub fn execute_nameyankdup(context: &mut Context) {
    if context.name_stack.len() >= 1 && context.name_stack.len() >= 1 {
        let stack_index = context.int_stack.pop().unwrap();
        let vec_index = crate::util::stack_to_vec(stack_index, context.name_stack.len());
        let &b = context.name_stack.get(vec_index).unwrap();
        context.name_stack.push(b);
    }
}

pub fn execute_nameyank(context: &mut Context) {
    if context.name_stack.len() >= 1 && context.name_stack.len() >= 1 {
        let stack_index = context.int_stack.pop().unwrap();
        let vec_index = crate::util::stack_to_vec(stack_index, context.name_stack.len());
        let b = context.name_stack.remove(vec_index);
        context.name_stack.push(b);
    }
}