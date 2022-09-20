use fnv::FnvHashMap;

use crate::{
    Data, Name, VirtualMachine, VirtualMachineMustHaveBool, VirtualMachineMustHaveCode, VirtualMachineMustHaveExec,
    VirtualMachineMustHaveFloat, VirtualMachineMustHaveInteger, VirtualMachineMustHaveName,
};

pub type Opcode = u32;

// Profiling shows that a huge amount of time is spent in a malloc/free cycle created because Code<Vm> is a boxed (heap-
// allocated) object. The trait object allowed us two things that need to be replaced to improve this significant
// performance penalty:
// - The lookup to find the correct instruction `execute` method to run
// - A place to store data for instructions that need it.
//
// We will replace this with a stack-based object consisting of an opcode and some optional data that has several
// choices of both stack and heap based storage.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Code {
    // Opcode zero is reserved for PushList
    opcode: Opcode,
    data: Data,
}

impl Code {
    pub fn new(opcode: Opcode, data: Data) -> Code {
        Code { opcode, data }
    }

    /// Convenience method for constructing a new list
    pub fn new_list(items: Vec<Code>) -> Code {
        Code::new(0, items.into())
    }

    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn get_data(&self) -> &Data {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Data {
        &mut self.data
    }

    /// Wraps the code and a virtual machine together so that the code can be printed.
    /// `println!("{}", code.for_display(my_vm))`
    pub fn for_display<'a, Vm: VirtualMachine>(&'a self, vm: &'a Vm) -> CodeWithVirtualMachine<'a, Vm> {
        CodeWithVirtualMachine { code: &self, vm }
    }

    /// Returns true if this code is a List
    pub fn is_list(&self) -> bool {
        self.opcode == 0
    }

    /// Returns true if this code is anything BUT a list
    pub fn is_atom(&self) -> bool {
        self.opcode != 0
    }

    /// Returns true if the specified code is equal to this item or any child
    pub fn contains(&self, look_for: &Code) -> bool {
        self == look_for || (self.is_list() && self.data.code_iter().unwrap().any(|i| i.contains(look_for)))
    }

    /// Returns the smallest sub-list that contains the specified code
    pub fn container(&self, look_for: &Code) -> Option<Code> {
        // If this is an atom it cannot be a container
        if self.is_list() {
            for i in self.data.code_iter().unwrap() {
                if i == look_for {
                    return Some(self.clone());
                }
            }
            for i in self.data.code_iter().unwrap() {
                if let Some(code) = i.container(look_for) {
                    return Some(code);
                }
            }
        }
        None
    }

    /// Similar to `contains` but does not recurse into Lists
    pub fn has_member(&self, look_for: &Code) -> bool {
        self == look_for || (self.is_list() && self.data.code_iter().unwrap().any(|i| i == look_for))
    }

    /// The discrepancy output is a HashMap of every unique sub-list and atom from the specified code
    pub fn discrepancy_items(&self) -> FnvHashMap<Code, i64> {
        let mut items = FnvHashMap::default();
        self.append_discrepancy_items(&mut items);

        items
    }

    /// Appends this item to an already-existing discrepancy items HashMap
    fn append_discrepancy_items(&self, items: &mut FnvHashMap<Code, i64>) {
        // Append 'self' whether it is an atom or a list
        let counter = items.entry(self.clone()).or_insert(0);
        *counter += 1;

        // If this is a list, also append all the items in the list
        if self.is_list() {
            for i in self.data.code_iter().unwrap() {
                i.append_discrepancy_items(items);
            }
        }
    }

    /// Coerces the item to a list
    pub fn to_list(&self) -> Vec<Code> {
        if self.is_list() {
            if let Data::CodeList(list) = self.get_data() {
                list.clone()
            } else {
                vec![self.clone()]
            }
        } else {
            vec![self.clone()]
        }
    }

    /// Returns the number of 'points' of the entire code. Each atom and list is considered one point.
    pub fn points(&self) -> i64 {
        if self.is_list() {
            let sub_points: i64 = self.data.code_iter().unwrap().map(|c| c.points()).sum();
            1 + sub_points
        } else {
            1
        }
    }

    /// Returns the item of code at the specified 'point' in the code tree if `point` is less than the number of points
    /// in the code. Returns the number of points used otherwise.
    pub fn extract_point(&self, point: i64) -> Extraction {
        if 0 == point {
            return Extraction::Extracted(self.clone());
        }
        let mut used = 1;
        if self.is_list() {
            for item in self.data.code_iter().unwrap() {
                match item.extract_point(point - used) {
                    Extraction::Extracted(code) => return Extraction::Extracted(code),
                    Extraction::Used(u) => used += u,
                }
            }
        }
        Extraction::Used(used)
    }

    /// Descends to the specified point in the code tree and swaps the list or atom there with the specified replacement
    /// code. If the replacement point is greater than the number of points in the Code, this has no effect.
    pub fn replace_point(&self, mut point: i64, replace_with: &Code) -> (Code, i64) {
        // If this is the replacement point, return the replacement
        if 0 == point {
            (replace_with.clone(), 1)
        } else if self.is_atom() || point < 1 {
            // If this is an atom or we've performed the replacement, everything gets returned as-is
            (self.clone(), 1)
        } else {
            // We need to track both the number of points used and the points remaining until replacement.
            let mut next_list = vec![];
            let mut total_used = 1;
            point -= 1;
            for item in self.data.code_iter().unwrap() {
                let (next, used) = item.replace_point(point, replace_with);
                point -= used;
                total_used += used;
                next_list.push(next);
            }
            (Code::new(0, Data::CodeList(next_list)), total_used)
        }
    }

    /// Similar to `extract_point` but does not recurse into lists
    pub fn position_of(&self, look_for: &Code) -> Option<usize> {
        if self.is_atom() {
            if self == look_for {
                Some(0)
            } else {
                None
            }
        } else {
            for (i, item) in self.data.code_iter().unwrap().enumerate() {
                if item == look_for {
                    return Some(i);
                }
            }
            None
        }
    }

    /// Returns a list of all names found in the instruction.
    pub fn extract_names(&self) -> Vec<Name> {
        let mut names = vec![];
        self.append_names(&mut names);
        names
    }

    fn append_names(&self, names: &mut Vec<Name>) {
        match self.get_data() {
            Data::Name(name) => names.push(name.clone()),
            Data::CodeList(list) => {
                for item in list.iter() {
                    item.append_names(names);
                }
            }
            _ => {}
        }
    }

    /// Returns a list of clones of all the atoms found in the instruction.
    pub fn extract_atoms(&self) -> Vec<Code> {
        let mut atoms = vec![];
        self.append_atoms(&mut atoms);

        atoms
    }

    fn append_atoms(&self, atoms: &mut Vec<Code>) {
        if self.is_atom() {
            atoms.push(self.clone());
        } else {
            for item in self.data.code_iter().unwrap() {
                item.append_atoms(atoms);
            }
        }
    }

    /// Returns the number of items in this list. Unlike 'points' it does not recurse into sub-lists
    pub fn len(&self) -> usize {
        match self.get_data() {
            Data::CodeList(list) => list.len(),
            _ => 1,
        }
    }

    /// Replaces the specified search code with the specified replacement code
    pub fn replace(&self, look_for: &Code, replace_with: &Code) -> Code {
        if self == look_for {
            replace_with.clone()
        } else if self.is_atom() {
            self.clone()
        } else {
            let mut next_list = vec![];
            for item in self.data.code_iter().unwrap() {
                next_list.push(item.replace(look_for, replace_with));
            }
            Code::new(0, Data::CodeList(next_list))
        }
    }
}

pub struct CodeWithVirtualMachine<'a, Vm: VirtualMachine> {
    code: &'a Code,
    vm: &'a Vm,
}

