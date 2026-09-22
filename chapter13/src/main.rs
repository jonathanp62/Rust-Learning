// (#)main.rs   0.1.0 09/16/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

use chapter13::Inventory;
use chapter13::ShirtColor;
use chapter13::sort_rectangles_by_height;
use chapter13::sort_rectangles_by_width;
use std::time::Duration;

use std::thread;
/// Main function.
fn main() {
    run();
}

/// Run the main program
fn run() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closures may provide the type information

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    println!("Result: {}", expensive_closure(5));

    let add_one = |x| x + 1;

    println!("Result: {}", add_one(5));
    
    borrowing();
    borrowing_mutably();

    sort_rectangles_by_width();
    sort_rectangles_by_height();

    iterating_using_for();
    iterating_using_next();
    consuming_the_iterator();
}

/// Borrowing immutability 
fn borrowing() {
    let list = vec![1, 2, 3];

    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");   // No aruments to the closure; list is an upvar captured from the surronding environment

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

/// Borrowing mutability 
fn borrowing_mutably() {
    let mut list = vec![1, 2, 3];

    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);  // This closure mutates the captured variable

    // Can't println the list as it has been borrowed mutably by the closure

    borrows_mutably();  // The mutable borrow ends after the closure is run
    println!("After calling closure: {list:?}");
}

/// Basic iteration
fn iterating_using_for() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

/// Iteration using next()
fn iterating_using_next() {
    let v1 = vec![4, 5, 6];
    let mut v1_iter = v1.iter();

    println!("Got: {}", v1_iter.next().unwrap());   // Some(&4)
    println!("Got: {}", v1_iter.next().unwrap());   // Some(&5)
    println!("Got: {}", v1_iter.next().unwrap());   // Some(&6)
    println!("Got: {:?}", v1_iter.next());          // None
}

/// Consuming iterators
fn consuming_the_iterator() {
        let v1 = vec![2, 4, 6];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        // The v1_iter is no longer available after being consumed
        
        println!("Total: {}", total);
}
