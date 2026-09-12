// (#)integration_test.rs   0.1.0 09/11/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use chapter11::{add, Rectangle};

#[test]
fn test_add_and_compare() {
    let larger_width = add(20, 35);
    let larger_height = add(10, 20);
    
    assert_eq!(larger_width, 55);
    assert_eq!(larger_height, 30);

    let smaller_width = add(5, 10);
    let smaller_height = add(2, 8);

    assert_eq!(smaller_width, 15);
    assert_eq!(smaller_height, 10);

    assert!(smaller_width < larger_width);
    assert!(smaller_height < larger_height);
    
    let larger = Rectangle {
        width: larger_width,
        height: larger_height,
    };
    
    let smaller = Rectangle {
        width: smaller_width,
        height: smaller_height,
    };

    assert!(larger.can_hold(&smaller));

}
