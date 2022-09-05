use crate::*;
use get_size::GetSize;
use pushgp_macros::*;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Float {
    inner: Decimal,
}

impl get_size::GetSize for Float {}
impl std::ops::Deref for Float {
    type Target = Decimal;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Decimal> for Float {
    fn from(inner: Decimal) -> Self {
        Float { inner }
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::ops::Add for Float {
    type Output = Float;
    fn add(self, rhs: Self) -> Self::Output {
        Float { inner: self.inner + rhs.inner }
    }
}

impl std::ops::Div for Float {
    type Output = Float;
    fn div(self, rhs: Self) -> Self::Output {
        Float { inner: self.inner / rhs.inner }
    }
}

impl std::ops::Mul for Float {
    type Output = Float;
    fn mul(self, rhs: Self) -> Self::Output {
        Float { inner: self.inner * rhs.inner }
    }
}

impl std::ops::Rem for Float {
    type Output = Float;
    fn rem(self, rhs: Self) -> Self::Output {
        Float { inner: self.inner % rhs.inner }
    }
}

impl std::ops::Sub for Float {
    type Output = Float;
    fn sub(self, rhs: Self) -> Self::Output {
        Float { inner: self.inner - rhs.inner }
    }
}

pub trait VirtualMachineMustHaveFloat<Vm> {
    fn float(&mut self) -> &mut Stack<Float>;
}

pub struct FloatLiteralValue {
    value: Float,
}

impl FloatLiteralValue {
    pub fn new(value: Float) -> FloatLiteralValue {
        FloatLiteralValue { value }
    }
}

impl StaticName for FloatLiteralValue {
    fn static_name() -> &'static str {
        "FLOAT.LITERALVALUE"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveFloat<Vm>> StaticInstruction<Vm> for FloatLiteralValue {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, value) = crate::parse::parse_code_float(input)?;
        Ok((rest, Box::new(FloatLiteralValue::new(Float { inner: value }))))
    }

    fn random_value(engine: &mut VirtualMachineEngine<Vm>) -> Box<dyn Instruction<Vm>> {
        use rand::Rng;
        let float: f64 = engine.get_rng().gen_range(-1f64..1f64);
        Box::new(FloatLiteralValue::new(Float { inner: Decimal::from_f64(float).unwrap() }))
    }
}

impl std::fmt::Display for FloatLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Decimals without a fractional part will parse as an integer
        if self.value.fract().is_zero() {
            write!(f, "{}.0", self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveFloat<Vm>> Instruction<Vm> for FloatLiteralValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        FloatLiteralValue::static_name()
    }

    fn size_of(&self) -> usize {
        self.value.get_size()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(FloatLiteralValue::new(self.value))
    }

    /// Executing a FloatLiteralValue pushes the literal value that was part of the data onto the stack
    fn execute(&mut self, vm: &mut Vm) {
        vm.float().push(self.value)
    }

    /// Eq for FloatLiteralValue must check that the other instruction is also a FloatLiteralValue and, if so, that the
    /// value is equivalent
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<FloatLiteralValue>() {
            self.value == other.value
        } else {
            false
        }
    }

    /// The hash value for FloatLiteralValue include the value in the hash as well as the name
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = FloatLiteralValue::static_name().as_bytes().iter().map(|c| *c).collect();
        let normalized = self.value.normalize().to_string();
        to_hash.extend_from_slice(normalized.as_bytes());
        seahash::hash(&to_hash[..])
    }
}

/// Pushes the cosine of the top item.F
#[stack_instruction(Float)]
fn cos(vm: &mut Vm, value: Float) {
    vm.float().push(Float { inner: Decimal::from_f64(value.to_f64().unwrap().cos()).unwrap() });
}

/// Defines the name on top of the NAME stack as an instruction that will push the top item of the FLOAT stack onto
/// the EXEC stack.
#[stack_instruction(Float)]
fn define(vm: &mut Vm, value: Float, name: Name) {
    vm.engine_mut().define_name(name, Box::new(FloatLiteralValue::new(value)));
}

/// Pushes the difference of the top two items; that is, the second item minus the top item.
#[stack_instruction(Float)]
fn difference(vm: &mut Vm, right: Float, left: Float) {
    vm.float().push(left - right);
}

/// Duplicates the top item on the FLOAT stack. Does not pop its argument (which, if it did, would negate the effect
/// of the duplication!).
#[stack_instruction(Float)]
fn dup(vm: &mut Vm) {
    vm.float().duplicate_top_item();
}

/// Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE otherwise.
#[stack_instruction(Float)]
fn equal(vm: &mut Vm, a: Float, b: Float) {
    vm.bool().push(a == b);
}

/// Empties the FLOAT stack.
#[stack_instruction(Float)]
fn flush(vm: &mut Vm) {
    vm.float().clear();
}

