# Full Workspace Quality and High-Assurance Pattern Audit
**Date:** 2026-04-02  
**Author:** Rusty (rust-developer agent)  
**Scope:** All 10 crates — umrs-core, umrs-hw, umrs-selinux, umrs-platform, umrs-ui, umrs-label, umrs-ls, umrs-stat, umrs-uname, umrs-c2pa  
**Type:** READ-ONLY audit — no code was modified

---

## Session Checklist Results

- **Outstanding reports assigned to coder:** None found.  
- **`cargo xtask clippy`:** Clean. Zero warnings.  
- **`cargo xtask test`:** Not run (read-only audit; known setrans failures in umrs-label are pre-existing).  
- **Git status:** Clean. No modified files from prior sessions.

---

## Summary Table

| Category | Count | Severity |
|---|---|---|
| Security advisories (RUSTSEC) | 4 total (1 vulnerability, 3 warnings) | See §1 |
| `unwrap()` calls in production code | 7 | Medium |
| `eprintln!` in library code | 2 | Medium |
| Bare `#[must_use]` without message | 168 | Low-Medium |
| `todo!()` stubs in public API | 1 | Medium |
| Unregistered module (`umrs-core/src/fs/`) | 1 | Low |
| Placeholder modules (empty bodies) | 2 | Low |
| `#[allow(dead_code)]` in production struct | 1 | Low |
| SecureReader bypass (annotated) | Several | Documented/intentional |
| Missing `//!` compliance sections | 0 | — |
| Pre-existing setrans test failures | 7 | Known |

---

## §1 — Supply Chain and Security Advisories

### RUSTSEC-2023-0071 — `rsa` crate: Marvin Attack (VULNERABILITY)

**Severity:** Medium (CVSS 5.9) — timing side-channel allowing potential key recovery.  
**Location:** `rsa 0.9.10` → `c2pa 0.78.7` → `umrs-c2pa`  
**Status:** No fixed version available upstream.  

This is a transitive dependency through the `c2pa` SDK. UMRS does not directly call RSA
operations; the `rsa` crate is used internally by the SDK for signature operations.
Assessment: on RHEL 10 FIPS-active systems, RSA signing uses the system OpenSSL FIPS module
via the `openssl` crate, not the pure-Rust `rsa` crate. The risk is present in non-FIPS
environments. **Action required:** Confirm with Jamie whether non-FIPS deployments are
in scope. If so, file a supply chain risk item and track the upstream c2pa SDK for an
update that removes or patches the `rsa` dependency.

### RUSTSEC-2026-0002 — `lru` 0.12.5: Unsound `IterMut` (WARNING)

**Location:** `lru 0.12.5` → `ratatui 0.29.0` → `ratatui-garnish 0.1.0` → `umrs-ui`  
**Root cause:** `ratatui-garnish` pins to the older `ratatui 0.29.0` which depends on
`lru 0.12.5`. The workspace's direct `ratatui` dependency specifies `"0.30.0"`, which
resolves to `lru 0.16.3` (safe). Both `lru` versions are in the lock file simultaneously:
`0.12.5` via `ratatui-garnish` and `0.16.3` via the direct `ratatui 0.30.0` path.  

**Assessment:** The unsound `IterMut` in `lru 0.12.5` is exercised by ratatui's internal
widget cache. The code path that calls `IterMut` is inside `ratatui-widgets`, which is
UI rendering code — not a security-relevant data path. However, soundness violations are
categorical safety issues.  
**Recommended path:** Check whether `ratatui-garnish` has a version compatible with
`ratatui 0.30.0`. If yes, bump the `ratatui-garnish` requirement. If not, assess whether
`ratatui-garnish` can be removed; its TUI garnish functionality may be achievable with
plain ratatui styling in `umrs-ui/src/theme.rs`.

### RUSTSEC-2024-0436 — `paste` 1.0.15: Unmaintained (WARNING)

**Location:** `paste` → `ratatui 0.29.0` → `ratatui-garnish`. Same root cause as above.
Resolution is the same as RUSTSEC-2026-0002.

### RUSTSEC-2024-0370 — `proc-macro-error` 1.0.4: Unmaintained (WARNING)

**Location:** `proc-macro-error` → `static-regular-grammar` → `iref` → `c2pa` → `umrs-c2pa`.
This is a proc-macro only used at compile time. Runtime impact is zero. Track the upstream
`c2pa` SDK for removal of this dependency.

