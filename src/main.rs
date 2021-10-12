use std::io;

fn main() {
    let repeated: String = "=".repeat(7);
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
