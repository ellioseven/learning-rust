extern crate rand;

use rand::random;
use std::io;

fn get_guess() -> u8 {
    loop {
        let mut buffer = String::new();
        let input = io::stdin();
        println!("Your guess: {}", buffer);
        input.read_line(&mut buffer).expect("No input.");
        match buffer.trim().parse() {
            Ok(v) => return v,
            Err(e) => println!("Couldn't understand input: {}", e)
        }
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess > correct {
        println!("You are too high.");
        return false;
    } else if guess < correct {
        println!("You are too low");
        return false;
    }

    println!("You got it!");
    return true;
}

fn main() {
    println!("Welcome to the guess game.!");

    let correct = random();
    
    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}
