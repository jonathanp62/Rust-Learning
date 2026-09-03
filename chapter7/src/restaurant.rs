// (#)restaurant.rs 0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The restaurant module. It must publish its submodules.

// Front of house module
pub mod front_of_house {
    // Hosting module
    pub mod hosting {
        // Add to the waiting list
        pub fn add_to_waitlist() {
            println!("Adding to waitlist...");
        }

        // Seat at a table
        pub fn seat_at_table() {
            println!("Seating at table...");
        }
    }

    // Serving module
    pub mod serving {
        // Take an order
        pub fn take_order() {
            println!("Taking order...");
        }

        // Serve an order
        pub fn serve_order() {
            println!("Serving order...");
        }

        // Take payment
        pub fn take_payment() {
            println!("Taking payment...");
        }
    }
}

// Back of house module
pub mod back_of_house {
    // Breakfast struct (All fields are public)
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    // Breakfast methods
    impl Breakfast {
        // Create a summer breakfast
        //
        // # Arguments
        //
        // * `toast` - The type of bread for the breakfast
        //
        // # Returns
        //
        // * `Breakfast` - A breakfast with the specified toast
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Appetizer enum
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Fix an incorrect order
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    // Cook an order
    pub fn cook_order() {
        println!("Cooking order...");
    }
}

use crate::restaurant::front_of_house::hosting as Host;
use crate::restaurant::front_of_house::serving as Server;

// Eat at the restaurant
pub fn eat_at_restaurant() {
    Host::add_to_waitlist();
    Host::seat_at_table();
    Server::take_order();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order 1: {:?}", order1);
    println!("Order 2: {:?}", order2);

    Server::serve_order();
    Server::take_payment();
}

// Deliver an order
pub fn deliver_order() {
    println!("Delivering order...");
}

// Order a summer breakfast
pub fn order_summer_breakfast() {
    let meal = back_of_house::Breakfast::summer("Wheat");

    println!("I'd like {} toast please", meal.toast);
    println!("I understand that {} are in season", meal.seasonal_fruit);
}
