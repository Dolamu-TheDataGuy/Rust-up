// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }


mod front_of_house {
    pub mod hosting() {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // absolute path
    }

    fn cook_order() {}
}


mod back_of_house {

    // class Breakfast
    pub struct Breakfast { // field within a struct is private
        pub toast: String,
        pub seasonal_fruit: String,
    }

    // method for breakfast class or struct
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches",)
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal: Breakfast = back_of_house::Breakfast::summer("common!");

    meal.toast = String::from("Wheat");
}


mod back_of_house {
    pub enum Appetizer { // enums variants  follow the privacy of the enum name unlike structs
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1: Appetizer = back_of_house::Appetizer::Soup;
    let order2: Appetizer = back_of_house::Appetizer::Salad;
}

// The use keyword
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

