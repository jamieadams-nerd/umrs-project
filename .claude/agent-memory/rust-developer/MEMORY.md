# Rust Developer Agent Memory

## Permanent Crate Dependency Rules (ARCHITECTURAL CONSTRAINT — NEVER VIOLATE)

| Crate          | Allowed workspace dependencies        |
|----------------|---------------------------------------|
| `umrs-platform`| None — no deps on selinux or core     |
| `umrs-selinux` | `umrs-platform` only                  |
| `umrs-core`    | `umrs-platform` and `umrs-selinux`    |

- Directions are **fixed**. Never reverse or add to them.
- Datatypes from `umrs_selinux::` must NOT be used in `umrs_platform`.
- Datatypes from `umrs_platform::` may be used by `umrs_selinux` and `umrs_core`.
- `umrs-platform` must never use `console::*` items for displaying.
- Before adding any `path = "../..."` dep to a `Cargo.toml`, verify it does not violate the table.
- If a proposed design requires a direction not listed here, STOP and raise it with Jamie.

## How Jamie Signals Permanent Rules

When Jamie says **"Add the following permanent constraint to CLAUDE.md: [rule]"**, that rule
must be written into `CLAUDE.md` as a standing, permanent rule — not just guidance for the
current session. The word "permanent" is the signal to treat it this way.

## Session Start Checklist
- Check `.claude/reports/` for outstanding security-auditor and security-engineer findings tagged "coder"
- Run `cargo xtask clippy && cargo xtask test` to verify workspace state
- Review git status for unverified changes from prior sessions

## Role Boundaries
- NEVER edit files under `docs/` — only note what needs updating for the tech-writer
- To notify the tech-writer: write an entry to `.claude/agent-memory/cross-team/notes.md`
  (from: rust-developer, to: tech-writer). Include the pattern name, file path, or API surface
  that needs documenting. Do NOT leave the note in your own MEMORY.md.
- Dependency approval: security-engineer agent can approve new crates (must notify Jamie, create report)
- When in doubt on pattern measurement granularity, consult security-engineer agent

## High-Assurance Pattern Decisions
- **TPI**: Does NOT apply to kernel attributes (booleans, dual-booleans). ASK Jamie if a kernel attribute has complex parsed structure beyond bool/dual-bool.
- **Constant-time comparison**: Use judgment. Not every label comparison needs `subtle::ConstantTimeEq` — apply where timing side-channels are a realistic threat.
- **Zeroize**: Reserved for key material, passwords, and cleartext secrets only. `SecurityContext` and MLS labels do NOT require zeroize.
- **Pattern timing**: Use judgment on granularity. Consult security-engineer when unsure.
- **Mandatory patterns** (e.g., ProcfsText for /proc/ reads, TOCTOU safety): Apply without pausing. Report what was applied after the fact.

## Clippy and Code Style
- AVOID `#[allow(...)]` whenever possible. ASK Jamie before adding any allow attribute.
- `expect()` policy: Deferred — Jamie will provide secure coding guides as reference material. Until then, avoid `expect()` in new code.

## Cargo.lock
- Not a protected file. Let it change naturally when Cargo.toml is modified.

## Examples
- Every new public module gets an example in the standard Cargo `examples/` directory.
- guest-coder agent uses these examples as its starting point — make them clear and complete.

## xtask Clippy and Cargo.toml Lints Interaction (IMPORTANT)
- `cargo xtask clippy` passes `-D warnings` on the command line to rustc/clippy.
- Command-line `-D warnings` OVERRIDES `[lints.clippy]` allow entries in `Cargo.toml`.
- Consequence: any lint suppressed only in `Cargo.toml` `[lints]` will still fire under xtask.
- Fix: add `#![allow(clippy::lint_name)]` at the crate root in the source file, AND document
  the rationale. The `Cargo.toml` entry can stay for non-xtask builds.
- Always ask Jamie before adding new `#[allow]` attributes.

## gettextrs / gettext-rs (as of 2026-03-10)
- The crate is named `gettext-rs` on crates.io (latest stable: 0.7.7).
- Library name when imported in Rust: `gettextrs` (use `use gettextrs::gettext;`).
- It provides a plain `gettext(str) -> String` function — NOT a `gettext!()` macro.
- For translated strings with substitution: use `format!()` with the translated template.
  Pattern: `format!("{} ({n}) ", gettext("label"))` — keep the static string separate.
- The `gettext-system` feature uses the OS-provided libintl (preferred on RHEL10).
- Has FFI dep (`gettext-sys`) — supply chain review required before adding.

## umrs-platform OS Detection Subsystem (completed pre-implementation gates 2026-03-11)

### New dependencies added to umrs-platform/Cargo.toml
- `rustix 0.38` (fs, process, system features) — statx, readlinkat, openat2, getpid
- `sha2 0.10` (no default features) — SHA-256 for integrity_check.rs only
- `thiserror 1` — DetectionError, OsReleaseParseError

