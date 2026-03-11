# Security Review — umrs-platform OS Detection Subsystem

```
Audit date: 2026-03-11
Depth: in-depth
Scope: components/rusty-gadgets/umrs-platform/src/detect/ (all files),
       components/rusty-gadgets/umrs-platform/src/confidence.rs,
       components/rusty-gadgets/umrs-platform/src/evidence.rs,
       components/rusty-gadgets/umrs-platform/src/os_identity.rs,
       components/rusty-gadgets/umrs-platform/src/os_release.rs,
       components/rusty-gadgets/umrs-platform/src/lib.rs
Reviewer: security-engineer agent
```

---

## Executive Summary

The OS detection pipeline is structurally sound: the kernel anchor phase
correctly gates on `PROC_SUPER_MAGIC`, procfs reads uniformly use
`ProcfsText` + `SecureReader`, the TPI agreement check on `os-release` is
correctly implemented (exact key-set equality, not subset), and the confidence
model enforces a monotonically downgrade-only invariant. The substrate identity
counter uses `saturating_add` throughout.

Five findings require attention before the subsystem can be considered
production-ready. The most urgent is a FIPS compliance gap in
`integrity_check.rs`: the module claims T4 integrity using an unvalidated
SHA-256 implementation without a runtime FIPS gate, and the evidence flag
`opened_by_fd: true` is asserted on a path-based `File::open`. Two MEDIUM
findings concern the stub probes promoting to T3 without emitting operator
warnings, and a second path-based read of `os-release` in `release_parse.rs`
that opens a TOCTOU window after the `statx` recorded in the candidate phase.
Three LOW findings address annotation gaps and documentation clarity.

---

## Findings

---

### F-01 [HIGH] — integrity_check.rs uses non-FIPS-validated SHA-256 to assert T4

**File**: `components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs`, lines 55–59, 135–146, 162–166

**Description**:
The module computes SHA-256 using `sha2::Sha256` from the RustCrypto family.
On RHEL 10 with FIPS mode active (`/proc/sys/kernel/fips_enabled == 1`), the
`sha2` crate is **not** a FIPS 140-2/140-3 validated cryptographic module.
When this digest comparison succeeds, the code unconditionally upgrades
confidence to `TrustLevel::IntegrityAnchored` (T4) at line 271. If FIPS mode
is active, asserting T4 on the basis of a non-validated primitive overstates
the integrity assurance of the result and may produce a false compliance claim.

The module's header doc block (lines 11–23) documents this limitation but
relies on the caller to "verify this posture satisfies their policy". For a
deployed RHEL 10 system, that deferred verification never occurs — the caller
(`detect/mod.rs`, line 224–232) runs the phase unconditionally with no FIPS
gate. A downstream consumer that queries `label_trust == TrustedLabel` on a
FIPS-active system receives a trust assertion backed by a non-validated
primitive, with no programmatic indication of the FIPS mismatch.

**Severity**: HIGH

**Control references**:
- NIST SP 800-53 SC-13: Cryptographic Protection — use only FIPS-approved
  algorithms and implementations.
- NIST SP 800-53 SI-7: Software and Information Integrity — integrity claims
  must be backed by appropriate mechanisms.
- CMMC L2 SC.3.177: Employ FIPS-validated cryptography when used to protect
  the confidentiality of CUI.

**Remediation owner**: coder

**Recommended action**: Before the SHA-256 compute at line 162, read
`/proc/sys/kernel/fips_enabled` via `ProcfsText` + `SecureReader`. If FIPS is
active (value `"1"`), the function must NOT attempt digest comparison and must
NOT upgrade to T4. Instead it should:
  1. Record an `EvidenceRecord` with `parse_ok: false` and a note of
     `"FIPS mode active: non-validated SHA-256 implementation; T4 not earned"`.
  2. Call `confidence.downgrade(TrustLevel::SubstrateAnchored, "FIPS mode: integrity check skipped")`.
  3. Return `false`.
