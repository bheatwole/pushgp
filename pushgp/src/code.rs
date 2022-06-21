use crate::{
    Instruction, VirtualMachine, VirtualMachineMustHaveBool, VirtualMachineMustHaveCode, VirtualMachineMustHaveExec,
    VirtualMachineMustHaveFloat, VirtualMachineMustHaveInteger, VirtualMachineMustHaveName,
};

// impl Code {
//     /// Parses the input string into Code or returns an error indicating where the parse failed. Instruction names are
//     /// translated using the specified virtual table
//     pub fn parse<State: std::fmt::Debug + Clone>(
//         virtual_table: &VirtualTable<State>,
//         input: &str,
//     ) -> Result<Code, ParseError> {
//         crate::parse::parse(virtual_table, input)
//     }

//     /// Parses the input string, but panics if there is a parsing error
//     pub fn must_parse<State: std::fmt::Debug + Clone>(virtual_table: &VirtualTable<State>, input: &str) -> Code {
//         crate::parse::parse(virtual_table, input).unwrap()
//     }

//     /// Returns true if this code is a List
//     pub fn is_list(&self) -> bool {
//         match &self {
//             Code::List(_) => true,
//             _ => false,
//         }
//     }

//     /// Returns true if this code is anything BUT a list
//     pub fn is_atom(&self) -> bool {
//         !self.is_list()
//     }

//     /// Returns true if the specified code is equal to this item or any child
//     pub fn contains(&self, look_for: &Code) -> bool {
//         if self == look_for {
//             return true;
//         }
//         match &self {
//             Code::List(list) => {
//                 for i in list {
//                     if i.contains(look_for) {
//                         return true;
//                     }
//                 }
//                 false
//             }
//             x => *x == look_for,
//         }
//     }

//     /// Returns the smallest sub-list that contains the specified code
//     pub fn container(&self, look_for: &Code) -> Option<Code> {
//         match &self {
//             Code::List(list) => {
//                 for i in list {
//                     if i == look_for {
//                         return Some(Code::List(list.clone()));
//                     }
//                 }
//                 for i in list {
//                     if let Some(code) = i.container(&look_for) {
//                         return Some(code);
//                     }
//                 }
//                 None
//             }
//             _ => None,
//         }
//     }

//     /// Similar to `contains` but does not recurse into Lists
//     pub fn has_member(&self, look_for: &Code) -> bool {
//         match &self {
//             Code::List(list) => {
//                 for item in list {
//                     if item == look_for {
//                         return true;
//                     }
//                 }
//                 false
//             }
//             &x => x == look_for,
//         }
//     }

//     /// Similar to `extract_point` but does not recurse into lists
//     pub fn position_of(&self, look_for: &Code) -> Option<usize> {
//         match &self {
//             Code::List(list) => {
//                 for (i, item) in list.iter().enumerate() {
//                     if item == look_for {
//                         return Some(i);
//                     }
//                 }
//                 None
//             }
//             &x => {
//                 if x == look_for {
//                     Some(0)
//                 } else {
//                     None
//                 }
//             }
//         }
//     }

//     /// The discrepancy output is a HashMap of every unique sub-list and atom from the specified code
//     pub fn discrepancy_items(&self) -> FnvHashMap<Code, i64> {
//         let mut items = FnvHashMap::default();

//         match &self {
//             Code::List(list) => {
//                 for i in list {
//                     if i.is_list() {
//                         let counter = items.entry(i.clone()).or_insert(0);
//                         *counter += 1;
//                     }
//                     for (key, count) in i.discrepancy_items() {
//                         let counter = items.entry(key).or_insert(0);
//                         *counter += count;
//                     }
//                 }
//             }
//             &atom => {
//                 let counter = items.entry(atom.clone()).or_insert(0);
//                 *counter += 1;
//             }
//         }

//         items
//     }

//     /// Coerces the item to a list, taking ownership. This is similar to a call to 'unwrap'
//     pub fn to_list(self) -> Vec<Code> {
//         match self {
//             Code::List(x) => x,
//             Code::InstructionWithData(inst, data) => vec![Code::InstructionWithData(inst, data)],
//         }
//     }

