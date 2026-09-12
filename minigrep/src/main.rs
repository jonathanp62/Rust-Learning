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
use std::process;

/// Main function.
fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

/// Run the search with the given configuration.
/// 
/// # Arguments
/// 
/// * `config` - The configuration for the search.
fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

/// Configuration for the search.
struct Config {
    query: String,
    file_path: String,
}

/// The config implementation.
impl Config {
    /// Build a config from command line arguments.
    /// 
    /// # Arguments
    /// 
    /// * `args` - A slice of strings containing the command line arguments.
    /// 
    /// # Returns
    /// 
    /// A Result containing the Config instance or an error message.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
