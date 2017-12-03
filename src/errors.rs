#[derive(Debug)]
pub enum GameError {
    ParseError(String),
    BadGuess(String),
    InvalidSolution(String),
    GameOver,
}
