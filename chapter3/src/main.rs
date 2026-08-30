// (#)main.rs   0.1.0   08/27/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

//! # Chapter 3 exercises
//!
//! ## Execution
//!
//! You can run this binary using Cargo:
//! ```bash
//! cargo run
//! ```

mod control_flow;
mod data_types;
mod exercises;
mod functions;
mod variables;

use control_flow::control_flow;
use data_types::data_types;
use exercises::exercises;
use functions::functions;
use variables::variables;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
    variables();
    data_types();
    functions();
    control_flow();
    exercises();
}
