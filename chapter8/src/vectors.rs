// (#)vectors.rs    0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

mod creation;
mod iteration;
mod read;
mod update;

/// Module that demonstrates vector usage in Rust

/// The vectors function
pub fn vectors() {
    let mut v = creation::create_vectors();

    update::update_vector(&mut v);
    read::read_vector(&v);
    iteration::iterate_vector(&v);
}
