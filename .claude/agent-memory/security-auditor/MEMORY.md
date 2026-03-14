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
- `umrs-platform/src/evidence.rs` — `records` field is now private (corrected as of
  2026-03-11 SEC audit); AU-10 invariant is enforced at the type-system level.

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

## SEC Pattern — Key Observations (2026-03-11)

- Pattern spec (pattern-sec.adoc) requires process start time from /proc/self/stat as
  second key entropy source; implementation uses wall-clock subsecond nanos instead.
  Code wins — either fix code or update spec.
- FIPS gate fails open on procfs read failure. For FIPS deployments the correct
  behavior is fail-closed (return true / disable caching).
- decode_cached_result re-runs the pipeline on every cache hit — the sealed cache
  provides tamper detection only, not I/O avoidance, until full serialization is
  implemented. Pattern spec flowchart contradicts this.
- On FIPS gate: cite SC-13 in Cargo.toml hmac dep comment, not just in source.

## Common Incorrect Citations Seen

- "NSA RTB TOCTOU" applied to probe-phase path-existence checks — these are
  path-based and not TOCTOU-safe; the citation scope must be narrowed to
  fd-anchored operations only.

## Modules with Known Annotation Debt (per prior audits)

See `.claude/reports/2026-03-11-os-detection-umrs-platform-surface-audit.md`
and `.claude/reports/2026-03-11-rpm-db-security-audit.md` for full finding lists.

## umrs-platform In-Depth Audit Results (2026-03-14)

Report: `.claude/reports/2026-03-14-security-auditor-umrs-platform-audit.md`

Key findings for future sessions:

### Confirmed Resolved Since 2026-03-11
- `evidence.rs` `records` field is now `pub(crate)` private — AU-10 enforced ✓
- SEC FIPS gate now correctly fails closed on procfs read error ✓
- `decode_cached_result` re-runs pipeline on cache hit — still present; design note
  explicitly acknowledges this; ongoing debt, not a regression.

### New High-Priority Open Gaps (2026-03-14)
1. **F-11 (HIGH)**: `integrity_check.rs` — T4 (TrustedLabel) asserted via
   unvalidated `sha2` crate on FIPS-active systems. No FIPS gate. Violates SC-13.
   Owner: coder.
2. **F-07 (HIGH)**: `SecureReader::read()` missing `#[must_use]`. Primary kernel
   read path. Owner: coder.

### SecureReader #[must_use] Debt Pattern
`kattrs/traits.rs`: SecureReader::new() has bare #[must_use] (no message);
SecureReader::read() and read_with_card() have no #[must_use] at all.
Both selinux read_generic() methods are also missing it.
This is a recurring pattern: the mandatory read engine is under-annotated
relative to the types that call it.

### SC-28 Claim Precision
`sealed_cache.rs` module doc overstates SC-28 protection: the HMAC seal covers
bytes that are never served to callers (pipeline always re-runs on hit). SC-28
claim must be narrowed to "tamper detection" not "protection while in cache."
Owner: tech-writer.
