// (#)creation.rs    0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// Module that demonstrates hash map creation in Rust

/// The create_hashmaps function
pub fn create_hashmaps() -> HashMap<String, i32> {
    create_using_new()
}

/// The create_using_new function
fn create_using_new() -> HashMap<String, i32> {
    println!("Creating a hash map using new");

    let mut scores: HashMap<String, i32> = HashMap::new();
    
    println!("Created an empty hash map: {:?}", scores);

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("After inserting values: {:?}", scores);

    scores
}
