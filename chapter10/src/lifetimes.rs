// (#)lifetimes.rs  0.1.0   09/08/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates lifetimes in Rust

/// The lifetimes function
pub fn lifetimes() {
    // Variables string1 and string2 have different lifetimes
    let string1 = String::from("abcd"); // Function-local lifetime (shorter)
    let string2 = "xyz";                  // String literal ('static')lifetime

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {result}");

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");  // Has the shorter lifetime so result can't outlive it
        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {result}");
    }
}

/// The longest function
/// 'a is a lifetime parameter that ensures the returned reference is valid
/// for at least as long as the shorter of the two input references.
/// 
/// # Arguments
/// 
/// * `x` - A string slice
/// * `y` - A string slice
/// 
/// # Returns
/// 
/// * `&str` - A string slice
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
