// (#)multi_type.rs 0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates a multi-type vector in Rust

/// The multi_type_vector function
pub fn multi_type_vector() {
    println!("Creating a multi-type vector");

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => {
                println!("Found an integer: {}", value);
                // You can use 'value' as an i32 here
            }
            SpreadsheetCell::Float(value) => {
                println!("Found a float: {}", value);
                // You can use 'value' as an f64 here
            }
            SpreadsheetCell::Text(value) => {
                println!("Found text: {}", value);
                // You can use 'value' as a &String here
            }
        }
    }
}