---

## §2 — umrs-core

**Path:** `libs/umrs-core/src/`

### 2.1 Unregistered module: `src/fs/`

`umrs-core/src/fs/mod.rs` exists on disk but is **not declared in `lib.rs`**. The module
self-documents this intentionally in its `//!` header, noting that it must not be wired in
until `ProcfsText`/`SysfsText` replaces the raw `std::fs::read_to_string` calls for
`/proc/mounts` and `/sys/class/block/`. The module is well-commented and fail-safe.

**No immediate action required.** The exclusion is correct and documented. When `umrs-core`
gains a `umrs-platform` dependency, this module can be enabled. Track as a future migration
item.

### 2.2 Placeholder audit module

`src/audit/emit.rs` and `src/audit/events.rs` both contain only `// Placeholder` comments.
They are declared in `audit/mod.rs` and therefore compile, but contribute nothing. This is
low risk — the intent is clear. The audit infrastructure design is documented in the module
header.

**Finding:** These placeholder files should either have stub types or be removed and replaced
with a single comment in `audit/mod.rs` noting what comes next. Two empty files are noise
for auditors.

### 2.3 `validate.rs` — `expect()` on compiled-in regex

`umrs_core::validate::get_regex()` calls `.expect("UmrsPattern regex failed to compile")`.
This is correct — the patterns are compile-time constants authored by the developer.
A panic here is a programmer error, not a runtime condition.

**Assessment:** Acceptable. The `expect` message is informative. No change needed. This
pattern is consistent with how `umrs-selinux::validate` handles the same situation.

### 2.4 `typography.rs` — `unwrap()` on character arithmetic

`map_char_circled()` calls `c.to_digit(10).unwrap()` (line 79) after establishing that
`c.is_ascii_digit()` is true. `to_digit(10)` on an ASCII digit always returns `Some`,
so this unwrap cannot panic in practice. However, it is a clippy violation.

**Finding (QA-01):** Replace `c.to_digit(10).unwrap()` with `c.to_digit(10).unwrap_or(0)`.
This is cosmetic and safe — the unwrap is proven by the preceding guard — but the project
rule is `#[deny(clippy::unwrap_used)]` and this should be consistent.

---

## §3 — umrs-hw

**Path:** `libs/umrs-hw/src/`

### 3.1 Overall quality: Excellent

The unsafe isolation boundary is well-documented and confined to a single `asm!` block.
The architecture-specific fallback (RDTSCP on x86_64, `CLOCK_MONOTONIC_RAW` on aarch64)
is clean. `#![forbid(unsafe_code)]` is intentionally absent and documented. Test coverage
in `tests/hw_timestamp_tests.rs` is present.

No findings.

---

## §4 — umrs-selinux

**Path:** `libs/umrs-selinux/src/`

### 4.1 `context.rs` — `todo!()` stub in public API

`SecurityContext::dominates()` is a `todo!()` macro stub:

```rust
pub fn dominates(&self, _other: &Self) -> bool {
    todo!("Lattice dominance logic pending CategorySet bitmask integration")
}
```

**Finding (QA-02, Medium):** A `todo!()` in a public `#[must_use]` method panics if any
caller invokes it. The module-level `//!` documents this stub, which is good. However,
the method lacks a `#[must_use]` message and there is no type-level suppression to prevent
callers from accidentally using it.

**Recommendation:** Add a `#[deprecated(note = "dominates() is not yet implemented; do not call")]`
attribute to the method until the lattice math is complete. This turns an accidental runtime
panic into a compile-time warning. The `todo!()` can remain as the body.

### 4.2 `mcs/colors.rs` — `eprintln!` in library code

Two distinct issues:

**Finding (QA-03, Medium):** Line 170 — `eprintln!("secolor.conf parse warning line {}: {}", ...)` 
is called in `parse_secolor_file()`. This writes to stderr from library code, bypassing the
`log` crate and journald. On a systemd-managed RHEL 10 service, `eprintln!` output is lost.

**Finding (QA-04, Medium):** Line 181 — `eprintln!("Examining: {line}")` is a debug-only
diagnostic that was apparently left in from development. It fires for **every non-empty,
non-comment line** in `secolor.conf`. This is chatty debug output in library code and must
be removed or converted to `log::trace!`.