//     /// Returns the number of 'points' of the entire code. Each atom and list is considered one point.
//     pub fn points(&self) -> i64 {
//         match &self {
//             Code::List(x) => {
//                 let sub_points: i64 = x.iter().map(|c| c.points()).sum();
//                 1 + sub_points
//             }
//             _ => 1,
//         }
//     }

//     /// Returns the item of code at the specified 'point' in the code tree if `point` is less than the number of points
//     /// in the code. Returns the number of points used otherwise.
//     pub fn extract_point(&self, point: i64) -> Extraction {
//         if 0 == point {
//             return Extraction::Extracted(self.clone());
//         }
//         match &self {
//             Code::List(list) => {
//                 let mut used = 1;
//                 for item in list {
//                     match item.extract_point(point - used) {
//                         Extraction::Extracted(code) => return Extraction::Extracted(code),
//                         Extraction::Used(u) => used += u,
//                     }
//                 }
//                 Extraction::Used(used)
//             }
//             _ => Extraction::Used(1),
//         }
//     }

//     /// Descends to the specified point in the code tree and swaps the list or atom there with the specified replacement
//     /// code. If the replacement point is greater than the number of points in the Code, this has no effect.
//     pub fn replace_point(&self, mut point: i64, replace_with: &Code) -> (Code, i64) {
//         // If this is the replacement point, return the replacement
//         if 0 == point {
//             (replace_with.clone(), 1)
//         } else if point < 1 {
//             // After we've performed the replacement, everything gets returned as-is
//             (self.clone(), 1)
//         } else {
//             // Lists get special handling, but atoms are returned as-is
//             match &self {
//                 Code::List(list) => {
//                     // We need to track both the number of points used and the points remaining until replacement.
//                     let mut next_list = vec![];
//                     let mut total_used = 1;
//                     point -= 1;
//                     for item in list {
//                         let (next, used) = item.replace_point(point, replace_with);
//                         point -= used;
//                         total_used += used;
//                         next_list.push(next);
//                     }
//                     (Code::List(next_list), total_used)
//                 }
//                 _ => (self.clone(), 1),
//             }
//         }
//     }

//     /// Returns the number of items in this list. Unlike 'points' it does not recurse into sub-lists
//     pub fn len(&self) -> usize {
//         match &self {
//             Code::List(list) => list.len(),
//             _ => 1,
//         }
//     }

//     /// Replaces the specified search code with the specified replacement code
//     pub fn replace(&self, look_for: &Code, replace_with: &Code) -> Code {
//         if self == look_for {
//             return replace_with.clone();
//         }

//         match &self {
//             Code::List(list) => {
//                 let mut next_list = vec![];
//                 for item in list {
//                     next_list.push(item.replace(look_for, replace_with));
//                 }
//                 Code::List(next_list)
//             }
//             &x => x.clone(),
//         }
//     }

//     pub fn displayable<'a, State: std::fmt::Debug + Clone>(
//         &'a self,
//         virtual_table: &'a VirtualTable<State>,
//     ) -> DisplayableCode<'a, State> {
//         DisplayableCode { code: self, virtual_table: virtual_table }
//     }

//     pub fn nom_fmt<State: std::fmt::Debug + Clone>(
//         &self,
//         virtual_table: &VirtualTable<State>,
//         f: &mut std::fmt::Formatter<'_>,
//     ) -> std::fmt::Result {
//         match &self {
//             Code::List(x) => {
//                 write!(f, "(")?;
//                 for c in x.iter() {
//                     write!(f, " ")?;
//                     c.nom_fmt(virtual_table, f)?;
//                 }
//                 write!(f, " )")
//             }
//             Code::InstructionWithData(id, d) => virtual_table.call_nom_fmt(*id, d, f),
//         }
//     }
// }

// An extraction can either return a piece of code or the number of points used
pub enum Extraction<Vm> {
    Extracted(Box<dyn Instruction<Vm>>),
    Used(i64),
}

