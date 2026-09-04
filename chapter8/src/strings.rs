// (#)strings.rs    0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

mod creation;
mod index;
mod iterate;
mod slice;
mod update;

/// Module that demonstrates string usage in Rust

/// The strings function
pub fn strings() {
    let mut s = creation::create_strings();
    
    update::update_string(&mut s);
    update::concatenate("Goodbye, World".to_string(), "!".to_string());
    update::format_strings("tic".to_string(), "tac".to_string(), "toe".to_string());

    index::index_strings();
    slice::slice_strings();
    iterate::iterate_strings();
}
