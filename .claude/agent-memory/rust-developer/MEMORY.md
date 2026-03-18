# Rust Developer Agent Memory

## Topic Files (linked memory)

- [Posture module history](posture_module_history.md) — Signal→Indicator rename, Phase 2b architecture, security review findings

## WARNING — Rogue .claude Directory

A `.claude/` directory at `components/rusty-gadgets/.claude/` was created by a prior agent
violating the Single .claude Directory Rule. It contains stale copies of memory files.
DO NOT READ OR WRITE to `components/rusty-gadgets/.claude/` — use the repo-root `.claude/` only.
The duplicate has been flagged to Jamie.

## Key Architectural Facts

- Primary workspace: `components/rusty-gadgets/` — production crates only
- Prototype workspace: `components/rust-prototypes/` — moved 2026-03-11; no xtask, use `cargo build/test` directly
- Primary crate: `umrs-selinux` (under `src/`)
- Platform crate: `umrs-platform` (detect pipeline, kattrs, evidence)
- HW crate: `umrs-hw` — ONLY crate WITHOUT `#![forbid(unsafe_code)]`; isolates RDTSCP asm
- All tests in `tests/`, never inline
- Run everything via `cargo xtask {fmt,clippy,test}` from `components/rusty-gadgets/`

## rust-prototypes workspace members (as of 2026-03-11)

- `cui-labels` — depends on `umrs-core` via `../../rusty-gadgets/umrs-core`
- `kernel-files` — no workspace deps
- `mcs-setrans` — depends on `umrs-selinux` via `../../rusty-gadgets/umrs-selinux`
- `vaultmgr` — depends on `umrs-core` via `../../rusty-gadgets/umrs-core`
- These were moved from rusty-gadgets; rustfmt.toml is an exact copy of rusty-gadgets/rustfmt.toml

## Toolchain

- Rust 1.92+ is the active toolchain on this system
- `is_multiple_of()` is stable in 1.92 — clippy enforces it over `% 2 == 0`
- `let ... && let ...` (let-chain guards) are stable in this toolchain

## System Prerequisites

- `sqlite-devel` must be installed on the build host (`sudo dnf install sqlite-devel`)
- Without it, `umrs-ls` and anything linking `umrs-platform` will fail with `-lsqlite3 not found`
- This is the RHEL package; Ubuntu equivalent is `libsqlite3-dev` (but target is RHEL)

## FFI Exception — rusqlite

`rusqlite` uses FFI (via `libsqlite3-sys`) to bind to C `libsqlite3`. This is a documented,
accepted exception to the "prefer pure Rust / avoid FFI" policy. Accepted by Jamie Adams
2026-03-11. Documented in `umrs-platform/Cargo.toml` and `rpm_db.rs` module doc.
Rationale: no pure-Rust SQLite handles RHEL 10 RPM DB reliably; system lib is RHEL-patched;
unsafe is fully encapsulated in rusqlite/libsqlite3-sys.

## umrs-platform detect/ Pipeline

Phase order: release_candidate → release_parse → pkg_substrate → file_ownership → integrity_check

Key types:
- `EvidenceBundle` — append-only; `records` field is PRIVATE (RPM-22 fix). Access via `.records()`, `.iter()`, `.push()`, `.len()`, `.is_empty()`
- `EvidenceRecord` — one per I/O event; `path_requested` = original path, `path_resolved` = symlink target if any
- `find_stat_for_path` — pub(super) in file_ownership.rs; searches bundle in reverse for (dev,ino) by path
- `find_resolved_path` — pub(super) in file_ownership.rs; searches bundle for symlink target by path_requested

## Symlink Path Resolution (RPM-16)

On RHEL 10, `/etc/os-release` is a symlink → `../usr/lib/os-release` (relative!).
The RPM DB records `/usr/lib/os-release` (absolute). Fix pattern:
1. `find_resolved_path(evidence, candidate_str)` returns the raw readlinkat result
2. If relative: join with candidate's parent dir, then `std::fs::canonicalize()`
3. Use canonicalized path for DB queries; keep original `candidate_str` in audit records
Both `file_ownership.rs` and `integrity_check.rs` import and apply this pattern.