Both `eprintln!` calls should be replaced with `log::warn!` (QA-03) and `log::trace!` (QA-04)
respectively. This is consistent with how the rest of the codebase handles logging.

### 4.3 `mcs/colors.rs` — `#[allow(clippy::unwrap_used)]` for `RwLock` poisoning

Three `#[allow(clippy::unwrap_used)]` annotations exist in `load_secolors_cached()` for
`RwLock::read()` and `RwLock::write()`. The comments document the rationale
("RwLock poisoning is unrecoverable").

**Assessment:** The rationale is sound. However, the project policy prefers `#[expect]`
over `#[allow]`. These should be migrated to `#[expect(clippy::unwrap_used, reason = "...")]`
opportunistically when the file is next touched.

### 4.4 `mcs/colors.rs` — TOCTOU concern with secolor.conf

`parse_secolor_file()` calls `fs::read_to_string(path)` — a raw file read without
provenance verification. The `secolor.conf` path is under `/etc/selinux/{policy}/` —
a configuration filesystem, not procfs or sysfs. The TOCTOU/SecureReader rule applies
only to `/proc/` and `/sys/` reads; this read is within scope.

**Assessment:** No violation. The architectural note (SecureReader mandatory only for
procfs/sysfs) is correctly applied here.

### 4.5 `context.rs` — Bare `#[must_use]` on `SecurityContext`

`SecurityContext` carries `#[must_use]` at the type level without a message string
(line 102 of `context.rs`).

**Finding (QA-05):** Per the Must-Use Contract Rule, `#[must_use]` must include a message
string. Should read: `#[must_use = "discarding a SecurityContext silently drops the access control label"]`.

### 4.6 `status.rs` — Raw `File::open` on `/etc/selinux/config`

`selinux_policy()` in `status.rs` opens `/etc/selinux/config` with `File::open`. This is
a configuration file, not a kernel pseudo-filesystem. The Trust Gate Rule says: gate config
reads behind a kernel status check. The caller (`selinux_policy()`) is called only after
the kernel check passes. This is architecturally correct.

**Assessment:** No violation.

### 4.7 `secure_dirent.rs` — `expect()` on `AbsolutePath` construction

Line 404: `.expect("AbsolutePath invariant: no null bytes")`. This asserts an invariant
that was established earlier in the constructor (the path was already opened as a valid
file, so null bytes cannot be present). The message is precise. Acceptable — this is a
TCB invariant, not a runtime condition. Consistent with how `xattrs.rs` handles `SensitivityLevel::new(0)`.

### 4.8 Test coverage: Strong

`tests/` contains 15 files covering context, category, xattrs, TPI errors, MLS levels,
POSIX identity, role, type, user, validate, MCS translator, dirlist, and label state.
Happy-path and error-path coverage appears strong. The `xattr_log_discipline_tests.rs` file
specifically tests the error information discipline pattern — this is mature.

**Gap:** No tests for `mcs/colors.rs` (`parse_secolor_file`, `resolve_colors`, `glob_match`).
The `QA-04` spurious `eprintln!` would pollute test output if these were added.

---

## §5 — umrs-platform

**Path:** `libs/umrs-platform/src/`

### 5.1 Overall quality: Excellent

The most architecturally mature crate in the workspace. Compile-time path binding,
`SecureReader` provenance verification, `SealedCache`, TPI on RPM header parsing,
and saturating arithmetic throughout are all correctly applied.

### 5.2 `posture/bootcmdline.rs` — Annotated unverified procfs read

`read_kernel_osrelease()` uses `std::fs::read_to_string(KERNEL_OSRELEASE)` without
`SecureReader`. The function carries an explicit security note documenting that:
- This read is heuristic-only (not a security assertion)
- The deployment assumption (SELinux enforcing = kernel owns procfs VFS) is stated
- A future upgrade path to `ProcfsText` is identified

**Assessment:** The annotation is compliant with the project's expectation that architectural
deviations be explicitly documented. No action required unless the deployment assumption
changes.

### 5.3 `posture/configured.rs` — Raw `std::fs::read_to_string` on sysctl.d files

`load_conf_file()` reads `/etc/sysctl.d/*.conf` via `std::fs::read_to_string`. These are
configuration files under `/etc/`, not procfs/sysfs. The SecureReader mandate applies only
to `/proc/` and `/sys/` reads. This is architecturally correct.

