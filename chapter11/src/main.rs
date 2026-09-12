// (#)main.rs   0.1.0 09/11/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use chapter11::add;
use chapter11::greeting;
use chapter11::Rectangle;

/// Main function.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: add(10,30),
    };

    println!("Rect1 width + Rect2 width = {}", add(rect1.width, rect2.width));
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Greeting: {}", greeting("Jonathan"));
}

// Integration test will test add in the context of creating new rectangles and then using rectangle methods.
