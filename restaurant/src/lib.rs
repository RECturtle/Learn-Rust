// Define a module with "mod"
// Place modules inside front_of_house for organization
// Modules can also hold structs, enums, and other consts, traits, functions
// Modules are great for organizing related functionality together
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    // Individual fields can be designated as public/private in a struct
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // enum fields are either all public or private (if enum is public,
    // fields are public)
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // use super to move to parent module 
        // and grab front_of_house w/o crate
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// We can use "use" to bring a path in scope (ex. global scope here)
// this allows us to using hosting directly in the fn eat_at_restaurant()
use crate::front_of_house::hosting;

// Create a function that utilizes code from front_of_house module
// pub keyword means this function is publically accessible
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    // now we can call hosting directly (due to "use" above)
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
} // Deciding on absolute vs relative path is situational
