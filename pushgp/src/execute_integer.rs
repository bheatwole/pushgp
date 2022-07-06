use crate::*;
use pushgp_macros::*;
use rust_decimal::prelude::ToPrimitive;

pub type Integer = i64;

pub trait VirtualMachineMustHaveInteger<Vm> {
    fn integer(&mut self) -> &mut Stack<Integer>;
}

pub struct IntegerLiteralValue {
    value: Integer,
}

impl IntegerLiteralValue {
    pub fn new(value: Integer) -> IntegerLiteralValue {
        IntegerLiteralValue { value }
    }
}

impl StaticName for IntegerLiteralValue {
    fn static_name() -> &'static str {
        "INTEGER.LITERALVALUE"
    }
}

impl<Vm: VirtualMachine + 'static + VirtualMachineMustHaveInteger<Vm>> StaticInstruction<Vm> for IntegerLiteralValue {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, value) = crate::parse::parse_code_integer(input)?;
        Ok((rest, Box::new(IntegerLiteralValue::new(value))))
    }

    fn random_value(vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        use rand::Rng;
        let value: i64 = vm.get_rng().gen_range(i64::MIN..=i64::MAX);
        Box::new(IntegerLiteralValue::new(value))
    }
}

impl std::fmt::Display for IntegerLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<Vm: VirtualMachine + 'static + VirtualMachineMustHaveInteger<Vm>> Instruction<Vm> for IntegerLiteralValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        IntegerLiteralValue::static_name()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(IntegerLiteralValue::new(self.value))
    }

    /// Executing a IntegerLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(&mut self, vm: &mut Vm) {
        vm.integer().push(self.value)
    }

    /// Eq for IntegerLiteralValue must check that the other instruction is also a IntegerLiteralValue and, if so, that the
    /// value is equivalent
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<IntegerLiteralValue>() {
            self.value == other.value
        } else {
            false
        }
    }

    /// The hash value for IntegerLiteralValue include the value in the hash as well as the name
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = IntegerLiteralValue::static_name().as_bytes().iter().map(|c| *c).collect();
        let normalized = self.value.to_string();
        to_hash.extend_from_slice(normalized.as_bytes());
        seahash::hash(&to_hash[..])
    }
}

/// Defines the name on top of the NAME stack as an instruction that will push the top item of the INTEGER stack
/// onto the EXEC stack.
#[stack_instruction(Integer)]
fn define(vm: &mut Vm, value: Integer, name: Name) {
    vm.name().define_name(name, Box::new(IntegerLiteralValue::new(value)));
}

/// Pushes the difference of the top two items; that is, the second item minus the top item.
#[stack_instruction(Integer)]
fn difference(vm: &mut Vm, right: Integer, left: Integer) {
    vm.integer().push(left - right);
}

/// Duplicates the top item on the INTEGER stack. Does not pop its argument (which, if it did, would negate the
/// effect of the duplication!).
#[stack_instruction(Integer)]
fn dup(vm: &mut Vm) {
    vm.integer().duplicate_top_item();
}

/// Pushes TRUE if the top two items on the INTEGER stack are equal, or FALSE otherwise.
#[stack_instruction(Integer)]
fn equal(vm: &mut Vm, a: Integer, b: Integer) {
    vm.bool().push(a == b);
}

/// Empties the INTEGER stack.
#[stack_instruction(Integer)]
fn flush(vm: &mut Vm) {
    vm.integer().clear();
}

/// Pushes 1 if the top BOOLEAN is TRUE, or 0 if the top BOOLEAN is FALSE.
#[stack_instruction(Integer)]
fn from_boolean(vm: &mut Vm, value: Bool) {
    vm.integer().push(if value { 1 } else { 0 });
}

/// Pushes the result of truncating the top FLOAT.
#[stack_instruction(Integer)]
fn from_float(vm: &mut Vm, value: Float) {
    vm.integer().push(value.to_i64().unwrap());
}

/// Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or FALSE otherwise.
#[stack_instruction(Integer)]
fn greater(vm: &mut Vm, right: Integer, left: Integer) {
    vm.bool().push(left > right);
}

/// Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or FALSE otherwise.
#[stack_instruction(Integer)]
fn less(vm: &mut Vm, right: Integer, left: Integer) {
    vm.bool().push(left < right);
}

/// Pushes the maximum of the top two items.
#[stack_instruction(Integer)]
fn max(vm: &mut Vm, a: Integer, b: Integer) {
    vm.integer().push(if a > b { a } else { b });
}

/// Pushes the minimum of the top two items.
#[stack_instruction(Integer)]
fn min(vm: &mut Vm, a: Integer, b: Integer) {
    vm.integer().push(if a < b { a } else { b });
}

/// Pushes the second stack item modulo the top stack item. If the top item is zero this acts as a NOOP. The modulus
/// is computed as the remainder of the quotient, where the quotient has first been truncated toward negative
/// infinity. (This is taken from the definition for the generic MOD function in Common Lisp, which is described for
/// example at http://www.lispworks.com/reference/HyperSpec/Body/f_mod_r.htm.)
#[stack_instruction(Integer)]
fn modulo(vm: &mut Vm, bottom: Integer, top: Integer) {
    if bottom != 0 {
        vm.integer().push(top % bottom);
    }
}

/// Pops the INTEGER stack.
#[stack_instruction(Integer)]
fn pop(vm: &mut Vm, _popped: Integer) {
}

/// Pushes the product of the top two items.
#[stack_instruction(Integer)]
fn product(vm: &mut Vm, right: Integer, left: Integer) {
    vm.integer().push(left * right);
}

/// Pushes the quotient of the top two items; that is, the second item divided by the top item. If the top item is
/// zero this acts as a NOOP.
#[stack_instruction(Integer)]
fn quotient(vm: &mut Vm, bottom: Integer, top: Integer) {
    if bottom != 0 {
        vm.integer().push(top / bottom);
    }
}

/// Pushes a newly generated random INTEGER that is greater than or equal to MIN-RANDOM-INTEGER and less than or
/// equal to MAX-RANDOM-INTEGER.
#[stack_instruction(Integer)]
fn rand(vm: &mut Vm) {
    let mut random_value = IntegerLiteralValue::random_value(vm);
    random_value.execute(vm);
}

/// Rotates the top three items on the INTEGER stack, pulling the third item out and pushing it on top. This is
/// equivalent to "2 INTEGER.YANK".
#[stack_instruction(Integer)]
fn rot(vm: &mut Vm) {
    vm.integer().rotate()
}

/// Inserts the second INTEGER "deep" in the stack, at the position indexed by the top INTEGER. The index position
/// is calculated after the index is removed.
#[stack_instruction(Integer)]
fn shove(vm: &mut Vm, position: Integer) {
    if !vm.integer().shove(position) {
        vm.integer().push(position);
    }
}

/// Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
#[stack_instruction(Integer)]
fn stack_depth(vm: &mut Vm) {
    let len = vm.integer().len() as i64;
    vm.integer().push(len);
}

/// Pushes the sum of the top two items.
#[stack_instruction(Integer)]
fn sum(vm: &mut Vm, a: Integer, b: Integer) {
    vm.integer().push(a + b);
}

/// Swaps the top two INTEGERs.
#[stack_instruction(Integer)]
fn swap(vm: &mut Vm) {
    vm.integer().swap();
}

/// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
/// The index is taken from the INTEGER stack, and the indexing is done after the index is removed.
#[stack_instruction(Integer)]
fn yank_dup(vm: &mut Vm, position: Integer) {
    if !vm.integer().yank_duplicate(position) {
        vm.integer().push(position);
    }
}

/// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
/// INTEGER stack, and the indexing is done after the index is removed.
#[stack_instruction(Integer)]
fn yank(vm: &mut Vm, position: Integer) {
    if !vm.integer().yank(position) {
        vm.integer().push(position);
    }
}
