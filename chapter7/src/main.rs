// (#)main.rs   0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

//! # Chapter 7 exercises
//!
//! ## Execution
//!
//! You can run this binary using Cargo:
//! ```bash
//! cargo run
//! ```

mod calculator;
mod garden;
mod restaurant;

use calculator::{add_numbers, subtract_numbers};
use chapter7::greet;

use crate::garden::fruits::Apple;
use crate::garden::vegetables::Asparagus;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
    greet(&String::from("Jonathan"));
    
    calculator_crate();
    garden_crate();
    restaurant_crate();
}


/// Calculator crate functions.
fn calculator_crate() {
    println!("5 + 3 = {}", add_numbers(5, 3));
    println!("5 - 3 = {}", subtract_numbers(5, 3));
}

/// Garden crate functions.
fn garden_crate() {
    let vegetable = Asparagus {};
    let fruit = Apple {};

    println!("I'm growing {vegetable:?}!");
    println!("I'm also growing {fruit:?}s!");
}


/// Restaurant crate functions.
fn restaurant_crate() {
    restaurant::eat_at_restaurant();
    restaurant::deliver_order();
    restaurant::back_of_house::fix_incorrect_order();
    restaurant::order_summer_breakfast();
}
