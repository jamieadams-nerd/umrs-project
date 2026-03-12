# Phase 0 Migration Manifest
# Antora Documentation Restructure

**Date**: 2026-03-12
**Author**: senior-tech-writer
**Plan source**: `.claude/plans/antora-doc-restructure.md`
**Vision source**: `.claude/jamies_brain/doc-vision.md`
**Status**: Complete — Phase 0 audit

---

## How to Use This Manifest

This document is the Phase 1 execution roadmap. It is the output of a full read-based audit of
every documentation file in the repository. No files were moved, edited, or deleted during this
phase.

Each table entry records what was actually read, not what the filename implies.

---

## Section 0a — Full Page Inventory

All `.adoc` pages in `docs/modules/*/pages/` mapped to their doc-vision domain and assessed
for module placement.

### ROOT module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `ROOT/pages/index.adoc` | §3 Project introduction | Yes | Entry point — adequate skeleton |
| `ROOT/pages/introduction.adoc` | §3, §4 What UMRS is | Yes | Solid page; covers scope and purpose |
| `ROOT/pages/getting-started.adoc` | §7 Deployment / getting started | Yes — borderline | Content is appropriate here; could xref deployment |
| `ROOT/pages/scope-and-audience.adoc` | §3, §4 What UMRS is | Yes | Good audience framing |
| `ROOT/pages/legal-notices.adoc` | §3 Project orientation | Yes | Administrative; stays in ROOT |
| `ROOT/pages/release-notes.adoc` | §3 Project orientation | Yes | Administrative; stays in ROOT |

### architecture module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `architecture/pages/index.adoc` | §3, §4 Intro / what UMRS is | Yes | Index page |
| `architecture/pages/rationale.adoc` | §4, §11 What UMRS is / development rationale | Yes | Strong-typing rationale summary; thin redirect to rationale-strongly-typed |
| `architecture/pages/rationale-strongly-typed.adoc` | §4, §11 What UMRS is / development | Yes | Strong-typing rationale — architecture is the right home |
| `architecture/pages/library-model.adoc` | §11 Development / UMRS libraries | Borderline | Describes the SELinux modeling library — fits architecture OR devel; leave here |
| `architecture/pages/mls-label-model.adoc` | §6 Security concepts / §16 Reference | Yes | MLS lattice, sensitivity levels, CUI mapping — architecture is correct |
| `architecture/pages/cui-structure.adoc` | §6 Security concepts | Yes | CUI framing — architecture is appropriate |
| `architecture/pages/umrs-prog-lang.adoc` | §11 Development — Why Rust | **DUPLICATE** | Identical content exists at `devel/pages/umrs-prog-lang.adoc`. One must be removed. Architecture home preferred; devel copy is the redundant one. |
| `architecture/pages/kernel-files-tpi.adoc` | §12 High-assurance patterns / §11 Development | Borderline | Architecture rationale for TPI — correct module; cross-reference from patterns/ |
| `architecture/pages/history/index` (implied) | §5 Historical background | Yes | History subdirectory is well placed |
| `architecture/pages/history/HACAMS.adoc` | §5 Historical background | Yes | Correct |
| `architecture/pages/history/case-studies.adoc` | §5 Historical / §24 Outreach | Yes | Appropriate |
| `architecture/pages/history/five-eyes-interop.adoc` | §5 Historical background | Yes | Appropriate |
| `architecture/pages/history/ibm-zos-os390.adoc` | §5 Historical background | Yes | Appropriate |
| `architecture/pages/history/microsoft-nt-orange.adoc` | §5 Historical background | Yes | Appropriate |
| `architecture/pages/history/mls-classified-talk.adoc` | §5, §4 Historical / What UMRS is | Yes | Appropriate |
| `architecture/pages/history/mls-history.adoc` | §5 Historical background | Yes | Appropriate |
| `architecture/pages/history/one-way-hashes.adoc` | §5, §10 Historical / Cryptography | Yes | Appropriate |
| `architecture/pages/history/ring-based-security.adoc` | §5 Historical background | Yes | Appropriate |
| `architecture/pages/history/selinux-history.adoc` | §5 Historical background | Yes | Appropriate |
| `architecture/pages/history/trusted-path-orange.adoc` | §5 Historical background | Yes | Appropriate |

