// (#)mod.rs    0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// The calculator module provides basic arithmetic operations.
///
/// This module contains functions for adding and subtracting integers.
/// 
/// The mod.rs style is older, but still used in some codebases. The
/// alternative is to put each module in its own file, as we do in the 
/// garden module.

/// Adds two numbers together.
///
/// # Arguments
///
/// * `a` - An integer to be added.
/// * `b` - An integer to be added.
///
/// # Returns
///
/// * `i32` - The sum of the two numbers.
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts two numbers.
///
/// # Arguments
///
/// * `a` - An integer to be subtracted.
/// * `b` - An integer to be subtracted.
///
/// # Returns
///
/// * `i32` - The difference of the two numbers.
pub fn subtract_numbers(a: i32, b: i32) -> i32 {
    a - b
}
