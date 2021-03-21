pub mod ignorance_example {
  pub fn ignore_value(_: i32, y: i32) {
    println!("I am only using y: {} value", y)
  }
  pub fn ignore_parts_2(numbers: &(i32, i32, i32, i32, i32)) {
    match numbers {
      (first, _, third, _, fifth) => {
        println!("Some number: {} {} {}", first, third, fifth)
      }
    }
  }
  pub fn unused_variable(x: i32, y: i32) {
    let _x = x;
    let y = y;
    println!("_x is unused {}", _x)
  }
  pub fn ignore_binding(s: Option<String>) {
    if let Some(_) = s {
      println!("Found a string");
    }
    // The underscore inside Some(_) will not take the ownership
    // since _ is ignored to bind the value in Rust
    println!("{:?}", s);
  }
  pub fn spread_ignore(numbers: &(i32, i32, i32, i32, i32)) {
    match numbers {
      (first, .., last) => {
        println!("Some numbers: {} {}", first, last)
      }
    }
  }
}
