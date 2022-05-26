use crate::*;
use base64::*;
use byte_slice_cast::*;
use fnv::FnvHashMap;
use pushgp_macros::*;
use rand::Rng;
use std::cell::RefCell;

pub type Name = String;

impl Literal<Name> for Name {
    fn parse(input: &str) -> nom::IResult<&str, Name> {
        crate::parse::parse_code_name(input)
    }

    fn random_value(rng: &mut rand::rngs::SmallRng) -> Name {
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
    defined_names: RefCell<FnvHashMap<String, Code<L>>>,
}

impl<L: LiteralEnum<L>> NameStack<L> {
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

    pub fn all_names(&self) -> Vec<String> {
        self.defined_names.borrow().keys().map(|k| k.clone()).collect()
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
    fn pop(&self) -> Option<Name> {
        self.stack.pop()
    }
    fn push(&self, item: Name) {
        self.stack.push(item)
    }
    fn peek(&self) -> Option<Name> {
        self.stack.peek()
    }
    fn len(&self) -> usize {
        self.stack.len()
    }
    fn duplicate_top_item(&self) {
        self.stack.duplicate_top_item()
    }
    fn clear(&self) {
        self.stack.clear()
    }
    fn rotate(&self) {
        self.stack.rotate()
    }
    fn shove(&self, position: i64) -> bool {
        self.stack.shove(position)
    }
    fn swap(&self) {
        self.stack.swap()
    }
    fn yank(&self, position: i64) -> bool {
        self.stack.yank(position)
    }
    fn yank_duplicate(&self, position: i64) -> bool {
        self.stack.yank_duplicate(position)
    }
}



instruction! {
    /// Duplicates the top item on the NAME stack. Does not pop its argument (which, if it did, would negate the effect
    /// of the duplication!).
    #[stack(Name)]
    fn dup(context: &mut Context) {
        context.name().duplicate_top_item();
    }
}

instruction! {
    /// Pushes TRUE if the top two NAMEs are equal, or FALSE otherwise.
    #[stack(Name)]
    fn equal(context: &mut Context, a: Name, b: Name) {
        context.bool().push(a == b);
    }
}

instruction! {
    /// Empties the NAME stack.
    #[stack(Name)]
    fn flush(context: &mut Context) {
        context.name().clear()
    }
}

instruction! {
    /// Pops the NAME stack.
    #[stack(Name)]
    fn pop(context: &mut Context, _popped: Name) {}
}

instruction! {
    /// Sets a flag indicating that the next name encountered will be pushed onto the NAME stack (and not have its
    /// associated value pushed onto the EXEC stack), regardless of whether or not it has a definition. Upon
    /// encountering such a name and pushing it onto the NAME stack the flag will be cleared (whether or not the pushed
    /// name had a definition).
    #[stack(Name)]
    fn quote(context: &mut Context) {
        context.name().set_should_quote_next_name(true)
    }
}

instruction! {
    /// Pushes a randomly selected NAME that already has a definition.
    #[stack(Name)]
    fn rand_bound_name(context: &mut Context) {
        let defined_names = context.name().all_names();
        let random_value = context.run_random_literal_function(|rng| {
            let pick = rng.gen_range(0..defined_names.len());
            defined_names[pick].clone()
        });
        context.name().push(random_value);
    }
}

instruction! {
    /// Pushes a newly generated random NAME.
    #[stack(Name)]
    fn rand(context: &mut Context) {
        let random_value = context.run_random_literal_function(Name::random_value);
        context.name().push(random_value);
    }
}

instruction! {
    /// Rotates the top three items on the NAME stack, pulling the third item out and pushing it on top. This is
    /// equivalent to "2 NAME.YANK".
    #[stack(Name)]
    fn rot(context: &mut Context) {
        context.name().rotate();
    }
}

instruction! {
    /// Inserts the top NAME "deep" in the stack, at the position indexed by the top INTEGER.
    #[stack(Name)]
    fn shove(context: &mut Context, position: Integer) {
        if !context.name().shove(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Pushes the stack depth onto the INTEGER stack.
    #[stack(Name)]
    fn stack_depth(context: &mut Context) {
        context.integer().push(context.name().len() as i64);
    }
}

instruction! {
    /// Swaps the top two NAMEs.
    #[stack(Name)]
    fn swap(context: &mut Context) {
        context.name().swap();
    }
}

instruction! {
    /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    /// The index is taken from the INTEGER stack.
    #[stack(Name)]
    fn yank_dup(context: &mut Context, position: Integer) {
        if !context.name().yank_duplicate(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
    /// INTEGER stack.
    #[stack(Name)]
    fn yank(context: &mut Context, position: Integer) {
        if !context.name().yank(position) {
            context.integer().push(position);
        }
    }
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
