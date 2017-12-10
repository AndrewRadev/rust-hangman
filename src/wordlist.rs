extern crate rand;

use std::io::BufRead;
use self::rand::Rng;

pub struct Wordlist {
    words: Vec<String>,
}

impl Wordlist {
    pub fn new() -> Self {
        Wordlist { words: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.words.len() == 0
    }

    pub fn from_io<T: BufRead>(io: T) -> Self {
        let mut wordlist = Wordlist::new();
        wordlist.load_io(io);
        wordlist
    }

    pub fn load_io<T: BufRead>(&mut self, io: T) {
        let new_words = io.lines().
            // unwrap results:
            map(|res| res.expect("Couldn't read a line from the given IO")).
            // remove extra whitespace:
            map(|l| l.trim().to_string()).
            // remove non-words:
            filter(|l| l.chars().all(char::is_alphabetic)).
            // remove empty lines:
            filter(|l| l.len() > 0).
            collect::<Vec<String>>();

        self.words.extend(new_words);
    }

    pub fn get(&self, n: usize) -> Option<&String> {
        self.words.get(n)
    }

    pub fn random(&self) -> &String {
        if self.words.len() == 0 {
            panic!("Attempted to get a random element from an empty wordlist!");
        }

        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0, self.words.len());

        &self.words[n]
    }
}
