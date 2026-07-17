# Security policy

Please report vulnerabilities privately through GitHub Security Advisories for
this repository. Do not open a public issue for an unbounded allocation,
artifact path traversal, helper-authentication bypass, protocol replay, or
runtime-package verification failure.

The supported line is the latest tagged release. Compute inputs are untrusted:
hosts must run protocol validation, enforce resource limits, reject symlinks in
verified packages, and never publish results after a failed parity or hash gate.

