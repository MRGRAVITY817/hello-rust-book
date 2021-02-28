pub mod closures_tutorial {
  use std::thread;
  use std::time::Duration;

  // Caching is functionality that you store data for function return value
  // which is really useful when the function you call is resource intensive
  struct Cacher<T>
  where
    T: Fn(u32) -> u32,
  {
    calculation: T,
    // Once the value is cached, we can only use that value.
    value: Option<u32>,
  }

  impl<T> Cacher<T>
  where
    T: Fn(u32) -> u32,
  {
    // constructor
    fn new(calculation: T) -> Cacher<T> {
      Cacher {
        calculation,
        value: None,
      }
    }
    // value caching
    fn value(&mut self, arg: u32) -> u32 {
      match self.value {
        Some(v) => v,
        None => {
          let v = (self.calculation)(arg);
          self.value = Some(v);
          v
        }
      }
    }
  }

  pub fn generate_workout(intensity: u32, random_number: u32) {
    // Using Cacher, we won't call the function after it's called.
    let mut expensive_result = Cacher::new(|num| {
      println!("calculating slowly..");
      thread::sleep(Duration::from_secs(2));
      num
    });
    if intensity < 25 {
      println!("Today, do {} pushups!", expensive_result.value(intensity));
      println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
      if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
      } else {
        println!(
          "Today, run for {} minutes!",
          expensive_result.value(intensity)
        );
      }
    }
  }
}
