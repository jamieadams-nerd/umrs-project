# Changelog

## 2026-03-19

### Added
- **Review routing restructure**: Created `docs/sage/reviews/` and `docs/imprimatur/reviews/` directories with naming convention `YYYY-MM-DD-<type>-<slug>.md` for traceability; established `docs/sage/inbox/` and confirmed `docs/imprimatur/inbox/` as content pickup points; archived blog audit from `.claude/reports/` to `docs/sage/reviews/2026-03-19-blog-cui-sign-lock.md`
- **Team collaboration documentation**: Updated `.claude/team-collaboration.md` with review routing rules, directory ownership, and auditor's tiered review format (ACCURATE/CONCERN/ERROR)
- **Vale documentation quality plan**: Created `.claude/plans/doc-quality-plan.md` — 5 UMRS Vale styles mapped to existing rule sources (STE, Citations, Terminology, Blog, Admonitions); 7 Vale rule types identified; Phase 1 ruleset bootstrap ready
- **Researcher knowledge acquisition plan**: Created `.claude/plans/researcher-knowledge-acquisition.md` combining three brain corpora: information theory + graph theory + search foundations (Shannon, MacKay, Edmonds, HNSW); Sage outreach knowledge base (SEO, developer trust, technical branding, content strategy); 12 skills across 6 categories (lifecycle, cross-reference, QA, synthesis, acquisition automation, team service)
- **UMRS Tool Init plan refactored**: Broke 1,347-line monolith into 8 swim-buddy-safe sub-phases (1a through 3) with execution order table, 45-minute heartbeat rule, and parallel opportunity notes; enriched env var corpus research with authoritative source table; added Appendix on String Sanitization Doctrine
- **High-assurance library backlog**: Created `.claude/plans/long-term/high-assurance-library-backlog.md` — 15 libraries across 6 domains (Filesystem Trust, Process & Privilege, System Topology, Configuration & Policy, Security Observability, Cryptographic Ops) with problem statements, API shapes, UMRS overlap, and complexity estimates
- **Rust FIPS cryptography strategies**: Created `docs/modules/cryptography/pages/rust-fips-strategies.adoc` documenting three approaches (OpenSSL FFI, rustls+aws-lc-rs, pure Rust crypto) with UMRS project position
- **Blog post audit remediation**: Applied all MUST-FIX items from security-auditor review — E-1 `#![forbid(unsafe_code)]` scope qualified with umrs-hw exception, C-1/C-2/C-3 country/compliance annotations qualified, C-6 country profiles labeled as Phase 2, GitHub URL corrected
- **File cuddling plan**: Created `.claude/plans/umrs-ls-file-cuddling.md` — compact view grouping for umrs-ls (3 phases, implement before TUI)
- **Xattr sanitization gap plan**: Created `.claude/plans/xattr-sanitization-gap.md` — prove shred doesn't touch xattrs, build stripping tool, document gap (4 phases)
- **Task created**: `doc-sync: devel guide — document crate dependency rationale` for tech-writer

### Changed
- **Cryptography module nav**: Updated `docs/modules/cryptography/nav.adoc` to include rust-fips-strategies page
- **Blog post edits**: Applied security-auditor feedback to `docs/sage/blogs/blog-cui-sign-lock.adoc` — all compliance-level and factual corrections now in place
- **Settings permissions**: Updated `.claude/settings.json` with expanded access for new subdirectory structure

### Removed
- **10 jamies_brain files archived**: Moved `install-vale.md`, `info-theory.txt`, `sage-food.txt`, `help-rearcher.txt`, `more_env_stuff.txt`, `sanitization.txt`, `scrub-strings.txt`, `other-possible-features.txt`, `rust-openssl-fips.txt`, `xattr-stripper.txt` to `.claude/jamies_brain/archive/`; `doc-restructure.md` and `file-cuddling-umrs-ls.md` converted to plans
- **docs/new_stuff retirement**: Transitioned from `docs/new_stuff/` to `docs/imprimatur/inbox/` and `docs/sage/inbox/` routing pattern

## 2026-03-18

### Added
- **CPU Security Corpus Phase 1A COMPLETE**: 6 reference files written to `.claude/references/cpu-extensions/crypto-accel/` covering AES-NI, VAES, SHA-NI, PCLMULQDQ, ARM crypto equivalents, and phase summary. Full 23-column matrix profiles with CVE tables, /proc/crypto driver mappings, FIPS utilization requirements, and software fallback risk analysis.
- **CET documentation COMPLETE**: 4 reference files written to `.claude/references/cpu-extensions/cet-docs/` covering Intel CET spec summary, Linux kernel CET support, RHEL 10 CET status (including critical Rust CET gap finding), and binary verification guide. Unblocks Phase 1F.
- **NIST SP 800-90B acquired**: PDF saved to `refs/nist/sp800-90B.pdf` with manifest entry updated. Unblocks Phase 1B.
- **Phases 1C, 1D, 1E research COMPLETE**: Comprehensive data gathered on vector extensions (9 features), TEE/confidential computing (10 features + ARM TrustZone), and speculative execution mitigations (8 features). 27 reference files pending write next session.
- **CPU corpus preflight checklist**: Created `.claude/plans/cpu-corpus-next-session-preflight.md` documenting all pending write operations and memory saves for next session.
- **CPU Security Corpus Phases 1E–1F COMPLETE**: Wrote pcid.md + vulnerability-sysfs-reference.md (Phase 1E); wrote cet-shadow-stack.md, cet-ibt.md, umip.md, pku.md, arm-access-controls.md (Phase 1F). RAG ingested `cpu-extensions` collection — 645 chunks from 60 files. Phases 1I/1J/1K actionable items folded into cpu-extension-probe plan.
- **Rusty CPU probe review**: Plan review at `.claude/reports/cpu-probe-openssl-plan-review.md` — identified 4 gaps in CPU probe, 3 in OpenSSL plan, proposed shared `ElfInspector` abstraction to eliminate duplication.
- **OpenSSL posture module plan drafted**: `.claude/plans/openssl-posture-module.md` — 3-layer detection (library presence, FIPS mode, algorithm support) with integration into umrs-platform. Approved use of `goblin` crate for ELF parsing.
- **Long-term plans directory created**: `.claude/plans/long-term/` for distant-horizon plans (project-restructure, assessment-engine, logspace-design).
- **CPU/crypto artifact absorption**: 4 active scripts moved to `.claude/agent-memory/rust-developer/reference/cpu-crypto/`; 7 analysis docs archived to `.claude/jamies_brain/archive/cpu-experiments/`; cpu-work/ and create_ima_keys.sh removed from jamies_brain.
- **Antora documentation theme COMPLETE (Wizard Edition)**: Full replacement of default steel-blue Antora theme with dark-first green monochrome design. Complete CSS rewrite with dark/light toggle (localStorage persistence), vendored Inter + JetBrains Mono WOFF2 fonts for network-isolated deployments, MIL-STD-38784B admonition hierarchy with color-coded severity levels (red=WARNING, amber=CAUTION, orange=IMPORTANT, blue=NOTE, green=TIP) and Unicode symbols, terminal-style code blocks, compact navigation, green square bullets, wizard logo header, minimal footer. Patched `antora-ui-default.zip` removing all default light-theme artifacts. Accessibility: high-contrast, print, reduced-motion support. Plan: G8/G10, retired to `completed/`.

