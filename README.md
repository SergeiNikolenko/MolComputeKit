# MolComputeKit

[![CI](https://github.com/SergeiNikolenko/MolComputeKit/actions/workflows/ci.yml/badge.svg)](https://github.com/SergeiNikolenko/MolComputeKit/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

MolComputeKit is a native molecular-compute SDK for Apple Silicon. It combines
deterministic Rust reference implementations with bounded Metal kernels and a
versioned artifact/wire protocol. Python and MLX are development-time parity
oracles only and are not production runtime dependencies.

MolComputeKit is a low-level numerical and protocol SDK. It accepts typed inputs
such as packed fingerprints, atomic coordinates, distance constraints, and MMFF
parameters. Host applications remain responsible for molecule parsing,
chemistry perception, UI, persistence, and result presentation.

The first release is extracted from Burrete commit
`3da922d2eb0d57008793c36e2af198345f1fb8b2`. Stable v1 protocol identifiers
retain their `burrete.*` spelling so existing artifacts remain readable.

## Capabilities

- Exact packed-fingerprint Tanimoto, sparse CSR construction, deterministic
  Butina clustering, and similarity search.
- DG, KDG, ETDG, ETDGv2, ETKDG, ETKDGv2, ETKDGv3, and srETKDGv3 conformer
  generation with deterministic seeds and adaptive `N x K` batching.
- MMFF94/MMFF94s energy and optimization with all seven energy terms and
  automatic BFGS/L-BFGS selection.
- Batched Horn alignment, RMSD, Gaussian shape similarity, and electrostatic
  similarity.
- RM1, AM1, PM3, PM6, PM6_SP, PM6_D, PM6_D3H4, and AM1* semiempirical methods,
  including SCF, DIIS, adaptive damping, d orbitals, D3, H4, and H-H repulsion.
- Bounded jobs, snapshots, EnginePack/ResultPack artifacts, capability reports,
  and a versioned control protocol.

## Release scope

The current release is `v0.1.0`. Portable CPU/reference code and protocol types
build on Linux and macOS. Metal execution requires macOS 14 or newer on Apple
Silicon plus a separately built, hash-verified runtime package. The repository
does not ship a prebuilt `metallib` or signed helper binary.

The optional RDKit adapter prepares conformer and MMFF inputs. The public Rust
facade does not turn SMILES directly into fingerprints, conformers, or force
field parameters.

## Repository layout

| Path | Responsibility |
| --- | --- |
| `crates/molcompute-kit` | Public facade and compatibility metadata |
| `crates/molcompute-service` | Authenticated out-of-process Metal helper |
| `crates/burrete-compute-core` | Portable deterministic CPU/reference core |
| `crates/burrete-compute-metal` | Apple Metal execution and package verification |
| `crates/burrete-compute-protocol` | Requests, jobs, packs, artifacts, and wire contracts |
| `compute/metal` | Reviewed Metal source and kernel contracts |
| `compute/rdkit-conformer` | Optional RDKit DG/ETK/MMFF parameter extractor source |
| `compute/semiempirical` | Pinned fixtures, generators, reference data, and licenses |
| `schemas/compute` | JSON Schema boundary and golden fixtures |

## Use from Rust

```toml
[dependencies]
molcompute-kit = { git = "https://github.com/SergeiNikolenko/MolComputeKit", tag = "v0.1.0" }
```

```rust
use molcompute_kit::{core, framework_info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let versions = framework_info();
    println!("MolComputeKit {}", versions.framework_version);

    let query = core::Fingerprint2048::from_words([u64::MAX; 32]);
    let library = [query, core::Fingerprint2048::ZERO];
    let scores = core::score_tanimoto_query(
        &query,
        &library,
        core::TanimotoQueryOptions::new(8 * 1024 * 1024)?,
    )?;

    println!("exact match: {:.3}", scores[0].similarity());
    println!("empty fingerprint: {:.3}", scores[1].similarity());
    Ok(())
}
```

Run the examples in the current source tree without a Metal runtime:

```bash
cargo run -p molcompute-kit --example cpu_similarity
cargo run -p molcompute-kit --example cpu_clustering
cargo run -p molcompute-kit --example cpu_alignment
cargo run -p molcompute-kit --example cpu_semiempirical
```

The `v0.1.0` tag contains the original `cpu_similarity` example only. All four
commands above document the current source tree.

Embedding applications load a verified Metal runtime through
`molcompute_kit::metal::MetalComputeRuntime`. Loading verifies the package
metadata, source and library hashes, device limits, and startup known-answer
tests. Applications must publish the actual backend, runtime identity,
precision, fallback reason, and package hashes from the protocol types rather
than claiming GPU execution from configuration alone.

For process isolation, build the standalone helper with:

```bash
cargo build --release -p molcompute-service
```

Applications that embed `ComputeServiceClient` must add the separate
`molcompute-service` package; it is not re-exported by the facade.

## Build and test

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
node tests/test-compute-schema-contract.mjs
node tests/test-metal-tanimoto-contract.mjs
```

On macOS with the Xcode Metal Toolchain installed:

```bash
compute/metal/build-metallib.sh compute/metal/runtime
```

## Documentation

The reader-facing Nextra site lives in [`docs/content`](docs/content). To run it
locally:

```bash
cd docs
npm ci
npm run dev
```

Start with the [documentation map](docs/README.md), then read
[Architecture](docs/ARCHITECTURE.md), [Validation](docs/VALIDATION.md),
[Extraction record](docs/EXTRACTION.md), [Integration](docs/INTEGRATION.md), and
[Attribution](docs/third-party/mlxmolkit-provenance.md).

## Platform policy

The CPU/reference and protocol crates are portable. The production GPU backend
targets macOS 14 or newer on Apple Silicon. Unsupported platforms compile the
stub backend and must report Metal as unavailable.

## License and provenance

MolComputeKit's original code is MIT licensed. Adapted equations, generated
tables, fixtures, and optional RDKit sources retain their applicable notices.
Read [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) before redistribution.
