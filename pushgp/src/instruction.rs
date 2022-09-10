use get_size::GetSize;

use crate::{Code, Extraction, StaticName, VirtualMachine, VirtualMachineEngine};

/// This trait includes the functions of an instruction that must remain static. If they were included on the Instruction
/// trait, it could no longer be a trait object
pub trait StaticInstruction<Vm: VirtualMachine>: StaticName {
    /// All Code must be parsable by 'nom' from a string.
    fn parse(input: &str) -> nom::IResult<&str, Code<Vm>>;

    /// All Code must be able to create a new 'random' value. For pure instructions that have no data, the 'random'
    /// value is always the same: the instruction. For instructions that do have data (BOOL.LITERALVALUE,
    /// INTEGER.LITERALVALUE, CODE.CODE, etc.), the instruction created will use the random number generator from the
    /// VirtualMachineEngine to create random data
    fn random_value(engine: &mut VirtualMachineEngine<Vm>) -> Code<Vm>;
}

/// The Instruction is a trait that allows use as a trait object. This significantly restricts what kinds of methods
/// we can include in this trait.
///
/// It is generic for a VirtualMachine. Most instructions will place additional `where` constraints on the VM. I.E. an
/// instruction may require the VM to implement VirtualMachineHasBoolStack, VirtualMachineHasCodeStack and
/// VirtualMachineHasGameState. (VirtualMachineHasGameState being a trait defined in the user's code)
pub trait Instruction<Vm: 'static>: std::any::Any + std::fmt::Display {
    /// The instruction must be able to report what type it is.
    fn as_any(&self) -> &dyn std::any::Any;

    /// Every instruction must have a name that is known at compile-time
    fn name(&self) -> &'static str;

    /// Every instruction must known how much memory it uses (stack + heap)
    fn size_of(&self) -> usize {
        0
    }

    /// The instruction must be able to clone itself, though we cannot implement the normal 'Clone' trait or the
    /// Instruction could not become a trait object
    fn clone(&self) -> Box<dyn Instruction<Vm>>;

    /// The instruction must be able to execute on a virtual machine. The instruction must never panic and may only
    /// update the state of the virtual machine
    fn execute(&mut self, vm: &mut Vm);

    /// An instruction must be able to determine if it is equal to another instruction. We cannot simply use PartialEq
    /// and Eq because then Instruction could not become a trait object.
    ///
    /// The default implementation simply checks to see if the static name string points to the same location in memory.
    /// Instructions that have data MUST override this behavior to check for data equivalence.
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        std::ptr::eq(self.name(), other.name())
    }

    /// An instruction must be able to provide a unique hash of itself and its data. The hash() of two instructions that
    /// are eq() to each other should also be equal.
    ///
    /// We cannot simply use Hash because then Instruction could not become a trait object (no methods with generics).
    ///
    /// The default implementation uses the sea-hash of the name. Instructions that have data MUST override this
    /// behavior to include the data in the hash.
    fn hash(&self) -> u64 {
        seahash::hash(self.name().as_bytes())
    }

    /// Returns true if this code is a List
    fn is_list(&self) -> bool {
        false
    }

    /// Returns true if this code is anything BUT a list
    fn is_atom(&self) -> bool {
        true
    }

    /// Returns true if the specified code is equal to this item or any child
    fn contains(&self, look_for: &dyn Instruction<Vm>) -> bool {
        self.eq(look_for)
    }

    /// Returns the smallest sub-list that contains the specified code
    fn container(&self, _look_for: &dyn Instruction<Vm>) -> Option<Box<dyn Instruction<Vm>>> {
        None
    }

    /// Similar to `contains` but does not recurse into Lists
    fn has_member(&self, look_for: &dyn Instruction<Vm>) -> bool {
        self.eq(look_for)
    }

    /// Similar to `extract_point` but does not recurse into lists
    fn position_of(&self, look_for: &dyn Instruction<Vm>) -> Option<usize> {
        if self.eq(look_for) {
            Some(0)
        } else {
            None
        }
    }

    /// The discrepancy output is a HashMap of every unique sub-list and atom from the specified code
    fn discrepancy_items(&self) -> fnv::FnvHashMap<Code<Vm>, i64> {
        let mut items = fnv::FnvHashMap::default();
        let counter = items.entry(self.clone()).or_insert(0);
        *counter += 1;

        items
    }

    /// Appends this item to an already-existing discrepancy items HashMap
    fn append_discrepancy_items(&self, items: &mut fnv::FnvHashMap<Code<Vm>, i64>) {
        let counter = items.entry(self.clone()).or_insert(0);
        *counter += 1;
    }

    /// Coerces the item to a list
    fn to_list(&self) -> Vec<Box<dyn Instruction<Vm>>> {
        vec![self.clone()]
    }

    /// Returns the number of 'points' of the entire code. Each atom and list is considered one point.
    fn points(&self) -> i64 {
        1
    }

    /// Returns the item of code at the specified 'point' in the code tree if `point` is less than the number of points
    /// in the code. Returns the number of points used otherwise.
    fn extract_point(&self, point: i64) -> Extraction<Vm> {
        if 0 == point {
            return Extraction::Extracted(self.clone());
        }
        Extraction::Used(1)
    }

    /// Descends to the specified point in the code tree and swaps the list or atom there with the specified replacement
    /// code. If the replacement point is greater than the number of points in the Code, this has no effect.
    fn replace_point(&self, point: i64, replace_with: &dyn Instruction<Vm>) -> (Box<dyn Instruction<Vm>>, i64) {
        // If this is the replacement point, return the replacement
        if 0 == point {
            (replace_with.clone(), 1)
        } else {
            // Atoms are returned as-is
            (self.clone(), 1)
        }
    }

    /// Returns a list of all names found in the instruction. The default implementation returns an empty list
    fn extract_names(&self) -> Vec<String> {
        vec![]
    }

    /// Returns a list of clones of all the atoms found in the instruction. The default implementation returns a list
    /// that contains a clone of itself
    fn extract_atoms(&self) -> Vec<Box<dyn Instruction<Vm>>> {
        vec![self.clone()]
    }

    /// Returns the number of items in this list. Unlike 'points' it does not recurse into sub-lists
    fn len(&self) -> usize {
        1
    }

    /// Replaces the specified search code with the specified replacement code
    fn replace(&self, look_for: &dyn Instruction<Vm>, replace_with: &dyn Instruction<Vm>) -> Box<dyn Instruction<Vm>> {
        if self.eq(look_for) {
            return replace_with.clone();
        }
        self.clone()
    }
}

