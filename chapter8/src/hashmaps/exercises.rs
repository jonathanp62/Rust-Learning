// (#)exercises.rs  0.1.0   09/05/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// Module that demonstrates hashmap updating in Rust

/// Structure to hold calculation results
struct CalculationResult {
    median: i32,
    modes: Vec<i32>,
}

/// The exercises function
pub fn exercises() {    
    integers();
    pig_latin();
}

/// Process integers and calculate median and modes
fn integers() {
    let ints = [21, 4, 1, 24, 9, 8, 8, 5, 24, 4, 22, 24, 29, 18, 3, 19, 14, 2, 1, 3, 7, 8, 17, 20, 1];
    
    println!("Input: {:?}", ints);
    
    let result = process_integers(&ints, false);

    println!("Median: {}, Modes: {:?}", result.median, result.modes);
}

/// Convert English words to Pig Latin
fn pig_latin() {
    let phrase = "The quick brown fox jumps over the lazy dog eating apples";
    
    println!("Phrase: {}", phrase);
    
    let result = process_pig_latin(&phrase, false);

    println!("Result: {}", result);
}

/// Process integers and calculate median and modes
/// 
/// # Arguments
/// 
/// * `ints` - A slice of integers to process
/// * `debug` - A boolean flag to enable debug output
/// 
/// # Returns
/// 
/// A `CalculationResult` containing the median and modes
fn process_integers(ints: &[i32], debug: bool) -> CalculationResult {
    // Store integers in a vector for sorting and median calculation

    let mut vector = Vec::new();
    
    for &num in ints {
        vector.push(num);
    }
    
    vector.sort();

    let median = vector[vector.len() / 2];
    
    if debug {
        println!("Median: {}", median);
    }

    // Create a hash map to count frequencies
    // Key: integer value, Value: frequency count

    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    
    for &number in &vector {
        let count = hash_map.entry(number).or_insert(0);
        
        *count += 1;
    }
    
    if debug {
        println!("Hash map: {:?}", hash_map);
    }
    
    let modes: Vec<i32> = if let Some(&max_freq) = hash_map.values().max() {
        if debug {
            println!("Max frequency: {}", max_freq);
        }

        hash_map
            .iter()
            .filter(|&(_, &count)| count == max_freq)
            .map(|(&number, _)| number)
            .collect()
    } else {
        Vec::new()  // Return empty vector if no elements in hashmap
    };
    
    
    if debug {
        println!("Modes: {:?}", modes);
    }
    
    CalculationResult {
        median,
        modes,
    }
}

/// Process Pig Latin conversion
/// 
/// # Arguments
/// 
/// * `phrase` - A string slice to convert
/// * `debug` - A boolean flag to enable debug output
/// 
/// # Returns
/// 
/// A `String` containing the Pig Latin conversion
fn process_pig_latin(phrase: &str, debug: bool) -> String {
    let words = split_into_words(phrase);

    let mut builder = String::new();

    for word in words {
        if debug {
            println!("Word: {}", word);
        }

        // Convert to lowercase and check if it starts with a vowel
        let lower_word = word.to_lowercase();
        let first_char = lower_word.chars().next().unwrap();
        
        if first_char == 'a' || first_char == 'e' || first_char == 'i' || first_char == 'o' || first_char == 'u' {
            // Word starts with a vowel, just add "way"
            builder.push_str(word);
            builder.push_str("hay ");
        } else {
            // Word starts with a consonant, move consonant cluster to end and add "ay"
            // For simplicity, we'll just move the first letter to the end and add "ay"
            let mut chars = word.chars();
            let first_letter = chars.next().unwrap();
            let rest = chars.as_str();

            builder.push_str(rest);
            builder.push(first_letter);
            builder.push_str("ay ");
        }
    }
    
    if debug {
        println!("Builder: {}", builder);
    }
    
    builder
}

/// Split a sentence into words
/// 
/// # Arguments
/// 
/// * `sentence` - A string slice to split
/// 
/// # Returns
/// 
/// A vector of string slices representing the words
fn split_into_words(sentence: &str) -> Vec<&str> {
    sentence.split_whitespace().collect()
}
