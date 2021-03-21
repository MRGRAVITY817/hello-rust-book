pub mod pattern_expressions {
  pub fn match_number(x: i32) {
    match x {
      1 | 2 => println!("One or two"),
      3..=5 => println!("Three to five"),
      _ => println!("anything"),
    }
  }
  pub fn match_chars(c: char) {
    match c {
      'a'..='j' => println!("early ASCII letter"),
      'k'..='z' => println!("late ASCII letter"),
      _ => println!("Something else"),
    }
  }
  pub fn match_if(x: i32, y: bool) {
    match x {
      3 | 4 | 5 if y => println!("yes"),
      _ => println!("no"),
    }
  }
  pub enum Message {
    Hello { id: i32 },
  }
  pub fn match_binding(m: &Message) {
    match m {
      Message::Hello {
        id: id_variable @ 3..=7,
      } => {
        println!("Found an id in range {}", id_variable)
      }
      Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
      }
      Message::Hello { id } => {
        println!("Found some other id: {}", id)
      }
    }
  }
  pub fn if_let_expression<T>(fav_color: Option<&str>, is_tues: bool, age: Result<u8, T>) {
    if let Some(color) = fav_color {
      println!("Using your favorite color, {}, as the background", color)
    } else if is_tues {
      println!("Tuesday is green day!")
    } else if let Ok(age) = age {
      if age > 30 {
        println!("Using purple as the background color");
      } else {
        println!("Using orange as the background color");
      }
    } else {
      println!("Using blue as the background color");
    }
    // Downsides of if let expression is that the compiler doesn't
    // check exhaustiveness, whereas match expression does.
  }
  pub fn stack_example(num: i32) {
    let mut stack = Vec::new();
    for i in 1..num {
      stack.push(i);
    }
    while let Some(top) = stack.pop() {
      print!("{} ", top);
    }
    println!("");
  }
  pub fn print_iter(v: &Vec<char>) {
    for (index, value) in v.iter().enumerate() {
      println!("{} is at index {}", value, index);
    }
  }
  pub fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
  }
}
