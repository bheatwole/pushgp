extern crate pushgp_macros;

mod set_instruction_weight;
mod target;
mod vm;
mod weight;
mod weight_finding_island;

use fnv::FnvHashMap;
use pushgp::{RunResult, VirtualMachine, World, WorldConfiguration};

pub use set_instruction_weight::*;
pub use target::*;
pub use vm::*;
pub use weight::*;
pub use weight_finding_island::*;

pub fn find_best_weights<TargetRunResult: RunResult, TargetVm: VirtualMachine>(
    world: &World<TargetRunResult, TargetVm>,
) -> FnvHashMap<&'static str, u8> {

    // Create the base Virtual Machine and add all instructions
    let vm = InstructionWeightVirtualMachine::new(world);

    // Create the world with its parameters
    let world_config = WorldConfiguration::default();
    let mut weight_finding_world = World::<WeightResult, InstructionWeightVirtualMachine<TargetRunResult, TargetVm>>::new(vm, world_config);

    // Add each island to the world
    weight_finding_world.create_island(Box::new(WeightFindingIsland::new(world.get_vm().engine().get_weights().get_instruction_names().len() * 3)));
    
    // Run the world until the best score hasn't increased in ten generations
    let mut generations_complete = 0;
    let mut best_result = WeightResult {
        score: 0,
        weights: FnvHashMap::default(),
    };
    let mut generations_since_new_best = 0;
    weight_finding_world.run_generations_while(|w| {
        generations_complete += 1;
        generations_since_new_best += 1;

        let island = w.get_island(0).unwrap();
        let best_run_result = island.most_fit_individual().unwrap().get_run_result().unwrap();
        println!("WeightFinder: Generation {}, best score is {}", generations_complete, best_run_result.score);
        if best_run_result.score > best_result.score {
            best_result = best_run_result.clone();
            generations_since_new_best = 0;
        }

        generations_since_new_best < 10
    });

    println!("WeightFinder: best weights are:");
    for (name, weight) in best_result.weights.iter() {
        println!("  {}: {}", weight, name);
    }
    best_result.weights
}
