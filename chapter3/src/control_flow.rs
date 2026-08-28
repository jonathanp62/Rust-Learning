// (#)control_flow.rs   0.1.0   08/28/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The control_flow function demonstrates 'if' and looping control flow
pub fn control_flow() {
    if_statement();
    loop_statement();
    while_statement();
    for_statement();
}

/// Demonstrate 'if' statements
fn if_statement() {
    println!("If statement");

    let number = 6;

    if number % 4 == 0 {
        println!("number {number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("number {number} is divisible by 2");
    } else {
        println!("number {number} is not divisible by 4, 3, or 2");
    }

    // Using 'if' in a let statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

/// Demonstrate 'loop' statements
fn loop_statement() {
    println!("Loop statement");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The counter is {result}");
}

/// Demonstrate 'while' statements
fn while_statement() {
    println!("While statement");
}

/// Demonstrate 'for' statements
fn for_statement() {
    println!("For statement");
}
