---
name: Control Citation Conventions
description: Canonical citation forms for NIST, CMMC, NSA RTB. Tier-awareness rules. Common incorrect citations and corrections. Load when auditing or adding control annotations.
type: project
---

## Canonical citation forms (from rust_design_rules.md)

- NIST: `NIST SP 800-53` (not `NIST 800-53`)
- NSA RTB: `NSA RTB` + principle name (e.g., `NSA RTB RAIN`)
- SSDF: `NIST SP 800-218 SSDF` + practice (e.g., `NIST SP 800-218 SSDF PW.4`)
- FIPS: `FIPS 140-2` or `FIPS 140-3` (with dash)
- CMMC: `CMMC` + domain and level (e.g., `CMMC SC.L2-3.13.10`)
- CCE: `CCE-NNNNN-N` uppercase, always follows a NIST citation, never standalone

## Common incorrect citations observed in codebase

- `NIST 800-53 CM-6(a)` → should be `NIST SP 800-53 CM-6(a)` (missing "SP")
  Seen in: `umrs-platform/src/posture/catalog.rs` nist_controls field (runtime display strings — these are exempt from the rule per rust_design_rules.md Citation Format Rule)
- Runtime `nist_controls` fields in catalog entries may use abbreviated forms per project rules

## Tiered annotation expectations

- Modules: always need a `## Compliance` section with canonical citations
- Security-critical types and functions: require explicit control citations
- Simple accessors and Display impls: no citation required if parent type is annotated
- Do not flag trivial items

## AU-10 awareness

AU-10 (non-repudiation) is FedRAMP HIGH only. Do not flag missing AU-10 citations as a gap
in MODERATE-tier code unless the code is specifically handling non-repudiation functionality.

## SI-7 tier note

SI-7 citations are correct at MODERATE and above. They are not wrong even if found in code
that also runs at LOW — the code is intended for MODERATE/HIGH deployments.
