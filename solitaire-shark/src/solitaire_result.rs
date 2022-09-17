use crate::GameState;

#[derive(Clone, Debug)]
pub struct SolitaireResults {
    games: Vec<GameState>,
}

impl pushgp::RunResult for SolitaireResults {}

impl SolitaireResults {
    pub fn new() -> SolitaireResults {
        SolitaireResults { games: vec![] }
    }

    /// Saves a game as part of the results for an individual
    pub fn save_game(&mut self, game: GameState) {
        self.games.push(game);
    }

    /// Counts the number of games where all cards are in the finished piles
    pub fn games_won(&self) -> usize {
        let mut number_won = 0;
        for game in self.games.iter() {
            if game.number_of_finished_cards() == 52 {
                number_won += 1;
            }
        }

        number_won
    }

    /// Counts the number of finished cards across all runs
    pub fn number_of_finished_cards(&self) -> usize {
        let mut number_finished = 0;
        for game in self.games.iter() {
            number_finished += game.number_of_finished_cards();
        }

        number_finished
    }

    pub fn number_of_draw_stack_cards(&self) -> usize {
        self.games
            .iter()
            .map(|game| game.number_of_cards_in_draw_pile() + game.number_of_cards_in_play_pile())
            .sum()
    }

    pub fn number_of_face_down_cards(&self) -> usize {
        self.games
            .iter()
            .map(|game| {
                game.number_of_face_down_cards_in_work_pile(0)
                    + game.number_of_face_down_cards_in_work_pile(1)
                    + game.number_of_face_down_cards_in_work_pile(2)
                    + game.number_of_face_down_cards_in_work_pile(3)
                    + game.number_of_face_down_cards_in_work_pile(4)
                    + game.number_of_face_down_cards_in_work_pile(5)
                    + game.number_of_face_down_cards_in_work_pile(6)
            })
            .sum()
    }

    pub fn number_of_face_up_cards(&self) -> usize {
        self.games
            .iter()
            .map(|game| {
                game.number_of_face_up_cards_in_work_pile(0)
                    + game.number_of_face_up_cards_in_work_pile(1)
                    + game.number_of_face_up_cards_in_work_pile(2)
                    + game.number_of_face_up_cards_in_work_pile(3)
                    + game.number_of_face_up_cards_in_work_pile(4)
                    + game.number_of_face_up_cards_in_work_pile(5)
                    + game.number_of_face_up_cards_in_work_pile(6)
            })
            .sum()
    }
}
