use crate::*;
use rand::prelude::SeedableRng;
use rand::rngs::SmallRng;

pub trait VirtualMachine {

    /// All virtual machines must expose a random number generator.
    fn get_rng(&mut self) -> &mut SmallRng;

    /// Various algorithms need to reliably repeat random number generation.
    fn set_rng_seed(&mut self, seed: Option<u64>);

    /// Clears all the stacks and defined names
    fn clear(&mut self);

    fn run(&mut self, max: usize) -> usize;
    fn next(&mut self) -> Option<usize>;

    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Box<dyn Instruction<Self>>>;
}


pub struct BaseVm {
    rng: SmallRng,
    exec_stack: Stack<Exec<BaseVm>>,
    bool_stack: Stack<Bool>,
    code_stack: Stack<Code<BaseVm>>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: Stack<String>,
    parser: Parser<BaseVm>,
    quote_next_name: bool,
    defined_names: fnv::FnvHashMap<String, Code<BaseVm>>,
}

impl BaseVm {
    fn new() -> BaseVm {

        let mut vm = BaseVm {
            rng: small_rng_from_optional_seed(None),
            exec_stack: Stack::new(),
            bool_stack: Stack::new(),
            code_stack: Stack::new(),
            float_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: Stack::new(),
            parser: Parser::new(),
            quote_next_name: false,
            defined_names: fnv::FnvHashMap::default(),
        };

        vm.parser.add_instruction::<VmBoolAnd>();
        vm.parser.add_instruction::<VmBoolDefine>();
        vm.parser.add_instruction::<VmBoolLiteralValue>();
        vm.parser.add_instruction::<VmBoolRand>();

        vm
    }
}

impl VirtualMachine for BaseVm {
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        &mut self.rng
    }

    fn set_rng_seed(&mut self, seed: Option<u64>) {
        self.rng = small_rng_from_optional_seed(seed);
    }

    fn clear(&mut self) {
        self.exec_stack.clear();
        self.bool_stack.clear();
        self.code_stack.clear();
        self.float_stack.clear();
        self.integer_stack.clear();
        self.name_stack.clear();
        self.quote_next_name = false;
        self.defined_names.clear();
    }

    fn run(&mut self, max: usize) -> usize {
        // trace!("{:?}", self);
        let mut total_count = 0;
        while let Some(count) = self.next() {
            total_count += count;
            if total_count >= max {
                break;
            }
        }
        total_count
    }

    fn next(&mut self) -> Option<usize> {
        // Pop the top piece of code from the exec stack and execute it.
        if let Some(exec) = self.exec_stack.pop() {
            exec.execute(self);

            // Return the number of points required to perform that action
            return Some(1);
        }

        // No action was found
        None
    }

    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Box<dyn Instruction<BaseVm>>> {
        self.parser.parse(input)
    }
}

fn small_rng_from_optional_seed(rng_seed: Option<u64>) -> SmallRng {
    if let Some(seed) = rng_seed {
        SmallRng::seed_from_u64(seed)
    } else {
        SmallRng::from_entropy()
    }
}

pub trait VirtualMachineMustHaveExecStack<Vm> {
    fn exec(&mut self) -> &mut Stack<Box<dyn Instruction<Vm>>>;
}

impl VirtualMachineMustHaveExecStack<BaseVm> for BaseVm {
    fn exec(&mut self) -> &mut Stack<Box<dyn Instruction<BaseVm>>> {
        &mut self.exec_stack
    }
}

pub trait VirtualMachineMustHaveNameStack {
    fn name(&mut self) -> &mut Stack<String>;
    fn should_quote_next_name(&self) -> bool;
    fn set_should_quote_next_name(&mut self, quote_next_name: bool);
    fn definition_for_name(&self, name: &String) -> Option<Box<dyn Instruction<BaseVm>>>;
    fn define_name(&mut self, name: String, code: Box<dyn Instruction<BaseVm>>);
    fn all_defined_names(&self) -> Vec<String>;
    fn defined_names_len(&self) -> usize;
}

impl VirtualMachineMustHaveNameStack for BaseVm {
    fn name(&mut self) -> &mut Stack<String> {
        &mut self.name_stack
    }

    /// Returns true if the next Name encountered on the Exec stack should be pushed to the Name stack instead of
    /// possibly running the Code associated with the Name.
    fn should_quote_next_name(&self) -> bool {
        self.quote_next_name
    }

    /// Sets whether or not the next Name encountered on the Exec stack should be pushed to the Name stack instead of
    /// possibly running the Code associated with the Name. Uses interior mutability.
    fn set_should_quote_next_name(&mut self, quote_next_name: bool) {
        self.quote_next_name = quote_next_name;
    }

    /// Returns the Code defined with the specified Name or None
    fn definition_for_name(&self, name: &String) -> Option<Box<dyn Instruction<BaseVm>>> {
        self.defined_names.get(name).map(|c| c.clone())
    }

    /// Defines the Code that will be placed on the top of the Exec stack when the specified Name is encountered. If the
    /// name was previously defined, the new definition replaces the old value.
    fn define_name(&mut self, name: String, code: Box<dyn Instruction<BaseVm>>) {
        self.defined_names.insert(name, code);
    }

    /// Returns a list of all previously defined names. May be empty if no names have been defined
    fn all_defined_names(&self) -> Vec<String> {
        self.defined_names.keys().map(|k| k.clone()).collect()
    }

    /// Returns the number of previously defined names. Avoids an expensive copy of all names if only the count is
    /// needed.
    fn defined_names_len(&self) -> usize {
        self.defined_names.len()
    }
}

pub trait VirtualMachineMustHaveBoolStack {
    fn bool(&mut self) -> &mut Stack<bool>;
}

