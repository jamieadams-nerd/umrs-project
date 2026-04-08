Audit date: 2026-04-08
Depth: in-depth
Scope: umrs-label/src/main.rs, umrs-ls/src/main.rs, umrs-ls/src/viewer_app.rs, umrs-stat/src/main.rs, Cargo.toml for all three crates
Reviewer: Knox (security-engineer)

# TUI Binary Crates — Security Code Review

## Summary Table

| Crate | ACCURATE | CONCERN | ERROR |
|---|---|---|---|
| umrs-label | 6 | 1 | 0 |
| umrs-ls | 8 | 3 | 0 |
| umrs-stat | 7 | 2 | 0 |
| **Total** | **21** | **6** | **0** |

---

## umrs-label

### A-1: `#![forbid(unsafe_code)]` Present
Line 35 of `main.rs` and line 7 of `lib.rs`. Both the binary and library crate roots carry the `forbid` directive. Compile-time proof of safe code.

### A-2: OsDetector Integration Correct
Lines 192-203 of `main.rs`. Data flows through validated newtypes: `rel.name.as_str()`, `rel.version_id.as_ref().map(|v| v.as_str())`. Falls back to `"unavailable"` on detection failure. No raw string extraction.

### A-3: No Direct System State Reads
The only `File::open` calls in the crate are in `cui/catalog.rs` (lines 166, 419) — these open user-specified JSON catalog files, not system paths. No reads on `/etc/`, `/proc/`, `/sys/`, or `/run/` anywhere in the crate.

### A-4: Fail-Closed on Detection Failure
Line 203: `.unwrap_or_else(|| "unavailable".to_owned())` — OsDetector failure produces a safe fallback string, no panic.

### A-5: No Command Injection Vectors
No `Command::new()`, no `std::process::Command`, no shell invocations. All input is from CLI args parsed via a simple iterator.

### A-6: Terminal Handling Safety
Lines 216-231: `ratatui::init()` enters alt-screen; `ratatui::restore()` runs unconditionally after the event loop. Logger is silenced (line 177) before TUI entry to prevent stderr corruption of the terminal buffer.

### C-1: `NO_COLOR` Not Checked
**Crate:** umrs-label
**Severity:** LOW
**Explanation:** The TUI mode uses ratatui's built-in color system. There is no `--color` flag and no check for the `NO_COLOR` environment variable. While the TUI itself is color-managed by ratatui, project rules state "Honor `NO_COLOR` environment variable unconditionally."
**Recommendation:** The coder should check `std::env::var("NO_COLOR")` and, if set, apply a monochrome theme. This applies to all three TUI crates but is lowest priority for umrs-label since it has no CLI color path.

---

## umrs-ls

### A-7: `#![forbid(unsafe_code)]` Present
Line 42 of `main.rs` and line 16 of `lib.rs`. Both crate roots carry the directive.

### A-8: OsDetector Integration Correct
Lines 414-425 of `main.rs`. Identical pattern to umrs-label. Validated newtypes, fail-closed to `"unavailable"`.

### A-9: SeColorConfig Fix Verified
Line 78: import changed from a hardcoded path function to `load_default as load_secolor_default`. Line 172: `load_secolor_default().ok()` — the function dynamically queries the active policy name via `selinux_policy()`, which is gated behind `is_selinux_enabled()` (kernel `/sys/fs/selinux` statfs check). The Trust Gate Rule is satisfied. No hardcoded `"targeted"` remains in umrs-ls.

### A-10: No Direct System State Reads (Except Justified Exception)
All direct `std::fs` calls in `main.rs`:
- Line 376: `std::fs::canonicalize(target)` — user-supplied path, not system state.
- Line 694: `std::fs::canonicalize(&resolved)` — user-typed goto-bar path.
- Line 803: `std::fs::read_dir(&parent_path)` — tab completion on user-directed paths (see C-2).

No reads on `/etc/`, `/proc/`, `/sys/`, or `/run/` from within umrs-ls itself.

### A-11: Terminal Handling Safety
Line 433: `ratatui::init()`. Line 602: `ratatui::restore()` runs unconditionally after the event loop exits (any cause). Logger silenced at line 373 before TUI entry.

### A-12: No Command Injection or Path Traversal in Navigation
`handle_enter()` (line 619) constructs paths via `current_path().join(name)` where `name` comes from tree metadata populated by `SecureDirent`. The `navigate_to()` call in `DirViewerApp` uses `list_directory()` which is fd-anchored. Navigation errors are shown via a modal overlay, not propagated as panics.

### A-13: Goto Bar Path Resolution
`resolve_goto_path()` (line 731) handles `~` expansion and relative paths. `canonicalize()` at line 694 resolves symlinks. If the target is a file, `parent()` is used (line 705). All paths are validated before navigation. No path injection possible — the path is used only for `list_directory` (read-only fd-anchored scan) or `canonicalize` (kernel-level resolution).

