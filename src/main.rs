use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let repeated: String = "=".repeat(7);

    let secret = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret);

    println!("{}", repeated);
    println!("Guess the number!");
    println!("{}", repeated);

    loop {

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num, // Ok value contains parsed number
                Err(_) => {
                    println!("Invalid guess, please try a number!");
                    println!("{}", repeated);
                    continue; // _ is a catch all value for all error values
                }
            };                      // just continues the loop

        println!("{}", repeated);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
