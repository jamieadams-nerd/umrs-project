# Security Auditor — Persistent Memory

## Topic Files
- [SCAP familiarization](scap_familiarization.md) — CCE mappings, STIG signal inventory, Tier-1 candidates
- [RMF methodology README](rmf-methodology-README.md) — SP800-37, 53A, 30, 39 corpus summary
- [Accreditation artifacts README](accreditation-artifacts-README.md) — SSP/SAP/SAR structures
- [Audit knowledge archive](audit_knowledge_archive.md) — CPU matrix, SEC pattern, RMF lifecycle, resolved items
- [Header terminology review](header-terminology-review.md) — SP 800-53A normative terms for TUI
- [Indicator definitions](indicator-definitions-plain-language.md) — 37 indicators, plain English
- [TUI/CLI corpus](tui-cli-corpus.md) — ratatui, crossterm, clap audit checkpoints

## Control Mapping Conventions

- TPI claims → cite NIST SP 800-53 SI-7 + NSA RTB
- Fail-closed → cite SI-10 + NSA RTB
- Bounded reads / checked arithmetic → cite NIST SP 800-218 SSDF PW.4.1
- Error information discipline → cite SI-12
- Audit record integrity / append-only → cite AU-10
- Non-bypassable security checks → cite NSA RTB RAIN
- TOCTOU fd-anchored I/O → cite NSA RTB TOCTOU + SI-7
- Component inventory (RPM/dpkg) → cite CM-8 + SA-12
- FIPS mode gating → cite SC-13 + CMMC L2 SC.3.177

## Recurring Gap Patterns

### pub field defeats immutability claim
Look for `pub records`, `pub entries`, `pub log` on AU-10 types.

### symlink path vs real path in DB queries
RHEL 10 `/etc/os-release` → `/usr/lib/os-release`. Use `path_resolved` for DB queries.

### rusqlite Error Display leaks path info
Wrap and emit only category + error code. Never forward Display verbatim.

### bundled SQLite on FIPS RHEL
`features = ["bundled"]` bypasses RHEL patching. Prefer system SQLite.

### test unwrap() is acceptable
`unwrap()` in `tests/` is fine per project policy. Do not flag.

## Common Incorrect Citations
- "NSA RTB TOCTOU" on path-based checks — narrowing needed; TOCTOU applies only to fd-anchored ops.

## Open HIGH Findings (current)

1. **F-11**: `integrity_check.rs` — T4 via unvalidated `sha2` on FIPS systems. No FIPS gate. SC-13. Owner: coder.
2. **F-07**: `SecureReader::read()` missing `#[must_use]`. Owner: coder.
3. **SecureReader #[must_use] debt**: `new()` has bare #[must_use]; `read()` and `read_with_card()` missing entirely.

### SC-28 Claim Precision
`sealed_cache.rs` overstates SC-28: HMAC seal covers bytes never served (pipeline re-runs). Narrow to "tamper detection". Owner: tech-writer.

## Key Audit Frameworks

### Accreditation Anchors
- UMRS code review = SP 800-53A "Examine"; code is a system design document
- Missing annotation = "Other Than Satisfied" on SA-11
- High risks: remediated within 30 days; no authorization until cleared
- Citations must use canonical SP 800-53 Rev 5 form
- ODP values belong in the SSP, not code annotations (SDR-005 PENDING)

### RMF Anchors
- UMRS audit report = SAR component; finding + remediation = POA&M entry
- UMRS HIGH severity = SP 800-30 High/Very High impact

### Portfolio-Wide Gap
All plans: strong Implement, weak Assess/Monitor artifact production. Runtime outputs not connected to SAR-consumable Examine objects. **Check every new plan for output artifact format.**

## Reports Index
- `2026-03-11-rpm-db-security-audit.md` — RPM findings
- `2026-03-11-os-detection-umrs-platform-surface-audit.md` — detect pipeline
- `2026-03-14-security-auditor-umrs-platform-audit.md` — in-depth platform
- `cpu-matrix-review/security-auditor-review.md` — CPU feature audit
- `tui-plan-security-review.md` — TUI audit card review
- `rmf-plan-review-2026-03-15.md` — RMF portfolio review
- `security-engineer-phase2b-review.md` — posture Phase 2b

## TUI Audit Card Patterns
- IndicatorValue for kernel flags → cite SI-7 + CM-6 (NOT SI-3)
- SELinux: `enforce` value `0` = permissive (Inactive), not Active
- `indicator_unavailable` must be visually distinct from `indicator_inactive`
- Evidence display: structured codes (`OK(sha256)`), not narrative strings
- Tool version as HeaderField required for SP 800-53A Examine object
- Header terms: "Assessment" (not Report), "Scope" (not Subject), "Assessed" (not Checked)

## SCAP/STIG Key Checkpoints
- `cce` field on `IndicatorDescriptor` is IMPLEMENTED (2026-03-18); 13 entries populated
- NIST control precision gaps ALL RESOLVED: DmesgRestrict→SI-11, KptrRestrict→SC-30, etc.
- Remaining open items: FipsEnabled CCE-89085-5 (partial coverage), ModuleSigEnforce/SuidDumpable need explanatory comments
- Auditd: 51 rules → 3 composite indicators recommended (not yet implemented)
- UMRS exceeds STIG: 10 CPU mitigations + ModulesDisabled + Lockdown
- Work instruction report: `.claude/reports/2026-03-18-cce-crossref-audit.md`
- Full details: [scap_familiarization.md](scap_familiarization.md)

## TUI/CLI Checkpoints
- NO_COLOR: crossterm handles implicitly; verify no raw `\x1b` bypasses
- `--json` required on structured-data commands
- `ratatui::init()` / `ratatui::restore()` must be paired
- Ratatui v0.30.0: `frame.area()` not `frame.size()`; `block::Title` removed
- Full details: [tui-cli-corpus.md](tui-cli-corpus.md)