### Changed
- **CPU security corpus plan moved to completed**: Marked COMPLETE and archived to `.claude/plans/completed/cpu-security-corpus-plan.md` after Phases 1E–1F completion and artifact absorption.
- **umrs-platform-expansion.md split**: Separated into `umrs-platform-posture-and-cross-platform.md` (active focus) and `cpu-extension-probe.md` (active, CPU-specific development).
- **cpu-extension-probe.md status**: Updated to reflect corpus completion, Rusty review findings, and integration with OpenSSL posture module plan.
- **Settings.json permissions expanded**: Added absolute-path Write and Edit permissions for `.claude/**` and `refs/**` to support subagent compatibility on write operations.
- **CPU security corpus plan updated**: All phase statuses reflected in `.claude/plans/cpu-security-corpus-plan.md` — Phases 1A, CET, 1C-1E research all marked complete or in-progress.
- **Agent memory updates**: Saved 3 new memory artifacts (CPU corpus push reminder, pre-create output dirs feedback, CET runtime verification finding) to researcher agent memory; saved feedback on no-subprocess rule to memory.
- **Signal → Indicator terminology eradicated**: Removed from all active files — 2 adoc docs, 2 Rust source comments, 3 plans, 1 review report, 1 familiarization doc, 11 reference corpus files. Historical reports preserved as-is.