/// While we cannot implement Clone for the raw trait object, we CAN implement Clone for the boxed instruction, which
/// allows us to put the Box<dyn Instruction> into a Stack<T: Clone>
impl<Vm: 'static> Clone for Box<dyn Instruction<Vm>> {
    fn clone(&self) -> Self {
        self.as_ref().clone()
    }
}

/// While we cannot implement Debug for the raw trait object, we CAN implement Debug for the boxed instruction, which
/// allows us to put the Box<dyn Instruction> into a Stack<T: Debug>
impl<Vm> std::fmt::Debug for Box<dyn Instruction<Vm>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

/// While we cannot implement PartialEq for the raw trait object, we CAN implement PartialEq for the boxed instruction,
/// which allows us to put the Box<dyn Instruction> into a Stack<T: PartialEq>
impl<Vm: 'static> std::cmp::PartialEq for Box<dyn Instruction<Vm>> {
    fn eq(&self, other: &Box<dyn Instruction<Vm>>) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}
impl<Vm: 'static> std::cmp::Eq for Box<dyn Instruction<Vm>> {}

/// While we cannot implement Hash for the raw trait object, we CAN implement Hash for the boxed instruction,
/// which allows us to put the Box<dyn Instruction> into a HashMap
impl<Vm: 'static> std::hash::Hash for Box<dyn Instruction<Vm>> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.as_ref().hash())
    }
}

impl<Vm: 'static> GetSize for Box<dyn Instruction<Vm>> {
    fn get_heap_size(&self) -> usize {
        self.size_of()
    }
}
