use crate::*;
use pushgp_macros::*;

pub type Bool = bool;

pub trait VirtualMachineMustHaveBool<Vm> {
    fn bool(&mut self) -> &mut Stack<Bool>;
}

struct BoolLiteralValue {
    value: bool,
}

impl BoolLiteralValue {
    fn new(value: bool) -> BoolLiteralValue {
        BoolLiteralValue { value }
    }
}

impl StaticName for BoolLiteralValue {
    fn static_name() -> &'static str {
        "BOOL.LITERALVALUE"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBool<Vm>> StaticInstruction<Vm> for BoolLiteralValue {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, value) = crate::parse::parse_code_bool(input)?;
        Ok((rest, Box::new(BoolLiteralValue::new(value))))
    }

    fn random_value(vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        use rand::Rng;
        Box::new(BoolLiteralValue::new(if 0 == vm.get_rng().gen_range(0..=1) { false } else { true }))
    }
}

impl std::fmt::Display for BoolLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "TRUE" } else { "FALSE" })
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBool<Vm>> Instruction<Vm> for BoolLiteralValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        BoolLiteralValue::static_name()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(BoolLiteralValue::new(self.value))
    }

    /// Executing a BoolLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(&self, vm: &mut Vm) {
        vm.bool().push(self.value)
    }

    /// Eq for BoolLiteralValue must check that the other instruction is also a BoolLiteralValue and, if so, that the
    /// value is equivalent
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<BoolLiteralValue>() {
            self.value == other.value
        } else {
            false
        }
    }

    /// The hash value for BoolLiteralValue include the value in the hash as well as the name
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = BoolLiteralValue::static_name().as_bytes().iter().map(|c| *c).collect();
        to_hash.push(if self.value { 1 } else { 0 });
        seahash::hash(&to_hash[..])
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
    vm.define_name(name, vm.make_literal_bool(value));
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
    let random_bool = vm.run_random_function(BoolLiteralValue::random_value).unwrap();
    if let Some(stack) = vm.get_stack("Bool") {
        stack.push(random_bool);
    }
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
    vm.integer().push(vm.bool().len() as i64);
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
