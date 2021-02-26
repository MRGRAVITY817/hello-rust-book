#[derive(Debug)]
pub struct Rectangle {
  length: u32,
  width: u32,
}

impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.length > other.length && self.width > other.width
  }
  pub fn is_too_big(&self) {
    if self.length > 10 {
      panic!("length is too big!");
    }
  }
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}

#[cfg(test)]
mod tests {
  // use all the crate resources
  use super::*;
  // To test this function, we use #[test] annotation
  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      length: 8,
      width: 7,
    };
    let smaller = Rectangle {
      length: 5,
      width: 3,
    };
    // assert! will check if the input returns the true boolean value
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_contain_larger() {
    let larger = Rectangle {
      length: 8,
      width: 7,
    };
    let smaller = Rectangle {
      length: 5,
      width: 3,
    };
    // we can make custom failure message with giving
    // message string as input
    assert!(
      !smaller.can_hold(&larger),
      "Smaller rectangle cannot contain larger one."
    );
  }
  #[test]
  // We can check whether the function emits panic with #[should_panic]
  // expected attribute will check the expected panic message
  #[should_panic(expected = "length is too big!")]
  fn should_panic_if_length_large() {
    let too_large = Rectangle {
      length: 19,
      width: 20,
    };
    too_large.is_too_big();
  }

  // using result<t,e> in tests
  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("Two plus two does not equal four"))
    }
  }
}
