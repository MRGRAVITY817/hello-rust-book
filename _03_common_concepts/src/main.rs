fn main() {
    // Variables and mutabillity
    let x = 5; // immutable
    let mut y = 9; // mutable
                   // Constants
                   // Normally use upper case
    const MAX_POINTS: u32 = 12;
    // Shadowing
    let x = x + 1; // This overwrites the x
    y = y + 1; // this mutates the y
               // Length
    let spaces_1 = "   ";
    let spaces_1 = spaces_1.len();
    let mut spaces_2 = String::from("Hello");
    spaces_2.push_str(" World");
    // spaces_2 = spaces_2.len(); <- THis is impossible since the type is not same
    println!("{} {} {} {} {}", x, y, MAX_POINTS, spaces_1, spaces_2);

    // There are types like scalar types(integer, boolean, float, character)
    // and also compound types(tuple, array, etc)

    // Tuple should be declared with inner types
    let tup: (i32, f64, u8) = (500, 10.4, 1);
    let (a, b, c) = tup;
    println!("{} {} {}", a, b, c);

    // Arrays
    // Size is fixed
    let months = ["January", "February", "March", "April", "May"];
    // repetitive type definition using semicolon
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    // repetitive initialization using semicolon
    let a_num = [3; 5];

    print_string_array(months);
    print_array(nums);
    print_array(a_num);

    // Conditions
    let any_num = 5;
    if any_num == 5 {
        println!("Any number is 5");
    } else {
        println!("Any number is not 5");
    }
    // Conditions can have return values
    let ret_cond = if any_num == 5 { true } else { false };
    println!("{}", ret_cond);

    // Loops can also have return types
    let mut count = 5;
    let ret_loop = loop {
        count -= 1;
        if count < 0 {
            break count * 2;
        }
    };
    println!("{}", ret_loop);

    // Conditional loop using while
    while count < 5 {
        print!("{} ", count); // print! will print without line change
        count += 1;
    }
    println!("");

    // We can make instant Range in for loop
    // let's say, countdown program
    for cd_num in (1..5).rev() {
        // rev() goes reverse order of range
        println!("Blast until....{} second", cd_num);
    }
}

// functions
// parameters also should be typed
fn print_array(arr: [i32; 5]) {
    // For loop using iter
    for a in arr.iter() {
        println!("{}", a);
    }
}
// &str is a String literal type
fn print_string_array(arr: [&str; 5]) {
    for a in arr.iter() {
        println!("{}", a);
    }
}