**Assessment:** No violation.

### 5.4 `detect/substrate/rpm_header.rs` — `#[allow(dead_code)]` on struct field

The `release` field in `RpmFileEntries` (line 267) carries `#[allow(dead_code)]` with a
comment: "Not used in the current pipeline but retained for completeness." This is a mild
quality concern.

**Finding (QA-06, Low):** If this field is not used, it should either be used or removed.
Retaining dead fields in a struct that participates in the security substrate parse path
adds audit surface. If future use is genuinely planned, replace `#[allow(dead_code)]` with
`#[expect(dead_code, reason = "retained for full NVR display in a future phase")]`.

### 5.5 `detect/integrity_check.rs`, `detect/mod.rs`, `detect/release_parse.rs` — `#[allow(clippy::too_many_lines)]`

Three production functions carry `#[allow(clippy::too_many_lines)]`. These are documented
as sequential pipeline functions where splitting would reduce clarity.

**Assessment:** Policy note — these should be migrated to `#[expect]` if not already. The
suppression is justified in each case (sequential validation pipelines). Verify that all
three use `#[allow]` and not `#[expect]` — migrate opportunistically.

### 5.6 `posture/catalog.rs` — Exemplary compile-time path binding

The static indicator catalog as a `const` Rust array is the reference implementation of
the Compile-Time Path Binding Rule. All paths, sysctl keys, desired values, and impact
tiers are compiler-verified. No findings.

### 5.7 Test coverage: Strong

13 test files. The Signal Evaluation Path Rule is honored — `posture_tests.rs` exercises
contradiction detection. The `rpm_header_tests.rs` covers TPI on RPM header parsing.

---

## §6 — umrs-ui

**Path:** `libs/umrs-ui/src/`

### 6.1 `dialog.rs` — `unwrap()` in example usage in doc comment

The doc comment in `dialog.rs` module header uses `.unwrap()` in example code:
`dialog.as_ref().unwrap().response`. This is in a comment block, not production code.
Low severity, but example code influences how consumers write callers.

**Finding (QA-07, Low):** Replace the `unwrap()` in the example with `if let Some(d) = dialog.as_ref()`.
Example code should model correct style.

### 6.2 Clippy configuration — `Cargo.toml` vs `lib.rs` duplication

`umrs-ui` specifies clippy lints in `Cargo.toml`'s `[lints.clippy]` section AND in
`lib.rs`'s `#![warn/deny/allow]` attributes. The `lib.rs` form takes precedence but the
two could drift. This is a minor consistency issue.

**Finding (QA-08, Low):** Choose one location for clippy configuration. The workspace
standard appears to be `lib.rs` attributes. Remove the `[lints.clippy]` section from
`umrs-ui/Cargo.toml` and ensure the `lib.rs` attributes are the single source of truth.

### 6.3 RUSTSEC-2026-0002 path via `ratatui-garnish`

See §1. `ratatui-garnish 0.1.0` pins to `ratatui 0.29.0`, which pulls `lru 0.12.5`.
The workspace direct dependency on `ratatui 0.30.0` does not fix this because
`ratatui-garnish` is independently resolved.

**Finding (QA-09, Medium):** Assess whether `ratatui-garnish` can be updated to a version
compatible with `ratatui 0.30.0`, or whether its features can be replicated without it.
Until resolved, the workspace carries a soundness warning in every build.

### 6.4 Test coverage: Good

10 test files covering all major subsystems. Viewer, dialog, config, keymap, theme,
indicators, header, audit card state — all covered.

---

## §7 — umrs-label

**Path:** `umrs-label/src/`

### 7.1 `cui/catalog.rs` — `File::open` on JSON catalogs

`load_catalog()` and `load_levels()` open JSON files via `File::open`. These are catalog
files on the regular filesystem (not kernel pseudo-filesystems). The SecureReader mandate
does not apply. This is correct.

### 7.2 Known pre-existing test failures

7 tests in `tests/setrans_tests.rs` fail due to CUI catalog category range mismatches.
This is a known state per the project memory.

### 7.3 Error types — `String` used for catalog load errors

`load_catalog()` and `load_levels()` return `Result<_, String>`. The rest of the workspace
uses `thiserror`-derived error enums. Using `String` for errors violates the Security
Findings as Data Rule — callers cannot programmatically distinguish a missing file from
a malformed JSON document.

