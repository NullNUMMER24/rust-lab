use std::io;
use std::cmp::Ordering;
use rand::Rng; // 0.8.5

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    // Parse input after trimming newline characters
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("The secret number is: {secret_number}");

    // Compare numeric values instead of strings
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! - you loose!"),
        Ordering::Greater => println!("Too big! - you loose!"),
        Ordering::Equal => println!("You win!"),
    }
}