Audit date: 2026-04-09
Depth: in-depth
Scope: umrs-ui, umrs-ls, umrs-platform, umrs-stat, umrs-label — five crate security posture review

---

# Five-Crate Security Code Review

## Summary

| Category | Count |
|---|---|
| ACCURATE | 18 |
| CONCERN  | 12 |
| ERROR    |  3 |

---

## ERROR Findings

### E-1: `umrs-platform` posture modules use `std::fs` on system paths without DIRECT-IO-EXCEPTION

**Files:**
- `libs/umrs-platform/src/posture/configured.rs` (lines 181, 219)
- `libs/umrs-platform/src/posture/modprobe.rs` (lines 332, 372)
- `libs/umrs-platform/src/posture/bootcmdline.rs` (lines 121, 217, 295)
- `libs/umrs-platform/src/posture/fips_cross.rs` (line 276)

**Severity:** MEDIUM

These modules read from `/etc/sysctl.d/`, `/etc/modprobe.d/`, `/etc/crypto-policies/state/current`, `/loader/entries/`, and `/proc/sys/kernel/osrelease` using raw `std::fs::read_to_string` and `std::fs::read_dir`. These are system paths (`/etc/`, `/proc/`) covered by the System State Read Prohibition Rule.

The posture module lives inside `umrs-platform` itself, so the question is whether these reads should route through `SecureReader` (the module's own engine) or whether being inside the platform crate constitutes an implicit exception. Currently, no `DIRECT-IO-EXCEPTION` comment is present on any of these call sites. The `kattrs` reader module, by contrast, uses `SecureReader` for all `/proc/` and `/sys/` reads.

For sysctl.d/modprobe.d files under `/etc/`, these are configuration files that influence posture assessment (a security-relevant decision: whether a sysctl meets the hardened baseline). Reading them without provenance verification means a tampered filesystem could feed false configuration data to the contradiction detector.

**Recommended fix:** Either:
1. Route `/proc/sys/kernel/osrelease` through `ProcfsText` + `SecureReader` (consistent with the rest of the module), or
2. Add explicit `DIRECT-IO-EXCEPTION` comments at each call site with justification. For `/etc/sysctl.d/` and `/etc/modprobe.d/`, the justification must address why configuration reads that feed contradiction detection are not trust-relevant.

**Remediation owner:** coder

---

### E-2: `umrs-ls` duplicates `build_marking_detail` from `umrs-label`

**File:** `umrs-ls/src/main.rs` (lines 305-383)

**Severity:** LOW

`umrs-ls/src/main.rs` contains a `build_marking_detail` function that is a near-verbatim copy of `umrs-label/src/tui/app.rs::marking_to_detail` (lines 447-522). The only difference is that umrs-ls suppresses the `level` field (sets it to `String::new()`). Both functions:
- Parse handling from string or locale object
- Build the same `additional` fields vector
- Construct the same `MarkingDetailData` struct

This duplication means a catalog schema change or new field must be updated in two places. The `umrs-label` crate already exports `marking_detail_us` and `marking_detail_ca` on `LabelRegistryApp`, and `umrs-ls` already depends on `umrs-label`.

**Recommended fix:** Make `marking_to_detail` public in `umrs-label` (or add a standalone `pub fn marking_to_detail(key, marking, flag) -> MarkingDetailData` in the `umrs_labels::tui::app` module), and call it from `umrs-ls` instead of maintaining a copy. If the `level` suppression is needed, add a builder method or field toggle on `MarkingDetailData`.

**Remediation owner:** coder

---

### E-3: `umrs-label` Cargo.toml declares four unused dependencies

**File:** `umrs-label/Cargo.toml` (lines 21-30)

**Severity:** LOW

The following dependencies are declared but never imported in any source file under `umrs-label/src/`:
- `anyhow = "1.0"` — zero imports
- `clap = { version = "4.5", features = ["derive"] }` — zero imports (manual arg parsing used instead)
- `chrono = "0.4"` — zero imports
- `nix = { version = "0.27", features = ["user", "fs"] }` — zero imports

Phantom dependencies increase the compiled binary size, expand the supply chain attack surface, and slow compilation. `clap` with `derive` pulls in `syn` and proc-macro infrastructure. `nix` links against libc.

**Recommended fix:** Remove all four from `[dependencies]`. If any are planned for future use, comment them out with a `# TODO:` note.

**Remediation owner:** coder

---

## CONCERN Findings

### C-1: `umrs-ls` and `umrs-label` Cargo.toml missing `pedantic`/`nursery` lint configuration

**Files:** `umrs-ls/Cargo.toml`, `umrs-label/Cargo.toml`

Both crates enable `clippy::pedantic` and `clippy::nursery` via `#![warn(...)]` in their source files (lib.rs and main.rs), but the `[lints.clippy]` section in their Cargo.toml files does not declare `pedantic` or `nursery`. The other three crates (`umrs-ui`, `umrs-stat`, `umrs-platform`) all declare them in Cargo.toml.

**Recommendation:** Add `pedantic = { level = "warn", priority = -1 }` and `nursery = { level = "warn", priority = -1 }` to both Cargo.toml `[lints.clippy]` sections for consistency. The source-level attributes still work, but the Cargo.toml declaration is the canonical location for the `xtask clippy` pipeline.

**Remediation owner:** coder

---

### C-2: `NO_COLOR` check result unused in `umrs-stat` and `umrs-label`

**Files:**
- `umrs-stat/src/main.rs` line 183: `let _no_color = std::env::var("NO_COLOR").is_ok();`
- `umrs-label/src/main.rs` line 245: `let _no_color = std::env::var("NO_COLOR").is_ok();`

Both binaries read `NO_COLOR` but discard the result into `_no_color`. The TODO comment in `umrs-label` mentions a `Theme::no_color()` variant is needed. Until that variant exists, these tools emit colored output even when `NO_COLOR` is set, violating the `no-color.org` specification and the TUI/CLI Design Principles rule ("Honor NO_COLOR environment variable unconditionally").

`umrs-ls` handles it correctly in CLI mode (line 464), gating color on both `--color` flag and `NO_COLOR` absence.

**Recommendation:** Implement `Theme::no_color()` or `Theme::mono()` in `umrs-ui` and wire it into both binaries. This is a compliance gap for the TUI/CLI rules.

**Remediation owner:** coder

---

### C-3: OS name resolution pattern duplicated across three binaries

**Files:**
- `umrs-stat/src/main.rs` lines 185-194
- `umrs-label/src/main.rs` lines 251-262
- `umrs-ls/src/main.rs` lines 619-630

Each binary independently calls `OsDetector::default().detect()`, extracts `os_release`, formats `name + version_id`, and falls back to `"unavailable"`. The logic is identical across all three. This should be a helper in `umrs-ui::indicators` (where `build_header_context` already lives) or a method on `OsDetector`.

**Recommendation:** Add a `pub fn detect_os_name() -> String` helper to `umrs-ui::indicators` that encapsulates the detect-extract-format-fallback pattern. All three binaries call it instead of duplicating the chain.

**Remediation owner:** coder

---

### C-4: `rustix` version split across workspace (0.38 vs 1.x)

**Files:**
- `libs/umrs-platform/Cargo.toml`: `rustix = "0.38"`
- `libs/umrs-selinux/Cargo.toml`: `rustix = "0.38"`
- `libs/umrs-hw/Cargo.toml`: `rustix = "0.38"`
- `umrs-c2pa/Cargo.toml`: `rustix = "0.38"`
- `libs/umrs-ui/Cargo.toml`: `rustix = "1"`
- `umrs-stat/Cargo.toml`: `rustix = "1"`

Two different major versions of `rustix` are compiled into the same workspace. This means two copies of the syscall wrapper layer are linked, increasing binary size and complicating supply chain auditing. The comment in `umrs-platform`'s Cargo.toml says "Same version already present in umrs-selinux" but `umrs-ui` and `umrs-stat` use `1.x`.

**Recommendation:** Align the workspace to a single `rustix` version. If `1.x` has the needed features, migrate all crates. If `0.38` is required for API reasons in `umrs-platform`, document the reason and pin both versions in the workspace `[dependencies]` table.

**Remediation owner:** coder

---

### C-5: `umrs-label` catalog loading uses `std::fs::File::open` on user data paths

**File:** `umrs-label/src/cui/catalog.rs` line 167

`load_catalog` opens JSON catalog files via `std::fs::File::open`. These are user data files (not system paths like `/etc/`, `/proc/`, `/sys/`), so the System State Read Prohibition Rule does not strictly apply. However, catalog files are the authoritative source of CUI label definitions (NIST SP 800-53 AC-16). A tampered catalog could show the operator incorrect marking information, leading to mislabeling.

**Recommendation:** Consider adding a catalog integrity check (e.g., SHA-256 digest verification against a known-good manifest) in a future phase. For now, document this as an accepted risk with a comment at the `File::open` call site.

**Remediation owner:** security-engineer (document) / coder (future integrity check)

---

### C-6: `umrs-stat` `read_elf_info` uses `std::fs::File::open` on user file — DIRECT-IO-EXCEPTION present but could be tighter

**File:** `umrs-stat/src/lib.rs` lines 222-251

The `DIRECT-IO-EXCEPTION` comment is present and justified: ELF magic read is display-only, no trust decision. However, the function opens the file by path (`std::fs::File::open(path)`) without fd anchoring. If the file is replaced between the `SecureDirent::from_path` call and the `read_elf_info` call, the ELF bytes could come from a different file (TOCTOU). Since this is display-only and explicitly not trust-relevant, this is acceptable — but the TOCTOU window should be documented in the exception comment.

**Recommendation:** Append to the DIRECT-IO-EXCEPTION comment: "TOCTOU: the file may have been replaced since SecureDirent construction; the ELF bytes are display hints, not assertions."

**Remediation owner:** coder

---

### C-7: `umrs-stat` `build_identity_rows` uses `std::fs::read_link` without DIRECT-IO-EXCEPTION

**File:** `umrs-stat/src/lib.rs` line 292

`std::fs::read_link(path)` is used to display the symlink target. This is a direct filesystem call on an arbitrary user path. It is not a system path, so the prohibition is not strictly triggered, but this is analogous to `read_elf_info` — display-only data that could be stale due to TOCTOU. No DIRECT-IO-EXCEPTION comment is present.

**Recommendation:** Add a `// DIRECT-IO-EXCEPTION:` comment documenting that this is display-only symlink target resolution.

**Remediation owner:** coder

---

### C-8: `extract_selinux_short` and `selinux_from_path` near-duplicate in `umrs-ls/viewer_app.rs`

**File:** `umrs-ls/src/viewer_app.rs` lines 442-491

`extract_selinux_short` (from a `SecureDirent`) and `selinux_from_path` (from a `Path`) both perform the same marking translation logic: get the level, construct a `SecurityRange`, query `GLOBAL_TRANSLATOR`, fall back to raw level string. The only difference is the input type. This pattern also appears in `umrs-stat/src/lib.rs` lines 376-382 (`build_security_rows` marking extraction).

**Recommendation:** Extract a shared `fn translate_marking(level: &MlsLevel) -> String` helper into `umrs-selinux` or `umrs-ui` that encapsulates the GLOBAL_TRANSLATOR lookup + fallback. All three call sites call it.

**Remediation owner:** coder

---

### C-9: `umrs-ls` `complete_goto_query` expands `~` via `std::env::var("HOME")`

**File:** `umrs-ls/src/main.rs` lines 1238-1244

The goto bar's tab completion resolves `~` by reading `HOME` from the environment. On a compromised or misconfigured system, `HOME` could point anywhere. This is UX-only (no security decision), and the DIRECT-IO-EXCEPTION is documented, but reading `HOME` without validation could lead the operator to navigate to an unexpected location.

**Recommendation:** Consider using `nix::unistd::User::from_uid(getuid()).home_dir` for a more robust home directory resolution. At minimum, document in the exception comment that `HOME` is user-controlled and not validated.

**Remediation owner:** coder

---

### C-10: `umrs-label` uses manual argument parsing instead of `clap`

**File:** `umrs-label/src/main.rs` lines 70-79

The binary uses `std::env::args().collect()` and manual string matching (`args.contains(...)`, `arg_value(...)`) for CLI arguments, despite declaring `clap` as a dependency (though unused — see E-3). This manual parsing:
- Does not validate argument combinations
- Does not provide `--help` or `--version` automatically
- Could accept malformed flags without error

`umrs-stat` uses `clap` correctly. `umrs-ls` also uses manual parsing but does not declare `clap`.

**Recommendation:** Either wire in `clap` (and remove the manual parsing) or remove the `clap` dependency (covered by E-3). If keeping manual parsing, add validation for unknown flags.

**Remediation owner:** coder

---

### C-11: `umrs-ui` depends on `systemd-journal-logger` but is a library crate

**File:** `libs/umrs-ui/Cargo.toml` line 21

`systemd-journal-logger` is a runtime logging backend. Library crates should not initialize or depend on specific logging backends — that is the binary's responsibility. The library should depend only on `log` for the logging facade. Binary crates that consume `umrs-ui` (umrs-stat, umrs-ls, umrs-label) all declare their own `systemd-journal-logger` dependency and initialize it in `main()`.

**Recommendation:** Remove `systemd-journal-logger` from `umrs-ui/Cargo.toml`. If any code in `umrs-ui` references it, refactor to use only the `log` facade.

**Remediation owner:** coder

---

### C-12: `umrs-label` uses `regex` crate — verify necessity

**File:** `umrs-label/Cargo.toml` line 24

`regex = "1"` is declared. This is a substantial dependency. If used only for simple pattern matching (e.g., CUI marking format validation), consider whether the validation could use `str` methods or a small purpose-built parser instead.

**Recommendation:** Audit `regex` usage in `umrs-label/src/validate.rs` — if the patterns are simple enough for `str::starts_with` / `str::contains` / manual parsing, remove the dependency. If regex is genuinely needed for CUI banner syntax validation, document why in Cargo.toml.

**Remediation owner:** coder

---

## ACCURATE Findings

### A-1: `#![forbid(unsafe_code)]` present on all crate roots

All ten crate root files (lib.rs and main.rs for umrs-ui, umrs-ls, umrs-platform, umrs-stat, umrs-label) carry `#![forbid(unsafe_code)]`. Compile-time proof of safe-code guarantee is intact.

### A-2: `#![deny(clippy::unwrap_used)]` enforced across all crates

All crate roots deny `unwrap_used`. No `.unwrap()` calls found in any of the five crates' source files (excluding comments).

### A-3: `#[must_use]` with message strings on all security-relevant return types

`umrs-stat/src/lib.rs` carries `#[must_use]` with descriptive messages on all public functions and on `FileStatApp` methods. `umrs-ui/src/indicators.rs` does the same for `read_security_indicators` and `build_header_context`. Compliant with the Must-Use Contract Rule.

### A-4: Provenance-verified reads in `umrs-ui/src/indicators.rs`

All kernel attribute reads (`SelinuxEnforce::read`, `ProcFips::read`, `KernelLockdown::read`, `ProcfsText` + `SecureReader` for boot_id, `SysfsText` + `SecureReader` for system_uuid) use the typed `StaticSource` path with `SecureReader` engine. No raw `File::open` on `/proc/` or `/sys/`. NSA RTB RAIN and NIST SP 800-53 SI-7 compliance confirmed.

### A-5: Trust Gate pattern applied for SELinux policy name

`umrs-ui/src/indicators.rs::read_selinux_status` reads the policy type via `selinux_policy()`, which is kernel-gated — it only reads `/etc/selinux/config` when the kernel confirms SELinux is active. Compliant with NIST SP 800-53 CM-6.

### A-6: Fail-closed error handling throughout

All indicator reads fall back to `IndicatorValue::Unavailable` on error. `find_fs_info` returns `None`. `read_elf_info` returns `None`. `DirMeta::from_path` falls back to `DirMeta::placeholder()`. No silent success on error paths.

### A-7: `umrs-stat` `find_fs_info` reads `/proc/mounts` via `ProcfsText` + `SecureReader`

Line 164-165: correctly uses `ProcfsText::new(PathBuf::from("/proc/mounts"))` and `SecureReader::new().read_generic_text(&node)`. Compliant with the prohibition on raw `/proc/` reads.

### A-8: `umrs-ls` directory listing routes through `SecureDirent` / `list_directory`

All directory content reads go through `umrs_selinux::utils::dirlist::list_directory` which uses fd-anchored `SecureDirent` construction. TOCTOU-safe.

### A-9: `umrs-ls` `complete_goto_query` DIRECT-IO-EXCEPTION properly documented

Lines 1207-1218: the exception is documented with clear justification (user-directed paths, not system state, UX-only). The exception criteria are met.

### A-10: `umrs-stat` `read_elf_info` DIRECT-IO-EXCEPTION properly documented

Lines 217-220: exception is documented. Display-only, no trust decision, no platform abstraction exists.

### A-11: `umrs-ls` `DirMeta::from_metadata_fallback` DIRECT-IO-EXCEPTION documented

Line 112: fallback for paths rejected by `ValidatedFileName` (e.g., `/`). Uses `get_file_context` (fd-anchored, TPI-validated) for SELinux context. `symlink_metadata` is the only raw call, and it is display-only header metadata.

### A-12: Read-only contracts enforced

All three TUI tools are read-only browsers. No mutation methods exist on `LabelRegistryApp`, `DirViewerApp`, or `FileStatApp`. `ViewerApp` trait provides no write operations. NIST SP 800-53 AC-3 confirmed.

### A-13: `systemd-journal-logger` present in all binary crates

`umrs-ls`, `umrs-stat`, `umrs-label` all declare `systemd-journal-logger` in their Cargo.toml and initialize it in `main()`. Audit logging infrastructure is present.

### A-14: Module-level `//!` doc blocks with compliance sections

All reviewed modules have `//!` documentation with `## Compliance` sections citing NIST SP 800-53 controls. `umrs-stat/src/lib.rs`, `umrs-label/src/lib.rs`, `umrs-ls/src/lib.rs`, and `umrs-ui/src/lib.rs` all comply.

### A-15: Security observations represented as typed data

`umrs-stat` renders observations from `SecurityObservation` enum variants with `ObservationKind` discrimination (Risk/Warning/Good). Compliant with the Security Findings as Data Rule.

### A-16: `build_header_context` includes pattern execution measurement

Debug-build timing is logged for both `read_security_indicators` and `build_header_context`, with pattern name and microsecond precision. Compliant with Pattern Execution Measurement Rule.

### A-17: `umrs-ui` library crate correctly abstracts shared TUI infrastructure

The three layout patterns (AuditCardApp, ViewerApp, ConfigApp) are cleanly separated. Shared popup rendering, theme, keymap, and indicator reading are centralized. This is the correct architectural boundary.

### A-18: `umrs-core/src/fs/mod.rs` correctly gated as NOT WIRED IN

The module with raw `std::fs` calls on `/proc/mounts` and `/sys/class/block/` is explicitly documented as not declared in `lib.rs` and not active. The architectural warning is clear and correct.

---

## Gap Analysis Summary

```
Files reviewed: 35+ source files across 5 crates
Total findings: 33 (18 ACCURATE, 12 CONCERN, 3 ERROR)
Policy artifacts written: none (no SELinux policy relevant to this review scope)
Policy artifacts needed: none
Documentation gaps: NO_COLOR compliance (C-2), TOCTOU documentation (C-6, C-7)
Code-vs-policy inconsistencies: posture module direct I/O without DIRECT-IO-EXCEPTION (E-1)
```

### Remediation Priority

| Priority | ID | Owner | Description |
|---|---|---|---|
| 1 | E-1 | coder | Posture module direct I/O: add DIRECT-IO-EXCEPTION or route through SecureReader |
| 2 | E-3 | coder | Remove phantom deps from umrs-label |
| 3 | C-2 | coder | Implement NO_COLOR Theme variant |
| 4 | C-4 | coder | Align rustix version across workspace |
| 5 | C-11 | coder | Remove systemd-journal-logger from umrs-ui library |
| 6 | E-2 | coder | Deduplicate build_marking_detail |
| 7 | C-3 | coder | Extract OS name detection helper |
| 8 | C-8 | coder | Extract marking translation helper |
| 9 | C-1 | coder | Add pedantic/nursery to Cargo.toml lint tables |
| 10 | C-10 | coder | Wire clap or validate manual args in umrs-label |
| 11 | C-6 | coder | Document TOCTOU in read_elf_info exception |
| 12 | C-7 | coder | Add DIRECT-IO-EXCEPTION to read_link call |
| 13 | C-9 | coder | Document HOME expansion risk |
| 14 | C-5 | security-engineer | Document catalog integrity gap |
| 15 | C-12 | coder | Audit regex necessity in umrs-label |

---

## Strengths Worth Preserving

1. **Provenance-verified read discipline is well established.** The `indicators.rs` module is a model of correct kernel attribute access — every read goes through `SecureReader` with typed sources. This pattern should be the template for any new indicator.

2. **DIRECT-IO-EXCEPTION documentation is thorough where present.** The `umrs-ls` goto completion and `umrs-stat` ELF reader exceptions are well-justified with clear scope boundaries. This is what good exception documentation looks like.

3. **Fail-closed error handling is consistent.** Every read operation returns a typed degraded-state marker on failure. No silent success paths were found.

4. **The `umrs-ui` library abstraction is architecturally sound.** The three-pattern template system (AuditCard, Viewer, Config) with shared popup/theme/keymap infrastructure prevents each binary from reinventing the rendering wheel. The code duplication findings (C-3, C-8, E-2) are opportunities to push more shared logic into this library.

5. **The `#![forbid(unsafe_code)]` + `#![deny(clippy::unwrap_used)]` baseline is enforced everywhere.** This is the strongest compile-time guarantee available and it holds across all five crates without exception.

6. **Module documentation with compliance citations is complete.** Every module has a `//!` block with a `## Compliance` section. This is rare in practice and valuable for auditors.
