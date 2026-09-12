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

/// Returns a greeting message.
///
/// # Arguments
///
/// * `name` - The name of the person to greet.
///
/// # Returns
///
/// * `String` - The greeting message.
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}


/// This function will panic.
pub fn will_panic() {
    panic!("This function will panic");
}

/// The rectangle struct.
pub struct Rectangle {
    pub width: u64,
    pub height: u64,
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
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// Calculate the area of the rectangle.
    /// 
    /// # Returns
    /// 
    /// * `u64` - The area of the rectangle
    pub fn area(&self) -> u64 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_with_assert() {
        let result = add(2, 2);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_add_with_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn test_greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    // This test will fail because the greeting function always includes "Jonathan" but demonstrates 
    // how to provide a custom error message. It is ignored by default.
    #[test]
    #[ignore]
    fn test_greeting_without_name() {
        let result = greeting("Carol");
        assert!(result.contains("Jonathan"), "Greeting should contain 'Jonathan'");
    }

    #[test]
    #[should_panic(expected = "function will panic")]
    fn test_will_panic() {
        will_panic();
    }

    #[test]
    fn test_larger_rect_can_hold_smaller_rect() {
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
    fn test_smaller_rect_cannot_hold_larger_rect() {
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

    #[test]
    fn test_area() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_eq!(larger.area(), 56);
        assert_eq!(smaller.area(), 5);
    }
}