pub fn add_base_instructions<
    Vm: VirtualMachine
        + VirtualMachineMustHaveBool<Vm>
        + VirtualMachineMustHaveCode<Vm>
        + VirtualMachineMustHaveExec<Vm>
        + VirtualMachineMustHaveFloat<Vm>
        + VirtualMachineMustHaveInteger<Vm>
        + VirtualMachineMustHaveName<Vm>,
>(
    vm: &mut Vm,
) {
    vm.add_instruction::<crate::execute_bool::BoolAnd>();
    vm.add_instruction::<crate::execute_bool::BoolDefine>();
    vm.add_instruction::<crate::execute_bool::BoolDup>();
    vm.add_instruction::<crate::execute_bool::BoolEqual>();
    vm.add_instruction::<crate::execute_bool::BoolFlush>();
    vm.add_instruction::<crate::execute_bool::BoolFromFloat>();
    vm.add_instruction::<crate::execute_bool::BoolFromInt>();
    vm.add_instruction::<crate::execute_bool::BoolNot>();
    vm.add_instruction::<crate::execute_bool::BoolOr>();
    vm.add_instruction::<crate::execute_bool::BoolPop>();
    vm.add_instruction::<crate::execute_bool::BoolRand>();
    vm.add_instruction::<crate::execute_bool::BoolRot>();
    vm.add_instruction::<crate::execute_bool::BoolShove>();
    vm.add_instruction::<crate::execute_bool::BoolStackDepth>();
    vm.add_instruction::<crate::execute_bool::BoolSwap>();
    vm.add_instruction::<crate::execute_bool::BoolYankDup>();
    vm.add_instruction::<crate::execute_bool::BoolYank>();
    vm.add_instruction::<crate::execute_code::CodeAppend>();
    vm.add_instruction::<crate::execute_code::CodeAtom>();
    vm.add_instruction::<crate::execute_code::CodeCar>();
    vm.add_instruction::<crate::execute_code::CodeCdr>();
    vm.add_instruction::<crate::execute_code::CodeCons>();
    vm.add_instruction::<crate::execute_code::CodeContainer>();
    vm.add_instruction::<crate::execute_code::CodeContains>();
    vm.add_instruction::<crate::execute_code::CodeDefine>();
    vm.add_instruction::<crate::execute_code::CodeDefinition>();
    vm.add_instruction::<crate::execute_code::CodeDiscrepancy>();
    vm.add_instruction::<crate::execute_code::CodeDoNCount>();
    vm.add_instruction::<crate::execute_code::CodeDoNRange>();
    vm.add_instruction::<crate::execute_code::CodeDoNTimes>();
    vm.add_instruction::<crate::execute_code::CodeDoN>();
    vm.add_instruction::<crate::execute_code::CodeDo>();
    vm.add_instruction::<crate::execute_code::CodeDup>();
    vm.add_instruction::<crate::execute_code::CodeEqual>();
    vm.add_instruction::<crate::execute_code::CodeExtract>();
    vm.add_instruction::<crate::execute_code::CodeFlush>();
    vm.add_instruction::<crate::execute_code::CodeFromBoolean>();
    vm.add_instruction::<crate::execute_code::CodeFromFloat>();
    vm.add_instruction::<crate::execute_code::CodeFromInteger>();
    vm.add_instruction::<crate::execute_code::CodeFromName>();
    vm.add_instruction::<crate::execute_code::CodeIf>();
    vm.add_instruction::<crate::execute_code::CodeInsert>();
    vm.add_instruction::<crate::execute_code::CodeLength>();
    vm.add_instruction::<crate::execute_code::CodeList>();
    vm.add_instruction::<crate::execute_code::CodeMember>();
    vm.add_instruction::<crate::execute_code::CodeNoop>();
    vm.add_instruction::<crate::execute_code::CodeNthCdr>();
    vm.add_instruction::<crate::execute_code::CodeNth>();
    vm.add_instruction::<crate::execute_code::CodeNull>();
    vm.add_instruction::<crate::execute_code::CodePop>();
    vm.add_instruction::<crate::execute_code::CodePosition>();
    vm.add_instruction::<crate::execute_code::CodeQuote>();
    vm.add_instruction::<crate::execute_code::CodeRand>();
    vm.add_instruction::<crate::execute_code::CodeRot>();
    vm.add_instruction::<crate::execute_code::CodeShove>();
    vm.add_instruction::<crate::execute_code::CodeSize>();
    vm.add_instruction::<crate::execute_code::CodeStackDepth>();
    vm.add_instruction::<crate::execute_code::CodeSubstitute>();
    vm.add_instruction::<crate::execute_code::CodeSwap>();
    vm.add_instruction::<crate::execute_code::CodeYankDup>();
    vm.add_instruction::<crate::execute_code::CodeYank>();
    vm.add_instruction::<crate::execute_exec::ExecDefine>();
    vm.add_instruction::<crate::execute_exec::ExecDoNCount>();
    vm.add_instruction::<crate::execute_exec::ExecDoNRange>();
    vm.add_instruction::<crate::execute_exec::ExecDoNTimes>();
    vm.add_instruction::<crate::execute_exec::ExecDup>();
    vm.add_instruction::<crate::execute_exec::ExecEqual>();
    vm.add_instruction::<crate::execute_exec::ExecFlush>();
    vm.add_instruction::<crate::execute_exec::ExecIf>();
    vm.add_instruction::<crate::execute_exec::ExecK>();
    vm.add_instruction::<crate::execute_exec::ExecPop>();
    vm.add_instruction::<crate::execute_exec::ExecRot>();
    vm.add_instruction::<crate::execute_exec::ExecShove>();
    vm.add_instruction::<crate::execute_exec::ExecStackDepth>();
    vm.add_instruction::<crate::execute_exec::ExecSwap>();
    vm.add_instruction::<crate::execute_exec::ExecS>();
    vm.add_instruction::<crate::execute_exec::ExecYankDup>();
    vm.add_instruction::<crate::execute_exec::ExecYank>();
    vm.add_instruction::<crate::execute_exec::ExecY>();
    vm.add_instruction::<crate::execute_float::FloatCos>();
    vm.add_instruction::<crate::execute_float::FloatDefine>();
    vm.add_instruction::<crate::execute_float::FloatDifference>();
    vm.add_instruction::<crate::execute_float::FloatDup>();
    vm.add_instruction::<crate::execute_float::FloatEqual>();
    vm.add_instruction::<crate::execute_float::FloatFlush>();
    vm.add_instruction::<crate::execute_float::FloatFromBoolean>();
    vm.add_instruction::<crate::execute_float::FloatFromInteger>();
    vm.add_instruction::<crate::execute_float::FloatGreater>();
    vm.add_instruction::<crate::execute_float::FloatLess>();
    vm.add_instruction::<crate::execute_float::FloatMax>();
    vm.add_instruction::<crate::execute_float::FloatMin>();
    vm.add_instruction::<crate::execute_float::FloatModulo>();
    vm.add_instruction::<crate::execute_float::FloatPop>();
    vm.add_instruction::<crate::execute_float::FloatProduct>();
    vm.add_instruction::<crate::execute_float::FloatQuotient>();
    vm.add_instruction::<crate::execute_float::FloatRand>();
    vm.add_instruction::<crate::execute_float::FloatRot>();
    vm.add_instruction::<crate::execute_float::FloatShove>();
    vm.add_instruction::<crate::execute_float::FloatSin>();
    vm.add_instruction::<crate::execute_float::FloatStackDepth>();
    vm.add_instruction::<crate::execute_float::FloatSum>();
    vm.add_instruction::<crate::execute_float::FloatSwap>();
    vm.add_instruction::<crate::execute_float::FloatTan>();
    vm.add_instruction::<crate::execute_float::FloatYankDup>();
    vm.add_instruction::<crate::execute_float::FloatYank>();
    vm.add_instruction::<crate::execute_integer::IntegerDefine>();
    vm.add_instruction::<crate::execute_integer::IntegerDifference>();
    vm.add_instruction::<crate::execute_integer::IntegerDup>();
    vm.add_instruction::<crate::execute_integer::IntegerEqual>();
    vm.add_instruction::<crate::execute_integer::IntegerFlush>();
    vm.add_instruction::<crate::execute_integer::IntegerFromBoolean>();
    vm.add_instruction::<crate::execute_integer::IntegerFromFloat>();
    vm.add_instruction::<crate::execute_integer::IntegerGreater>();
    vm.add_instruction::<crate::execute_integer::IntegerLess>();
    vm.add_instruction::<crate::execute_integer::IntegerMax>();
    vm.add_instruction::<crate::execute_integer::IntegerMin>();
    vm.add_instruction::<crate::execute_integer::IntegerModulo>();
    vm.add_instruction::<crate::execute_integer::IntegerPop>();
    vm.add_instruction::<crate::execute_integer::IntegerProduct>();
    vm.add_instruction::<crate::execute_integer::IntegerQuotient>();
    vm.add_instruction::<crate::execute_integer::IntegerRand>();
    vm.add_instruction::<crate::execute_integer::IntegerRot>();
    vm.add_instruction::<crate::execute_integer::IntegerShove>();
    vm.add_instruction::<crate::execute_integer::IntegerStackDepth>();
    vm.add_instruction::<crate::execute_integer::IntegerSum>();
    vm.add_instruction::<crate::execute_integer::IntegerSwap>();
    vm.add_instruction::<crate::execute_integer::IntegerYankDup>();
    vm.add_instruction::<crate::execute_integer::IntegerYank>();
    vm.add_instruction::<crate::execute_name::NameDup>();
    vm.add_instruction::<crate::execute_name::NameEqual>();
    vm.add_instruction::<crate::execute_name::NameFlush>();
    vm.add_instruction::<crate::execute_name::NamePop>();
    vm.add_instruction::<crate::execute_name::NameQuote>();
    vm.add_instruction::<crate::execute_name::NameRandBoundName>();
    vm.add_instruction::<crate::execute_name::NameRand>();
    vm.add_instruction::<crate::execute_name::NameRot>();
    vm.add_instruction::<crate::execute_name::NameShove>();
    vm.add_instruction::<crate::execute_name::NameStackDepth>();
    vm.add_instruction::<crate::execute_name::NameSwap>();
    vm.add_instruction::<crate::execute_name::NameYankDup>();
    vm.add_instruction::<crate::execute_name::NameYank>();
}

