use std::str::FromStr;
use std::io::BufReader;
use std::fs::File;
use std::env;

use wordlist::Wordlist;
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

pub fn get_wordlist(wordlist_path: Option<String>) -> Result<Wordlist, GameError> {
    let mut wordlist = Wordlist::new();

    // Try to load wordlist file from args
    if let Some(filename) = wordlist_path {
        let args_file = File::open(&filename)?;
        let reader = BufReader::new(args_file);
        println!("DEBUG: Loading wordlist from: {:?}", filename);
        wordlist.load_io(reader);
    }

    // Try to load ~/.hangman_words.txt
    if let Some(mut home_file_path) = env::home_dir() {
        home_file_path.push(".hangman_words.txt");

        if let Ok(home_file) = File::open(&home_file_path) {
            println!("DEBUG: Loading wordlist from: {:?}", home_file_path);
            let reader = BufReader::new(home_file);
            wordlist.load_io(reader);
        }
    }

    // If nothing has been loaded, fall back to some built-ins
    if wordlist.is_empty() {
        println!("DEBUG: Nothing else loaded, loading wordlist from built-in words.txt");
        let reader = BufReader::new(include_str!("../words.txt").as_bytes());
        wordlist.load_io(reader)
    }

    if wordlist.is_empty() {
        return Err(GameError::EmptyWordlist);
    }

    Ok(wordlist)
}
