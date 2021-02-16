pub mod hashmap_tutorial {
  // We have to include hash map from stdio::collections
  use std::collections::HashMap;
  pub fn hashmap_basic() {
    // Init hashmap
    let mut scores = HashMap::new();
    // insert items
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 22);
    println!("{:?}", scores);

    // We can use vector tuple to make hashmap
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];
    let new_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", new_scores);
  }
  pub fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
  }
  pub fn access_update() {
    // we can iterate hash map with for loop
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 30);
    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }
    // overwrite value
    scores.insert(String::from("Blue"), 40);
    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }
    // Insert value if key has no value
    // Since key "Red" does not exist, it will insert with value 50
    scores.entry(String::from("Red")).or_insert(50);
    // key Blue exists, so this won't be updated
    scores.entry(String::from("Blue")).or_insert(50);
    for (key, value) in &scores {
      println!("{}: {}", key, value);
    }
    // Update value based on old value
    let text = "Hello World and World";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
      // return type of or_insert() is a reference of the value
      let count = map.entry(word).or_insert(0);
      // so we use dereference * to mutate it
      *count += 1;
    }
    println!("{:?}", map);
  }
}
