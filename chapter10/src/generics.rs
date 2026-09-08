// (#)generics.rs   0.1.0   09/08/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates generics in Rust

/// The generics function
pub fn generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    // We can call the function with a vector because Vector implements std::ops::Deref
    // which allows us to pass a reference to the vector

    let result = largest(&number_list);

    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest char is {result}");
    
    structs();
    structs_and_methods();
}

/// The largest function
/// 
/// # Type Parameters
/// 
/// * `T` - The type of the elements in the list that must implement `PartialOrd`
/// 
/// # Arguments
/// 
/// * `list` - A slice of elements of type `T`
/// 
/// # Returns
/// 
/// * `&T` - A reference to the largest element in the list
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// Structs with generics
fn structs() {
    /// Point struct with generic type T
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("int_point.x = {}, int_point.y = {}", int_point.x, int_point.y);
    println!("float_point.x = {}, float_point.y = {}", float_point.x, float_point.y);
}

/// Structs with generic methods
fn structs_and_methods() {
    /// Point struct with generic type T
    struct Point<T> {
        x: T,
        y: T,
    }

    /// Implementation of Point struct with generic type T
    impl<T> Point<T> {
        /// Returns a reference to the x field
        fn x(&self) -> &T {
            &self.x
        }
        
        /// Returns a reference to the y field
        fn y(&self) -> &T {
            &self.y
        }       
    }

    let int_point = Point { x: 50, y: 100 };
    let float_point = Point { x: 10.0, y: 40.0 };

    println!("int_point.x = {}, int_point.y = {}", int_point.x(), int_point.y());
    println!("float_point.x = {}, float_point.y = {}", float_point.x(), float_point.y());
}
