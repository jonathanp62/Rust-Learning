// (#)main.rs   0.1.0   09/08/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

//! # Chapter 10 exercises
//!
//! ## Execution
//!
//! You can run this binary using Cargo:
//! ```bash
//! cargo run
//! ```

mod generics;
mod lifetimes;
mod traits;

use generics::generics;
use lifetimes::lifetimes;
use traits::traits;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
    generics();
    traits();
    lifetimes();
}
