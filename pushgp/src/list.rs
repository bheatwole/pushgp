use crate::*;

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

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm> + 'static> StaticInstruction<Vm> for PushList<Vm> {
    // The PushList cannot be parsed this way because it requires recursive parsing (and thus access to the parser). See
    // parse.rs for the implementation of recursive parsing
    fn parse(input: &str) -> nom::IResult<&str, Code<Vm>> {
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }

    // A PushList should typically have its weight set to zero and never called for a random value. The tree of
    // Code values is created in the random code generation.
    fn random_value(_vm: &mut Vm) -> Code<Vm> {
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

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm> + 'static> Instruction<Vm> for PushList<Vm> {
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
}