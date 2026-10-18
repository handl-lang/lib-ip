//! `ip` — IP address, CIDR, and socket utilities for the handl runtime.
//!
//! Pure-`.hnl` package: `index.hnl` (contract + impl imports) +
//! `index_flat.hnl` (flattened single-source for `stdlib::MODULES`).
//! `ip.cidr.*`/`ip.socket.*` resolve via nested records in the module's
//! export — no Rust impl.

/// The handl source declaring the ip `op` interface.
pub const IP_HNL: &str = include_str!("index.hnl");

/// `.hnl` package source set — index + sibling files (wired into kits via
/// `handl_module_from_sources`).
pub const IP_FUNCS: &[&str] = &[
    "parse","format","is_loopback","is_multicast","is_private","is_link_local","is_unspecified",
    "cidr.parse","cidr.format","cidr.contains","cidr.network","cidr.broadcast",
    "socket.parse","socket.format",
];
pub const IP_SOURCES: &[(&str, &str, &[&str])] = &[
    ("num", include_str!("internal/num.hnl"), &["dec_int","int_dec"]),
    ("ipv4", include_str!("ipv4.hnl"), &["parse","format","is_loopback","is_multicast","is_private","is_link_local","is_unspecified","parse_int","format_int","int_of"]),
    ("cidr", include_str!("cidr.hnl"), &["cidr_parse","cidr_format","contains","network","broadcast"]),
    ("socket", include_str!("socket.hnl"), &["socket_parse","socket_format"]),
];

/// Flattened single-source `.hnl` (for `stdlib::MODULES` table rows).
pub const IP_FLAT_HNL: &str = include_str!("index_flat.hnl");
