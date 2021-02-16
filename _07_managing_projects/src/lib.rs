// Modules let you organize, scope well and keep path privacy
// Crates are tree of modules, for now we have lib.rs and main.rs,
// so we have one binary crate and a library crate.s
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    // super refers to the parent module of the current module,
    // which is crate(lib.rs) now.
    super::serve_order();
  }
  fn cook_order() {}
  // Making structs and enums public
  pub struct Breakfast {
    // we can designate publicity selectively
    pub toast: String,
    seasonal_fruit: String,
  }
  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast), // public user can access this part
        seasonal_fruit: String::from("peaches"), // but cannot access this one.
      }
    }
  }
  // Use public enums, unlike struct the variations will all be public
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

// We can use "use" keyword to abbreviate the namespace
// use with absolute path
use crate::front_of_house::hosting;
// use with relative path
// use self::front_of_house::hosting;
// Hashmap
use std::collections::HashMap;
// Providing new import name
use std::io::Result as IoResult;
// fn function1() -> IoResult<()> {}

// We can import from other file
mod middle_of_house; // the module name should be filename
                     // We can also re-export using 'pub use'
                     // which means it will now be available outside this code
pub use crate::middle_of_house::guesting;

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();
  // Relative path
  front_of_house::hosting::add_to_waitlist();
  // Followed by use keyword
  // Idiomatically we import hosting, not hosting::add_to_waitlist()
  // Because it's good to know where the function is from
  hosting::add_to_waitlist();
  // But for very common ones, we just bring the specific route
  let mut map = HashMap::new();
  map.insert(1, 2);

  let mut meal = back_of_house::Breakfast::summer("Rye");
  // you can change meal
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
  // You cannot do this, since the meal.seasonal_fruit is private.
  // meal.seasonal_fruit = String::from("blueberries");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}
