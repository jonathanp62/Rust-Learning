// (#)enum_values.rs    0.1.0   09/02/2026
//
// @author   Jonathan Parker
// @version  0.1.0
// @since    0.1.0
//
// Copyright (c) 2026 by Jonathan Parker.

// SPDX-License-Identifier: MIT

mod ip_combo;

use ip_combo::IpCombo;

/// An enum representing different IP address kinds
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

/// A struct representing an IP address
/// 
/// # Fields
/// 
/// * `kind` - An `IpAddrKind` enum value representing the IP address type
/// * `address` - A `String` representing the IP address
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/// The enum_values function demonstrates enums
pub fn enum_values() {
    println!("Enum values");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);    

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    echo_ip(home);
    echo_ip(loopback);

    let home = IpCombo::V4(String::from("127.0.0.1"));
    let loopback = IpCombo::V6(String::from("::1"));
    
    echo_combo(home);
    echo_combo(loopback);
}

/// The route function demonstrates routing based on IP address kind
/// 
/// # Arguments
/// 
/// * `ip_type` - An `IpAddrKind` enum value representing the IP address type
fn route(ip_type: IpAddrKind) {
    println!("Routing to IP: {:?}", ip_type);   // Basic debug formatting
}

/// The echo_ip function demonstrates printing an IP address
/// 
/// # Arguments
/// 
/// * `ip` - An `IpAddr` struct representing the IP address
fn echo_ip(ip: IpAddr) {
    println!("IP: {:?} - {}", ip.kind, ip.address);
}


/// The echo_combo function demonstrates printing an IP address
/// 
/// # Arguments
/// 
/// * `ip` - An `IpCombo` enum value representing the IP address
fn echo_combo(ip: IpCombo) {
    let (kind, addr) = match ip {
        IpCombo::V4(addr) => ("V4", addr),
        IpCombo::V6(addr) => ("V6", addr),
    };
    println!("IP: {} - {}", kind, addr);
}
