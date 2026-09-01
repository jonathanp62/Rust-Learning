// (#)struct_definition.rs  0.1.0   09/01/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The user struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Define and instantiating structs
pub fn struct_definition() {
    println!("Defining and instantiating structs");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    
    if user1.active {
        println!("User username: {}", user1.username);
        println!("User email: {}", user1.email);
        println!("User sign in count: {}", user1.sign_in_count);
    }

    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user-two"),
    );
    
    // Struct Update Syntax - .. copies remaining fields from user2
    let user3 = User {
        username: String::from("user-three"),
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("User3 username: {}", user3.username);
    println!("User3 email: {}", user3.email);
    println!("User3 sign in count: {}", user3.sign_in_count);
    
    tuple_structs();
    unit_like_structs();
}

/// Build user function
/// 
/// # Arguments
/// 
/// * `email` - The email address of the user
/// * `username` - The username of the user
/// 
/// # Returns
/// 
/// * `User` - The user object
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

/// Tuple structs
fn tuple_structs() {
    println!("Tuple structs");
    
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);
}

fn unit_like_structs() {
    println!("Unit-like structs");
    
    struct AlwaysEqual;
    
    let _subject = AlwaysEqual;
}
