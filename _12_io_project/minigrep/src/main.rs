use minigrep::Config;
use std::env; // for get environment args
use std::process; // to print in stderr for terminal

fn main() {
    let args: Vec<String> = env::args().collect();
    // env.args[0] is a project name, so user input args starts from index 1.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprint is used to print specifically for errors to stderr
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
