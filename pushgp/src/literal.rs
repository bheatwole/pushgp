use nom::IResult;
use rand::rngs::SmallRng;
use std::fmt::Display;
use std::hash::Hash;

/// This is a marker trait for all the other traits a literal must implement so that it can participate as a member of
/// the Code<L> enum
pub trait Literal<Value>: Clone + Display + Eq + Hash + PartialEq {
    /// All literals must be parsable by 'nom' from a string into a value
    fn parse(input: &str) -> IResult<&str, Value>;

    /// All literals must also be able to write to a string that can later be parsed by nom. The default implementation
    /// simply uses the std::fmt::Display trait for the Value, but this allows for overriding type aliases for literals
    /// that already have a Display implementation.
    fn nom_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }

    /// All literals must be able to provide a random value for code generation purposes
    fn random_value(rng: &mut SmallRng) -> Value;
}

/// This is the trait that must be implemented for the wrapper around all Literal types. While it mostly has the same
/// functions as the Literal trait, the usage is different enough that it warrants it's own trait.
pub trait LiteralEnum<EnumWrapper>: Clone + Display + Eq + Hash + PartialEq {
    /// All literals must be parsable by 'nom' from a string into a value
    fn parse(input: &str) -> IResult<&str, EnumWrapper>;

    /// All literals must also be able to write to a string that can later be parsed by nom. The default implementation
    /// simply uses the std::fmt::Display trait for the Value, but this allows for overriding type aliases for literals
    /// that already have a Display implementation.
    fn nom_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// This trait allows generic code to test for the existence of a Literal type in the LiteralEnum and still function
/// (with reduced functionality) if that Literal type is not supported by the particular program. For example, our
/// generic code needs to be able to know what to do when it encounters a name, but not every context will support
/// names. 
pub trait LiteralEnumHasLiteralValue<Enum, Value>
where
    Enum: LiteralEnum<Enum>,
    Value: Literal<Value>,
{
    fn supports_literal_type() -> bool {
        false
    }

    fn make_from_value(_value: Value) -> Enum {
        panic!("this context does not support literals of this type!")
    }
}
