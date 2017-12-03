use std::str::FromStr;

use errors::GameError;

#[derive(Debug)]
pub enum Command {
    TryLetter(char),
    TryWord(String),
    Info,
    Help,
    Quit,
}

impl FromStr for Command {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();

        macro_rules! err {
            () => { Err(GameError::ParseError(s.clone())) }
        }

        match s.chars().nth(0) {
            Some('q') => Ok(Command::Quit),
            Some('i') => Ok(Command::Info),
            Some('h') => Ok(Command::Help),
            Some('t') => {
                let words = s.split_whitespace().collect::<Vec<&str>>();

                let guess = match words.get(2) {
                    Some(word) => word,
                    None => return err!(),
                };

                let command = match words.get(1).and_then(|s| s.chars().nth(0)) {
                    Some(c) => c,
                    None => return err!(),
                };

                if command == 'l' {
                    if guess.chars().count() > 1 {
                        err!()
                    } else if let Some(c) = guess.chars().nth(0) {
                        Ok(Command::TryLetter(c))
                    } else {
                        err!()
                    }
                } else if command == 'w' {
                    Ok(Command::TryWord(String::from(*guess)))
                } else {
                    err!()
                }
            },
            _ => err!(),
        }
    }
}

