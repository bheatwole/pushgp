use std::cmp::Ordering;

use pushgp::IslandCallbacks;

use crate::{island_common::*, solitaire_result::SolitaireResults, SolitareVm};

pub struct IslandFive {
    common: IslandCommon,
}

impl IslandFive {
    pub fn new() -> IslandFive {
        IslandFive {
            common: IslandCommon::new(),
        }
    }
}

impl IslandCallbacks<SolitaireResults, SolitareVm> for IslandFive {
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
        // island_five_fitness_score_fn: run 100 games and score on fewest cards in face_up piles, then win rate
        let a_result = a.get_run_result().unwrap();
        let b_result = b.get_run_result().unwrap();
        let mut cmp = b_result
            .number_of_face_up_cards()
            .cmp(&a_result.number_of_face_up_cards());

        if Ordering::Equal == cmp {
            cmp = a_result.games_won().cmp(&b_result.games_won());
        }

        cmp
    }

    fn score_individual(&self, i: &pushgp::Individual<SolitaireResults>) -> u64 {
        5200 - i.get_run_result().unwrap().number_of_face_up_cards() as u64
    }

    fn clone(&self) -> Box<dyn IslandCallbacks<SolitaireResults, SolitareVm>> {
        Box::new(IslandFive {
            common: self.common.clone(),
        })
    }
}
