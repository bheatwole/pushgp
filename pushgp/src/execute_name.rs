use base64::*;
use byte_slice_cast::*;
use crate::*;
use fnv::FnvHashMap;
use std::cell::RefCell;

pub type Name = String;

impl Literal<Name> for Name {
    fn parse(input: &str) -> nom::IResult<&str, Name> {
        crate::parse::parse_code_name(input)
    }

    fn random_value<R: rand::Rng>(rng: &mut R) -> Name {
        let random_value = rng.gen_range(0..=u64::MAX);

        let slice: [u64; 1] = [random_value];
        let b64 = encode(slice.as_byte_slice());
        let name = "RND.".to_owned() + &b64;

        name
    }
}

pub trait ContextHasNameStack<L: LiteralEnum<L>> {
    fn name(&self) -> &NameStack<L>;
    fn make_literal_name(value: Name) -> Code<L>;
}

#[derive(Debug, PartialEq)]
pub struct NameStack<L: LiteralEnum<L>> {
    stack: Stack<Name>,
    quote_next_name: RefCell<bool>,
    defined_names: RefCell<FnvHashMap<String, Code<L>>>
}

impl <L: LiteralEnum<L>> NameStack<L> {
    pub fn should_quote_next_name(&self) -> bool {
        *self.quote_next_name.borrow()
    }

    pub fn set_should_quote_next_name(&self, quote_next_name: bool) {
        *self.quote_next_name.borrow_mut() = quote_next_name
    }

    pub fn definition_for_name(&self, name: &String) -> Option<Code<L>> {
        self.defined_names.borrow().get(name).map(|c| c.clone())
    }

    pub fn define_name(&self, name: String, code: Code<L>) {
        self.defined_names.borrow_mut().insert(name, code);
    }
}

impl<L: LiteralEnum<L>> StackTrait<Name> for NameStack<L> {
    fn new() -> NameStack<L> {
        NameStack {
            stack: Stack::new(),
            quote_next_name: RefCell::new(false),
            defined_names: RefCell::new(FnvHashMap::default()),
        }
    }
    fn pop(&self) -> Option<Name> { self.stack.pop() }
    fn push(&self, item: Name) { self.stack.push(item) }
    fn len(&self) -> usize { self.stack.len() }
    fn duplicate_top_item(&self) { self.stack.duplicate_top_item() }
    fn clear(&self) { self.stack.clear() }
    fn rotate(&self) { self.stack.rotate() }
    fn shove(&self, position: i64) -> bool { self.stack.shove(position) }
    fn swap(&self) { self.stack.swap() }
    fn yank(&self, position: i64) -> bool { self.stack.yank(position) }
    fn yank_duplicate(&self, position: i64) -> bool { self.stack.yank_duplicate(position) }
}

// pub fn execute_namedup<C: Context + ContextStack<Name>>(context: &mut C) {
//     context.get_stack().duplicate_top_item()
// }

// pub fn execute_nameequal<C: Context + ContextStack<Name> + ContextStack<Bool>>(context: &mut C) {
//     if <C as ContextStack<Name>>::len(context) >= 2 {
//         let a: Name = context.pop().unwrap();
//         let b: Name = context.pop().unwrap();
//         context.push(a == b);
//     }
// }

// pub fn execute_nameflush(context: &mut Context) {
//     context.name_stack.clear();
// }

// pub fn execute_namepop(context: &mut Context) {
//     if context.name_stack.len() >= 1 {
//         context.name_stack.pop();
//     }
// }

// pub fn execute_namequote(context: &mut Context) {
//     context.quote_next_name = true;
// }

// pub fn execute_namerandboundname(context: &mut Context) {
//     let len = context.defined_names.len() as i64;
//     if len > 0 {
//         let index = context.config.random_int_in_range(0..len);
//         if let Some(name) = context.defined_names.keys().skip(index as usize).next() {
//             context.name_stack.push(*name);
//         }
//     }
// }

// pub fn execute_namerand(context: &mut Context) {
//     context.name_stack.push(context.config.random_name())
// }

// pub fn execute_namerot(context: &mut Context) {
//     let a = context.name_stack.pop().unwrap();
//     let b = context.name_stack.pop().unwrap();
//     let c = context.name_stack.pop().unwrap();
//     context.name_stack.push(b);
//     context.name_stack.push(a);
//     context.name_stack.push(c);
// }

// pub fn execute_nameshove(context: &mut Context) {
//     if context.name_stack.len() >= 1 && context.name_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.name_stack.len());
//         let b = context.name_stack.pop().unwrap();
//         context.name_stack.insert(vec_index, b);
//     }
// }

// pub fn execute_namestackdepth(context: &mut Context) {
//     context.int_stack.push(context.name_stack.len() as i64);
// }

// pub fn execute_nameswap(context: &mut Context) {
//     let a = context.name_stack.pop().unwrap();
//     let b = context.name_stack.pop().unwrap();
//     context.name_stack.push(a);
//     context.name_stack.push(b);
// }

// pub fn execute_nameyankdup(context: &mut Context) {
//     if context.name_stack.len() >= 1 && context.name_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.name_stack.len());
//         let &b = context.name_stack.get(vec_index).unwrap();
//         context.name_stack.push(b);
//     }
// }

// pub fn execute_nameyank(context: &mut Context) {
//     if context.name_stack.len() >= 1 && context.name_stack.len() >= 1 {
//         let stack_index = context.int_stack.pop().unwrap();
//         let vec_index = crate::util::stack_to_vec(stack_index, context.name_stack.len());
//         let b = context.name_stack.remove(vec_index);
//         context.name_stack.push(b);
//     }
// }