//! `ip` — IP address, CIDR, and socket utilities for the handl runtime.
//!
//! `src/index.hnl` is the single source of truth for the ip interface
//! (`op` declarations); `module.rs` holds the Rust atoms behind them
//! (`IpParse`, `CidrParse`, …) registered as handls in an [`OpRegistry`].
//!
//! Extracted from `stdlib/crates/plugin-lib-stdlib/src/ops/ip/` per the
//! stdlib → `lib-*` split (ADR-0128). The [`IpModule`] re-export is
//! preserved for consumers that resolve the module through
//! `kit-canonical`.
//!
//! The `IpLibraryPlugin` / `LibraryPlugin` shim is transitional: it is kept
//! because `kit-canonical` still registers it directly; the stdlib package
//! table now contributes [`IpModule`] instead.
#![allow(dead_code)]

use std::sync::Arc;

use handl_plugin_lib_handl::{HandlLibraryPlugin, OpRegistry};

mod addr;
mod cidr;
mod error;
mod module;
mod socket;

#[cfg(test)]
mod tests;

pub use addr::{IpAddr, Ipv4, Ipv6};
pub use cidr::{Cidr, Cidr4, Cidr6};
pub use error::IpError;
pub use module::{
    CidrBroadcast, CidrContains, CidrFormat, CidrNetwork, CidrParse, IpFormat, IpIsLinkLocal,
    IpIsLoopback, IpIsMulticast, IpIsPrivate, IpIsUnspecified, IpModule, IpParse, SocketFormat,
    SocketParse,
};
pub use socket::SocketAddr;

/// The handl source declaring the ip `op` interface — the single source
/// of truth for the ip API surface.
pub const IP_HNL: &str = include_str!("index.hnl");

/// The op names declared in `index.hnl`.
pub const IP_OPS: &[&str] = &[
    "ip.parse",
    "ip.format",
    "ip.is_loopback",
    "ip.is_multicast",
    "ip.is_private",
    "ip.is_link_local",
    "ip.is_unspecified",
    "ip.cidr.parse",
    "ip.cidr.format",
    "ip.cidr.contains",
    "ip.cidr.network",
    "ip.cidr.broadcast",
    "ip.socket.parse",
    "ip.socket.format",
];

// ---------------------------------------------------------------------------
// Op Registry
// ---------------------------------------------------------------------------

/// Build the [`OpRegistry`] for ip, registering the Rust handls under
/// their op names.
pub fn ip_op_registry() -> OpRegistry {
    let mut registry = OpRegistry::new();
    registry.register("ip.parse", Arc::new(IpParse));
    registry.register("ip.format", Arc::new(IpFormat));
    registry.register("ip.is_loopback", Arc::new(IpIsLoopback));
    registry.register("ip.is_multicast", Arc::new(IpIsMulticast));
    registry.register("ip.is_private", Arc::new(IpIsPrivate));
    registry.register("ip.is_link_local", Arc::new(IpIsLinkLocal));
    registry.register("ip.is_unspecified", Arc::new(IpIsUnspecified));
    registry.register("ip.cidr.parse", Arc::new(CidrParse));
    registry.register("ip.cidr.format", Arc::new(CidrFormat));
    registry.register("ip.cidr.contains", Arc::new(CidrContains));
    registry.register("ip.cidr.network", Arc::new(CidrNetwork));
    registry.register("ip.cidr.broadcast", Arc::new(CidrBroadcast));
    registry.register("ip.socket.parse", Arc::new(SocketParse));

// ---------------------------------------------------------------------------
// IpLibraryPlugin (wraps HandlLibraryPlugin with op registry)
// ---------------------------------------------------------------------------

/// A `Library` plugin that registers the `ip` implementation via the
/// generic [`HandlLibraryPlugin`] with an [`OpRegistry`].
pub struct IpLibraryPlugin {
    inner: HandlLibraryPlugin,
}

impl IpLibraryPlugin {
    /// Create a new `IpLibraryPlugin`.
    pub fn new() -> Self {
        let registry = Arc::new(ip_op_registry());
        Self {
            inner: HandlLibraryPlugin::new_with_caps("ip", "lib.ip", "lib-ip", IP_HNL, &[])
                .with_ops(IP_OPS, registry),
        }
    }
}

impl Default for IpLibraryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl handl_platform::Plugin for IpLibraryPlugin {
    fn id(&self) -> &str {
        self.inner.id()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn capabilities(&self) -> &[handl_platform::CapabilityId] {
        self.inner.capabilities()
    }

    fn metadata(
        &self,
        capability: &handl_platform::CapabilityId,
    ) -> Option<handl_platform::CapabilityMetadata> {
        self.inner.metadata(capability)
    }

    fn contribute(
        &self,
        ctx: &mut handl_platform::ContributionContext,
    ) -> Result<(), handl_platform::PlatformError> {
        self.inner.contribute(ctx)
    }

    fn run(
        &self,
        request: &handl_platform::CapabilityRequest,
        ctx: &mut handl_platform::PluginContext,
    ) -> Result<handl_platform::CapabilityResponse, handl_platform::PlatformError> {
        self.inner.run(request, ctx)
    }
}

/// Build a [`HandlLibraryPlugin`] for ip with the op registry.
pub fn ip_handl_plugin() -> HandlLibraryPlugin {
    let registry = Arc::new(ip_op_registry());
    HandlLibraryPlugin::new_with_caps("ip", "lib.ip", "lib-ip", IP_HNL, &[])
        .with_ops(IP_OPS, registry)
}

/// Legacy `LibraryPlugin` alias — delegates to [`IpLibraryPlugin`].
pub type LibraryPlugin = IpLibraryPlugin;

    registry.register("ip.socket.format", Arc::new(SocketFormat));
    registry
}