On a FIPS-active system the pipeline will then cap at T3. This is the correct,
auditable behavior. A log::warn! must fire so operators are aware of the cap.
The alternative — replacing `sha2` with a FIPS-validated provider via `kcapi`
or OpenSSL FIPS module — would allow T4 on FIPS systems but requires FFI and
is out of scope for pure-Rust policy.

---

### F-02 [HIGH] — integrity_check.rs sets opened_by_fd: true on a path-based File::open

**File**: `components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs`, lines 135–146, 174–176, 272–279, 298–304

**Description**:
At line 136, the candidate file is opened with `File::open(candidate)` — a
path-based open, not fd-anchored. At lines 174, 273, and 299, every
`EvidenceRecord` produced in this function sets `opened_by_fd: true`. This
assertion is false: `File::open` resolves the path through the VFS without
any re-verification of `(dev, ino)`, which means the file that was `statx`'d
in the `release_candidate` phase is not provably the same file that was
opened here.

The `opened_by_fd` field's definition in `evidence.rs` (lines 167–170) is:
"Whether the file was opened via an fd-anchored call ... `false` means a
path-based open was used — callers should note this in `notes`."
Setting it to `true` on a path-based open is a false provenance record. Audit
reviewers examining the `EvidenceBundle` will incorrectly believe the file was
opened with TOCTOU protection it did not receive.

The `(dev, ino)` anchoring design is documented in `file_ownership.rs` (lines
28–34): "When the probe's `query_ownership` implementation receives this pair,
it can cross-check the path's current on-disk `(dev, ino)` to detect
substitution. The stub implementations do not perform this check." The
integrity_check phase has the same gap — it holds the `(dev, ino)` from
`release_candidate` but does not re-verify before or after opening.

**Severity**: HIGH

**Control references**:
- NIST SP 800-53 SI-7: Software and Information Integrity — integrity
  verification must be anchored to a verified file identity.
- NSA RTB TOCTOU: fd-anchored access eliminates the check-then-use window.
- NIST SP 800-53 AU-3: Audit Record Content — provenance records must
  accurately reflect how data was obtained.

**Remediation owner**: coder

**Recommended action**: Two separate fixes are needed.

**Fix A — Correct the opened_by_fd flag**: Change all `opened_by_fd` fields
set to `true` in `integrity_check.rs` to `false`, and add a note to each
record: `"path-based open: File::open(candidate); (dev,ino) not re-verified"`.

**Fix B — Implement (dev, ino) re-verification**: After `File::open`, call
`rustix::fs::fstat` or `fstatx` on the open `File` fd to obtain the current
`(dev, ino)`. Compare against the values in the `EvidenceBundle` from the
`release_candidate` phase (retrievable via `find_stat_for_path`). If the
pair does not match, call `confidence.downgrade` and return `false`.

These two fixes are independent. Fix A is the minimum required to remove the
false provenance claim. Fix B adds the actual TOCTOU protection and should
follow in the next iteration.

---

### F-03 [MEDIUM] — release_parse.rs reads os-release a second time by path with no (dev, ino) re-verification

**File**: `components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs`, lines 229–258

**Description**:
`read_candidate()` calls `std::fs::read_to_string(candidate)` at line 235.
The `candidate` path was selected and `statx`'d in the `release_candidate`
phase, but `read_to_string` is a path-based open that does not verify
`(dev, ino)` against the previously recorded stat. Between the end of
`release_candidate` and this read, a symlink could be replaced, a bind mount
interposed, or the file swapped. The TPI key-set gate cannot compensate for
this: if the attacker controls both the old and new file, they can craft both
to produce the same key set while substituting the values that drive the
`OsId`, `CpeName`, or `VersionId` fields.

Evidence records produced here also mark `opened_by_fd: false` (lines 193,
242, 248), which is correctly set — but the TOCTOU window itself is not
documented in the code, creating a risk that a future maintainer treats it as
a complete implementation.

**Severity**: MEDIUM

