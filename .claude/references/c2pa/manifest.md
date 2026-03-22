# C2PA Reference Collection

**Purpose:** Feasibility assessment for UMRS vault chain-of-custody integration
**Plan:** `.claude/plans/c2pa-vault-prototype.md`
**Downloaded:** 2026-03-22

## Documents

| Document | Version | File | SHA-256 | Source |
|---|---|---|---|---|
| C2PA Technical Specification | 2.2 (2025-05-01) | `C2PA_Specification_v2.2.pdf` | `9d0826049910f2304483c0399ec97ee28f4abc76e19e26a9a0ae0df654fa2abb` | https://spec.c2pa.org/specifications/specifications/2.2/specs/_attachments/C2PA_Specification.pdf |

## Online-Only References (no PDF available)

- C2PA Specification v2.3 (HTML): https://spec.c2pa.org/specifications/specifications/2.3/specs/C2PA_Specification.html
- C2PA Explainer v2.3 (HTML): https://spec.c2pa.org/specifications/specifications/2.3/explainer/Explainer.html
- C2PA AI/ML Guidance v2.3 (HTML): https://spec.c2pa.org/specifications/specifications/2.3/ai-ml/ai_ml.html
- C2PA Security Considerations: referenced in spec, URL TBD

## Rust Ecosystem

- **Crate:** `c2pa` v0.78.2 — https://crates.io/crates/c2pa
- **License:** MIT OR Apache-2.0
- **Repo:** https://github.com/contentauth/c2pa-rs
- **Maintainer:** ContentAuthenticity (Adobe CAI)
- **Status:** Beta (0.x.x)
- **MSRV:** 1.88.0+

### UMRS Compatibility Flags

- **FFI concern:** Librarian flagged OpenSSL FFI issue (#350) in the repo. The `c2pa-c-ffi` crate
  exists as a separate package, suggesting the core crate may have internal C FFI for crypto ops.
  This would conflict with `#![forbid(unsafe_code)]` and the FFI prohibition.
- **Spike action:** Phase 1 of the prototype plan must audit the dep tree for unsafe/FFI before
  any integration work begins.
