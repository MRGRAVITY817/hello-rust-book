pub mod string_tutorial {
  pub fn string_basic() {
    // INIT string
    let s1 = String::new();
    let mut s1 = String::from("foo");
    // push string part
    s1.push_str("bar");
    // push character
    s1.push('!');
    println!("{}", &s1);
  }
  pub fn concat_string() {
    // Concat string
    let s2 = String::from("Hello ");
    let s3 = String::from("World");
    // Concatenating string to s4 is like
    // push_str
    let s4 = s2 + &s3; // s2 no longer usable
    println!("{}", s4);
    // Use format macro
    let harry = String::from("Harry");
    let james = String::from("James");
    let potter = String::from("Potter");
    let full_name = format!("{} {} {}", harry, james, potter);
    println!("{}", full_name);
  }
}
