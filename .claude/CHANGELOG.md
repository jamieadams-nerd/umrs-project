# Changelog

## 2026-03-10

### Added
- RAG reference library under `.claude/rag/` — semantic search over kernel and SELinux reference material
- `rag-ingest` skill (`.claude/skills/rag-ingest/`) — ingests new or updated reference material into the RAG database
- `rag-query` skill (`.claude/skills/rag-query/`) — semantic search over the RAG library; triggered by agents working on SELinux, MLS, kernel internals, IMA, dm-crypt, capabilities, xattrs, CUI, and related topics
- Reference collections indexed into RAG: kernel docs (`.claude/references/kernel-docs/`) and Linux FHS 2.3 (`.claude/references/linux-fhs-2.3/`)
- RAG configuration document at `.claude/configure_rag.md`
- Crate dependency rules section in `CLAUDE.md` — fixed architectural constraint table: `umrs-platform` has no workspace deps; `umrs-selinux` depends on `umrs-platform` only; `umrs-core` depends on both
- New symbols module in `umrs-core/src/console/symbols.rs`
- New typography module in `umrs-core/src/console/typography.rs`

### Changed
- `rust-developer` agent definition updated
- `refs/manifest.md` updated to reflect current reference document inventory

## 2026-03-07

### Added
- `observations.rs` module with public `SecurityObservation` and `ObservationKind` enums; re-exported at crate root
- `ObservationKind::Good`, `ObservationKind::Warning`, `ObservationKind::Risk` polarity classification for observations
- `ImaHashPresent` observation (Good) — fires when `security.ima` xattr is present on regular files
- `ImmutableFlagSet` observation (Good) — fires when `FS_IMMUTABLE_FL` inode flag is set
- `SetuidWritable` observation (Risk) — fires when setuid file has group or world write permission
- `LinuxOwnership::resolve()` method for NSS-backed name resolution via `nix::unistd`; fixes orphaned-account detection
- Encryption icon and column documentation to `umrs-ls.adoc` with SC-28 control mapping
- Developer verification guide for encryption icon in `docs/_scratch/notes/encrypt-icon-verification.adoc`
- `SysfsText` type in new `kattrs/sysfs.rs` module for filesystem-rooted sysfs text sources

### Changed
- `SecurityObservation` moved from inline module in `secure_dirent.rs` to dedicated `src/observations.rs`
- `ObservationKind` variant added to each `SecurityObservation`; `kind()` method on enum
- `SetuidBitSet` and `SetgidBitSet` reclassified from Risk to Warning
- `AccessDenied` reclassified from Risk to Warning (valid tightened-directory posture)
- `WorldWritable` no longer fires on symbolic links
- `SecureDirent::from_path()` now calls `LinuxOwnership::resolve()` for proper NSS name resolution; only orphaned accounts now report as unresolved
- `umrs-ls` IOV column (O posture marker) now uses `o.kind() == ObservationKind::Risk` instead of hand-maintained blocklist
- `kattrs/mod.rs` refactored to support filesystem magic validation strategy with type-level binding
- `kattrs/traits.rs` visibility annotations clarified

### Fixed
- Removed `NoImaProtection` observation (fired constantly when IMA not deployed — constant noise)
- Name resolution now accurate: files owned by deleted accounts correctly show as unresolved; previously all files showed as unresolved
- New Risk observations automatically light up O column without requiring code changes

## 2026-03-05

### Changed
- `SecureDirent` error enum: removed dead `AccessDenied` variant (inaccessible inodes are signaled via `access_denied: bool` field on success, not via error path)
- `dirlist.rs`: distinguished two restricted-access conditions: `<restricted>` for DAC/MAC denial, `<unlabeled> :: <no-level>` for genuinely missing xattr
- `dirlist.rs`: early-return logic in `extract_group_key` checks `access_denied` flag first; removed dead error match arm; eliminated unused imports and variables
- Documentation updated: `secure_dirent.rs`, `dirlist.rs`, and `ls_ha.rs` reflect access-denied vs unlabeled distinction

### Fixed
- `ls_ha` grouping now correctly reflects access restrictions vs missing labels; denied entries render under `<restricted>` header with bold+underline style

## 2026-03-04

