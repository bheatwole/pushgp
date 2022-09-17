use std::cmp::Ordering;

use pushgp::IslandCallbacks;

use crate::{island_common::*, solitaire_result::SolitaireResults, SolitareVm};

pub struct IslandFour {
    common: IslandCommon,
}

impl IslandFour {
    pub fn new() -> IslandFour {
        IslandFour {
            common: IslandCommon::new(),
        }
    }
}

impl IslandCallbacks<SolitaireResults, SolitareVm> for IslandFour {
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
        // island_four_fitness_score_fn: run 100 games and score on fewest cards in face_down piles, then win rate
        let a_result = a.get_run_result().unwrap();
        let b_result = b.get_run_result().unwrap();
        let mut cmp = b_result.number_of_face_down_cards().cmp(&a_result.number_of_face_down_cards());

        if Ordering::Equal == cmp {
            cmp = a_result.games_won().cmp(&b_result.games_won());
        }

        cmp
    }
}
