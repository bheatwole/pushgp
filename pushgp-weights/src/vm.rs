use pushgp::*;

use crate::{InstructionName, Weight, VirtualMachineMustHaveWeight, VirtualMachineMustHaveInstructionName};

/// InstructionWeightVirtualMachine is used to determine the optimal instruction weights for another VirtualMachine. It
/// does this by executing the initial run of random individuals on another world and using the genetic algorithm with
/// a fitness function that tries to maximize the total score of all random individuals in all islands of that world.
///
/// To effectively use InstructionWeightVirtualMachine the Islands of the target World must all implement the
/// `score_instruction` function in the `IslandCallbacks` trait.
#[derive(Debug, PartialEq)]
pub struct InstructionWeightVirtualMachine<
    TargetRunResult: RunResult,
    TargetVm: VirtualMachine,
> {
    engine: VirtualMachineEngine<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>,
    instruction_name_stack: Stack<InstructionName>,
    weight_stack: Stack<Weight>,
    target_world: World<TargetRunResult, TargetVm>,
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine> VirtualMachine
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    fn engine(&self) -> &VirtualMachineEngine<Self> {
        &self.engine
    }

    fn engine_mut(&mut self) -> &mut VirtualMachineEngine<Self> {
        &mut self.engine
    }

    fn clear(&mut self) {
        self.engine.clear();
        self.instruction_name_stack.clear();
        self.weight_stack.clear();
    }

    fn size_of(&self) -> usize {
        self.engine.size_of() + self.instruction_name_stack.size_of() + self.weight_stack.size_of()
    }
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine>
    VirtualMachineMustHaveExec<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    fn exec(&mut self) -> &mut Stack<Code> {
        self.engine.exec()
    }
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine>
    VirtualMachineMustHaveInstructionName<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    fn instruction_name(&mut self) -> &mut Stack<InstructionName> {
        &mut self.instruction_name_stack
    }
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine>
    VirtualMachineMustHaveWeight<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    fn weight(&mut self) -> &mut Stack<Weight> {
        &mut self.weight_stack
    }
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine>
    DoesVirtualMachineHaveName for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    const HAS_NAME: bool = false;
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine> OpcodeConvertor
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    /// Returns the name for the specified opcode, or None if the opcode does not exist
    fn name_for_opcode(&self, opcode: Opcode) -> Option<&'static str> {
        self.engine().name_for_opcode(opcode)
    }

    /// Returns the opcode for the specified name, or None if the named instruction has not been registered
    fn opcode_for_name(&self, name: &'static str) -> Option<Opcode> {
        self.engine().opcode_for_name(name)
    }
}
