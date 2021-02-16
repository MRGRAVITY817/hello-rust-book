mod hashmap_test;
mod string_test;
mod vector_test;

use crate::hashmap_test::hashmap_tutorial;
use crate::string_test::string_tutorial;
use crate::vector_test::vector_tutorial;

fn main() {
    // Vectors
    vector_tutorial::indexing();
    vector_tutorial::mutable_iterating();
    vector_tutorial::enum_vectors();
    // Strings
    string_tutorial::string_basic();
    string_tutorial::concat_string();
    // Hash Maps
    hashmap_tutorial::hashmap_basic();
    hashmap_tutorial::ownership();
    hashmap_tutorial::access_update();
}
