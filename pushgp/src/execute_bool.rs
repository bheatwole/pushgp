use crate::*;
use pushgp_macros::*;
use rand::Rng;

pub type Bool = bool;

pub trait MustHaveBoolStackInContext {
    fn bool(&self) -> Stack<Bool>;
    fn make_literal_bool(&self, value: Bool) -> Code;
}

impl MustHaveBoolStackInContext for NewContext {
    fn bool(&self) -> Stack<Bool> {
        Stack::<Bool>::new(self.get_stack("Bool").unwrap())
    }

    fn make_literal_bool(&self, value: Bool) -> Code {
        let id = self.id_for_name(BoolLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_bool(value)))
    }
}

impl From<InstructionData> for Bool {
    fn from(data: InstructionData) -> Self {
        data.get_bool().unwrap()
    }
}

impl Into<InstructionData> for Bool {
    fn into(self) -> InstructionData {
        InstructionData::from_bool(self)
    }
}

pub struct BoolLiteralValue {}
impl Instruction for BoolLiteralValue {
    /// Every instruction must have a name
    fn name() -> &'static str {
        "BOOL.LITERALVALUE"
    }

    /// All instructions must be parsable by 'nom' from a string. Parsing an instruction will either return an error to
    /// indicate the instruction was not found, or the optional data, indicating the instruction was found and parsing
    /// should cease.
    fn parse(input: &str) -> nom::IResult<&str, Option<InstructionData>> {
        let (rest, value) = crate::parse::parse_code_bool(input)?;
        Ok((rest, Some(InstructionData::from_bool(value))))
    }

    /// All instructions must also be able to write to a string that can later be parsed by nom.
    fn nom_fmt(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if data.as_ref().unwrap().get_bool().unwrap() { "TRUE" } else { "FALSE" })
    }

    /// If the instruction makes use of InstructionData, it must be able to generate a random value for code generation.
    /// If it does not use InstructionData, it just returns None
    fn random_value(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        Some(InstructionData::from_bool(if 0 == rng.gen_range(0..=1) { false } else { true }))
    }

    /// Instructions are pure functions on a Context and optional InstructionData. All parameters are read from the
    /// Context and/or data and all outputs are updates to the Context.
    fn execute(context: &crate::context::NewContext, data: Option<InstructionData>) {
        context.get_stack("Bool").unwrap().push(data.unwrap())
    }

    fn add_to_virtual_table(table: &mut VirtualTable) {
        table.add_entry(Self::name(), Self::parse, Self::nom_fmt, Self::random_value, Self::execute);
    }
}

instruction! {
    /// Pushes the logical AND of the top two BOOLEANs onto the EXEC stack
    #[stack(Bool)]
    fn and(context: &mut Context, a: Bool, b: Bool) {
        context.bool().push(a && b);
    }
}

instruction! {
    /// Defines the name on top of the NAME stack as an instruction that will push the top item of the BOOLEAN stack
    #[stack(Bool)]
    fn define(context: &mut Context, value: Bool, name: Name) {
        context.define_name(name, context.make_literal_bool(value));
    }
}

instruction! {
    /// Duplicates the top item on the BOOLEAN stack. Does not pop its argument (which, if it did, would negate the
    /// effect of the duplication!)
    #[stack(Bool)]
    fn dup(context: &mut Context) {
        context.bool().duplicate_top_item();
    }
}

instruction! {
    /// Pushes TRUE if the top two BOOLEANs are equal, or FALSE otherwise
    #[stack(Bool)]
    fn equal(context: &mut Context, a: Bool, b: Bool) {
        context.bool().push(a == b);
    }
}

instruction! {
    /// Empties the BOOLEAN stack
    #[stack(Bool)]
    fn flush(context: &mut Context) {
        context.bool().clear();
    }
}

instruction! {
    /// Pushes FALSE if the top FLOAT is 0.0, or TRUE otherwise
    #[stack(Bool)]
    fn from_float(context: &mut Context, f: Float) {
        context.bool().push(!f.is_zero());
    }
}

instruction! {
    /// Pushes FALSE if the top INTEGER is 0, or TRUE otherwise
    #[stack(Bool)]
    fn from_int(context: &mut Context, i: Integer) {
        context.bool().push(i != 0);
    }
}

instruction! {
    /// Pushes the logical NOT of the top BOOLEAN
    #[stack(Bool)]
    fn not(context: &mut Context, b: Bool) {
        context.bool().push(!b);
    }
}

instruction! {
    /// Pushes the logical OR of the top two BOOLEANs
    #[stack(Bool)]
    fn or(context: &mut Context, a: Bool, b: Bool) {
        context.bool().push(a || b);
    }
}

instruction! {
    /// Pops the BOOLEAN stack
    #[stack(Bool)]
    fn pop(context: &mut Context, _a: Bool) {
    }
}

instruction! {
    /// Pushes a random BOOLEAN
    #[stack(Bool)]
    fn rand(context: &mut Context) {
        let random_bool = context.run_random_literal_function(BoolLiteralValue::random_value).unwrap();
        context.get_stack("Bool").unwrap().push(random_bool);
    }
}

instruction! {
    /// Rotates the top three items on the BOOLEAN stack, pulling the third item out and pushing it on top. This is
    /// equivalent to "2 BOOLEAN.YANK"
    #[stack(Bool)]
    fn rot(context: &mut Context) {
        context.bool().rotate();
    }
}

instruction! {
    /// Inserts the top BOOLEAN "deep" in the stack, at the position indexed by the top INTEGER
    #[stack(Bool)]
    fn shove(context: &mut Context, position: Integer) {
        if !context.bool().shove(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Pushes the stack depth onto the INTEGER stack
    #[stack(Bool)]
    fn stack_depth(context: &mut Context) {
        context.integer().push(context.bool().len() as i64);
    }
}

instruction! {
    /// Swaps the top two BOOLEANs
    #[stack(Bool)]
    fn swap(context: &mut Context) {
        context.bool().swap();
    }
}

instruction! {
    /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    /// The index is taken from the INTEGER stack
    #[stack(Bool)]
    fn yank_dup(context: &mut Context, position: Integer) {
        if !context.bool().yank_duplicate(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from theF
    /// INTEGER stack
    #[stack(Bool)]
    fn yank(context: &mut Context, position: Integer) {
        if !context.bool().yank(position) {
            context.integer().push(position);
        }
    }
}
