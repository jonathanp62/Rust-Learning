// (#)creation.rs    0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates string creation in Rust

/// The create_strings function
pub fn create_strings() -> String {
    create_using_new();
    create_from_literal();
    create_from_string()
}

/// The create_using_new function
fn create_using_new() {
    println!("Creating string using new");

    let s = String::new();

    println!("Created an empty string: {}", s);
}

/// The create_from_literal function
fn create_from_literal() {
    println!("Creating string using literal");

    let s = "Hello".to_string();

    println!("Created a string from literal: {}", s);
}

/// The create_from_string function
/// 
/// # Returns
/// 
/// * `String` - A new string
fn create_from_string() -> String {
    println!("Creating string from string");
    
    let s = String::from("Hello");

    println!("Created a string from string: {}", s);

    s
}
