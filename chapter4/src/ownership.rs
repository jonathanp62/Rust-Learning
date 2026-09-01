// (#)ownership.rs  0.1.0   08/31/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The ownership function demonstrates variables and ownership on the stack and the heap
pub fn ownership() {
    println!("Ownership");

    string_type();
    borrowing();
    reassignment();
    cloning();
    stack_only_copy();
    functional_ownership();
    return_values_and_scope();
}

/// This function demonstrates the String type
fn string_type() {
    // Create a mutable String variable (as opposed to a string literal)
    let mut s = String::from("Hello");

    // push_str() appends a literal to a String
    s.push_str(", world!");

    // This will print `Hello, world!`
    println!("{}", s);

    // s goes out of scope and is freed here
}

/// This function demonstrates borrowing
fn borrowing() {
    let s1 = String::from("Hello");

    // This transfers (moves) ownership from s1 to s2 and s1 goes out of scope
    let s2 = s1;
    
    println!("{}", s2);

    // s2 goes out of scope and is freed here
}

/// This function demonstrates reassignment
fn reassignment() {
    let mut s = String::from("hello");

    println!("Length of '{}' is {}", s, s.len());
    
    // The original value is dropped when s is reassigned
    s = String::from("Ahoy");

    println!("{s}, world!");
}

/// This function demonstrates cloning
fn cloning() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Both s1 and s2 go out of scope and are freed here
}

/// This function demonstrates stack-only copy
fn stack_only_copy() {
    let x = 5;
    let y = x;

    // x is not freed as it is on the stack and not dropped
    println!("x = {}, y = {}", x, y);

    // Both x and y go out of scope and are freed here
}

/// This function demonstrates functional ownership
fn functional_ownership() {
    // s comes into scope
    let s = String::from("hey, there");
    
    // s's value moves into the function
    // and becomes no longer valid here
    takes_ownership(s); 

    // x comes into scope
    let x = 5;                  

    // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    makes_copy(x);     
}

/// This function takes ownership of a String
/// 
/// # Arguments
/// 
/// * `some_string` - A String to take ownership of
fn takes_ownership(some_string: String) {
    // Variable some_string comes into scope
    println!("Taking ownership of {some_string}");

    // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
}


/// This function makes a copy of an i32
/// 
/// # Arguments
/// 
/// * `some_integer` - An i32 to make a copy of
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Making a copy of {some_integer}");

    // Here, some_integer goes out of scope. Nothing special happens.
}

/// This function demonstrates returning values and scope
fn return_values_and_scope() {
    // Function gives_ownership moves its return value into _s1
    let _s1 = gives_ownership();

    // Variable s2 comes into scope
    let s2 = String::from("hello");    

    // Variable s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let _s3 = takes_and_gives_back(s2);

    // Here, _s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. _s1 goes out of scope and is dropped.
}

/// This function gives ownership of a String
/// 
/// # Returns
/// 
/// * `String` - A String that is given ownership
fn gives_ownership() -> String {
    // Variable some_string comes into scope
    let some_string = String::from("yours");

    // Variable some_string is returned and moves out to the calling function
    some_string
}

/// This function takes a String and returns a String.
/// 
/// # Arguments
/// 
/// * `a_string` - A String to take ownership of
/// 
/// # Returns
/// 
/// * `String` - A String that is given ownership
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    // a_string is returned and moves out to the calling function

    a_string  
}
