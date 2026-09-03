// (#)creation.rs    0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates vector creation in Rust

/// The create_vector function
pub fn create_vectors() -> Vec<i32> {
    create_using_new();
    create_using_initial_values()   // No semi-colon means return value
}

/// The create_using_new function
fn create_using_new() {
    println!("Creating a vector using new");

    let v: Vec<i32> = Vec::new();
    
    println!("Created an empty vector: {:?}", v);
}

/// The create_using_initial_values function
fn create_using_initial_values() -> Vec<i32> {
    println!("Creating a vector using initial values");
    
    let v = vec![1, 2, 3];

    println!("Created a vector with initial values: {:?}", v);
    
    v
}
