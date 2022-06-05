use crate::*;
use pushgp_macros::*;
use rand::Rng;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

pub type Float = Decimal;

pub trait MustHaveFloatStackInContext {
    fn float(&self) -> Stack<Float>;
    fn make_literal_float(&self, value: Float) -> Code;
}

impl<State: std::fmt::Debug + Clone> MustHaveFloatStackInContext for Context<State> {
    fn float(&self) -> Stack<Float> {
        Stack::<Float>::new(self.get_stack("Float"))
    }

    fn make_literal_float(&self, value: Float) -> Code {
        let id = self.get_virtual_table().id_for_name(FloatLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_f64(value.to_f64().unwrap())))
    }
}

impl From<InstructionData> for Float {
    fn from(data: InstructionData) -> Self {
        Decimal::from_f64(data.get_f64().unwrap()).unwrap()
    }
}

impl Into<InstructionData> for Float {
    fn into(self) -> InstructionData {
        InstructionData::from_f64(self.to_f64().unwrap())
    }
}

pub struct FloatLiteralValue {}
impl Instruction for FloatLiteralValue {
    /// Every instruction must have a name
    fn name() -> &'static str {
        "FLOAT.LITERALVALUE"
    }

    /// All instructions must be parsable by 'nom' from a string. Parsing an instruction will either return an error to
    /// indicate the instruction was not found, or the optional data, indicating the instruction was found and parsing
    /// should cease.
    fn parse(input: &str) -> nom::IResult<&str, Option<InstructionData>> {
        let (rest, value) = crate::parse::parse_code_float(input)?;
        Ok((rest, Some(InstructionData::from_f64(value.to_f64().unwrap()))))
    }

    /// All instructions must also be able to write to a string that can later be parsed by nom.
    fn nom_fmt(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", data.as_ref().unwrap().get_f64().unwrap())
    }

    /// If the instruction makes use of InstructionData, it must be able to generate a random value for code generation.
    /// If it does not use InstructionData, it just returns None
    fn random_value(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        let float: f64 = rng.gen_range(-1f64..1f64);
        Some(InstructionData::from_f64(float))
    }

    /// Instructions are pure functions on a Context and optional InstructionData. All parameters are read from the
    /// Context and/or data and all outputs are updates to the Context.
    fn execute<State: std::fmt::Debug + Clone>(context: &crate::context::Context<State>, data: Option<InstructionData>) {
        context.get_stack("Float").unwrap().push(data.unwrap())
    }

    fn add_to_virtual_table<State: std::fmt::Debug + Clone>(table: &mut VirtualTable<State>) {
        table.add_entry(Self::name(), Self::parse, Self::nom_fmt, Self::random_value, Self::execute);
    }
}

instruction! {
    /// Pushes the cosine of the top item.F
    #[stack(Float)]
    fn cos(context: &mut Context, value: Float) {
        context.float().push(Decimal::from_f64(value.to_f64().unwrap().cos()).unwrap());
    }
}

instruction! {
    /// Defines the name on top of the NAME stack as an instruction that will push the top item of the FLOAT stack onto
    /// the EXEC stack.
    #[stack(Float)]
    fn define(context: &mut Context, value: Float, name: Name) {
        context.define_name(name, context.make_literal_float(value));
    }
}

instruction! {
    /// Pushes the difference of the top two items; that is, the second item minus the top item.
    #[stack(Float)]
    fn difference(context: &mut Context, right: Float, left: Float) {
        context.float().push(left - right);
    }
}

