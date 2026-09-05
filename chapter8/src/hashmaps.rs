// (#)hashmaps.rs   0.1.0   09/03/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

mod access;
mod creation;
mod exercises;
mod update;

/// Module that demonstrates hashmap usage in Rust

/// The hashmaps function
pub fn hashmaps() {
    let mut scores = creation::create_hashmaps();
    
    update::update_hashmap(&mut scores);    
    access::access_hashmap(&scores);
    exercises::exercises();
}
