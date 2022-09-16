use crate::*;

pub trait VirtualMachine:
    Sized + DoesVirtualMachineHaveName + VirtualMachineMustHaveExec<Self> + 'static + OpcodeConvertor
{
    /// The engine implements functions that are common to all virtual machines. Each VirtualMachine must have an engine
    fn engine(&self) -> &VirtualMachineEngine<Self>;

    /// Most of the engine functions are mut and so we also need a mutating accessor.
    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self>;

    /// Clears the data out of the VirtualMachine, making it ready for new code
    fn clear(&mut self);

    /// Returns the amount of memory used by the virtual machine
    fn size_of(&self) -> usize;

    /// Runs the VirtualMachine until the Exec stack is empty or the specified number of instructions have been
    /// processed. The default implementation rarely needs to be overridden.
    fn run(&mut self, max: usize) -> ExitStatus {
        // trace!("{:?}", self);
        let mut total_count = 0;
        while let Some(count) = self.next() {
            total_count += count;
            if total_count >= max {
                return ExitStatus::ExceededInstructionCount;
            }
            let size = self.size_of();
            if size >= self.engine().get_configuration().get_max_memory_size() {
                return ExitStatus::ExceededMemoryLimit;
            }
        }
        
        ExitStatus::Normal(total_count)
    }

    /// Processes the next instruction from the Exec stack. The return type allows for some VirtualMachines to indicate
    /// how expensive an instruction was. Typically returns Some(1)
    fn next(&mut self) -> Option<usize> {
        // Pop the top piece of code from the exec stack and execute it.
        if let Some(exec) = self.engine_mut().exec().pop() {
            if let Some(execute_fn) = self.engine().execute_fn(exec.get_opcode()) {
                execute_fn(exec, self);

                // Return the number of points required to perform that action
                return Some(1);
            }
        }

        // No action was found
        None
    }

    /// Returns the random number generator used by the VirtualMachine.
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        self.engine_mut().get_rng()
    }

    /// Formats a code object in the way that std::fmt::Display expects, except with Code as a parameter
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, code: &Code) -> std::fmt::Result {
        self.engine().fmt(f, code)
    }

    /// Calls the random_value function for the instruction that is specified using a type parameter. That means you
    /// have to know the type at compile time.
    fn random_value<I: Instruction<Self>>(&mut self) -> Code {
        I::random_value(self.engine_mut())
    }

    /// Calls the execute function for the instruction that is specified using a type parameter. That means you have to
    ///  know the type at compile time.
    fn execute_immediate<I: Instruction<Self>>(&mut self, code: Code) {
        I::execute(code, self)
    }
}

#[derive(Debug, PartialEq)]
pub struct BaseVm {
    engine: VirtualMachineEngine<BaseVm>,
    bool_stack: Stack<Bool>,
    code_stack: Stack<Code>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: NameStack,
}

impl BaseVm {
    pub fn new(seed: Option<u64>, config: Configuration) -> BaseVm {
        let vm = BaseVm {
            engine: VirtualMachineEngine::new(seed, config),
            bool_stack: Stack::new(),
            code_stack: Stack::new(),
            float_stack: Stack::new(),
            integer_stack: Stack::new(),
            name_stack: NameStack::new(),
        };

        vm
    }
}

impl VirtualMachine for BaseVm {
    fn engine(&self) -> &VirtualMachineEngine<Self> {
        &self.engine
    }

    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self> {
        &mut self.engine
    }

    fn clear(&mut self) {
        self.engine.clear();
        self.bool_stack.clear();
        self.code_stack.clear();
        self.float_stack.clear();
        self.integer_stack.clear();
        self.name_stack.clear();
    }

    fn size_of(&self) -> usize {
        self.engine.size_of()
            + self.bool_stack.size_of()
            + self.code_stack.size_of()
            + self.float_stack.size_of()
            + self.integer_stack.size_of()
            + self.name_stack.size_of()
    }
}

impl VirtualMachineMustHaveBool<BaseVm> for BaseVm {
    fn bool(&mut self) -> &mut Stack<bool> {
        &mut self.bool_stack
    }
}

impl VirtualMachineMustHaveCode<BaseVm> for BaseVm {
    fn code(&mut self) -> &mut Stack<Code> {
        &mut self.code_stack
    }
}

impl VirtualMachineMustHaveExec<BaseVm> for BaseVm {
    fn exec(&mut self) -> &mut Stack<Code> {
        self.engine.exec()
    }
}

impl VirtualMachineMustHaveFloat<BaseVm> for BaseVm {
    fn float(&mut self) -> &mut Stack<Float> {
        &mut self.float_stack
    }
}

impl VirtualMachineMustHaveInteger<BaseVm> for BaseVm {
    fn integer(&mut self) -> &mut Stack<Integer> {
        &mut self.integer_stack
    }
}

impl VirtualMachineMustHaveName<BaseVm> for BaseVm {
    fn name(&mut self) -> &mut NameStack {
        &mut self.name_stack
    }
}

impl DoesVirtualMachineHaveName for BaseVm {
    const HAS_NAME: bool = true;
}

impl OpcodeConvertor for BaseVm {
    /// Returns the name for the specified opcode, or None if the opcode does not exist
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str> {
        self.engine().name_for_opcode(opcode)
    }

    /// Returns the opcode for the specified name, or None if the named instruction has not been registered
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode> {
        self.engine().opcode_for_name(name)
    }
}