pub fn add_base_literals<
    Vm: VirtualMachine
        + VirtualMachineMustHaveBool<Vm>
        + VirtualMachineMustHaveExec<Vm>
        + VirtualMachineMustHaveFloat<Vm>
        + VirtualMachineMustHaveInteger<Vm>
        + VirtualMachineMustHaveName<Vm>
        + 'static,
>(
    vm: &mut Vm,
) {
    // These must be last, with Name the very last of all. The reason is that parsing runs in order from top to bottom
    // and all the 'normal' instructions use an exact match. However the literal values use more involved parsing and
    // Name is the catch-all (anything that does not parse earlier will become a Name up to the next white-space).
    vm.add_instruction::<crate::list::PushList<Vm>>();
    vm.add_instruction::<crate::execute_bool::BoolLiteralValue>();
    vm.add_instruction::<crate::execute_float::FloatLiteralValue>();
    vm.add_instruction::<crate::execute_integer::IntegerLiteralValue>();
    vm.add_instruction::<crate::execute_name::NameLiteralValue>();
}

#[cfg(test)]
mod tests {
    use super::Extraction;
    use crate::*;
    use rust_decimal::{prelude::ToPrimitive, Decimal};

    // fn make_literal_bool<State: std::fmt::Debug + Clone>(virtual_table: &VirtualTable<State>, value: bool) -> Code {
    //     let id = virtual_table.id_for_name(BoolLiteralValue::name()).unwrap();
    //     Code::InstructionWithData(id, Some(InstructionData::from_bool(value)))
    // }

