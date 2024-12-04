fn serve_order() {}
mod front_of_house ;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        crate::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfirst {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfirst {
        pub fn summer(toast: &str) -> Breakfirst {
            Breakfirst {
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
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfirst::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("i like {}", meal.toast);
    hosting::add_to_waitlist();
}
