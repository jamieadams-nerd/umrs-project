# Security Audit Report — OS Detection Subsystem (`umrs-platform`)

```
Audit date:  2026-03-11
Depth:       surface
Auditor:     security-auditor agent
Scope:       umrs-platform/src/{confidence,evidence,os_identity,os_release}.rs
             umrs-platform/src/detect/{mod,label_trust,kernel_anchor,mount_topology,
               release_candidate,pkg_substrate,file_ownership,integrity_check,
               release_parse}.rs
             umrs-platform/src/detect/substrate/{mod,rpm,dpkg}.rs
             umrs-platform/src/lib.rs
             umrs-platform/Cargo.toml
Files reviewed: 18
Total findings: 22 (3 HIGH, 9 MEDIUM, 10 LOW)
```

---

## `detect/integrity_check.rs` — 4 findings

**Finding 1 — HIGH**
`run_inner` (line 136): File opened with `std::fs::File::open(candidate)` — path-based, no fd anchor, no RESOLVE_NO_SYMLINKS. TOCTOU window between `release_candidate` statx and this open. Additionally, evidence records set `opened_by_fd: true` for a path-based open — doc/code inconsistency.
Violates: NSA RTB TOCTOU; NIST SP 800-53 SI-7.
Fix: Open with `rustix::fs::openat2` + `RESOLVE_NO_SYMLINKS`; re-verify `(dev, ino)` via fstat on open fd before hashing; set `opened_by_fd` accurately.

**Finding 2 — HIGH**
Module doc + Cargo.toml line 24: `sha2 0.10` (RustCrypto) is not FIPS 140-2/3 validated. On RHEL 10 with FIPS mode active, producing a T4/TrustedLabel result using a non-validated primitive violates FIPS 140-2 Section 4.1 and NIST SP 800-53 SC-13.
Violates: FIPS 140-2 §4.1; NIST SP 800-53 SC-13, SI-7.
Fix: Add a FIPS-mode guard. On FIPS-active systems, refuse T4 and document the gap explicitly rather than silently asserting T4.

**Finding 3 — MEDIUM**
`check_algorithm_policy` Unknown arm (line 238): log message interpolates raw `alg` string from package DB without length cap. SI-12 truncation must be applied.
Violates: NIST SP 800-53 SI-12.
Fix: Truncate `alg` to ≤32 characters before interpolation.

**Finding 4 — MEDIUM**
`compare_and_record` (line 260): T4 upgrade gate — security-critical function with no doc comment and no compliance citation.
Violates: CLAUDE.md annotation requirement; NIST SP 800-53 SI-7; NSA RTB RAIN.
Fix: Add doc comment with control citations (SI-7, SC-28, NSA RTB RAIN).

**Finding 5 — LOW**
`read_bounded` (line 346): cites only "NSA RTB" — not formally available. Add NIST 800-218 SSDF PW.4.1 and NIST SP 800-53 SI-10.

---

## `detect/release_parse.rs` — 4 findings

**Finding 6 — HIGH**
`read_candidate` (line 235): `std::fs::read_to_string(candidate)` is unbounded — `max_read_bytes` is not passed through. Also a TOCTOU second-open without fd anchor or `(dev, ino)` re-verification.
Violates: NSA RTB TOCTOU; NIST SP 800-53 SI-7, SI-10.
Fix: Pass `max_read_bytes`, use bounded read, open with `openat2`, re-verify `(dev, ino)`.

**Finding 7 — MEDIUM**
`assign_label_trust` (lines 486–488): contradiction description embeds raw `OsId` value. Sets a risky precedent under SI-12.
Violates: NIST SP 800-53 SI-12.
Fix: Omit the value — `"os-release ID field does not match substrate-derived distro"`. Value is already in the evidence bundle.

**Finding 8 — MEDIUM**
`key_sets_agree` (line 356): TPI agreement checks only key-set identity, not values. Crafted input with identical keys but different `ID=`/`NAME=` values would pass the gate while feeding divergent data.
Violates: NIST SP 800-53 SI-7; NSA RTB TPI.
Fix: Add value comparison for at minimum `ID=` and `NAME=` between the two parse maps; fail closed on disagreement.

**Finding 9 — LOW**
`build_os_release` (line 388): optional field parse failures are silently discarded with `.ok()`. No log entry when a field fails validation.
Violates: NIST SP 800-53 AU-3, SI-10.
Fix: Replace `.ok()` with explicit match; emit `log::debug!` (without the value) when parse returns `Err`.

---

## `detect/release_candidate.rs` — 2 findings

**Finding 10 — MEDIUM**
`probe_candidate` (line 136): `statx` called with `AtFlags::empty()` — follows symlinks. S_IWOTH check applies to target, not the symlink itself. A world-writable symlink pointing to a non-writable target passes.
Violates: NIST SP 800-53 SI-7; NSA RTB TOCTOU.
Fix: Second `statx` with `AT_SYMLINK_NOFOLLOW` on paths where `readlinkat` succeeds; apply S_IWOTH check to link inode.

**Finding 11 — LOW**
Lines 179–180: setuid bit detected but only recorded in evidence note — no `log::warn!` emitted.
Violates: CLAUDE.md Loud Failure; NIST SP 800-53 CM-6.
Fix: Emit `log::warn!` on setuid detection; evaluate confidence downgrade.

---

## `detect/mount_topology.rs` — 2 findings

