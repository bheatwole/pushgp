use crate::{InstructionData, ParseError, VirtualTable};
use fnv::FnvHashMap;
use std::hash::Hash;

// Code is the basic building block of a PushGP program. It's the translation between human readable and machine
// readable strings.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Code {
    // A list is just a list containing other code (which can be lists) and may also be empty (len() == 0)
    List(Vec<Code>),

    // Code can be an instruction which may have data parsed from the human-readable string. The 'usize' is an entry in
    // the VirtualTable associated with the Context
    InstructionWithData(usize, Option<InstructionData>),
}

impl Code {
    /// Parses the input string into Code or returns an error indicating where the parse failed. Instruction names are
    /// translated using the specified virtual table
    pub fn parse(virtual_table: &VirtualTable, input: &str) -> Result<Code, ParseError> {
        crate::parse::parse(virtual_table, input)
    }

    /// Parses the input string, but panics if there is a parsing error
    pub fn must_parse(virtual_table: &VirtualTable, input: &str) -> Code {
        crate::parse::parse(virtual_table, input).unwrap()
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

    /// The discrepancy output is a HashMap of every unique sub-list and atom from the specified code
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
            Code::InstructionWithData(inst, data) => vec![Code::InstructionWithData(inst, data)],
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

    pub fn displayable<'a>(&'a self, virtual_table: &'a VirtualTable) -> DisplayableCode<'a> {
        DisplayableCode { code: self, virtual_table: virtual_table }
    }

    pub fn nom_fmt(&self, virtual_table: &VirtualTable, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Code::List(x) => {
                write!(f, "(")?;
                for c in x.iter() {
                    write!(f, " ")?;
                    c.nom_fmt(virtual_table, f)?;
                }
                write!(f, " )")
            }
            Code::InstructionWithData(id, d) => virtual_table.call_nom_fmt(*id, d, f),
        }
    }
}

pub struct DisplayableCode<'a> {
    code: &'a Code,
    virtual_table: &'a VirtualTable,
}

