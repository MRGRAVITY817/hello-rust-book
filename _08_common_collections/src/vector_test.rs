pub mod vector_tutorial {
  pub fn indexing() {
    // Vectors can store more than one value in single data structure
    let v = vec![1, 2, 3, 4, 5];
    // indexing vector using []
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // indexing vector using .get(), which returns Options<T>
    // if we used &v[100], it will spit error, but using .get,
    // we get None value.
    match v.get(100) {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
    }
  }
  pub fn mutable_iterating() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    let first = &v[0];
    // v.push(7); <- Can't do this since v is borrowed immutable previously.
    println!("The first element is {}", first);

    let newV = vec![1, 2, 3, 4];
    // iterating vector
    for i in &newV {
      print!("{} ", i);
    }
    let mut mutV = vec![3, 4, 5, 6];
    // iterate and mutate vector
    for i in &mut mutV {
      *i += 50;
    }
    println!("{:?}", &mutV);
  }
  pub fn enum_vectors() {
    #[derive(Debug)]
    enum CharacterInfo {
      Height(u32),
      Name(String),
      Strength(f64),
    }
    // We can make various typed vector using enum
    let row = vec![
      CharacterInfo::Height(172),
      CharacterInfo::Name(String::from("Hoon")),
      CharacterInfo::Strength(11122.22),
    ];
    println!("{:?}", &row);
  }
}