impl<'a, Vm: VirtualMachine> std::fmt::Display for CodeWithVirtualMachine<'a, Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vm.fmt(f, self.code)
    }
}

// An extraction can either return a piece of code or the number of points used
#[derive(Debug, PartialEq)]
pub enum Extraction {
    Extracted(Code),
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
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolAnd>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolDefine>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolDup>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolEqual>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolFlush>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolFromFloat>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolFromInt>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolNot>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolOr>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolPop>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolRand>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolRot>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolShove>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolStackDepth>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolSwap>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolYankDup>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolYank>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeAppend>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeAtom>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeCar>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeCdr>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeCons>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeContainer>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeContains>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDefine>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDefinition>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDiscrepancy>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDoNCount>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDoNRange>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDoNTimes>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDoN>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDo>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeDup>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeEqual>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeExtract>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeFlush>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeFromBoolean>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeFromFloat>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeFromInteger>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeFromName>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeIf>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeInsert>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeLength>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeList>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeMember>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeNoop>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeNthCdr>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeNth>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeNull>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodePop>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodePosition>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeQuote>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeRand>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeRot>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeShove>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeSize>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeStackDepth>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeSubstitute>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeSwap>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeYankDup>();
    vm.engine_mut().add_instruction::<crate::execute_code::CodeYank>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecDefine>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecDoNCount>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecDoNRange>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecDoNTimes>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecDup>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecEqual>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecFlush>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecIf>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecK>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecPop>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecRot>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecShove>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecStackDepth>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecSwap>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecS>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecYankDup>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecYank>();
    vm.engine_mut().add_instruction::<crate::execute_exec::ExecY>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatCos>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatDefine>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatDifference>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatDup>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatEqual>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatFlush>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatFromBoolean>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatFromInteger>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatGreater>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatLess>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatMax>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatMin>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatModulo>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatPop>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatProduct>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatQuotient>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatRand>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatRot>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatShove>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatSin>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatStackDepth>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatSum>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatSwap>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatTan>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatYankDup>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatYank>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerDefine>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerDifference>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerDup>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerEqual>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerFlush>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerFromBoolean>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerFromFloat>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerGreater>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerLess>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerMax>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerMin>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerModulo>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerPop>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerProduct>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerQuotient>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerRand>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerRot>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerShove>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerStackDepth>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerSum>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerSwap>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerYankDup>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerYank>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameDup>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameEqual>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameFlush>();
    vm.engine_mut().add_instruction::<crate::execute_name::NamePop>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameQuote>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameRandBoundName>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameRand>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameRot>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameShove>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameStackDepth>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameSwap>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameYankDup>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameYank>();
}

