# Extraction record

MolComputeKit v0.1.0 was extracted from Burrete commit
`3da922d2eb0d57008793c36e2af198345f1fb8b2` on 2026-07-17.

## Included

- `crates/burrete-compute-core` as the portable scientific reference core.
- `crates/burrete-compute-metal` as the verified Metal backend.
- `crates/burrete-compute-protocol` as the bounded compatibility protocol.
- The compute helper transport and dispatch modules, moved into
  `crates/molcompute-service` without Tauri, Grid, preview, or database code.
- Metal kernels and contracts, RDKit extractor sources and fixtures,
  semiempirical generators/reference assets/licenses, schemas, and golden
  fixtures.
- A new `molcompute-kit` facade, standalone CI, documentation, and release
  workflow.

## Intentionally excluded

- Burrete Grid selection, SQL persistence, menus, WebView/Tauri commands, Mol*
  presentation, and report tabs. These are host adapters rather than molecular
  compute framework responsibilities.
- Python/MLX packaging, command-line examples, caches, and upstream benchmark
  outputs.
- Prebuilt metallib and helper binaries. They are toolchain- and signing-bound
  release artifacts and must be generated or attached to a tagged release.

## Compatibility policy

Existing `burrete.*`, `BURRETE_COMPUTE_*`, EnginePack, ResultPack, BCEX, BMFX,
and Metal entrypoint identifiers remain unchanged in v0.1.0. Neutralizing them
requires a versioned v2 migration with dual readers; silent renaming is
forbidden.

