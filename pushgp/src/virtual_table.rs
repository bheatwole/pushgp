use crate::{Code, Context, Instruction, InstructionData};
use fnv::FnvHashMap;

pub type VirtualTableParse = fn(input: &str) -> nom::IResult<&str, Option<InstructionData>>;
pub type VirtualTableNomFmt = fn(data: &Option<InstructionData>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
pub type VirtualTableRandomValue = fn(rng: &mut rand::rngs::SmallRng) -> Option<InstructionData>;

#[derive(Clone)]
pub struct VirtualTable<State: std::fmt::Debug + Clone> {
    /// Maps the names found in the printed code to the indices used in all the other fields
    names: FnvHashMap<&'static str, usize>,

    /// Virtual table for Instruction::parse
    parse: Vec<VirtualTableParse>,

    /// Virtual table for Instruction::nom_fmt
    nom_fmt: Vec<VirtualTableNomFmt>,

    /// Virtual table for Instruction::random_value
    random_value: Vec<VirtualTableRandomValue>,

    /// Virtual table for Instruction::execute
    execute: Vec<fn(context: &crate::context::Context<State>, data: Option<InstructionData>)>,
}

impl<State: std::fmt::Debug + Clone> VirtualTable<State> {
    pub fn new() -> VirtualTable<State> {
        VirtualTable {
            names: FnvHashMap::default(),
            parse: vec![],
            nom_fmt: vec![],
            random_value: vec![],
            execute: vec![],
        }
    }

    pub fn new_with_all_instructions() -> VirtualTable<State> {
        let mut virtual_table = VirtualTable::new();
    
        virtual_table.add_base_instructions();
        virtual_table.add_base_literals();
        virtual_table
    }

    pub fn add_entry(
        &mut self,
        name: &'static str,
        parse: VirtualTableParse,
        nom_fmt: VirtualTableNomFmt,
        random_value: VirtualTableRandomValue,
        execute: fn(context: &crate::context::Context<State>, data: Option<InstructionData>),
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

    pub fn call_execute(&self, instruction: usize, context: &Context<State>, data: Option<InstructionData>) {
        assert!(instruction < self.execute.len());
        self.execute[instruction](context, data)
    }

    pub fn add_base_instructions(&mut self) {
        crate::execute_bool::BoolAnd::add_to_virtual_table(self);
        crate::execute_bool::BoolDefine::add_to_virtual_table(self);
        crate::execute_bool::BoolDup::add_to_virtual_table(self);
        crate::execute_bool::BoolEqual::add_to_virtual_table(self);
        crate::execute_bool::BoolFlush::add_to_virtual_table(self);
        crate::execute_bool::BoolFromFloat::add_to_virtual_table(self);
        crate::execute_bool::BoolFromInt::add_to_virtual_table(self);
        crate::execute_bool::BoolNot::add_to_virtual_table(self);
        crate::execute_bool::BoolOr::add_to_virtual_table(self);
        crate::execute_bool::BoolPop::add_to_virtual_table(self);
        crate::execute_bool::BoolRand::add_to_virtual_table(self);
        crate::execute_bool::BoolRot::add_to_virtual_table(self);
        crate::execute_bool::BoolShove::add_to_virtual_table(self);
        crate::execute_bool::BoolStackDepth::add_to_virtual_table(self);
        crate::execute_bool::BoolSwap::add_to_virtual_table(self);
        crate::execute_bool::BoolYankDup::add_to_virtual_table(self);
        crate::execute_bool::BoolYank::add_to_virtual_table(self);
        crate::execute_code::CodeAppend::add_to_virtual_table(self);
        crate::execute_code::CodeAtom::add_to_virtual_table(self);
        crate::execute_code::CodeCar::add_to_virtual_table(self);
        crate::execute_code::CodeCdr::add_to_virtual_table(self);
        crate::execute_code::CodeCons::add_to_virtual_table(self);
        crate::execute_code::CodeContainer::add_to_virtual_table(self);
        crate::execute_code::CodeContains::add_to_virtual_table(self);
        crate::execute_code::CodeDefine::add_to_virtual_table(self);
        crate::execute_code::CodeDefinition::add_to_virtual_table(self);
        crate::execute_code::CodeDiscrepancy::add_to_virtual_table(self);
        crate::execute_code::CodeDoNCount::add_to_virtual_table(self);
        crate::execute_code::CodeDoNRange::add_to_virtual_table(self);
        crate::execute_code::CodeDoNTimes::add_to_virtual_table(self);
        crate::execute_code::CodeDoN::add_to_virtual_table(self);
        crate::execute_code::CodeDo::add_to_virtual_table(self);
        crate::execute_code::CodeDup::add_to_virtual_table(self);
        crate::execute_code::CodeEqual::add_to_virtual_table(self);
        crate::execute_code::CodeExtract::add_to_virtual_table(self);
        crate::execute_code::CodeFlush::add_to_virtual_table(self);
        crate::execute_code::CodeFromBoolean::add_to_virtual_table(self);
        crate::execute_code::CodeFromFloat::add_to_virtual_table(self);
        crate::execute_code::CodeFromInteger::add_to_virtual_table(self);
        crate::execute_code::CodeFromName::add_to_virtual_table(self);
        crate::execute_code::CodeIf::add_to_virtual_table(self);
        crate::execute_code::CodeInsert::add_to_virtual_table(self);
        crate::execute_code::CodeLength::add_to_virtual_table(self);
        crate::execute_code::CodeList::add_to_virtual_table(self);
        crate::execute_code::CodeMember::add_to_virtual_table(self);
        crate::execute_code::CodeNoop::add_to_virtual_table(self);
        crate::execute_code::CodeNthCdr::add_to_virtual_table(self);
        crate::execute_code::CodeNth::add_to_virtual_table(self);
        crate::execute_code::CodeNull::add_to_virtual_table(self);
        crate::execute_code::CodePop::add_to_virtual_table(self);
        crate::execute_code::CodePosition::add_to_virtual_table(self);
        crate::execute_code::CodeQuote::add_to_virtual_table(self);
        crate::execute_code::CodeRand::add_to_virtual_table(self);
        crate::execute_code::CodeRot::add_to_virtual_table(self);
        crate::execute_code::CodeShove::add_to_virtual_table(self);
        crate::execute_code::CodeSize::add_to_virtual_table(self);
        crate::execute_code::CodeStackDepth::add_to_virtual_table(self);
        crate::execute_code::CodeSubstitute::add_to_virtual_table(self);
        crate::execute_code::CodeSwap::add_to_virtual_table(self);
        crate::execute_code::CodeYankDup::add_to_virtual_table(self);
        crate::execute_code::CodeYank::add_to_virtual_table(self);
        crate::execute_exec::ExecDefine::add_to_virtual_table(self);
        crate::execute_exec::ExecDoNCount::add_to_virtual_table(self);
        crate::execute_exec::ExecDoNRange::add_to_virtual_table(self);
        crate::execute_exec::ExecDoNTimes::add_to_virtual_table(self);
        crate::execute_exec::ExecDup::add_to_virtual_table(self);
        crate::execute_exec::ExecEqual::add_to_virtual_table(self);
        crate::execute_exec::ExecFlush::add_to_virtual_table(self);
        crate::execute_exec::ExecIf::add_to_virtual_table(self);
        crate::execute_exec::ExecK::add_to_virtual_table(self);
        crate::execute_exec::ExecPop::add_to_virtual_table(self);
        crate::execute_exec::ExecRot::add_to_virtual_table(self);
        crate::execute_exec::ExecShove::add_to_virtual_table(self);
        crate::execute_exec::ExecStackDepth::add_to_virtual_table(self);
        crate::execute_exec::ExecSwap::add_to_virtual_table(self);
        crate::execute_exec::ExecS::add_to_virtual_table(self);
        crate::execute_exec::ExecYankDup::add_to_virtual_table(self);
        crate::execute_exec::ExecYank::add_to_virtual_table(self);
        crate::execute_exec::ExecY::add_to_virtual_table(self);
        crate::execute_float::FloatCos::add_to_virtual_table(self);
        crate::execute_float::FloatDefine::add_to_virtual_table(self);
        crate::execute_float::FloatDifference::add_to_virtual_table(self);
        crate::execute_float::FloatDup::add_to_virtual_table(self);
        crate::execute_float::FloatEqual::add_to_virtual_table(self);
        crate::execute_float::FloatFlush::add_to_virtual_table(self);
        crate::execute_float::FloatFromBoolean::add_to_virtual_table(self);
        crate::execute_float::FloatFromInteger::add_to_virtual_table(self);
        crate::execute_float::FloatGreater::add_to_virtual_table(self);
        crate::execute_float::FloatLess::add_to_virtual_table(self);
        crate::execute_float::FloatMax::add_to_virtual_table(self);
        crate::execute_float::FloatMin::add_to_virtual_table(self);
        crate::execute_float::FloatModulo::add_to_virtual_table(self);
        crate::execute_float::FloatPop::add_to_virtual_table(self);
        crate::execute_float::FloatProduct::add_to_virtual_table(self);
        crate::execute_float::FloatQuotient::add_to_virtual_table(self);
        crate::execute_float::FloatRand::add_to_virtual_table(self);
        crate::execute_float::FloatRot::add_to_virtual_table(self);
        crate::execute_float::FloatShove::add_to_virtual_table(self);
        crate::execute_float::FloatSin::add_to_virtual_table(self);
        crate::execute_float::FloatStackDepth::add_to_virtual_table(self);
        crate::execute_float::FloatSum::add_to_virtual_table(self);
        crate::execute_float::FloatSwap::add_to_virtual_table(self);
        crate::execute_float::FloatTan::add_to_virtual_table(self);
        crate::execute_float::FloatYankDup::add_to_virtual_table(self);
        crate::execute_float::FloatYank::add_to_virtual_table(self);
        crate::execute_integer::IntegerDefine::add_to_virtual_table(self);
        crate::execute_integer::IntegerDifference::add_to_virtual_table(self);
        crate::execute_integer::IntegerDup::add_to_virtual_table(self);
        crate::execute_integer::IntegerEqual::add_to_virtual_table(self);
        crate::execute_integer::IntegerFlush::add_to_virtual_table(self);
        crate::execute_integer::IntegerFromBoolean::add_to_virtual_table(self);
        crate::execute_integer::IntegerFromFloat::add_to_virtual_table(self);
        crate::execute_integer::IntegerGreater::add_to_virtual_table(self);
        crate::execute_integer::IntegerLess::add_to_virtual_table(self);
        crate::execute_integer::IntegerMax::add_to_virtual_table(self);
        crate::execute_integer::IntegerMin::add_to_virtual_table(self);
        crate::execute_integer::IntegerModulo::add_to_virtual_table(self);
        crate::execute_integer::IntegerPop::add_to_virtual_table(self);
        crate::execute_integer::IntegerProduct::add_to_virtual_table(self);
        crate::execute_integer::IntegerQuotient::add_to_virtual_table(self);
        crate::execute_integer::IntegerRand::add_to_virtual_table(self);
        crate::execute_integer::IntegerRot::add_to_virtual_table(self);
        crate::execute_integer::IntegerShove::add_to_virtual_table(self);
        crate::execute_integer::IntegerStackDepth::add_to_virtual_table(self);
        crate::execute_integer::IntegerSum::add_to_virtual_table(self);
        crate::execute_integer::IntegerSwap::add_to_virtual_table(self);
        crate::execute_integer::IntegerYankDup::add_to_virtual_table(self);
        crate::execute_integer::IntegerYank::add_to_virtual_table(self);
        crate::execute_name::NameDup::add_to_virtual_table(self);
        crate::execute_name::NameEqual::add_to_virtual_table(self);
        crate::execute_name::NameFlush::add_to_virtual_table(self);
        crate::execute_name::NamePop::add_to_virtual_table(self);
        crate::execute_name::NameQuote::add_to_virtual_table(self);
        crate::execute_name::NameRandBoundName::add_to_virtual_table(self);
        crate::execute_name::NameRand::add_to_virtual_table(self);
        crate::execute_name::NameRot::add_to_virtual_table(self);
        crate::execute_name::NameShove::add_to_virtual_table(self);
        crate::execute_name::NameStackDepth::add_to_virtual_table(self);
        crate::execute_name::NameSwap::add_to_virtual_table(self);
        crate::execute_name::NameYankDup::add_to_virtual_table(self);
        crate::execute_name::NameYank::add_to_virtual_table(self);
    }

    pub fn add_base_literals(&mut self) {
        // These must be last, with Name the very last of all. The reason is that parsing runs in order from top to bottom
        // and all the 'normal' instructions use an exact match. However the literal values use more involved parsing and
        // Name is the catch-all (anything that does not parse earlier will become a Name up to the next white-space).
        crate::execute_bool::BoolLiteralValue::add_to_virtual_table(self);
        crate::execute_float::FloatLiteralValue::add_to_virtual_table(self);
        crate::execute_integer::IntegerLiteralValue::add_to_virtual_table(self);
        crate::execute_name::NameLiteralValue::add_to_virtual_table(self);
    }
}

impl<State: std::fmt::Debug + Clone> std::fmt::Debug for VirtualTable<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut self_keys: Vec<&'static str> = self.names.keys().map(|k| *k).collect();
        self_keys.sort();

        f.debug_struct("VirtualTable").field("names", &self_keys).finish()
    }
}

impl<State: std::fmt::Debug + Clone> PartialEq for VirtualTable<State> {
    fn eq(&self, other: &Self) -> bool {
        let mut self_keys: Vec<&'static str> = self.names.keys().map(|k| *k).collect();
        let mut other_keys: Vec<&'static str> = other.names.keys().map(|k| *k).collect();

        self_keys.sort();
        other_keys.sort();

        self_keys == other_keys
    }
}
