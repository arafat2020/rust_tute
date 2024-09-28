use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number!");
        println!("Please Enter a number");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Your guess is {guess}");
        let guess: u32 = guess.trim().parse().expect("Failed to perse number");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You Guessed write!");
                break
            },
        }
    }
}
