// (#)variables.rs  0.1.0   08/27/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The variables function demonstrates immutable and mutable variables
pub fn variables() {
    // Immutable variable
    let x = 5;

    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 15;

    println!("The value of y is: {}", y);

    // Reassigning a new value to y
    y = 20;

    println!("The value of y is: {}", y);

    constants();
    shadowing();
}

/// The constants function demonstrates constants
fn constants() {
    println!("Constants");
    
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadowing() {
    println!("Shadowing");
    
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y in the outer scope is: {y}");

    let spaces = "   ";

    // Shadowing with a different type
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}