impl VirtualMachineMustHaveBoolStack for BaseVm {
    fn bool(&mut self) -> &mut Stack<bool> {
        &mut self.bool_stack
    }
}

impl<Vm> VirtualMachineMustHaveBool<Vm> for BaseVm {
    fn bool(&mut self) -> &mut Stack<Bool> {
        &mut self.bool_stack
    }
}

struct VmBoolAnd {}

impl StaticName for VmBoolAnd {
    fn static_name() -> &'static str {
        "BOOL.AND"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBoolStack> StaticInstruction<Vm> for VmBoolAnd {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, _) = nom::bytes::complete::tag(VmBoolAnd::static_name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, Box::new(VmBoolAnd {})))
    }

    fn random_value(_vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolAnd {})
    }
}

impl std::fmt::Display for VmBoolAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(VmBoolAnd::static_name())
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack> Instruction<Vm> for VmBoolAnd {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        VmBoolAnd::static_name()
    }


    /// The instruction must be able to clone itself, though we cannot implement the normal 'Clone' trait or the
    /// Instruction could not become a trait object
    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolAnd{})
    }

    /// The instruction must be able to execute on a virtual machine. The instruction must never panic and may only
    /// update the state of the virtual machine
    fn execute(&self, vm: &mut Vm) {
        if vm.bool().len() >= 2 {
            let a = vm.bool().pop().unwrap();
            let b = vm.bool().pop().unwrap();
            vm.bool().push(a && b);
        }
    }

}


struct VmBoolDefine {}

impl StaticName for VmBoolDefine {
    fn static_name() -> &'static str {
        "BOOL.DEFINE"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveNameStack> StaticInstruction<Vm> for VmBoolDefine {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, _) = nom::bytes::complete::tag(Self::static_name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, Box::new(VmBoolDefine {})))
    }

    fn random_value(_vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolDefine {})
    }
}

impl std::fmt::Display for VmBoolDefine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(VmBoolDefine::static_name())
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveNameStack> Instruction<Vm> for VmBoolDefine {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        VmBoolDefine::static_name()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolDefine{})
    }

    fn execute(&self, vm: &mut Vm) {
        if vm.bool().len() >= 1 && vm.name().len() >= 1 {
            let name = vm.name().pop().unwrap();
            let value = vm.bool().pop().unwrap();
            vm.define_name(name, Box::new(VmBoolLiteralValue::new(value)))
        }
    }
}


struct VmBoolLiteralValue {
    value: bool
}

impl VmBoolLiteralValue {
    fn new(value: bool) -> VmBoolLiteralValue {
        VmBoolLiteralValue {
            value
        }
    }
}

impl StaticName for VmBoolLiteralValue {
    fn static_name() -> &'static str {
        "BOOL.LITERALVALUE"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBoolStack> StaticInstruction<Vm> for VmBoolLiteralValue {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, value) = crate::parse::parse_code_bool(input)?;
        Ok((rest, Box::new(VmBoolLiteralValue::new(value))))
    }

    fn random_value(vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        use rand::Rng;
        Box::new(VmBoolLiteralValue::new(if 0 == vm.get_rng().gen_range(0..=1) { false } else { true }))
    }
}

impl std::fmt::Display for VmBoolLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "TRUE" } else { "FALSE" })
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack> Instruction<Vm> for VmBoolLiteralValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        VmBoolLiteralValue::static_name()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolLiteralValue::new(self.value))
    }

    fn execute(&self, vm: &mut Vm) {
        vm.bool().push(self.value)
    }

    /// An instruction must be able to determine if it is equal to another instruction. We cannot simply use PartialEq
    /// and Eq because then Instruction could not become a trait object.
    ///
    /// The default implementation simply checks to see if the static name string points to the same location in memory.
    /// Instructions that have data MUST override this behavior to check for data equivalence.
    fn eq(&self, other: &dyn Instruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<VmBoolLiteralValue>() {
            self.value == other.value
        } else {
            false
        }
    }

    /// An instruction must be able to provide a unique hash of itself and its data. The hash() of two instructions that
    /// are eq() to each other should also be equal.
    ///
    /// We cannot simply use Hash because then Instruction could not become a trait object (no methods with generics).
    ///
    /// The default implementation uses the sea-hash of the name. Instructions that have data MUST override this
    /// behavior to include the data in the hash.
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = VmBoolLiteralValue::static_name().as_bytes().iter().map(|c| *c).collect();
        to_hash.push(if self.value { 1 } else { 0 });
        seahash::hash(&to_hash[..])
    }
}


struct VmBoolRand {}

impl StaticName for VmBoolRand {
    fn static_name() -> &'static str {
        "BOOL.RAND"
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveExecStack<Vm>> StaticInstruction<Vm> for VmBoolRand {

    fn parse(input: &str) -> nom::IResult<&str, Box<dyn Instruction<Vm>>> {
        let (rest, _) = nom::bytes::complete::tag(Self::static_name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, Box::new(VmBoolRand {})))
    }

    fn random_value(vm: &mut Vm) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolRand {})
    }
}

impl std::fmt::Display for VmBoolRand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(VmBoolRand::static_name())
    }
}

impl<Vm: VirtualMachine + VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveExecStack<Vm>> Instruction<Vm> for VmBoolRand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn name(&self) -> &'static str {
        VmBoolRand::static_name()
    }

    fn clone(&self) -> Box<dyn Instruction<Vm>> {
        Box::new(VmBoolRand{})
    }

    fn execute(&self, vm: &mut Vm) {
        let rand = VmBoolLiteralValue::random_value(vm);
        vm.exec().push(rand)
    }
}