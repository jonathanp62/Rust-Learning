// (#)functions.rs  0.1.0   08/28/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The functions function demonstrates functions
pub fn functions() {
    function_with_parameters(5, 10);

    println!("The return value is: {}", function_with_return_value());
    println!("The sum of 5 and 10 is: {}", sum(5, 10));
}

/// The function_with_parameters function demonstrates functions with parameters
///
/// # Arguments
///
/// * `a` - The first integer
/// * `b` - The second integer
fn function_with_parameters(x: i32, y: i32) {
    println!("Function with parameters");
    println!("x = {}, y = {}", x, y);
}

/// The function_with_return_value function demonstrates functions with return values
/// 
/// # Returns
///
/// The number 64
fn function_with_return_value() -> i32 {
    println!("Function with return value");

    // The absence of the semicolon makes this an expression, which returns a value
    64
}

/// The sum function demonstrates functions with return values
///
/// # Arguments
///
/// * `a` - The first integer to add
/// * `b` - The second integer to add
///
/// # Returns
///
/// The sum of `a` and `b`
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
