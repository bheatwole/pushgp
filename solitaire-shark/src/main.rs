mod card;
mod game_state;
mod suit;

pub use card::Card;
pub use game_state::GameState;
pub use suit::Suit;

fn main() {
    // Parameters:
    // max_instructions_per_context: 100_000
    // island_population_size: 100
    // island_migration_after_generations: 10
    // island_migration_count: 10
    // island_migration_selection_curve: PreferenceForFit
    // island_migration_replacement_selection_curve: StrongPreferenceForUnfit
    // generation_state: struct with vec of 100 seeds
    // pre_generation_setup: fn that randomly selects 100 seeds
    // island_one_fitness_score_fn: run 100 games and score on most games won, then smallest code size
    // island_two_fitness_score_fn: run 100 games and score on most cards to finished stacks, then win rate
    // island_three_fitness_score_fn: run 100 games and score on fewest cards in draw+play piles, then win rate
    // island_four_fitness_score_fn: run 100 games and score on fewest cards in face_down piles, then win rate
    // island_five_fitness_score_fn: run 100 games and score on fewest cards in face_up piles, then win rate
    // island_six_fitness_score_fn: run 100 games and score on smallest code size, then win rate
    // island_seven_fitness_score_fn: run 100 games and score on fewest instructions executed, then win rate

    println!("Hello, world!");

    push_context!(
        name: SolitaireContext,
        state: GameState,
        global_state: None,
        stacks: [ bool, i64, Decimal, f64, Code, Exec, Name, Card, Suit ],
        islands: [
            {
                name: Main,
                state: IslandState,
                fitness: main_island_fitness,
            }
        ]
        instructions: ["BOOL.AND", "GAME.DRAWTHREE"],
    );
}

fn main_island_fitness()