impl<'a> std::fmt::Display for DisplayableCode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code.nom_fmt(self.virtual_table, f)
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
    use crate::*;
    use rust_decimal::{prelude::ToPrimitive, Decimal};

    fn make_literal_bool(virtual_table: &VirtualTable, value: bool) -> Code {
        let id = virtual_table.id_for_name(BoolLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_bool(value)))
    }

    fn make_literal_float(virtual_table: &VirtualTable, value: Float) -> Code {
        let id = virtual_table.id_for_name(FloatLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_f64(value.to_f64().unwrap())))
    }

    fn make_literal_integer(virtual_table: &VirtualTable, value: i64) -> Code {
        let id = virtual_table.id_for_name(IntegerLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_i64(value)))
    }

    fn make_literal_name<S: Into<String>>(virtual_table: &VirtualTable, value: S) -> Code {
        let id = virtual_table.id_for_name(NameLiteralValue::name()).unwrap();
        Code::InstructionWithData(id, Some(InstructionData::from_string(value)))
    }

    fn make_instruction(virtual_table: &VirtualTable, instruction: &'static str) -> Code {
        let id = virtual_table.id_for_name(instruction).unwrap();
        Code::InstructionWithData(id, None)
    }

    #[test]
    fn not_parsable() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let result = Code::parse(&virtual_table, "( DOESNT WORK");
        assert!(result.is_err());
    }

    #[test]
    fn code_display() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let code = Code::List(vec![]);
        assert_eq!("( )", format!("{}", code.displayable(&virtual_table)));

        let code = Code::List(vec![
            Code::List(vec![
                make_literal_bool(&virtual_table, true),
                make_literal_float(&virtual_table, Decimal::new(12345, 6)),
                make_literal_integer(&virtual_table, -12784),
                make_literal_name(&virtual_table, "a_name"),
            ]),
            make_instruction(&virtual_table, "BOOL.AND"),
        ]);
        assert_eq!("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )", format!("{}", code.displayable(&virtual_table)));
    }

    #[test]
    fn code_points() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let code = Code::List(vec![
            Code::List(vec![
                make_literal_bool(&virtual_table, true),
                make_literal_float(&virtual_table, Decimal::new(12345, 6)),
                make_literal_integer(&virtual_table, -12784),
                make_literal_name(&virtual_table, "a_name"),
            ]),
            make_instruction(&virtual_table, "BOOL.AND"),
        ]);
        assert_eq!(7, code.points());
    }

    #[test]
    fn extract_point() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let code = Code::must_parse(&virtual_table, "( A ( B ) )");
        assert_eq!(4, code.points());
        assert_eq!(code.extract_point(0), Extraction::Extracted(code.clone()));
        assert_eq!(code.extract_point(1), Extraction::Extracted(Code::must_parse(&virtual_table, "A")));
        assert_eq!(code.extract_point(2), Extraction::Extracted(Code::must_parse(&virtual_table, "( B )")));
        assert_eq!(code.extract_point(3), Extraction::Extracted(Code::must_parse(&virtual_table, "B")));
    }

    #[test]
    fn replace_point() {
        let virtual_table = new_virtual_table_with_all_instructions();
        let code = Code::must_parse(&virtual_table, "( A ( B ) )");
        assert_eq!(
            code.replace_point(0, &Code::must_parse(&virtual_table, "C")).0,
            Code::must_parse(&virtual_table, "C")
        );
        assert_eq!(
            code.replace_point(1, &Code::must_parse(&virtual_table, "C")).0,
            Code::must_parse(&virtual_table, "( C ( B ) )")
        );
        assert_eq!(
            code.replace_point(2, &Code::must_parse(&virtual_table, "C")).0,
            Code::must_parse(&virtual_table, "( A C )")
        );
        assert_eq!(
            code.replace_point(3, &Code::must_parse(&virtual_table, "C")).0,
            Code::must_parse(&virtual_table, "( A ( C ) )")
        );
        assert_eq!(
            code.replace_point(4, &Code::must_parse(&virtual_table, "C")).0,
            Code::must_parse(&virtual_table, "( A ( B ) )")
        );
    }

    #[test]
    fn code_discrepancy_items() {
        let virtual_table = new_virtual_table_with_all_instructions();
        // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
        let code = Code::must_parse(&virtual_table, "( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )");
        let items = code.discrepancy_items();
        assert_eq!(1, *items.get(&Code::must_parse(&virtual_table, "ANAME")).unwrap());
        assert_eq!(1, *items.get(&Code::must_parse(&virtual_table, "( 3 ( 1 ) )")).unwrap());
        assert_eq!(1, *items.get(&Code::must_parse(&virtual_table, "3")).unwrap());
        assert_eq!(2, *items.get(&Code::must_parse(&virtual_table, "( 1 )")).unwrap());
        assert_eq!(3, *items.get(&Code::must_parse(&virtual_table, "1")).unwrap());
        assert_eq!(5, items.len());
    }

    #[test]
    fn code_len() {
        let virtual_table = new_virtual_table_with_all_instructions();
        // `len` returns the number of elements in the direct list (not sub-lists)
        assert_eq!(0, Code::must_parse(&virtual_table, "( )").len());
        assert_eq!(1, Code::must_parse(&virtual_table, "( A )").len());
        assert_eq!(2, Code::must_parse(&virtual_table, "( A B )").len());
        assert_eq!(2, Code::must_parse(&virtual_table, "( A ( B C ) )").len());

        // It also returns 1 for atoms
        assert_eq!(1, Code::must_parse(&virtual_table, "A").len());
    }

    #[test]
    fn replace() {
        let virtual_table = new_virtual_table_with_all_instructions();
        assert_eq!(
            Code::must_parse(&virtual_table, "B"),
            Code::must_parse(&virtual_table, "A")
                .replace(&Code::must_parse(&virtual_table, "A"), &Code::must_parse(&virtual_table, "B"))
        );
        assert_eq!(
            Code::must_parse(&virtual_table, "( B )"),
            Code::must_parse(&virtual_table, "( A )")
                .replace(&Code::must_parse(&virtual_table, "A"), &Code::must_parse(&virtual_table, "B"))
        );
        assert_eq!(
            Code::must_parse(&virtual_table, "( B B )"),
            Code::must_parse(&virtual_table, "( A A )")
                .replace(&Code::must_parse(&virtual_table, "A"), &Code::must_parse(&virtual_table, "B"))
        );
        assert_eq!(
            Code::must_parse(&virtual_table, "B"),
            Code::must_parse(&virtual_table, "( A )")
                .replace(&Code::must_parse(&virtual_table, "( A )"), &Code::must_parse(&virtual_table, "B"))
        );
        assert_eq!(
            Code::must_parse(&virtual_table, "( B )"),
            Code::must_parse(&virtual_table, "( ( A ) )")
                .replace(&Code::must_parse(&virtual_table, "( A )"), &Code::must_parse(&virtual_table, "B"))
        );
        assert_eq!(
            Code::must_parse(&virtual_table, "( A A ( A A ) )"),
            Code::must_parse(&virtual_table, "( A ( B ) ( A ( B ) ) ) )")
                .replace(&Code::must_parse(&virtual_table, "( B )"), &Code::must_parse(&virtual_table, "A"))
        );
    }
}
