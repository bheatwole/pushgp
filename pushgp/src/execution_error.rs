/// Used to determine how a program's code performed when run on the virtual machine
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExecutionError {
    /// The instruction attempted to perform an illegal operation (such as divide by zero). All stack values used by the
    /// instruction were removed.
    IllegalOperation,

    /// The instruction needed a certain number and kind of stack values and there were not enough. All stacks left as
    /// they were
    InsufficientInputs,

    /// The program used more memory than allowed. This could be trying to push to a stack that is full, or trying to
    /// allocate on a heap that is full.
    OutOfMemory,
}
