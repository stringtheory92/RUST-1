mod back_of_house {
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

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod front_of_house;
pub use crate::front_of_house::hosting;
pub use crate::front_of_house::print_hello;



pub use crate::back_of_house::{Breakfast, Appetizer};

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    print_hello();
    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
}



