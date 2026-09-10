// (#)lib.rs    0.1.0 09/10/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Adds two numbers together.
///
/// # Arguments
///
/// * `left` - The left number to add.
/// * `right` - The right number to add.
///
/// # Returns
///
/// * `u64` - The sum of the two numbers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// The rectangle struct.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// The rectangle implementation.
impl Rectangle {
    /// Return true if this rectangle can contain the other rectangle.
    /// 
    /// # Arguments
    /// 
    /// * `self` - This rectangle
    /// * 'other` - The other rectangle
    /// 
    /// # Returns
    /// 
    /// * `bool` - The result of the comparison
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);

        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
