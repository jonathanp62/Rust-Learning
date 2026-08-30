// (#)data_types.rs 0.1.0   08/27/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The data_types function demonstrates data types
pub fn data_types() {
    scalar_types();
    tabular_types();
}

/// The scalar_types function demonstrates scalar types
fn scalar_types() {
    println!("Scalar types");

    let x = 2.0;   // f64
    let y: f32 = 3.0;   // f32
    let z: bool = true; // bool
    let e: char = '😈'; // char can be any Unicode scalar value

    println!("x = {}, y = {}, z = {}, e = {}", x, y, z, e);
    
    numeric_operations();
}

/// The numeric_operations function demonstrates numeric operations
fn numeric_operations() {
    println!("Numeric operations");
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    
    println!(
        "sum = {}, difference = {}, product = {}, quotient = {}, remainder = {}", 
        sum, 
        difference, 
        product, 
        quotient, 
        remainder
    );
}

/// The tabular_types function demonstrates tabular types
fn tabular_types() {
    println!("Tabular (compound) types");
    
    tuples();
    arrays();
}

/// The tuples function demonstrates tuples
fn tuples() {
    println!("Tuples");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Tuple destructuring
    let (x, y, z) = tup;
    
    println!("x = {}, y = {}, z = {}", x, y, z);

    let t: (i32, f64, u8) = (500, 6.4, 1);

    // Tuple indexing
    let five_hundred = t.0;
    let six_point_four = t.1;
    let one = t.2;
    
    println!("five_hundred = {}, six_point_four = {}, one = {}", five_hundred, six_point_four, one);

    // The tuple without any values has a special name, unit. 
    // This value and its corresponding type are both written () and represent an empty value
    // or an empty return type. Expressions implicitly return the unit value if they don’t
    // return any other value.
}

/// The arrays function demonstrates arrays
fn arrays() {
    println!("Arrays");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    println!("The first month is: {}", months[0]);
    println!("The last month is: {}", months[11]);

    // Array declaration with explicit type and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The first element of a is: {}", a[0]);
    println!("The last element of a is: {}", a[4]);

    // Array initialization with repeated values
    let b = [3; 5];

    println!("The first element of b is: {}", b[0]);
    println!("The last element of b is: {}", b[4]);
}
