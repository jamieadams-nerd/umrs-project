# Rust Developer Agent Memory — "Rusty"
# Alias: Rusty. No "The".
# Portrait: docs/modules/ai-transparency/images/rusty.png

## Topic Files

- [Coding conventions](coding_conventions.md) — source file headers (SPDX), use ordering, no doc tests, test tags (TEST-ID/REQUIREMENT/COMPLIANCE), i18n coding rule
- [Phase 1 / Phase 2 positioning](phase1_phase2_positioning.md) — Targeted policy = labeling fidelity + awareness, NOT enforcement; MLS adds enforcement (Phase 2)
- [Info theory foundations familiarization](info_theory_foundations_familiarization.md) — Shannon, MacKay, MDL, Kolmogorov, Dijkstra, A*, HNSW, spectral clustering, AC; RAG + MLS + posture design implications
- [Posture module history](posture_module_history.md) — Signal→Indicator rename, Phase 2b architecture, security review findings
- [SCAP/STIG familiarization](scap_familiarization.md) — CCE mappings, new indicator candidates
- [Doc placement feedback](feedback_doc_placement.md) — `# Errors` section placement rules
- [TUI reference](tui_reference.md) — umrs-ui (was umrs-tui) architecture, binaries, tests, i18n, patterns
- [Detect pipeline reference](detect_reference.md) — dev encoding, RPM fixes, SEC pattern, EvidenceBundle
- [TUI phases](tui_phase1.md) / [phase2](tui_phase2.md) / [phase3](tui_phase3.md) / [phase45](tui_phase45.md)
- [Timestamp module](timestamp_module.md) — BootSessionTimestamp/Duration, CLOCK_MONOTONIC_RAW
- [CPU access controls familiarization](cpu_access_controls_familiarization.md) — Phase 1E/1F: PCID, VulnerabilityReport, CET-SS/IBT, UMIP, PKU, ARM PAC/BTI/MTE, Rust CET limitation

## CPU/Crypto Reference Scripts

`reference/cpu-crypto/` contains Jamie's empirical detection scripts from octopussy (Ubuntu 24.04.3, ARM64):
- `cpu_info.sh` — Layer 1 + Layer 2: CPU flags, kernel crypto drivers, topology, entropy
- `umrs-openssl-audit.sh` — Layer 3: OpenSSL version/providers, algorithm surface, ARM CE benchmarks, kernel crypto cross-ref
- `create_ima_keys.sh` — IMA/EVM key generation (KMK, EVM-key, RSA keypair, X509 DER)
- `ima-reresh.sh` — IMA/EVM recursive re-signing with evmctl

These are reference implementations for the CPU extension probe's three-layer activation model.
OpenSSL is a system-wide trust anchor — binary analysis must trace linkage to it.

## Key Architectural Facts

- Primary workspace: `components/rusty-gadgets/` — production crates only
- Prototype workspace: `components/rust-prototypes/` — out of scope; no xtask
- Primary crate: `umrs-selinux`; Platform crate: `umrs-platform`
- UI library crate: `umrs-ui` (renamed from `umrs-tui` 2026-03-22); binary: `umrs-uname`
- File stat crate: `umrs-stat` (extracted from umrs-tui 2026-03-22); binary: `umrs-stat`
- HW crate: `umrs-hw` — ONLY crate WITHOUT `#![forbid(unsafe_code)]`
- All tests in `tests/`, never inline
- Run via `cargo xtask {fmt,clippy,test}` from `components/rusty-gadgets/`
- Binary crates that need testable modules: add `[lib]` + `src/lib.rs` to expose modules for `tests/`
- `umrs-ls` now has `[lib]` (lib.rs) exposing `pub mod grouping`; binary in main.rs imports via `umrs_ls::grouping::`

## posture Catalog — Display Fields (added 2026-03-22 Session 2)

