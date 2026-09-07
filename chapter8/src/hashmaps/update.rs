// (#)update.rs 0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// Module that demonstrates hashmap updating in Rust

/// The update_hashmap function
/// 
/// # Arguments
/// 
/// * `map` - A mutable reference to a hashmap of String and i32
pub fn update_hashmap(map: &mut HashMap<String, i32>) {    
    overwrite_value(map);
    insert_if_key_missing(map);
}

/// The overwrite_value function
/// 
/// # Arguments
/// 
/// * `map` - A mutable reference to a hashmap of String and i32
fn overwrite_value(map: &mut HashMap<String, i32>) {
    map.insert(String::from("Blue"), 25);
    
    println!("Overwritten value: {:?}", map);
}

/// The insert_if_key_missing function
/// 
/// # Arguments
/// 
/// * `map` - A mutable reference to a hashmap of String and i32
fn insert_if_key_missing(map: &mut HashMap<String, i32>) {
    map.entry(String::from("Green")).or_insert(30);
    map.entry(String::from("Green")).or_insert(30);
    
    println!("Inserted value: {:?}", map);
}
