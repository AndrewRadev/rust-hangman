use std::io::{self, Write};
use structopt::StructOpt;

use hangman::game::Game;
use hangman::input::{self, Command, Options};
use hangman::tui;

fn main() {
    let options = Options::from_args();
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
