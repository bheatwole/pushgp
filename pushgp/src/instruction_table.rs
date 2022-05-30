use crate::InstructionData;
use fnv::FnvHashMap;

pub type VirtualTableParse = fn(input: &str) -> nom::IResult<&str, Option<InstructionData>>;
pub type VirtualTableNomFmt = fn(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
pub type VirtualTableRandomValue = fn(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData>;
pub type VirtualTableExecute = fn(context: &mut crate::context::NewContext, data: &Option<InstructionData>);

pub struct VirtualTable {
    /// Maps the names found in the printed code to the indices used in all the other fields
    names: FnvHashMap<&'static str, usize>,

    /// Virtual table for Instruction::parse
    parse: Vec<VirtualTableParse>,

    /// Virtual table for Instruction::nom_fmt
    nom_fmt: Vec<VirtualTableNomFmt>,

    /// Virtual table for Instruction::random_value
    random_value: Vec<VirtualTableRandomValue>,

    /// Virtual table for Instruction::execute
    execute: Vec<VirtualTableExecute>,
}

impl VirtualTable {
    pub fn new() -> VirtualTable {
        VirtualTable {
            names: FnvHashMap::default(),
            parse: vec![],
            nom_fmt: vec![],
            random_value: vec![],
            execute: vec![],
        }
    }

    pub fn add_entry(
        &mut self,
        name: &'static str,
        parse: VirtualTableParse,
        nom_fmt: VirtualTableNomFmt,
        random_value: VirtualTableRandomValue,
        execute: VirtualTableExecute,
    ) {
        let id = self.parse.len();
        self.names.insert(name, id);
        self.parse.push(parse);
        self.nom_fmt.push(nom_fmt);
        self.random_value.push(random_value);
        self.execute.push(execute);
    }
}

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

    pub fn get(&self, name: &String) -> Option<&fn(&mut C)> {
        self.table.get(name)
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