- `IndicatorDescriptor` has two new fields: `description: &'static str` and `recommended: Option<&'static str>`
- All 37 entries populated: 27 Phase 1/2a indicators have non-empty `description`; Phase 2b sub-indicators use `description: ""` and `recommended: None`
- `catalog::lookup(id: IndicatorId) -> Option<&'static IndicatorDescriptor>` — re-exported at `posture::lookup`
- `posture::display` module: `annotate_live_value`, `annotate_integer`, `annotate_signed_integer` — re-exported at posture root
- TUI main.rs now calls `lookup(id).map_or("", |d| d.description)` and `lookup(id).and_then(|d| d.recommended)` and `annotate_live_value(id, live)` instead of local helpers
- Tests: `display_tests.rs` (23 tests), `catalog_lookup_tests.rs` (13 tests)

## Toolchain

- Rust 1.92+; `is_multiple_of()` stable; let-chain guards stable

## System Prerequisites

- `sqlite-devel` required (`sudo dnf install sqlite-devel`)
- Without it, `umrs-ls`/`umrs-platform` fail with `-lsqlite3 not found`

## FFI Exception — rusqlite

Accepted exception (Jamie 2026-03-11). System `libsqlite3` on RHEL; no bundled feature.
Documented in `umrs-platform/Cargo.toml` and `rpm_db.rs`.

## detect/ Pipeline

Phase order: release_candidate → release_parse → pkg_substrate → file_ownership → integrity_check

Key types:
- `EvidenceBundle` — append-only; `records` PRIVATE. Access: `.records()`, `.iter()`, `.push()`
- `EvidenceRecord` — `path_requested` = original, `path_resolved` = symlink target

## Symlink Path Resolution (RPM-16)

RHEL 10 `/etc/os-release` → `../usr/lib/os-release` (relative symlink).
RPM DB has absolute path. Fix: `find_resolved_path` → canonicalize → use for DB queries.

## umrs-selinux Import Paths

- `observations` module is **private** — import via root: `umrs_selinux::ObservationKind`
- `SelinuxCtxState` re-exported at root
- `fs_encrypt` and `posix` are `pub mod`
- `SecurityContext`: `.user()`, `.role()`, `.security_type()`, `.level()` → `Option<&MlsLevel>`
- `FileSize` is `Copy` — pass by value. Has `.as_u64()`.
- `SecureDirent::access_denied` is a `pub bool` field

## Outstanding Audit Findings

Reports in `.claude/reports/` with coder-assigned items:
- `2026-03-11-rpm-db-security-audit.md`: RPM-01, -02, -04, -06, -07, -11, -12, -15, -16, -19, -20, -22, -27, -28
- `2026-03-11-os-detection-umrs-platform-surface-audit.md`: multiple HIGH/MEDIUM
- `sec-audit-2026-03-11.md`: SEC-05, -07, -09, -10 remaining

## umrs-platform API Ergonomics (2026-03-20)

- `TrustLevel`: `Display` impl added — shows "T0 — Untrusted" etc.
- `OsId`, `OsName`, `VersionId`: `Display` impl added — delegates to inner String.
- `DetectionError`: thiserror-generated Display already existed; example updated to use `{}` not `{:?}`.
- `DetectionResult::kernel_release`: `Option<KernelRelease>` added; populated by kernel_anchor phase from `/proc/sys/kernel/osrelease` via ProcfsText. `corroborated=false` (single procfs source only).
- `is_installed` return type: changed from `bool` to `Result<bool, PackageQueryError>`. `PackageQueryError` lives in `detect::substrate::mod`, re-exported at `detect::` and crate root. Stub (no rpm-db feature) returns `Err(DatabaseUnavailable)`.
- `IndicatorDescriptor::label`: new `&'static str` field added; all 37 catalog entries populated with short column-header labels.
- `lib.rs` module doc: cross-crate pointer to `umrs-selinux` added at top.
- Tests/examples updated: `rpm_db_tests.rs`, `rpm_probe.rs`, `os_detect.rs`, `system_summary.rs`.
- `umrs-platform` suppresses: `doc_markdown`, `module_name_repetitions`, `option_if_let_else` but NOT `missing_errors_doc` — add `# Errors` section to Result-returning public functions.

## posture Module

