/// Used to determine how a program's code performed when run on the virtual machine
pub enum ExitStatus {
    /// The program exited after running all instructions on the exec stack. The number of instructions run is returned
    /// in the first parameter, and the number of 
    Normal(ExitStats),

    /// The program ran to the max number of instructions allowed and could have run longer.
    ExceededInstructionCount(ExitStats),

    /// The program used more memory than allowed
    ExceededMemoryLimit(ExitStats),

    /// The program encountered an opcode that was not expected
    InvalidOpcode(ExitStats),
}

pub struct ExitStats {
    pub total_instruction_count: usize,
    pub total_noop_count: usize,
}