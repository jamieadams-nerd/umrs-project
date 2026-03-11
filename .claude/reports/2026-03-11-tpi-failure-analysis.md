# Security Auditor Assessment — TPI Failure in umrs-ls for `/etc/NetworkManager`

**Date:** 2026-03-11
**Active audit depth:** in-depth
**Files examined:** `xattrs.rs`, `secure_dirent.rs`, `context.rs`, `type_id.rs`, `user.rs`, `role.rs`

---

## Root Cause

The error is not a TPI security event. It is a **parser constraint bug**.

`validate_type()` in `type_id.rs` restricted the character set for SELinux type names to `[a-z0-9_]` — lowercase only. The label on `/etc/NetworkManager` is `system_u:object_r:NetworkManager_etc_t:s0`. Both Path A and Path B call `SelinuxType::from_str()` — so both failed. Path A fails first, short-circuits, and the TPI gate is never reached.

**Status:** Fixed in task #2 — `is_ascii_lowercase()` changed to `is_ascii_alphabetic()`.

---

## Finding 1 — HIGH: Validator rejects valid SELinux policy type names

**File:** `type_id.rs`, line 168
**Finding:** The character-set constraint `[a-z0-9_]` did not match actual SELinux policy. The SELinux policy compiler (checkpolicy) allows uppercase letters in type names. `NetworkManager_etc_t`, `NetworkManager_t`, and `RPM_t` are all valid RHEL10 policy types.
**Citation:** NIST 800-53 SI-10; NIST 800-218 SSDF PW.4.4
**Status:** Fixed in task #2.

## Finding 2 — HIGH: False negative — parse failure indistinguishable from genuinely unlabeled

**File:** `secure_dirent.rs`, lines 928–934
**Finding:** The catch-all `Err(e)` arm leaves `selinux_ctx = None` for both `ENODATA` (genuinely unlabeled inode) and parse failures (parser bug). Both produce identical output: `<unlabeled> :: <no-level>`. An operator may conclude the object has no label when it is actually correctly labeled. This is a false negative in the security audit output.
**Citation:** NIST 800-53 AU-3, SI-12
**Remediation:** Introduce a separate display state (e.g., `<parse-error>` or `<unverifiable>`) for parse failures.

## Finding 3 — HIGH: TPI gate can be silently bypassed by a single-path parse failure

**File:** `xattrs.rs`, lines 65–68 and 77–86
**Finding:** When Path A fails, the function returns early before reaching the TPI cross-check gate. The redundancy check is never performed for any input that Path A cannot parse. The current architecture conflates two structurally distinct outcomes — "one path failed to complete" and "both paths completed but disagreed."

**Recommended remediation:** Introduce a typed error enum:
```rust
TpiError::PathAFailed(reason)       // parser/validator bug
TpiError::PathBFailed(reason)       // parser/validator bug
TpiError::Disagreement(a, b)        // CRITICAL security event
```
These three cases should produce different log levels, different `SecurityObservation` variants, and different display strings.
**Citation:** NSA RTB RAIN (Non-Bypassability); NIST 800-53 SI-7

## Finding 4 — LOW/MEDIUM: Nom error surfaces raw input slice in log

**File:** `xattrs.rs`, line 66
**Finding:** `log::error!("TPI Path A (nom) failed: {e}")` interpolates the nom error, which includes the verbatim input slice. For `NetworkManager_etc_t` this is not sensitive. However, if the parse failure occurs during MLS level parsing (e.g., `s3:c100,c200`), the sensitivity level and category set would appear in the error log — an SI-12 violation.
**Citation:** NIST 800-53 SI-12
**Remediation:** Log only the error kind without raw input; store full detail in controlled audit records.

---

## Key Distinction for Documentation

| Condition | Security meaning | Appropriate action |
|---|---|---|
| Both paths agree | Normal — label verified | Display label |
| Path A or B fails (format/validator bug) | Code defect — label unverifiable | Display `<parse-error>`, log WARN, `SecurityObservation::ParseFailure` |
| Both paths succeed but disagree | Potential integrity attack | Display `<restricted>`, log CRITICAL, `SecurityObservation::TpiDisagreement` |

The CRITICAL log should be reserved for condition 3 only. Condition 2 should never produce a CRITICAL event.