### deployment module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `deployment/pages/index.adoc` | §7 Deployment | Yes | Index |
| `deployment/pages/filesystem-layout.adoc` | §7 Deployment | Yes | Correct |
| `deployment/pages/dual-network-interface.adoc` | §7, §9 Deployment / HA enhancements | Yes | Correct |
| `deployment/pages/linux-baseline.adoc` | §8 Baseline OS hardening | Yes | Correct |
| `deployment/pages/ima-evm-setup.adoc` | §9 HA enhancements | Yes | Correct |
| `deployment/pages/kernel-lockdown-moddisable.adoc` | §9 HA enhancements | Yes | Correct |
| `deployment/pages/tmp-security.adoc` | §9 HA enhancements | Yes | Correct |
| `deployment/pages/rhel/rhel10-directory-structure.adoc` | §7 Deployment (OS-specific) | Yes | Correct |
| `deployment/pages/rhel/rhel10-installation.adoc` | §7 Deployment (OS-specific) | Yes | Correct |
| `deployment/pages/rhel/rhel10-openscap.adoc` | §8 Baseline OS hardening | Yes | Correct |
| `deployment/pages/rhel/rhel10-packages.adoc` | §7 Deployment (OS-specific) | Yes | Correct |
| `deployment/pages/rhel/rhel10-setrans.adoc` | §7 Deployment (OS-specific) | Yes | Correct |
| `deployment/pages/ubuntu/ubuntu.adoc` | §7 Deployment (OS-specific) | Yes | Correct |
| `deployment/pages/structured-logging.adoc` | §15 Logging and auditing | **WRONG MODULE** | This is a logging architecture overview, not a deployment procedure. Move to `logging-audit/`. |
| `deployment/pages/how-to-structure-log.adoc` | §15 Logging and auditing | **WRONG MODULE** | Logging architecture how-to. Move to `logging-audit/`. |

### devel module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `devel/pages/index.adoc` | §11 Development | Yes | Index |
| `devel/pages/build-tooling.adoc` | §11 Development | Yes | Correct |
| `devel/pages/cargo-notes.adoc` | §11 Development | Yes | Correct |
| `devel/pages/compliance-annotations.adoc` | §11, §12 Development / Patterns | Yes | Correct |
| `devel/pages/git-commit-signing.adoc` | §11 Development | Yes | Correct |
| `devel/pages/high-assurance-patterns.adoc` | §12 HA patterns | Borderline — devel or patterns? | Narrative developer guide to patterns — suitable as the devel entry point into patterns/. Keep here as the guide; patterns/ has the individual pages |
| `devel/pages/i18n.adoc` | §11 Development | Yes | Correct |
| `devel/pages/nom-parser.adoc` | §11, §12 Development / Patterns | Yes | Correct — developer-facing pattern explanation |
| `devel/pages/os-detection-deep-dive.adoc` | §11 Development | Yes | Correct |
| `devel/pages/rust-must-use-contract.adoc` | §11, §12 Development / Patterns | Yes | Correct |
| `devel/pages/rust-style-guide.adoc` | §11 Development | Yes | Correct |
| `devel/pages/secure-bash.adoc` | §11 Development | Yes | Correct |
| `devel/pages/secure-python.adoc` | §11 Development | Yes | Correct |
| `devel/pages/umrs-prog-lang.adoc` | §11 Development — Why Rust | **DUPLICATE** | Identical content at `architecture/pages/umrs-prog-lang.adoc`. Delete this copy in Phase 1; keep the architecture copy. |

### logging-audit module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `logging-audit/pages/index.adoc` | §15 Logging and auditing | Yes | Index |
| `logging-audit/pages/logging-capacity.adoc` | §15 Logging | Yes | Correct |
| `logging-audit/pages/log-lifecycle-model.adoc` | §15 Logging | Yes | Correct |
| `logging-audit/pages/log-tuning.adoc` | §15 Logging | Yes | Correct |
| `logging-audit/pages/auditing-noise.adoc` | §15 Logging and auditing | Yes | Correct |

### operations module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `operations/pages/index.adoc` | §14 Operations | Yes | Index |
| `operations/pages/operations.adoc` | §14 Operations | Yes | Correct |
| `operations/pages/admin-index.adoc` | §14 Operations | Yes | Correct |
| `operations/pages/admin-install.adoc` | §14 Operations | Yes | Correct |
| `operations/pages/aide-README.adoc` | §9, §14 HA enhancements / Operations | Borderline | AIDE operations guidance — operations is a reasonable home; could also be deployment/. Leave here. |
| `operations/pages/chain-intro.adoc` | §15, §14 Logging / Operations | Yes | Log chain signing — operations is correct |
| `operations/pages/chain-verify-sign.adoc` | §15, §14 Logging / Operations | Yes | Log signing operations — correct |
| `operations/pages/ima-evm-ops.adoc` | §9, §14 HA enhancements / Operations | Yes | IMA/EVM operational guidance — correct |
| `operations/pages/key-management.adoc` | §10, §14 Crypto / Operations | Yes | Crypto key lifecycle — operations is correct |
| `operations/pages/key-manager-tool.adoc` | §13, §14 Tools / Operations | Yes | HA-Sign tool operations — correct |
| `operations/pages/troubleshooting.adoc` | §14 Operations | Yes | Correct |
| `operations/pages/umrs-signing-README.adoc` | §15, §14 Logging / Operations | Yes | Signing overview — operations is correct |

