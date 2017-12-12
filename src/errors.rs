use std::io;
use std::convert::From;

#[derive(Debug)]
pub enum GameError {
    IoError(io::Error),
    ParseError(String),
    BadGuess(String),
    InvalidSolution(String),
    GameOver,
    EmptyWordlist,
}

impl From<io::Error> for GameError {
    fn from(e: io::Error) -> Self {
        GameError::IoError(e)
    }
}