### Workspace Cargo.toml change
- Added `[profile.release] overflow-checks = true` (NIST SA-15, ANSSI Rust Guide)

### New modules in umrs-platform/src/ (all complete and clippy-clean)
- `confidence.rs` — TrustLevel (T0–T4), Contradiction, ConfidenceModel (+upgrade, +downgrade, +record_contradiction)
- `evidence.rs` — FileStat, SourceKind, DigestAlgorithm, PkgDigest, EvidenceRecord, EvidenceBundle
- `os_identity.rs` — OsFamily, Distro, KernelRelease, CpuArch, SubstrateIdentity (+add_fact using saturating_add, +meets_t3_threshold)
- `os_release.rs` — OsRelease + all newtypes (OsId, OsName, VersionId, OsVersion, Codename, CpeName, ValidatedUrl, VariantId, BuildId) + OsReleaseParseError
- `detect/mod.rs` — OsDetector (default limits), DetectionResult, DetectionError (thiserror); detect() is todo!()
- `detect/label_trust.rs` — LabelTrust enum (UntrustedLabelCandidate, LabelClaim, TrustedLabel, IntegrityVerifiedButContradictory)
- `detect/substrate/mod.rs` — PackageProbe trait, ProbeResult, FileOwnership, InstalledDigest

### Phase module naming (semantic, not phase0..phase6)
kernel_anchor, mount_topology, release_candidate, pkg_substrate, file_ownership, integrity_check, release_parse
All are commented out in detect/mod.rs pending implementation.

### cargo audit status
- cargo-audit 0.22.1 installed at /home/jadams/.cargo/bin/cargo-audit
- Zero advisories against all 152 resolved crates as of 2026-03-11

## Pre-Existing Build Issues (Do NOT Fix Without Instruction)
- `umrs-core/src/console/macros.rs` — unused `tr_core` import causes compile failure
- `vaultmgr` — depends on `umrs-core` macro, fails to compile as a result
- These are from an incomplete i18n migration in a prior session; out of scope until instructed

## Critical Clippy Patterns
- `pub(crate)` inside a private module fires `redundant_pub_crate` — use `pub(super)` in phase modules
- Nested `if` blocks must be collapsed with `&&` (`collapsible_if`)
- `match Option { Some(x) => x, None => { return ...; } }` → `let Some(x) = ... else { ... };`
- `map(...).flatten()` on Option → `.and_then(...)`
- Lifetime elision: `fn foo<'a>(x: &'a str) -> &'a str` → `fn foo(x: &str) -> &str`
- `&Option<T>` parameter → `Option<&T>` (clippy::ref_option)
- `format!("{}", x)` in log! macro → `log::debug!("... {x}")` (uninlined_format_args)
- `redundant_clone` fires on `.clone()` when the value is dropped immediately after

## Phase Module Convention
- All phase `run()` functions: `pub(super) fn run(evidence, confidence, ...) -> ...`
- Timing wrapper in every `run()`: `#[cfg(debug_assertions)] let t0 = Instant::now()`
- Always delegate to `run_inner()` to keep timing wrapper minimal

## SecureReader / ProcfsText / SysfsText Usage
- `/proc/` reads: `ProcfsText::new(path)` + `SecureReader::<ProcfsText>::new().read_generic_text(&node)`
- `/sys/` reads: `SysfsText::new(path)` + `SecureReader::<SysfsText>::new().read_generic_text(&node)`
- Static kernel attrs (selinuxfs, securityfs): `SecureReader::<SelinuxEnforce>::new().read()`
- `SelinuxEnforce::PATH` requires `use crate::kattrs::StaticSource as _;` in scope
- `EnforceState` has no `is_enforcing()` — compare directly: `state == EnforceState::Enforcing`

## TPI in release_parse.rs
- Path A: `nom` — tokenizer for KEY=VALUE / KEY="VALUE" / comments / blanks
- Path B: `split_once('=')` — independent line scanner
- Agreement check: key sets must be identical; fail closed on disagreement
- Values for OsRelease construction come from Path A only

## EvidenceRecord Construction
- `EvidenceRecord` has 11 fields; most are `None` for a given record
- Pattern: fill only what you know; leave others `None`
- Always push to evidence even on failure (audit completeness)

## RPM DB Query Support (completed 2026-03-11)

### New dependency: rusqlite 0.31 (feature-gated)
- Feature: `rpm-db` (default ON); `rusqlite = { version = "0.31", features = ["bundled"], optional = true }`
- `bundled` feature vendors SQLite — no system library dep, hermetic binary.

### New files under umrs-platform/src/detect/substrate/
- `rpm_header.rs` — TPI parser (nom + manual byte-slicing); `parse_rpm_header()` is public; types: `RpmHeader`, `RpmFileEntry`, `RpmDigestAlgo`, `RpmHeaderError`, `IndexEntry`
- `rpm_db.rs` — `RpmDb` (read-only SQLite handle); public methods: `open()`, `query_file_owner()`, `query_file_digest()`, `is_installed()`

