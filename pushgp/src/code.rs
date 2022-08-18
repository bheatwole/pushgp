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

impl<Vm: 'static> std::cmp::PartialEq for Extraction<Vm> {
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
    vm.engine_mut().add_instruction::<crate::list::PushList<Vm>>();
    vm.engine_mut().add_instruction::<crate::execute_bool::BoolLiteralValue>();
    vm.engine_mut().add_instruction::<crate::execute_float::FloatLiteralValue>();
    vm.engine_mut().add_instruction::<crate::execute_integer::IntegerLiteralValue>();
    vm.engine_mut().add_instruction::<crate::execute_name::NameLiteralValue>();
}
