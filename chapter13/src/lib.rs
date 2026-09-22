// (#)lib.rs 0.1.0 09/18/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Shirt color enum
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

/// Inventory struct
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

/// Inventory implementation
impl Inventory {
    /// Give away a shirt based on user preference
    /// 
    /// If the user has a preference, it will be used.
    /// Otherwise, the most stocked shirt color will be used.
    /// 
    /// # Arguments
    /// 
    /// * `user_preference` - An optional shirt color preference
    /// 
    /// # Returns
    /// 
    /// * The shirt color to give away
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Function unwrap_or_else() takes a closure that is only called if the option is None
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /// Find the most stocked shirt color
    /// 
    /// # Returns
    /// 
    /// * The shirt color with the most stock
    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

/// Rectangle struct
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

/// Sort rectangles by width
pub fn sort_rectangles_by_width() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // Sort by width using a closure that implements trait FnMut because it is called multiple times

    list.sort_by_key(|r| r.width);

    for r in &list {
        println!("Rectangle width: {}, height: {}", r.width, r.height);
    }
}


/// Sort rectangles by height
pub fn sort_rectangles_by_height() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // Sort by height using a closure that implements trait FnMut because it is called multiple times
    // The closure captures the mutable reference to num_sort_operations
    
    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.height
    });

    for r in &list {
        println!("Rectangle width: {}, height: {}", r.width, r.height);
    }
    
    println!("Number of sort operations: {}", num_sort_operations);
}