### A-14: JSON Output Does Not Leak Sensitive State
`emit_json()` (line 946) serializes only directory listing metadata (names, sizes, SELinux types, markings). No system state, no raw xattr values, no error details in the JSON output.

### C-2: DIRECT-IO-EXCEPTION Review — Tab Completion
**Crate:** umrs-ls
**Severity:** ACCURATE (exception is properly justified)
**Explanation:** The `complete_goto_query()` function (line 773) uses `std::fs::read_dir()` directly. The exception comment (lines 761-772) correctly states:
1. The System State Read Prohibition covers `/etc/`, `/proc/`, `/sys/`, `/run/` — not user-directed arbitrary paths.
2. The operation is UX-only: no security decision, no audit surface.
The exception is properly documented with a `DIRECT-IO-EXCEPTION` tag and review date.
**Assessment:** Exception is justified. No action required.

### C-3: `NO_COLOR` Not Checked in CLI Color Path
**Crate:** umrs-ls
**Severity:** LOW
**Explanation:** Line 261: `let use_color = args.contains(&"--color".to_owned())`. Color is opt-in (good), but when `--color` is passed, the `NO_COLOR` env var is not consulted. Per the project TUI/CLI rules and the `NO_COLOR` standard, if `NO_COLOR` is set, color should be suppressed regardless of `--color`.
**Recommendation:** The coder should add: `let use_color = args.contains(&"--color".to_owned()) && std::env::var_os("NO_COLOR").is_none();`

### C-4: `std::fs::canonicalize` Follows Symlinks Without Restriction
**Crate:** umrs-ls
**Severity:** LOW
**Explanation:** Lines 376 and 694 use `std::fs::canonicalize()` which follows symlinks. In the TUI context (interactive user navigation), this is standard behavior and not a security concern. However, in a hardened deployment, an attacker with write access to a symlink target could redirect the TUI to display a different directory than the operator intended. This is defense-in-depth only — the listing itself is read-only and the operator sees the canonical path in the header.
**Recommendation:** Document this behavior. No code change required at this time. If the tool is ever used in automated (non-interactive) pipelines, re-evaluate symlink following policy.

---

## umrs-stat

### A-15: `#![forbid(unsafe_code)]` Present
Line 5 of `main.rs` and in `[lints.rust]` table of `Cargo.toml` (`unsafe_code = "forbid"`). Both paths enforce the guarantee.

### A-16: OsDetector Integration Correct
Lines 791-808 of `main.rs`. Same validated newtype pattern as the other crates. Falls back to `"unavailable"`.

### A-17: `/proc/mounts` Read via Provenance-Verified Path
Lines 150-151: `ProcfsText::new(PathBuf::from("/proc/mounts"))` followed by `SecureReader::new().read_generic_text(&node)`. This satisfies the System State Read Prohibition — `/proc/mounts` is read through the provenance-verified `ProcfsText` engine, not raw `File::open`.

### A-18: ELF Header Read Properly Scoped
Lines 201-232: `read_elf_info()` uses `std::fs::File::open(path)` on the user-supplied target path. The comment (lines 197-200) correctly states this is display-only, not a trust-relevant assertion. The function reads exactly 20 bytes. No security decision depends on this result.

### A-19: Symlink Target Read is Display-Only
Line 270: `std::fs::read_link(path)` on the user-supplied path. Used only to display the symlink target string. Falls back to `"(unreadable)"` on error. Not used in any security decision.

### A-20: No Panics, No Unwraps
`#![deny(clippy::unwrap_used)]` at line 8. Grep confirms zero `.unwrap()` calls. All error paths produce user-facing messages via the audit card error rows or `eprintln!`.

### A-21: Error Messages Do Not Leak Sensitive State
Line 739: `"error: path contains non-UTF-8 characters and cannot be displayed"` — no path content leaked. The `SecDirError` displayed in the Security tab (line 551) contains the error kind and the path the operator already supplied. No internal system state is exposed.

### C-5: `Box::leak` for `&'static str`
**Crate:** umrs-stat
**Severity:** LOW
**Explanation:** Lines 604 and 624 use `Box::leak()` to produce a `&'static str` required by the `AuditCardApp` trait. This leaks memory (never freed). In a short-lived binary that runs once per invocation, this is acceptable. However, if `FileStatApp` is ever used in a long-running context (daemon, repeated invocations in the same process), this becomes a memory leak.
**Recommendation:** Document the one-shot usage constraint in the `FileStatApp` doc comment. If the trait signature can be changed to accept `&str` with a lifetime parameter, that would be the clean fix, but this is a design decision for the coder, not a security finding.

