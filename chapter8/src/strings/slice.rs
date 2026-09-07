// (#)slice.rs  0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates string slicing in Rust

/// The slice_strings function
pub fn slice_strings() {
    // Rust does support slicing strings with byte indices but it can panic 
    // if the index is not at a character boundary so ASCII characters work best

    let s = String::from("Hello, World!");
    let slice_left = &s[0..5];
    let slice_right = &s[7..12];
    
    println!("First five characters: {}", slice_left);
    println!("Last five characters: {}", slice_right);
}