37 signals, `PostureSnapshot::collect()`, contradiction detection, FipsCrossCheck.
Tests: posture_tests.rs (79+), posture_modprobe_tests.rs (50), posture_bootcmdline_tests.rs (10).
See [posture_module_history.md](posture_module_history.md) for full details.

## Error Information Discipline Pattern

Canonical: `posture/configured.rs`. Debug logs: key + line number, suppress raw values in release.
`snapshot.rs collect_one()`: dual `cfg(debug_assertions)` blocks.

## TUI Kernel Tab Enhancement (Phases 1-4, 2026-03-20)

Implemented in `umrs-tui/src/main.rs`:
- `translate_live_value(id, live)` — Phase 1 value translation with parenthetical annotations
- `translate_integer(id, u64)` / `translate_signed_integer(id, i64)` — annotation dispatch
- `LiveValue::Integer` is `u32`; `LiveValue::SignedInteger` is `i32` (use `u64::from()` / `i64::from()`)
- `append_indicator_group()` now takes `description: &'static str` parameter (Phase 2)
- Trust tab: "No downgrade — full trust retained" (positive framing); contradictions get explanation row
- `Action::ShowHelp` added to keymap.rs; bound to `?` and `F1`
- `const fn help_text_for_tab(usize) -> &'static str` — per-tab help text
- Dialog state `help_dialog: Option<DialogState>` in event loop; rendered via `render_dialog()`
- Navigation suppressed while dialog is open; `?` / `Enter` / `Esc` dismiss it
- 3 new tests in `umrs-tui/tests/keymap_tests.rs`: question_mark, f1, show_help_does_not_change_state

## TUI Polish Round 4 (2026-03-20)

Changes in `umrs-tui/src/`:
- `app.rs`: `IndicatorValue::Active` → `Enabled`, `Inactive` → `Disabled` (deep rename)
- `indicators.rs`: SELinux shows `"Enforcing (Targeted)"` — policy type from `selinux_policy()` (Trust Gate gated); FIPS shows `"Enabled"/"Disabled"`
- `main.rs`: `indicator_to_display` updated; FIPS annotation 0→"Disabled"/1→"Enabled"; tab order now 0=OS Info, 1=Kernel Security, 2=Trust/Evidence (Trust/Evidence always last — UMRS convention); summary pane separators removed (tighter); curated note added to kernel security summary; evidence chain has blank separator after sticky `TableHeader`
- Tests: indicators_tests, header_tests, theme_tests all updated to `Enabled`/`Disabled` variant names

## TUI Round 6 Final Polish (2026-03-20)

Changes in `umrs-tui/src/`:
- `app.rs`: `IndicatorRow` gains `recommendation: Option<&'static str>` field; new constructor
  `indicator_row_recommended(key, value, description, recommendation, hint)`
- `data_panel.rs`: `expand_row_lines` renders `[ Recommended: <value> ]` line (dim+italic) for
  red indicators below the description; `expanded_row_line_count` accounts for the extra line
- `main.rs`: `indicator_recommended(id) -> Option<&'static str>` — 37-entry lookup table for
  hardened values; `indicator_group_rows` now passes recommendation for `meets_desired=Some(false)`
  indicators only (green shows None)
- `main.rs`: `build_kernel_security_summary_rows(snap, kernel_version)` — kernel version prominently
  in pinned summary; kernel baseline placeholder with comment marker; curated note has blank line
  above and below; "Indicators/Hardened/Not Hardened/No Assessment" labels i18n-wrapped
- `main.rs`: `append_kernel_identity_rows` removed entirely — OS Release, Version ID, System UUID,
  Active LSM removed from Kernel Security scrollable rows (they were OS info, not evidence)
- `main.rs`: `build_kernel_security_rows` signature simplified — no os_release, system_uuid params
- `main.rs`: `from_result`/`from_error` no longer accept or use `system_uuid` parameter
- `main.rs`: `read_system_uuid` import removed; `OsRelease` retained for `os_name_from_release`
- `main.rs`: Comprehensive i18n pass — all major user-visible strings wrapped with `i18n::tr()`:
  tab names, summary labels, group titles, group descriptions, evidence table headers, status
  messages, error descriptions. `const fn` helpers cannot use `tr()` but callers wrap their returns.
