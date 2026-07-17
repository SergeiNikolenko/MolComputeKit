#!/usr/bin/env bash

set -euo pipefail

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
node tests/test-compute-schema-contract.mjs
node tests/test-metal-tanimoto-contract.mjs

