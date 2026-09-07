// (#)read.rs   0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates vector reading in Rust

/// The read_vector function
/// 
/// # Arguments
/// 
/// * `v` - A reference to a vector of i32 integers
pub fn read_vector(v: &Vec<i32>) {
    println!("Reading a vector");

    let third: &i32 = &v[2];    // This will panic if the vector doesn't have 3 elements

    println!("The third element is {third}");

    let fourth: Option<&i32> = v.get(3);    // This will return Some()
    let sixth: Option<&i32> = v.get(5);     // This will return None if the vector doesn't have 4 elements

    match fourth {
        Some(fourth) => println!("The fourth element is {fourth}"),
        None => println!("There is no fourth element."),
    }

    match sixth {
        Some(sixth) => println!("The sixth element is {sixth}"),
        None => println!("There is no sixth element."),
    }
}
