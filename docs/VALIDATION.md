# Validation

MolComputeKit uses layered validation because a successful GPU command buffer
is not evidence of scientific correctness.

## Continuous checks

- Rust formatting and Clippy with warnings denied.
- Full workspace unit and contract tests on Linux and macOS.
- JSON Schema fixtures in both valid and invalid classes.
- Metal source-to-contract ABI inspection.
- CPU known-answer tests for similarity, conformers, MMFF, alignment, and
  semiempirical methods.
- macOS Metal tests for source compilation and CPU/GPU parity where supported
  by the runner.

## Reference sources

- RDKit 2025.03.4 for fingerprints, conformer parameter extraction, and MMFF.
- `mlxmolkit` commit `9e7337f6f93c40a39ad0187991151944a4f1e274` for selected
  behavior and formula parity.
- PYSEQM commit `6ced9ea66160428e06d37df18e9f565b8123f84a` for NDDO/SCF
  parity.
- OpenMOPAC commit `052691223d19935a89f0fe18cd12301bd83e4201` for parameters
  and correction behavior.

The complete file-by-file mapping is in
`docs/third-party/mlxmolkit-provenance.md`.

## Real-device evidence

The extracted v20 implementation was exercised on Apple M2 Pro before the
standalone split. Evidence covered exact Tanimoto, alignment and scoring, all
eight semiempirical method identities, DG, ETK, MMFF, stereo validation,
adaptive batching, and the 32-case all-variant conformer corpus.

GitHub-hosted CI is a regression gate, not a replacement for signed-package
tests on physical Apple Silicon. Release candidates should additionally record
the device model, unified-memory size, macOS version, metallib SHA-256, helper
SHA-256 when a helper is used, and actual backend for every workflow.

The standalone split was then rerun against the same hash-bound v20 package.
The extracted helper passed Tanimoto, alignment/shape/ESP, RM1 SCF, DG, ETK,
MMFF, and stereo dispatch. The direct runtime passed the complete 24-case
RDKit MMFF94/MMFF94s CPU/Metal corpus. The verified metallib SHA-256 was
`341d858756cfd33438304e0d643d4ad647081df7678f88e407cc2734e87a2c84`.

## Local commands

```bash
./scripts/check.sh
```

The Metal package build additionally requires the Xcode Metal Toolchain:

```bash
xcodebuild -downloadComponent MetalToolchain
compute/metal/build-metallib.sh compute/metal/runtime
```
