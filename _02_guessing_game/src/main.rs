use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess. ");
        // "let" is variable
        // in rust, variables are basically immutable,
        // so if we want it mutable, we use "mut"
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // & is reference, which lets you.. be awesome :D
            .expect("Failed to read line"); // if read_line(io::Result type) returns no ok

        // When we type 5, it will actually recieve '5\n',
        // so we first trim the last \n part, then we parse string to int
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Error handling using match
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue will go to start point of this loop
        };

        println!("You guessed: {}", guess);
        // "match" will do right function when guess.cmp(&secret_number) matches left expression
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