**Control references**:
- NIST SP 800-53 SI-7: Software and Information Integrity.
- NSA RTB TOCTOU: fd-anchored access eliminates the check-then-use window.

**Remediation owner**: coder

**Recommended action**: Replace `std::fs::read_to_string(candidate)` with an
fd-anchored read:
  1. Open the file with `rustix::fs::open` or `std::fs::File::open`.
  2. Immediately call `fstatx` / `fstat` on the returned fd.
  3. Compare `(dev, ino)` against the `FileStat` recorded by `release_candidate`.
  4. If they do not match, downgrade and return `None`.
  5. Read content from the already-open fd (e.g., `read_to_string` on the
     `File` handle, or `rustix::io::read_to_end`).
This eliminates the path-reopen window. Update `opened_by_fd` in the evidence
records to `true` once this is implemented.

Until the fix is implemented, add a code comment at line 235 explicitly
documenting the gap: "TOCTOU gap: path-based open; (dev, ino) not re-verified
against release_candidate statx record."

---

### F-04 [MEDIUM] — Stub probes upgrade to T3 without operator warning when can_query_ownership and can_verify_digest are both false

**File**: `components/rusty-gadgets/umrs-platform/src/detect/substrate/rpm.rs`, lines 156–163;
`components/rusty-gadgets/umrs-platform/src/detect/substrate/dpkg.rs`, lines 135–142;
`components/rusty-gadgets/umrs-platform/src/detect/pkg_substrate.rs`, lines 99–118, 148–149

**Description**:
Both `RpmProbe::probe()` and `DpkgProbe::probe()` return `ProbeResult` with
`parse_ok: true`, `can_query_ownership: false`, and `can_verify_digest: false`.
When `pkg_substrate::run_inner` selects such a probe, it passes the T3
threshold check and may upgrade confidence to `TrustLevel::SubstrateAnchored`
(line 149 of `pkg_substrate.rs`). An operator examining the `DetectionResult`
would see T3 asserted but would have no programmatic indication that the probe
is a stub incapable of ownership queries or digest verification. The T3 level
implies that "identity derived from ≥2 independent facts" (per the
`TrustLevel` doc), but those "facts" here are only filesystem presence checks,
not any structural parse of DB records. No `log::warn!` fires anywhere in the
selection loop to inform operators.

The `ProbeResult` struct exposes `can_query_ownership` and `can_verify_digest`
precisely to allow callers to act on this information, but `pkg_substrate.rs`
does not inspect these flags before asserting T3.

**Severity**: MEDIUM

**Control references**:
- NIST SP 800-53 CM-8: Information System Component Inventory — T3 claims
  must reflect actual verification depth.
- NIST SP 800-53 SA-12: Supply Chain Risk Management — stub evidence is not
  equivalent to DB-parsed evidence.
- NSA RTB RAIN: non-bypassable trust assertions must not overstate assurance.

**Remediation owner**: coder

**Recommended action**: In `pkg_substrate::run_inner`, after the selected probe
is chosen and before the T3 upgrade, check the flags:

```
if !selected_probe_result.can_query_ownership || !selected_probe_result.can_verify_digest {
    log::warn!(
        "pkg_substrate: probe '{}' is a stub — \
         can_query_ownership={}, can_verify_digest={}; \
         T3 asserted on presence-only evidence",
        selected_probe_result.probe_name,
        selected_probe_result.can_query_ownership,
        selected_probe_result.can_verify_digest,
    );
    // Record in evidence
    evidence.push(EvidenceRecord {
        ...
        notes: vec!["stub probe: ownership/digest capability absent".to_owned()],
        ...
    });
}
```

The log::warn! must fire regardless of whether T3 is subsequently granted.
This ensures operators understand the reduced assurance level when reviewing
system logs. Consider whether a stub-only T3 should carry a distinct
sub-tier or note field — that design decision belongs to the coder, but the
at-minimum warning is required now.

---

### F-05 [MEDIUM] — rpm.rs and dpkg.rs use Path::exists() — TOCTOU check-then-use on package DB presence