**Finding (QA-10, Medium):** Define a `CatalogError` enum (e.g., `Io(std::io::Error)`,
`Parse(serde_json::Error)`, `MissingField(&'static str)`) and return that from catalog
load functions. This aligns `umrs-label` with the workspace error pattern and allows
callers to match on error kind. This is the most significant code quality gap in this crate.

### 7.4 Test coverage: Adequate

`catalog_tests.rs`, `setrans_tests.rs`, `validate_tests.rs` are present.

---

## §8 — umrs-ls

**Path:** `umrs-ls/src/`

### 8.1 `lib.rs` — `#[allow(clippy::map_unwrap_or)]`

`lib.rs` carries `#[allow(clippy::map_unwrap_or)]` at crate level. This is a broader
suppression than needed.

**Finding (QA-11, Low):** Migrate to `#[expect]` if this lint still fires, or remove it
if it no longer applies after a code change. A crate-level allow for a lint that may not
even be triggering is dead suppression.

### 8.2 No library examples

`umrs-ls` has a `lib.rs` exposing `grouping` for test access, but no `examples/` directory.
The Module and Workspace Conventions require at least one example per public module.

**Finding (QA-12, Low):** Add an `examples/group_demo.rs` demonstrating `group_entries()`
and `sibling_summary()`.

### 8.3 Test coverage: Adequate

`tests/grouping_tests.rs` covers the public grouping API.

---

## §9 — umrs-stat

**Path:** `umrs-stat/src/`

### 9.1 `main.rs` — `eprintln!` in error handling

Line 768: `eprintln!(...)` used in the CLI main error path. This is a binary entry point —
`eprintln!` is appropriate here for pre-logger-init error reporting.

**Assessment:** No violation.

### 9.2 Overall: Good

The crate correctly uses `SecureDirent::from_path()` as its primary data gathering path,
which encapsulates TPI, TOCTOU safety, and provenance verification behind the API boundary.
The three-tab audit card structure is well-designed.

---

## §10 — umrs-uname

**Path:** `umrs-uname/src/`

### 10.1 Overall: Good

Correctly delegates detection to `OsDetector` which enforces the full provenance and trust
pipeline. The audit card displays the trust tier, evidence chain, and contradiction findings.

No findings beyond what is already covered by cross-cutting issues.

---

## §11 — umrs-c2pa

**Path:** `umrs-c2pa/src/`

### 11.1 `creds.rs` and `validate.rs` — Raw read of `/proc/self/status`

`creds.rs` line 474 and `validate.rs` line 549 both read `/proc/self/status` via
`std::fs::read_to_string("/proc/self/status")`. This is a procfs path — under the
Mandatory pattern, procfs reads MUST use `ProcfsText` + `SecureReader`.

**Finding (QA-13, Medium):** These reads bypass the provenance verification layer.
The reads are used to check process memory usage (for diagnostic purposes only in
`creds.rs`) and to check uid (for the permission check in `validate.rs`). Neither is
a security-critical assertion, but the rule is categorical for procfs paths.

**Recommendation:** Either:
1. Route through `ProcfsText` + `SecureReader` from `umrs-platform` (requires adding
   `umrs-platform` as a dependency to `umrs-c2pa`).
2. Obtain the uid via `rustix::process::getuid()` (already available through transitive
   deps), which avoids the procfs read entirely for the uid check.
3. Add `umrs-platform` as a dep and use `SecureReader` for the `/proc/self/status` read,
   consistent with the workspace standard.

Option 2 is cleanest for the uid check. Option 1/3 for the memory diagnostic.

### 11.2 `trust.rs` — `#[allow(unused_imports)]`

Line 53: `#[allow(unused_imports)]` on `use crate::verbose;`. This suppresses a warning
about an import that is conditionally used by the `verbose!` macro. This is acceptable
but should be `#[expect]`.

### 11.3 `error.rs` — Error type is not thiserror

`InspectError` implements `Display` manually (documented: to support i18n). The `source()`
method is manually implemented. The from impls are manually implemented. This is the
correct design decision for i18n compatibility — the module doc explains it clearly.

**Assessment:** Design is sound and well-documented. No findings.

### 11.4 RUSTSEC-2023-0071 (`rsa` crate)

See §1 for the full analysis.

