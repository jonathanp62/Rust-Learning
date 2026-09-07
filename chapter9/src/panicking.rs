// (#)panicking.rs  0.1.0   09/07/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Module that demonstrates panicking in Rust

/// The panicking function
pub fn panicking() {
    crash_and_burn(false);
}

/// The crash and burn function
/// 
/// # Arguments
/// 
/// * `do_panic` - A boolean value that determines whether to panic or not
fn crash_and_burn(do_panic: bool) {
    if do_panic {
        panic!("Crash and burn!");
    } else {
        println!("Skipping the crash and burn");
    }
}
