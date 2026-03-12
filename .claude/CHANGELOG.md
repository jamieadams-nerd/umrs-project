# Changelog

## 2026-03-11

### Added
- **Pattern library expansion**: 3 new pattern pages — `pattern-execution-measurement.adoc` (debug-mode timing discipline), `pattern-layered-separation.adoc` (write/read/display layer separation), `pattern-audit-cards.adoc` (`AttributeCard<T>` structured audit output); pattern index and nav updated
- **TPI failure analysis report**: Security-auditor assessment at `.claude/reports/2026-03-11-tpi-failure-analysis.md` — 4 findings (3 HIGH, 1 LOW/MEDIUM); all 3 HIGH findings remediated (tasks #5, #6, #7)
- **TpiError / XattrReadError / SelinuxCtxState**: Typed error hierarchy and 4-state label model in `xattrs.rs`, `secure_dirent.rs`, `observations.rs`; `SecurityObservation::SelinuxParseFailure` (Warning) and `TpiDisagreement` (Risk) variants; 43 new tests across `tpi_error_tests.rs`, `selinux_label_state_tests.rs`, `xattr_log_discipline_tests.rs`
- **TPI behavior documentation**: New section in `docs/modules/umrs-tools/pages/umrs-ls.adoc` — three-outcome table, log signature patterns, operator procedures
- **TpiError enum and XattrReadError**: New typed error hierarchy in `xattrs.rs` — `TpiError::PathAFailed`, `PathBFailed`, `Disagreement`; `XattrReadError::OsError`, `Tpi`; both paths always attempted before gate evaluation; 18 tests in `tpi_error_tests.rs`
- **SelinuxCtxState enum**: New `Labeled`, `Unlabeled`, `ParseFailure`, `TpiDisagreement` states in `secure_dirent.rs` replacing `Option<SecurityContext>`; distinct display strings (`<unlabeled>`, `<parse-error>`, `<unverifiable>`); 20 tests in `selinux_label_state_tests.rs`
- **SecurityObservation variants**: `SelinuxParseFailure` (Warning) and `TpiDisagreement` (Risk) added to `observations.rs`
- **TPI behavior documentation**: New section in `docs/modules/umrs-tools/pages/umrs-ls.adoc` — three-outcome table, log signature patterns, operator procedures for each failure mode
- **Log discipline tests**: `xattr_log_discipline_tests.rs` — 5 tests verifying nom error output never contains raw input strings (SI-12)
- **umrs-platform RPM subsystem**: New `src/detect/substrate/rpm_header.rs` (pure-Rust RPM blob parser with TPI dual-path parsing, checked arithmetic, bounds-safe indexing, error information discipline); `src/detect/substrate/rpm_db.rs` (read-only SQLite layer for `/var/lib/rpm/rpmdb.sqlite`, feature-gated `rpm-db`); `src/os_identity.rs` (typed platform identity: `OsFamily`, `Distro`, `KernelRelease`, `CpuArch`, `SubstrateIdentity`)
- **umrs-platform OS detection phases**: New `src/detect/file_ownership.rs` (ownership verification phase); reworked `src/detect/pkg_substrate.rs` (RPM-first dispatch, T3 threshold, Biba pre-check); updated `src/detect/integrity_check.rs` (FIPS gate via ProcfsText)
- **umrs-platform examples**: `examples/os_detect.rs`, `examples/rpm_probe.rs`, `examples/display.rs` — three new OS detection examples
- **umrs-platform integration tests**: `tests/rpm_header_tests.rs` (16 tests), `tests/rpm_db_tests.rs` (7 tests)
- **High-assurance patterns module** in docs: `patterns/nav.adoc` navigation; `patterns/pages/index.adoc` reference table; 13 pattern pages (TPI, TOCTOU, Provenance, Fail-Closed, Non-Bypassability, Secure Arithmetic, Bounds-Safe Indexing, Error Discipline, Loud Failure, Zeroize, Constant-Time Comparison, SEC/Sealed Evidence Cache, Supply Chain Hygiene) with threat analysis, codebase examples, and control citations
- **SEC pattern (Sealed Evidence Cache)** — New high-assurance pattern for reusing expensive verification results via HMAC-SHA-256 sealing, TTL, and FIPS gate (NIST 800-53 SC-28, SC-12)
- **architecture/ module rebuild**: `architecture/nav.adoc` restructured into 4 sections; new pages: `case-studies.adoc` (12 failure cases mapped to UMRS controls), `mls-label-model.adoc`, `cui-structure.adoc`, `integrity-and-provenance.adoc`, `rationale.adoc`, `truth-concepts.adoc` (placeholder)
- **reference/ module expansion**: New 9 pages — `fips-cryptography-cheat-sheet.adoc`, `key-recommendation-list.adoc`, `cui-descriptions.adoc`, `cui-category-abbreviations.adoc`, `setrans-technical.adoc`, `example-setrans-conf.adoc`, `rhel-selinux-users.adoc`, `mls-colors.adoc`, `umrs-mls-registry.adoc`; new `compliance-frameworks.adoc` standards registry
- **devel/ module improvements**: Substantially rewritten `i18n.adoc` (7-step workflow, architecture rule, domain listing); new `compliance-annotations.adoc` (with codebase examples); converted `rust-must-use-contract.adoc` from markdown
- **ROOT/ module onboarding**: New `introduction.adoc` (~500 words) and `getting-started.adoc` (3-path onboarding: deploy, develop, audit)
- **admin/ placeholder module** — Created with stub navigation; admin content merged into operations/
- **Deployment module**: New `ubuntu.adoc` placeholder; `rhel10-packages.adoc` extended with post-install packages section
- **umrs-core console modules**: `BoxStyle` struct in `src/console/symbols.rs` with `SLIM`, `BOLD`, `ROUNDED` constants and `icons` submodule; `TypographyStyle` enum and `stylize()` function in `src/console/typography.rs` (Unicode mathematical alphabets)
- **umrs-core validation**: New `src/validate.rs` — `UmrsPattern` enum with `is_valid()` validator using `OnceLock<Mutex<HashMap>>` regex cache
- **i18n domains**: Active `umrs-logspace` translations (5 strings: Resource Pool, Mount point, Total/Free space, Lifecycle) with fr_CA locale; updated `umrs-ls` (5 active locales: fr_CA, fr_FR, en_GB, en_AU, en_NZ); `umrs-state` (8 strings, fr_CA)
- **umrs-logspace i18n wiring**: `print_pools()` now calls `tr()` for all user-visible strings; `umrs-core` added as dependency
- **Security audit report**: 29-finding security audit at `components/rusty-gadgets/.claude/reports/2026-03-11-rpm-db-security-audit.md` documenting findings and remediation status
- **Kernel security posture probe plan**: `.claude/plans/kernel-security-posture-probe.md` — Design for new `posture/` module in `umrs-platform` reading 22 kernel security hardening signals (sysctl, cmdline, securityfs) with static Rust catalog, dual-check model (live vs. configured), typed contradictions, and two-tier API (simple and expert)

### Changed
- **CLAUDE.md refactored**: Reduced from 378 to ~195 lines (~48%); removed duplicate workspace tree and HA pattern descriptions that duplicated `.claude/rules/` and `docs/modules/patterns/`; added missing crates (`umrs-platform`, `umrs-ls`) to workspace layout; added 2 new architectural review triggers (`#[must_use]`, trust gates); added pointer to pattern library instead of duplicating content
- **High-assurance pattern rules expanded**: 6 new rules distilled from `docs/modules/devel/pages/high-assurance-patterns.adoc` into `.claude/rules/high_assurance_pattern_rules.md` — Must-Use Contract, Validate at Construction, Trust Gate, Security Findings as Data, Compile-Time Path Binding, Fixed-Size Deterministic Layout
- **TPI architecture reworked**: `read_context()` in `xattrs.rs` now always runs both parse paths before evaluating the gate; single-path failures logged at WARN, disagreements at CRITICAL; nom error output sanitized to prevent MLS level leakage (SI-12); `SecureDirent` label state replaced `Option<SecurityContext>` with `SelinuxCtxState` enum — `<parse-error>` and `<unverifiable>` now distinct from `<unlabeled>`
- **umrs-platform Cargo.toml**: Added `rusqlite 0.31` (optional, rpm-db), `sha2 0.10`, `thiserror 1`; FFI exception documented for rusqlite/libsqlite3-sys
- **umrs-selinux data files**: `secolor.conf` and `setrans.conf` relocated from crate root to `data/` subdirectory
- **umrs-ls**: Updated `ObservationKind` import to use `umrs_selinux` re-export
- **umrs-state**: i18n wiring for 8 translatable strings
- **Documentation structure**: Antora `antora.yml` updated to register `patterns/` module; architecture, reference, and devel modules restructured
- **devel/ module nav**: Updated with pattern subsections and high-assurance pattern reference table; added "Platform Internals" section with OS Detection Pipeline deep-dive
- **patterns/ module nav**: Added "OS Detection: A Trust Ladder" under "Patterns — Verification Pipelines"
- **Technical documentation**: Multiple .txt/.md files converted to .adoc across modules; originals quarantined to `docs/_scratch/`
- **i18n developer guide**: Content merged from `resources/i18n/developer-guide.md` into `devel/pages/i18n.adoc`
- **OS Detection documentation alignment**: All illustrative code snippets in both pattern and deep-dive documents corrected to match current `umrs-platform` source (EvidenceRecord fields, phase signatures, ConfidenceModel API, Phase 4 soft-gate behavior, LabelTrust paths, caller contracts)

### Fixed
- **umrs-selinux type validation**: `validate_type()` in `type_id.rs` used `is_ascii_lowercase()` instead of `is_ascii_alphabetic()`, rejecting valid SELinux types like `NetworkManager_etc_t`; both TPI paths shared the bug; fixed with 6 new regression tests (3 type tests, 3 context tests)
- **umrs-platform security audit fixes applied**: RPM-22 (HIGH: `EvidenceBundle::records` made private to enforce AU-10); RPM-01 (replaced nightly `is_multiple_of` API with stable Rust); RPM-02 (rusqlite error Display no longer leaks filesystem paths — SI-12); RPM-07 (added `ArrayLengthMismatch` error for previously silent empty return); RPM-24 (Cargo.toml split `rpm-db` from `rpm-db-bundled` features)
- **Documentation gaps**: High-assurance patterns previously scattered; now consolidated in patterns/ module with unified threat/pattern/example structure
- **i18n architecture clarity**: Rules and workflow documented in devel/ with active domains, feature gates, and locale listings
- **OS Detection pattern documentation**: Pattern page `pattern-os-detection.adoc` — "OS Detection: A Trust Ladder" targeting auditors and newcomers; covers 7-phase trust ladder (T0–T4), threat model, Mermaid diagram, LabelTrust verdict table, EvidenceBundle rationale, and security controls mapping
- **OS Detection deep-dive documentation**: Developer guide `os-detection-deep-dive.adoc` covering per-phase sections (What/Why/Code/Controls), state machine diagram, ConfidenceModel and EvidenceBundle architecture, LabelTrust caller contract, sealed memory cache design, and API reference table
- **Documentation Sync Rule**: New rule in `.claude/rules/agent_behavior_rules.md` requiring `rust-developer` to create `doc-sync:` tasks for `tech-writer` when OS detection public APIs, phase logic, or type definitions change

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
