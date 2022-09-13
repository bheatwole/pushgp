use crate::*;
use pushgp_macros::*;

pub type Bool = bool;

pub trait VirtualMachineMustHaveBool<Vm> {
    fn bool(&mut self) -> &mut Stack<Bool>;
}

pub struct BoolLiteralValue {}

impl StaticName for BoolLiteralValue {
    fn static_name() -> &'static str {
        "BOOL.LITERALVALUE"
    }
}

impl BoolLiteralValue {
    pub fn new_code<Oc: OpcodeConvertor>(oc: &Oc, value: Bool) -> Code {
        let opcode = oc.opcode_for_name(Self::static_name()).unwrap();
        Code::new(opcode, value.into())
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBool<Vm>> Instruction<Vm> for BoolLiteralValue {
    fn parse<'a>(input: &'a str, opcode: u32) -> nom::IResult<&'a str, Code> {
        let (rest, value) = crate::parse::parse_code_bool(input)?;
        Ok((rest, Code::new(opcode, value.into())))
    }

    fn fmt(f: &mut std::fmt::Formatter<'_>, code: &Code, _vtable: &InstructionTable<Vm>) -> std::fmt::Result {
        if let Some(value) = code.get_data().bool_value() {
            write!(f, "{}", if value { "TRUE" } else { "FALSE" })
        } else {
            panic!("fmt called for BoolLiteralValue with Code that does not have a boolean value stored")
        }
    }

    fn random_value(engine: &mut VirtualMachineEngine<Vm>) -> Code {
        use rand::Rng;
        let value = if 0 == engine.get_rng().gen_range(0..=1) { false } else { true };
        BoolLiteralValue::new_code(engine, value)
    }

    /// Executing a BoolLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(code: Code, vm: &mut Vm) {
        if let Some(value) = code.get_data().bool_value() {
            vm.bool().push(value);
        }
    }
}

/// Pushes the logical AND of the top two BOOLEANs onto the EXEC stack
#[stack_instruction(Bool)]
fn and(vm: &mut Vm, a: Bool, b: Bool) {
    vm.bool().push(a && b);
}

/// Defines the name on top of the NAME stack as an instruction that will push the top item of the BOOLEAN stack
#[stack_instruction(Bool)]
fn define(vm: &mut Vm, value: Bool, name: Name) {
    let code = BoolLiteralValue::new_code(vm, value);
    vm.engine_mut().define_name(name, code);
}

/// Duplicates the top item on the BOOLEAN stack. Does not pop its argument (which, if it did, would negate the
/// effect of the duplication!)
#[stack_instruction(Bool)]
fn dup(vm: &mut Vm) {
    vm.bool().duplicate_top_item();
}

/// Pushes TRUE if the top two BOOLEANs are equal, or FALSE otherwise
#[stack_instruction(Bool)]
fn equal(vm: &mut Vm, a: Bool, b: Bool) {
    vm.bool().push(a == b);
}

/// Empties the BOOLEAN stack
#[stack_instruction(Bool)]
fn flush(vm: &mut Vm) {
    vm.bool().clear();
}

/// Pushes FALSE if the top FLOAT is 0.0, or TRUE otherwise
#[stack_instruction(Bool)]
fn from_float(vm: &mut Vm, f: Float) {
    vm.bool().push(!f.is_zero());
}

/// Pushes FALSE if the top INTEGER is 0, or TRUE otherwise
#[stack_instruction(Bool)]
fn from_int(vm: &mut Vm, i: Integer) {
    vm.bool().push(i != 0);
}

/// Pushes the logical NOT of the top BOOLEAN
#[stack_instruction(Bool)]
fn not(vm: &mut Vm, b: Bool) {
    vm.bool().push(!b);
}

/// Pushes the logical OR of the top two BOOLEANs
#[stack_instruction(Bool)]
fn or(vm: &mut Vm, a: Bool, b: Bool) {
    vm.bool().push(a || b);
}

/// Pops the BOOLEAN stack
#[stack_instruction(Bool)]
fn pop(vm: &mut Vm, _a: Bool) {}

/// Pushes a random BOOLEAN
#[stack_instruction(Bool)]
fn rand(vm: &mut Vm) {
    let random_value = vm.random_value::<BoolLiteralValue>();
    vm.execute_immediate::<BoolLiteralValue>(random_value);
}

/// Rotates the top three items on the BOOLEAN stack, pulling the third item out and pushing it on top. This is
/// equivalent to "2 BOOLEAN.YANK"
#[stack_instruction(Bool)]
fn rot(vm: &mut Vm) {
    vm.bool().rotate();
}

/// Inserts the top BOOLEAN "deep" in the stack, at the position indexed by the top INTEGER
#[stack_instruction(Bool)]
fn shove(vm: &mut Vm, position: Integer) {
    if !vm.bool().shove(position) {
        vm.integer().push(position);
    }
}

/// Pushes the stack depth onto the INTEGER stack
#[stack_instruction(Bool)]
fn stack_depth(vm: &mut Vm) {
    let len = vm.bool().len() as i64;
    vm.integer().push(len);
}

/// Swaps the top two BOOLEANs
#[stack_instruction(Bool)]
fn swap(vm: &mut Vm) {
    vm.bool().swap();
}

/// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
/// The index is taken from the INTEGER stack
#[stack_instruction(Bool)]
fn yank_dup(vm: &mut Vm, position: Integer) {
    if !vm.bool().yank_duplicate(position) {
        vm.integer().push(position);
    }
}

/// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from theF
/// INTEGER stack
#[stack_instruction(Bool)]
fn yank(vm: &mut Vm, position: Integer) {
    if !vm.bool().yank(position) {
        vm.integer().push(position);
    }
}
