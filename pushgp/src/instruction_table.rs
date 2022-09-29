use fnv::FnvHashMap;
use lazy_static::lazy_static;
use prometheus::{
    core::{AtomicF64, AtomicU64, GenericCounter},
    register_counter_vec, register_int_counter_vec, CounterVec, IntCounterVec,
};
use quanta::Clock;

use crate::{Code, CodeParser, ExecutionError, Instruction, Opcode, PushList, VirtualMachine, VirtualMachineEngine};

pub type NameFn = fn() -> &'static str;
pub type ParseFn = fn(input: &str, opcode: Opcode) -> nom::IResult<&str, Code>;
pub type FmtFn<Vm> =
    fn(f: &mut std::fmt::Formatter<'_>, code: &Code, vtable: &InstructionTable<Vm>) -> std::fmt::Result;
pub type RandomValueFn<Vm> = fn(engine: &mut VirtualMachineEngine<Vm>) -> Code;
pub type ExecuteFn<Vm> = fn(code: Code, vm: &mut Vm) -> Result<(), ExecutionError>;

lazy_static! {
    static ref INSTRUCTION_COUNTER_VEC: IntCounterVec = register_int_counter_vec!(
        "instruction_executions_total",
        "The number of times an instruction was executed",
        &["name"]
    )
    .unwrap();
    static ref INSTRUCTION_TIME_VEC: CounterVec = register_counter_vec!(
        "instruction_execution_duration_seconds",
        "The amount of time spent executing each instruction",
        &["name"]
    )
    .unwrap();
}

/// The instruction table allows a single point of entry for the lookup of the main function that every instruction has.
/// This is used to convert from opcode to executation and back.
///
/// It's okay to use a boxed trait object here because these are constructed once during the virtual machine setup and
/// then only referenced. Its use is similar to a compiled virtual table.
///
/// The first entry in every InstructionTable is for PushList, which fixes the 'zero' opcode to reference PushList. All
/// other instructions have opcodes in the order in which they are added to the table
#[derive(Clone)]
pub struct InstructionTable<Vm: VirtualMachine> {
    name_functions: Vec<NameFn>,
    parse_functions: Vec<ParseFn>,
    fmt_functions: Vec<FmtFn<Vm>>,
    random_value_functions: Vec<RandomValueFn<Vm>>,
    execute_functions: Vec<ExecuteEntry<Vm>>,
    lookup_opcode_by_name: FnvHashMap<&'static str, Opcode>,
    clock: Clock,
}

pub trait OpcodeConvertor {
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str>;
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode>;
}

impl<Vm: VirtualMachine> InstructionTable<Vm> {
    pub fn new() -> InstructionTable<Vm> {
        let mut instructions = InstructionTable {
            name_functions: vec![],
            parse_functions: vec![],
            fmt_functions: vec![],
            random_value_functions: vec![],
            execute_functions: vec![],
            lookup_opcode_by_name: FnvHashMap::default(),
            clock: Clock::new(),
        };

        instructions.add_instruction::<PushList>();

        instructions
    }

    pub fn add_instruction<I: Instruction<Vm>>(&mut self) -> Opcode {
        assert!(
            self.name_functions.len() < u32::MAX as usize,
            "Added too many instructions. Please reconsider why you really need 4 billion instructions"
        );
        let opcode = self.name_functions.len() as Opcode;
        let name = I::static_name();
        self.name_functions.push(I::static_name);
        self.parse_functions.push(I::parse);
        self.fmt_functions.push(I::fmt);
        self.random_value_functions.push(I::random_value);
        self.execute_functions.push(ExecuteEntry {
            execute_function: I::execute,
            instruction_count_metric: INSTRUCTION_COUNTER_VEC.get_metric_with_label_values(&[name]).unwrap(),
            instruction_duration: INSTRUCTION_TIME_VEC.get_metric_with_label_values(&[name]).unwrap(),
        });
        self.lookup_opcode_by_name.insert(name, opcode);

        opcode
    }

    /// Using the opcode of the Code object, call the appropriate format function. This may need to recursively call
    /// format for child objects (PushList does this), so also provide a reference to the table
    pub fn fmt(&self, f: &mut std::fmt::Formatter<'_>, code: &Code) -> std::fmt::Result {
        if let Some(fmt_fn) = self.fmt_functions.get(code.get_opcode() as usize) {
            fmt_fn(f, code, &self)
        } else {
            panic!("UNKNOWN_OPCODE {}", code.get_opcode());
        }
    }

    /// Returns the random value fn pointer for the specified opcode or None
    pub fn random_value_fn(&self, opcode: Opcode) -> Option<RandomValueFn<Vm>> {
        self.random_value_functions.get(opcode as usize).map(|f| *f)
    }

    /// Returns the execute fn pointer for the specified opcode or None
    pub fn execute_fn(&self, opcode: Opcode) -> Option<(ExecuteFn<Vm>, InstructionTimer)> {
        self.execute_functions.get(opcode as usize).map(|f| {
            f.instruction_count_metric.inc();
            (f.execute_function, f.instruction_duration.start_timer(self.clock.clone()))
        })
    }
}

impl<Vm: VirtualMachine> CodeParser for InstructionTable<Vm> {
    fn parse<'a>(&self, input: &'a str) -> nom::IResult<&'a str, Code> {
        // Loop through the instructions to see if any can successfully parse the input. Skip the first one which is
        // always PushList. The opcode is the index
        for (index, parse_fn) in self.parse_functions.iter().enumerate().skip(1) {
            let opcode = index as Opcode;
            match parse_fn(input, opcode) {
                Ok((rest, code)) => return Ok((rest, code)),
                Err(_) => {
                    // Continue searching
                }
            }
        }

        // Return an error if none of our parsers could parse the string
        Err(nom::Err::Error(nom::error::make_error(input, nom::error::ErrorKind::Verify)))
    }
}

