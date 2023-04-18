mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::{hosting, serving};
pub use crate::back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let order_1 = back_of_house::Appetizer::Soup;
    let order_2 = back_of_house::Appetizer::Salad;
    let mut meal_order = back_of_house::Breakfast::summer("Rye");

    meal_order.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal_order.toast);
}