use crate::util::stack_to_vec;

trait VmNamed {
    /// All Code must have a name that is known at compile-time
    fn name() -> &'static str;
}

trait VmCode<Vm> {
    /// All Code must be parsable by 'nom' from a string.
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn VmInstruction<Vm>>>;

    /// All Code must be able to create a new 'random' value. For pure instructions that have no data, the 'random'
    /// value is always the same: the instruction. For instructions that do have data (BOOL.LITERALVALUE,
    /// INTEGER.LITERALVALUE, CODE.CODE, etc.), the instruction created will be random
    fn random_value(rng: &mut rand::rngs::SmallRng) -> Box<dyn VmInstruction<Vm>>;
}

/// The VmInstruction is a trait that allows use as a trait object. This significantly restricts what kinds of methods
/// we can include in this trait.
///
/// It is generic for a VirtualMachine. Most instructions will place additional `where` constraints on the VM. I.E. an
/// instruction may require the VM to implement VirtualMachineHasBoolStack, VirtualMachineHasCodeStack and
/// VirtualMachineHasGameState. (VirtualMachineHasGameState being a trait defined in the user's code)
pub trait VmInstruction<Vm>: std::any::Any + std::fmt::Display {
    fn as_any(&self) -> &dyn std::any::Any;

    /// Every instruction must have a name that is known at compile-time
    fn self_name(&self) -> &'static str;

    /// The instruction must be able to clone itself, though we cannot implement the normal 'Clone' trait or the
    /// VmInstruction could not become a trait object
    fn clone(&self) -> Box<dyn VmInstruction<Vm>>;

    /// The instruction must be able to execute on a virtual machine. The instruction must never panic and may only
    /// update the state of the virtual machine
    fn execute(&self, vm: &mut Vm);

    /// An instruction must be able to determine if it is equal to another instruction. We cannot simply use PartialEq
    /// and Eq because then VmInstruction could not become a trait object.
    ///
    /// The default implementation simply checks to see if the static name string points to the same location in memory.
    /// Instructions that have data MUST override this behavior to check for data equivalence.
    fn eq(&self, other: &dyn VmInstruction<Vm>) -> bool {
        std::ptr::eq(self.self_name(), other.self_name())
    }

    /// An instruction must be able to provide a unique hash of itself and its data. The hash() of two instructions that
    /// are eq() to each other should also be equal.
    ///
    /// We cannot simply use Hash because then VmInstruction could not become a trait object (no methods with generics).
    ///
    /// The default implementation uses the sea-hash of the name. Instructions that have data MUST override this
    /// behavior to include the data in the hash.
    fn hash(&self) -> u64 {
        seahash::hash(self.self_name().as_bytes())
    }

    /// Returns true if this code is a List
    fn is_list(&self) -> bool {
        false
    }

    /// Returns true if this code is anything BUT a list
    fn is_atom(&self) -> bool {
        true
    }

    /// Returns true if the specified code is equal to this item or any child
    fn contains(&self, look_for: &dyn VmInstruction<Vm>) -> bool {
        self.eq(look_for)
    }

    /// Returns the smallest sub-list that contains the specified code
    fn container(&self, _look_for: &dyn VmInstruction<Vm>) -> Option<Box<dyn VmInstruction<Vm>>> {
        None
    }

    /// Similar to `contains` but does not recurse into Lists
    fn has_member(&self, look_for: &dyn VmInstruction<Vm>) -> bool {
        self.eq(look_for)
    }

    /// Similar to `extract_point` but does not recurse into lists
    fn position_of(&self, look_for: &dyn VmInstruction<Vm>) -> Option<usize> {
        if self.eq(look_for) {
            Some(0)
        } else {
            None
        }
    }

    /// The discrepancy output is a HashMap of every unique sub-list and atom from the specified code
    fn discrepancy_items(&self) -> fnv::FnvHashMap<u64, i64> {
        let mut items = fnv::FnvHashMap::default();
        let counter = items.entry(self.hash()).or_insert(0);
        *counter += 1;

        items
    }

    /// Coerces the item to a list
    fn to_list(&self) -> Vec<Box<dyn VmInstruction<Vm>>> {
        vec![self.clone()]
    }

    /// Returns the number of 'points' of the entire code. Each atom and list is considered one point.
    fn points(&self) -> i64 {
        1
    }

    /// Returns the item of code at the specified 'point' in the code tree if `point` is less than the number of points
    /// in the code. Returns the number of points used otherwise.
    fn extract_point(&self, point: i64) -> Extraction<Vm> {
        if 0 == point {
            return Extraction::Extracted(self.clone());
        }
        Extraction::Used(1)
    }

