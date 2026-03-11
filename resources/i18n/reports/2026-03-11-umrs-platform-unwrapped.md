# Wrapping Report — umrs-platform

Crate: umrs-platform
Domain: umrs-platform
Date: 2026-03-11

## Summary

This report is the developer's work instruction for wrapping user-facing strings
in `umrs-platform` for gettext extraction. It covers the same strings that have
been catalogued in the `.pot` template at
`resources/i18n/umrs-platform/umrs-platform.pot`.

**i18n crate**: `gettextrs = "0.7.7"` (same as `umrs-core` and `umrs-ls`).
**Domain name**: `umrs-platform`
**Macro to use**: `tr_platform(msgid)` — provided by the `i18n` module described
below. Use `tr_platform!()` or `umrs_platform::i18n::tr_platform()` as the
developer prefers; the module design mirrors `umrs-core::i18n::tr_core()`.

---

## Step 1: Add the dependency to Cargo.toml

In `components/rusty-gadgets/umrs-platform/Cargo.toml`, add:

```toml
gettextrs = "0.7.0"
```

(Exact version to match the workspace. Confirm against `umrs-core/Cargo.toml`
which currently pins `"0.7.7"`.)

---

## Step 2: Create src/i18n.rs

Create `components/rusty-gadgets/umrs-platform/src/i18n.rs` mirroring the
pattern in `umrs-core/src/i18n.rs`. Key difference: the library-scoped function
is named `tr_platform` and the domain is `"umrs-platform"`. The `tr()` function
(caller-domain lookup) and `tr_platform()` function (own-domain lookup) should
both be present, matching the `umrs-core` dual-function design.

The `init()` function signature and `OnceLock` pattern are identical. No other
changes are needed at the module level.

Add `pub mod i18n;` to `src/lib.rs`.

---

## Step 3: String wrapping instructions

Apply `tr_platform("original string")` at each site below. The string literal
argument must match the `msgid` in the `.pot` file exactly.

Because `thiserror` `#[error("...")]` attributes are proc-macro strings, they
cannot be wrapped with a runtime function call. Those strings are listed in
Section A below with a special note. All other sites (log macros) are in
Section B.

---

### Section A — `thiserror` `#[error]` attributes (detect/mod.rs, os_release.rs)

These strings are compiled into `Display` implementations by `thiserror`. They
cannot be wrapped with `tr_platform()` at the annotation site. The correct
approach is to wrap them at the call site where the error is formatted for
display (e.g., `eprintln!("{}", err)` becomes `eprintln!("{}", tr_platform(&err.to_string()))`).
A simpler alternative for library errors is to leave `#[error]` strings in
English and translate only at the binary display boundary.

**Recommendation for the developer**: leave `#[error]` strings as-is for now;
wrap them at display call sites in binary crates (`umrs-ls` or future tools)
that surface these errors to the user. Mark these strings in the `.pot` as
`# TRANSLATOR: shown via Display impl — wrap at binary display boundary`.

The strings are catalogued here for completeness so they appear in the `.pot`
and can be tracked:

```
file: components/rusty-gadgets/umrs-platform/src/detect/mod.rs
line: 81
string: "procfs is not real procfs — cannot establish kernel anchor"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/detect/mod.rs
line: 89
string: "PID coherence broken: syscall={syscall} procfs={procfs}"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/detect/mod.rs
line: 99
string: "I/O error during kernel anchor: {0}"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 48
string: "invalid ID field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 52
string: "invalid NAME field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 56
string: "invalid VERSION_ID field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 60
string: "invalid VERSION field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 64
string: "invalid VERSION_CODENAME field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 68
string: "invalid CPE_NAME field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 72
string: "invalid URL field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 76
string: "invalid VARIANT_ID field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 80
string: "invalid BUILD_ID field"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 84
string: "duplicate key"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 88
string: "non-UTF-8 content in os-release"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 92
string: "line too long ({0} bytes)"
note: thiserror #[error] — wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/os_release.rs
line: 96
string: "required field missing"
note: thiserror #[error] — wrap at binary display boundary
```

---

### Section B — log::warn! and log::error! strings

These strings appear inside `log::warn!()` and `log::error!()` macro invocations
and can be wrapped with `tr_platform()`. The pattern is:

```rust
// Before:
log::warn!("mount_topology: could not read mountinfo: {e}");

// After:
log::warn!("{}", tr_platform("mount_topology: could not read mountinfo: {e}"));
```

For strings with inline format arguments, extract the static prefix as the msgid
and format separately, or use a pre-formatted string approach. See the
`umrs-core` or `umrs-ls` wrapping patterns for the established convention.

**Note on log strings**: log messages at `debug!` level are not user-facing in
the sense that end users never see them during normal operation. Only `warn!` and
`error!` level messages may realistically surface to users or operators. The
developer should decide with the translator whether `warn!`/`error!` log strings
in a library crate warrant runtime translation (they add overhead to every log
call). An acceptable alternative is to wrap only at the binary display layer.
This report lists all `warn!` and `error!` strings for completeness; the
developer and architect should agree on scope before implementing.