### 11.5 `ingest.rs` — TOCTOU handling is exemplary

`ingest_file()` reads the source file into memory once, then both SHA-256 and the signing
operation use the in-memory buffer. This is explicitly documented as eliminating the
TOCTOU window. This is the correct pattern and should be noted as a reference implementation
for future file-processing pipelines in this workspace.

---

## §12 — Cross-Cutting Findings

### 12.1 Bare `#[must_use]` without message string (168 instances)

The Must-Use Contract Rule requires: `#[must_use = "reason why the return value matters"]`.
168 `#[must_use]` annotations across the workspace are bare — they carry no message string.
The bulk are in `umrs-selinux/src/secure_dirent.rs` (simple accessors), `umrs-platform/`
(evidence and os_release newtypes), and `umrs-selinux/src/status.rs`.

**Assessment:** Many of these are simple accessor functions on types whose parent struct is
already annotated with compliance references. The Tiered Annotation Expectations Rule
permits omitting citations on simple accessors. However, the Must-Use Contract Rule still
requires the message string even on simple accessors.

**Finding (QA-14, Low-Medium):** This is a systemic gap affecting every crate. The fix is
mechanical: add a brief message string to each. Priority should be:
1. Security-relevant types first: `SecurityContext`, `MlsLevel`, `CategorySet`
2. `umrs-platform` evidence and posture types
3. Simple accessors last (acceptable if deferred)

The `security_auditor` agent should flag this in their next review cycle.

### 12.2 `#[allow]` vs `#[expect]` inconsistency

The project rules state: prefer `#[expect(lint_name)]` over `#[allow(lint_name)]`.
Several `#[allow]` suppressions exist in non-crate-root files across `umrs-selinux`,
`umrs-platform`, and `umrs-c2pa`. These should be migrated to `#[expect]` opportunistically.
`#[expect]` errors if the lint stops firing, preventing dead suppressions from accumulating.

**Finding (QA-15, Low):** Migrate `#[allow]` to `#[expect]` in all non-crate-root files
on next touch. Priority files: `mcs/colors.rs` (3 instances), `detect/integrity_check.rs`,
`detect/mod.rs`, `detect/release_parse.rs`, `detect/substrate/rpm_header.rs`.

### 12.3 Error type inconsistency: `String` vs typed enums

`umrs-label` uses `String` for catalog load errors. All other crates use `thiserror`-derived
enums or manually-implemented structured error types. This is the only cross-crate
inconsistency in error design. (See QA-10.)

### 12.4 Clippy configuration: mixed `lib.rs` attributes and `Cargo.toml` `[lints]` sections

`umrs-ui` specifies clippy configuration in both `lib.rs` and `Cargo.toml`. The other
library crates use only `lib.rs` attributes. The binary crates (`umrs-stat`, `umrs-uname`,
`umrs-ls`) use `main.rs`/`lib.rs` attributes. This is internally consistent per crate type
but the `umrs-ui` duplication should be resolved. (See QA-08.)

### 12.5 HA Pattern opportunities not yet applied

The following undocumented HA pattern opportunities were identified:

**Pattern Opportunity: Constant-Time Comparison for Certificate Fingerprints (umrs-c2pa)**

`umrs-c2pa/src/c2pa/creds.rs` compares certificate subjects and validity dates as
strings. If any future comparison involves trust anchors or credential hashes (not
currently the case), `subtle::ConstantTimeEq` should be applied. Current comparisons
are metadata checks, not cryptographic comparisons — this is currently out of scope.
No action required. Flag if certificate fingerprint comparison is added.

**Pattern Opportunity: Zeroize on `umrs-c2pa` ephemeral key bytes**

In `creds.rs`, when private key PEM bytes are read into a `Zeroizing<Vec<u8>>`, this
is correctly handled. When cert bytes are read into a plain `Vec<u8>` (line ~267), this
is not zeroized. Cert data is not secret material, so this is not a violation — but
note that any future operation reading private key data into a non-zeroizing buffer
would be a finding.

**Pattern Opportunity: TPI on BLS entry parsing (umrs-platform)**

`parse_bls_content()` in `bootcmdline.rs` uses a single-path parser (manual string
splitting). BLS entries are boot configuration — security-relevant but not a credential
or label parse. The TPI rule explicitly does not apply to kernel attribute parsers
(boolean/dual-boolean). BLS entries are more complex than boolean, but they are
boot parameters rather than security labels. This is a judgment call for Jamie — raise
before implementing.

