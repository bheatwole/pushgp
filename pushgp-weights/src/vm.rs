use fnv::FnvHashMap;
use pushgp::*;

use crate::{VirtualMachineMustHaveWeight, Weight, WorldTarget, Target, VirtualMachineMustHaveTarget};

/// InstructionWeightVirtualMachine is used to determine the optimal instruction weights for another VirtualMachine. It
/// does this by executing the initial run of random individuals on another world and using the genetic algorithm with
/// a fitness function that tries to maximize the total score of all random individuals in all islands of that world.
///
/// To effectively use InstructionWeightVirtualMachine the Islands of the target World must all implement the
/// `score_instruction` function in the `IslandCallbacks` trait.
#[derive(Clone, Debug, PartialEq)]
pub struct InstructionWeightVirtualMachine<TargetRunResult: RunResult, TargetVm: VirtualMachine> {
    engine: VirtualMachineEngine<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>,
    integer_stack: Stack<Integer>,
    weight_stack: Stack<Weight>,
    world_target: WorldTarget<TargetRunResult, TargetVm>,
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine>
    InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    pub fn new(
        world: &World<TargetRunResult, TargetVm>,
    ) -> InstructionWeightVirtualMachine<TargetRunResult, TargetVm> {
        let mut vm = InstructionWeightVirtualMachine {
            engine: VirtualMachineEngine::new(
                None,
                Configuration::new(65536, 100, 99, 1, 0, FnvHashMap::default()),
            ),
            integer_stack: Stack::new(),
            weight_stack: Stack::new(),
            world_target: WorldTarget::new(world),
        };

        vm.add_instructions();

        vm
    }

    fn add_instructions(&mut self) {
        self.engine_mut()
            .add_instruction::<pushgp::IntegerLiteralValue>();
        self.engine_mut()
            .add_instruction::<crate::set_instruction_weight::ExecSetInstructionWeight>();
        self.engine_mut()
            .add_instruction::<crate::weight::WeightLiteralValue>();
    }
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
        self.integer_stack.clear();
        self.weight_stack.clear();
    }

    fn size_of(&self) -> usize {
        self.engine.size_of() + self.integer_stack.size_of() + self.weight_stack.size_of()
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
    VirtualMachineMustHaveInteger<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    fn integer(&mut self) -> &mut Stack<Integer> {
        &mut self.integer_stack
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
    VirtualMachineMustHaveTarget<InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
{
    fn target(&mut self) -> &mut dyn Target {
        &mut self.world_target
    }
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine> DoesVirtualMachineHaveName
    for InstructionWeightVirtualMachine<TargetRunResult, TargetVm>
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
