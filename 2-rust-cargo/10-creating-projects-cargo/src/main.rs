use std::io;

fn main() {
    println!("Welcome to the guess game.!");
    println!("Your guess:");

    let mut buffer = String::new();
    let input = io::stdin();
    input.read_line(&mut buffer).expect("No input.");

    println!("Your guess: {}", buffer);
}
