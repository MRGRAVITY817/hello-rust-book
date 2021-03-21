pub mod destruct_examples {
  pub struct Point {
    x: i32,
    y: i32,
  }
  impl Point {
    pub fn new(x: i32, y: i32) -> Point {
      Point { x, y }
    }
  }
  pub fn destruct_struct(p: &Point) {
    match p {
      Point { x, y: 0 } => println!("On the x axis at {}", x),
      Point { x: 0, y } => println!("On the y axis at {}", y),
      Point { x, y } => println!("Not on the axis, on point ({}, {})", x, y),
    }
  }
  pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }
  pub fn destruct_enum(m: &Message) {
    match m {
      Message::Quit => {
        println!("The Quit variant has no data to destruct.")
      }
      Message::Move { x, y } => {
        println!("Move in the x direction {} and in the y direction {}", x, y)
      }
      Message::Write(text) => {
        println!("Text message: {}", text)
      }
      Message::ChangeColor(r, g, b) => {
        println!("Change the color to red {}, green {}, blue {}", r, g, b)
      }
    }
  }
  pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }
  pub enum Palette {
    Blank,
    Move { x: i32, y: i32 },
    ChangeColor(Color),
  }
  pub fn destruct_nested_enum(palette: &Palette) {
    match palette {
      Palette::ChangeColor(Color::Rgb(r, g, b)) => {
        println!("Change the color to red {}, green {}, blue {}", r, g, b);
      }
      Palette::ChangeColor(Color::Hsv(h, s, v)) => {
        println!(
          "Change the color to hue {}, saturation {}, value{}",
          h, s, v
        );
      }
      _ => (),
    }
  }
}
