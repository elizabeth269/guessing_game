use rand::Rng;
use std::cmp::Ordering; //ordering is an enum that has a vairant less,greater and equal...beacus eyou are comparing values
use std::io; //enum that has variant ok(success) or err(tell you why it failed)

fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");
    loop {
        println!("please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to red line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        //comparing values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
//enum is a type with many state.
//each possible state is a variant
//a crate is a collection of Rust source code files.