### C-6: `NO_COLOR` Not Checked
**Crate:** umrs-stat
**Severity:** LOW
**Explanation:** umrs-stat is TUI-only (ratatui). Same concern as C-1. The `Theme::default()` applies colors without consulting `NO_COLOR`.
**Recommendation:** Same as C-1.

---

## Dependency Review

### umrs-label Cargo.toml
- `umrs-platform` present (line 19). Required for OsDetector. Correct.
- Dependencies are appropriate for a TUI catalog browser. No unnecessary crates.
- `nix` with `user` and `fs` features — used in the library for UID/GID resolution and secure dirent operations. Correct.

### umrs-ls Cargo.toml
- `umrs-platform` present (line 19). Required for OsDetector. Correct.
- `gettext-rs` with `gettext-system` feature — links to system gettext. This is correct for i18n but means the binary depends on a system C library. Acceptable for RHEL 10 deployment.
- No unnecessary dependencies observed.

### umrs-stat Cargo.toml
- `umrs-platform` present (line 29). Required for OsDetector. Correct.
- `tree_magic_mini` — used for MIME detection (display-only). Acceptable.
- `systemd-journal-logger` — journal logging instead of stderr. Correct for TUI that owns stderr.
- `rustix` with `system` feature — present but no usage found in `main.rs`. This may be a leftover dependency.
- No unnecessary dependencies beyond the possible `rustix` leftover.

---

## Strengths Worth Preserving

1. **Consistent OsDetector pattern across all three crates.** The same fail-closed, newtype-validated pattern appears identically in all three binaries. This makes the pattern auditable and reduces the risk of a crate-specific regression.

2. **Logger silencing before TUI entry.** All three crates silence `log::set_max_level(Off)` before entering the ratatui alt-screen. This prevents stderr corruption and eliminates a class of information leakage through log messages during interactive use.

3. **SeColorConfig dynamic policy detection.** The replacement of the hardcoded `/etc/selinux/targeted/secolor.conf` path with `load_default()` (which queries the kernel for the active policy name) is the correct architecture. It satisfies the Trust Gate Rule and will work correctly under MLS policy without modification.

4. **`#![forbid(unsafe_code)]` on all crate roots.** Every binary and library crate root carries the `forbid` directive. This is a compile-time proof, not a convention.

5. **`#![deny(clippy::unwrap_used)]` on all binary crates.** Combined with zero `.unwrap()` calls confirmed by grep, this eliminates a class of unexpected panics in user-facing tools.

6. **DIRECT-IO-EXCEPTION documentation pattern.** The tab completion exception in umrs-ls (lines 761-772) is a model for how to document justified exceptions to project security rules: states what rule applies, why the exception is valid, and carries a review date.

7. **Modal error overlays instead of panics.** Navigation failures in umrs-ls produce a dismissible overlay (line 655). umrs-stat shows error rows in the audit card (line 617). Neither crate crashes on I/O failure.

8. **`/proc/mounts` read via `ProcfsText` in umrs-stat.** The filesystem info helper correctly uses the provenance-verified read path, not raw `File::open`. This is exactly the pattern the project requires.

---

## Gap Analysis

Files reviewed: 6 source files + 3 Cargo.toml files
Total findings: 27 (21 ACCURATE, 6 CONCERN, 0 ERROR)

### Policy artifacts written
None required. These are user-space TUI tools with no privilege elevation, no daemon mode, and no file mutation. SELinux policy for these binaries will be authored during M3.5 (deployment architecture) when `umrs_exec_t` and process domain transitions are defined.

### Documentation gaps
- `NO_COLOR` behavior is undocumented for all three crates. When the coder adds the check, the tech-writer should document the behavior in each tool's usage documentation.
- The `DIRECT-IO-EXCEPTION` pattern in umrs-ls should be referenced in the developer guide as a template for future exceptions.

### Code-vs-policy inconsistencies
- None. All three crates correctly route system state reads through validated pipelines. The SeColorConfig fix eliminates the last hardcoded policy-name assumption.

### Remediation Owner Summary

| ID | Severity | Owner | Description |
|---|---|---|---|
| C-1 | LOW | coder | umrs-label: check `NO_COLOR` env var |
| C-3 | LOW | coder | umrs-ls: check `NO_COLOR` env var when `--color` is passed |
| C-4 | LOW | coder | umrs-ls: document symlink-following behavior of canonicalize |
| C-5 | LOW | coder | umrs-stat: document one-shot constraint on `Box::leak` usage |
| C-6 | LOW | coder | umrs-stat: check `NO_COLOR` env var |
| — | LOW | tech-writer | Document `NO_COLOR` behavior across all three tools |

No HIGH or MEDIUM findings. All concerns are LOW severity.
