pub mod generic_tutorial {
  pub fn what_is_generic() {
    // We sometimes want to reduce duplicated codes
    // caused by condition-for-all-types programming
    struct Point<T> {
      x: T, // We annotate generic type in angled brackets,
      y: T,
    }
    let will_work = Point { x: 1.0, y: 2.0 };
    println!("x: {}, y: {}", will_work.x, will_work.y);
    // x,y has to have same type
    // let wont_work = Point { x: 1, y: "hello" };

    // If you want to use different types for x and y,
    // you should annotate two generic types
    struct EulerPoint<T, U> {
      x: T,
      y: U,
    }
    // You can use generic in methods
    impl<T, U> EulerPoint<T, U> {
      // getter for x and y
      fn x(&self) -> &T {
        &self.x
      }
      fn y(&self) -> &U {
        &self.y
      }
    }
    let now_will_work = EulerPoint { x: 1, y: "hello" };
    println!("x: {}, y: {}", now_will_work.x(), now_will_work.y());
  }
}
