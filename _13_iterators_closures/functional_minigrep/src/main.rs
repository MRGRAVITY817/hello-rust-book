use functional_minigrep::Config;
use std::env; // for get environment args
use std::process; // to print in stderr for terminal

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprint is used to print specifically for errors to stderr
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = functional_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
