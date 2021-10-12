use std::io;
use rand::Rng;

fn main() {
    let repeated: String = "=".repeat(7);

    let secret = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret);

    println!("{}", repeated);
    println!("Guess the number!");
    println!("{}", repeated);

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("{}", repeated);
    println!("You guessed: {}", guess);
}