**File**: `components/rusty-gadgets/umrs-platform/src/detect/substrate/rpm.rs`, lines 76, 116–117, 121, 129;
`components/rusty-gadgets/umrs-platform/src/detect/substrate/dpkg.rs`, lines 69, 109

**Description**:
Both stub probes use `Path::new(...).exists()` to check for the presence of
the RPM database root, the Packages file, and the dpkg status file. `exists()`
resolves the path and calls `stat(2)` — the path is not re-verified when the
probe returns. In `rpm.rs`, the Packages file path is checked a second time at
line 121 for the `which` string selection. Between the first check at line 116
and the re-check at line 121, the file could be replaced (e.g., in a container
or orchestrated environment).

More broadly, none of these existence checks verify filesystem magic (`statfs`)
to confirm the path is on the expected filesystem type. A tmpfs mounted over
`/var/lib/rpm/` would pass the presence check and return `facts_count = 2`,
allowing T3 to be asserted on a fabricated DB location.

The `EvidenceRecord`s produced by these probes correctly set `opened_by_fd:
false` (rpm.rs line 145, dpkg.rs line 123), which is accurate, but the
limitation is not called out in code comments.

**Severity**: MEDIUM

**Control references**:
- NIST SP 800-53 SI-7: Software and Information Integrity — provenance
  verification before trusting any artifact.
- NSA RTB TOCTOU: check-then-use windows on security-relevant paths.
- NIST SP 800-53 CM-8: component inventory accuracy.

**Remediation owner**: coder

**Recommended action**: For both stubs, add a `statfs` on the DB root
directory immediately after the initial existence check. Verify that the
filesystem magic is an expected persistent filesystem type (e.g., ext4
`0xEF53`, xfs `0x58465342`, btrfs `0x9123683E`, tmpfs `0x01021994` should
be a warning or reject depending on policy). Record `fs_magic` in the
`EvidenceRecord`. Additionally, collapse the duplicate `Path::exists()` call
in `rpm.rs` (lines 116 and 121) into a single check with the result stored
in a boolean.

This does not eliminate the TOCTOU window entirely (that requires opening the
file and using fd-based operations, deferred to full stub implementation), but
it reduces the attack surface significantly and removes the fabricated-tmpfs
vector. Add a comment in both files: "Path-based existence check only —
see stub limitation note in module doc."

---

### F-06 [LOW] — release_candidate.rs: statx uses AT_EMPTY_PATH but statx is on the symlink; subsequent file reading uses the symlink path, not the resolved target

**File**: `components/rusty-gadgets/umrs-platform/src/detect/release_candidate.rs`, lines 136–142, 169–172, 194–219

**Description**:
`probe_candidate()` calls `statx(CWD, path_str, AtFlags::empty(), StatxFlags::ALL)` at line 136.
With `AtFlags::empty()` (i.e., no `AT_SYMLINK_NOFOLLOW`), `statx` follows the
symlink and returns metadata for the **resolved target**, not for the symlink
itself. On RHEL 10, `/etc/os-release` is a symlink to `/usr/lib/os-release`.

The `(dev, ino)` pair recorded in the `FileStat` (lines 196–208) is therefore
from the resolved target (`/usr/lib/os-release`), not from `/etc/os-release`.
However, the `path_requested` field in the `EvidenceRecord` (line 202) is
set to `path_str` (`"/etc/os-release"`), and the function returns `path.to_path_buf()`
(line 219) — the symlink path, not the resolved path. Subsequent phases
(`integrity_check`, `release_parse`) receive the symlink path and open it by
name, which will again follow the symlink. This is internally consistent, but
the `FileStat.ino` in the evidence record belongs to the target while the
`path_requested` field names the symlink — creating an ambiguous audit record.

