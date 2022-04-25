use fnv::FnvHashMap;
use std::pin::Pin;

pub struct InstructionTable<C> {
    table: FnvHashMap<String, fn (&mut C)>
}

impl<C> InstructionTable<C> {
    pub fn new() -> InstructionTable<C> {
        InstructionTable {
            table: FnvHashMap::default(),
        }
    }

    pub fn set(&mut self, name: &str, call: fn(&mut C)) {
        self.table.insert(name.to_owned(), call);
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