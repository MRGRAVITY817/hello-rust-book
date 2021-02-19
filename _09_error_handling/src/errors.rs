pub mod errors_tutorial {
  use std::fs::File;
  use std::io::{self, ErrorKind, Read};

  pub fn basic_panic() {
    // We use panic for Unrecoverable errors
    // panic!("Crash and burn!");

    // This opens file, and when having error then,
    // 1. if it's NotFound error, we make the new file with name hello.txt
    // 2. if other errors, we just panic the problem opening file.
    // and when error occurs while creating the file, we also omit panic.
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
          panic!("Problem opening the file: {:?}", error);
        })
      } else {
        panic!("Problem opening the file: {:?}", error);
      }
    });

    println!("{:#?}", _f);

    // we can make panic with message with expect()
    let _f2 = File::open("world.txt").expect("Failed to open world.txt");
  }

  pub fn propagating() -> Result<String, io::Error> {
    // We sometimes want to do with error, not just ending the program.
    let f = File::open("hello.txt");
    let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
    }
  }

  pub fn question_operator() -> Result<String, io::Error> {
    // ? operator can abbreviate the match{ok, err} part
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
  }
}