- `tests/data_types_tests.rs`: Updated `IndicatorRow` destructure to include `recommendation`;
  2 new tests: `data_row_indicator_row_recommended_stores_recommendation`,
  `data_row_indicator_row_recommended_none_matches_indicator_row`

## TUI Polish Pass (2026-03-20)

Changes in `umrs-tui/src/`:
- `data_panel.rs`: `TABLE_COL1_WIDTH` 20→28 (fits "Kernel attributes (/sys)" = 24 chars)
- `data_panel.rs`: sticky `TableHeader` — leading `TableHeader` row extracted from scrollable
  content, rendered fixed at top with `Modifier::BOLD | Modifier::REVERSED`
- `main.rs`: Summary labels capitalized — "Label Trust", "Trust Tier", "Description",
  "Downgrade Reasons", "Contradictions", "Evidence Records"
- `main.rs`: "evidence records" count moved from scrollable evidence section to pinned Summary box
- `main.rs`: trust level labels humanized: "T3 — Platform Verified", "T1 — Kernel Anchored" etc.
- `main.rs`: build_status "Substrate Anchored"→"Platform Verified"
- `main.rs`: OS tab platform labels: "Platform Family/Distro/Version/Facts/Identity"
- `status_bar.rs`: KEY_LEGEND const + right-aligned key legend; elided on narrow terminals
- Internal Rust types (`SubstrateAnchored` enum variant etc.) NOT renamed — display layer only

## umrs-c2pa Security Review Remediation (2026-04-01)

Fixed all Knox + Herb findings. Key patterns applied:
- `zeroize::Zeroizing<Vec<u8>>` on all private key material (K-1, K-4, K-5)
- `O_NOFOLLOW` on private key reads via `OpenOptionsExt::custom_flags(libc::O_NOFOLLOW)` (K-2)
- Mode 0600 at `create_new` time, not post-chmod (K-11) — `OpenOptionsExt::mode(0o600)`
- Single-read TOCTOU fix in `ingest_file` — read once, hash + sign from same buffer (K-14)
- Permission checks in `validate.rs` (K-6, K-7, K-13) — WARN for key file, WARN for world-writable trust anchors
- Graceful journald fallback — `env_logger` on `Err`, not `.expect()` (K-12)
- Random serial numbers via `BigNum::pseudo_rand(128, ...)` (K-3)
- Full `//!` module doc + Compliance section added to `validate.rs` and `creds.rs` (E-1, E-2)
- All `#[must_use]` annotations now have descriptive message strings (E-3, C-1..C-9)
- `euid` check uses `/proc/self/status` Uid: line — no `unsafe { libc::geteuid() }`
- `libc` crate added for `O_NOFOLLOW` constant only (safe, no unsafe blocks)
- `env_logger` crate added for journald fallback

## umrs-c2pa Trust List (2026-04-02)

Trust wiring in Phase 2.2/2.3 complete. New `trust.rs` module; `build_c2pa_settings()` returns `c2pa::Settings`.
All manifest reader functions now accept `&UmrsConfig`. Use `c2pa::Reader::from_context(ctx).with_file(path)`, not `Reader::from_file(path)`.
See [project_umrs_c2pa.md](project_umrs_c2pa.md) for full details.

## umrs-labels Catalog (2026-03-27)

`src/cui/catalog.rs`: `CatalogMetadata` (flatten for nation-specific fields; `catalog_name`/`authority` `#[serde(default)]` for LEVELS.json compat), `Catalog` (both `labels` and `markings` `#[serde(default)]`), `Marking`/`Label` (`handling: serde_json::Value` — US=string, CA=object), `LevelRegistry`/`LevelDefinition` + `load_levels()`. `country_code()` helper on `Catalog`. Tests: `catalog_tests.rs` (47 tests covering fixture, US, CA, LEVELS.json).
