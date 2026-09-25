// (#)main.rs   0.1.0 09/12/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use minigrep::search_case_sensitive;
use minigrep::search_case_insensitive;

use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// Main function.
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}' in file {}...", config.query, config.file_path);

    if let Err(e) = run(config, false) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// Run the search with the given configuration.
/// The Box<dyn Error> means the function will return a type that implements
/// the Error trait, but we don’t have to specify what particular type the 
/// return value will be.
/// 
/// # Arguments
/// 
/// * `config` - The configuration for the search.
/// 
/// # Returns
/// 
/// A Result indicating success or an error.
fn run(config: Config, debug: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if debug {
        println!("With text:\n{contents}");
    }

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!(">>>{line}<<<");
    }

    Ok(())
}

/// Configuration for the search.
struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/// The config implementation.
impl Config {
    /// Build a config from command line arguments.
    /// 
    /// # Arguments
    /// 
    /// * `args` - An iter
    /// 
    /// # Returns
    /// 
    /// A Result containing the Config instance or an error message.
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();    // Skip the name of the process

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // We’re using the is_ok method on the Result to check whether the environment variable is set,
        // which means the program should do a case-insensitive search. If the IGNORE_CASE environment
        // variable isn’t set to anything, is_ok will return false and the program will perform a case-sensitive search.
        // We don’t care about the value of the environment variable, just whether it’s set or unset,
        // so we’re checking is_ok rather than using unwrap, expect, or any of the other methods we’ve seen on Result.
        
        let ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}
