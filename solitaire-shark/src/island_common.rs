use pushgp::VirtualMachine;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{solitaire_result::SolitaireResults, GameState, SolitareVm};

const GAMES_PER_RUN: usize = 100;

#[derive(Clone)]
pub struct IslandCommon {
    rng: SmallRng,
    game_seeds: Vec<u64>,
}

impl IslandCommon {
    pub fn new() -> IslandCommon {
        IslandCommon {
            rng: SmallRng::from_entropy(),
            game_seeds: vec![],
        }
    }
}

impl IslandCommon {
    /// Before all individuals run, create 100 seeds for the games each will play. This gives every individual on an island
    /// the same 100 shuffled decks.
    pub fn generate_game_seeds(&mut self) {
        while self.game_seeds.len() < GAMES_PER_RUN {
            self.game_seeds.push(self.rng.gen());
        }
    }

    pub fn run_individual(
        &mut self,
        vm: &mut SolitareVm,
        individual: &mut pushgp::Individual<SolitaireResults>,
    ) {
        let mut result = SolitaireResults::new();

        // Play 100 games
        for game_index in 0..GAMES_PER_RUN {
            // Clear the stacks and defined functions from any previous runs
            vm.clear();

            // Setup this individuals' code and functions
            vm.engine_mut().set_code(individual.get_code().clone());
            for (name, code) in individual.get_defined_names().iter() {
                vm.engine_mut().define_name(name.clone(), code.clone());
            }

            // Setup a new GameState. If this is not the first game, we also need to save the previous game's state.
            let previous_game =
                vm.swap_game_state(GameState::new(*self.game_seeds.get(game_index).unwrap()));
            if game_index != 0 {
                result.save_game(previous_game);
            }

            // Run the vm for up to 10_000 instructions
            vm.run(10_000);
        }

        // Save the GameState from the last game
        let last_game = vm.swap_game_state(GameState::new(1));
        result.save_game(last_game);

        // Save the output of all games in the SolitaireResults for the Individual
        individual.set_run_result(Some(result));
    }
}
