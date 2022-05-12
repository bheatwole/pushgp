use crate::Name;
use nom::IResult;
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
    fn random_value<R: rand::Rng>(rng: &mut R) -> Value;
}

pub trait SupportsLiteralNames<L: Literal<L>> {
    fn supports_literal_names() -> bool {
        false
    }

    fn make_literal_name(_name: Name) -> L {
        panic!("this type does not support literal names!")
    }
}