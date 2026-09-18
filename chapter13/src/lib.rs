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
