use fnv::FnvHashMap;
use pushgp::{VirtualMachine, RunResult, IslandCallbacks, Individual};

use crate::{InstructionWeightVirtualMachine, VirtualMachineMustHaveTarget};

#[derive(Clone, Debug, PartialEq)]
pub struct WeightResult {
    pub score: u64,
    pub weights: FnvHashMap<&'static str, u8>,
}

impl RunResult for WeightResult {}

#[derive(Clone)]
pub struct WeightFindingIsland {
    max_instructions: usize
}

impl WeightFindingIsland {
    pub fn new(max_instructions: usize) -> WeightFindingIsland {
        WeightFindingIsland { max_instructions }
    }
}


impl<TargetRunResult: RunResult, TargetVm: VirtualMachine> IslandCallbacks<WeightResult, InstructionWeightVirtualMachine<TargetRunResult, TargetVm>> for WeightFindingIsland {
    fn run_individual(
        &mut self,
        vm: &mut InstructionWeightVirtualMachine<TargetRunResult, TargetVm>,
        individual: &mut pushgp::Individual<WeightResult>,
    ) {
        // Reset the target world back to blank state, but with its islands intact
        vm.target().reset_target();

        // Reset our VM and then add this individual's code
        vm.clear();
        vm.engine_mut().set_code(individual.get_code().clone());

        // Run this VM for a while. This will update the configuration of weights in the target VM.
        vm.run(self.max_instructions);

        // Fill the target world with randomly selected individuals using the new weight configuration and run the
        // target world for one generation
        vm.target().fill_and_run_one_generation();

        // Sum the score of all first generation individuals across all islands
        let score = vm.target().calculate_world_score();

        // That's the score of this weight configuration. Also save the weights that this individual set in the target
        // VM so that we can report that back if this happens to be the winner.
        individual.set_run_result(Some(WeightResult {
            score,
            weights: vm.target().get_weights(),
        }));
    }

    fn score_individual(&self, i: &Individual<WeightResult>) -> u64 {
        i.get_run_result().unwrap().score
    }

    fn clone(&self) -> Box<dyn IslandCallbacks<WeightResult, InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>> {
        Box::new(Clone::clone(self))
    }
}