// (#)access.rs 0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// Module that demonstrates hashmap access in Rust

/// The access_hashmap function
/// 
/// # Arguments
/// 
/// * `map` - A hashmap of String and i32
pub fn access_hashmap(map: HashMap<String, i32>) {    
    let blue_score = map.get("Blue");

    // The get method returns an Option<&V>; if there’s no value for that key in the hash map,
    // get will return None. This program handles the Option by calling copied to get an Option<i32> 
    // rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t have an 
    // entry for the key.

    let yellow_score = map.get("Yellow").copied().unwrap_or(0);
    let orange_score = map.get("Orange").copied().unwrap_or(0);

    println!("Blue score: {:?}", blue_score);
    println!("Yellow score: {:?}", yellow_score);
    println!("Orange score: {:?}", orange_score);

    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
