// (#)match_control_flow.rs 0.1.0   09/02/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// A coin enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// A US state enum
#[allow(dead_code)] // Suppress dead code warnings for unused variants
#[derive(Debug)]    // Allow the state name to be printed
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

/// The match_control_flow function demonstrates match control flow
pub fn match_control_flow() {
    println!("Match Control Flow");
    
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Maryland);
    
    println!("Value of penny: {}", value_in_cents(penny));
    println!("Value of nickel: {}", value_in_cents(nickel));
    println!("Value of dime: {}", value_in_cents(dime));
    println!("Value of quarter: {}", value_in_cents(quarter));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let dice_roll = 3;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

}

/// The value_in_cents function returns the value of a coin
/// 
/// # Arguments
/// 
/// * `coin` - A coin enum
/// 
/// # Returns
/// 
/// * `u8` - The value of the coin
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

/// The plus_one function adds one to an Option<i32>
/// 
/// # Arguments
/// 
/// * `x` - An Option<i32>
/// 
/// # Returns
/// 
/// * `Option<i32>` - An Option<i32> with the value plus one
fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), // The i binds to the value contained in Some
        }
}

/// The add_fancy_hat function adds a fancy hat
fn add_fancy_hat() {
    println!("Adding fancy hat");
}

/// The remove_fancy_hat function removes a fancy hat
fn remove_fancy_hat() {
    println!("Removing fancy hat");
}

/// The move_player function moves a player
/// 
/// # Arguments
/// 
/// * `num_spaces` - The number of spaces to move
fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces", num_spaces);
}
