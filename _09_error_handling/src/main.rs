// custom crates
mod errors;
mod guessing;

use std::error::Error;
use std::fs::File;
// custom mods
use errors::errors_tutorial;
use guessing::custom_type;

// Since main function is special, we use this kind of Result type
fn main() -> Result<(), Box<dyn Error>> {
    // errors_tutorial::basic_panic();
    let prop = errors_tutorial::propagating();
    println!("{:?}", prop);
    let prop = errors_tutorial::question_operator();
    println!("{:?}", prop);
    // We can use custom types of validation
    custom_type::guessing_game();
    // ? operator can only be used for "Result" type
    let f = File::open("world.txt")?;
    Ok(())
}
