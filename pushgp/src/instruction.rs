use crate::{Code, InstructionTable, Opcode, VirtualMachine, VirtualMachineEngine, VirtualMachineMustHaveExec};

/// The Instruction is a trait that allows use as a trait object. This significantly restricts what kinds of methods
/// we can include in this trait.
///
/// It is generic for a VirtualMachine. Most instructions will place additional `where` constraints on the VM. I.E. an
/// instruction may require the VM to implement VirtualMachineHasBoolStack, VirtualMachineHasCodeStack and
/// VirtualMachineHasGameState. (VirtualMachineHasGameState being a trait defined in the user's code)
pub trait Instruction<Vm: VirtualMachine + VirtualMachineMustHaveExec<Vm>> {
    /// Every instruction must have a name that is known at compile-time
    fn name(&self) -> &'static str;

    /// Every instruction must be parsable by 'nom' from a string. While the instruction will know what text to look for
    /// and how to create its data, the opcode will vary from one virtual machine to another, and so it is passed as a
    /// parameter.
    fn parse<'a>(&self, input: &'a str, opcode: Opcode) -> nom::IResult<&'a str, Code>;

    /// Every instruction must be able to turn itself from a code object back into a string. Instructions that contain
    /// other instructions will also need access to the instruction table to call `fmt` on the child instructions
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, code: &Code, vtable: &InstructionTable<Vm>) -> std::fmt::Result;

    /// Every instruction must be able to create a new 'random' value. For pure instructions that have no data, the
    /// 'random' value is always the same: a no-data Code. For instructions that do have data (BOOL.LITERALVALUE,
    /// INTEGER.LITERALVALUE, CODE.CODE, etc.), the instruction created will use the random number generator from the
    /// VirtualMachineEngine to create random data.
    ///
    /// The instruction can look up it's opcode by name from the engine.
    fn random_value(&self, engine: &mut VirtualMachineEngine<Vm>) -> Code;

    /// Every instruction must be able to execute itself using a Code object to store data. The instruction must never
    /// panic and may only update the state of the virtual machine. The 'Code' object is consumed by this call.
    fn execute(&self, code: Code, vm: &mut Vm);
}
