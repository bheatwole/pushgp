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
    InstructionByName(String),
}

impl Code {
    /// Create new code from the specified text
    pub fn new(src: &str) -> Code {
        crate::parse::parse_code(src)
    }

    /// Returns true if this code is a List
    pub fn is_list(&self) -> bool {
        match &self {
            Code::List(_) => true,
            _ => false,
        }
    }

    /// Returns true if this code is anything BUT a list
    pub fn is_atom(&self) -> bool {
        !self.is_list()
    }

    /// Returns true if the specified code is equal to this item or any child
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

    /// Returns the smallest sub-list that contains the specified code
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

    /// Similar to `contains` but does not recurse into Lists
    pub fn has_member(&self, look_for: &Code) -> bool {
        match &self {
            Code::List(list) => {
                for item in list {
                    if item == look_for {
                        return true;
                    }
                }
                false
            }
            &x => x == look_for,
        }
    }

    /// Similar to `extract_point` but does not recurse into lists
    pub fn position_of(&self, look_for: &Code) -> Option<usize> {
        match &self {
            Code::List(list) => {
                for (i, item) in list.iter().enumerate() {
                    if item == look_for {
                        return Some(i);
                    }
                }
                None
            }
            &x => {
                if x == look_for {
                    Some(0)
                } else {
                    None
                }
            }
        }
    }

    /// The discrepancy output is a hashset of every unique sub-list and atom from the specified code
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

    /// Coerces the item to a list, taking ownership. This is similar to a call to 'unwrap'
    pub fn to_list(self) -> Vec<Code> {
        match self {
            Code::List(x) => x,
            Code::LiteralBool(b) => vec![Code::LiteralBool(b)],
            Code::LiteralFloat(f) => vec![Code::LiteralFloat(f)],
            Code::LiteralInteger(i) => vec![Code::LiteralInteger(i)],
            Code::LiteralName(n) => vec![Code::LiteralName(n)],
            Code::Instruction(inst) => vec![Code::Instruction(inst)],
            Code::InstructionByName(name) => vec![Code::InstructionByName(name)],
        }
    }

    /// Returns the number of 'points' of the entire code. Each atom and list is considered one point.
    pub fn points(&self) -> i64 {
        match &self {
            Code::List(x) => {
                let sub_points: i64 = x.iter().map(|c| c.points()).sum();
                1 + sub_points
            }
            _ => 1,
        }
    }

    /// Returns the item of code at the specified 'point' in the code tree if `point` is less than the number of points
    /// in the code. Returns the number of points used otherwise.
    pub fn extract_point(&self, point: i64) -> Extraction {
        if 0 == point {
            return Extraction::Extracted(self.clone());
        }
        match &self {
            Code::List(list) => {
                let mut used = 1;
                for item in list {
                    match item.extract_point(point - used) {
                        Extraction::Extracted(code) => return Extraction::Extracted(code),
                        Extraction::Used(u) => used += u,
                    }
                }
                Extraction::Used(used)
            }
            _ => Extraction::Used(1),
        }
    }

    /// Descends to the specified point in the code tree and swaps the list or atom there with the specified replacement
    /// code. If the replacement point is greater than the number of points in the Code, this has no effect.
    pub fn replace_point(&self, mut point: i64, replace_with: &Code) -> (Code, i64) {
        // If this is the replacement point, return the replacement
        if 0 == point {
            (replace_with.clone(), 1)
        } else if point < 1 {
            // After we've performed the replacement, everything gets returned as-is
            (self.clone(), 1)
        } else {
            // Lists get special handling, but atoms are returned as-is
            match &self {
                Code::List(list) => {
                    // We need to track both the number of points used and the points remaining until replacement.
                    let mut next_list = vec![];
                    let mut total_used = 1;
                    point -= 1;
                    for item in list {
                        let (next, used) = item.replace_point(point, replace_with);
                        point -= used;
                        total_used += used;
                        next_list.push(next);
                    }
                    (Code::List(next_list), total_used)
                }
                _ => (self.clone(), 1),
            }
        }
    }

    /// Returns the number of items in this list. Unlike 'points' it does not recurse into sub-lists
    pub fn len(&self) -> usize {
        match &self {
            Code::List(list) => list.len(),
            _ => 1,
        }
    }

    /// Replaces the specified search code with the specified replacement code
    pub fn replace(&self, look_for: &Code, replace_with: &Code) -> Code {
        if self == look_for {
            return replace_with.clone();
        }

        match &self {
            Code::List(list) => {
                let mut next_list = vec![];
                for item in list {
                    next_list.push(item.replace(look_for, replace_with));
                }
                Code::List(next_list)
            }
            &x => x.clone(),
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
            Code::InstructionByName(v) => write!(f, "{}", v),
        }
    }
}

// An extraction can either return a piece of code or the number of points used
#[derive(Debug, PartialEq)]
pub enum Extraction {
    Extracted(Code),
    Used(i64),
}

#[cfg(test)]
mod tests {
    use super::Extraction;
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
        assert_eq!("( ( TRUE 0.012345 -12784 KCMAAAAAAAA= ) BOOLAND )", format!("{}", code));
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
    fn extract_point() {
        let code = Code::new("( A ( B ) )");
        assert_eq!(4, code.points());
        assert_eq!(code.extract_point(0), Extraction::Extracted(code.clone()));
        assert_eq!(code.extract_point(1), Extraction::Extracted(Code::new("A")));
        assert_eq!(code.extract_point(2), Extraction::Extracted(Code::new("( B )")));
        assert_eq!(code.extract_point(3), Extraction::Extracted(Code::new("B")));
    }

    #[test]
    fn replace_point() {
        let code = Code::new("( A ( B ) )");
        assert_eq!(code.replace_point(0, &Code::new("C")).0, Code::new("C"));
        assert_eq!(code.replace_point(1, &Code::new("C")).0, Code::new("( C ( B ) )"));
        assert_eq!(code.replace_point(2, &Code::new("C")).0, Code::new("( A C )"));
        assert_eq!(code.replace_point(3, &Code::new("C")).0, Code::new("( A ( C ) )"));
        assert_eq!(code.replace_point(4, &Code::new("C")).0, Code::new("( A ( B ) )"));
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

    #[test]
    fn code_len() {
        // `len` returns the number of elements in the direct list (not sub-lists)
        assert_eq!(0, Code::new("( )").len());
        assert_eq!(1, Code::new("( A )").len());
        assert_eq!(2, Code::new("( A B )").len());
        assert_eq!(2, Code::new("( A ( B C ) )").len());

        // It also returns 1 for atoms
        assert_eq!(1, Code::new("A").len());
    }

    #[test]
    fn replace() {
        assert_eq!(Code::new("B"), Code::new("A").replace(&Code::new("A"), &Code::new("B")));
        assert_eq!(Code::new("( B )"), Code::new("( A )").replace(&Code::new("A"), &Code::new("B")));
        assert_eq!(Code::new("( B B )"), Code::new("( A A )").replace(&Code::new("A"), &Code::new("B")));
        assert_eq!(Code::new("B"), Code::new("( A )").replace(&Code::new("( A )"), &Code::new("B")));
        assert_eq!(Code::new("( B )"), Code::new("( ( A ) )").replace(&Code::new("( A )"), &Code::new("B")));
        assert_eq!(
            Code::new("( A A ( A A ) )"),
            Code::new("( A ( B ) ( A ( B ) ) ) )").replace(&Code::new("( B )"), &Code::new("A"))
        );
    }
}
