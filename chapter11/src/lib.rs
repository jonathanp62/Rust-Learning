// (#)lib.rs    0.1.0 09/10/2026
//
// @author   Jonathan Parker
// @version  1.0
// @since    1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

/// Adds two numbers together.
///
/// # Arguments
///
/// * `left` - The left number to add.
/// * `right` - The right number to add.
///
/// # Returns
///
/// * `u64` - The sum of the two numbers.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        
        assert_eq!(result, 4);
    }
}