    /// Descends to the specified point in the code tree and swaps the list or atom there with the specified replacement
    /// code. If the replacement point is greater than the number of points in the Code, this has no effect.
    fn replace_point(&self, point: i64, replace_with: &dyn VmInstruction<Vm>) -> (Box<dyn VmInstruction<Vm>>, i64) {
        // If this is the replacement point, return the replacement
        if 0 == point {
            (replace_with.clone(), 1)
        } else {
            // Atoms are returned as-is
            (self.clone(), 1)
        }
    }

    /// Returns the number of items in this list. Unlike 'points' it does not recurse into sub-lists
    fn len(&self) -> usize {
        1
    }

    /// Replaces the specified search code with the specified replacement code
    fn replace(
        &self,
        look_for: &dyn VmInstruction<Vm>,
        replace_with: &dyn VmInstruction<Vm>,
    ) -> Box<dyn VmInstruction<Vm>> {
        if self.eq(look_for) {
            return replace_with.clone();
        }
        self.clone()
    }
}

/// We CAN implement Clone for the boxed instruction, which allows us to put the Box<dyn Instruction> into a
/// VmStack<T: Clone>
impl<Vm> Clone for Box<dyn VmInstruction<Vm>> {
    fn clone(&self) -> Self {
        self.as_ref().clone()
    }
}

fn compile_test<Vm>(_test: &dyn VmInstruction<Vm>) {}

// An extraction can either return an instruction or the number of points used
pub enum Extraction<Vm> {
    Extracted(Box<dyn VmInstruction<Vm>>),
    Used(i64),
}

pub struct VmStack<T: Clone> {
    stack: Vec<T>,
}

// NOTE! Every Stack has a unique type, but not every stack has a Literal. For example, the Exec stack has the type
// of Exec which is an alias of Code, but neither Exec or Code have a literal.

impl<T: Clone> VmStack<T> {
    pub fn new() -> VmStack<T> {
        VmStack { stack: vec![] }
    }

    pub fn new_from_vec(stack: Vec<T>) -> VmStack<T> {
        VmStack { stack }
    }

    /// Returns the top item from the Stack or None if the stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    /// Pushes the specified item onto the top of the stack
    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    /// Returns the length of the Stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Duplicates the top item of the stack. This should not change the Stack or panic if the stack is empty
    pub fn duplicate_top_item(&mut self) {
        let mut duplicate = None;

        // This patten avoid mutable and immutable borrow of stack at the same time
        if let Some(top_item) = self.stack.last() {
            duplicate = Some(top_item.clone());
        }
        if let Some(new_item) = duplicate {
            self.stack.push(new_item);
        }
    }

    /// Deletes all items from the Stack
    pub fn clear(&mut self) {
        self.stack.clear()
    }

    /// Rotates the top three items on the stack, pulling the third item out and pushing it on top. This should not
    /// modify the stack if there are fewer than three items
    pub fn rotate(&mut self) {
        if self.stack.len() >= 3 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            let third = self.pop().unwrap();
            self.push(second);
            self.push(first);
            self.push(third);
        }
    }

    /// Pops the top item of the stack and pushes it down the specified number of positions. Thus `shove(0)` has no
    /// effect. The position is taken modulus the original size of the stack. I.E. `shove(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `shove(2)` or `[ 'A', 'C', 'B' ]`.
    ///
    /// Returns true if a shove was performed (even if it had no effect)
    pub fn shove(&mut self, position: i64) -> bool {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.stack.pop().unwrap();
            self.stack.insert(vec_index, item);
            true
        } else {
            false
        }
    }

    /// Reverses the position of the top two items on the stack. No effect if there are not at least two items.
    pub fn swap(&mut self) {
        if self.stack.len() >= 2 {
            let first = self.pop().unwrap();
            let second = self.pop().unwrap();
            self.push(first);
            self.push(second);
        }
    }

    /// Removes an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank(2)` or `[ 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    pub fn yank(&mut self, position: i64) -> bool {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let item = self.stack.remove(vec_index);
            self.stack.push(item);
            true
        } else {
            false
        }
    }

    /// Copies an item by its index from deep in the stack and pushes it onto the top. The position is taken modulus
    /// the original size of the stack. I.E. `yank_duplicate(5)` on a stack consisting of
    /// `[ 'C', 'B', 'A' ]` would result in effectively `yank_duplicate(2)` or `[ 'C', 'B', 'A', 'C' ]`.
    ///
    /// Returns true if a yank was performed (even if it had no effect)
    pub fn yank_duplicate(&mut self, position: i64) -> bool {
        if self.stack.len() > 0 {
            let vec_index = stack_to_vec(position, self.stack.len());
            let duplicate = self.stack.get(vec_index).unwrap().clone();
            self.stack.push(duplicate);
            true
        } else {
            false
        }
    }
}