pub fn add_base_literals<
    Vm: VirtualMachine
        + VirtualMachineMustHaveBool<Vm>
        + VirtualMachineMustHaveExec<Vm>
        + VirtualMachineMustHaveFloat<Vm>
        + VirtualMachineMustHaveInteger<Vm>
        + VirtualMachineMustHaveName<Vm>,
>(
    vm: &mut Vm,
) {
    // These must be last, with Name the very last of all. The reason is that parsing runs in order from top to bottom
    // and all the 'normal' instructions use an exact match. However the literal values use more involved parsing and
    // Name is the catch-all (anything that does not parse earlier will become a Name up to the next white-space).
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolLiteralValue>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatLiteralValue>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerLiteralValue>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameLiteralValue>();
}

#[cfg(test)]
mod tests {
    use super::Extraction;
    use crate::*;

    fn new_base_vm() -> BaseVm {
        let mut vm = BaseVm::new(None, Configuration::new_simple());
        add_base_instructions(&mut vm);
        add_base_literals(&mut vm);

        vm
    }

    #[test]
    fn print_all_opcode_names() {
        // This 'test' prints the name associated with every opcode in the BaseVm. This is used when debugging test code
        // which lists the opcode, but not the name. Run with `cargo test print_all_opcode_names -- --show-output` and
        // keep handy for running the next test
        let vm = new_base_vm();
        let mut opcode = 0;
        while let Some(name) = vm.name_for_opcode(opcode) {
            println!("{} => {}", opcode, name);
            opcode += 1;
        }
    }

    #[test]
    fn not_parsable() {
        let vm = new_base_vm();
        let result = vm.engine().parse("( DOESNT WORK");
        assert!(result.is_err());
    }

    #[test]
    fn code_display() {
        let vm = new_base_vm();
        let code = Code::new_list(vec![]);
        assert_eq!("( )", format!("{}", code.for_display(&vm)));

        let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
        assert_eq!("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )", format!("{}", code.for_display(&vm)));
    }

