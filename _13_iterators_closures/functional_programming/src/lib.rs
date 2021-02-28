mod iterators_ex;
use iterators_ex::iterators_tutorial;
use iterators_ex::iterators_tutorial::Counter;
use iterators_ex::iterators_tutorial::Shoe;

#[cfg(test)]
pub mod tests {
  use super::*;
  #[test]
  fn iter_next_test() {
    let v1 = vec![1, 2, 3, 4];
    assert_eq!(
      iterators_tutorial::iter_next(&v1),
      vec![Some(&2), Some(&3), Some(&4), None]
    );
  }
  #[test]
  fn iter_sum_test() {
    let v1 = vec![1, 2, 3, 4];
    assert_eq!(iterators_tutorial::iter_sum(&v1), 10)
  }
  #[test]
  fn iter_map_collect_test() {
    let v1 = vec![1, 2, 3, 4];
    assert_eq!(iterators_tutorial::iter_map(&v1), vec![2, 3, 4, 5]);
  }
  #[test]
  fn filters_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 15,
        style: String::from("sandal"),
      },
      Shoe {
        size: 10,
        style: String::from("boots"),
      },
    ];
    let in_my_size = iterators_tutorial::iter_filter(shoes, 10);
    assert_eq!(
      in_my_size,
      vec![
        Shoe {
          size: 10,
          style: String::from("sneaker"),
        },
        Shoe {
          size: 10,
          style: String::from("boots"),
        },
      ]
    )
  }
  #[test]
  fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
  }
  #[test]
  fn using_other_iterator_trait_methods() {
    // skip() skips the given order of index
    let sum: u32 = Counter::new()
      // zip() will make a pair (original, new) for original.zip(new)
      .zip(Counter::new().skip(1))
      .map(|(a, b)| a * b)
      .filter(|x| x % 3 == 0)
      .sum();
    assert_eq!(18, sum);
  }
}