---

## §13 — Pre-Existing Known Issues

### 13.1 Seven failing setrans tests in umrs-label

`tests/setrans_tests.rs` has 7 failures related to CUI catalog category range mismatches.
These are known pre-existing failures unrelated to this audit's scope.

### 13.2 `SecurityContext::dominates()` todo!() stub

Documented in §4.1 (QA-02). The lattice dominance math is pending `CategorySet`
bitmask integration.

---

## Priority Ranking

| ID | Finding | Crate | Priority |
|---|---|---|---|
| RUSTSEC-2023-0071 | `rsa` Marvin Attack via c2pa SDK | umrs-c2pa | High — assess FIPS scope |
| QA-13 | Raw `/proc/self/status` reads bypass SecureReader | umrs-c2pa | Medium |
| RUSTSEC-2026-0002 | `lru 0.12.5` unsound via ratatui-garnish | umrs-ui, umrs-stat, umrs-uname | Medium |
| QA-02 | `dominates()` is a silent panic via `todo!()` | umrs-selinux | Medium |
| QA-03 | `eprintln!` in `parse_secolor_file()` bypasses log/journald | umrs-selinux | Medium |
| QA-04 | Debug `eprintln!("Examining: {line}")` fires on every conf line | umrs-selinux | Medium |
| QA-10 | `String` error type in catalog load functions | umrs-label | Medium |
| QA-01 | `unwrap()` in `map_char_circled()` — proven but inconsistent | umrs-core | Low |
| QA-05 | Bare `#[must_use]` on `SecurityContext` without message | umrs-selinux | Low |
| QA-14 | 168 bare `#[must_use]` without message strings across workspace | All | Low |
| QA-15 | `#[allow]` → `#[expect]` migration pending | umrs-selinux, umrs-platform | Low |
| QA-06 | `#[allow(dead_code)]` on struct field in rpm_header.rs | umrs-platform | Low |
| QA-07 | `unwrap()` in dialog.rs doc comment example | umrs-ui | Low |
| QA-08 | Clippy config in both `lib.rs` and `Cargo.toml` in umrs-ui | umrs-ui | Low |
| QA-09 | `ratatui-garnish` pinned to ratatui 0.29.0 (lru vulnerability) | umrs-ui | Medium |
| QA-11 | Crate-level `#[allow(map_unwrap_or)]` may be dead suppression | umrs-ls | Low |
| QA-12 | No `examples/` directory in umrs-ls | umrs-ls | Low |

---

## Strengths Worth Preserving

1. **`#![forbid(unsafe_code)]` across 9 of 10 crates.** The one exception (`umrs-hw`) is
   correctly documented as the designated unsafe boundary with a clear architectural rationale.

2. **TPI on security-critical parse paths.** `xattrs.rs` with `nom` + `FromStr` cross-check,
   the RPM header parser using the same dual-path approach — these are production-quality
   implementations of the pattern.

3. **`SealedCache` in umrs-platform.** HMAC-SHA-256 seal with FIPS-aware disable path,
   ephemeral key derived from boot_id + process start time, `ZeroizeOnDrop` — textbook
   implementation.

4. **`SecureDirent` design.** Parse-once constructor, TOCTOU-safe fd-anchored I/O,
   typed security findings as enum variants — the cleanest type in the codebase.

5. **Compile-time indicator catalog.** The `const` array approach in `posture/catalog.rs`
   is the right design for audit-relevant configuration: compiler-verified, no runtime I/O
   error path, no substitution attack surface.

6. **`umrs-c2pa` ingest TOCTOU pattern.** Single in-memory read used by both SHA-256 and
   signing — the explicit documentation of why this eliminates the TOCTOU window is
   exactly what auditors need to see.

7. **`SubstrateIdentity::add_fact()` with `saturating_add`.** Correctly applies the ANSSI
   guideline for integer arithmetic on security-relevant values. The doc comment citing
   ANSSI is exemplary.

8. **Error Information Discipline in `posture/configured.rs`.** The `log::debug!` that
   deliberately suppresses configuration values (only logging key and line number) is a
   reference implementation with an explicit comment explaining why. This is exactly what
   the rule requires.

---

*Report complete. No code was modified during this audit.*
