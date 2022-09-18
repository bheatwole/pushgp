use fnv::FnvHashMap;
use pushgp::{RunResult, VirtualMachine, World, Configuration};

use crate::Weight;

pub trait VirtualMachineMustHaveTarget<Vm> {
    fn target(&mut self) -> &mut dyn Target;
}

pub trait Target {
    fn set_target_weight(&mut self, random_index: usize, weight: Weight);
    fn reset_target(&mut self);
    fn fill_and_run_one_generation(&mut self);
    fn calculate_world_score(&self) -> u64;
    fn get_weights(&self) -> FnvHashMap<&'static str, u8>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorldTarget<TargetRunResult: RunResult, TargetVm: VirtualMachine> {
    world: World<TargetRunResult, TargetVm>,
    original_config: Configuration,
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine> WorldTarget<TargetRunResult, TargetVm> {
    pub fn new(world: &World<TargetRunResult, TargetVm>) -> WorldTarget<TargetRunResult, TargetVm> {
        WorldTarget {
            world: world.clone(),
            original_config: world.get_vm().engine().get_configuration().clone(),
        }
    }
}

impl<TargetRunResult: RunResult, TargetVm: VirtualMachine> Target
    for WorldTarget<TargetRunResult, TargetVm>
{
    fn set_target_weight(&mut self, random_index: usize, weight: Weight) {
        // Get the target VM
        let vm = self.world.get_vm_mut();

        // Get the list of instruction names from the target VM and use the randomly picked integer % len to get the
        // name of the instruction.
        let names = vm.engine().get_weights().get_instruction_names();
        let pick = random_index % names.len();
        let instruction_name = names[pick];

        // Update the configuration of the target VM
        let mut config = vm.engine().get_configuration().clone();
        config.set_instruction_weight(instruction_name, weight);
        vm.engine_mut().reset_configuration(config);
    }

    fn reset_target(&mut self) {
        self.world.reset_all_islands();
        self.world.get_vm_mut().engine_mut().reset_configuration(self.original_config.clone());
    }

    fn fill_and_run_one_generation(&mut self) {
        self.world.fill_all_islands();
        self.world.run_one_generation();
    }

    fn calculate_world_score(&self) -> u64 {
        let mut score: u64 = 0;
        for island_id in 0..self.world.get_number_of_islands() {
            let island = self.world.get_island(island_id).unwrap();
            for individual_id in 0..island.len() {
                score = score.saturating_add(island.score_for_individual(individual_id).unwrap());
            }
        }

        score
    }

    fn get_weights(&self) -> FnvHashMap<&'static str, u8> {
        self.world.get_vm().engine().get_configuration().get_weights().clone()
    }
}
