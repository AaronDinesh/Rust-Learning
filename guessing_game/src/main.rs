use std::io;
use rand::{self, Rng}:Rng;

fn main() {
    println!("Guess the number!");
    let mut secret = rand::thread_rng().gen_range(1..=100);
    
    println!("Input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    print!("You guessed {guess}");
}
