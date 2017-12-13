extern crate hangman;

use hangman::game::Game;
use hangman::input::{self, Command};
use hangman::tui;

use std::io::{self, Write};

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="hangman", about="A game of Hangman")]
pub struct Options {
    #[structopt(short="w", long="wordlist", help="The path to a word list")]
    wordlist_path: Option<String>,

    #[structopt(short="a", long="attempts", help="The number of attempts to guess the word", default_value="10")]
    attempts: u32,

    #[structopt(short="d", long="debug", help="Show debug info")]
    debug: bool,
}

fn main() {
    let options = Options::from_args();
    println!("{:?}", options);

    let wordlist = match input::get_wordlist(options.wordlist_path) {
        Ok(w) => w,
        Err(e) => {
            println!("\n{}", e);
            std::process::exit(1);
        }
    };

    let mut game = Game::new(wordlist.random(), options.attempts).unwrap();
    let stdin = io::stdin();

    tui::clear_screen(options.debug);
    println!("Let's play hangman!");

    loop {
        println!("{}", game);

        if game.is_over() {
            break;
        }

        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            println!("\nInvalid input!");
            continue;
        };

        let command = match input.parse::<Command>() {
            Ok(c) => c,
            Err(e) => {
                println!("\n{}\n", e);
                continue;
            }
        };

        tui::clear_screen(options.debug);

        match command {
            Command::TryLetter(c)  => tui::print_guess_response(game.guess_letter(c)),
            Command::TryWord(word) => tui::print_guess_response(game.guess_word(&word)),
            Command::Help          => tui::print_help(),
            Command::Info          => tui::print_info(&game),
            Command::Quit          => break,
        };

        println!("");
    }

    println!("Good game!");
}
