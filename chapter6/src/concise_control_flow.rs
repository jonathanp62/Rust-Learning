// (#)concise_control_flow.rs   0.1.0   09/02/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// A coin enum
enum Coin {
    Dime,
    Quarter(UsState),
}

/// A US state enum
#[derive(Debug)]    // Allow the state name to be printed
enum UsState {
    Alabama,
    Alaska,
}

/// Implementation of the UsState enum
impl UsState {
    /// Check if the state existed in a given year
    /// 
    /// # Arguments
    /// 
    /// * `year` - The year to check
    /// 
    /// # Returns
    /// 
    /// * `bool` - True if the state existed in the given year, false otherwise
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

/// The concise_control_flow function demonstrates concise control flow
pub fn concise_control_flow() {
    println!("Concise Control Flow");

    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // if let is a concise alternative to match for single patterns
    // max binds to the value inside Some

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
    let alabama = Coin::Quarter(UsState::Alabama);
    let alaska = Coin::Quarter(UsState::Alaska);
    let dime = Coin::Dime;

    if let Some(description) = describe_state_quarter(alabama) {
        println!("{description}");
    }
    
    if let Some(description) = describe_state_quarter(alaska) {
        println!("{description}");
    }
    
    if let Some(description) = describe_state_quarter(dime) {
        println!("{description}");
    } else {
        println!("Not a state quarter");
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
