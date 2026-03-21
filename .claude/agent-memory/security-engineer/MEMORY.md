# Security Engineer — Persistent Memory
# Alias: Knox (pending Jamie confirmation — alias TBD per cross-team notes 2026-03-21)

## Cross-Team Channel
To notify the tech-writer or senior-tech-writer of compliance findings that require doc
updates, new control mappings, or audit gaps: write an entry to
`.claude/agent-memory/cross-team/notes.md` (from: security-engineer, to: tech-writer or
senior-tech-writer). Read the file at session start for entries addressed to you.

## SCAP/STIG Corpus Reference
- Full familiarization notes: `.claude/agent-memory/security-engineer/scap_familiarization.md`
- 451 RHEL 10 STIG signals; 13 existing UMRS indicators matched to CCEs (annotations missing — Phase 3a for rust-developer)
- 51 audit-rule signals with zero UMRS coverage; Tier 1 (14 signals) are xattr/SELinux-management/kmod — highest priority
- HIGH deployment risks: fapolicyd blocks ~/.local/bin binaries by default; UMRS RPM must include fapolicyd trust entry
- UMRS adds value STIG cannot: configured-vs-live contradiction detection for sysctl and SELinux enforce state
- RAG limitation: scap-security-guide collection stores as single chunks; use direct file reads (rg on stig-signal-index.md) not RAG
- Phase 3d COMPLETE: `docs/modules/architecture/pages/stig-methodology-comparison.adoc` written; added to nav.adoc Security Model section

## SELinux Type Naming Conventions (umrs-platform)
- No dedicated SELinux types written yet for the OS detection subsystem.
- First policy artifact authoring: defer until coder closes F-01/F-02/F-03.

## Established DAC/MAC Patterns

### ProcfsText / SysfsText
See CLAUDE.md §Critical Coding Rules and agents/rust-developer.md for the mandatory rule.

### opened_by_fd Flag Discipline
`EvidenceRecord::opened_by_fd` must be `true` ONLY when the file was actually
opened via an fd-anchored call (ProcfsText/SysfsText engine or explicit
`openat2` + re-verified `fstat`). `File::open(path)` = `opened_by_fd: false`.
Violation found in `integrity_check.rs` — F-02 in 2026-03-11 audit.

## Recurring Gap Patterns Across Components

### Pattern: FIPS gate missing before non-validated cryptographic primitive
When any code asserts a trust/integrity claim backed by a RustCrypto primitive
(`sha2`, `sha3`, etc.), there must be a runtime FIPS gate reading
`/proc/sys/kernel/fips_enabled` via `ProcfsText` before the claim is made.
If FIPS is active, the phase must downgrade and emit `log::warn!`.
First observed: `integrity_check.rs` T4 assertion. Flag in every future review.

### Pattern: Stub that returns parse_ok=true with can_query_ownership=false
Stubs that partially implement `PackageProbe` must emit `log::warn!` when
selected for T3 assertion with capability flags false. Without the warning,
operators see T3 in logs with no indication it was presence-only evidence.
First observed: `rpm.rs` + `dpkg.rs` stubs. Flag in every future review.

### Pattern: Path::exists() used for security-relevant presence checks
`Path::exists()` is a TOCTOU check-then-use. For package DB or security-file
presence, require at minimum a `statfs` magic check after existence confirmation.
Record `fs_magic` in the EvidenceRecord. Flag everywhere this appears.
Also seen in posture module: `configured.rs` and `modprobe.rs` use `dir.exists()`
before `read_dir()`. Correct fix: remove the guard and handle `NotFound` from
`read_dir()` / `read_to_string()` directly (matching `BootIdReader` pattern).

### Pattern: collect_one / summary debug log leaking raw configured values
Any function that assembles a `SignalReport` and logs both live and configured
raw values at `log::debug!` WITHOUT `#[cfg(debug_assertions)]` gating violates
Error Information Discipline. The loader functions (`configured.rs`,
`modprobe.rs`) suppress values correctly; the assembler (`snapshot.rs:collect_one`)
must not bypass that discipline. Flag whenever a summary log emits `c.raw` for
configured values from /etc/ paths outside debug_assertions gating.

### Pattern: Second path-based file open after statx
A file selected via `statx` in one phase and re-opened by path in a later
phase is a TOCTOU window. The correct pattern is:
  open → fstat → compare (dev,ino) → read from the same fd.
First observed: `release_parse.rs` `read_candidate()` re-opening by path.

## Build/Packaging Integration Notes
- No RPM spec or semodule integration written for OS detection subsystem yet.
- OS detection produces no installed files; SELinux policy needed when binaries
  using `OsDetector::detect()` are packaged and assigned types.

## Audit Reports Produced
- `2026-03-11-os-detection-umrs-platform.md` — 8 findings (2H, 3M, 3L)
  Key issues: FIPS gate missing (F-01), false opened_by_fd (F-02),
  release_parse TOCTOU (F-03), stub T3 warning (F-04), Path::exists TOCTOU (F-05).
