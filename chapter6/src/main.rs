// (#)main.rs   0.1.0   09/02/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

//! # Chapter 6 exercises
//!
//! ## Execution
//!
//! You can run this binary using Cargo:
//! ```bash
//! cargo run
//! ```

mod enum_values;
mod options;

use enum_values::enum_values;
use options::options;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
    enum_values();
    options();
}
