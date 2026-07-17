# Integration guide

## Rust applications

Depend on `molcompute-kit` for the facade or on an individual internal crate
when a lower-level boundary is intentional. Start from the protocol limits,
freeze input identity, build validated packs, then select CPU or Metal through
an explicit backend policy.

## macOS applications

A production host should package the generated Metal runtime directory as a
read-only resource. Load it through the verified runtime API and retain the
reported package identity in every result artifact.

Long-running or untrusted jobs should execute through the separately signed
`molcompute-service` binary. Hosts must place it in their platform-appropriate
helper location and bind helper identity, runtime hashes, resource limits, and
per-job capabilities before accepting output. The v1 environment and wire names
retain the `BURRETE_COMPUTE_*` prefix for compatibility.

## RDKit adapter

The optional `compute/rdkit-conformer` source extracts DG/ETK constraints and
MMFF parameters from official RDKit APIs. It is deliberately separate from the
numeric Metal runtime. Applications may provide another extractor if it emits
the same validated binary contracts and records its chemistry engine identity.

## Result evidence

Do not expose a single `gpu: true` flag. Publish at least:

- requested and actual backend;
- device and runtime identity;
- precision policy;
- host and GPU timing;
- fallback reason;
- input, engine, result, and artifact hashes;
- convergence and per-record failure state.
