# Contributing

Changes must preserve bounded inputs, deterministic CPU behavior, truthful
backend evidence, and the v1 artifact/wire contracts unless a new protocol
version is introduced.

Before opening a pull request, run:

```bash
./scripts/check.sh
```

Changes to Metal source require matching kernel-contract updates and CPU/Metal
parity evidence. Changes based on third-party implementations or data require a
new entry in the provenance ledger and the applicable license notice.

Do not add Python, MLX, RDKit Python, or MOPAC as a production runtime
dependency. Reference tooling and fixture generation must remain optional.