### Fixed
- **RHEL 10 Rust CET gap identified**: Found that Rust stable compiler lacks CET support (tracked at rust-lang/rust#93754); UMRS binaries will not have shadow stack protection on RHEL 10. Classified as INFORMATIONAL; Rust memory safety provides alternate CFI mitigation.
- **Stray .claude/settings.local.json deleted**: File violated Settings Files Hard Rule; removed from working tree.
- **Rogue .claude/plans/.claude/ directory removed**: Empty directory eliminated; no content loss.

## 2026-03-17

### Added
- **UMRS Tool Initialization API plan**: `.claude/plans/umrs-tool-init.md` — comprehensive environment audit and validated accessors for tool startup; SanitizedEnv validated constructor with 8 validation classes (PATH, LD_PRELOAD, locale, network, auth, TZ, shell, crypto); 30+ environment variable denylist (including CVE-2023-4911 GLIBC_TUNABLES mitigation); SensitiveValue newtype for confidential vars; journald-native logging with fallback cascade to stderr; i18n auto-detection; security-engineer and security-auditor reviews completed with all findings incorporated; NIST SP 800-53 AU-5 rationale documented
- **Quote of the Day (QOTD) corpus plan**: `.claude/plans/qotd-quotes-corpus.md` — ~55 curated engineering culture quotes with terminal and ratatui popup display; JSON corpus format for easy additions; typography module text effects reuse; designed to reflect team identity and system security posture
- **Antora documentation theme plan**: `.claude/plans/antora-doc-theme.md` — custom wizard-motif dark-first design with two-register admonition color system (WARNING/CAUTION=alert register, IMPORTANT/NOTE/TIP=neutral register); vendored fonts for offline deployment; accessibility and print-friendly pass; NIST control tag pills for compliance visibility; senior-tech-writer review completed with all findings incorporated
- **Antora multi-component split plan**: `.claude/plans/antora-multi-component-split.md` — split monolithic Antora site into 5 audience-focused components (Project, CUI Labeling, Operations, Development, AI) plus collection home; identified 272 cross-module xrefs requiring migration; defined cross-component linking strategy and audience routing; senior-tech-writer review completed with all findings incorporated
- **XDG Base Directory Specification reference**: `.claude/references/xdg-basedir-spec.md` — specification v0.8 with key finding that ~/.local/bin is a convention, not a formal standard; informs deployment model for tool initialization
- **Security review reports**: Three comprehensive multi-agent reviews completed: `2026-03-17-umrs-tool-init-security-review.md` (security-engineer, 11 findings), `2026-03-17-umrs-tool-init-compliance-audit.md` (security-auditor, 12 findings), and `multi-component-split-review.md` (senior-tech-writer, 12 findings)
- **Agent memory files**: Created independent memory files for researcher, security-auditor, and security-engineer agents on specialized topics (corpus conventions, annotation debt, citation mapping, denylist reference, unsafe boundary knowledge, XDG reference)
- **Signal → Indicator terminology refactor (Phase 2)**: Renamed `define_sysctl_signal!` macro to `define_sysctl_indicator!` (13 invocations); renamed 3 functions (append_signal_group, signal_group_rows, read_live_cmdline_signal) to indicator equivalents; renamed 4 variables (boot_signals, new_signals, blacklist_signals, modprobe_signals) to indicator equivalents; updated ~170 doc comments and inline comments across umrs-platform/src/posture/, umrs-platform/tests/, umrs-platform/examples/, and umrs-tui/src/main.rs — completes Phase 2 of Signal→Indicator refactor
- **Source code comment citations normalized**: Fixed `NIST 800-53` → `NIST SP 800-53` and `NIST 800-218` → `NIST SP 800-218` across 18 Rust files (umrs-selinux, umrs-platform, umrs-core, umrs-tui); removed 3 internal review references from umrs-platform/tests/posture_tests.rs per Internal Reference Prohibition Rule
- **Doc sections added — `# Errors` and `# Panics`**: Removed `#![allow(clippy::missing_errors_doc)]` from umrs-platform, umrs-selinux, umrs-hw; removed `#![allow(clippy::missing_panics_doc)]` from umrs-selinux; added 61 `# Errors` doc sections across 19 source files in umrs-platform and umrs-selinux; added `# Panics` doc section to load_secolors_cached() in umrs-selinux — all clippy clean, all tests pass
- **SCAP/STIG corpus collection**: Confirmed `scap-security-guide` collection in ChromaDB (7 chunks, 451 signals); researcher familiarization complete; identified chunking limitation and documented workaround (direct file reads for oversized markdown tables)
- **Security auditor corpus Phase 3 — Technical Compliance**: Cross-referenced 36 posture indicators against 451 STIG rules; 20/36 indicators have direct STIG rule matches with CCEs; 10 CPU mitigation indicators exceed STIG baseline (UMRS value-add); identified gaps: 51 auditd rules (zero coverage), 19 network sysctl rules (zero coverage); identified 7 Tier-1 candidate new indicators (BpfJitHarden, NetIpv4Forwarding, NetIpv4AcceptRedirects, NetIpv4TcpSyncookies, CanModuleBlacklisted, SctpModuleBlacklisted, TipcModuleBlacklisted); coverage report written to refs/reports/stig-signal-coverage.md
- **Agent permissions**: Added `Write(components/**)` and `Edit(components/**)` to .claude/settings.json for agent permissions on workspace components
- **Source code comment cleanup Task 3 COMPLETE**: Von Neumann reviewed all 5 crates (umrs-selinux, umrs-platform, umrs-core, umrs-ls, umrs-tui) producing 84 findings (18 HIGH, ~31 MEDIUM, ~35 LOW); Rusty fixed all HIGH and MEDIUM findings; consolidated report at `.claude/reports/2026-03-17-module-comment-review.md`
- **Module Documentation Checklist Rule**: Added to `.claude/rules/rust_design_rules.md` — every `.rs` file must have `//!` block with purpose, key types, and `## Compliance` section; names `secure_dirent.rs` and `posix/primitives.rs` as exemplary templates
- **`cargo xtask doc-check` command**: Mechanical gate that scans all `src/**/*.rs` files and fails if any lack a `//!` module doc block; identified 24 remaining files with documentation debt in prototype/utility crates
- **New-crate skill updated**: `//!` block template now includes full `## Compliance` section placeholder with instructions to fill in actual controls; removed stale `missing_errors_doc`/`missing_panics_doc` suppressions
- **FIPS procfs path bug (umrs-platform)**: Corrected `/proc/sys/kernel/fips_enabled` → `/proc/sys/crypto/fips_enabled` in `sealed_cache.rs`, `detect/integrity_check.rs`, tests, examples, and Cargo.toml comment — was causing fail-open on FIPS detection on RHEL 10
- **umrs-core `fs/mod.rs`**: Added missing `HashSet` import and `ENCRYPTED_FS_TYPES` constant; replaced `unwrap_or_default()` with explicit match; module left unwired (needs SecureReader migration before enabling)
- **umrs-core `audit/events.rs`**: Added `pub mod events;` declaration to `audit/mod.rs` — module was unreachable dead code
- **umrs-tui `keymap.rs`**: Fixed wrong NIST control citation `AC-2` (Account Management) → `AC-12` (Session Termination) in 3 locations
- **NIST citation normalization**: Fixed `NIST 800-53` → `NIST SP 800-53` and `NSA Raise-the-Bar` → `NSA RTB` across umrs-selinux (~15 occurrences in xattrs.rs, mcs/translator.rs, secure_dirent.rs)
- **umrs-selinux doc fixes**: "Catagories" typo, "this crate"→"this module", duplicate sentences in sensitivity.rs, stale filename in posix/identity.rs, added `//!` blocks to xattrs.rs/posix/mod.rs/mls/range.rs, expanded status.rs Trust Gate doc, documented TPI architecture and MlsLevel duplication in context.rs, added compliance citations to 12+ modules
- **umrs-platform doc fixes**: Added ProcfsText mention to kattrs/procfs.rs, clarified findings() in posture/mod.rs, added missing types to lib.rs module table, fixed "a indicator" grammar, added headings to 5 files
- **SCAP/STIG Phase 2 COMPLETE**: All 5 agents familiarized with STIG corpus — rust-developer mapped 13 existing indicators to CCEs and identified 20 new candidates in 3 tiers; security-auditor refined Tier-1 to 13 candidates and proposed 3 composite auditd indicators; security-engineer identified fapolicyd deployment risk and prioritized 14 Tier-1 audit rules; tech-writer defined CCE citation format for Rust/Antora/CLI; senior-tech-writer mapped CCE table to reference/ module with audience routing rules
- **SCAP/STIG Phase 3a COMPLETE**: Added `cce: Option<&'static str>` field to `IndicatorDescriptor` in catalog.rs; populated 13 confirmed STIG matches (CCE-88686-1 through CCE-88971-7); updated 4 NIST citation divergences to STIG-authoritative mappings (KptrRestrict, DmesgRestrict, YamaPtraceScope, UnprivBpfDisabled)
- **SCAP/STIG Phase 3b COMPLETE**: CCE citation format section added to `docs/modules/devel/pages/compliance-annotations.adoc`; 5 approved terms (CCE, SCAP, STIG, RHEL 10 STIG, XCCDF) added to glossary and approved terminology list
- **SCAP/STIG Phase 3d COMPLETE**: STIG methodology comparison page at `docs/modules/architecture/pages/stig-methodology-comparison.adoc` — documents configured-vs-live advantage over STIG's point-in-time checks with 3 concrete cases and 6-indicator comparison table
- **CCE provenance rule**: Added to Citation Format Rule in `.claude/rules/rust_design_rules.md` — CCE canonical form, STIG version recording, conflict flagging requirement
- **Signal→Indicator terminology cleanup**: Fixed all remaining "Signal" references in scap-stig-corpus-plan.md
- **OS-detect kernel-tab enhancement Phase 0 COMPLETE**: Security-auditor produced authoritative plain-language indicator definitions for all 37 indicators, 6 groups, trust evidence tab (symbol meanings, T0-T4 tiers, positive framing for "all gates passed"), header indicators, and identified 9 Phase 2b indicators not yet rendered in TUI with recommended group names (CPU VULNERABILITY MITIGATIONS, CORE DUMP POLICY)
- **CPU security corpus Phase 0.5 COMPLETE**: Verified all 10 spec update actions already present in cpu-matrix.md; bumped to v3 with verification checklist; placed researcher-ready spec at `.claude/references/cpu-extensions/cpu-matrix.md`
- **Tool-init plan addendum**: Incorporated Jamie's env var research (`more_env_stuff.txt`) — systemd runtime vars (INVOCATION_ID, JOURNAL_STREAM, NOTIFY_SOCKET, SYSTEMD_EXEC_PID) for Tier 2 allowlist; container detection vars as informational ScrubReport signals; pre-implementation researcher task to acquire authoritative env var sources (man 7 environ, ld.so, systemd.exec, POSIX)

### Changed
- **jamies_brain archive restructuring**: Archived 11 files/directories (`kernel-probe-grouping.txt`, `rusty-optimize.txt`, entire `asm/` subtree, `env-scrubbing.txt`, `path-scrubbing.txt`, `fun/` subtree, `doc-theme/doc-theme.md`) into `.claude/jamies_brain/archive/` with DO NOT DELETE preservation semantics; files already incorporated into completed plans or existing documentation/skills
- **Agent memory consolidation**: Moved 7 agent memory files from rogue `components/rusty-gadgets/.claude/` directory to correct location at `/.claude/agent-memory/*/`; removed empty rogue directory
- **Project memory updates**: Normalized performance-baseline.md status header to project convention; recorded initial release deployment model (~/.local/bin, XDG paths, no root required); clarified that kernel-probe-grouping.txt source was archived (capability matrix already implemented)
- **Settings permissions**: Fine-tuned `.claude/settings.json` for agent archive operations and future knowledge work
- **Plan status updates**: source-code-comment-cleanup Tasks 1-3 COMPLETE; security-auditor-corpus Phase 3 complete; scap-stig-corpus-plan Phase 1 complete and Phase 3c done; umrs-assessment-engine unblocked (posture probe Phase 2c + auditor corpus both done); high-assurance-writing-guide unblocked (SCAP ingestion done)
- **Plans closed**: `source-code-comment-cleanup` marked COMPLETE; `security-auditor-corpus` marked COMPLETE; `scap-stig-corpus-plan` marked COMPLETE (all phases done)
- **scap-stig-corpus-plan**: Signal→Indicator terminology throughout; Phase 2 and Phase 3 status updated to complete
- **os-detect-kernel-tab-enhancement plan**: Phase 0 marked COMPLETE
- **cpu-security-corpus-plan**: Phase 0.5 marked COMPLETE, Phase 1A next

### Fixed
- **Rogue .claude directories**: Consolidated stray `components/rusty-gadgets/.claude/` with 7 unique memory files and 2 stubs; no duplicate local.settings.json found
- **Memory debt**: Created explicit feedback memory for Jamie to archive jamies_brain files after conversion to plans, preventing accumulation of stale brain-state documents

## 2026-03-16

### Added
- **Kernel security posture signals (Phase 2b)**: 8 CPU mitigation sub-signals added to catalog (SpectreV2Off, SpectreV2UserOff, MdsOff, TsxAsyncAbortOff, L1tfOff, RetbleedOff, SrbdsOff, NoSmtOff) — all KernelCmdline class with CmdlineAbsent desired values; CorePattern signal with dual-path TPI validation and DesiredValue::Custom
- **S-01 HIGH fix**: KernelCmdline contradiction detection functional — collect_one() routes cmdline signals through meets_cmdline() for BLS options evaluation; BootDrift and EphemeralHotfix now fire correctly for all cmdline signals
- **M-03 fix**: Path-traversal validation added to is_module_loaded() and read_module_param() — rejects `/`, `\0`, `..`
- **M-02 fix**: `/usr/bin/false` added to hard-blacklist sentinel test
- **M-01 fix**: Stale scope doc comment in modprobe.rs updated
- **Signal catalog**: Expanded from 27 to 36 signals total (22 Phase 1 + 5 Phase 2a + 9 Phase 2b)
- **Posture tests**: 93 tests passing (up from 78); full workspace clean
- **kernel-probe-signals.adoc**: Authoritative reference page with all 37 signals across 8 groups; what/why/good/bad/controls per signal; contradiction taxonomy; trust evidence explanation
- **cpu-extensions.adoc**: CPU extension reference with 6 groups, three-layer activation model, Mermaid diagram, vendor-canonical naming, FIPS relevance, detection paths
- **update-checklists.adoc**: Kernel version update, CPU extension update, and signal deprecation checklists with step-by-step procedures in STE mode
- **Architectural decisions resolved (umrs-platform-expansion.md)**: CpuSignalId=separate enum, MAC=phase-level decision table, serialization=JSON, timing=after assessment types stabilize
- **platform-api-enrichment.md plan**: Moves labels/descriptions from TUI to umrs-platform; adds SignalGroup enum, description field on SignalDescriptor, i18n-ready strings
- **Security-engineer Phase 2b review**: Complete audit at `.claude/reports/security-engineer-phase2b-review.md` — 8 findings (0 CRITICAL, 1 HIGH, 2 MEDIUM, 5 LOW), all addressed
- **Tech-writer corpus (Phases 1-2)**: 42 reference files acquired (Google/Microsoft style guides 23 files, MIL-STD-38784B, NIST author instructions, GSA Plain Language guidelines); ingested into ChromaDB `tech-writer-corpus` collection (779 chunks)
- **Agent memory consolidation**: Moved 6 rust-developer and security-auditor memory files from rogue `.claude/` directories to repo-root `.claude/agent-memory/`
- **Admonition audit**: 15 admonition corrections across 10 `.adoc` files per MIL-STD-38784B adapted hierarchy (WARNING=security/data-loss, CAUTION=recoverable degradation, IMPORTANT=prerequisite, NOTE=supplementary, TIP=optional)
- **Admonition hierarchy rule**: Created `.claude/rules/admonition_hierarchy.md` — standalone always-active rule defining MIL-STD-38784B adapted hierarchy; not gated behind STE mode
- **posture-probe-internals.adoc**: New developer guide in devel module covering signal taxonomy, CPU mitigation sub-signals, CorePattern TPI walkthrough, and operational rundown
- **pattern-tpi.adoc**: Updated with CorePattern as second TUI codebase example alongside SecurityContext
- **Tech-writer corpus familiarization**: Both writers completed corpus familiarization with style guides and government standards (Google, Microsoft, MIL-STD-38784B, NIST, Plain Language)
- **Common Criteria (CC) acquired**: CC:2022 Parts 1 & 2 (ISO/IEC 15408) from commoncriteriaportal.org; ingested into `tech-writer-corpus` collection (466 chunks)
- **CC glossary entries**: 6 new terms (EAL, PP, SFR, ST, TOE, TSF) added to glossary with CC definitions
- **SCAP/STIG Phase 0+1 complete**: 451 signals extracted from rhel10-playbook-stig.yml; generated `stig-signal-index.md` (451 rows) and `cce-nist-crossref.md` (451 CCEs); ingested into RAG as `scap-security-guide` collection
- **Capability matrix**: Jamie's 7-domain operator-facing groupings incorporated into TUI, kernel probe, and platform expansion plans; technical reference created at `.claude/references/capability-matrix-domains.md`
- **SCAP/STIG ingestion plan**: Created `.claude/plans/scap-stig-corpus-plan.md` — Phases 0+1 complete, Phase 2 (5-agent familiarization) and Phase 3 (CCE annotations, coverage gap report, methodology comparison) pending
- **High-assurance writing guide plan**: Placeholder created at `.claude/plans/high-assurance-writing-guide.md` — shareable guide for teaching agents/humans to write high-assurance documentation
- **CCE cross-referencing requirement**: New project requirement — cite CCE identifiers alongside NIST controls in source code and documentation
- **Settings.json permissions broadened**: Added `Write(docs/**)`, `Edit(docs/**)`, `Write(.claude/**)`, `Edit(.claude/**)`
- **Agent definitions updated**: Both writer agents now have "Always-Active Rules" section pointing to admonition hierarchy
- **umrs-hw crate**: New workspace crate with safe RDTSCP wrapper (x86_64) and CLOCK_MONOTONIC_RAW fallback (aarch64); `read_hw_timestamp() -> u64` public API; `tsc_is_invariant() -> bool` via CPUID leaf 0x80000007; AU-8 annotated; 7 integration tests, 1 example; isolated unsafe boundary (workspace's only crate without `#![forbid(unsafe_code)]`)
- **umrs-platform phase duration measurement**: `DetectionPhase` enum (7 variants), `PhaseDuration` struct, `phase_durations: Vec<PhaseDuration>` added to `DetectionResult`; `duration_ns: Option<u64>` added to `EvidenceRecord` (44 construction sites updated); all 7 detect phases instrumented with hw timestamp pairs; `BootSessionTimestamp` module (CLOCK_MONOTONIC_RAW, nanosecond precision); `BootSessionDuration` type with checked arithmetic; `TimestampError` enum; clock anomaly recorded as downgrade reason in `ConfidenceModel`; 17 timestamp integration tests, timestamp_demo example
- **CLAUDE.md ASM Usage Policy**: Three-gate test framework, permitted use cases (FFI, unsafe boundary, performance-critical SIMD), prohibited patterns (crypto, security-critical logic, branchless secrets), required template; `umrs-hw` added to workspace layout and dependency table
- **ROADMAP expansions**: G11 — Multi-Site Documentation Architecture (6 Documentation Sets as separate Antora sites); G12 — Documentation Theme & Visual Identity (custom dark theme, wizard mascot, security aesthetics); references distributed to serving sites instead of standalone; design principles added (Usability co-equal with security, Explain don't abbreviate)
- **Documentation fixes**: `index.adoc` — 6 typos fixed, xref links added to all Documentation Set names, references row updated; `introduction.adoc` — 3 typos fixed, long line wrapped, Jamie's personal voice preserved; `ROOT/nav.adoc` — broken `getting-started.adoc` reference removed; `ai-transparency/auditor-guide.adoc` and `ai-transparency/index.adoc` — broken xrefs to deleted getting-started.adoc fixed
- **Skills**: `asm-guidance` skill installed at `.claude/skills/asm-guidance/` with SKILL.md, intrinsics-map.md, asm-templates.md
- **Performance engineering corpus acquired**: Phases 1-3 (21 chapters Rust Performance Book, rustc profiling guides, Drepper memory paper, Agner Fog optimization manuals, Brendan Gregg Linux perf, Criterion.rs, flamegraph-rs); ingested into RAG as `performance-corpus` collection (549 chunks); rust-developer familiarization complete — 4 knowledge artifacts produced
- **Agent memory**: Human-centered design principle captured (usability co-equal with security, consistency breeds predictability); Jamie's 35-year engineering background and system card context recorded
- **Performance baseline plan**: Created `.claude/plans/performance-baseline.md` — criterion benchmarks, debug instrumentation timing (via `#[cfg(debug_assertions)]`), and optimization workflow for umrs-platform, umrs-selinux, and umrs-core; timing routes through umrs-platform's `BootSessionTimestamp` (no direct umrs-hw dependency); benchmark implementation deferred to next session
- **Rust performance corpus review**: rust-developer analyzed performance corpus against codebase; identified 8 optimization opportunities (BufRead::lines() allocation, split().collect::<Vec>() overuse, HashMap::with_capacity missing, Vec<char> hex decode, release profile LTO); confirmed 4 good patterns already in use (CategorySet fixed-size, boxed enum variants, BufReader, checked_* arithmetic); flagged potential `unwrap_used` violation in validate.rs:97 for remediation

### Changed
- **Tech-writer cross-team note**: Doc-sync request for Phase 2b posture marked resolved; CPU Mitigation Sub-Signals section, CorePattern TPI Validation section, signal count update (27→37), and SEC caching deferred note confirmed present in `posture-probe-internals.adoc` and `pattern-tpi.adoc`; `make docs` passes
- **Infrastructure**: Removed empty rogue `.claude/` directory at `components/rusty-gadgets/.claude/`
- **ai-transparency/nav.adoc**: Removed duplicate "AI in This Project" entry; restructured for collapsibility
- **Navigation audit**: "AI in This Project" duplicate removed from ROOT/nav.adoc
- **Style decisions resolved**: SDR-009 (third person for architecture, second person for procedures), SDR-010 (inclusive terms in narrative, standard terms in specs, editorial notes for verbatim quotes), SDR-011 (CC SFR prose register — two registers coexist)

### Fixed
- **4 rogue `.claude/` directories consolidated**: jamies_brain/.claude, plans/.claude, rag/.claude, components/rusty-gadgets/.claude — unique files merged to repo-root, rogue dirs removed; all permissions already covered by `.claude/settings.json` wildcards

## 2026-03-15

### Added
- **Settings & Infrastructure**: WebFetch permissions for nvlpubs.nist.gov and fedramp.gov; Bash(pandoc:*) permission for DOCX template conversion
- **Accreditation artifacts RAG collection**: Downloaded 6 of 8 NIST/FedRAMP documents; converted 3 DOCX templates to .txt via pandoc; ingested 405 chunks
- **FedRAMP reference directory**: Created `refs/fedramp/` with corrected URLs and 3 playbook/template documents
- **Security-auditor corpus familiarization**: Completed Phase 1–2 of security-auditor-corpus.md plan; 5 knowledge artifacts (policy-landscape, framework-relationships, audit-procedures, maturity-model-guide, fedramp-playbook-guide)
- **AI Transparency module (ai-transparency/)**: New Antora module with 13 pages covering agent roles, knowledge pipeline, RAG collections, corpus familiarization, knowledge provenance, skills catalog, auditor guide, case study, feedback/rules, and workflow mechanics; registered in antora.yml and ROOT nav
- **ROADMAP**: Added G10 (AI Transparency) goal and M5 milestone; added tool enhancement brainstorm (umrs-file-stat, umrs-os-detect, report export, integrated help, TUI interaction design, wizard mascot concept)
- **TUI/CLI reference corpus**: Acquired 12 reference items (ratatui website, API docs v0.30.0, examples, CLIG guidelines, NO_COLOR standard, clap docs/cookbook, ratatui ARCHITECTURE.md, BREAKING-CHANGES.md, color-eyre, awesome-tuis, awesome-ratatui, crossterm API docs); saved to `.claude/references/tui-cli/` (27 files across 6 subdirectories)
- **TUI/CLI RAG collection**: Ingested `tui-cli` collection into ChromaDB — 262 chunks indexed
- **TUI/CLI knowledge artifacts (rust-developer)**: Produced 3 knowledge files in `.claude/knowledge/tui-cli/`: concept-index (9 core concepts), cross-reference-map (22 connections), style-decision-record (10 rulings + 3 design decisions)
- **TUI/CLI corpus familiarization (security-auditor)**: Completed Phase 1–2; audit memory saved to `.claude/agent-memory/security-auditor/tui-cli-corpus.md`
- **TUI/CLI design decisions**: PH-1 (Async tokio event loops — approved), PH-2 (Keyboard-only interaction for now), PH-3 (Shell completions via clap_complete for bash/zsh/fish — approved); JSON output (`--json`) established as future standard for all tools
- **Settings consolidated**: Updated `.claude/settings.json` with consolidated WebFetch (domain:*) and added Write(.claude/references/**)
- **Agent aliases**: Researcher assigned "The Librarian"; senior-tech-writer assigned "The Imprimatur"; tech-writer assigned "Von Neumann"; umrs-translator assigned "Simone"; guest-coder assigned "Summer Intern"

### Changed
- **FedRAMP reference URLs corrected**: Updated broken URLs in SOURCE.md and refs/manifest.md (Rev5 reorganization)
- **Security-auditor-corpus.md plan**: Phases 1 and 2 marked complete; Phase 3 in progress
- **ROADMAP**: Added mood-reactive wizard mascot concept (Unix Magic poster heritage) with tone/informal elements guidance

### Fixed
- **Removed redundant ROOT/pages/ai-transparency.adoc**: Module index is now the single source for ai-transparency content
- **FedRAMP documents**: Marked 2 training PDFs (SAP/SAR) as permanently unavailable (removed from fedramp.gov in Rev5)
- **"Tooling" section in ai-transparency index**: Identified Claude Code as primary platform; ChatGPT/Gemini as side research tools

## 2026-03-14

### Added
- **Developer guide introduction**: Added devel module intro page covering high-assurance design principles, pattern library reference, and Rust workflow guidance
- **Ripgrep capability**: Added ripgrep (rg) search capability to Claude Code agent for efficient source code discovery across workspace
- **Cargo metadata search**: Added cargo metadata search skill to rust-developer agent for crate dependency analysis and workspace structure queries
- **UML diagram generation from Rust code**: New skill enabling creation of UML diagrams from Rust source code for architectural documentation and type relationship visualization
- **Security-engineer review report (Phase 2a)**: Completed security review of umrs-platform posture module; report saved to `.claude/reports/posture-security-review-phase2a.md`; documented 7 findings (1 HIGH, 4 MEDIUM, 2 LOW) with remediation guidance
- **Tech-writer source comment review report**: Audited all 37 .rs files in umrs-platform/src/ for clarity and compliance; report saved to `.claude/reports/umrs-platform-comment-review.md`; identified 11 fixes across citation format, bare #[must_use], and source comment discipline

### Changed
- **Documentation structure reorganized**: Further refinement of Antora module hierarchy and navigation following Phase 4 restructure completion
- **Antora navigation deduplicated**: Comprehensive audit and cleanup of all 12 nav.adoc files; removed 8 duplicate cross-module entries that caused sidebar items to appear twice
- **ROOT/nav.adoc**: Removed cross-module references to Architecture, Security Concepts, Deployment, Development, Patterns, UMRS Tools, Operations, Logging/Audit, Cryptography, Reference, and Glossary — these modules have their own nav sections; added `what-is-umrs.adoc` to Introduction section
- **architecture/nav.adoc**: Removed Design Rationale section (moved to devel); removed 4 security-concepts cross-refs (reference-monitor, rtb-vnssa, integrity-and-provenance, truth-concepts) — these belong in security-concepts module per VISION.md
- **devel/nav.adoc**: Added Design Rationale section with architecture cross-refs (rationale-strongly-typed, library-model) and consolidated language choice rationale (was duplicated as both top-level item and architecture cross-ref)
- **operations/nav.adoc**: Removed git-commit-signing cross-ref (owned by devel) and "UMRS Tools →" pointer (separate top-level section)
- **glossary/nav.adoc**: Removed duplicate index entry (was listed as both section header and list item)
- **umrs-platform source citations**: Fixed 8 citation format violations in posture/catalog.rs (NIST 800-53 → NIST SP 800-53), sealed_cache.rs, fips_cross.rs, modprobe.rs, snapshot.rs (NIST 800-218 → NIST SP 800-218 SSDF); fixed 4 SSDF PW citations in kattrs/selinux.rs and kattrs/security.rs to NIST SP 800-218 SSDF PW.4.1 canonical form
- **umrs-platform source corrections**: Fixed 2 factual errors (FIPS SP 800-90B → NIST SP 800-90B); removed per-constant NIST citations from kattrs/traits.rs per Source Comment Discipline Rule; merged duplicate SA-8 citation in detect/mod.rs
- **sealed_cache.rs #[must_use]**: Added descriptive message string to bare #[must_use] annotation on status() return type

### Fixed
- **Broken xref in getting-started.adoc**: Updated `architecture:rationale.adoc` → `architecture:rationale-strongly-typed.adoc` after deleting redundant summary page

### Removed
- **architecture/pages/rationale.adoc**: Deleted redundant summary page that only pointed to `rationale-strongly-typed.adoc` (already in nav via Design Rationale section)

## 2026-03-13

### Added
- **nist-pqc RAG collection expanded**: 10 supplementary web articles added to `.claude/references/nist-pqc/web/`; collection grew from 209 to 264 chunks (+55). New files: cloudflare-pqc-standards.md, nist-pqc-announcement-2024.md, hklaw-pqc-standards-2024.md, serverion-pqc-standards-en.md, serverion-pqc-standards-no.md, csrc-nist-pqc-project.md, csrc-nist-pqc-standardization.md, wolfssl-fips-203-204-205.md, csa-fips-203-204-205-quantum-safe.md, sectigo-pqc-algorithm-winners.md, terraquantum-pqc-standards.md. Sources: Cloudflare, NIST, Holland & Knight, Serverion, NIST CSRC, wolfSSL, Cloud Security Alliance, Sectigo, Terra Quantum.
- **PQC documentation task**: `.claude/plans/pqc-documentation-task.md` created for senior-tech-writer and tech-writer — covers PQC emergence, FIPS 203/204/205 detail, algorithm replacement mapping (RSA/ECDH/ECDSA → ML-KEM/ML-DSA/SLH-DSA), and developer awareness additions to crypto docs
- **RAG pipeline bug documented**: `--source` flag does not exist in ingest.py; correct invocation uses `--collection <name>` — recorded in researcher MEMORY.md
- **Plan created**: `.claude/plans/docs-new-stuff-crypto-and-navbar.md` — comprehensive analysis of `docs/new-stuff/crypto.md` and `docs/new-stuff/left-navbar.md` with disposition decisions, overlap assessment, and task specifications
- **PQC Status Tracker created**: `.claude/agent-memory/researcher/pqc-tracker.md` — living team-readable document tracking NIST PQC program milestones, RHEL 10 availability (10.0 preview, 10.1 GA, FIPS partial), FIPS hybrid exceptions, and 16 monitoring sources (NIST, Red Hat, cryptographic conferences)
- **nist-pqc collection expanded to 285 chunks**: Added 2 FIPS PDFs (FIPS 203/204/205 references) plus 12 web articles covering NIST standardization, RHEL PQC roadmap, crypto-safety guidance, and algorithm reference implementations
- **Cryptography module created**: New Antora module `docs/modules/cryptography/` housing 6 pages (fips-cryptography-cheat-sheet, key-recommendation-list, openssl-no-vendoring, crypto-post-quantum, crypto-policy-tiers, crypto-cpu-extensions) moved from `reference/` via git mv; module landing page (`index.adoc`) and collapsible nav with Classical Baseline / Post-Quantum / Deployment groupings
- **crypto-post-quantum.adoc expanded**: New "The Quantum Threat" section (Shor's algorithm, Grover's algorithm, harvest-now/decrypt-later, CRQC timeline, NIST standardization history); "Algorithm Replacement Mapping" table (RSA/ECDH/ECDSA/DSA → ML-KEM/ML-DSA); SLH-DSA CNSA 2.0 exclusion note; ML-KEM KEM vs NIKE API note; Migration section with NIST IR 8547 deprecation timeline, FIPS 206/HQC development status, FIPS provider requirement, and hybrid deployment guidance; SI-7 added to control mapping; new "RHEL 10 PQC Availability" section with status table (10.0/10.1/FIPS), hybrid exception detail, source footnotes
- **Cryptographic Usage Map created**: New page `crypto-usage-map.adoc` mapping 13 OS-level crypto subsystems (IMA/EVM, dm-crypt, journald, shadow, RPM signing, SSH, TLS, SELinux, DNSSEC, Kerberos, kernel module signing, audit hashing, systemd-homed) and 5 planned UMRS usage items (umrs-crypto crate, SEC pattern HMAC, CA bundle validation, audit log chaining, threat model assessment)
- **Glossary expanded**: Added 13 new cryptography terms (Asymmetric Cryptography, CRQC, Crypto Policy, Digital Signature, FIPS Mode, Grover's Algorithm, HMAC, Hybrid Cryptography, KDF, KEM, RSA, Shor's Algorithm, Symmetric Cryptography)
- **Researcher standing refresh instructions**: Added to researcher MEMORY.md for periodic nist-pqc collection library updates using ingest.py

### Changed
- **ROOT navigation restructured**: Flat navigation replaced with grouped structure — Getting Started at top, Introduction group (merged pages), logical module groupings, AI/Legal/Release Notes moved to bottom
- **introduction.adoc merged with what-is-umrs.adoc**: Content from `what-is-umrs.adoc` incorporated into `introduction.adoc` to create single Introduction page; file preserved but removed from nav
- **architecture/nav.adoc consolidated**: "Existing OS Technologies" and "Justification and Case Studies" merged into single "Historical Context" section
- **security-concepts/nav.adoc**: Added "Foundations" and "Integrity and Trust" section groupings
- **fips-cryptography-cheat-sheet.adoc enhanced**: Added referenced standards preamble (FIPS 140-3, 203, 204, 205, SP 800-131A) and hash use case NOTE documenting audit log chaining and file integrity verification
- **crypto-policy-tiers.adoc improved**: Enhanced "Typical use" column entries in KDF table
- **antora.yml**: Registered new cryptography module in navigation
- **reference/nav.adoc and reference/pages/index.adoc**: Updated to reflect 6 pages moved to cryptography module
- **Deployment documentation**: Added Cryptographic Policy section to `rhel10-packages.adoc`

### Fixed
- **docs/new-stuff source files processed**: `crypto.md` and `left-navbar.md` moved to `docs/new-stuff/used/` after analysis and disposition
- **Cross-module xrefs updated**: All references to moved crypto pages corrected across glossary, devel, architecture modules

## 2026-03-12

### Added
- **Antora doc restructure — Phases 2–4 complete**: All four phases of `.claude/plans/antora-doc-restructure.md` complete; plan and `doc-vision.md` archived to `.claude/plans/completed/`
- **Patterns module taxonomy (Phase 2)**: All 16 standard pattern pages updated with two-zone page structure (`== Why This Pattern Exists` / `== The Pattern`); provenance badges (NOTE admonitions with NIST/RTB citations) added to TPI, TOCTOU, Provenance, Non-Bypassability, Fail-Closed, Zeroize; `patterns/nav.adoc` reorganized into 5 groups (Architectural, Coding Techniques, Observability, Process, Deep Dives); pattern index table expanded with Sub-group and Concept basis columns; cross-links added between 8 pattern pages and `security-concepts/` pages; reverse links added to `reference-monitor.adoc`, `integrity-and-provenance.adoc`, `rtb-vnssa.adoc`
- **ROOT pages written (Phase 3)**: `what-is-high-assurance.adoc` — six high-assurance properties table, HA vs traditional comparison, real-world domains, HACAMS lineage (from README.md); `what-is-umrs.adoc` — platform table, MLS label hierarchy, CUI handling, control mapping philosophy (from UMRS-PROJECT.md); `ai-transparency.adoc` — agent roles table (8 agents), AI responsibilities, review requirements, auditor guidance
- **Glossary populated (Phase 3)**: `glossary/pages/index.adoc` — 27+ definitions across three groups: Assurance & Integrity (Assurance, Attestation, Auditability, Authenticity, Chain of Custody, Custody Trust vs Content Trust, Integrity, Integrity Assurance, Non-repudiation, Provenance); SELinux & MLS (Audit Event, Category Set, CUI, MAC, MLS Range, Policy Enforcement, Reference Monitor, Security Context, Security Label, Sensitivity Level); Cryptography (AES, Authenticated Encryption, Cipher Mode, DRBG, ECDH, ECDSA, FIPS, Hash Function, HKDF, ML-DSA, ML-KEM, PQC, SLH-DSA)
- **Crypto reference pages filled (Phase 3)**: `reference/pages/crypto-post-quantum.adoc` — ML-KEM parameter table (512/768/1024), ML-DSA parameter table (44/65/87), SLH-DSA variants, migration approach, SC-12/SC-13 mapping; `reference/pages/crypto-policy-tiers.adoc` — 4-tier framework (Preferred/Approved/Baseline/Disallowed), 8 algorithm category tables (hash, symmetric, cipher modes, MAC, signatures, KEM, KDF, RNG), disallowed list
- **SELinux reference pages rewritten (Phase 4a)**: All five pages converted from raw Markdown to proper AsciiDoc using `umrs-selinux/src/` Rust `///` doc comments as authoritative source — `selinux/sensitivity.adoc` (Bell-LaPadula rules table, SensitivityLevel type, kernel source correspondence), `selinux/category_set.adoc` (dominance truth table, dense vs sparse ebitmap design deviation, kernel equivalence table), `selinux/user.adoc` (POSIX vs SELinux identity, SelinuxUser validation), `selinux/role.adoc` (RBAC layer, object_r note, SelinuxRole validation), `selinux/security_type.adoc` (domain vs type distinction, mixed-case naming rule matching kernel policy parser, SelinuxType validation)
- **Approved terminology list created**: `.claude/approved_terminology.md` — preferred English forms and do-not-abbreviate rules; key decisions: `security context` (preferred, five-component label), `sensitivity level` (preferred; "sensitivity label" non-preferred), `security label` (colloquial synonym for security context — do not use as primary term), `high-assurance` (never abbreviate to "HA" — reads as high-availability)
- **Cross-team terminology reconciliation**: Three vocabulary files aligned — `approved_terminology.md`, `glossary/index.adoc`, `.claude/agent-memory/umrs-translator/vocabulary.md`; removed `sensitivity label` (étiquette de sensibilité) as non-preferred; added `sensitivity level` (niveau de sensibilité); moved `security label` and `MLS label` to removed/non-preferred section

### Changed
- **Cross-module links wired (Phase 4b)**: `deployment/pages/index.adoc` now links to operations, logging-audit, and glossary; `operations/pages/index.adoc`, `logging-audit/pages/index.adoc`, `security-concepts/pages/index.adoc`, `architecture/pages/index.adoc` all have Related Modules / See Also sections; devel ↔ patterns wired via nav.adoc
- **Translator vocabulary corrected**: Removed non-preferred entries (`sensitivity label`, `security label`, `MLS label`); added `sensitivity level` | `niveau de sensibilité`
- **`what-is-umrs.adoc` terminology fix**: "Every process and file has a security label" corrected to "Every process and file has a security context"

### Fixed
- **`reference/selinux/context.adoc` rebuilt**: Previous version used Markdown `##` headers (rendered as literal text in Antora); stated four fields (missing category set); rebuilt as proper AsciiDoc with correct five-component structure and xrefs prefixed with `selinux/` for subdirectory resolution
- **SELinux reference page xrefs**: Sibling xrefs in `context.adoc` corrected from bare filenames to `selinux/` prefix (Antora resolves from module `pages/` root, not current page subdirectory)

### Removed
- **15 superseded `_scratch` files deleted**: `aide-README.md`, `chain-intro.txt`, `chain-verify-sign.txt`, `HACAMS.md`, `kernel-files-TPI.md`, `logging-capacity.txt`, `log-lifecycle-model.txt`, `log-tuning.txt`, `nom_parser.md`, `RATIONALE_for_HA.adoc`, `rhel10-openscap.txt`, `RTB.md`, `rust-must-use-contract.md`, `TW0-NETIF-JUSTIFICATION.md`, `notes/terminology.txt` — all superseded by promoted Antora pages
- **Rogue `.claude/` directories removed**: `.claude/.claude/` (stray `settings.local.json` — all permissions already covered by wildcards in `.claude/settings.json`; stale MEMORY.md copy); `docs/.claude/` (empty)


- **CMMC Final Rule (32 CFR Part 170)**: Downloaded 89 FR 83092 (Oct 15, 2024, effective Dec 16, 2024) to `refs/dod/cmmc-32cfr170-final-rule.pdf`; ingested into RAG collection `cmmc` (282 chunks)
- **CMMC Assessment Guide Level 2 v2.13**: Downloaded Sep 2024 edition from dodcio.defense.gov to `refs/dod/cmmc-assessment-guide-l2.pdf`; ingested into RAG collection `cmmc` (263 chunks)
- **NIST SP 800-171A Rev 3**: Downloaded assessment procedures companion to `refs/nist/sp800-171Ar3.pdf`; ingested into `nist` RAG collection
- **DoD 5200.01 Information Security Program** (5 documents from esd.whs.mil): DoDI 5200.01, DoDM 5200.01 Volumes 1-3, DoDI 5200.48 (CUI); all downloaded to `refs/dod/`, ingested into RAG collection `dod-5200` (360 chunks)
- **RAG collections**: 3 new collections — `cmmc` (545 chunks), `dod-5200` (360 chunks), `rustdoc-book` (194 chunks), `asciidoctor-ref` (67 chunks), `dita-spec` (100 chunks); `nist` collection expanded from 461 to 1,447 chunks
- **Cross-team note**: Reference document topology guide — explains three-layer reference system (`rag-query` → `.claude/references/` → `refs/`) for all agents
- **Approved source**: `esd.whs.mil` (DoD WHS Issuances portal) added to researcher approved sources
- **Documentation restructure plan**: Comprehensive 5-phase plan at `.claude/plans/antora-doc-restructure.md` aligned with vision document (`.claude/jamies_brain/doc-vision.md`); phases: 0 (Audit — completed), 1 (Structural foundation — completed), 2 (Patterns taxonomy), 3 (New content), 4 (Curation); 5 phase tasks created on task board with blocking dependencies
- **Phase 0 audit report**: Audited 130 Antora pages, mapped to doc-vision domains; triaged 46 `docs/_scratch/` files (12 delete, 19 promote, 15 discard); triaged 5 `docs/modules/deployment/pages/_archive/` files (all superseded); assessed README/UMRS-PROJECT/UMRS-PLAN for content extraction; migration manifest at `.claude/reports/phase0-migration-manifest.md`
- **Phase 1 structural changes**: Deleted empty `security-compliance/` module; deleted duplicate `architecture/umrs-prog-lang.adoc`; moved `structured-logging.adoc` and `how-to-structure-log.adoc` to `logging-audit/`; promoted cryptography subdirectory pages to `reference/pages/` top-level; deleted all 5 files in `deployment/pages/_archive/`; created `glossary/` module skeleton; created 6 placeholder pages (ROOT: `what-is-umrs.adoc`, `what-is-high-assurance.adoc`, `ai-transparency.adoc`; reference: `crypto-post-quantum.adoc`, `crypto-policy-tiers.adoc`, `crypto-cpu-extensions.adoc`); updated nav.adoc files and fixed xrefs; `make docs` passes clean
- **Rogue .claude/ directories removed**: Deleted `.claude/plans/.claude/`, `.claude/.claude/`, and `.claude/rag/.claude/` containing stale session permissions and duplicate agent memory
- **Plan retired**: `sealed-evidence-cache.md` marked completed and moved to `.claude/plans/completed/` (SEC pattern fully implemented in `umrs-platform/src/sealed_cache.rs`)
- **Documentation freeze notice**: Posted to cross-team notes

### Changed
- **refs/manifest.md**: Corrected CMMC Final Rule entry — was pointing to wrong document (2023-27756 OMB submission); now correctly cites 89 FR 83092 (document 2024-22905, Oct 2024); corrected Assessment Guide L2 URL from `AssessmentGuide_L2.pdf` (404) to `AssessmentGuideL2v2.pdf` (v2.13); all 5 DoD 5200.01 entries updated from PENDING to downloaded with SHA-256 checksums
- **Researcher memory**: Updated collection inventory, retrieval patterns, approved sources, and pending items
- **Phase 1 nav and xrefs**: All Antora nav.adoc files updated; cross-module xrefs corrected to reflect structural moves and new placeholder pages; all 9 modules now consistently registered and navigable

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
- **Antora module registration**: All 9 modules now registered in `antora.yml` (previously only ROOT); module sidebar now includes architecture, deployment, reference, operations, devel, patterns, security-concepts, logging-audit, and umrs-tools
- **ROOT/nav.adoc restructured**: Fixed broken `include::` syntax; converted to flat top-level module links with `[collapsible]` sections
- **Module nav xrefs corrected**: All xrefs across architecture, deployment, reference, operations, and devel modules updated to match actual subdirectory paths and cross-module references
- **Navigation collapsibility**: Added `[collapsible]` to all module nav sections to reduce sidebar clutter
- **security-concepts and logging-audit modules created**: New nav and index pages for both modules
- **reference/pages/index.adoc created**: Previously missing
- **File moves completed**: `security-model.adoc` (ROOT → security-concepts), `case-studies.adoc` and `mls-classified-talk.adoc` (architecture → architecture/pages/history/), `TW0-NETIF-JUSTIFICATION.adoc` renamed to `dual-network-interface.adoc` in deployment, `structured-logging.adoc` and `how-to-structure-log.adoc` (operations → deployment), `auditing-noise.adoc` (operations → logging-audit)
- **plus-ref-monitor-info.txt merged**: Content integrated into `reference-monitor.adoc` with new sections (UMRS-vs-kernel distinction, LSM architecture, historical sources) and Mermaid diagrams
- **Operations nav cleaned up**: Removed dead entries for relocated files
- **Deployment nav enhanced**: Added assurance enhancement entries
- **Architecture nav renamed**: "Case Studies" → "Justification and Case Studies" with history/ paths
- **truth-concepts.adoc restored**: Re-established as standalone page in security-concepts module
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
- **Antora module registration and nav xrefs**: Module nav files were not registered in `antora.yml` and xrefs were broken (incorrect paths, stale file locations, missing cross-module references); all 9 modules now registered and xrefs corrected across navigation
- **ROOT nav include syntax**: Fixed broken `include::` directive in ROOT/nav.adoc causing sidebar build failures
- **File path references in nav**: Updated all nav.adoc files to reflect actual subdirectory structure (e.g., `architecture/pages/history/` for moved case studies)
- **reference/pages/index.adoc**: Missing file created to restore module completeness
- **Stray directories**: Removed empty archive/ subdirectory and stray .claude/ directory inside .claude/plans/
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
