use crate::{Code, InstructionData, NewContext};
use fnv::FnvHashMap;

pub type VirtualTableParse = fn(input: &str) -> nom::IResult<&str, Option<InstructionData>>;
pub type VirtualTableNomFmt = fn(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
pub type VirtualTableRandomValue = fn(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData>;
pub type VirtualTableExecute = fn(context: &crate::context::NewContext, data: Option<InstructionData>);

#[derive(Clone)]
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

    pub fn all_instruction_names(&self) -> Vec<&'static str> {
        self.names.keys().map(|k| *k).collect()
    }

    pub fn len(&self) -> usize {
        self.parse.len()
    }

    pub fn id_for_name<S: AsRef<str>>(&self, name: S) -> Option<usize> {
        self.names.get(name.as_ref()).map(|v| *v)
    }

    pub fn call_parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code> {
        for (id, parse) in self.parse.iter().enumerate() {
            match parse(input) {
                Ok((rest, data)) => return Ok((rest, Code::InstructionWithData(id, data))),
                Err(_) => {
                    // Continue searching
                }
            }
        }

        // Return an error if we could not parse this item
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }

    pub fn call_nom_fmt(
        &self,
        instruction: usize,
        data: &Option<InstructionData>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        assert!(instruction < self.nom_fmt.len());
        self.nom_fmt[instruction](data, f)
    }

    pub fn call_random_value(&self, instruction: usize, rng: &mut rand::rngs::SmallRng) -> Option<InstructionData> {
        assert!(instruction < self.random_value.len());
        self.random_value[instruction](rng)
    }

    pub fn call_execute(&self, instruction: usize, context: &NewContext, data: Option<InstructionData>) {
        assert!(instruction < self.execute.len());
        self.execute[instruction](context, data)
    }
}

impl std::fmt::Debug for VirtualTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut self_keys: Vec<&'static str> = self.names.keys().map(|k| *k).collect();
        self_keys.sort();

        f.debug_struct("VirtualTable").field("names", &self_keys).finish()
    }
}

impl PartialEq for VirtualTable {
    fn eq(&self, other: &Self) -> bool {
        let mut self_keys: Vec<&'static str> = self.names.keys().map(|k| *k).collect();
        let mut other_keys: Vec<&'static str> = other.names.keys().map(|k| *k).collect();

        self_keys.sort();
        other_keys.sort();

        self_keys == other_keys
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
