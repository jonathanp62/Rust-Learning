// (#)references.rs 0.1.0   08/31/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The references function demonstrates references and borrowing
pub fn references() {
    println!("References and borrowing");

    let s1 = String::from("Hello");

    println!("Length of '{}': {}", s1, calculate_length(&s1));

    // s1 is still valid here because calculate_length only borrowed it

    println!("s1 is still valid: {}", s1);

    mutable_references();
    dangling_reference();
}

/// This function demonstrates references
/// 
/// # Arguments
/// 
/// * `s` - A reference to a String
/// 
/// # Returns
/// 
/// * `usize` - The length of the String
fn calculate_length(s: &String) -> usize {
    s.len()
}


/// This function demonstrates mutable references
fn mutable_references() {
    // There can only be one mutable reference to a variable at a time
    // If there is no mutable reference, you can have immutable references to the same variable
    
    let mut s = String::from("Hello");

    append_to_string(&mut s);

    println!("{}", s);
}

/// This function appends ", world!" to a string
/// 
/// # Arguments
/// 
/// * `some_string` - A mutable reference to a String
fn append_to_string(some_string: &mut String) {
    some_string.push_str(", world!");
}

/// This function demonstrates how to avoid dangling references
fn dangling_reference() {
    println!("{}", not_dangled());
}

/// This function returns a String, not a reference to one
fn not_dangled() -> String {
    let s = String::from("A string without a dangling reference");

    s

    // Never return a reference to a local variable
}