## Dev Encoding Mismatch — FIXED

`release_candidate.rs` records `dev_combined = (stx_dev_major << 32) | stx_dev_minor`.
`statx` exposes major/minor as separate u32 fields; this encodes them as `(major as u64) << 32 | minor`.

`nix::sys::stat::stat()` and `rustix::fs::fstat()` return `st_dev` as the kernel's compact
`dev_t` encoding (e.g., device 253:0 → 64768, not `(253 << 32) | 0`).

Fix applied in two places:
- `rpm.rs::query_ownership_inner`: decompose with `nix::sys::stat::major/minor` (return `u64`),
  reassemble as `(st_major << 32) | st_minor`.
- `integrity_check.rs` fstat block: decompose with `rustix::fs::major/minor` (return `u32`),
  reassemble as `(u64::from(maj) << 32) | u64::from(min)`.

After fix: `os_detect` example shows `owner=centos-stream-release` and T4 digest verified.

## mcs_translator Test Fixture Path

Test fixture is at `umrs-selinux/data/setrans.conf` (moved from crate root in a prior session).
`tests/mcs_translator.rs` and `examples/color_demo.rs` both use `"data/setrans.conf"`.
All 5 mcs_translator tests pass as of 2026-03-11.

## EvidenceBundle IntoIterator

`IntoIterator for &EviduxBundle` is implemented (required by clippy `iter_without_into_iter`).
`for rec in bundle.iter()` and `for rec in &bundle` both work.

## Error Display Discipline (RPM-02)

rusqlite::Error Display can leak paths. Use:
`e.sqlite_error_code().map_or(-1_i32, |c| c as i32)` to extract only the error code.

## RPM-07: fail-closed on ArrayLengthMismatch

`RpmHeaderError::ArrayLengthMismatch { expected, dirindexes, digests }` added.
`extract_file_list` now returns this error instead of silently returning `Vec::new()`.

## SEC Pattern — Sealed Evidence Cache (implemented 2026-03-11)

- Module: `umrs-platform/src/sealed_cache.rs`
- Tests: `umrs-platform/tests/sealed_cache_tests.rs` (16 tests, all passing)
- Example: `umrs-platform/examples/sealed_cache_demo.rs`
- Public re-exports from `lib.rs`: `SealedCache`, `CacheStatus`, `DEFAULT_TTL_SECS`, `MAX_TTL_SECS`
- New deps: `hmac = "0.12"`, `zeroize = { version = "1", features = ["derive"] }`
- FIPS gate: read at construction via ProcfsText at `/proc/sys/crypto/fips_enabled`; caching disabled if FIPS active
- Seal covers: TrustLevel byte + SHA-256(EvidenceBundle) + boot_id + os_id + version_id + probe_used
- Key derivation: SHA-256(boot_id || 0x00 || starttime_ticks_le) — two entropy sources
  - boot_id: /proc/sys/kernel/random/boot_id (session binding)
  - starttime: /proc/self/stat field 22 (process invocation binding)
  - Both read via ProcfsText + SecureReader; fail-closed if either is unavailable (no fallback)
- FIPS gate: fail-CLOSED — returns true (disable caching) on ANY read failure, not just "1"
- Conservative cache hit: re-runs pipeline on verified hit (full deserialization deferred)
- `OsDetector::detect()` is NOT integrated — caller uses `SealedCache::query()` instead
- map_or pattern: `.map_or(b"" as &[u8], |x| ...)` is required when default is a byte literal

## umrs-selinux Import Paths (confirmed 2026-03-12)

- `observations` module is **private** — import via crate root re-exports:
  `umrs_selinux::ObservationKind`, `umrs_selinux::SecurityObservation`
- `SelinuxCtxState` is re-exported at `umrs_selinux::SelinuxCtxState`
- `fs_encrypt` and `posix` are `pub mod` — import directly
- `SecurityContext` accessors: `.user()`, `.role()`, `.security_type()`, `.level()` (returns `Option<&MlsLevel>`)
- `MlsLevel::raw()` returns `&str`; `.level()` returns `Option<&MlsLevel>`
- `FileSize` is `Copy` — pass by value, not `&FileSize`. Has `.as_u64()` accessor.
- `SecureDirent::access_denied` is a `pub bool` field (not inside `SelinuxCtxState`)

