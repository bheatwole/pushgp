mod card;
mod game_state;
mod island_common;
mod island_one;
mod island_two;
mod pile;
mod run_result;
mod suit;
mod vm;

pub use card::{Card, VirtualMachineMustHaveCard};
pub use game_state::GameState;
use island_one::IslandOne;
use island_two::IslandTwo;
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
    // island_eight_fitness_score_fn: run 100 games and score on fewest noop instructions executed, then win rate

    // Create the initial configuration
    let config = pushgp::Configuration::new(
        100 * 1024 * 1024,
        1000,
        99,
        1,
        1,
        fnv::FnvHashMap::default(),
    );

    // Create the base Virtual Machine and add all instructions
    let mut vm = SolitareVm::new(1, config);
    add_instructions(&mut vm);

    // Create the world with its parameters
    let world_config = WorldConfiguration::default();
    let mut world = World::<RunResult, SolitareVm>::new(vm, world_config);

    // Add each island to the world
    world.create_island(Box::new(IslandOne::new()));
    world.create_island(Box::new(IslandTwo::new()));

    // Run the world for 10_000 generations
    let mut generations_complete = 0;
    world.run_generations_until(|world| {
        generations_complete += 1;
        println!("Generation {} is complete", generations_complete);
        let most_fit_island_one = world.get_island(0).unwrap().most_fit_individual().unwrap();
        println!(
            "  island one:   {:.04}% games won",
            most_fit_island_one.get_run_result().unwrap().games_won() as f64 / 100.0f64
        );
        let most_fit_island_two = world.get_island(1).unwrap().most_fit_individual().unwrap();
        println!(
            "  island two:   {:.04} avg finished cards",
            most_fit_island_two.get_run_result().unwrap().number_of_finished_cards() as f64 / 100.0f64
        );

        generations_complete < 10_000
    });
}
