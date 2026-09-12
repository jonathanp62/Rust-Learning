// (#)main.rs   0.1.0 09/12/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::env;
use std::fs;

/// Main function.
fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

/// Configuration for the search.
struct Config {
    query: String,
    file_path: String,
}

/// Parse command line arguments into query and file path.
/// 
/// # Arguments
/// 
/// * `args` - A slice of strings containing the command line arguments.
/// 
/// # Returns
/// 
/// A tuple containing the query and file path.
fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

    Config {
        query: query.clone(),
        file_path: file_path.clone(),
    }
}