## AuditCardApp `report_subject()` Pattern

`report_subject()` returns `&'static str`. To use a runtime string:
`let s: &'static str = Box::leak(runtime_string.into_boxed_str());`
Store `s` in the struct. One-time allocation per binary run — acceptable.

## tree_magic_mini Supply Chain Note

`tree_magic_mini = "3"` added to `umrs-tui` only.
- Pure-Rust MIME detection via magic bytes + file path
- No network access; display-only (not trust-relevant, not policy)
- Path-based API (`from_filepath`), not fd-based — documented in source comment

## umrs-tui Binaries (as of 2026-03-12)

- `umrs-os-detect-tui` — `src/main.rs`
- `umrs-file-stat` — `src/bin/file_stat.rs`
  Tabs: Identity / Security / Observations
  Deps: `umrs-selinux`, `umrs-platform`, `tree_magic_mini`
  Identity tab: path, type, symlink target, MIME, ELF info, size, mode, inode, device,
    hard links (yellow if >1 non-dir), owner, group, mount point, filesystem info from /proc/mounts

## /proc/mounts Lookup Pattern (file_stat.rs `find_fs_info`)

- Use `ProcfsText::new(PathBuf::from("/proc/mounts")).ok()?` + `SecureReader::<ProcfsText>::new().read_generic_text(&node).ok()?`
- Walk lines, split_whitespace for (device, mount_point, fs_type)
- Longest-prefix match on `path.to_str()?.starts_with(mp)` — tracks `best_len`
- Returns `Option<FsInfo>` — caller shows rows only if Some

## ELF Header Read Pattern (file_stat.rs `read_elf_info`)

- `std::fs::File::open(path).ok()?` + `f.read_exact(&mut [0u8; 20]).ok()?`
- Display-only — same policy as tree_magic_mini; document "not trust-relevant"
- EI_CLASS: byte 4 (1=ELF32, 2=ELF64)
- e_type: bytes 16-17 little-endian `u16::from_le_bytes([buf[16], buf[17]])`
- Returns `Option<ElfInfo { class: &'static str, elf_type: &'static str }>`

## umrs-tui Architecture (implemented 2026-03-12)

- Library crate `umrs_tui` (lib.rs) + binary `umrs-tui` (main.rs)
- Modules: `app`, `theme`, `keymap`, `layout`, `header`, `tabs`, `data_panel`, `status_bar`
- Entry trait: `AuditCardApp` (object-safe). `report_name` / `report_subject` return `&'static str`.
- State: `AuditCardState::new(tab_count)` — owns active_tab, scroll_offset, should_quit
- Render: `render_audit_card(frame, f.area(), &dyn AuditCardApp, &state, &theme)`
- Layout: Vertical[header(WIZARD_SMALL.height+2), 1 tab, Min(0) data, 1 status]. Header row splits [Min(0) text | Exact(WIZARD_SMALL.width+2) logo].
- Main binary: `OsDetectApp` — Tab 0 = OS info, Tab 1 = Trust/Evidence
- Example: `examples/show_logo.rs` — robot gallery (guest-coder API entry point)
- Deps added: `crossterm = "0.28"`, `rustix = { version = "1", features = ["system"] }`, `systemd-journal-logger = "2"`
- Journald init: best-effort `JournalLog::new()?.install()`; TUI never writes to stderr

## Clippy Patterns Confirmed (2026-03-12)

- `missing_const_for_fn`: pure match-on-enum fns must be `const fn`
- `unnecessary_literal_bound`: trait methods returning literal `&str` need `&'static str`
- `cast_possible_truncation` on small known-safe `usize as u16`: use `#[allow(...)]` with comment explaining the value is always in range
- `#[allow(...)]` in layout.rs for WIZARD_SMALL dimension casts (width=15, height=7)

## Outstanding Audit Findings

