// (#)results.rs  0.1.0   09/07/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::{self, Read};

/// Module that demonstrates using the Result<T, E> enum in Rust

/// The results function
pub fn results() {
    opening_a_file();

    let username = read_username_from_file("hello.txt");

    println!("Username: {username:?}");

    let username = read_username_from_file("users.txt");

    println!("Username: {username:?}");

    let username = using_question_mark("hello.txt");

    println!("Username: {username:?}");

    let username = using_question_mark("users.txt");

    println!("Username: {}", username.unwrap());
}

/// Opening a file
fn opening_a_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {error:?}");
            return;
        }
    };

    drop(greeting_file);    // Close the file explicitly
}

/// Reading a file
/// 
/// # Arguments
/// 
/// * `file_name` - The name of the file to read
/// 
/// # Returns
/// 
/// * `Result<String, io::Error>` - The contents of the file or an error
fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username.trim_end().to_string()),
        Err(e) => Err(e),
    }
}

/// Using the question mark operator. This function
/// is equivalent to the `read_username_from_file` function.
/// 
/// # Arguments
/// 
/// * `file_name` - The name of the file to read
/// 
/// # Returns
/// 
/// * `Result<String, io::Error>` - The contents of the file or an error
fn using_question_mark(file_name: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(file_name)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username.trim_end().to_string())
}