### patterns module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `patterns/pages/index.adoc` | §12 HA software patterns | Yes | Pattern reference table |
| `patterns/pages/pattern-audit-cards.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-bounds-safe.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-constant-time.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-error-discipline.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-execution-measurement.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-fail-closed.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-layered-separation.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-loud-failure.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-non-bypassability.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-os-detection.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-provenance.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-sec.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-secure-arithmetic.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-supply-chain.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-toctou.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-tpi.adoc` | §12 Patterns | Yes | Correct |
| `patterns/pages/pattern-zeroize.adoc` | §12 Patterns | Yes | Correct |

### reference module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `reference/pages/index.adoc` | §16 Reference material | Yes | Index |
| `reference/pages/compliance-frameworks.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/cryptography/fips-cryptography-cheat-sheet.adoc` | §10 Cryptography / §16 Reference | Yes | Correct placement in reference/cryptography/ |
| `reference/pages/cryptography/key-recommendation-list.adoc` | §10 Cryptography / §16 Reference | Yes | Correct |
| `reference/pages/cryptography/openssl-no-vendoring.adoc` | §10, §11 Crypto / Development | Borderline | Development guidance more than pure reference — could move to devel/ but acceptable here |
| `reference/pages/cui/cui-category-abbreviations.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/cui/cui-descriptions.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/booleans.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/category_set.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/context.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/example-setrans-conf.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/mcs.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/mls-colors.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/rhel-selinux-users.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/role.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/secolor.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/security_type.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/sensitivity.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/setrans-technical.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/umrs-mls-registry.adoc` | §16 Reference | Yes | Correct |
| `reference/pages/selinux/user.adoc` | §16 Reference | Yes | Correct |

### security-concepts module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `security-concepts/pages/index.adoc` | §6 Security concepts / assurance principles | Yes | Index |
| `security-concepts/pages/integrity-and-provenance.adoc` | §6 Security concepts | Yes | Correct |
| `security-concepts/pages/reference-monitor.adoc` | §6 Security concepts | Yes | Correct |
| `security-concepts/pages/rtb-vnssa.adoc` | §6 Security concepts | Yes | Correct |
| `security-concepts/pages/security-model.adoc` | §6 Security concepts | Yes | Correct |
| `security-concepts/pages/truth-concepts.adoc` | §6 Security concepts | Yes | Correct |

### umrs-tools module

| File | Vision domain | In correct module? | Notes |
|---|---|---|---|
| `umrs-tools/pages/index.adoc` | §13 UMRS tools | Yes | Index |
| `umrs-tools/pages/umrs-logspace.adoc` | §13 UMRS tools | Yes | Correct |
| `umrs-tools/pages/umrs-ls.adoc` | §13 UMRS tools | Yes | Correct |
| `umrs-tools/pages/umrs-state.adoc` | §13 UMRS tools | Yes | Correct |
| `umrs-tools/pages/umrs-tooling.adoc` | §13 UMRS tools | Yes | Index/overview page — correct |
| `umrs-tools/pages/umrs-tool-shred.adoc` | §13 UMRS tools | Yes | Correct |
| `umrs-tools/pages/umrs-tool-shred-usage.adoc` | §13 UMRS tools | Yes | Correct |

### security-compliance module

| Status | Notes |
|---|---|
| **Empty** | Directory exists with a `pages/` subdirectory but no files. No nav registered. Delete this module in Phase 1 per locked decision in the plan. |

---

## Section 0a Summary — Actions Required

| Action | Files involved | Phase |
|---|---|---|
| Delete `security-compliance/` module | Empty directory | Phase 1a |
| Move `deployment/structured-logging.adoc` | → `logging-audit/` | Phase 1c |
| Move `deployment/how-to-structure-log.adoc` | → `logging-audit/` | Phase 1c |
| Resolve duplicate `umrs-prog-lang.adoc` | Keep `architecture/` copy, delete `devel/` copy | Phase 1c |
| Create `glossary/` module | New module | Phase 1b |

---

## Section 0b — `docs/_scratch/` Triage

All 50 files read and classified. Format: **Promote** or **Delete**, with rationale.

### Files superseded by existing Antora pages (Delete)

