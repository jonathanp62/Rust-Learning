// (#)main.rs   0.1.0   09/01/2026
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

mod rectangles;
mod rectangle_methods;
mod struct_definition;

use rectangles::rectangles;
use rectangle_methods::rectangle_methods;
use struct_definition::struct_definition;

/// The main entry point for application execution.
///
/// This function is called automatically by the runtime when the binary starts.
fn main() {
    struct_definition();
    rectangles();
    rectangle_methods();
}
