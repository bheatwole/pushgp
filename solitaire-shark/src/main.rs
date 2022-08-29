mod card;
mod game_state;
mod pile;
mod run_result;
mod suit;
mod vm;

pub use card::{Card, VirtualMachineMustHaveCard};
pub use game_state::GameState;
use pushgp::{World, WorldConfiguration};
pub use suit::Suit;
pub use vm::{SolitareVm, VirtualMachineMustHaveGame};

use crate::{run_result::RunResult, vm::add_instructions};

fn main() {
    // Parameters:
    // max_instructions_per_context: 100_000
    // island_population_size: 100
    // island_migration_after_generations: 10
    // island_migration_count: 10
    // island_migration_selection_curve: PreferenceForFit
    // island_migration_replacement_selection_curve: StrongPreferenceForUnfit
    // island_genetic_operation_selection_curve: PreferenceForFit
    // generation_state: struct with vec of 100 seeds
    // pre_generation_setup: fn that randomly selects 100 seeds
    // island_one_fitness_score_fn: run 100 games and score on most games won, then smallest code size
    // island_two_fitness_score_fn: run 100 games and score on most cards to finished stacks, then win rate
    // island_three_fitness_score_fn: run 100 games and score on fewest cards in draw+play piles, then win rate
    // island_four_fitness_score_fn: run 100 games and score on fewest cards in face_down piles, then win rate
    // island_five_fitness_score_fn: run 100 games and score on fewest cards in face_up piles, then win rate
    // island_six_fitness_score_fn: run 100 games and score on smallest code size, then win rate
    // island_seven_fitness_score_fn: run 100 games and score on fewest instructions executed, then win rate

    // Create the initial configuration
    let config = pushgp::Configuration::new(1000, 99, 1, 1, fnv::FnvHashMap::default());

    // Create the base Virtual Machine and add all instructions
    let mut vm = SolitareVm::new(1, config);
    add_instructions(&mut vm);

    // Create the world with its parameters
    let world_config = WorldConfiguration::default();
    let mut world = World::<RunResult, SolitareVm>::new(vm, world_config);

    // Add each island to the world
    // TODO

    // Run the world for 10_000 generations
    let mut generations_complete = 0;
    world.run_generations_until(|_world| {
        generations_complete += 1;
        println!("Generation {} is complete", generations_complete);
        generations_complete < 10_000
    });
}
