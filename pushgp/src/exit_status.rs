/// Used to determine how a program's code performed when run on the virtual machine
pub enum ExitStatus {
    /// The program exited after running all instructions on the exec stack. The number of instructions run is returned.
    Normal(usize),

    /// The program ran to the max number of instructions allowed and could have run longer.
    ExceededInstructionCount,

    /// The program used more memory than allowed
    ExceededMemoryLimit,
}