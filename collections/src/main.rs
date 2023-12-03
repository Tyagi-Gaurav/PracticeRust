use collections::my_vector;
use collections::my_string;
use collections::my_hashmap;

fn main() {
    // vectors();
    // strings();
    hashmaps();
}

fn strings() {
    my_string::initialize();
    my_string::update_string();
    my_string::concatenation();
    my_string::indexing_strings();
    my_string::iterating_over_strings();
}

fn vectors() {
    my_vector::vector();
    my_vector::update_vector();
    my_vector::read_values();
    my_vector::iterate_over_vec();
    my_vector::hold_different_types_in_vec();
}

fn hashmaps() {
    my_hashmap::initialize();
    my_hashmap::accessing();
    my_hashmap::hashmaps_ownership();
    my_hashmap::add_key_value_when_not_present();
    my_hashmap::update_value_based_on_old_value();
}