//! MolComputeKit public facade.
//!
//! The implementation is split into a deterministic CPU/reference core, a
//! versioned protocol, and a verified Apple Metal runtime. The internal crate
//! names retain the original Burrete v1 ABI names for compatibility.

pub use burrete_compute_core as core;
pub use burrete_compute_metal as metal;
pub use burrete_compute_protocol as protocol;

/// Framework-level compatibility information.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameworkInfo {
    pub framework_version: &'static str,
    pub protocol_version: u32,
    pub metal_runtime_version: &'static str,
}

/// Returns the versions that an embedding application must record in its
/// provenance and backend evidence.
#[must_use]
pub const fn framework_info() -> FrameworkInfo {
    FrameworkInfo {
        framework_version: env!("CARGO_PKG_VERSION"),
        protocol_version: protocol::PROTOCOL_VERSION,
        metal_runtime_version: metal::NATIVE_METAL_RUNTIME_VERSION,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_bound_protocol_and_runtime_versions() {
        let info = framework_info();
        assert_eq!(info.framework_version, "0.1.0");
        assert_eq!(info.protocol_version, 1);
        assert_eq!(info.metal_runtime_version, "burrete-native-metal-v20");
    }
}