    // fn make_literal_float<State: std::fmt::Debug + Clone>(virtual_table: &VirtualTable<State>, value: Float) -> Code {
    //     let id = virtual_table.id_for_name(FloatLiteralValue::name()).unwrap();
    //     Code::InstructionWithData(id, Some(InstructionData::from_f64(value.to_f64().unwrap())))
    // }

    // fn make_literal_integer<State: std::fmt::Debug + Clone>(virtual_table: &VirtualTable<State>, value: i64) -> Code {
    //     let id = virtual_table.id_for_name(IntegerLiteralValue::name()).unwrap();
    //     Code::InstructionWithData(id, Some(InstructionData::from_i64(value)))
    // }

    // fn make_literal_name<S: Into<String>, State: std::fmt::Debug + Clone>(
    //     virtual_table: &VirtualTable<State>,
    //     value: S,
    // ) -> Code {
    //     let id = virtual_table.id_for_name(NameLiteralValue::name()).unwrap();
    //     Code::InstructionWithData(id, Some(InstructionData::from_string(value)))
    // }

    // fn make_instruction<State: std::fmt::Debug + Clone>(
    //     virtual_table: &VirtualTable<State>,
    //     instruction: &'static str,
    // ) -> Code {
    //     let id = virtual_table.id_for_name(instruction).unwrap();
    //     Code::InstructionWithData(id, None)
    // }