| File | Superseded by | Verdict |
|---|---|---|
| `logging-capacity.txt` | `logging-audit/logging-capacity.adoc` | **Delete** — raw AI conversation transcript; all substance extracted and improved in the Antora page |
| `log-lifecycle-model.txt` | `logging-audit/log-lifecycle-model.adoc` | **Delete** — raw AI conversation transcript; Antora page is the complete, correct version |
| `log-tuning.txt` | `logging-audit/log-tuning.adoc` | **Delete** — raw AI conversation transcript; Antora page is authoritative |
| `chain-intro.txt` | `operations/chain-intro.adoc` | **Delete** — raw AI conversation transcript; Antora page supersedes it |
| `chain-verify-sign.txt` | `operations/chain-verify-sign.adoc` | **Delete** — infer superseded (same pattern as all others) |
| `kernel-files-TPI.md` | `architecture/kernel-files-tpi.adoc` | **Delete** — early draft notes; Antora page is the complete version |
| `nom_parser.md` | `devel/nom-parser.adoc` | **Delete** — early draft notes; Antora page is the complete version |
| `rust-must-use-contract.md` | `devel/rust-must-use-contract.adoc` | **Delete** — early draft; Antora page is the complete version |
| `TW0-NETIF-JUSTIFICATION.md` | `deployment/dual-network-interface.adoc` | **Delete** — raw draft with typos; Antora page is the improved version |
| `aide-README.md` | `operations/aide-README.adoc` | **Delete** — raw AI dump of aide.conf example; Antora page is the complete version |
| `umrs-signing-README.md` | `operations/umrs-signing-README.adoc` | **Delete** — raw AI conversation; Antora page is the complete version |
| `mls-classified-talk.adoc` (in `_scratch/`) | `architecture/history/mls-classified-talk.adoc` | **Delete** — duplicate of the architecture history page |

### Files with content to promote (Promote)

