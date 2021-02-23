mod generic_ex;
mod lifetime_ex;
mod trait_ex;

use generic_ex::generic_tutorial;
use lifetime_ex::lifetime_tutorial;
use trait_ex::trait_tutorial;

fn main() {
    generic_tutorial::what_is_generic();
    trait_tutorial::what_is_trait();
    trait_tutorial::trait_largest();
    lifetime_tutorial::what_is_lifetime();
    lifetime_tutorial::lifetime_with_struct();
}
