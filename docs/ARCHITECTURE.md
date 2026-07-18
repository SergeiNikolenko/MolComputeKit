# Architecture

MolComputeKit separates scientific truth, execution, persistence, and product
integration. Applications should depend on the public facade and keep their UI
or database adapters outside the framework.

## Layers

1. `burrete-compute-core` contains deterministic CPU/reference algorithms. It
   is the numerical oracle for GPU parity and the truthful fallback backend.
2. `burrete-compute-metal` owns Metal buffers, dispatch, batching, package
   verification, device evidence, and post-dispatch parity gates.
3. `burrete-compute-protocol` defines bounded requests, jobs, snapshots,
   artifacts, packed arrays, capabilities, errors, and wire messages.
4. `molcompute-kit` is the stable integration facade. It exposes the three
   layers without tying them to Tauri, Grid, Mol*, or a particular application.
5. `molcompute-service` provides the authenticated file-descriptor data plane
   and isolated execution process used by production hosts.

## GPU-first does not mean GPU-only

Dense numerical work runs on Metal when the verified runtime and memory plan
admit it. Sequential policy, chemistry perception, validation, and publication
remain on the host when moving them to the GPU would reduce correctness or add
no useful parallelism. Execution plans and result provenance report the actual
backend of every stage; a host must not collapse a multi-stage pipeline into a
single `GPU` claim.

Examples include deterministic Butina representative selection after Metal CSR
construction and RDKit topology/parameter extraction before Metal DG/ETK/MMFF.

## Memory and determinism

- Similarity construction is tiled and never materializes a dense `N x N`
  matrix.
- Conformer constraints are stored once per molecule and reused across its
  conformers.
- Every allocation is checked against protocol and request budgets before
  dispatch.
- Conformer random streams derive from immutable molecule, job, variant,
  conformer, and retry identities; batch size cannot change the assigned seed.
- Result publication is artifact-based and records exact engine identities.

## Compatibility

The v1 JSON and binary identifiers intentionally retain `burrete.*` names.
Renaming them would invalidate persisted artifacts and existing host
integrations. Future neutral identifiers require a new protocol version and an
explicit migration, not an in-place spelling change.

## Host integration boundary

Host applications own molecule selection, Grid/database queries, UI actions,
3D rendering, and report presentation. They translate those inputs into bounded
MolComputeKit requests and translate immutable results back to their product
surface.
