// game based on how many time you guess untill y get the correct value
// once u right your guess the robot will guide u if corret or less/greater than the win.
use std::io;

fn main() {
    println!("Welcome to the guessing game");
    println!("please enter your guess :");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read");

    println!("your guess: {}", guess);
}
