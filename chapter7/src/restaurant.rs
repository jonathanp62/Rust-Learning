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

// Eat at the restaurant
pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
}

// Deliver an order
pub fn deliver_order() {
    println!("Delivering order...");
}
