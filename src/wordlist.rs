use std::io::BufRead;

pub struct Wordlist {
    words: Vec<String>,
}

impl Wordlist {
    pub fn from_io<T: BufRead>(io: T) -> Self {
        let words = io.
            lines().
            filter_map(Result::ok).
            map(|l| l.trim().to_string()).
            collect();

        Wordlist { words }
    }

    pub fn get(&self, n: usize) -> Option<&String> {
        self.words.get(n)
    }
}
