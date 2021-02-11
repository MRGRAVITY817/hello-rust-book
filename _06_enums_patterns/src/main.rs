// Enumerator
#[derive(Debug)]
enum Message {
    // Elements in enums are called "variants"
    Quit,                       // no value type
    Move { x: i32, y: i32 },    // struct type
    Write(String),              // String type
    ChangeColor(i32, i32, i32), // Tuple type
}
// It's possible to make methods for enums
impl Message {
    fn call(&self) {
        println!("Hello?");
    }
}

#[derive(Debug)]
enum Province {
    Kyeongki,
    ChoongChung,
    KyeongSang,
    Jeolla,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Province), // Province value type
}

fn main() {
    // We use variants with double colon operator
    let m0 = Message::Quit;
    let m1 = Message::Move { x: 1, y: 2 };
    let m2 = Message::Write(String::from("Some string"));
    let m3 = Message::ChangeColor(1, 2, 3);
    println!("{:#?} {:#?} {:#?}", m1, m2, m3);
    m0.call();

    // There's a built in enum called "Option<T>"
    let some_number = Some(5);
    let some_string = Some("A string");
    let absent_number: Option<i32> = None; // Better than null
    println!("{:?} {:?} {:?}", some_number, some_string, absent_number);

    let some_coin = Coin::Quarter(Province::Jeolla);
    let value_coin = value_in_cents(&some_coin);
    println!("{}", value_coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?}", six, none);

    println!("{}", count_up(&some_coin));
}

fn value_in_cents(coin: &Coin) -> u8 {
    // match arms are useful to make exhaustive condition control
    // for enums, which means we have to cover all the enum cases
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(province) => {
            println!("Province Quarter from {:?}!", province);
            25
        }
    }

    // WE can use "_" placeholder to shorten the other options
    // that we are not interested in.
    // match coin {
    //     _ => 1,
    //     Coin::Dime => 10,
    // }
}

// Using match for option control
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn count_up(coin: &Coin) -> u8 {
    // We can make a fast expression for simple matches
    // using "if let ~ else" function
    let mut count = 0;
    if let Coin::Quarter(province) = coin {
        println!("Province Quarter from {:?}!", province);
        count += 1;
        count
    } else {
        count += 2;
        count
    }
}
