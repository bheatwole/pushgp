use crate::*;
use fnv::FnvHashMap;
use log::*;

pub struct InstructionTable<C> {
    table: FnvHashMap<String, fn(&mut C)>,
}

impl<C> InstructionTable<C> {
    pub fn new() -> InstructionTable<C> {
        InstructionTable { table: FnvHashMap::default() }
    }

    pub fn set(&mut self, name: &str, call: fn(&mut C)) {
        self.table.insert(name.to_owned(), call);
    }

    pub fn execute(&self, name: &String, context: &mut C) {
        trace!("executing instruction {}", name);
        if let Some(func) = self.table.get(name) {
            func(context)
        } else {
            debug!("unable to find function for {} in instruction table", name);
        }
    }

    /// Returns a sorted list of the names of all instructions. This list is suitable for a binary search. It does make
    /// a copy of each key, so do not call this function very often
    pub fn all_instruction_names(&self) -> Vec<String> {
        self.all_instruction_names_borrowed().iter().map(|n| (*n).clone()).collect()
    }

    /// Returns a sorted list of the names of all instructions. The items are borrowed from the instruction table. This
    /// list is suitable for a binary search.
    pub fn all_instruction_names_borrowed(&self) -> Vec<&String> {
        let mut self_keys: Vec<&String> = self.table.keys().collect();
        self_keys.sort();

        self_keys
    }
}

impl<C> std::fmt::Debug for InstructionTable<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut self_keys: Vec<&String> = self.table.keys().collect();
        self_keys.sort();

        write!(f, "instruction_table: {:?}", self_keys)
    }
}

impl<C> PartialEq for InstructionTable<C> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_keys: Vec<&String> = self.table.keys().collect();
        let mut other_keys: Vec<&String> = other.table.keys().collect();

        self_keys.sort();
        other_keys.sort();

        self_keys == other_keys
    }
}

/// Creates a new instruction table that contains every instruction known in the base library. This is suitable for
/// appending your custom instructions. This will fail to compile if your Context does not include stacks for all
/// base library types.
pub fn new_instruction_table_with_all_instructions<C, L>() -> InstructionTable<C>
where
    C: Context + ContextHasBoolStack<L> + ContextHasNameStack<L>,
    L: LiteralEnum<L>,
{
    let mut instructions = InstructionTable::new();
    crate::execute_bool::BoolAnd::<C, L>::add_to_table(&mut instructions);
    crate::execute_bool::BoolDefine::<C, L>::add_to_table(&mut instructions);
    crate::execute_bool::BoolDup::<C, L>::add_to_table(&mut instructions);

    instructions
}
