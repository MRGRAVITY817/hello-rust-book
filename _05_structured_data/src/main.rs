// This will print struct type
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Start with "impl" to define struct method
impl Rectangle {
    // Pass the parameter "&self" to use fields
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// We can use multiple impl blocks
impl Rectangle {
    // Associated Function is a struct function without need to use struct fields
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 4,
        height: 3,
    };
    // we access data in struct using period
    println!("rect1's area is {}", rect1.area());

    // {:?} is used to format print the struct
    println!("rect1 is {:?}", rect1);
    // {:#?} is used to prettify print the struct
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 1,
        height: 2,
    };
    let rect3 = Rectangle {
        width: 5,
        height: 7,
    };

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    // we use associated functions with :: operator
    let square1 = Rectangle::square(4);
    println!("square1 is {:#?}", square1);
}
