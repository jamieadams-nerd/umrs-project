# Security-Auditor Agent Memory Index

## Project Context

- Target: RHEL 10, SELinux enforcing, FIPS assumed active, DoD/government audit exposure
- All public Rust items need NIST SP 800-53, CMMC, or NSA RTB citations per CLAUDE.md
- Citation format rule: `NIST SP 800-53` (not `NIST 800-53`); `NSA RTB`; `NIST SP 800-218 SSDF`

## Control Mapping Conventions Established

- `kattr` reads verified against kernel fs magic → NIST SP 800-53 SI-7 + NSA RTB RAIN
- Compile-time path binding → NIST SP 800-218 SSDF PW.4 + NSA RTB (Compile-Time Path Binding)
- TPI (dual-path parsing) → NSA RTB RAIN (fail-closed on disagreement)
- Trust gate (config reads gated on kernel subsystem active) → NIST SP 800-53 CM-6
- Error information discipline (no sensitive data in errors) → NIST SP 800-53 SI-11
- Security findings as enum variants → NIST SP 800-53 AU-3
- `#[must_use]` on security-relevant return values → NIST SP 800-53 SI-10, SA-11

## Known Annotation Debt (as of 2026-03-15)

- No outstanding annotation debt identified in `umrs-platform/src/posture/` or `kattrs/`
  as of last review. All signal catalog entries carry control citations.

## Advisory Documents

- [Kernel Security Tab Content Advisory](kernel-tab-content-review.md) — 2026-03-15
  Comprehensive input for Phase 7 of the TUI Enhancement Plan. Covers:
  recommended items, proposed groupings (8 groups), CA-7 priority ordering,
  CPU security information detail, items missing from current signal catalog,
  two-column layout recommendations, display value policy.

- [Three-Layer Evaluation Model Advisory](three-layer-evaluation-review.md) — 2026-03-15
  Analysis of the proposed live/configured/expected evaluation model for kernel security
  assessment. Covers: SP 800-53A methodology alignment, missing finding states
  (Unverifiable, ReliesOnDefault, PolicyDependent), expected-value governance and OSCAL
  mapping, G4 assessment engine integration path, ContradictionKind integration, control
  citations (CM-6, CA-7, SI-7, AU-3, RA-5), and risks (false confidence, baseline drift,
  cmdline contradiction gap). Eight prioritized recommendations for rust-developer and
  tech-writer.

## Common Incorrect Citations (Do Not Repeat)

- The posture catalog uses abbreviated form `"NIST 800-53 SI-7"` in the `nist_controls`
  field (runtime display string — acceptable per Citation Format Rule for runtime output).
  In Rust doc comments (`///` or `//!`), the canonical form `NIST SP 800-53` is required.

## Modules With Known Annotation Coverage

- `umrs-platform/src/posture/` — Phase 1 + Phase 2a + Phase 2b: fully cited
- `umrs-platform/src/kattrs/` — fully cited
- `umrs-tui/src/` — TUI library; current annotation level unknown (not yet audited at depth)
