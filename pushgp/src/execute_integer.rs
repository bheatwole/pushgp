use crate::*;
use pushgp_macros::*;
use rand::Rng;
use rust_decimal::prelude::ToPrimitive;

pub type Integer = i64;

impl Literal<Integer> for Integer {
    fn parse(input: &str) -> nom::IResult<&str, Integer> {
        crate::parse::parse_code_integer(input)
    }

    fn random_value(rng: &mut rand::rngs::SmallRng) -> Integer {
        rng.gen_range(i64::MIN..=i64::MAX)
    }
}

pub trait ContextHasIntegerStack<L: LiteralEnum<L>> {
    fn integer(&self) -> &Stack<Integer>;
    fn make_literal_integer(value: Integer) -> Code<L>;
}

instruction! {
    /// Defines the name on top of the NAME stack as an instruction that will push the top item of the INTEGER stack
    /// onto the EXEC stack.
    #[stack(Integer)]
    fn define(context: &mut Context, value: Integer, name: Name) {
        context.name().define_name(name, C::make_literal_integer(value));
    }
}

instruction! {
    /// Pushes the difference of the top two items; that is, the second item minus the top item.
    #[stack(Integer)]
    fn difference(context: &mut Context, right: Integer, left: Integer) {
        context.integer().push(left - right);
    }
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
    /// Empties the INTEGER stack.
    #[stack(Integer)]
    fn flush(context: &mut Context) {
        context.integer().clear();
    }
}

instruction! {
    /// Pushes 1 if the top BOOLEAN is TRUE, or 0 if the top BOOLEAN is FALSE.
    #[stack(Integer)]
    fn from_boolean(context: &mut Context, value: Bool) {
        context.integer().push(if value { 1 } else { 0 });
    }
}

instruction! {
    /// Pushes the result of truncating the top FLOAT.
    #[stack(Integer)]
    fn from_float(context: &mut Context, value: Float) {
        context.integer().push(value.to_i64().unwrap());
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or FALSE otherwise.
    #[stack(Integer)]
    fn greater(context: &mut Context, right: Integer, left: Integer) {
        context.bool().push(left > right);
    }
}

instruction! {
    /// Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or FALSE otherwise.
    #[stack(Integer)]
    fn less(context: &mut Context, right: Integer, left: Integer) {
        context.bool().push(left < right);
    }
}

instruction! {
    /// Pushes the maximum of the top two items.
    #[stack(Integer)]
    fn max(context: &mut Context, a: Integer, b: Integer) {
        context.integer().push(if a > b { a } else { b });
    }
}

instruction! {
    /// Pushes the minimum of the top two items.
    #[stack(Integer)]
    fn min(context: &mut Context, a: Integer, b: Integer) {
        context.integer().push(if a < b { a } else { b });
    }
}

instruction! {
    /// Pushes the second stack item modulo the top stack item. If the top item is zero this acts as a NOOP. The modulus
    /// is computed as the remainder of the quotient, where the quotient has first been truncated toward negative
    /// infinity. (This is taken from the definition for the generic MOD function in Common Lisp, which is described for
    /// example at http://www.lispworks.com/reference/HyperSpec/Body/f_mod_r.htm.)
    #[stack(Integer)]
    fn modulo(context: &mut Context, bottom: Integer, top: Integer) {
        if bottom != 0 {
            context.integer().push(top % bottom);
        }
    }
}

instruction! {
    /// Pops the INTEGER stack.
    #[stack(Integer)]
    fn pop(context: &mut Context, _popped: Integer) {
    }
}

instruction! {
    /// Pushes the product of the top two items.
    #[stack(Integer)]
    fn product(context: &mut Context, right: Integer, left: Integer) {
        context.integer().push(left * right);
    }
}

instruction! {
    /// Pushes the quotient of the top two items; that is, the second item divided by the top item. If the top item is
    /// zero this acts as a NOOP.
    #[stack(Integer)]
    fn quotient(context: &mut Context, bottom: Integer, top: Integer) {
        if bottom != 0 {
            context.integer().push(top / bottom);
        }
    }
}

instruction! {
    /// Pushes a newly generated random INTEGER that is greater than or equal to MIN-RANDOM-INTEGER and less than or
    /// equal to MAX-RANDOM-INTEGER.
    #[stack(Integer)]
    fn rand(context: &mut Context) {
        let random_value = context.run_random_literal_function(Integer::random_value);
        context.integer().push(random_value);
    }
}

instruction! {
    /// Rotates the top three items on the INTEGER stack, pulling the third item out and pushing it on top. This is
    /// equivalent to "2 INTEGER.YANK".
    #[stack(Integer)]
    fn rot(context: &mut Context) {
        context.integer().rotate()
    }
}

instruction! {
    /// Inserts the second INTEGER "deep" in the stack, at the position indexed by the top INTEGER. The index position
    /// is calculated after the index is removed.
    #[stack(Integer)]
    fn shove(context: &mut Context, position: Integer) {
        if !context.integer().shove(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
    #[stack(Integer)]
    fn stack_depth(context: &mut Context) {
        context.integer().push(context.integer().len() as i64);
    }
}

instruction! {
    /// Pushes the sum of the top two items.
    #[stack(Integer)]
    fn sum(context: &mut Context, a: Integer, b: Integer) {
        context.integer().push(a + b);
    }
}

instruction! {
    /// Swaps the top two INTEGERs.
    #[stack(Integer)]
    fn swap(context: &mut Context) {
        context.integer().swap();
    }
}

instruction! {
    /// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
    /// The index is taken from the INTEGER stack, and the indexing is done after the index is removed.
    #[stack(Integer)]
    fn yank_dup(context: &mut Context, position: Integer) {
        if !context.integer().yank_duplicate(position) {
            context.integer().push(position);
        }
    }
}

instruction! {
    /// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
    /// INTEGER stack, and the indexing is done after the index is removed.
    #[stack(Integer)]
    fn yank(context: &mut Context, position: Integer) {
        if !context.integer().yank(position) {
            context.integer().push(position);
        }
    }
}
