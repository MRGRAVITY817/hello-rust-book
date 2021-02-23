pub mod trait_tutorial {
  use std::fmt::Display;
  pub fn what_is_trait() {
    struct Pair<T> {
      x: T,
      y: T,
    }
    // constructor
    impl<T> Pair<T> {
      // "Self" returns the object itself
      fn new(x: T, y: T) -> Self {
        Self { x, y }
      }
    }
    // By using traits, we can restrict the use of this methods
    // only if the T type satisfies the Display and Partial order traits
    impl<T: Display + PartialOrd> Pair<T> {
      fn cmp_display(&self) {
        if self.x >= self.y {
          println!("The largest member is x = {}", self.x);
        } else {
          println!("The largest member is y = {}", self.y);
        }
      }
    }
    // test
    let new_pair = Pair::new(10, 20);
    new_pair.cmp_display();
  }
  pub fn trait_largest() {
    // Since the items in list[T] should be ordered,
    // and we will return the copy of the element,
    // T should have "PartialOrd" and "Copy" trait
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
      let mut largest = list[0];
      for &item in list.iter() {
        if item > largest {
          largest = item;
        }
      }
      largest
    }
    // test
    let number_list = vec![35, 23, 11, 55];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
  }
}