Reports in `.claude/reports/` with findings assigned to "coder":
- `2026-03-11-rpm-db-security-audit.md`: coder-assigned RPM-01, -02, -04, -06, -07, -11, -12, -15, -16, -19, -20, -22, -27, -28
- `2026-03-11-os-detection-umrs-platform-surface-audit.md`: multiple HIGH/MEDIUM items
- `sec-audit-2026-03-11.md`: SEC-01, SEC-02 (HIGH) resolved. SEC-04, SEC-06, SEC-08 (LOW) resolved.
  Remaining coder items: SEC-05 (evidence digest coverage), SEC-07 (tamper test), SEC-09 (Cargo.toml citation), SEC-10 (lib.rs citations).

## Clippy Style Preferences

- Use explicit `match` over `.map().unwrap_or_else()` — clearer and avoids lint
- Use `let x && let y` chains for if-let guards
- `is_multiple_of(2)` is preferred over `% 2 == 0` by clippy 1.92
- `match` with one Some/Ok arm and one None/Err arm fires `single_match_else` — use `if let ... else` or `let Some(...) = ... else { return ... }` (let-else)
- `let-else` form: `let Some(x) = expr else { return None; };` — preferred for early-return error paths
- Test helper functions that can be `const fn` will trigger `missing_const_for_fn` — make them const, never suppress

## umrs-tui Test Suite (added 2026-03-12)

4 integration test files under `umrs-tui/tests/` — 92 tests total, all passing:
- `audit_card_state_tests.rs` (27) — `AuditCardState` + `Action` state machine
- `keymap_tests.rs` (18) — `KeyMap` default bindings and custom bind
- `data_types_tests.rs` (18) — `DataRow`, `TabDef`, `StatusMessage`, `StyleHint`, `StatusLevel`, `tabs_from_labels`
- `theme_tests.rs` (11) — `status_bg_color` and `style_hint_color` const color mapping
- `trait_impl_tests.rs` (18) — mock `AuditCardApp` impl; object safety verified

Key test patterns:
- `AuditCardState::handle_action` takes `&Action` (reference)
- `keymap_tests.rs` `key()` helper must be `const fn` to pass clippy
- Mock impl fails-closed on invalid tab index (returns empty Vec, no panic)
- Object safety test: assign `&app` to `&dyn AuditCardApp` — compile failure = not object-safe

## i18n Integration in umrs-tui (added 2026-03-12)

- `i18n::init("umrs-tui")` — first line of `main()` in BOTH binaries, before logging setup
- `i18n::tr("msgid")` returns `String` — compatible with `impl Into<String>` in DataRow::new
- Library header labels ("Report", "Host", "Subject") wrapped with `i18n::tr()` in `header.rs`
- `card_title()` default method added to `AuditCardApp` — returns `String`, not `&'static str`
- Both binaries override `card_title()` to return `i18n::tr("...")` for their specific title
- `report_name()` and `report_subject()` intentionally stay `&'static str` — do NOT change
- Header title construction: `let title = format!(" {} ", app.card_title());`
- Header field label padding: `format!("{:<8} : ", i18n::tr("Report"))` — 8-char left-pad

## Clippy too_many_lines (100-line limit)

- Adding `i18n::tr()` wrappers can push a function over 100 lines
- Fix: extract logical blocks into helper functions, NOT suppress with `#[allow]`
- Example: `build_inode_flag_rows(dirent)` extracted from `build_security_rows()` in file_stat.rs

## posture Module — see posture_module.md (Phase 1+2a+2b, 37 signals, implemented 2026-03-16)

Key: 37 signals, `PostureSnapshot::collect()`, contradiction detection, FipsCrossCheck.
Tests: posture_tests.rs (79+), posture_modprobe_tests.rs (50), posture_bootcmdline_tests.rs (10)

## timestamp Module — see timestamp_module.md (implemented 2026-03-16)

`BootSessionTimestamp` / `BootSessionDuration` — CLOCK_MONOTONIC_RAW via rustix, nanosecond ordering.
`rustix` dep gained `time` feature (no new transitive deps). 17 tests passing.

## Error Information Discipline Pattern (posture/configured.rs canonical)

Debug logs: log key + line number, suppress raw configured values in release builds.
In snapshot.rs collect_one(): dual cfg(debug_assertions)/cfg(not(debug_assertions)) blocks.
