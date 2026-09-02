// (#)options.rs    0.1.0   09/02/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The options function demonstrates option enums
pub fn options() {
    println!("Option Enums");

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    if some_number.is_some() {
        println!("Some number: {}", some_number.unwrap());
    }

    if some_char.is_some() {
        println!("Some char: {}", some_char.unwrap());
    }

    if absent_number.is_none() {
        println!("None");
    }
}
