use std::fmt::{self, Display, Write};

use crate::errors::GameError;
use crate::game::{Game, GameState};

impl Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GameError::IoError(ref e) => {
                f.write_fmt(format_args!("{}", e))
            }
            &GameError::ParseError(ref input) => {
                f.write_fmt(format_args!("Invalid command: {:?}. ", &input))?;
                f.write_str("Try the 'help' command for a list of valid commands")
            }
            &GameError::BadGuess(ref message) => {
                f.write_str(&message)
            },
            &GameError::InvalidSolution(ref word) => {
                f.write_fmt(format_args!("Invalid word given for solution: {:?}. ", &word))?;
                f.write_str("Needs to consist of alphabetical letters only")
            },
            &GameError::GameOver => {
                f.write_str("Game is already over!")
            },
            &GameError::EmptyWordlist => {
                f.write_str("Couldn't load any valid wordlist!\n")?;
                f.write_str("Searched locations:\n")?;
                f.write_str("  - ~/.hangman_words.txt\n")?;
                f.write_str("  - A file provided via `-w <filename>`\n")?;
                f.write_str("\n")?;
                f.write_str("Note that a wordlist needs to consist of at least one valid word\n")?;
                f.write_str("(made up of alphabetical characters only)\n")?;
                Ok(())
            },
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut chars = self.solution.chars();

        match self.state {
            GameState::Won => {
                return f.write_fmt(format_args!("You won! ^_^\nThe word was: {}", self.solution))
            },
            GameState::Lost => {
                return f.write_fmt(format_args!("You lost! :/\nThe word was: {}", self.solution))
            },
            GameState::InProgress => f.write_fmt(format_args!("Guesses remaining: {}\n", self.attempts_remaining))?,
        }

        if let Some(letter) = chars.next() {
            self.fmt_letter(f, letter)?;

            while let Some(letter) = chars.next() {
                f.write_char(' ')?;
                self.fmt_letter(f, letter)?;
            }
        }

        Ok(())
    }
}

impl Game {
    fn fmt_letter(&self, f: &mut fmt::Formatter, letter: char) -> fmt::Result {
        if self.attempted_letters.contains(&letter) {
            f.write_char(letter)
        } else {
            f.write_char('_')
        }
    }
}

pub fn print_help() {
    println!("");
    println!("Valid commands:");
    println!("  - help            => Show these instructions");
    println!("  - info            => Information about attempted words/letters");
    println!("  - quit            => Quit the game");
    println!("  - try letter <c>  => Try this letter");
    println!("  - try word <word> => Try an entire word");
    println!("");
}

pub fn print_info(game: &Game) {
    println!("Information about the state of the game:");
    println!("  - Attempted letters: {:?}", game.attempted_letters);
    println!("  - Attempted words:   {:?}", game.attempted_words);
}

pub fn print_guess_response(response: Result<bool, GameError>) {
    match response {
        Ok(true) => println!("Good guess!"),
        Ok(false) => println!("Sorry, no dice"),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn clear_screen(debug: bool) {
    if debug { return; }

    print!("{}[2J", 27 as char);
    print!("{}[1;1H", 27 as char);
}
