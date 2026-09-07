// (#)main.rs   0.1.1   09/07/2026
// (#)main.rs   0.1.0   08/26/2026
//
// @author   Jonathan Parker
// @version  0.1.1
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

//! # Guessing Game
//!
//! ## Execution
//!
//! You can run this binary using Cargo:
//! ```bash
//! cargo run
//! ```

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    let mut number_of_tries = 0;

    loop {
        println!("Please input your guess:");

        let mut input = String::new();  // Variable input is mutable

        // Read the user's input into the input variable when Result's variant is OK
        // If the variant is Err, the expect method will terminate the program

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the input string into an i32

        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Create a new Guess instance from the parsed input (shadowing the previous variable)

        let guess: Guess = Guess::new(guess);
        
        number_of_tries = number_of_tries + 1;

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
    println!("Number of tries: {}", number_of_tries);
}

/// A guess for the secret number.
///
/// This struct is used to validate that the guess is within the valid range (1-100)
/// and was introduced in chapter 9 to demonstrate custom validation.
pub struct Guess {
    value: i32,
}

/// Implementation of the Guess struct and also introduced in chapter 9.
impl Guess {
    /// Creates a new Guess instance.
    ///
    /// # Arguments
    ///
    /// * `value` - The guess value to be validated.
    ///
    /// # Panics
    ///
    /// This function will panic if the value is not within the valid range (1-100).
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
