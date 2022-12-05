// game based on how many time you guess untill y get the correct value
// once u right your guess the robot will guide u if corret or less/greater than the win.

use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");
    println!("please enter your guess :");

    let secret_guess = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read, please try again");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess: {}", guess);

        match guess.cmp(&secret_guess) {
            Ordering::Equal => println!("Great, you win!"),
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => {
                println!("Too big");
                break;
            }
        }
    }
}
