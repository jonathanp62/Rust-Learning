// (#)main.rs   0.1.0 09/12/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::env;

/// Main function.
fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
