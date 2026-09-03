// (#)lib.rs    0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Greets a person by name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person to greet.
///
/// # Returns
///
/// * `()` - This function does not return a value.
pub fn greet(name: &String) {
    println!("Hello, {}! This is coming from the library crate.", name);
}

/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - An integer to be added.
/// * `b` - An integer to be added.
///
/// # Returns
///
/// * `i32` - The sum of the two numbers.
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