| File | Content summary | Vision domain | Target module | Phase |
|---|---|---|---|---|
| `HACAMS.md` | Clean definition of HACAMS; what it meant, how it evolved, modern equivalents. Well-written prose. | §5 Historical background | Draw from for `architecture/history/HACAMS.adoc` if that page needs enrichment; also seeds ROOT "What is High Assurance" (Phase 3c). NOTE: `architecture/history/HACAMS.adoc` already exists — compare before promoting. | Phase 3c |
| `RTB.md` | RAIN principles, PEFA, NSA RTB public guidance summary; good technical content. | §6 Security concepts / assurance principles | `security-concepts/` — RTB concepts page exists (`rtb-vnssa.adoc`); this may enrich it | Phase 3 |
| `HIGH_ASSURANCE_EXTRA.txt` | Architectural checkpoint capturing TPI, reference monitor, TOCTOU, lattice math, and audit principles in one structured summary. Dense design notes. | §6, §12 Security concepts / Patterns | Source material for enriching pattern pages; also useful for `security-concepts/reference-monitor.adoc` | Phase 3 |
| `RATIONALE_for_HA.adoc` | Formal rationale document: SELinux MLS + IMA/EVM; real-world case studies (Deepwater Horizon, Mars Orbiter); why HA matters. AsciiDoc format already. | §4, §5 What UMRS is / Historical context | Could enrich `architecture/rationale.adoc` or seed ROOT "What is High Assurance" page | Phase 3c |
| `rhel10-openscap.txt` | Step-by-step OpenSCAP install/use procedure on RHEL 10. Plain text but accurate. | §8 Baseline OS hardening | Compare with `deployment/rhel/rhel10-openscap.adoc` before promoting; may add missing steps | Phase 3 |
| `notes/terminology.txt` | Precise, engineering-grade definitions of: provenance, attestation, and related terms. Raw AI output but high quality. | §17 Glossary | Seed content for `glossary/` module (Phase 1b / 3b) | Phase 1b / 3b |
| `notes/umrs-concepts.txt` | Discussion of unclassified MLS concept, teaching value, FIPS/SELinux integration; framing of UMRS as a third-space (not classified, not toy). | §4 What UMRS is / §3 Project intro | ROOT "What is UMRS" page (Phase 3c) | Phase 3c |
| `notes/case-studies.txt` | Three non-classified real-world cases: Twitter employee misuse, evidentiary failure (US v. Vayner), and others. Compelling narrative. | §5 Historical / §4 What UMRS is / §24 Outreach | `architecture/history/case-studies.adoc` already exists — assess overlap; this has different case studies | Phase 3 |
| `notes/umrs-levels-cui.txt` | CUI definition, relationship to MLS, EO 13556 background. Well-grounded. | §6 Security concepts / §4 What UMRS is | Compare with `architecture/cui-structure.adoc`; may enrich it | Phase 3 |
| `notes/umrs_inconvenience.txt` | Explanation of why MLS makes labels hard to change — the friction is intentional governance mechanism. Excellent conceptual content. | §6 Security concepts | Could become a concept page: "Why Label Changes Are Deliberate" in `security-concepts/` | Phase 3 |
| `TPI_DUAL_LOGIC_FLOW.txt` | Detailed narrative of the full TPI pipeline from raw bytes on disk to verified SecurityContext — threat model, TOCTOU hazards, inode anchoring, TPI gate. Very high quality. | §12 Patterns / §11 Development | Enriches `patterns/pattern-tpi.adoc` and `architecture/kernel-files-tpi.adoc` | Phase 2/3 |
| `fgexattr.md` | Explains why `fgetxattr` is the gold standard for provenance: TOCTOU elimination, TCB minimization, atomic metadata, null-byte attack prevention. Technical depth. | §12 Patterns / §6 Security concepts | Enriches `patterns/pattern-toctou.adoc` and `patterns/pattern-provenance.adoc` | Phase 2/3 |
| `category_set_math.md` | Formal set notation for CategorySet validation: dominance relation, Bell-LaPadula, TPI gate, incomparability. | §16 Reference / §12 Patterns | Could become a formal reference page under `reference/selinux/` or enrich `reference/selinux/category_set.adoc` | Phase 3 |
| `MLS_CATEGORIES_SET_MATH.txt` | Unicode-formatted version of same lattice math (dominance, flow verification, isolation). | §16 Reference | Same target as above; compare and use the better version | Phase 3 |
| `notes/extra_checks.txt` | RTB Data Sanitization / Protocol Break technique; Type-State Rust pattern for sanitized data. | §12 Patterns | Could seed a new pattern page: "Data Sanitization / Protocol Break" in `patterns/` | Phase 2/3 |
| `IVM-SYSTEMD.txt` | Systemd service for IMA policy loading on boot. Accurate technical procedure. | §9 HA enhancements | Could enrich `deployment/ima-evm-setup.adoc` with the systemd unit | Phase 3 |
| `logging_notrs.txt` | `umrs-shred` design: advisory locking rationale, audit trail discipline, CUI shred rationale, media sanitization limitations. | §13 UMRS tools / §14 Operations | Compare with `umrs-tools/umrs-tool-shred.adoc`; this has unique advisory lock and CUI rationale content | Phase 3 |
| `notes/encrypt-icon-verification.adoc` | Developer handoff note: encryption icon behavior in `umrs-ls`, verification procedure. AsciiDoc already. | §13 UMRS tools (devel note) | Tech-writer should use this to update `umrs-tools/umrs-ls.adoc` — already targeted at that page | Immediate (tech-writer task) |
| `umrs-selinux-doc-README.md` | Master technical spec for `umrs-selinux`: all invariants, TCB architecture, TPI parsers, CategorySet bitmask, lattice math, code state summary. Very valuable design document. | §11 Development / §12 Patterns / §16 Reference | Draw from for `devel/` API narrative; some content may enrich `reference/selinux/` pages | Phase 3 |
| `apache-mls-project.txt` | Design snapshot for an Apache module that reads SELinux MLS label and emits it as an HTTP response header. | §13 UMRS tools (future) | Not in the current tool set. Candidate for a `umrs-tools/` concept page on future integrations. **Low priority**. | Phase 4 |
| `chrome-mls-extension.txt` | Chrome extension design to display MLS classification from Apache header. | §13 UMRS tools (future) | Companion to apache-mls project. Future integrations. **Low priority**. | Phase 4 |
| `notes/CQRS.txt` | Explains CQRS pattern and how it maps to UMRS tool architecture (write = probe, read = viewer). | §12 Patterns / §11 Development | Useful framing for `patterns/pattern-layered-separation.adoc` or a concept addition | Phase 3 |

### Files to discard (no Antora value)

