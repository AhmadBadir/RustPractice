
//nested use
///use std::cmp::Ordering;
///use std::io;
//this above two lines become
use std::{io, cmp::Ordering};
//using self in nested use
//use std::io;
//use std::io::Write
//becomes
use std::io::{Write};

use std::collections::*;


//using external file as module "front_of_house.rs"
mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist()
    }

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
        }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
        }
    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
                }
            }
        }
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        }
    fn cook_order() {}
    }

mod customer {
    pub fn eat_at_restaurant() {
        //uncomment this would not compile
        //hosting::add_to_waitlist();
        }
    }

// pub fn eat_at_restaurant() {
//     //Absolute path
//     //crate::front_of_house::hosting::add_to_waitlist();
//
//     //Relative path
//     //front_of_house::hosting::add_to_waitlist();
//
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//
//     meal.toast = String::from("Wheat");
//
//     println!("I'd like {} toast please", meal.toast);
//
//     // The next line won't compile if we uncomment it; we're not allowed
//         // to see or modify the seasonal fruit that comes with the meal
//         // meal.seasonal_fruit = String::from("blueberries");
//
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
//
// //usage of use
//     hosting::add_to_waitlist();
//
//
//     }