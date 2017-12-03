use std::collections::HashSet;
use std::iter::FromIterator;

use errors::GameError;

pub enum GameState {
    InProgress,
    Won,
    Lost,
}

pub struct Game {
    pub attempted_letters: HashSet<char>,
    pub attempted_words: HashSet<String>,
    pub attempts_remaining: u32,
    pub state: GameState,
    pub solution: String,
    pub solution_letters: HashSet<char>
}

impl Game {
    pub fn new(solution: &str, attempts: u32) -> Result<Self, GameError> {
        if solution.is_empty() {
            return Err(GameError::InvalidSolution(String::new()))
        }

        if !solution.chars().all(char::is_alphabetic) {
            return Err(GameError::InvalidSolution(String::from(solution)))
        }

        let solution = String::from(solution);
        let attempted_letters = HashSet::new();
        let attempted_words = HashSet::new();
        let solution_letters = HashSet::from_iter(solution.chars());

        Ok(Game {
            solution, solution_letters,
            attempted_letters, attempted_words,

            attempts_remaining: attempts,
            state: GameState::InProgress,
        })
    }

    pub fn guess_letter(&mut self, guess: char) -> Result<bool, GameError> {
        if self.is_over() {
            return Err(GameError::GameOver);
        }

        if self.attempted_letters.contains(&guess) {
            return Err(GameError::BadGuess(String::from("already attempted this letter!")));
        }

        self.attempted_letters.insert(guess);

        if self.attempted_letters.is_superset(&self.solution_letters) {
            self.state = GameState::Won;
            Ok(true)
        } else if self.solution.chars().find(|c| *c == guess).is_some() {
            Ok(true)
        } else {
            self.attempts_remaining -= 1;

            if self.attempts_remaining == 0 {
                self.state = GameState::Lost;
            }

            Ok(false)
        }
    }

    pub fn guess_word(&mut self, guess: &str) -> Result<bool, GameError> {
        if self.is_over() {
            return Err(GameError::GameOver);
        }

        if self.attempted_words.contains(guess) {
            return Err(GameError::BadGuess(String::from("already attempted this word!")));
        }

        self.attempted_words.insert(String::from(guess));

        if guess == self.solution {
            self.state = GameState::Won;
            Ok(true)
        } else {
            self.attempts_remaining -= 1;

            if self.attempts_remaining == 0 {
                self.state = GameState::Lost;
            }

            Ok(false)
        }
    }

    pub fn is_over(&self) -> bool {
        match self.state {
            GameState::InProgress => false,
            _                     => true,
        }
    }
}
