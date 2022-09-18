use std::cmp::Ordering;

use pushgp::IslandCallbacks;

use crate::{island_common::*, solitaire_result::SolitaireResults, SolitareVm};

pub struct IslandThree {
    common: IslandCommon,
}

impl IslandThree {
    pub fn new() -> IslandThree {
        IslandThree {
            common: IslandCommon::new(),
        }
    }
}

impl IslandCallbacks<SolitaireResults, SolitareVm> for IslandThree {
    fn pre_generation_run(&mut self, _individuals: &[pushgp::Individual<SolitaireResults>]) {
        self.common.generate_game_seeds();
    }

    fn run_individual(
        &mut self,
        vm: &mut SolitareVm,
        individual: &mut pushgp::Individual<SolitaireResults>,
    ) {
        self.common.run_individual(vm, individual);
    }

    fn sort_individuals(
        &self,
        a: &pushgp::Individual<SolitaireResults>,
        b: &pushgp::Individual<SolitaireResults>,
    ) -> std::cmp::Ordering {
        // island_three_fitness_score_fn: run 100 games and score on fewest cards in draw+play piles, then win rate
        let a_result = a.get_run_result().unwrap();
        let b_result = b.get_run_result().unwrap();
        let mut cmp = a_result
            .number_of_draw_stack_cards()
            .cmp(&b_result.number_of_draw_stack_cards());

        if Ordering::Equal == cmp {
            cmp = a_result.games_won().cmp(&b_result.games_won());
        }

        cmp
    }

    fn clone(&self) -> Box<dyn IslandCallbacks<SolitaireResults, SolitareVm>> {
        Box::new(IslandThree {
            common: self.common.clone(),
        })
    }
}
