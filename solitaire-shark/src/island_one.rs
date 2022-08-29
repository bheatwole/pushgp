use std::cmp::Ordering;

use pushgp::{IslandCallbacks, VirtualMachine, VirtualMachineMustHaveName};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{run_result::RunResult, GameState, SolitareVm};

const GAMES_PER_RUN: usize = 100;

pub struct IslandOne {
    rng: SmallRng,
    game_seeds: Vec<u64>,
}

impl IslandOne {
    pub fn new() -> IslandOne {
        IslandOne {
            rng: SmallRng::from_entropy(),
            game_seeds: vec![],
        }
    }
}

impl IslandCallbacks<RunResult, SolitareVm> for IslandOne {
    fn pre_generation_run(&mut self, _individuals: &[pushgp::Individual<RunResult, SolitareVm>]) {
        // Before all individuals run, create 100 seeds for the games each will play. This gives every individual the
        // same 100 shuffled decks.
        while self.game_seeds.len() < GAMES_PER_RUN {
            self.game_seeds.push(self.rng.gen());
        }
    }

    fn run_individual(
        &mut self,
        vm: &mut SolitareVm,
        individual: &mut pushgp::Individual<RunResult, SolitareVm>,
    ) {
        let mut result = RunResult::new();

        // Play 100 games
        for game_index in 0..GAMES_PER_RUN {
            // Clear the stacks and defined functions from any previous runs
            vm.engine_mut().clear();

            // Setup this individuals' code and functions
            vm.engine_mut().set_code(individual.get_code().clone());
            for (name, code) in individual.get_defined_names().iter() {
                vm.name().define_name(name.clone(), code.clone());
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

        // Save the output of all games in the RunResult for the Individual
        individual.set_run_result(Some(result));
    }

    fn sort_individuals(
        &self,
        a: &pushgp::Individual<RunResult, SolitareVm>,
        b: &pushgp::Individual<RunResult, SolitareVm>,
    ) -> std::cmp::Ordering {
        // island_one_fitness_score_fn: run 100 games and score on most games won, then smallest code size
        let a_result = a.get_run_result().as_ref().unwrap();
        let b_result = b.get_run_result().as_ref().unwrap();
        let mut cmp = a_result.games_won().cmp(&b_result.games_won());

        if Ordering::Equal == cmp {
            cmp = a.get_code().points().cmp(&b.get_code().points());
        }

        cmp
    }
}
