// (#)exercises.rs  0.1.0   08/29/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The exercises function
pub fn exercises() {
    println!("Exercises");
    
    convert_temperatures();
    generate_fibonacci();
    twelve_days_of_christmas();
}

/// Convert temperatures between Fahrenheit and Celsius
fn convert_temperatures() {
    println!("Converting temperatures");

    println!("63°F = {:.2}°C", convert_fahrenheit_to_celsius(63.0));    // Note the rounding to 2 decimal places
    println!("22°C = {:.2}°F", convert_celsius_to_fahrenheit(22.0));
}

/// Convert Fahrenheit to Celsius
///
/// # Arguments
///
/// * `temp` - The temperature in Fahrenheit
///
/// # Returns
///
/// The temperature in Celsius
fn convert_fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

/// Convert Celsius to Fahrenheit
/// 
/// # Arguments
/// 
/// * `temp` - The temperature in Celsius
/// 
/// # Returns
/// 
/// The temperature in Fahrenheit
fn convert_celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}

/// Generate the nth Fibonacci number
fn generate_fibonacci() {
    println!("Generating the nth Fibonacci number");
}

/// Print the lyrics to the song "The Twelve Days of Christmas"
fn twelve_days_of_christmas() {
    println!("Twelve days of Christmas");
}
