use crate::*;
use lazy_static::lazy_static;
use prometheus::{register_int_counter, register_int_counter_vec, IntCounter, IntCounterVec};

lazy_static! {
    pub static ref NOOP_ILLEGAL_OPERATION_COUNTER: IntCounter = register_int_counter!(
        "noop_executions_illegal_operation_total",
        "The number of times an instruction execution resulted in a NOOP because of illegal operation",
    )
    .unwrap();
    pub static ref NOOP_INSUFFICIENT_INPUTS_COUNTER: IntCounter = register_int_counter!(
        "noop_executions_insufficient_inputs_total",
        "The number of times an instruction execution resulted in a NOOP because of insufficient inputs",
    )
    .unwrap();
    pub static ref PROGRAM_EXIT_COUNTER_VEC: IntCounterVec = register_int_counter_vec!(
        "program_exit_total",
        "The number of times a program finished executing",
        &["exit_reason"]
    )
    .unwrap();
}

pub trait VirtualMachine:
    Clone + Sized + DoesVirtualMachineHaveName + VirtualMachineMustHaveExec<Self> + 'static + OpcodeConvertor
{
    /// The engine implements functions that are common to all virtual machines. Each VirtualMachine must have an engine
    fn engine(&self) -> &VirtualMachineEngine<Self>;

    /// Most of the engine functions are mut and so we also need a mutating accessor.
    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self>;

    /// Clears the data out of the VirtualMachine, making it ready for new code
    fn clear(&mut self);

    /// Runs the VirtualMachine until the Exec stack is empty or the specified number of instructions have been
    /// processed. The default implementation rarely needs to be overridden.
    fn run(&mut self, max: usize) -> ExitStatus {
        // trace!("{:?}", self);
        let mut stats = ExitStats { total_instruction_count: 0, total_noop_count: 0 };
        loop {
            match self.next() {
                Ok(count) => stats.total_instruction_count += count,
                Err(ExecutionError::ExecStackEmpty) => {
                    PROGRAM_EXIT_COUNTER_VEC.get_metric_with_label_values(&["normal"]).unwrap().inc();
                    return ExitStatus::Normal(stats);
                }
                Err(ExecutionError::IllegalOperation) => {
                    stats.total_instruction_count += 1;
                    NOOP_ILLEGAL_OPERATION_COUNTER.inc();
                    stats.total_noop_count += 1;
                }
                Err(ExecutionError::InsufficientInputs) => {
                    stats.total_instruction_count += 1;
                    NOOP_INSUFFICIENT_INPUTS_COUNTER.inc();
                    stats.total_noop_count += 1;
                }
                Err(ExecutionError::OutOfMemory) => {
                    PROGRAM_EXIT_COUNTER_VEC.get_metric_with_label_values(&["exceeded_memory_limit"]).unwrap().inc();
                    return ExitStatus::ExceededMemoryLimit(stats);
                }
                Err(ExecutionError::InvalidOpcode) => {
                    PROGRAM_EXIT_COUNTER_VEC.get_metric_with_label_values(&["exceeded_invalid_opcode"]).unwrap().inc();
                    return ExitStatus::InvalidOpcode(stats);
                }
            }

            if stats.total_instruction_count >= max {
                PROGRAM_EXIT_COUNTER_VEC.get_metric_with_label_values(&["exceeded_instruction_count"]).unwrap().inc();
                return ExitStatus::ExceededInstructionCount(stats);
            }
        }
    }

    /// Processes the next instruction from the Exec stack. The return type allows for some VirtualMachines to indicate
    /// how expensive an instruction was. Typically returns Ok(1)
    fn next(&mut self) -> Result<usize, ExecutionError> {
        // Pop the top piece of code from the exec stack and execute it.
        let exec = self.engine_mut().exec().pop().ok_or(ExecutionError::ExecStackEmpty)?;
        let (execute_fn, _timer) = self.engine().execute_fn(exec.get_opcode()).ok_or(ExecutionError::InvalidOpcode)?;
        execute_fn(exec, self)?;

        Ok(1)
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
    fn execute_immediate<I: Instruction<Self>>(&mut self, code: Code) -> Result<(), ExecutionError> {
        I::execute(code, self)
    }
}

#[derive(Clone, Debug, PartialEq)]
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
            engine: VirtualMachineEngine::new(seed, config, 20),
            bool_stack: Stack::new(200),
            code_stack: Stack::new(20),
            float_stack: Stack::new(200),
            integer_stack: Stack::new(200),
            name_stack: NameStack::new(200),
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
