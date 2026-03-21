---
name: citation-conventions
description: Citation format conventions established by the UMRS team, including exemptions
type: project
---

## Canonical Citation Forms (doc comments and `///`)

- `NIST SP 800-53` (not `NIST 800-53`) — required in all Rust doc comments
- `NSA RTB` followed by principle (e.g., `NSA RTB RAIN`)
- `NIST SP 800-218 SSDF` followed by practice (e.g., `NIST SP 800-218 SSDF PW.4`)
- `FIPS 140-2` or `FIPS 140-3` (with dash)
- `CMMC` followed by domain and level (e.g., `CMMC SC.L2-3.13.10`)
- `CCE-NNNNN-N` always follows a NIST control, never alone

## Display String Exemption

Runtime output strings (e.g., `nist_controls` fields in `IndicatorDescriptor` catalog entries,
`nist_controls` in JSON output) **may use the abbreviated `NIST 800-53` form** for display compactness.
This is explicitly permitted by the Citation Format Rule in `.claude/rules/rust_design_rules.md`.

**Do not flag abbreviated citations in `IndicatorDescriptor.nist_controls` fields** — they are
runtime display strings, not doc comments.

## CCE Convention

- CCE must be paired with the STIG version and date on the same line or block
- Example: `// CCE-89232-3 (RHEL 10 STIG, scap-security-guide 2026-03-17)`
- Never cite CCE alone without a NIST control

**Why:** Auditors queried why `NIST 800-53` appeared in catalog data; the exemption is explicit
but not obvious from the rule text. This note prevents false audit findings.
