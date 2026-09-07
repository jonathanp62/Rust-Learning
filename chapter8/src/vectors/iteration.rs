// (#)iteration.rs  0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates vector iteration in Rust

/// The iterate_vector function
/// 
/// # Arguments
/// 
/// * `v` - A reference to a vector of i32 integers
pub fn iterate_vector(v: &Vec<i32>) {
    println!("Iterating over a vector");
    
    for i in v {
        println!("{}", i);
    }
}
