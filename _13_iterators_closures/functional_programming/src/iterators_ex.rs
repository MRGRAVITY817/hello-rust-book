pub mod iterators_tutorial {
  // Return type of the next() is Option<> type
  pub fn iter_next<'a>(v1: &'a Vec<i32>) -> Vec<Option<&'a i32>> {
    // To trace iteration, we make it mutable
    let mut v1_iter = v1.iter();
    let mut next_vector = Vec::new();
    v1_iter.next();
    for _ in v1.iter() {
      next_vector.push(v1_iter.next());
    }
    next_vector
  }
  // Return type of the sum should be i32
  pub fn iter_sum<'a>(v1: &'a Vec<i32>) -> i32 {
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    total
  }
  pub fn iter_map(v1: &[i32]) -> Vec<i32> {
    // using map(CLOSURE), we can make a new vector with each elements processed
    // by input closure function.
    v1.iter().map(|x| x + 1).collect()
    // When we just use map, it doesn't actually do returns.
    // When we want to get returned value, use collect().
  }
  #[derive(Debug, PartialEq)]
  pub struct Shoe {
    pub size: u32,
    pub style: String,
  }
  // filter() creates an new iterated object filtered by given conditional closure
  // or should we say, the Predicate.
  pub fn iter_filter(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() creates an iterator that takes an ownership of the given vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
  }
  pub struct Counter {
    count: u32,
  }
  impl Counter {
    pub fn new() -> Counter {
      Counter { count: 0 }
    }
  }
  // We can make custom iterator(s) for struct
  impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
      self.count += 1;
      if self.count < 6 {
        Some(self.count)
      } else {
        None
      }
    }
  }
}
