// Chapter 4
// Understading Ownership

fn main() {
    // This will be stored in stack part of the memory
    let str_example = "Hello world, I am immutable and stored in stack";
    // This will be stored in heap part of the memory
    let mut s = String::from("Hello");
    s.push_str(" world, I am mutable and stored in heap");
    println!("{}", str_example);
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("{}", s2);
    // Error that "the value moved" appears in compile time! Awesome :D
    // println!("{}", s1);

    let x = 5;
    let y = x;
    // This is ok, since the integer, float, bool, characters, and tuples with those types can
    // be copied automatically.
    println!("{}", y);

    // after this, some_string gets dropped,
    // so it unavailable to access some_string
    let some_string = String::from("Some String");
    takes_ownership(some_string);

    // we can still use some_integer after this,
    // since the copy of it has passed to the function
    let some_integer = 5;
    makes_copy(some_integer);

    // This will give ownership of value in function to this var
    let s3 = gives_ownership();
    println!("{}", s3);

    let s4 = String::from("hello");
    // This will take s4's ownership and move it to the s5
    let s5 = takes_and_gives_back(s4);
    println!("{}", s5);

    let s6 = String::from("Anything");
    let (s7, s7_len) = calculate_length(s6);
    println!("The length of '{}' is {}", s7, s7_len);

    // Above example had to return string in tuple,
    // But if we use & reference, we can make function to  borrow
    // the value, so no need to do unnecessary return process
    let s8 = String::from("Test Borrow");
    // Be careful, the borrowed value is immutable as default
    let s8_len = borrowing_length(&s8);
    println!("The length of '{}' is {}", s8, s8_len);

    // We can make mutable access for reference values
    let mut s9 = String::from("Hello");
    change_borrowed(&mut s9);
    println!("{}", s9);

    // But borrowing is often restricted with some rules
    // We can have as much as immutable reference as we want
    // But after immutable, we cannot borrow.
    // let mut s10 = String::from("Mutable String");
    let s10 = String::from("Mutable String");
    // let m3 = &mut s10;
    let m1 = &s10; // ok
    let m2 = &s10; // ok

    // But only one immutable reference, and it should also
    // be referenced first, or else we cannot reference it.
    println!("Original: {} / Reference: {}, {}", s10, m1, m2);

    // Dangle Nope!
    let s11 = no_dangle();
    println!("{}", s11);

    // Find the first word's length
    let mut s = String::from("Hoon Wee");
    let first_len = first_word_len(&s);
    println!("{}", first_len);
    // This will make s to empty string, ""
    s.clear();

    // We can use slice to ref the part in String
    let s12 = String::from("Rust Project");
    let part1 = &s12[0..4]; // From 0 til before 4
    let part2 = &s12[5..]; // Til the end
    println!("{} {}", part1, part2);
    // let part3 = &s12[..]; // All string
    // let part3 = &s12[..12]; // from the start

    // Real first word return function using slice!
    let s13 = String::from("Harry Potter");
    let fw = first_word(&s13);
    let s14 = "Potter"; // Actually string literals are Slices (&str)
    println!("{} {}", fw, s14);

    // We can use slices with other kinds of arrays
    let a1 = [1, 2, 3, 4, 5];
    let as1 = &a1[1..3];
    for a in as1 {
        println!("{}", a);
    }
}

// Data is dropped after escaping the scope
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn borrowing_length(s: &String) -> usize {
    s.len()
}

fn change_borrowed(some_string: &mut String) {
    some_string.push_str(" world!");
}

// This will make error for making dangling pointer
// fn dangle() -> &String {
//     let s = String::from("something");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("Something");
    s
}

fn first_word_len(s: &String) -> usize {
    // we will first make the String to Char Array, which each is a byte size.
    let bytes = s.as_bytes();
    // This will find the space before the second word, and return the index number.
    // enumerate() returns tuple (index, value)
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return index;
        }
    }
    // If we don't find the space, it means the give string is a word itself,
    // so we just return the length of it.
    s.len()
}

// It's better to pass String slice than string itself.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
