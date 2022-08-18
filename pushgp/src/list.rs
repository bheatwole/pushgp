use crate::*;
use fnv::FnvHashMap;

#[derive(Debug)]
pub struct PushList<Vm> {
    value: Vec<Code<Vm>>,
}

impl<Vm> PushList<Vm> {
    pub fn new(value: Vec<Code<Vm>>) -> PushList<Vm> {
        PushList { value }
    }
}

impl<Vm> StaticName for PushList<Vm> {
    fn static_name() -> &'static str {
        "__PUSH.LIST"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> StaticInstruction<Vm> for PushList<Vm> {
    // The PushList cannot be parsed this way because it requires recursive parsing (and thus access to the parser). See
    // parse.rs for the implementation of recursive parsing
    fn parse(input: &str) -> nom::IResult<&str, Code<Vm>> {
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }

    // A PushList should typically have its weight set to zero and never called for a random value. The tree of
    // Code values is created in the random code generation.
    fn random_value(_engine: &mut VirtualMachineEngine<Vm>) -> Code<Vm> {
        Box::new(PushList::new(vec![]))
    }
}

impl<Vm> std::fmt::Display for PushList<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for c in self.value.iter() {
            write!(f, " {}", c)?;
        }
        write!(f, " )")
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> Instruction<Vm> for PushList<Vm> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        PushList::<Vm>::static_name()
    }

    fn clone(&self) -> Code<Vm> {
        let mut cloned_list = vec![];
        for item in self.value.iter() {
            cloned_list.push(item.clone());
        }
        Box::new(PushList::new(cloned_list))
    }

    /// Executing a PushList pushes the items of the list onto the Exec stack so that the first item in the list is
    /// executed next
    fn execute(&mut self, vm: &mut Vm) {
        while let Some(item) = self.value.pop() {
            vm.exec().push(item);
        }
    }

    /// Eq for PushList must check that the other instruction is also a PushList and, if so, that each item in each list
    /// is equivalent
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<PushList<Vm>>() {
            if self.value.len() == other.value.len() {
                for i in 0..self.value.len() {
                    if self.value.get(i).unwrap() != other.value.get(i).unwrap() {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// The hash value for PushList include the value in the hash as well as the name
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = PushList::<Vm>::static_name().as_bytes().iter().map(|c| *c).collect();
        for item in self.value.iter() {
            to_hash.push(b' ');
            let normalized = item.hash().to_string();
            to_hash.extend_from_slice(normalized.as_bytes());
        }
        seahash::hash(&to_hash[..])
    }

    /// Returns true if this code is a List
    fn is_list(&self) -> bool {
        true
    }

    /// Returns true if this code is anything BUT a list
    fn is_atom(&self) -> bool {
        false
    }

    /// Returns true if the specified code is equal to this item or any child
    fn contains(&self, look_for: &dyn Instruction<Vm>) -> bool {
        for i in self.value.iter() {
            if i.as_ref().eq(look_for) {
                return true;
            }

            if i.is_list() {
                if i.contains(look_for) {
                    return true;
                }
            }
        }
        false
    }

    /// Returns the smallest sub-list that contains the specified code
    fn container(&self, look_for: &dyn Instruction<Vm>) -> Option<Box<dyn Instruction<Vm>>> {
        for i in self.value.iter() {
            if i.as_ref().eq(look_for) {
                return Some(Box::new(PushList::new(self.value.clone())));
            }
        }
        for i in self.value.iter() {
            if let Some(code) = i.container(look_for) {
                return Some(code);
            }
        }
        None
    }

    /// Similar to `contains` but does not recurse into Lists
    fn has_member(&self, look_for: &dyn Instruction<Vm>) -> bool {
        for item in self.value.iter() {
            if item.as_ref().eq(look_for) {
                return true;
            }
        }
        false
    }

    /// Similar to `extract_point` but does not recurse into lists
    fn position_of(&self, look_for: &dyn Instruction<Vm>) -> Option<usize> {
        for (i, item) in self.value.iter().enumerate() {
            if item.as_ref().eq(look_for) {
                return Some(i);
            }
        }
        None
    }

    /// The discrepancy output is a HashMap of every unique sub-list and atom from the specified code
    fn discrepancy_items(&self) -> FnvHashMap<Code<Vm>, i64> {
        let mut items = FnvHashMap::default();
        for i in self.value.iter() {
            if i.is_list() {
                let counter = items.entry(i.clone()).or_insert(0);
                *counter += 1;
            }
            for (key, count) in i.discrepancy_items() {
                let counter = items.entry(key).or_insert(0);
                *counter += count;
            }
        }

        items
    }

    /// Coerces the item to a list
    fn to_list(&self) -> Vec<Box<dyn Instruction<Vm>>> {
        self.value.clone()
    }

    /// Returns the number of 'points' of the entire code. Each atom and list is considered one point.
    fn points(&self) -> i64 {
        let sub_points: i64 = self.value.iter().map(|c| c.points()).sum();
        1 + sub_points
    }

    /// Returns the item of code at the specified 'point' in the code tree if `point` is less than the number of points
    /// in the code. Returns the number of points used otherwise.
    fn extract_point(&self, point: i64) -> Extraction<Vm> {
        if 0 == point {
            return Extraction::Extracted(self.clone());
        }
        let mut used = 1;
        for item in self.value.iter() {
            match item.extract_point(point - used) {
                Extraction::Extracted(code) => return Extraction::Extracted(code),
                Extraction::Used(u) => used += u,
            }
        }
        Extraction::Used(used)
    }

    /// Descends to the specified point in the code tree and swaps the list or atom there with the specified replacement
    /// code. If the replacement point is greater than the number of points in the Code, this has no effect.
    fn replace_point(&self, mut point: i64, replace_with: &dyn Instruction<Vm>) -> (Box<dyn Instruction<Vm>>, i64) {
        // If this is the replacement point, return the replacement
        if 0 == point {
            (replace_with.clone(), 1)
        } else if point < 1 {
            // After we've performed the replacement, everything gets returned as-is
            (self.clone(), 1)
        } else {
            // We need to track both the number of points used and the points remaining until replacement.
            let mut next_list = vec![];
            let mut total_used = 1;
            point -= 1;
            for item in self.value.iter() {
                let (next, used) = item.replace_point(point, replace_with);
                point -= used;
                total_used += used;
                next_list.push(next);
            }
            (Box::new(PushList::new(next_list)), total_used)
        }
    }

    /// Returns the number of items in this list. Unlike 'points' it does not recurse into sub-lists
    fn len(&self) -> usize {
        self.value.len()
    }

    /// Replaces the specified search code with the specified replacement code
    fn replace(&self, look_for: &dyn Instruction<Vm>, replace_with: &dyn Instruction<Vm>) -> Box<dyn Instruction<Vm>> {
        if self.eq(look_for) {
            return replace_with.clone();
        }

        let mut next_list = vec![];
        for item in self.value.iter() {
            next_list.push(item.replace(look_for, replace_with));
        }
        Box::new(PushList::new(next_list))
    }
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
    fn not_parsable() {
        let vm = new_base_vm();
        let result = vm.engine().parse("( DOESNT WORK");
        assert!(result.is_err());
    }

    #[test]
    fn code_display() {
        let code = PushList::<BaseVm>::new(vec![]);
        assert_eq!("( )", format!("{}", code));

        let vm = new_base_vm();
        let (_, code) = vm.engine().parse("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )").unwrap();
        assert_eq!("( ( TRUE 0.012345 -12784 a_name ) BOOL.AND )", format!("{}", code));
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
        assert_eq!(&code.replace_point(0, replace_with.as_ref()).0, &vm.engine().must_parse("C"));
        assert_eq!(&code.replace_point(1, replace_with.as_ref()).0, &vm.engine().must_parse("( C ( B ) )"));
        assert_eq!(&code.replace_point(2, replace_with.as_ref()).0, &vm.engine().must_parse("( A C )"));
        assert_eq!(&code.replace_point(3, replace_with.as_ref()).0, &vm.engine().must_parse("( A ( C ) )"));
        assert_eq!(&code.replace_point(4, replace_with.as_ref()).0, &vm.engine().must_parse("( A ( B ) )"));
    }

    #[test]
    fn code_discrepancy_items() {
        let vm = new_base_vm();
        // The discrepancy output is a hashset of every unique sub-list and atom from the specified code
        let code = vm.engine().must_parse("( ANAME ( 3 ( 1 ) ) 1 ( 1 ) )");
        let items = code.discrepancy_items();
        assert_eq!(1, *items.get(&vm.engine().must_parse("ANAME")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("( 3 ( 1 ) )")).unwrap());
        assert_eq!(1, *items.get(&vm.engine().must_parse("3")).unwrap());
        assert_eq!(2, *items.get(&vm.engine().must_parse("( 1 )")).unwrap());
        assert_eq!(3, *items.get(&vm.engine().must_parse("1")).unwrap());
        assert_eq!(5, items.len());
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
            &vm.engine().must_parse("A").replace(vm.engine().must_parse("A").as_ref(), vm.engine().must_parse("B").as_ref())
        );
        assert_eq!(
            &vm.engine().must_parse("( B )"),
            &vm.engine().must_parse("( A )").replace(vm.engine().must_parse("A").as_ref(), vm.engine().must_parse("B").as_ref())
        );
        assert_eq!(
            &vm.engine().must_parse("( B B )"),
            &vm.engine().must_parse("( A A )").replace(vm.engine().must_parse("A").as_ref(), vm.engine().must_parse("B").as_ref())
        );
        assert_eq!(
            &vm.engine().must_parse("B"),
            &vm.engine().must_parse("( A )").replace(vm.engine().must_parse("( A )").as_ref(), vm.engine().must_parse("B").as_ref())
        );
        assert_eq!(
            &vm.engine().must_parse("( B )"),
            &vm.engine().must_parse("( ( A ) )").replace(vm.engine().must_parse("( A )").as_ref(), vm.engine().must_parse("B").as_ref())
        );
        assert_eq!(
            &vm.engine().must_parse("( A A ( A A ) )"),
            &vm.engine().must_parse("( A ( B ) ( A ( B ) ) )")
                .replace(vm.engine().must_parse("( B )").as_ref(), vm.engine().must_parse("A").as_ref())
        );
    }
}
