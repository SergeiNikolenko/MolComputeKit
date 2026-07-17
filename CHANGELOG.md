# Changelog

## 0.1.0 - 2026-07-17

- Extracted the native molecular CPU/Metal compute layer from Burrete commit
  `3da922d2eb0d57008793c36e2af198345f1fb8b2`.
- Added a standalone Rust facade, CI, contract tests, architecture and
  integration documentation, and complete third-party provenance.
- Extracted the authenticated compute helper from Tauri into the independent
  `molcompute-service` crate.
- Preserved the v1 `burrete.*` protocol identifiers for artifact compatibility.
