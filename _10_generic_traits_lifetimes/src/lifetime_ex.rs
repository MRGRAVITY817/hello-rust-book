pub mod lifetime_tutorial {
  use std::fmt::Display;
  // Introduction to lifetime
  pub fn what_is_lifetime() {
    // if we pass & reference as parameter, we need to tell the function
    // about lifetime. or else, it won't know what lifetime standards
    // should it consider
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    // we can make the trait implement cleaner using "where".
    where
      T: Display,
    {
      println!("Announcement! {}", ann);
      if x.len() > y.len() {
        x
      } else {
        y
      }
    }
    let s1 = String::from("Hello world");
    let s2 = String::from("See you again bros!");
    let s3 = longest_with_an_announcement(&s1, &s2, "Look here");
    println!("{} is longer!", s3);
  }
  // lifetime with struct
  pub fn lifetime_with_struct() {
    struct ImportantExcerpt<'a> {
      part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
      fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention Please: {}", announcement);
        self.part
      }
    }
    let novel = String::from("Call me Hoon. Some call me Tripboi.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
      part: first_sentence,
    };
    println!("{}", i.announce_and_return_part("the first part is"));
  }
}
