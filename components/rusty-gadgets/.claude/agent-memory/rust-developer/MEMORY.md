# Rust Developer Agent Memory

## Project: UMRS — rusty-gadgets workspace

### Key Paths
- Workspace root: `components/rusty-gadgets/`
- Primary crate: `umrs-selinux/`
- Platform crate: `umrs-platform/`
- Run commands: `cargo xtask clippy`, `cargo xtask test`, `cargo xtask fmt`

### Pre-Existing Build Issues (Do Not Fix Without Instruction)
- `umrs-core/src/console/macros.rs` — unused `tr_core` import causes compile failure
- `vaultmgr` — depends on `umrs-core` macro, fails to compile as a result
- These are from an incomplete i18n migration in a prior session; out of scope

### umrs-platform OS Detection Pipeline (Completed 2026-03-11)

Seven phase modules in `umrs-platform/src/detect/`:
1. `kernel_anchor.rs` — hard gate: procfs magic + PID coherence + boot_id + lockdown
2. `mount_topology.rs` — soft: mount namespace + mountinfo + statfs /etc
3. `release_candidate.rs` — soft: statx os-release candidate, symlink resolution
4. `pkg_substrate.rs` — soft: RPM/dpkg probe, SELinux enforce Biba pre-check, T3
5. `file_ownership.rs` — soft: query probe for package ownership of candidate
6. `integrity_check.rs` — soft: SHA-256 compute + compare vs package DB
7. `release_parse.rs` — soft: TPI (nom + split_once), substrate corroboration, LabelTrust

Substrate stubs: `detect/substrate/rpm.rs`, `detect/substrate/dpkg.rs`

### Critical Clippy Patterns
- `pub(crate)` inside a private module fires `redundant_pub_crate` — use `pub(super)` in phase modules
- Nested `if` blocks must be collapsed with `&&` (`collapsible_if`)
- `match Option { Some(x) => x, None => { return ...; } }` → `let Some(x) = ... else { ... };`
- `map(...).flatten()` on Option → `.and_then(...)`
- Lifetime elision: `fn foo<'a>(x: &'a str) -> &'a str` → `fn foo(x: &str) -> &str`
- `&Option<T>` parameter → `Option<&T>` (clippy::ref_option)
- `format!("{}", x)` in log! macro → `log::debug!("... {x}")` (uninlined_format_args)
- `redundant_clone` fires on `.clone()` when the value is dropped immediately after

### Phase Module Convention
- All phase `run()` functions: `pub(super) fn run(evidence, confidence, ...) -> ...`
- Timing wrapper in every `run()`: `#[cfg(debug_assertions)] let t0 = Instant::now()`
- Always delegate to `run_inner()` to keep timing wrapper minimal

### SecureReader / ProcfsText / SysfsText
- `/proc/` reads: `ProcfsText::new(path)` + `SecureReader::<ProcfsText>::new().read_generic_text(&node)`
- `/sys/` reads: `SysfsText::new(path)` + `SecureReader::<SysfsText>::new().read_generic_text(&node)`
- Static kernel attrs (selinuxfs, securityfs): `SecureReader::<SelinuxEnforce>::new().read()`
- `SelinuxEnforce::PATH` requires `use crate::kattrs::StaticSource as _;` in scope
- `EnforceState` has no `is_enforcing()` — compare directly: `state == EnforceState::Enforcing`

### overflow-checks
- `[profile.release] overflow-checks = true` added to workspace `Cargo.toml` (NIST SA-15)
- Citation: ANSSI Rust Secure Coding Guide, Finding 1

### sha2 FIPS Posture
- `sha2 0.10` is NOT FIPS 140-2/3 validated
- Used ONLY for file integrity comparison in `integrity_check.rs`
- NOT for key derivation, MACs, or authentication
- Must be replaced with FIPS provider on systems requiring validated SHA-256 for this operation

### TPI in release_parse.rs
- Path A: `nom` — tokenizer for KEY=VALUE / KEY="VALUE" / comments / blanks
- Path B: `split_once('=')` — independent line scanner
- Agreement check: key sets must be identical; fail closed on disagreement
- Values for OsRelease construction come from Path A only

### EvidenceRecord Construction
- `EvidenceRecord` has 11 fields; most are `None` for a given record
- Pattern: fill only what you know; leave others `None`
- Always push to evidence even on failure (audit completeness)

### Test Structure
- Integration tests in `tests/` only — never inline
- umrs-platform tests: `tests/kattrs_tests.rs` (26 tests, all pure logic)
