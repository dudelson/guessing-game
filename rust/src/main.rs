extern crate rand;

use std::io;
use std::io::prelude::*;
use rand::Rng;

/// Plays a single game of guess the number with the secret number to be guessed
/// provided as an argument, then returns
fn game_loop(secret: i32) {
    println!("I am thinking of a number between 1 and 100. Can you guess it?");
    loop {
        let mut guess_string = String::new();
        print!("Enter guess: ");
        io::stdout().flush().expect("flush stdout");
        let _u: usize = io::stdin()
            .read_line(&mut guess_string)
            .expect("read guess from stdin");
        let guess: i32 = guess_string.trim().parse().expect("convert guess to int");
        if guess == secret {
            println!("YOU GOT IT!!");
            return;
        } else if guess < secret {
            println!("too low");
        } else {
            println!("too high");
        }
    }
}

fn main() {
    loop {
        let secret: i32 = rand::thread_rng().gen_range(1, 101);
        game_loop(secret);
        let mut play_again_string = String::new();
        print!("Play again? (Y/n) ");
        io::stdout().flush().expect("flush stdout");
        io::stdin()
            .read_line(&mut play_again_string)
            .expect("read play again from stdin");
        if play_again_string.starts_with("n") || play_again_string.starts_with("N") {
            println!("Nice playing with you!");
            break;
        }
    }
}