pub struct BaseVm {
    rng: rand::rngs::SmallRng,
    exec_stack: VmStack<Box<dyn VmInstruction<BaseVm>>>,
    name_stack: VmStack<String>,
    bool_stack: VmStack<bool>,
    parsers: Vec<fn(input: &str) -> nom::IResult<&str, Box<dyn VmInstruction<BaseVm>>>>,
    quote_next_name: bool,
    defined_names: fnv::FnvHashMap<String, Box<dyn VmInstruction<BaseVm>>>,
}

impl BaseVm {
    fn new() -> BaseVm {
        use rand::prelude::SeedableRng;

        let mut vm = BaseVm {
            rng: rand::rngs::SmallRng::from_entropy(),
            exec_stack: VmStack::new(),
            name_stack: VmStack::new(),
            bool_stack: VmStack::new(),
            parsers: vec![],
            quote_next_name: false,
            defined_names: fnv::FnvHashMap::default(),
        };

        vm.parsers.push(VmBoolAnd::parse);
        vm.parsers.push(VmBoolDefine::parse);
        vm.parsers.push(VmBoolLiteralValue::parse);
        vm.parsers.push(VmBoolRand::parse);

        vm
    }

    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Box<dyn VmInstruction<BaseVm>>> {
        for parse_fn in self.parsers.iter() {
            match parse_fn(input) {
                Ok((rest, instruction)) => return Ok((rest, instruction)),
                Err(_) => {
                    // Continue searching
                }
            }
        }

        // Return an error if none of our parsers could parse the string
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }
}

pub trait VirtualMachineMustHaveRng {
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng;
}

impl VirtualMachineMustHaveRng for BaseVm {
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        &mut self.rng
    }
}

pub trait VirtualMachineMustHaveExecStack {
    fn exec(&mut self) -> &mut VmStack<Box<dyn VmInstruction<BaseVm>>>;
}

impl VirtualMachineMustHaveExecStack for BaseVm {
    fn exec(&mut self) -> &mut VmStack<Box<dyn VmInstruction<BaseVm>>> {
        &mut self.exec_stack
    }
}

pub trait VirtualMachineMustHaveNameStack {
    fn name(&mut self) -> &mut VmStack<String>;
    fn should_quote_next_name(&self) -> bool;
    fn set_should_quote_next_name(&mut self, quote_next_name: bool);
    fn definition_for_name(&self, name: &String) -> Option<Box<dyn VmInstruction<BaseVm>>>;
    fn define_name(&mut self, name: String, code: Box<dyn VmInstruction<BaseVm>>);
    fn all_defined_names(&self) -> Vec<String>;
    fn defined_names_len(&self) -> usize;
}

impl VirtualMachineMustHaveNameStack for BaseVm {
    fn name(&mut self) -> &mut VmStack<String> {
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
    fn definition_for_name(&self, name: &String) -> Option<Box<dyn VmInstruction<BaseVm>>> {
        self.defined_names.get(name).map(|c| c.clone())
    }

    /// Defines the Code that will be placed on the top of the Exec stack when the specified Name is encountered. If the
    /// name was previously defined, the new definition replaces the old value.
    fn define_name(&mut self, name: String, code: Box<dyn VmInstruction<BaseVm>>) {
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
    fn bool(&mut self) -> &mut VmStack<bool>;
}

impl VirtualMachineMustHaveBoolStack for BaseVm {
    fn bool(&mut self) -> &mut VmStack<bool> {
        &mut self.bool_stack
    }
}

pub trait VirtualMachineMustHaveJunkStack {
    fn junk(&mut self) -> &mut VmStack<String>;
}

struct VmBoolAnd {}

impl VmNamed for VmBoolAnd {
    fn name() -> &'static str {
        "BOOL.AND"
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack> VmCode<Vm> for VmBoolAnd {
    fn parse(input: &str) -> nom::IResult<&str, Box<dyn VmInstruction<Vm>>> {
        let (rest, _) = nom::bytes::complete::tag(VmBoolAnd::name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, Box::new(VmBoolAnd {})))
    }

    fn random_value(_rng: &mut rand::rngs::SmallRng) -> Box<dyn VmInstruction<Vm>> {
        Box::new(VmBoolAnd {})
    }
}

impl std::fmt::Display for VmBoolAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(VmBoolAnd::name())
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack> VmInstruction<Vm> for VmBoolAnd {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn self_name(&self) -> &'static str {
        VmBoolAnd::name()
    }


    /// The instruction must be able to clone itself, though we cannot implement the normal 'Clone' trait or the
    /// VmInstruction could not become a trait object
    fn clone(&self) -> Box<dyn VmInstruction<Vm>> {
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

impl VmBoolDefine {
    fn name() -> &'static str {
        "BOOL.DEFINE"
    }

    fn parse<Vm: VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveNameStack>(input: &str) -> nom::IResult<&str, Box<dyn VmInstruction<Vm>>> {
        let (rest, _) = nom::bytes::complete::tag(Self::name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, Box::new(VmBoolDefine {})))
    }

    fn random_value<Vm: VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveNameStack>(_rng: &mut rand::rngs::SmallRng) -> Box<dyn VmInstruction<Vm>> {
        Box::new(VmBoolDefine {})
    }
}

impl std::fmt::Display for VmBoolDefine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(VmBoolDefine::name())
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack + VirtualMachineMustHaveNameStack> VmInstruction<Vm> for VmBoolDefine {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn self_name(&self) -> &'static str {
        VmBoolDefine::name()
    }

