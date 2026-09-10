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

    structs();
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


/// The structs function
/// Demonstrates struct definitions with lifetime parameters
fn structs() {
    /// The ImportantExcerpt struct
    /// 
    /// # Fields
    /// 
    /// * `part` - A reference to a string slice with lifetime 'a
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    /// Implementation of ImportantExcerpt
    impl<'a> ImportantExcerpt<'a> {
        /// Returns the level of the excerpt
        /// 
        /// # Returns
        /// 
        /// * `i32` - The level of the excerpt
        fn level(&self) -> i32 {
            3
        }

        /// Announces and returns the part of the excerpt
        /// 
        /// # Arguments
        /// 
        /// * `announcement` - A string slice
        /// 
        /// # Returns
        /// 
        /// * `&str` - A string slice
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Important excerpt: {}", i.part);
    println!("Level: {}", i.level());
    println!("Announcement: {}", i.announce_and_return_part("Hello"));
}
