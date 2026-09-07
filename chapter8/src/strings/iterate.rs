// (#)iterate.rs    0.1.0   09/04/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates string iteration in Rust

/// The iterate_strings function
pub fn iterate_strings() {
    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }

    for c in "😈🍆👅💦".chars() {
        println!("{c}");
    }

    for c in "horny".chars() {
        println!("{c}");
    }
}