The `resolve_symlink()` helper at line 256 pushes a separate `EvidenceRecord`
with `source_kind: SourceKind::SymlinkTarget` that names the target, which
partially mitigates the confusion, but `file_ownership.rs` searches the
evidence bundle by `path_requested` (line 164) to find the `FileStat`. If
`find_stat_for_path` is called with the symlink path, it finds the record
whose `(dev, ino)` belongs to the resolved target — which is correct, but
not documented.

**Severity**: LOW

**Control references**:
- NIST SP 800-53 AU-3: Audit Record Content — records must unambiguously
  identify the artifact accessed.
- NIST SP 800-53 SI-7: Software and Information Integrity — file identity
  anchoring must be unambiguous.

**Remediation owner**: coder

**Recommended action**:
  1. Add a note to the primary `EvidenceRecord` pushed at line 191 when the
     path is a symlink: `"statx followed symlink; (dev,ino) is of resolved target, not symlink"`.
  2. Consider populating `path_resolved` in the primary record (currently `None`
     at line 196) with the resolved path when a symlink was detected, so that
     the `FileStat`'s `(dev, ino)` is unambiguously linked to the target path.
  3. Add a code comment at line 136 explaining why `AtFlags::empty()` (symlink
     follow) is intentional and what `(dev, ino)` the result represents.

---

### F-07 [LOW] — mount_topology.rs: /etc statfs EvidenceRecord uses SourceKind::RegularFile

**File**: `components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs`, lines 256–267

**Description**:
`read_etc_statfs()` records the filesystem magic from `statfs("/etc")` in an
`EvidenceRecord` with `source_kind: SourceKind::RegularFile`. The `/etc`
target is a directory, not a regular file, and the data source is a `statfs(2)`
syscall returning filesystem-level metadata — not a file read. No variant in
`SourceKind` precisely matches a `statfs` call on a directory. Using
`RegularFile` causes audit reviewers to misclassify the record as a file read.

Additionally, `opened_by_fd: false` at line 259 is correct (no fd was opened),
but a note explaining this was a `statfs(2)` on a directory path would help
auditors distinguish this from a failed or missing file open.

**Severity**: LOW

**Control references**:
- NIST SP 800-53 AU-3: Audit Record Content — records must correctly identify
  the type and method of data acquisition.

**Remediation owner**: coder

**Recommended action**: Either:
  (a) Add a `StatfsResult` variant to `SourceKind` in `evidence.rs` to
      represent `statfs(2)` calls and use it here, or
  (b) Leave `SourceKind::RegularFile` but add to `notes` a string of:
      `"statfs(2) on directory /etc — not a file read"` so audit reviewers
      can interpret the record accurately.
Option (a) is cleaner and more maintainable as the detection pipeline grows.

---

### F-08 [LOW] — Compliance annotations missing on public items in detect/ and os_release.rs

**File**: Multiple — see locations below.

**Description**:
The project's CLAUDE.md requires that all public items carry NIST 800-53,
CMMC, or NSA RTB citations. The following public items are missing citations
entirely or carry only a module-level citation with no item-level annotation
on security-critical functions:

1. `detect/mod.rs` line 188 (`OsDetector::detect`) — the pipeline entry
   point has no doc comment beyond `"NIST SP 800-53 SA-8, CM-6, SI-7"` on
   the struct. The `detect()` method itself carries no annotation.

2. `os_release.rs` line 118 (`OsId::parse`) — the parse method for the
   security-critical OS identifier field carries no control citation in its
   doc comment, only a prose description.

3. `os_release.rs` line 406 (`OsRelease::ansi_color` field) — documented as
   "informational only; not validated", but carries no explicit note that this
   field MUST NOT be used for policy decisions. An auditor reading the field
   in isolation might not recognize its untrusted status.

4. `evidence.rs` line 231 (`EvidenceBundle::push`) — appends the audit trail
   record; no citation to AU-10 (Non-Repudiation) in the method doc comment,
   only at the struct level.

