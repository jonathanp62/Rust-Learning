// (#)update.rs 0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates vector updating in Rust

/// The update_vector function
/// 
/// # Arguments
/// 
/// * `v` - A mutable reference to a vector of i32 integers
pub fn update_vector(v: &mut Vec<i32>) {    
    // Add an element to the vector
    v.push(4);
    v.push(5);
    v.push(6);
    
    println!("Updated vector: {:?}", v);

    v.pop();

    println!("Updated vector after pop: {:?}", v);
}
