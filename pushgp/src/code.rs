use crate::{
    Instruction, VirtualMachine, VirtualMachineMustHaveBool, VirtualMachineMustHaveCode, VirtualMachineMustHaveExec,
    VirtualMachineMustHaveFloat, VirtualMachineMustHaveInteger, VirtualMachineMustHaveName,
};

// An extraction can either return a piece of code or the number of points used
#[derive(Debug)]
pub enum Extraction<Vm> {
    Extracted(Box<dyn Instruction<Vm>>),
    Used(i64),
}

impl<Vm> std::cmp::PartialEq for Extraction<Vm> {
    fn eq(&self, other: &Extraction<Vm>) -> bool {
        match self {
            Extraction::Extracted(self_code) => match other {
                Extraction::Extracted(other_code) => self_code.eq(other_code),
                _ => false,
            },
            Extraction::Used(self_points) => match other {
                Extraction::Used(other_points) => self_points == other_points,
                _ => false,
            },
        }
    }
}

pub fn add_base_instructions<
    Vm: VirtualMachine + 'static
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
    vm.add_instruction::<crate::execute_code::CodeRandNoName>();
    vm.add_instruction::<crate::execute_code::CodeRot>();
    vm.add_instruction::<crate::execute_code::CodeSelectGeneticOperation>();
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