5. `detect/label_trust.rs` line 35 (`LabelTrust` enum and each variant) —
   the enum-level doc references NSA RTB and NIST CM-8/SI-7 correctly. The
   individual variant doc comments do not explicitly state which control
   governs when each variant must be used. `TrustedLabel` (line 55) and
   `IntegrityVerifiedButContradictory` (line 63) are the variants that drive
   policy enforcement decisions and should each carry at minimum a reference
   to NIST SI-7 and the relevant CMMC control.

**Severity**: LOW

**Control references**:
- NIST SP 800-218 SSDF PW.1.2: Review and/or analyze human-readable code;
  annotation discipline supports this review.

**Remediation owner**: coder

**Recommended action**: Add NIST/RTB citations to each item listed above.
Suggested minimum additions:
- `OsDetector::detect`: `"NIST SP 800-53 SA-8 (Security Engineering Principles) — orchestrates layered, fail-closed platform verification."`
- `OsId::parse`: `"NIST SP 800-53 SI-10 — validates input to the security-relevant OS identifier field at construction."`
- `OsRelease::ansi_color`: Add `"MUST NOT be used for policy or identity decisions; informational display only."` to the field doc.
- `EvidenceBundle::push`: `"NIST SP 800-53 AU-10 — records are append-only; callers cannot remove or reorder them after push."`
- `LabelTrust::TrustedLabel` and `LabelTrust::IntegrityVerifiedButContradictory`: Add per-variant citations to NIST SI-7 and the consequence for policy decisions.

---

## Compliance Gap Summary

| Control | Status | Notes |
|---|---|---|
| NIST SP 800-53 SC-13 (FIPS Cryptography) | GAP — HIGH | `sha2::Sha256` is not FIPS-validated; no runtime FIPS gate before T4 assertion. F-01. |
| NIST SP 800-53 SI-7 (SW/Info Integrity) | PARTIAL | TPI implemented correctly; TOCTOU gap in integrity_check and release_parse. F-02, F-03. |
| NIST SP 800-53 AU-3 (Audit Record Content) | PARTIAL | `opened_by_fd: true` set incorrectly in integrity_check.rs; SourceKind::RegularFile misused for statfs. F-02, F-07. |
| NIST SP 800-53 AU-10 (Non-Repudiation) | MINOR GAP | EvidenceBundle::push missing item-level AU-10 citation. F-08. |
| NIST SP 800-53 CM-8 (Component Inventory) | PARTIAL | T3 asserted on stub evidence without operator warning. F-04. |
| NIST SP 800-53 SA-12 (Supply Chain Risk) | PARTIAL | Stub probes assert T3 without DB-level parse; no warning emitted. F-04. |
| NSA RTB TOCTOU | PARTIAL | Kernel anchor and procfs reads are correct; integrity_check and release_parse have path-reopen gaps. F-02, F-03, F-05. |
| NSA RTB RAIN (Non-bypassability) | PARTIAL | ProcfsText/SysfsText used consistently for kernel pseudo-fs reads; stub T3 assertion overstates assurance. F-04. |
| NIST SP 800-218 SSDF PW.1.2 (Annotations) | MINOR GAP | Several public items missing control citations. F-08. |
| CMMC L2 SC.3.177 (FIPS Cryptography) | GAP — HIGH | Same as SC-13 gap above. F-01. |
| CMMC L2 SI.1.210 (Integrity Checking) | PARTIAL | Integrity phase is structurally correct but not FIPS-compliant. F-01. |

---

## Patterns Applied Correctly

The following areas represent strong, correct implementation of high-assurance
patterns:

**ProcfsText / SysfsText usage** — Every read from `/proc/` and `/sys/` in the
entire subsystem routes through `ProcfsText`/`SysfsText` + `SecureReader`. No
raw `File::open` on kernel pseudo-filesystem paths was found. The
`KernelLockdown` reader uses the existing `StaticSource` pattern. This is
exactly the mandatory pattern from the project MEMORY.md.

**Bounded reads** — Every procfs read has a content-size cap enforced after
read (`MAX_PROC_READ` in kernel_anchor.rs, `max_mountinfo_bytes` from the
orchestrator). `read_bounded()` in integrity_check.rs uses `.take(n+1)` to
detect oversize files before allocating.

