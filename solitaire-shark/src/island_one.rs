use std::cmp::Ordering;

use pushgp::IslandCallbacks;

use crate::{island_common::*, run_result::RunResult, SolitareVm};

pub struct IslandOne {
    common: IslandCommon,
}

impl IslandOne {
    pub fn new() -> IslandOne {
        IslandOne {
            common: IslandCommon::new(),
        }
    }
}

impl IslandCallbacks<RunResult, SolitareVm> for IslandOne {
    fn pre_generation_run(&mut self, _individuals: &[pushgp::Individual<RunResult>]) {
        self.common.generate_game_seeds();
    }

    fn run_individual(
        &mut self,
        vm: &mut SolitareVm,
        individual: &mut pushgp::Individual<RunResult>,
    ) {
        self.common.run_individual(vm, individual);
    }

    fn sort_individuals(
        &self,
        a: &pushgp::Individual<RunResult>,
        b: &pushgp::Individual<RunResult>,
    ) -> std::cmp::Ordering {
        // island_one_fitness_score_fn: run 100 games and score on most games won, then smallest code size
        let a_result = a.get_run_result().unwrap();
        let b_result = b.get_run_result().unwrap();
        let mut cmp = a_result.games_won().cmp(&b_result.games_won());

        if Ordering::Equal == cmp {
            cmp = a.get_code().points().cmp(&b.get_code().points());
        }

        cmp
    }
}
