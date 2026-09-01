// (#)slices.rs 0.1.0   08/31/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The slices function demonstrates slices
pub fn slices() {
    println!("Slices");

    let s = String::from("hello world");

    // The &str type is a slice that references part of a string

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello = {}, world = {}", hello, world);
    
    left_slice();
    right_slice();
    whole_slice();

    let goodbye = "Goodbye, cruel world!";

    println!("First word = {}", first_word(&s));
    println!("First word = {}", first_word(goodbye));
    
    array_slice();
}

/// Find a left slice
fn left_slice() {
    let s = String::from("hello");

    let _slice = &s[0..2];  // These two statements are equivalent
    let slice = &s[..2];
    
    println!("left slice = {}", slice);
}

/// Find a right slice
fn right_slice() {
    let s = String::from("hello");
    let len = s.len();

    let _slice = &s[3..len];    // These two statements are equivalent
    let slice = &s[3..];

    println!("right slice = {}", slice);
}

/// Find the whole slice
fn whole_slice() {
    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[0..len];    // These two statements are equivalent
    let slice = &s[..];

    println!("whole slice = {}", slice);
}

/// Find and return the first word in a string
/// 
/// # Arguments
/// 
/// * `s` - A string slice
/// 
/// # Returns
/// 
/// * `&str` - A string slice containing the first word
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/// Demonstrate array slices
fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("slice = {:?}", slice);    // [2, 3]
}
