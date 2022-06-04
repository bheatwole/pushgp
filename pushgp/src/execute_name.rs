use crate::*;
use base64::*;
use byte_slice_cast::*;
use pushgp_macros::*;
use rand::Rng;

pub type Name = String;

pub trait MustHaveNameStackInContext {
    fn name(&self) -> Stack<Name>;
    fn make_literal_name(&self, value: Name) -> Code;
}

impl MustHaveNameStackInContext for Context {
    fn name(&self) -> Stack<Name> {
        Stack::<Name>::new(self.get_stack("Name").unwrap())
    }

    fn make_literal_name(&self, value: Name) -> Code {
        let id = self.get_virtual_table().id_for_name(NameLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_string(value)))
    }
}

impl From<InstructionData> for Name {
    fn from(data: InstructionData) -> Self {
        data.get_string().unwrap()
    }
}

impl Into<InstructionData> for Name {
    fn into(self) -> InstructionData {
        InstructionData::from_string(self)
    }
}

pub struct NameLiteralValue {}
impl Instruction for NameLiteralValue {
    /// Every instruction must have a name
    fn name() -> &'static str {
        "NAME.LITERALVALUE"
    }

    /// All instructions must be parsable by 'nom' from a string. Parsing an instruction will either return an error to
    /// indicate the instruction was not found, or the optional data, indicating the instruction was found and parsing
    /// should cease.
    fn parse(input: &str) -> nom::IResult<&str, Option<InstructionData>> {
        let (rest, value) = crate::parse::parse_code_name(input)?;
        Ok((rest, Some(InstructionData::from_string(value))))
    }

    /// All instructions must also be able to write to a string that can later be parsed by nom.
    fn nom_fmt(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", data.as_ref().unwrap().get_string().unwrap())
    }

    /// If the instruction makes use of InstructionData, it must be able to generate a random value for code generation.
    /// If it does not use InstructionData, it just returns None
    fn random_value(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        let random_value = rng.gen_range(0..=u64::MAX);

        let slice: [u64; 1] = [random_value];
        let b64 = encode(slice.as_byte_slice());
        let name = "RND.".to_owned() + &b64;
        Some(InstructionData::from_string(name))
    }

    /// Instructions are pure functions on a Context and optional InstructionData. All parameters are read from the
    /// Context and/or data and all outputs are updates to the Context.
    fn execute(context: &crate::context::Context, data: Option<InstructionData>) {
        let name = data.unwrap().get_string().unwrap();
        if context.should_quote_next_name() {
            context.get_stack("Name").unwrap().push(InstructionData::from_string(name));
            context.set_should_quote_next_name(false);
        } else {
            match context.definition_for_name(&name) {
                None => context.get_stack("Name").unwrap().push(InstructionData::from_string(name)),
                Some(code) => context.exec().push(code.into()),
            }
        }
    }

    fn add_to_virtual_table(table: &mut VirtualTable) {
        table.add_entry(Self::name(), Self::parse, Self::nom_fmt, Self::random_value, Self::execute);
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
        context.set_should_quote_next_name(true)
    }
}

instruction! {
    /// Pushes a randomly selected NAME that already has a definition.
    #[stack(Name)]
    fn rand_bound_name(context: &mut Context) {
        let defined_names = context.all_defined_names();
        if defined_names.len() > 0 {
            let random_value = context.run_random_function(|rng| {
                let pick: usize = rng.gen_range(0..defined_names.len());
                defined_names[pick].clone()
            });
            context.name().push(random_value);
        }
    }
}

instruction! {
    /// Pushes a newly generated random NAME.
    #[stack(Name)]
    fn rand(context: &mut Context) {
        let random_value = context.run_random_function(NameLiteralValue::random_value).unwrap();
        context.get_stack("Name").unwrap().push(random_value);
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
