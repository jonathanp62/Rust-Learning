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

    println!("The 10th Fibonacci number is {}", compute_nth_fibonacci(10));
    println!("The 17th Fibonacci number is {}", compute_nth_fibonacci(17));
}

/// Compute the nth Fibonacci number recursively
/// 
/// # Arguments
/// 
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// 
/// The nth Fibonacci number
fn compute_nth_fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        compute_nth_fibonacci(n - 1) + compute_nth_fibonacci(n - 2)
    }
}

/// Print the lyrics to the song "The Twelve Days of Christmas"
fn twelve_days_of_christmas() {
    println!("Twelve days of Christmas");

    let verses = [
        "On the first day of Christmas my true love gave to me: a Partridge in a Pear Tree.", 
        "On the second day of Christmas my true love gave to me: two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the third day of Christmas my true love gave to me: three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the fourth day of Christmas my true love gave to me: four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the fifth day of Christmas my true love gave to me: five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the sixth day of Christmas my true love gave to me: six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the seventh day of Christmas my true love gave to me: seven Swans-a-Swimming, six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the eighth day of Christmas my true love gave to me: eight Maids-a-Milking, seven Swans-a-Swimming, six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the ninth day of Christmas my true love gave to me: nine Ladies Dancing, eight Maids-a-Milking, seven Swans-a-Swimming, six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the tenth day of Christmas my true love gave to me: ten Lords-a-Leaping, nine Ladies Dancing, eight Maids-a-Milking, seven Swans-a-Swimming, six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the eleventh day of Christmas my true love gave to me: eleven Pipers Piping, ten Lords-a-Leaping, nine Ladies Dancing, eight Maids-a-Milking, seven Swans-a-Swimming, six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree.", 
        "On the twelfth day of Christmas my true love gave to me: twelve Drummers Drumming, eleven Pipers Piping, ten Lords-a-Leaping, nine Ladies Dancing, eight Maids-a-Milking, seven Swans-a-Swimming, six Geese-a-Laying, five Gold Rings, four Calling Birds, three French Hens, two Turtle Doves, and a Partridge in a Pear Tree."];

    for verse in verses {
        println!("{verse}");
    }
}