### Added
- `umrs-platform` crate with kernel attribute reader interface
- 12 integration tests for `kattrs` module in `umrs-platform/tests/kattrs_tests.rs`
- `SecureReader::read_with_card()` method for provenance-verified attribute card construction
- `#[must_use]` attributes on `GenericKernelBool::path()` and `GenericDualBool::path()`
- NIST 800-53 and NSA RTB compliance annotations to `AttributeCard`, `StaticSource`, `KernelFileSource`, `SecureReader`, `GenericKernelBool`, `GenericDualBool`, `SelinuxEnforce`, `SelinuxMls`, `SelinuxPolicyVers`, `ProcFips`
- Agent definition files under `.claude/agents/`: researcher, security-auditor, umrs-translator, guest-coder, guest-admin, and tech-writer with full role specifications and operational constraints
- Writing mode specifications document to guide presentation across project communications

### Changed
- Moved `kattrs.rs` from `umrs-selinux::utils` to `umrs-platform` crate as primary home
- `umrs-selinux` now imports kattrs types from `umrs-platform` dependency
- `StaticSource::read()` default now delegates to `SecureReader::<Self>::new().read()` for non-bypassable provenance verification
- `execute_read` TOCTOU race eliminated: file opened first, then fd-anchored `fstatfs()` called on open descriptor instead of path-based call
- `GenericKernelBool` and `GenericDualBool` fields made private; validation bound at construction via `new_selinux()`
- `read_generic` uses `node.expected_magic` instead of hardcoded constant for filesystem magic validation
- `execute_read` integrity failures now emit detailed path to error log (audit stream) and return generic fixed string to caller
- `GenericDualBool::parse` uses bounds-safe indexing (`.get()` with `ok_or_else()`)
- `SelinuxPolicyVers::parse` suppresses `ParseIntError` detail in fixed error string
- `AttributeCard<T>` now owns its value (no lifetime) and carries `read_at: SystemTime` timestamp
- `AttributeCard` display format shows unix-second read timestamp
- Clippy findings fixed across `kernel-files`, `cui-labels`, `vaultmgr`, `mcs-setrans` crates
- Moved tech-writer agent from global scope to project-scoped `.claude/agents/` with Antora module map and audience-specific guidance
- Distilled senior-tech-writer agent in global space to improve clarity while preserving behavioral instructions

### Fixed
- High-assurance pattern documentation corrected for TOCTOU (file-first pattern) and `StaticSource` delegation
- `AttributeCard` documentation updated for owned value semantics and `read_with_card()` constructor
- Error information discipline applied: sensitive data no longer present in caller-visible error strings
- Kernel attribute reader provenance chain now non-bypassable via sealed trait delegation

## 2026-03-03

### Added
- Kernel lockdown and module hardening page to deployment module (`kernel-lockdown-moddisable.adoc`)
- Protected Files Rule to `.claude/rules/agent_behavior_rules.md` with explicit glob patterns for files the agent must never edit
- `changelog-updater` agent definition at `.claude/agents/changelog-updater.md`
- `Next Steps` sections to `linux-baseline.adoc`, `filesystem-layout.adoc`, `tmp-security.adoc`, `rhel10-openscap.adoc`, `rhel10-packages.adoc`, and `rhel10-setrans.adoc`

### Changed
- Deployment module nav updated to include kernel lockdown page in the RHEL 10 sequence
- `ima-evm-setup.adoc` Next Steps updated to link forward to kernel lockdown page
- `rhel10-installation.adoc` Next Steps corrected to match nav order and include IMA/EVM and kernel lockdown
- `CLAUDE.md` file-restriction rules removed and consolidated into the Protected Files Rule
- `high_assurance_pattern_rules.md` clarified "debug mode" to mean `#[cfg(debug_assertions)]` in Rust
- `assurance_rules.md` clarified that security control alignment is a design objective, not a blocker

### Fixed
- `systemctl daemon-reexec` corrected to `daemon-reload` in `tmp-security.adoc`
- RHEL 7 STIG IDs in `filesystem-layout.adoc` compliance table noted as approximate references pending RHEL 10 STIG publication
- Orphan bullet folded into Repository Interaction Rule in `agent_behavior_rules.md`
