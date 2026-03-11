# Security Auditor — Persistent Memory

## Control Mapping Conventions

- TPI claims → cite NIST SP 800-53 SI-7 (two independent paths) + NSA RTB
- Fail-closed parser behavior → cite SI-10 (input validation) + NSA RTB
- Bounded reads / checked arithmetic → cite NIST SP 800-218 SSDF PW.4.1
- Error information discipline → cite NIST SP 800-53 SI-12
- Audit record integrity / append-only → cite NIST SP 800-53 AU-10
- Non-bypassable security checks → cite NSA RTB RAIN
- TOCTOU fd-anchored I/O → cite NSA RTB TOCTOU + SI-7
- Component inventory (RPM/dpkg queries) → cite CM-8 + SA-12
- FIPS mode gating → cite NIST SP 800-53 SC-13 + CMMC L2 SC.3.177

## Known Annotation Debt

- `umrs-platform/src/detect/` — all phase modules are well-annotated as of 2026-03-11
- `umrs-platform/src/evidence.rs` — AU-10 claim on `EvidenceBundle` is unsupported
  because `records` is `pub` (finding RPM-22); coder must make field private

## Recurring Gap Patterns

### Pattern: pub field defeats immutability claim
When a struct claims append-only or non-repudiation (AU-10) but exposes its
backing Vec as `pub`, the invariant is not enforced. Look for `pub records`,
`pub entries`, `pub log` on audit-trail types.

### Pattern: symlink path vs real path in DB queries
On RHEL 10, `/etc/os-release` is a symlink to `/usr/lib/os-release`. Any code
that records a symlink path from `statx(AT_EMPTY_PATH)` (which follows symlinks)
then passes that *symlink path* to an RPM DB query will fail: the RPM DB owns
the real path, not the symlink path. Always use `path_resolved` from the evidence
record for DB queries when available. (See finding RPM-16, 2026-03-11 audit.)

### Pattern: rusqlite Error Display leaks path info
`rusqlite::Error`'s Display can include SQL text and paths. Never forward it
verbatim in user-visible output; wrap and emit only a category + error code.

### Pattern: bundled SQLite on FIPS RHEL
`rusqlite` with `features = ["bundled"]` compiles a vendored SQLite that is not
covered by RHEL's patching pipeline. For FIPS deployments, using the system
SQLite (no "bundled" feature) is preferable and should be evaluated.

### Pattern: test unwrap() is acceptable
`unwrap()` in `tests/` is acceptable per project policy. Do not flag it.

## Common Incorrect Citations Seen

- "NSA RTB TOCTOU" applied to probe-phase path-existence checks — these are
  path-based and not TOCTOU-safe; the citation scope must be narrowed to
  fd-anchored operations only.

## Modules with Known Annotation Debt (per prior audits)

See `.claude/reports/2026-03-11-os-detection-umrs-platform-surface-audit.md`
and `.claude/reports/2026-03-11-rpm-db-security-audit.md` for full finding lists.
