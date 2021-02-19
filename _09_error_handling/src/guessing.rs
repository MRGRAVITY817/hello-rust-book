pub mod custom_type {
  use rand::Rng;
  use std::cmp::Ordering;
  use std::io;
  pub fn guessing_game() {
    // Encapsulate Guess input with struct
    pub struct Guess {
      value: i32,
    }
    impl Guess {
      // Associated Function / constructor to validate input
      pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
          panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
      }
      // Getter method
      pub fn value(&self) -> i32 {
        self.value
      }
    }
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
      println!("Please input your guess. ");
      let mut guess = String::new();
      io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      // Advanced error handling using custom type validation
      let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue, // continue will go to start point of this loop
      };

      if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100");
        continue;
      }

      println!("You guessed: {}", guess);
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
}