| File | Reason |
|---|---|
| `AGR-NOTES.txt` | CUI Agriculture handling deltas in JSON model form. Work-in-progress for `cui-labels` crate, not documentation content. |
| `chain-script.txt` | Raw Bash script prototype with `§`-prefixed lines. Implementation artifact, not documentation. |
| `umrs-signing-helper_funcs.txt` | Raw Bash logging helper functions (syslog wrappers). Implementation artifact. |
| `umrs-state-auditd-info-probe.txt` | Raw Rust code snippet for running `auditctl -s`. Implementation artifact. |
| `umrs-state-main_full.txt` | Full `main.rs` prototype code with section symbols. Implementation artifact. |
| `umrs-state-sysinfo-probe.txt` | Rust snippet using `sysinfo` crate. Implementation artifact. |
| `umrs-logspace-notes.txt` | Early Rust architecture notes for `umrs-logspace` crate. Implementation artifact. |
| `umrs-logspace-final_notes.txt` | Detailed Rust source tree and code for `umrs-logspace`. Implementation artifact. |
| `umrs-shred-notes.txt` | Advisory lock rationale snippet; largely subsumed by `logging_notrs.txt` and existing tool docs. |
| `selinux-policy-junk-NOTES.md` | Notes on SELinux policy macro usage (not writing docs from a raw AI tutorial). Implementation guidance. |
| `LS_HA_RESTRICTED_NOTES.txt` | Bug note about `umrs-ls` showing `<unlabeled>` where the system shows a type. Development issue note, not documentation. |
| `UMRS_LABELS-tool.txt` | Command-line notes for `umrs-labels` tool (argument syntax). Trivial; covered by tool docs. |
| `rhel10-scripts-JSON.md` | Notes on `jq` commands for extracting markings from JSON. Trivial tooling notes. |
| `umrs-core-unicode.txt` | Unicode escape sequences for block characters (visual elements). UI development notes. |
| `unicode_symbols.txt` | Shell echo commands testing Unicode/ANSI rendering. UI development notes. |
| `notes/aide_check_one_file.txt` | Explanation of how to check a single file with AIDE. Useful operationally but superseded by `operations/aide-README.adoc` coverage area. **Low priority promote** or discard. |
| `notes/AsciiDoc-Notes.txt` | Raw AI output on AsciiDoc getting started. No longer needed — docs are Antora-based already. |
| `notes/i18n.txt` | Raw AI conversation about Rust i18n approach. Superseded by `devel/i18n.adoc`. |
| `UMRS.cil` | SELinux CIL policy snippet. Source code artifact, not documentation. |
| `UMRS_CUI.cil` | SELinux CIL for CUI categories. Source code artifact, not documentation. |

---

## Section 0c — `docs/modules/deployment/pages/_archive/` Triage

All 5 files read and compared against active pages.

| Archive file | Active page | Comparison verdict |
|---|---|---|
| `filesystem-layout.md` | `deployment/pages/filesystem-layout.adoc` | **Delete** — archive has the 64GB table but lacks the VM layout section and STIG compliance notes that the Antora page includes. The Antora page is a full superset. |
| `ISOLATED-TMP.md` | `deployment/pages/tmp-security.adoc` | **Delete** — archive has a longer historical narrative and per-user/per-process isolation discussion. The Antora page is more structured but some historical detail (sticky bit history, per-user `tmpfs` PAM model) is not in the active page. **Before deleting**: extract the per-user/per-process isolation section and historical background for `tmp-security.adoc` enrichment (Phase 3). |
| `umrs-tmp-filesystems-README.md` | `deployment/pages/tmp-security.adoc` | **Delete** — contains a thorough historical narrative of `/tmp` (Bell Labs 1971, sticky bit origin, symbolic link attacks). The Antora page references this history briefly. **Before deleting**: the full historical narrative could enrich `tmp-security.adoc`'s background section (Phase 3). This is lower priority — log and defer. |
| `rhel10-README.md` | `deployment/pages/rhel/rhel10-installation.adoc` | **Delete** — planning checklist (install FIPS, network interfaces, packages). The active `rhel10-installation.adoc` covers this material fully. Archive is a rough notes file; no unique content worth preserving. |
| `vm-filesystem-layout.md` | `deployment/pages/filesystem-layout.adoc` | **Partial promote** — archive has a VM-specific layout table (VMware, Parallels, KVM, VirtualBox) with different sizing recommendations for virtual environments. The active `filesystem-layout.adoc` has a bare-metal section only; it has a `VM Layout` section but content may need to be compared directly. **Action**: read `filesystem-layout.adoc` in full to confirm whether VM section exists before deleting `vm-filesystem-layout.md`. Flag for Phase 1. |

---

## Section 0d — `docs/new-stuff/crypto.md` Disposition

**Confirmed vision §10 (Cryptography) and §17 (Glossary) seed material.**

Content read and verified:

- 8 algorithm categories: one-way hashes, symmetric encryption, asymmetric/key exchange, digital signatures, MAC/HMAC, key derivation, password hashing, and post-quantum (ML-KEM/ML-DSA/SLH-DSA per FIPS 203/204/205)
- Policy tier framework: Preferred / Approved / Acceptable / Disallowed
- Each category includes algorithm, key sizes, status, and usage notes
- Ends with a cryptographic glossary section (hash function, cipher, KDF, MAC/HMAC, key encapsulation, digital signature, post-quantum cryptography, FIPS)

**Target**: `reference/cryptography/` — the existing subdirectory already has three pages (`fips-cryptography-cheat-sheet.adoc`, `key-recommendation-list.adoc`, `openssl-no-vendoring.adoc`). The `crypto.md` content would:

