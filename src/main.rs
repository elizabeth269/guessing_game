use std::io;

fn main() {
    println!("guess the number!");
    println!("please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to red line");
    println!("you guessed: {guess}");

    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
//enum is a type with many state.
//each possible state is a variant
//a crate is a collection of Rust source code files.