    #[test]
    fn code_points() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
        assert_eq!(7, code.points());
    }

    #[test]
    fn extract_point() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( A ( B ) )").unwrap();
        assert_eq!(4, code.points());
        assert_eq!(code.extract_point(0), Extraction::Extracted(code.clone()));
        assert_eq!(code.extract_point(1), Extraction::Extracted(vm.engine().must_parse("A")));
        assert_eq!(code.extract_point(2), Extraction::Extracted(vm.engine().must_parse("( B )")));
        assert_eq!(code.extract_point(3), Extraction::Extracted(vm.engine().must_parse("B")));
    }

    #[test]
    fn replace_point() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( A ( B ) )").unwrap();
        let replace_with = vm.engine().must_parse("C");
        assert_eq!(&code.replace_point(0, &replace_with).0, &vm.engine().must_parse("C"));
        assert_eq!(&code.replace_point(1, &replace_with).0, &vm.engine().must_parse("( C ( B ) )"));
        assert_eq!(&code.replace_point(2, &replace_with).0, &vm.engine().must_parse("( A C )"));
        assert_eq!(&code.replace_point(3, &replace_with).0, &vm.engine().must_parse("( A ( C ) )"));
        assert_eq!(&code.replace_point(4, &replace_with).0, &vm.engine().must_parse("( A ( B ) )"));
    }

    #[test]
    fn extract_names() {
        let vm = new_base_vm();
        let code = vm.engine().must_parse("( ANAME ( 1 TRUE ANAME ) BNAME ( ( CNAME ANAME ) ) )");
        let names = code.extract_names();
        assert_eq!(5, names.len());
        assert_eq!(Name::from("ANAME"), names[0]);
        assert_eq!(Name::from("ANAME"), names[1]);
        assert_eq!(Name::from("BNAME"), names[2]);
        assert_eq!(Name::from("CNAME"), names[3]);
        assert_eq!(Name::from("ANAME"), names[4]);
    }

    #[test]
    fn extract_atoms() {
        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
        let atoms = code.extract_atoms();
        assert_eq!(5, atoms.len());
        assert_eq!(&vm.engine().must_parse("TRUE"), &atoms[0]);
        assert_eq!(&vm.engine().must_parse("0.012345"), &atoms[1]);
        assert_eq!(&vm.engine().must_parse("-12784"), &atoms[2]);
        assert_eq!(&vm.engine().must_parse("a_name"), &atoms[3]);
        assert_eq!(&vm.engine().must_parse("BOOL.AND"), &atoms[4]);
    }

    #[test]
    fn code_discrepancy_items() {
        let vm = new_base_vm();
        // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
        let code = vm.engine().must_parse("( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )");
        let items = code.discrepancy_items();
        assert_eq!(1, *items.get(&vm.engine().must_parse("( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("ANAME")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("( 3 ( 1 ) )")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("3")).unwrap());
        assert_eq!(2, *items.get(&vm.engine().must_parse("( 1 )")).unwrap());
        assert_eq!(3, *items.get(&vm.engine().must_parse("1")).unwrap());
        assert_eq!(6, items.len());
    }

    #[test]
    fn code_len() {
        let vm = new_base_vm();
        // `len` returns the number of elements in the direct list (not sub-lists)
        assert_eq!(0, vm.engine().must_parse("( )").len());
        assert_eq!(1, vm.engine().must_parse("( A )").len());
        assert_eq!(2, vm.engine().must_parse("( A B )").len());
        assert_eq!(2, vm.engine().must_parse("( A ( B C ) )").len());

        // It also returns 1 for atoms
        assert_eq!(1, vm.engine().must_parse("A").len());
    }

    #[test]
    fn replace() {
        let vm = new_base_vm();
        assert_eq!(
            &vm.engine().must_parse("B"),
            &vm.engine().must_parse("A").replace(&vm.engine().must_parse("A"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( B )"),
            &vm.engine().must_parse("( A )").replace(&vm.engine().must_parse("A"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( B B )"),
            &vm.engine().must_parse("( A A )").replace(&vm.engine().must_parse("A"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("B"),
            &vm.engine().must_parse("( A )").replace(&vm.engine().must_parse("( A )"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( B )"),
            &vm.engine()
                .must_parse("( ( A ) )")
                .replace(&vm.engine().must_parse("( A )"), &vm.engine().must_parse("B"))
        );
        assert_eq!(
            &vm.engine().must_parse("( A A ( A A ) )"),
            &vm.engine()
                .must_parse("( A ( B ) ( A ( B ) ) )")
                .replace(&vm.engine().must_parse("( B )"), &vm.engine().must_parse("A"))
        );
    }
}
