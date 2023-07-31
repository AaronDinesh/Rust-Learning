use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    
    loop {
        println!("Input your guess: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        print!("You guessed {guess}\n");

        match guess.cmp(&secret){
            Ordering::Less => println!("Too small\n"),
            Ordering::Equal => {
                print!("You win\n");
                break;
            },
            Ordering::Greater => println!("Too big\n"),
        };    
    }


}