/// Pushes 1.0 if the top BOOLEAN is TRUE, or 0.0 if the top BOOLEAN is FALSE.
#[stack_instruction(Float)]
fn from_boolean(vm: &mut Vm, value: Bool) {
    vm.float().push(if value { Decimal::new(1, 0).into() } else { Decimal::new(0, 0).into() });
}

/// Pushes a floating point version of the top INTEGER.
#[stack_instruction(Float)]
fn from_integer(vm: &mut Vm, value: Integer) {
    vm.float().push(Decimal::new(value, 0).into());
}

/// Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or FALSE otherwise.
#[stack_instruction(Float)]
fn greater(vm: &mut Vm, right: Float, left: Float) {
    vm.bool().push(left > right);
}

/// Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or FALSE otherwise.
#[stack_instruction(Float)]
fn less(vm: &mut Vm, right: Float, left: Float) {
    vm.bool().push(left < right);
}

/// Pushes the maximum of the top two items.
#[stack_instruction(Float)]
fn max(vm: &mut Vm, a: Float, b: Float) {
    vm.float().push(if a > b { a } else { b });
}

/// Pushes the minimum of the top two items.
#[stack_instruction(Float)]
fn min(vm: &mut Vm, a: Float, b: Float) {
    vm.float().push(if a < b { a } else { b });
}

/// Pushes the second stack item modulo the top stack item. If the top item is zero this acts as a NOOP. The modulus
/// is computed as the remainder of the quotient, where the quotient has first been truncated toward negative
/// infinity. (This is taken from the definition for the generic MOD function in Common Lisp, which is described for
/// example at http://www.lispworks.com/reference/HyperSpec/Body/f_mod_r.htm.)
#[stack_instruction(Float)]
fn modulo(vm: &mut Vm, bottom: Float, top: Float) {
    if bottom != Decimal::ZERO.into() {
        vm.float().push(top % bottom);
    }
}

/// Pops the FLOAT stack.
#[stack_instruction(Float)]
fn pop(vm: &mut Vm, _popped: Float) {}

/// Pushes the product of the top two items.
#[stack_instruction(Float)]
fn product(vm: &mut Vm, right: Float, left: Float) {
    vm.float().push(left * right);
}

/// Pushes the quotient of the top two items; that is, the second item divided by the top item. If the top item is
/// zero this acts as a NOOP.
#[stack_instruction(Float)]
fn quotient(vm: &mut Vm, bottom: Float, top: Float) {
    if bottom != Decimal::ZERO.into() {
        vm.float().push(top / bottom);
    }
}

/// Pushes a newly generated random FLOAT that is greater than or equal to MIN-RANDOM-FLOAT and less than or equal
/// to MAX-RANDOM-FLOAT.
#[stack_instruction(Float)]
fn rand(vm: &mut Vm) {
    let mut random_value = FloatLiteralValue::random_value(vm.engine_mut());
    random_value.execute(vm);
}

/// Rotates the top three items on the FLOAT stack, pulling the third item out and pushing it on top. This is
/// equivalent to "2 FLOAT.YANK".
#[stack_instruction(Float)]
fn rot(vm: &mut Vm) {
    vm.float().rotate();
}

/// Inserts the top FLOAT "deep" in the stack, at the position indexed by the top INTEGER.
#[stack_instruction(Float)]
fn shove(vm: &mut Vm, position: Integer) {
    if !vm.float().shove(position) {
        vm.integer().push(position);
    }
}

/// Pushes the sine of the top item.
#[stack_instruction(Float)]
fn sin(vm: &mut Vm, value: Float) {
    vm.float().push(Decimal::from_f64(value.to_f64().unwrap().sin()).unwrap().into());
}

/// Pushes the stack depth onto the INTEGER stack.
#[stack_instruction(Float)]
fn stack_depth(vm: &mut Vm) {
    let len = vm.float().len() as i64;
    vm.integer().push(len);
}

/// Pushes the sum of the top two items.
#[stack_instruction(Float)]
fn sum(vm: &mut Vm, right: Float, left: Float) {
    vm.float().push(left + right);
}

/// Swaps the top two BOOLEANs.
#[stack_instruction(Float)]
fn swap(vm: &mut Vm) {
    vm.float().swap();
}

/// Pushes the tangent of the top item.
#[stack_instruction(Float)]
fn tan(vm: &mut Vm, value: Float) {
    vm.float().push(Decimal::from_f64(value.to_f64().unwrap().tan()).unwrap().into());
}

/// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
/// The index is taken from the INTEGER stack.
#[stack_instruction(Float)]
fn yank_dup(vm: &mut Vm, position: Integer) {
    if !vm.float().yank_duplicate(position) {
        vm.integer().push(position);
    }
}

/// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
/// INTEGER stack.
#[stack_instruction(Float)]
fn yank(vm: &mut Vm, position: Integer) {
    if !vm.float().yank(position) {
        vm.integer().push(position);
    }
}
