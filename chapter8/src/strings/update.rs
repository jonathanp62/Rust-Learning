// (#)update.rs 0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates string updating in Rust

/// The update_string function
/// 
/// # Arguments
/// 
/// * `s` - A mutable reference to a string
pub fn update_string(s: &mut String) {    
    let s = update_using_push_str(s);

    update_using_push(s);
}


/// The concatenate function
/// 
/// # Arguments
/// 
/// * `s1` - A string to concatenate
/// * `s2` - A string to concatenate
pub fn concatenate(s1: String, s2: String) {
    let s = s1 + &s2;   // Note s1 has been moved here and can no longer be used
    
    println!("Concatenated string: {}", s);
}

/// The format_strings function
/// 
/// # Arguments
/// 
/// * `s1` - A string to format
/// * `s2` - A string to format
/// * `s3` - A string to format
pub fn format_strings(s1: String, s2: String, s3: String) {
    let s = format!("{}-{}-{}", s1, s2, s3);
    
    println!("Formatted string: {}", s);
}

/// The update_using_push_str function
/// 
/// # Arguments
/// 
/// * `s` - A mutable reference to a string
/// 
/// # Returns
/// 
/// * `&mut String` - A mutable reference to the updated string
fn update_using_push_str(s: &mut String) -> &mut String {    
    s.push_str(", World");

    println!("Updated string: {}", s);

    s
}

/// The update_using_push function
/// 
/// # Arguments
/// 
/// * `s` - A mutable reference to a string
fn update_using_push(s: &mut String) {    
    s.push('!');

    println!("Updated string: {}", s);
}
