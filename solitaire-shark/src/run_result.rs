use crate::GameState;

#[derive(Clone, Debug)]
pub struct RunResult {
    games: Vec<GameState>,
}

impl RunResult {
    pub fn new() -> RunResult {
        RunResult { games: vec![] }
    }

    /// Saves a game as part of the RunResult for an individual
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
}
