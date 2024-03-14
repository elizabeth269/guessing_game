use rand::Rng;
use std::io;

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to red line");
    println!("you guessed: {guess}");
}
//enum is a type with many state.
//each possible state is a variant
//a crate is a collection of Rust source code files.
