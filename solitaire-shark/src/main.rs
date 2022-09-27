mod card;
mod game_state;
mod island_common;
mod island_five;
mod island_four;
mod island_one;
mod island_three;
mod island_two;
mod pile;
mod solitaire_result;
mod suit;
mod vm;

pub use card::{Card, VirtualMachineMustHaveCard};
pub use game_state::GameState;
use island_five::IslandFive;
use island_four::IslandFour;
use island_one::IslandOne;
use island_three::IslandThree;
use island_two::IslandTwo;

use pushgp::{VirtualMachine, World, WorldConfiguration};
pub use suit::Suit;
pub use vm::{SolitareVm, VirtualMachineMustHaveGame};

use crate::{solitaire_result::SolitaireResults, vm::add_instructions};

fn main() {
    // Starup prometheus exporter
    prometheus_exporter::start("0.0.0.0:9184".parse().expect("failed to parse binding"))
        .expect("failed to start prometheus exporter");

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
    let mut world = World::<SolitaireResults, SolitareVm>::new(vm, world_config);

    // Add each island to the world
    world.create_island(Box::new(IslandOne::new()));
    world.create_island(Box::new(IslandTwo::new()));
    world.create_island(Box::new(IslandThree::new()));
    world.create_island(Box::new(IslandFour::new()));
    world.create_island(Box::new(IslandFive::new()));

    // Calculate the best instructions. Commented out for now because this doesn't seem to be effective
    // let weights = world.heuristically_calculate_instruction_weights(1000);
    // for (instruction, weight) in weights.iter() {
    //     println!("{:3} for {}", weight, instruction);
    // }

    // Calculate the best instructions
    let weights = pushgp_weights::find_best_weights(&world);
    let mut config = world.get_vm().engine().get_configuration().clone();
    config.set_all_instruction_weights(weights);
    world.get_vm_mut().engine_mut().reset_configuration(config);

    // Run the world for 10_000 generations
    let mut generations_complete = 0;
    world.run_generations_while(|world| {
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
            most_fit_island_two
                .get_run_result()
                .unwrap()
                .number_of_finished_cards() as f64
                / 100.0f64
        );
        let most_fit_island_three = world.get_island(2).unwrap().most_fit_individual().unwrap();
        println!(
            "  island three:   {:.04} avg remaining draw+play cards",
            most_fit_island_three
                .get_run_result()
                .unwrap()
                .number_of_draw_stack_cards() as f64
                / 100.0f64
        );
        let most_fit_island_four = world.get_island(3).unwrap().most_fit_individual().unwrap();
        println!(
            "  island four:   {:.04} avg remaining face down cards",
            most_fit_island_four
                .get_run_result()
                .unwrap()
                .number_of_face_down_cards() as f64
                / 100.0f64
        );
        let most_fit_island_five = world.get_island(4).unwrap().most_fit_individual().unwrap();
        println!(
            "  island five:   {:.04} avg remaining face up cards",
            most_fit_island_five
                .get_run_result()
                .unwrap()
                .number_of_face_up_cards() as f64
                / 100.0f64
        );

        generations_complete < 10_000
    });
}
