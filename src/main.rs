extern crate hangman;

use hangman::game::Game;
use hangman::input::Command;
use hangman::wordlist::Wordlist;
use hangman::tui;

use std::io::{self, Write, BufReader};
use std::fs::File;

fn main() {
    let file = File::open("words.txt").expect("Couldn't open words.txt");
    let reader = BufReader::new(file);
    let wordlist = Wordlist::from_io(reader);

    let mut game = Game::new(wordlist.random(), 10).unwrap();
    let stdin = io::stdin();

    tui::clear_screen();
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

        tui::clear_screen();

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
