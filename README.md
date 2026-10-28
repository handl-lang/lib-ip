# lib-ip

`ip` — IP address, CIDR, and socket primitives (Handl stdlib port).

Part of the stdlib → `lib-*` split (ADR-0128); moved from
`stdlib/crates/plugin-lib-stdlib/src/ops/ip/` and
`stdlib/handl-packages/ip/`.

- `src/index.hnl` — the interface: `op` declarations for `ip.*`,
  `ip.cidr.*`, and `ip.socket.*`.
- `src/addr.rs`, `src/cidr.rs`, `src/socket.rs`, `src/error.rs` — the
  value types (`IpAddr`, `Cidr`, `SocketAddr`, `IpError`).
- `src/module.rs` — the Rust handls (`IpParse`, `CidrParse`, …) and the
  aggregator `IpModule`.
- `src/lib.rs` — `IP_HNL` / `IP_OPS` pins plus `ip_op_registry()`.

## License

MIT.
