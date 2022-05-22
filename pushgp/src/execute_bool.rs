use crate::*;
use pushgp_macros::*;

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
        context.name().define_name(name, C::make_literal_bool(value));
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
        //context.bool().push(Bool::random_value());
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