instruction! {
    /// Duplicates the top item on the FLOAT stack. Does not pop its argument (which, if it did, would negate the effect
    /// of the duplication!).
    #[stack(Float)]
    fn dup(context: &mut Context) {
        context.float().duplicate_top_item();
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE otherwise.
    #[stack(Float)]
    fn equal(context: &mut Context, a: Float, b: Float) {
        context.bool().push(a == b);
    }
}

instruction! {
    /// Empties the FLOAT stack.
    #[stack(Float)]
    fn flush(context: &mut Context) {
        context.float().clear();
    }
}

instruction! {
    /// Pushes 1.0 if the top BOOLEAN is TRUE, or 0.0 if the top BOOLEAN is FALSE.
    #[stack(Float)]
    fn from_boolean(context: &mut Context, value: Bool) {
        context.float().push(if value {
            Decimal::new(1, 0)
        } else {
            Decimal::new(0, 0)
        });
    }
}

instruction! {
    /// Pushes a floating point version of the top INTEGER.
    #[stack(Float)]
    fn from_integer(context: &mut Context, value: Integer) {
        context.float().push(Decimal::new(value, 0));
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or FALSE otherwise.
    #[stack(Float)]
    fn greater(context: &mut Context, right: Float, left: Float) {
        context.bool().push(left > right);
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or FALSE otherwise.
    #[stack(Float)]
    fn less(context: &mut Context, right: Float, left: Float) {
        context.bool().push(left < right);
    }
}

instruction! {
    /// Pushes the maximum of the top two items.
    #[stack(Float)]
    fn max(context: &mut Context, a: Float, b: Float) {
        context.float().push(if a > b { a } else { b });
    }
}

instruction! {
    /// Pushes the minimum of the top two items.
    #[stack(Float)]
    fn min(context: &mut Context, a: Float, b: Float) {
        context.float().push(if a < b { a } else { b });
    }
}

instruction! {
    /// Pushes the second stack item modulo the top stack item. If the top item is zero this acts as a NOOP. The modulus
    /// is computed as the remainder of the quotient, where the quotient has first been truncated toward negative
    /// infinity. (This is taken from the definition for the generic MOD function in Common Lisp, which is described for
    /// example at http://www.lispworks.com/reference/HyperSpec/Body/f_mod_r.htm.)
    #[stack(Float)]
    fn modulo(context: &mut Context, bottom: Float, top: Float) {
        if bottom != Decimal::ZERO {
            context.float().push(top % bottom);
        }
    }
}

instruction! {
    /// Pops the FLOAT stack.
    #[stack(Float)]
    fn pop(context: &mut Context, _popped: Float) {
    }
}

instruction! {
    /// Pushes the product of the top two items.
    #[stack(Float)]
    fn product(context: &mut Context, right: Float, left: Float) {
        context.float().push(left * right);
    }
}

instruction! {
    /// Pushes the quotient of the top two items; that is, the second item divided by the top item. If the top item is
    /// zero this acts as a NOOP.
    #[stack(Float)]
    fn quotient(context: &mut Context, bottom: Float, top: Float) {
        if bottom != Decimal::ZERO {
            context.float().push(top / bottom);
        }
    }
}

instruction! {
    /// Pushes a newly generated random FLOAT that is greater than or equal to MIN-RANDOM-FLOAT and less than or equal
    /// to MAX-RANDOM-FLOAT.
    #[stack(Float)]
    fn rand(context: &mut Context) {
        let random_value = context.run_random_function(FloatLiteralValue::random_value).unwrap();
        context.get_stack("Float").unwrap().push(random_value);
    }
}

instruction! {
    /// Rotates the top three items on the FLOAT stack, pulling the third item out and pushing it on top. This is
    /// equivalent to "2 FLOAT.YANK".
    #[stack(Float)]
    fn rot(context: &mut Context) {
        context.float().rotate();
    }
}

instruction! {
    /// Inserts the top FLOAT "deep" in the stack, at the position indexed by the top INTEGER.
    #[stack(Float)]
    fn shove(context: &mut Context, position: Integer) {
        if !context.float().shove(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Pushes the sine of the top item.
    #[stack(Float)]
    fn sin(context: &mut Context, value: Float) {
        context.float().push(Decimal::from_f64(value.to_f64().unwrap().sin()).unwrap());
    }
}

instruction! {
    /// Pushes the stack depth onto the INTEGER stack.
    #[stack(Float)]
    fn stack_depth(context: &mut Context) {
        context.integer().push(context.float().len() as i64);
    }
}

instruction! {
    /// Pushes the sum of the top two items.
    #[stack(Float)]
    fn sum(context: &mut Context, right: Float, left: Float) {
        context.float().push(left + right);
    }
}

instruction! {
    /// Swaps the top two BOOLEANs.
    #[stack(Float)]
    fn swap(context: &mut Context) {
        context.float().swap();
    }
}

instruction! {
    /// Pushes the tangent of the top item.
    #[stack(Float)]
    fn tan(context: &mut Context, value: Float) {
        context.float().push(Decimal::from_f64(value.to_f64().unwrap().tan()).unwrap());
    }
}

instruction! {
    /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    /// The index is taken from the INTEGER stack.
    #[stack(Float)]
    fn yank_dup(context: &mut Context, position: Integer) {
        if !context.float().yank_duplicate(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
    /// INTEGER stack.
    #[stack(Float)]
    fn yank(context: &mut Context, position: Integer) {
        if !context.float().yank(position) {
            context.integer().push(position);
        }
    }
}
