//Entire module tree is rooted under the implicit module named crate


//The module tree should be defined in src/lib.rs.

//If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private
mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast : String,
        seasonal_fruit : String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { //This is needed because breakfast has a private field and cannot be created from outside.
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //All fields of enum are public 
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();

    //After adding use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); //This can be changed as its a public field.
    println!("I'd like {} toast please", meal.toast); 
}

fn deliver_order() {

}