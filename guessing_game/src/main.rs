// (#)main.rs   0.1.0   08/26/2026
//
// @author   Jonathan Parker
// @version  0.1.0
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

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();  // Variable guess is mutable

        // Read the user's input into the guess variable when Result's variant is OK
        // If the variant is Err, the expect method will terminate the program

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parse the guess string into a u32 (shadowing the previous variable)

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
