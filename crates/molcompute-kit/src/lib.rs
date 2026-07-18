//! Public facade for MolComputeKit.
//!
//! The facade keeps three integration surfaces under one dependency:
//!
//! - [`core`] contains deterministic CPU/reference algorithms. Use it for
//!   portable execution, validation, and Metal parity checks.
//! - [`metal`] loads a hash-verified Apple Metal runtime package. Runtime
//!   availability must be observed, never inferred from configuration.
//! - [`protocol`] defines bounded requests, capabilities, jobs, packs, and wire
//!   messages.
//!
//! The facade operates on typed numerical inputs. Molecule parsing, chemistry
//! perception, persistence, and UI remain responsibilities of the embedding
//! application.
//!
//! # CPU example
//!
//! ```rust
//! use molcompute_kit::core::{
//!     score_tanimoto_query, Fingerprint2048, TanimotoQueryOptions,
//! };
//!
//! let query = Fingerprint2048::from_words([u64::MAX; 32]);
//! let library = [query, Fingerprint2048::ZERO];
//! let scores = score_tanimoto_query(
//!     &query,
//!     &library,
//!     TanimotoQueryOptions::new(8 * 1024 * 1024)?,
//! )?;
//!
//! assert_eq!(scores[0].similarity(), 1.0);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

/// Deterministic CPU/reference algorithms and typed numerical inputs.
pub use burrete_compute_core as core;
/// Hash-verified Apple Metal runtime and package inspection APIs.
pub use burrete_compute_metal as metal;
/// Bounded request, capability, artifact, job, and wire contracts.
pub use burrete_compute_protocol as protocol;

/// Version identities that an embedding application records with its results.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameworkInfo {
    /// SemVer version of the public `molcompute-kit` facade.
    pub framework_version: &'static str,
    /// Version of the request, artifact, and wire protocol.
    pub protocol_version: u32,
    /// Identity required from a compatible Metal runtime package.
    pub metal_runtime_version: &'static str,
}

/// Returns the versions that an embedding application must record in its
/// provenance and backend evidence.
///
/// This reports compatibility identities, not Metal availability. Use a
/// capability report or load [`metal::MetalComputeRuntime`] before presenting a
/// GPU backend as available.
///
/// # Examples
///
/// ```rust
/// let info = molcompute_kit::framework_info();
/// assert_eq!(info.protocol_version, 1);
/// assert!(!info.framework_version.is_empty());
/// assert!(!info.metal_runtime_version.is_empty());
/// ```
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
