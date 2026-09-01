// (#)rectangle_methods.rs  0.1.0   09/01/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// A rectangle with width and height
/// 
/// # Fields
/// 
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/// Rectangle methods
impl Rectangle {
    /// Calculate the area of the rectangle
    /// 
    /// # Returns
    /// 
    /// * `u32` - The area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Check if this rectangle can hold the other rectangle
    /// 
    /// # Arguments
    /// 
    /// * `other` - The other rectangle to check
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if this rectangle can hold the other rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Rectangle constructor methods (associated functions)
impl Rectangle {
    /// Create a square with the given size
    /// 
    /// # Arguments
    /// 
    /// * `size` - The size of the square
    /// 
    /// # Returns
    /// 
    /// * `Self` - A new rectangle with the given size
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/// The rectangle methods example module
pub fn rectangle_methods() {
    println!("Rectangle methods");

    let rect1 = Rectangle {
        width: 25,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(10);
    
    println!("Square: {:#?}", square);
}
