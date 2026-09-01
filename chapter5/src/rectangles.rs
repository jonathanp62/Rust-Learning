// (#)rectangles.rs 0.1.0   09/01/2026
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

/// The rectangles example module
pub fn rectangles() {
    println!("Rectangles");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Print with debug (trait) formatting
    println!("rect1 is {rect1:?}");     // Basic debug formatting
    println!("rect1 is {rect1:#?}");    // Pretty print with indentation

    dbg!(&rect1);                       // Print with file:line:column format to STDERR

    println!("The area of the rectangle is {} square pixels", area(&rect1));
}

/// Calculate the area of a rectangle
/// 
/// # Arguments
/// 
/// * `rectangle` - The reference to a rectangle to calculate the area of
/// 
/// # Returns
/// 
/// * `u32` - The area of the rectangle
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