impl<Vm: VirtualMachine> OpcodeConvertor for InstructionTable<Vm> {
    /// Returns the name for the specified opcode, or None if the opcode does not exist
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str> {
        self.name_functions.get(opcode as usize).map(|name_fn| name_fn())
    }

    /// Returns the opcode for the specified name, or None if the named instruction has not been registered
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode> {
        self.lookup_opcode_by_name.get(name).map(|o| *o)
    }
}

impl<Vm: VirtualMachine> std::fmt::Debug for InstructionTable<Vm> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InstructionTable with {} instructions", self.name_functions.len())
    }
}

impl<Vm: VirtualMachine> std::cmp::PartialEq for InstructionTable<Vm> {
    fn eq(&self, other: &InstructionTable<Vm>) -> bool {
        if self.name_functions.len() != other.name_functions.len() {
            return false;
        }
        for i in 0..self.name_functions.len() {
            if self.name_functions[i] != other.name_functions[i] {
                return false;
            }
        }

        true
    }
}

#[derive(Clone)]
struct ExecuteEntry<Vm: VirtualMachine> {
    pub execute_function: ExecuteFn<Vm>,
    pub instruction_count_metric: GenericCounter<AtomicU64>,
    pub instruction_duration: GenericCounter<AtomicF64>,
}

pub struct InstructionTimer {
    // The counter for recording the duration. We do not use a histogram because the durations are so small
    counter: GenericCounter<AtomicF64>,
    start: u64,
    clock: Clock,
}

impl Drop for InstructionTimer {
    fn drop(&mut self) {
        let end = self.clock.raw();
        self.counter.inc_by(self.clock.delta(self.start, end).as_secs_f64());
    }
}

trait StartTimer {
    fn start_timer(&self, clock: Clock) -> InstructionTimer;
}

impl StartTimer for GenericCounter<AtomicF64> {
    fn start_timer(&self, clock: Clock) -> InstructionTimer {
        InstructionTimer { counter: self.clone(), start: clock.raw(), clock }
    }
}