**TPI implementation** — `release_parse.rs` correctly implements Two-Path
Independence: `parse_with_nom` and `parse_with_split` share zero code, and
`key_sets_agree` checks exact set equality (both `a.len() != b.len()` and
`a.keys().all(|k| b.contains_key(k))`). This is correct and not merely subset
checking. The fail-closed path downgrades confidence and pushes an evidence
record before returning.

**Saturating arithmetic** — `SubstrateIdentity::add_fact()` uses
`saturating_add(1)` with an explicit comment citing the ANSSI Rust Guide. No
unchecked integer operations were found on security-relevant counter values.

**Confidence model invariant** — `ConfidenceModel` enforces downgrade-only
semantics via a private `level` field. `upgrade()` is a no-op when called
with a value below the current level. Callers cannot arbitrarily raise trust.

**SELinux enforce pre-check** — `pkg_substrate.rs` correctly reads SELinux
enforce mode via `SecureReader::<SelinuxEnforce>` before asserting T3, with
the rationale (Biba integrity pre-check) clearly documented and recorded in
the evidence bundle.

**Error information discipline** — Reviewed all `log::warn!` and `log::error!`
calls. None include security labels, credentials, key material, or raw file
content. Downgrade reason strings in `ConfidenceModel` are policy-level
descriptions only.

**PID coherence check** — `kernel_anchor.rs` correctly uses `saturating_add(0)`
pattern to make the non-negative contract visible before the `u32` cast, and
fails closed with a structured `DetectionError` rather than panicking on
mismatch.

**Fail-closed TPI contradiction recording** — `confidence.record_contradiction`
in `release_parse.rs` is called with a downgrade before returning, and the
`EvidenceRecord` is pushed. The pipeline does not silently continue with a
degraded result.

---

## Files Reviewed

| File | Lines |
|---|---|
| `detect/kernel_anchor.rs` | 314 |
| `detect/mount_topology.rs` | 287 |
| `detect/release_candidate.rs` | 278 |
| `detect/pkg_substrate.rs` | 224 |
| `detect/file_ownership.rs` | 172 |
| `detect/integrity_check.rs` | 358 |
| `detect/release_parse.rs` | 532 |
| `detect/label_trust.rs` | 70 |
| `detect/substrate/mod.rs` | 158 |
| `detect/substrate/rpm.rs` | 175 |
| `detect/substrate/dpkg.rs` | 154 |
| `detect/mod.rs` | 259 |
| `confidence.rs` | 184 |
| `evidence.rs` | 246 |
| `os_identity.rs` | 214 |
| `os_release.rs` | 407 |
| `lib.rs` | 69 |

**Files reviewed**: 17
**Total findings**: 8 (2 HIGH, 3 MEDIUM, 3 LOW)

**Policy artifacts written**: none (OS detection subsystem; no SELinux policy artifacts required by this review)

**Policy artifacts needed**: none for this scope

**Documentation gaps**:
- No deployment documentation reviewed in this audit; the deployment guide should be updated to state that on FIPS-active RHEL 10 systems, the integrity check phase will cap at T3, not T4, until a FIPS-validated SHA-256 provider is integrated.
- The operator guide should note that when both `can_query_ownership` and `can_verify_digest` are false, the logged T3 level reflects presence-only evidence.

**Code-vs-policy inconsistencies**:
- `integrity_check.rs` module doc states FIPS limitation and that "callers should verify" — but the orchestrator (`detect/mod.rs`) provides no mechanism for callers to detect or act on this condition. The limitation is documented but not enforced programmatically.
- `file_ownership.rs` doc comment states that probes "must verify both the path and the `(dev, ino)` pair" (line 17), but the current stub implementations explicitly skip this (lines 166–169 in rpm.rs, lines 145–148 in dpkg.rs). The stub is correctly labeled, but `integrity_check.rs` is not a stub and also fails to re-verify.
