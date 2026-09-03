// (#)main.rs   0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

//! # Chapter 5 exercises
//!
//! ## Execution
//!
//! You can run this binary using Cargo:
//! ```bash
//! cargo run
//! ```

mod hashmaps;
mod strings;
mod vectors;

use hashmaps::hashmaps;
use strings::strings;
use vectors::vectors;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
   vectors();
   hashmaps();
   strings();
}