### substrate/mod.rs change
- Both `rpm_header` and `rpm_db` are `pub mod` (required for integration tests).
- Types in both modules are `pub` (not `pub(super)`) for the same reason.

### rpm.rs changes
- `RpmProbe` gains `db: Mutex<Option<RpmDb>>` field (feature-gated)
- `probe()` delegates to `probe_inner()` + `try_open_db()` to stay under 100-line limit
- TOCTOU: `query_ownership` re-stats the path and checks `(dev, ino)` match before returning
- `pub fn is_installed(pkgname: &str) -> bool` re-exported via `detect::is_installed`

### Integration tests
- `tests/rpm_header_tests.rs` — 16 pure-logic tests (no filesystem needed)
- `tests/rpm_db_tests.rs` — 7 tests (skip if no RPM DB present)

### Key clippy patterns encountered
- `ok_or(fn_call())` → `ok_or_else(|| fn_call())` when fn has side effects / is non-Copy
- `ok_or_else(|| struct_literal)` → `ok_or(struct_literal)` when struct fields are already bound variables (clippy::unnecessary_closure_called_with_none)
- `% 2 != 0` → `.is_multiple_of(2).not()` or `!x.is_multiple_of(2)` (clippy::manual_is_multiple_of)
- `let guard = match x.lock() { Ok(g) => g, Err(_) => return None }` → `let Ok(guard) = x.lock() else { return None; }`

### Pre-existing test failure (do NOT fix without instruction)
- `umrs-selinux/tests/mcs_translator.rs` — 5 tests fail because `setrans.conf` was deleted
  (visible as `D components/rusty-gadgets/umrs-selinux/setrans.conf` in git status)

## umrs-platform Examples (updated 2026-03-11)

- `examples/display.rs` — minimal example; kept in place (do not delete)
- `examples/os_detect.rs` — full OS detection walkthrough (created 2026-03-11)
- `examples/rpm_probe.rs` — RPM substrate probe demo (pre-existing)

### os_detect.rs design notes
- `is_installed(name: &str) -> bool` — returns plain `bool`, NOT `Option<bool>`.
  The task spec incorrectly described it as `Option<bool>`; actual signature confirmed in rpm.rs.
- ANSI colour guard: `std::io::IsTerminal` on `stdout()` — no extra deps needed.
- All `Contradiction.description` and `LabelTrust::IntegrityVerifiedButContradictory.contradiction`
  strings are truncated to 64 chars via `.chars().take(64).collect()` before display (SI-12).
- Evidence record index uses `i.saturating_add(1)` throughout (secure arithmetic).

## Outstanding Audit Findings Assigned to Coder (2026-03-11)

From `.claude/reports/2026-03-11-os-detection-umrs-platform.md` (security-engineer):
- **F-01 HIGH** `integrity_check.rs`: FIPS gate missing — sha2 not FIPS-validated; no guard before T4
- **F-02 HIGH** `integrity_check.rs`: `opened_by_fd: true` set on path-based `File::open` (false provenance)
- **F-03 MEDIUM** `release_parse.rs`: second path-based os-release read; no (dev, ino) re-verification
- **F-04 MEDIUM** stub probes assert T3 without `log::warn!` when ownership/digest capability absent
- **F-05 MEDIUM** `rpm.rs`/`dpkg.rs`: `Path::exists()` micro-TOCTOU; no statfs magic check on DB root
- **F-06–F-08 LOW** annotation gaps, symlink record ambiguity, statfs SourceKind accuracy

From `.claude/reports/2026-03-11-os-detection-umrs-platform-surface-audit.md` (security-auditor):
22 additional findings including HIGH FIPS gap (duplicate of F-01), TPI value comparison gap,
`opened_by_fd` inconsistency, and documentation/annotation gaps.

None of these findings were addressed in the os_detect example session — they require source
changes in `integrity_check.rs`, `release_parse.rs`, `pkg_substrate.rs`, `substrate/rpm.rs`,
`substrate/dpkg.rs`. Address before next production gate.

## Known Pre-existing Issues Fixed (2026-03-10)
- `umrs-selinux/src/observations.rs:160` — `missing_const_for_fn` on `SecurityObservation::kind()`.
  Fixed: added `const` to the fn signature.
- `umrs-ls/src/main.rs:66` — `BOX_SLIM_CONN` declaration missing semicolon (syntax error).
  Fixed: added semicolon and extra space cleanup.
- `umrs-ls/src/main.rs` — `format_push_string` and dead-code (`BOX_BOLD`, `BOX_SLIM_CONN`)
  lints were suppressed in Cargo.toml but fired under xtask -D warnings.
  Fixed: added crate-level `#![allow(clippy::format_push_string)]` and item-level
  `#[allow(dead_code)]` on the WIP box-drawing constants.
