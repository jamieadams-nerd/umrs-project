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

## RMF Methodology Knowledge Artifacts (2026-03-15)

Corpus familiarization pass completed. Five artifacts written to
`.claude/agent-memory/security-auditor/`:
- `concept-index.md` — SP800-37, SP800-53A, SP800-30, SP800-39
- `cross-reference-map.md` — agreements, tensions, chains, gaps
- `style-decision-record.md` — SDR-001 through SDR-005 (SDR-005 PENDING)
- `term-glossary.md` — ~25 canonical RMF terms
- `rmf-methodology-README.md` — collection summary

### Key RMF Anchors for Audit Work
- Missing annotation on public item → "other than satisfied" on SA-11 (code review)
  or PL-2 (system documentation); Examine object: system design documentation
- UMRS audit report = SAR component; finding + remediation = POA&M entry
- UMRS HIGH severity = SP 800-30 High or Very High impact on operations/assets
- AO risk determination (R-2) requires documented risk assessment (SP 800-30) as input
- ODP values belong in the SSP, not code annotations — SDR-005 PENDING

## CPU Feature Matrix — Key Audit Principles (2026-03-14)

Report: `.claude/reports/cpu-matrix-review/security-auditor-review.md`

### Two-Layer Model: Hardware Capability vs. Software Utilization
A CPU feature being present is necessary but not sufficient for a security posture claim.
The audit must verify BOTH layers:
1. CPU reports the feature (CPUID, /proc/cpuinfo)
2. Software actually uses it (/proc/crypto, openssl probing, ELF binary headers)

Key example: AES-NI present in CPU + OpenSSL compiled without AES-NI support = HIGH finding
on a FIPS system. The software-AES fallback may have timing side-channels the hardware
eliminates, and may not be FIPS-validated for the specific algorithm combination.

### /proc/crypto as Primary Detection Interface
`/proc/crypto` shows which algorithm implementations are registered by the kernel and whether
they are hardware-backed (e.g., `aes-aesni` vs `aes-generic`). The `fips_allowed` field and
`selftest: passed` are FIPS-relevant. This interface is more authoritative than /proc/cpuinfo
for determining whether hardware acceleration is actually in use.

### NIST SP 800-90B Required for RDRAND/RDSEED Classification
Cannot classify CPU entropy features as Critical/Important without consulting SP 800-90B.
The posture catalog already cites this document (SignalId::RandomTrustCpu). RDRAND is
Critical on FIPS systems; classification must be consistent across hardware and OS layers.

### Missing Entire Category: Defensive CPU Controls
The spec is missing speculative-execution mitigations (IBRS, IBPB, STIBP, SSBD, MDS/MD_CLEAR)
and CPU-enforced access controls (SMEP, SMAP, CET, UMIP, NX/XD). These are Critical/Defensive
and directly connected to existing posture signals (SignalId::Mitigations, Pti).

Primary detection for mitigations: `/sys/devices/system/cpu/vulnerabilities/` — not /proc/cpuinfo.

### CET Binary Verification
CET requires CPU + kernel + per-binary ELF opt-in (-fcf-protection=full). Verify via
`.note.gnu.property` section in ELF headers (`eu-readelf -n <binary>` or `objdump -p`).
UMRS binaries themselves must carry CET headers if auditing for CET compliance.

## TUI Audit Card Plan Review (2026-03-15)

Report: `.claude/reports/tui-plan-security-review.md`

### Established Patterns for TUI Audit Card Work

- `IndicatorValue` for kernel posture flags → cite SI-7 + CM-6 (NOT SI-3, which is malware)
- `SecurityWarning` dialog acknowledgements → must document AU-10 requirement in API doc comment;
  caller is responsible for emitting audit log entry; library cannot enforce this
- Evidence display verification column → structured codes (e.g., `OK(sha256)`) not narrative
  strings; assessors need checkable claims not conclusions
- SELinux indicator must distinguish `permissive` (Inactive) from `enforcing` (Active); reading
  `/sys/fs/selinux/enforce` value `0` must NOT render as `Active("0")` — it means permissive
- `indicator_unavailable` must be visually distinct from `indicator_inactive`; both cannot share
  `DarkGray` because unavailable is a potential security concern requiring investigation
- `TwoColumnTable` in Phase 6 plan is actually three columns — naming error caught; pattern:
  always check column count vs. variant name when reviewing evidence table designs
- Evidence records currently have no timestamps — this is a known `EvidenceRecord` gap in
  umrs-platform; TUI display cannot fix it but headers should show detection run time as minimum
- Tool version as a `HeaderField` is required for audit card to serve as SP 800-53A Examine object

### Portfolio Gap Pattern — TUI-Specific
TUI plans produce runtime display only — no persisted assessment artifacts. Same gap seen
across all UMRS plans. Evidence tab display is not an Examine object until it can be exported
as structured data (future G4 work).

## RMF Plan Review — Portfolio-Wide Gap Pattern (2026-03-15)

Report: `.claude/reports/rmf-plan-review-2026-03-15.md`

### Systemic Cross-Plan Gap: Strong Implement, Weak Assess/Monitor Artifact Production
All three active plans (kernel posture probe, CPU corpus, umbrella) correctly implement
mechanisms but do not specify how mechanism outputs become persisted assessment artifacts.
This is a consistent portfolio gap: runtime `SignalReport` / `DetectionResult` / iterator
outputs are not connected to SAR-consumable Examine objects (CM-6 deviation records,
CA-7 monitoring strategy documents, CM-8 persistent inventory).

When reviewing new plans, always check: does the plan specify an output artifact format
that an assessor can use as an Examine object, or does it produce only runtime data?

### "Other Than Satisfied" Controls Across Current Plans (2026-03-15)
- CA-7: monitoring frequency ODP undefined (posture probe)
- CM-6(iii): no persistent deviation document produced (posture probe)
- SC-13: Layer 2/3 utilization assessment procedure absent (CPU corpus)
- SI-7: Ubuntu dpkg path has no Test coverage (umbrella)
- SC-28: DetectionResult serialization deferred; interim control undocumented (umbrella)
- CM-8: inventory persistence mechanism undefined for OS Detection (umbrella)

### RMF Lifecycle Mapping for UMRS Plan Components
- Signal catalog / baseline definition → Select S-2, S-3
- Runtime collection mechanism → Monitor M-2 (ongoing assessments)
- Contradiction detection output → Monitor M-3 (ongoing remediation)
- Control implementation (FIPS gate, CET, etc.) → Implement I-2
- Open architectural decisions (CpuSignalId, MAC abstraction, serialization) → Authorize R-1
- Research corpus (CPU features) → Select S-2 pre-requisite (knowledge base for control tailoring)