```
file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 119
string: "kernel_anchor: /proc/self/stat path rejected by ProcfsText"
macro to use: log::error!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 128
string: "kernel_anchor: /proc/self/stat failed filesystem magic check"
macro to use: log::error!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 131
string: "kernel_anchor: /proc/self/stat I/O error"
macro to use: log::error!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 138
string: "kernel_anchor: /proc/self/stat content exceeds expected size"
macro to use: log::error!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 178
string: "kernel_anchor: getpid() returned non-positive value"
macro to use: log::error!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 184
string: "kernel_anchor: could not parse PID from /proc/self/stat"
macro to use: log::error!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: ~191
string: "kernel_anchor: PID coherence failure (syscall={syscall_pid}, procfs={proc_pid})"
macro to use: log::error!("{}", tr_platform("..."))
note: format arguments must be handled separately; msgid is the static template

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 222
string: "kernel_anchor: could not read boot_id: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 245
string: "kernel_anchor: boot_id content unexpectedly large, ignoring"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs
line: 298
string: "kernel_anchor: could not read kernel lockdown mode: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs
line: 84
string: "mount_topology: partial failure — confidence not upgraded to T2"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs
line: 123
string: "mount_topology: could not read mnt namespace link: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs
line: 160
string: "mount_topology: ProcfsText rejected mountinfo path: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs
line: 172
string: "mount_topology: could not read mountinfo: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs
line: ~194
string: "mount_topology: mountinfo content ({} bytes) exceeds cap ({} bytes)"
macro to use: log::warn!("{}", ...)
note: format arguments content.len() and max_mountinfo_bytes must remain inline

file: components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs
line: 271
string: "mount_topology: statfs(/etc) failed: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_candidate.rs
line: 110
string: "release_candidate: no usable os-release candidate found at any standard path"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_candidate.rs
line: 149
string: "release_candidate: {path_str} is world-writable — rejecting candidate"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/pkg_substrate.rs
line: 121
string: "pkg_substrate: no probe succeeded — substrate identity unavailable"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/pkg_substrate.rs
line: ~132
string: "pkg_substrate: identity has only {} fact(s) — T3 threshold not met (requires 2)"
macro to use: log::warn!("{}", ...)
note: format argument identity.facts_count must remain inline

file: components/rusty-gadgets/umrs-platform/src/detect/pkg_substrate.rs
line: ~154
string: "pkg_substrate: SELinux is not in enforce mode — T3 degraded; confidence remains at EnvAnchored"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/pkg_substrate.rs
line: 208
string: "pkg_substrate: could not read SELinux enforce state: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/file_ownership.rs
line: 99
string: "file_ownership: no stat record found for candidate — cannot anchor ownership query"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/file_ownership.rs
line: 134
string: "file_ownership: {candidate_str} has no package owner — T4 cannot be reached"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: 123
string: "integrity_check: no installed digest available for {candidate_str} — T4 not earned"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: ~171
string: "integrity_check: package DB uses SHA-512 but we computed SHA-256 — cross-algorithm comparison not supported; T4 not earned"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: 139
string: "integrity_check: could not open {candidate_str}: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: 152
string: "integrity_check: read failed for {candidate_str}: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: ~213
string: "integrity_check: package DB digest for {candidate_str} uses MD5 — algorithm is cryptographically weak; T4 not earned"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: ~239
string: "integrity_check: unknown digest algorithm '{alg}' for {candidate_str} — T4 not earned"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: 289
string: "integrity_check: SHA-256 digest MISMATCH for {candidate_str} — file may have been modified"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs
line: 353
string: "os-release file exceeds maximum read limit"
note: io::Error payload — surfaces through error Display; wrap at binary display boundary

file: components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs
line: ~125
string: "release_parse: line in {candidate_str} exceeds max_line_len ({line_len} bytes)"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs
line: 139
string: "release_parse: nom parser failed for {candidate_str}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs
line: 153
string: "release_parse: TPI disagreement — nom and split_once produced different key sets"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs
line: ~182
string: "release_parse: OsRelease construction failed for {candidate_str}: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs
line: 238
string: "release_parse: could not read {candidate_str}: {e}"
macro to use: log::warn!("{}", tr_platform("..."))

file: components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs
line: ~490
string: "release_parse: substrate contradiction: {contradiction}"
macro to use: log::warn!("{}", tr_platform("..."))
```

---

## Strings explicitly excluded

The following string categories were scanned and intentionally excluded:

- `log::debug!` strings — not user-facing under normal operation; excluded per
  project scope.
- String literals used as map keys (`"ID"`, `"NAME"`, `"VERSION_ID"`, etc.) —
  internal parsing logic, not user-facing.
- Evidence record `notes` field values — internal audit trail data, never shown
  to end users at the surface.
- `confidence.downgrade()` reason strings — internal model state, not surfaced
  to end users.
- Field values such as `"rpm"`, `"dpkg"`, `"procfs gate: PROC_SUPER_MAGIC verified"` —
  internal evidence annotations.
- `OsReleaseParseError` inner `String` payloads (the offending input values) —
  these are untrusted user-controlled data and must not be extracted into `.po`
  files (NIST SP 800-53 SI-12).

---

## Scope decision required

Before the developer implements wrapping, please confirm the agreed scope with
the architect:

1. **Library log strings** — should `warn!`/`error!` strings in a library crate
   be wrapped at runtime? This adds a `gettextrs` lookup per log call. The
   alternative is to wrap only at the binary display boundary.

2. **`thiserror` strings** — confirmed approach is wrap-at-display-boundary in
   binary crates. No changes to `#[error]` annotations.

Once scope is confirmed, the developer implements Section B wrapping and adds
`gettextrs` to `Cargo.toml`. The translator will then run `xtr` to regenerate
the `.pot` from source and validate all msgids match.
