mod destruct;
mod ignorance;
mod pattern_place;

use destruct::destruct_examples;
use destruct::destruct_examples::Color;
use destruct::destruct_examples::Message;
use destruct::destruct_examples::Palette;
use destruct::destruct_examples::Point;
use ignorance::ignorance_example;
use pattern_place::pattern_expressions;

fn main() {
    // match
    let match_num = 1;
    pattern_expressions::match_number(match_num);
    let match_char = 'k';
    pattern_expressions::match_chars(match_char);

    // match with conditional arm
    let x = 3;
    let y = true;
    pattern_expressions::match_if(x, y);

    // match arm with bind variable
    let msg = pattern_expressions::Message::Hello { id: 4 };
    pattern_expressions::match_binding(&msg);

    // If-let expression
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    pattern_expressions::if_let_expression(favorite_color, is_tuesday, age);

    // While-let conditional loops
    let num = 5;
    pattern_expressions::stack_example(num);

    // For loops
    let v = vec!['a', 'b', 'c'];
    pattern_expressions::print_iter(&v);

    // Function parameters
    let coords = (12, 44);
    pattern_expressions::print_coordinates(&coords);

    // Destructuring struct
    let p = Point::new(12, 0);
    destruct_examples::destruct_struct(&p);

    // Destructuring enums
    let m = Message::ChangeColor(2, 3, 4);
    destruct_examples::destruct_enum(&m);

    // Destructuring nested enum
    let palette = Palette::ChangeColor(Color::Hsv(3, 4, 5));
    destruct_examples::destruct_nested_enum(&palette);

    // Ignore function var
    let coords = (12, 4);
    ignorance_example::ignore_value(coords.0, coords.1);

    // Ignore parts
    let mut setting_value = Some(5);
    let new_setting_value = Some(6);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("The value already exists")
        }
        _ => setting_value = new_setting_value,
    }
    println!("{}", &setting_value.unwrap());

    // Ignore parts in tuple
    let numbers = (1, 2, 3, 4, 5);
    ignorance_example::ignore_parts_2(&numbers);

    // Unused Variables
    ignorance_example::unused_variable(3, 5);

    // ignore binding
    let s = Some(String::from("Hello"));
    ignorance_example::ignore_binding(s);

    // Ignore parts using spread
    ignorance_example::spread_ignore(&numbers)
}
