use crate::Instruction;
use base64::*;
use byte_slice_cast::*;
use fnv::FnvHashMap;
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter, Result};

// Code is the basic building block of a PushGP program. It's the translation between human readable and machine
// readable strings.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Code {
    // A list is just a list containing other code (which can be lists) and may also be empty (len() == 0)
    List(Vec<Code>),

    // Code can be literal values
    LiteralBool(bool),
    LiteralFloat(Decimal),
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

    pub fn contains(&self, look_for: &Code) -> bool {
        if self == look_for {
            return true;
        }
        match &self {
            Code::List(list) => {
                for i in list {
                    if i.contains(look_for) {
                        return true;
                    }
                }
                false
            }
            x => *x == look_for,
        }
    }

    pub fn container(&self, look_for: &Code) -> Option<Code> {
        match &self {
            Code::List(list) => {
                for i in list {
                    if i == look_for {
                        return Some(Code::List(list.clone()));
                    }
                }
                for i in list {
                    if let Some(code) = i.container(&look_for) {
                        return Some(code);
                    }
                }
                None
            }
            _ => None,
        }
    }

    // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
    pub fn discrepancy_items(&self) -> FnvHashMap<Code, i64> {
        let mut items = FnvHashMap::default();

        match &self {
            Code::List(list) => {
                for i in list {
                    if i.is_list() {
                        let counter = items.entry(i.clone()).or_insert(0);
                        *counter += 1;
                    }
                    for (key, count) in i.discrepancy_items() {
                        let counter = items.entry(key).or_insert(0);
                        *counter += count;
                    }
                }
            }
            &atom => {
                let counter = items.entry(atom.clone()).or_insert(0);
                *counter += 1;
            }
        }

        items
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
    use rust_decimal::Decimal;

    #[test]
    fn code_display() {
        let code = Code::List(vec![]);
        assert_eq!("( )", format!("{}", code));

        let code = Code::List(vec![
            Code::List(vec![
                Code::LiteralBool(true),
                Code::LiteralFloat(Decimal::new(12345, 6)),
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
                Code::LiteralFloat(Decimal::new(12345, 6)),
                Code::LiteralInteger(-12784),
                Code::LiteralName(9000),
            ]),
            Code::Instruction(Instruction::BoolAnd),
        ]);
        assert_eq!(7, code.points());
    }

    #[test]
    fn code_discrepancy_items() {
        // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
        let code = Code::new("( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )");
        let items = code.discrepancy_items();
        assert_eq!(1, *items.get(&Code::new("ANAME")).unwrap());
        assert_eq!(1, *items.get(&Code::new("( 3 ( 1 ) )")).unwrap());
        assert_eq!(1, *items.get(&Code::new("3")).unwrap());
        assert_eq!(2, *items.get(&Code::new("( 1 )")).unwrap());
        assert_eq!(3, *items.get(&Code::new("1")).unwrap());
        assert_eq!(5, items.len());
    }
}
