use adder;

// in test directory, you don't need to
// annotate #[config(test)]
#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}