    fn clone(&self) -> Box<dyn VmInstruction<Vm>> {
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
    fn name() -> &'static str {
        "BOOL.LITERALVALUE"
    }

    fn new(value: bool) -> VmBoolLiteralValue {
        VmBoolLiteralValue {
            value
        }
    }

    fn parse<Vm: VirtualMachineMustHaveBoolStack>(input: &str) -> nom::IResult<&str, Box<dyn VmInstruction<Vm>>> {
        let (rest, value) = crate::parse::parse_code_bool(input)?;
        Ok((rest, Box::new(VmBoolLiteralValue::new(value))))
    }

    fn random_value<Vm: VirtualMachineMustHaveBoolStack>(rng: &mut rand::rngs::SmallRng) -> Box<dyn VmInstruction<Vm>> {
        use rand::Rng;
        Box::new(VmBoolLiteralValue::new(if 0 == rng.gen_range(0..=1) { false } else { true }))
    }
}

impl std::fmt::Display for VmBoolLiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "TRUE" } else { "FALSE" })
    }
}

impl<Vm: VirtualMachineMustHaveBoolStack> VmInstruction<Vm> for VmBoolLiteralValue {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn self_name(&self) -> &'static str {
        VmBoolLiteralValue::name()
    }

    fn clone(&self) -> Box<dyn VmInstruction<Vm>> {
        Box::new(VmBoolLiteralValue::new(self.value))
    }

    fn execute(&self, vm: &mut Vm) {
        vm.bool().push(self.value)
    }

    /// An instruction must be able to determine if it is equal to another instruction. We cannot simply use PartialEq
    /// and Eq because then VmInstruction could not become a trait object.
    ///
    /// The default implementation simply checks to see if the static name string points to the same location in memory.
    /// Instructions that have data MUST override this behavior to check for data equivalence.
    fn eq(&self, other: &dyn VmInstruction<Vm>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<VmBoolLiteralValue>() {
            self.value == other.value
        } else {
            false
        }
    }

    /// An instruction must be able to provide a unique hash of itself and its data. The hash() of two instructions that
    /// are eq() to each other should also be equal.
    ///
    /// We cannot simply use Hash because then VmInstruction could not become a trait object (no methods with generics).
    ///
    /// The default implementation uses the sea-hash of the name. Instructions that have data MUST override this
    /// behavior to include the data in the hash.
    fn hash(&self) -> u64 {
        let mut to_hash: Vec<u8> = VmBoolLiteralValue::name().as_bytes().iter().map(|c| *c).collect();
        to_hash.push(if self.value { 1 } else { 0 });
        seahash::hash(&to_hash[..])
    }
}


struct VmBoolRand {}

impl VmBoolRand {
    fn name() -> &'static str {
        "BOOL.RAND"
    }

    fn parse<Vm: VirtualMachineMustHaveExecStack + VirtualMachineMustHaveRng>(input: &str) -> nom::IResult<&str, Box<dyn VmInstruction<Vm>>> {
        let (rest, _) = nom::bytes::complete::tag(Self::name())(input)?;
        let (rest, _) = crate::parse::space_or_end(rest)?;

        Ok((rest, Box::new(VmBoolRand {})))
    }

    fn random_value<Vm: VirtualMachineMustHaveExecStack + VirtualMachineMustHaveRng>(_rng: &mut rand::rngs::SmallRng) -> Box<dyn VmInstruction<Vm>> {
        Box::new(VmBoolRand {})
    }
}

impl std::fmt::Display for VmBoolRand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(VmBoolRand::name())
    }
}

impl<Vm: VirtualMachineMustHaveExecStack + VirtualMachineMustHaveRng> VmInstruction<Vm> for VmBoolRand {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn self_name(&self) -> &'static str {
        VmBoolRand::name()
    }

    fn clone(&self) -> Box<dyn VmInstruction<Vm>> {
        Box::new(VmBoolRand{})
    }

    fn execute(&self, vm: &mut Vm) {
        let rand = VmBoolLiteralValue::random_value(vm.get_rng());
        vm.exec().push(rand)
    }
}