    // #[test]
    // fn not_parsable() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     let result = Code::parse(&virtual_table, "( DOESNT WORK");
    //     assert!(result.is_err());
    // }

    // #[test]
    // fn code_display() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     let code = Code::List(vec![]);
    //     assert_eq!("( )", format!("{}", code.displayable(&virtual_table)));

    //     let code = Code::List(vec![
    //         Code::List(vec![
    //             make_literal_bool(&virtual_table, true),
    //             make_literal_float(&virtual_table, Decimal::new(12345, 6)),
    //             make_literal_integer(&virtual_table, -12784),
    //             make_literal_name(&virtual_table, "a_name"),
    //         ]),
    //         make_instruction(&virtual_table, "BOOL.AND"),
    //     ]);
    //     assert_eq!("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )", format!("{}", code.displayable(&virtual_table)));
    // }

    // #[test]
    // fn code_points() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     let code = Code::List(vec![
    //         Code::List(vec![
    //             make_literal_bool(&virtual_table, true),
    //             make_literal_float(&virtual_table, Decimal::new(12345, 6)),
    //             make_literal_integer(&virtual_table, -12784),
    //             make_literal_name(&virtual_table, "a_name"),
    //         ]),
    //         make_instruction(&virtual_table, "BOOL.AND"),
    //     ]);
    //     assert_eq!(7, code.points());
    // }

    // #[test]
    // fn extract_point() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     let code = Code::must_parse(&virtual_table, "( A ( B ) )");
    //     assert_eq!(4, code.points());
    //     assert_eq!(code.extract_point(0), Extraction::Extracted(code.clone()));
    //     assert_eq!(code.extract_point(1), Extraction::Extracted(Code::must_parse(&virtual_table, "A")));
    //     assert_eq!(code.extract_point(2), Extraction::Extracted(Code::must_parse(&virtual_table, "( B )")));
    //     assert_eq!(code.extract_point(3), Extraction::Extracted(Code::must_parse(&virtual_table, "B")));
    // }

