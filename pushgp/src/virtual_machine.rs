use crate::*;

pub trait VirtualMachine: Sized + DoesVirtualMachineHaveName + VirtualMachineMustHaveExec<Self> + 'static {
    /// The engine implements functions that are common to all virtual machines. Each VirtualMachine must have an engine
    fn engine(&self) -> &VirtualMachineEngine<Self>;

    /// Most of the engine functions are mut and so we also need a mutating accessor.
    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self>;

    /// Runs the VirtualMachine until the Exec stack is empty or the specified number of instructions have been
    /// processed. The default implementation rarely needs to be overridden.
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

    /// Processes the next instruction from the Exec stack. The return type allows for some VirtualMachines to indicate
    /// how expensive an instruction was. Typically returns Some(1)
    fn next(&mut self) -> Option<usize> {
        // Pop the top piece of code from the exec stack and execute it.
        if let Some(mut exec) = self.engine_mut().exec().pop() {
            exec.execute(self);

            // Return the number of points required to perform that action
            return Some(1);
        }

        // No action was found
        None
    }

    /// Creates a new random instruction
    fn generate_random_instruction(&mut self) -> Code<Self> {
        let generator = self.engine_mut().pick_random_instruction_generator();
        generator(self)
    }

    /// Returns the random number generator used by the VirtualMachine.
    fn get_rng(&mut self) -> &mut rand::rngs::SmallRng {
        self.engine_mut().get_rng()
    }
}

#[derive(Debug, PartialEq)]
pub struct BaseVm {
    engine: VirtualMachineEngine<BaseVm>,
    bool_stack: Stack<Bool>,
    code_stack: Stack<Code<BaseVm>>,
    float_stack: Stack<Float>,
    integer_stack: Stack<Integer>,
    name_stack: NameStack<BaseVm>,
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

    /// Must of the engine functions are mut
    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self> {
        &mut self.engine
    }
}

impl VirtualMachineMustHaveBool<BaseVm> for BaseVm {
    fn bool(&mut self) -> &mut Stack<bool> {
        &mut self.bool_stack
    }
}

impl VirtualMachineMustHaveCode<BaseVm> for BaseVm {
    fn code(&mut self) -> &mut Stack<Code<BaseVm>> {
        &mut self.code_stack
    }
}

impl VirtualMachineMustHaveExec<BaseVm> for BaseVm {
    fn exec(&mut self) -> &mut Stack<Code<BaseVm>> {
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
    fn name(&mut self) -> &mut NameStack<BaseVm> {
        &mut self.name_stack
    }
}

impl DoesVirtualMachineHaveName for BaseVm {
    const HAS_NAME: bool = true;
}
