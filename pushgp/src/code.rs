use crate::Instruction;
use base64::*;
use byte_slice_cast::*;
use std::fmt::{Display, Formatter, Result};

// Code is the basic building block of a PushGP program. It's the translation between human readable and machine
// readable strings.
#[derive(Clone, Debug, PartialEq)]
pub enum Code {
    // A list is just a list containing other code (which can be lists) and may also be empty (len() == 0)
    List(Vec<Code>),

    // Code can be literal values
    LiteralBool(bool),
    LiteralFloat(f64),
    LiteralInteger(i64),
    LiteralName(u64),

    // Code can be an instruction
    Instruction(Instruction),
}

impl Code {
    pub fn new(src: &str) -> Code {
        crate::parse::parse_code(src)
    }

    pub fn is_list(&self) -> bool {
        match &self {
            Code::List(_) => true,
            _ => false,
        }
    }
    pub fn is_atom(&self) -> bool {
        !self.is_list()
    }

    pub fn take_list(self) -> Option<Vec<Code>> {
        match self {
            Code::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn to_list(&self) -> Code {
        match &self {
            Code::List(x) => Code::List(x.clone()),
            Code::LiteralBool(b) => Code::List(vec![Code::LiteralBool(*b)]),
            Code::LiteralFloat(f) => Code::List(vec![Code::LiteralFloat(*f)]),
            Code::LiteralInteger(i) => Code::List(vec![Code::LiteralInteger(*i)]),
            Code::LiteralName(n) => Code::List(vec![Code::LiteralName(*n)]),
            Code::Instruction(inst) => Code::List(vec![Code::Instruction(*inst)]),
        }
    }

    pub fn points(&self) -> i64 {
        match &self {
            Code::List(x) => {
                let sub_points: i64 = x.iter().map(|c| c.points()).sum();
                1 + sub_points
            }
            _ => 1,
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Code::List(x) => {
                write!(f, "(")?;
                for c in x.iter() {
                    write!(f, " {}", c)?;
                }
                write!(f, " )")
            }
            Code::LiteralBool(v) => write!(f, "{}", if *v { "TRUE" } else { "FALSE" }),
            Code::LiteralFloat(v) => write!(f, "{}", v),
            Code::LiteralInteger(v) => write!(f, "{}", v),
            Code::LiteralName(v) => {
                let slice: [u64; 1] = [*v];
                let b64 = encode(slice.as_byte_slice());
                write!(f, "{}", b64)
            }
            Code::Instruction(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Instruction};

    #[test]
    fn code_display() {
        let code = Code::List(vec![]);
        assert_eq!("( )", format!("{}", code));

        let code = Code::List(vec![
            Code::List(vec![
                Code::LiteralBool(true),
                Code::LiteralFloat(0.012345),
                Code::LiteralInteger(-12784),
                Code::LiteralName(9000),
            ]),
            Code::Instruction(Instruction::BoolAnd),
        ]);
        assert_eq!(
            "( ( TRUE 0.012345 -12784 KCMAAAAAAAA= ) BOOLAND )",
            format!("{}", code)
        );
    }

    #[test]
    fn code_points() {
        let code = Code::List(vec![
            Code::List(vec![
                Code::LiteralBool(true),
                Code::LiteralFloat(0.012345),
                Code::LiteralInteger(-12784),
                Code::LiteralName(9000),
            ]),
            Code::Instruction(Instruction::BoolAnd),
        ]);
        assert_eq!(7, code.points());
    }
}