- `security-engineer-posture-2a-review.md` — 5 findings (0C, 1H, 1M, 3L)
  Key issue: `evaluate_configured_meets` cannot parse the "blacklisted" sentinel
  string, silently suppressing BootDrift contradiction for all 4 DMA-surface
  blacklist signals. Also: module parameter values logged unredacted at debug
  level during modprobe.d merge.
  **Status: findings now RESOLVED in current code (blacklist sentinel fix implemented).**
- `posture-security-review-phase2a.md` — 7 findings (0C, 1H, 4M, 2L)
  Key issue (F-01 HIGH): `collect_one` debug summary log in `snapshot.rs` emits
  raw configured values for ALL 27 signals in release builds — bypasses the
  Error Information Discipline applied in `configured.rs` and `modprobe.rs`.
  F-05 MEDIUM: negative sysctl.d configured values (e.g., `perf_event_paranoid=-1`)
  parse as `None` in `evaluate_configured_meets`, suppressing EphemeralHotfix.

## Posture Probe — Known Type Patterns
- `ModprobeConfig::raw` field for blacklist signals uses sentinel `"blacklisted"`.
  `evaluate_configured_meets` must handle this sentinel — it is not a u32.
  Any future signal that uses a non-integer configured-value sentinel must
  extend `evaluate_configured_meets` or use a dedicated evaluation path.
- `FipsCrossCheck::as_configured_value()` raw field is a human-readable summary
  string (e.g., `"marker=present cmdline=fips=1"`). It cannot participate in
  the standard `evaluate_configured_meets` integer path; `configured_meets`
  will always be `None` for FIPS by construction. This is currently latent
  (no incorrect result) but should be documented.
- `KernelCmdline` class signals (Phase 2b): `configured_value.raw` is the full
  BLS options string. `evaluate_configured_meets` returns `None` for it
  (not an integer, not "blacklisted"). A dedicated token-based evaluation path
  MUST be implemented in `collect_one()` for contradiction detection to work
  for cmdline signals. Until S-01 (Phase 2b review) is resolved, BootDrift and
  EphemeralHotfix are silently suppressed for all six KernelCmdline signals.

## Audit Reports Produced — Phase 2b
- `security-engineer-phase2b-review.md` — 8 findings (0C, 1H, 2M, 5L)
  Key issue (S-01 HIGH): KernelCmdline contradiction detection non-functional
  (configured_meets always None; BootDrift/EphemeralHotfix never produced for
  cmdline signals despite configured_value populated from BLS entries).
  M-02: /usr/bin/false missing from hard-blacklist sentinel set in modprobe.rs.
  M-03: is_module_loaded/read_module_param public API accepts path-traversal
  characters in module_name.

## Advisory configured-value read pattern (posture module)
Files under /etc/ and /boot/ (sysctl.d, modprobe.d, BLS entries) use raw
std::fs::read_to_string / read_dir — NOT SecureReader. This is intentional and
correct: they represent advisory configured (intended) values; the live kernel
state (from /proc/ and /sys/) is always authoritative. Do not flag raw reads
on these paths as provenance violations in future reviews.

Exception: /proc/sys/kernel/osrelease in bootcmdline.rs is read raw for
heuristic BLS entry selection. This is safe due to SELinux procfs write
protection, but the SELinux dependency must be documented (Finding B-01).

## M3.5 Deployment Architecture Context (2026-03-21)
- M3.5 is the security-engineer's primary milestone: transition from ~/.local/bin to
  /usr/bin (RPM-managed) with full SELinux type enforcement, fapolicyd, and AIDE.
- Current binary inventory: umrs-ls, umrs-os-detect-tui, umrs-file-stat
  (renaming in progress: umrs-file-stat→umrs-stat, umrs-tui main→umrs-uname).
- Existing policy artifacts in components/platforms/rhel10/umrs-selinux/policy/:
  umrs.te / umrs.fc / umrs.if — defines data custody types (config/data/log/httpd).
  umrs_cui.te / umrs_cui.if — CUI custody types hierarchy.
  cui_lei.te / cui_lei.fc / cui_lei.if — LEI subcategory types.
  umrs_cui.fc — EMPTY. No file context entries exist for CUI data paths.
  umrs.fc — covers /usr/share/umrs, /etc/umrs, /var/lib/umrs, /var/log/umrs, /var/www/umrs.
  CRITICAL GAP: no umrs_exec_t type exists. No binary file contexts exist.
  No domain transition rules exist. No process domain is defined.
- Makefile exists and is correct (semodule -i loop, delegates to selinux devel Makefile).
- fapolicyd integration: zero artifacts exist. HIGH priority for STIG-compliant hosts.
- AIDE integration: zero artifacts exist.
- RPM spec: does not exist.
- Phase 1 = targeted policy, labeling only. Phase 2 = MLS enforcement.
- umrs-mcs will manage setrans.conf (do not author setrans.conf directly).
- xattr-sanitization-gap plan exists (draft) — umrs-xattr-strip tool planned;
  xattr namespace policy table documented in that plan is authoritative for
  security.* xattr handling rules.
