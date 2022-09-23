/// Used to determine how a program's code performed when run on the virtual machine
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExecutionError {
    /// The instruction attempted to perform an illegal operation (such as divide by zero). All stack values used by the
    /// instruction were removed. This is a recoverable error, but the future state of the program might be unexpected.
    IllegalOperation,

    /// The instruction needed a certain number and kind of stack values and there were not enough. All stacks left as
    /// they were. This is a recoverable error.
    InsufficientInputs,

    /// The program used more memory than allowed. This could be trying to push to a stack that is full, or trying to
    /// allocate on a heap that is full. Not recoverable
    OutOfMemory,

    /// An opcode was used that did not map to a valid instruction. Not recoverable
    InvalidOpcode,

    /// The program does not have anymore instructions to execute. It's debatable if this is really an error, but it
    /// does alter the flow of the program. Not recoverable (normal termination)
    ExecStackEmpty,
}