    // #[test]
    // fn replace_point() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     let code = Code::must_parse(&virtual_table, "( A ( B ) )");
    //     assert_eq!(
    //         code.replace_point(0, &Code::must_parse(&virtual_table, "C")).0,
    //         Code::must_parse(&virtual_table, "C")
    //     );
    //     assert_eq!(
    //         code.replace_point(1, &Code::must_parse(&virtual_table, "C")).0,
    //         Code::must_parse(&virtual_table, "( C ( B ) )")
    //     );
    //     assert_eq!(
    //         code.replace_point(2, &Code::must_parse(&virtual_table, "C")).0,
    //         Code::must_parse(&virtual_table, "( A C )")
    //     );
    //     assert_eq!(
    //         code.replace_point(3, &Code::must_parse(&virtual_table, "C")).0,
    //         Code::must_parse(&virtual_table, "( A ( C ) )")
    //     );
    //     assert_eq!(
    //         code.replace_point(4, &Code::must_parse(&virtual_table, "C")).0,
    //         Code::must_parse(&virtual_table, "( A ( B ) )")
    //     );
    // }

    // #[test]
    // fn code_discrepancy_items() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
    //     let code = Code::must_parse(&virtual_table, "( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )");
    //     let items = code.discrepancy_items();
    //     assert_eq!(1, *items.get(&Code::must_parse(&virtual_table, "ANAME")).unwrap());
    //     assert_eq!(1, *items.get(&Code::must_parse(&virtual_table, "( 3 ( 1 ) )")).unwrap());
    //     assert_eq!(1, *items.get(&Code::must_parse(&virtual_table, "3")).unwrap());
    //     assert_eq!(2, *items.get(&Code::must_parse(&virtual_table, "( 1 )")).unwrap());
    //     assert_eq!(3, *items.get(&Code::must_parse(&virtual_table, "1")).unwrap());
    //     assert_eq!(5, items.len());
    // }

    // #[test]
    // fn code_len() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     // `len` returns the number of elements in the direct list (not sub-lists)
    //     assert_eq!(0, Code::must_parse(&virtual_table, "( )").len());
    //     assert_eq!(1, Code::must_parse(&virtual_table, "( A )").len());
    //     assert_eq!(2, Code::must_parse(&virtual_table, "( A B )").len());
    //     assert_eq!(2, Code::must_parse(&virtual_table, "( A ( B C ) )").len());

    //     // It also returns 1 for atoms
    //     assert_eq!(1, Code::must_parse(&virtual_table, "A").len());
    // }

    // #[test]
    // fn replace() {
    //     let virtual_table = VirtualTable::<()>::new_with_all_instructions();
    //     assert_eq!(
    //         Code::must_parse(&virtual_table, "B"),
    //         Code::must_parse(&virtual_table, "A")
    //             .replace(&Code::must_parse(&virtual_table, "A"), &Code::must_parse(&virtual_table, "B"))
    //     );
    //     assert_eq!(
    //         Code::must_parse(&virtual_table, "( B )"),
    //         Code::must_parse(&virtual_table, "( A )")
    //             .replace(&Code::must_parse(&virtual_table, "A"), &Code::must_parse(&virtual_table, "B"))
    //     );
    //     assert_eq!(
    //         Code::must_parse(&virtual_table, "( B B )"),
    //         Code::must_parse(&virtual_table, "( A A )")
    //             .replace(&Code::must_parse(&virtual_table, "A"), &Code::must_parse(&virtual_table, "B"))
    //     );
    //     assert_eq!(
    //         Code::must_parse(&virtual_table, "B"),
    //         Code::must_parse(&virtual_table, "( A )")
    //             .replace(&Code::must_parse(&virtual_table, "( A )"), &Code::must_parse(&virtual_table, "B"))
    //     );
    //     assert_eq!(
    //         Code::must_parse(&virtual_table, "( B )"),
    //         Code::must_parse(&virtual_table, "( ( A ) )")
    //             .replace(&Code::must_parse(&virtual_table, "( A )"), &Code::must_parse(&virtual_table, "B"))
    //     );
    //     assert_eq!(
    //         Code::must_parse(&virtual_table, "( A A ( A A ) )"),
    //         Code::must_parse(&virtual_table, "( A ( B ) ( A ( B ) ) ) )")
    //             .replace(&Code::must_parse(&virtual_table, "( B )"), &Code::must_parse(&virtual_table, "A"))
    //     );
    // }
}