1. Become `reference/cryptography/index.adoc` — or enrich the existing cheat sheet
2. The glossary section seeds `glossary/pages/glossary.adoc` (Phase 3b)

**Open question** (per plan §Open Questions #2): Is this better as `reference/cryptography/` expanded or as a standalone `cryptography/` module? The content is dense enough for a standalone module, but the existing `reference/cryptography/` pages are already there. Recommendation: expand `reference/cryptography/` rather than create a new module — this avoids module proliferation while keeping crypto content unified.

**Post-use action**: Delete `docs/new-stuff/crypto.md` after content is promoted (Phase 3a).

---

## Section 0e — Top-Level Repo Files

### `README.md`

**Content**: Strong "what is high assurance" narrative with HACAMS history, real-world examples (military/intelligence, nuclear, aviation, medical), feature table comparing HA vs traditional, and project description.

**Extraction targets**:
- "What is a High-Assurance System" section → ROOT `pages/what-is-high-assurance.adoc` (new page, Phase 1d)
- HACAMS history + real-world examples → supplements `architecture/history/HACAMS.adoc`
- Feature comparison table → ROOT or `security-concepts/`

**Post-extraction action**: README stays at repo root as the GitHub landing page. Antora pages become the authoritative version; README can xref or summarize.

### `UMRS-PROJECT.md`

**Content**: Project identity, MLS label hierarchy table (GENERAL / PUBLIC / U-CONTROLLED), CUI explanation with NARA cite, assurance philosophy, adoption invitation, personal note.

**Extraction targets**:
- Project identity paragraph and MLS label table → ROOT `pages/what-is-umrs.adoc` (new page, Phase 1d)
- CUI / MLS relationship discussion → can enrich `architecture/cui-structure.adoc` or `architecture/mls-label-model.adoc`
- Adoption invitation and philosophy → ROOT introduction pages

**Post-extraction action**: File stays at repo root. Antora pages become authoritative.

### `UMRS-PLAN.md`

**Content**: Milestone roadmap (M1–M3) with feature backlog checkboxes. Project planning document.

**Verdict**: Reference-only. No narrative content to extract into Antora. Keep as-is at repo root as a planning artifact. Consider moving to `.claude/` if it clutters the root, but that is outside documentation scope.

---

## Section 0f — Gap Analysis: Vision Domains with No Existing Content

The following doc-vision domains have no dedicated Antora pages:

| Vision domain | §  | Current state | Notes |
|---|---|---|---|
| Glossary | §17 | **No module** | `glossary/` module does not exist. Seed content exists in `_scratch/notes/terminology.txt` and `docs/new-stuff/crypto.md`. Creating this module is Phase 1b. |
| AI transparency | §18 | **No page** | No page anywhere describes AI agent roles in the project. Skeleton planned in Phase 1d; full content in Phase 3d. |
| "What is UMRS" dedicated page | §4 | Partial | `ROOT/introduction.adoc` covers it but the vision calls for a dedicated, detailed page. Phase 1d. |
| "What is High Assurance" dedicated page | §3 | Partial | `ROOT/introduction.adoc` touches it; not a dedicated page. README has strong content. Phase 1d. |
| Post-quantum cryptography | §10 | No Antora page | Three pages in `reference/cryptography/` exist but PQC (FIPS 203/204/205) has no dedicated treatment. `docs/new-stuff/crypto.md` has it. Phase 3a. |
| Protocol Break / Data Sanitization pattern | §12 | No page | `notes/extra_checks.txt` has this content. Not in `patterns/`. Phase 2/3. |
| CDS-MIB / log-state concepts (background) | §15 | Partial | Lifecycle model page covers it; no dedicated concept page on the historical CDS-MIB framing. Low priority. |
| MLS friction / label governance concept | §6 | No page | `notes/umrs_inconvenience.txt` has good content. Would become a `security-concepts/` concept page. Phase 3. |
| "Why Rust" detailed page | §11 | No standalone page | `devel/umrs-prog-lang.adoc` (to be deleted — duplicate) and `architecture/umrs-prog-lang.adoc` cover this. Needs to be in devel/ as a proper "Why Rust" page. Phase 1c: after resolving duplicate, ensure one canonical page in devel/. |
| Secure coding guide references | §11 | No page | Vision §11 calls for pointing developers to three secure coding guides. No page does this. Phase 3. |
| UMRS crate introduction / trusted foundations page | §11 | Partial | `devel/` has individual pages but no "start here" overview of all crates as trusted foundations. Phase 3. |

---

## Phase 1 Execution Checklist

Items to execute in Phase 1, in dependency order:

### Phase 1a — Remove empty module
- [ ] Delete `docs/modules/security-compliance/` directory entirely
- [ ] Confirm no `antora.yml` nav references it (check `docs/antora.yml`)

### Phase 1b — Create glossary module
- [ ] Create `docs/modules/glossary/pages/`
- [ ] Create `docs/modules/glossary/pages/index.adoc` (skeleton)
- [ ] Create `docs/modules/glossary/nav.adoc`
- [ ] Register in `docs/antora.yml` nav list
- [ ] Seed initial terms from `_scratch/notes/terminology.txt` (then delete that file)

### Phase 1c — Execute content migrations

**Move misplaced pages:**
- [ ] Move `deployment/structured-logging.adoc` → `logging-audit/structured-logging.adoc`
- [ ] Move `deployment/how-to-structure-log.adoc` → `logging-audit/how-to-structure-log.adoc`
- [ ] Update `deployment/nav.adoc` (remove entries)
- [ ] Update `logging-audit/nav.adoc` (add entries)
- [ ] Fix any xrefs that point to old deployment/ paths

**Resolve duplicate:**
- [ ] Delete `devel/pages/umrs-prog-lang.adoc`
- [ ] Ensure `architecture/pages/umrs-prog-lang.adoc` is registered in architecture nav
- [ ] Update any xrefs in devel/ that point to `devel:umrs-prog-lang.adoc`
- [ ] Create a new `devel/pages/why-rust.adoc` that is development-oriented (can reference the architecture rationale page rather than duplicate it)

**Delete confirmed superseded _scratch files:**
- [ ] `logging-capacity.txt`, `log-lifecycle-model.txt`, `log-tuning.txt`
- [ ] `chain-intro.txt`, `chain-verify-sign.txt`
- [ ] `kernel-files-TPI.md`, `nom_parser.md`, `rust-must-use-contract.md`
- [ ] `TW0-NETIF-JUSTIFICATION.md`, `aide-README.md`, `umrs-signing-README.md`
- [ ] `_scratch/mls-classified-talk.adoc` (duplicate of architecture history page)
- [ ] All "discard" files listed in Section 0b

**Delete confirmed superseded _archive files:**
- [ ] `_archive/filesystem-layout.md`
- [ ] `_archive/ISOLATED-TMP.md` (after extracting per-user isolation content — see note)
- [ ] `_archive/umrs-tmp-filesystems-README.md` (after extracting historical narrative — see note)
- [ ] `_archive/rhel10-README.md`
- [ ] `_archive/vm-filesystem-layout.md` — READ `filesystem-layout.adoc` VM section first; if VM layout already present in full, delete; if not, extract VM table before deleting

### Phase 1d — Strengthen ROOT module
- [ ] Create `ROOT/pages/what-is-umrs.adoc` — draw from `UMRS-PROJECT.md` and `ROOT/introduction.adoc`
- [ ] Create `ROOT/pages/what-is-high-assurance.adoc` — draw from `README.md` high-assurance narrative
- [ ] Create `ROOT/pages/ai-transparency.adoc` — skeleton page (full content Phase 3d)
- [ ] Update `ROOT/nav.adoc` with new pages
- [ ] Update `ROOT/pages/introduction.adoc` to link to new pages

### Phase 1e — Validate
- [ ] `make docs` — zero errors
- [ ] All nav files consistent with actual pages
- [ ] No broken xrefs

---

## Unresolved Items Flagged for Jamie

1. **`vm-filesystem-layout.md` vs `filesystem-layout.adoc` VM section**: Read `filesystem-layout.adoc` completely to confirm whether the VM layout table is already present before deleting the archive file. If the VM layout section is thin or missing, extract the VM table from the archive first.

2. **`ISOLATED-TMP.md` per-user/per-process isolation content**: This archive has solid conceptual content on per-user and per-process temporary directories that the active `tmp-security.adoc` does not cover in depth. Before deleting, decide whether to enrich `tmp-security.adoc` with this material.

3. **Open question #1 — Hardening as module or subsection**: IMA, kernel lockdown, `/tmp` isolation, AIDE, OpenSCAP are currently scattered across `deployment/` and `operations/`. The current placement works, but as content grows it may warrant a `hardening/` subsection within `deployment/`. Defer decision until after Phase 1 moves are complete.

4. **Open question #2 — Cryptography: standalone module or reference subdir**: Recommendation in this manifest is to expand `reference/cryptography/`. Jamie should confirm before Phase 3a work begins.

5. **`devel/umrs-prog-lang.adoc` duplicate**: After deleting the devel copy, decide whether to create a new `devel/why-rust.adoc` as a development-oriented entry point, or whether the architecture page plus an xref is sufficient.
