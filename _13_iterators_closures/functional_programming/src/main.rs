mod closures_ex;
use closures_ex::closures_tutorial;

fn main() {
    // Testing closures
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;
    closures_tutorial::generate_workout(simulated_user_specific_value, simulated_random_number);
    // Testing iterators
}
