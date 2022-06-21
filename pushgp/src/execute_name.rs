use crate::*;
use base64::*;
use byte_slice_cast::*;
use pushgp_macros::*;

pub type Name = String;

pub trait VirtualMachineMustHaveName<Vm> {
    fn name(&mut self) -> &mut Stack<Name>;
    fn should_quote_next_name(&self) -> bool;
    fn set_should_quote_next_name(&mut self, quote_next_name: bool);
    fn definition_for_name(&self, name: &String) -> Option<Code<Vm>>;
    fn define_name(&mut self, name: String, code: Code<Vm>);
    fn all_defined_names(&self) -> Vec<String>;
    fn defined_names_len(&self) -> usize;
}

pub struct NameLiteralValue {
    value: Name,
}

impl NameLiteralValue {
    pub fn new<S: ToString>(value: S) -> NameLiteralValue {
        NameLiteralValue { value: value.to_string() }
    }
}

impl StaticName for NameLiteralValue {
    fn static_name() -> &'static str {
        "NAME.LITERALVALUE"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm> + VirtualMachineMustHaveName<Vm>> StaticInstruction<Vm>
    for NameLiteralValue
{
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, value) = crate::parse::parse_code_name(input)?;
        Ok((rest, Box::new(NameLiteralValue::new(value))))
    }

    fn random_value(vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        use rand::Rng;
        let random_value = vm.get_rng().gen_range(0..=u64::MAX);

        let slice: [u64; 1] = [random_value];
        let b64 = encode(slice.as_byte_slice());
        let name = "RND.".to_owned() + &b64;
        Box::new(NameLiteralValue::new(name))
    }
}

impl std::fmt::Display for NameLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm> + VirtualMachineMustHaveName<Vm>> Instruction<Vm>
    for NameLiteralValue
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        NameLiteralValue::static_name()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(NameLiteralValue::new(self.value.clone()))
    }

    /// Executing a NameLiteralValue typically pushes the definition of a name onto the Exec stack if the Name is
    /// defined, or pushes the Name onto the Name stack if the Name is not defined yet. However the NAME.QUOTE
    /// instruction can alter this behavior by forcing the next Name to be pushed to the Name stack whether or not it
    /// already has a definition.
    fn execute(&mut self, vm: &mut Vm) {
        if vm.should_quote_next_name() {
            vm.name().push(self.value.clone());
            vm.set_should_quote_next_name(false);
        } else {
            match vm.definition_for_name(&self.value) {
                None => vm.name().push(self.value.clone()),
                Some(code) => vm.exec().push(code.clone()),
            }
        }
    }

    /// Eq for NameLiteralValue must check that the other instruction is also a NameLiteralValue and, if so, that the
    /// value is equivalent
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<NameLiteralValue>() {
            self.value == other.value
        } else {
            false
        }
    }

    /// The hash value for NameLiteralValue include the value in the hash as well as the name
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = NameLiteralValue::static_name().as_bytes().iter().map(|c| *c).collect();
        to_hash.extend_from_slice(self.value.as_bytes());
        seahash::hash(&to_hash[..])
    }
}

/// Duplicates the top item on the NAME stack. Does not pop its argument (which, if it did, would negate the effect
/// of the duplication!).
#[stack_instruction(Name)]
fn dup(context: &mut Context) {
    context.name().duplicate_top_item();
}

/// Pushes TRUE if the top two NAMEs are equal, or FALSE otherwise.
#[stack_instruction(Name)]
fn equal(context: &mut Context, a: Name, b: Name) {
    context.bool().push(a == b);
}

/// Empties the NAME stack.
#[stack_instruction(Name)]
fn flush(context: &mut Context) {
    context.name().clear()
}

/// Pops the NAME stack.
#[stack_instruction(Name)]
fn pop(context: &mut Context, _popped: Name) {}

/// Sets a flag indicating that the next name encountered will be pushed onto the NAME stack (and not have its
/// associated value pushed onto the EXEC stack), regardless of whether or not it has a definition. Upon
/// encountering such a name and pushing it onto the NAME stack the flag will be cleared (whether or not the pushed
/// name had a definition).
#[stack_instruction(Name)]
fn quote(context: &mut Context) {
    context.set_should_quote_next_name(true)
}

/// Pushes a randomly selected NAME that already has a definition.
#[stack_instruction(Name)]
fn rand_bound_name(context: &mut Context) {
    let defined_names = context.all_defined_names();
    if defined_names.len() > 0 {
        let random_value = context.run_random_function(|rng| {
            let pick: usize = rng.gen_range(0..defined_names.len());
            defined_names[pick].clone()
        });
        context.name().push(random_value);
    }
}

/// Pushes a newly generated random NAME.
#[stack_instruction(Name)]
fn rand(context: &mut Context) {
    let random_value = context.run_random_function(NameLiteralValue::random_value).unwrap();
    if let Some(stack) = context.get_stack("Name") {
        stack.push(random_value);
    }
}

/// Rotates the top three items on the NAME stack, pulling the third item out and pushing it on top. This is
/// equivalent to "2 NAME.YANK".
#[stack_instruction(Name)]
fn rot(context: &mut Context) {
    context.name().rotate();
}

/// Inserts the top NAME "deep" in the stack, at the position indexed by the top INTEGER.
#[stack_instruction(Name)]
fn shove(context: &mut Context, position: Integer) {
    if !context.name().shove(position) {
        context.integer().push(position);
    }
}

/// Pushes the stack depth onto the INTEGER stack.
#[stack_instruction(Name)]
fn stack_depth(context: &mut Context) {
    context.integer().push(context.name().len() as i64);
}

/// Swaps the top two NAMEs.
#[stack_instruction(Name)]
fn swap(context: &mut Context) {
    context.name().swap();
}

/// Pushes a copy of an indexed item "deep" in the stack onto the top of the stack, without removing the deep item.
/// The index is taken from the INTEGER stack.
#[stack_instruction(Name)]
fn yank_dup(context: &mut Context, position: Integer) {
    if !context.name().yank_duplicate(position) {
        context.integer().push(position);
    }
}

/// Removes an indexed item from "deep" in the stack and pushes it on top of the stack. The index is taken from the
/// INTEGER stack.
#[stack_instruction(Name)]
fn yank(context: &mut Context, position: Integer) {
    if !context.name().yank(position) {
        context.integer().push(position);
    }
}