**Finding 12 — MEDIUM**
`read_mnt_namespace` (line 103): `readlinkat` for `/proc/self/ns/mnt` is called without routing through `ProcfsText` + `SecureReader`. Module doc claims "All procfs reads use ProcfsText + SecureReader" — this is inaccurate.
Violates: NSA RTB RAIN; NIST SP 800-53 SI-7. Doc-vs-code inconsistency.
Fix: Either correct the doc to accurately state this read does not go through ProcfsText (and explain why), or verify PROC_SUPER_MAGIC before readlinkat.

**Finding 13 — LOW**
`read_etc_statfs` (line 259): evidence record uses `SourceKind::RegularFile` for a `statfs(2)` call on a directory.
Violates: NIST SP 800-53 AU-3.
Fix: Add `SourceKind::DirectoryStatfs` variant or document deviation in notes.

---

## `detect/pkg_substrate.rs` — 1 finding

**Finding 14 — MEDIUM**
`check_selinux_enforce` (line 200): evidence note uses `{:?}` Debug formatting for `EnforceState`. Use controlled output (Display or explicit match) not Debug.
Violates: NIST SP 800-53 SI-12.
Fix: Match `EnforceState` to a known string ("Enforcing"/"Permissive"/"Unknown") explicitly.

---

## `detect/substrate/rpm.rs` and `detect/substrate/dpkg.rs` — 2 findings

**Finding 15 — HIGH**
Both stubs assert `parse_ok=true` and T3-eligible based solely on `Path::exists()`. T3 is being asserted from "the RPM DB directory exists" — not structural DB validation. T3 evidence quality does not match T3 semantics.
Violates: NIST SP 800-53 SI-7, CM-8; NSA RTB TOCTOU; trust model integrity.
Fix: Emit `log::warn!` at the T3 upgrade site when `can_verify_digest=false` and `can_query_ownership=false`. Full implementations must open, fstat, and structurally validate before asserting T3.

**Finding 16 — MEDIUM**
`rpm.rs` lines 116, 121, 129: `Path::exists()` called twice on same path without fd anchor between — micro-TOCTOU.
Violates: NSA RTB TOCTOU.
Fix: Cache first `exists()` result in a `let` binding; reuse.

---

## `os_release.rs` — 2 findings

**Finding 17 — MEDIUM**
`OsReleaseParseError` `String` payloads: truncation is by convention only. `{e:?}` Debug formatting at any call site would expose full payload.
Violates: NIST SP 800-53 SI-12.
Fix: Cap `String` payload to 64 chars inside each `parse()` function at error construction, or add test asserting all error payloads ≤64 chars.

**Finding 18 — LOW**
`OsRelease.ansi_color: Option<String>` bypasses the validated-newtype pattern. Module doc claims "raw strings never cross module boundary" — this field is a direct counterexample. Terminal escape injection risk.
Violates: NIST SP 800-53 SI-10; module doc inconsistency.
Fix: Create `AnsiColor` validated newtype (ASCII digits, semicolons, ≤32 chars for SGR syntax).

---

## `lib.rs` — 1 finding

**Finding 19 — LOW**
Crate root compliance section cites only `NIST 800-53 SI-7` and `NSA RTB RAIN`. Missing: AU-3, CM-6, CM-8, SA-12, SC-13, NIST 800-218 SSDF PW.4.
Violates: CLAUDE.md module annotation requirement.
Fix: Expand crate root compliance section.

---

## `Cargo.toml` — 1 finding

**Finding 20 — LOW**
`log = { version = "0.4", features = ["release_max_level_info"] }` silences debug-level logs in release builds. Provenance detail from detection phases is lost if EvidenceBundle is not persisted.
Violates: NIST SP 800-53 AU-3, AU-12.
Fix: Document that EvidenceBundle is the authoritative audit record and must be persisted by callers. Evaluate whether `release_max_level_warn` is more appropriate.

---

## `detect/file_ownership.rs` — 1 finding

**Finding 21 — LOW**
`find_stat_for_path` (line 162): `(dev, ino)` convention (symlink-followed statx) is not documented. Future PackageProbe implementations may use different flags producing inconsistent comparisons.
Violates: NSA RTB TOCTOU (anchor convention must be consistently documented).
Fix: Add doc comment stating the `(dev, ino)` anchor always reflects the symlink-target stat.

---

## `confidence.rs` — 1 finding

**Finding 22 — LOW**
`ConfidenceModel::upgrade` (line 144): security-critical function with no compliance citation despite module-level NSA RTB RAIN claim applying directly.
Violates: CLAUDE.md annotation requirement.
Fix: Add NSA RTB RAIN and NIST SP 800-53 SA-9 to `upgrade()` doc comment.

---

## Trust Model Integrity Assessment

**Monotone downgrade-only:** Sound. `level` is private; `upgrade()` and `downgrade()` correctly enforce the invariant.

**TrustedLabel without T4:** Not possible. Both stubs return `None` from `installed_digest()` so T4 is unreachable while stubs are active. Finding 15 concern is inflated T3, not bypassed T4.

---

## Summary Table

| Severity | Count |
|---|---|
| HIGH | 3 |
| MEDIUM | 9 |
| LOW | 10 |
| **Total** | **22** |

**Three doc-vs-code inconsistencies:**
- `integrity_check.rs`: `opened_by_fd: true` for path-based open
- `mount_topology.rs` module doc: "All procfs reads use ProcfsText" — namespace symlink does not
- `os_release.rs` module doc: "no raw strings" — `ansi_color: Option<String>` contradicts this
