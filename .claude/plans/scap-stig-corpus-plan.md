# SCAP/STIG Corpus Ingestion Plan

**Created:** 2026-03-16
**Status:** Phase 1 COMPLETE. Phase 3c done via security-auditor-corpus plan. Phases 2 (agent familiarization) and 3a/3b/3d pending.
**Source:** `.claude/references/scap-security-guide/rhel10-playbook-stig.yml`
**ROADMAP Goals:** G2 (Security Posture Assessment), G5 (Security Tools), G8 (High-Assurance Patterns)
**Agent:** researcher (ingestion), then rust-developer + security-engineer + security-auditor + tech-writer + senior-tech-writer (familiarization)

---

## Ingestion Notes (2026-03-17)

**Collection:** `scap-security-guide` in ChromaDB at `/media/psf/repos/ai-rag-vdb/chroma`
**Chunks stored:** 7 total (2 primary data chunks + 5 extraction script chunks)
**Date ingested:** 2026-03-17

### Chunking limitation — known issue

The two primary data files (`stig-signal-index.md` and `cce-nist-crossref.md`) are stored as
single massive chunks (~23,000 and ~21,000 tokens respectively). The ingest pipeline's markdown
chunker splits on H1/H2/H3 headings — both files have only one heading, with the entire content
as a single flat table. Tables have no paragraph breaks between rows, so the token-bounded splitter
cannot subdivide them.

**Impact:** Every RAG query against `scap-security-guide` returns the same two chunks regardless
of query specificity. The chunks are too large for the embedding model to produce discriminating
vectors. Targeted lookups by CCE or signal name do not work — the entire table is always returned.

**Workaround (current session):** Direct file reads of `stig-signal-index.md` and
`cce-nist-crossref.md` work reliably for CCE/signal lookup within this context window.
Agents needing a specific CCE or signal should read the file directly rather than relying on
RAG semantic search for this collection.

**Recommended fix (future):** Re-generate the index files broken into alphabetical sections
(e.g., `## Signals: A-C`, `## Signals: D-G`) so the chunker can split them into ~50 rows per
chunk. This would require modifying `extract_stig_index.py` to emit section headings and
re-running ingestion with `--force`.

### Corpus content summary (researcher familiarization)

**451 unique signals across the following check method categories:**
- `sysctl` (kernel parameter checks) — ~35 signals, all directly relevant to UMRS posture catalog
- `audit-rule` (audit framework rules) — ~55 signals covering DAC modification, file deletion, privileged commands, login events
- `file-check` (file ownership/permissions) — ~120 signals covering /etc/ and system dirs
- `package-check` (package presence/absence) — ~30 signals
- `cmdline` (kernel command line arguments) — ~8 signals (grub, audit, pti, vsyscall)
- `service-check` (systemd service state) — ~5 signals
- `other` (various — PAM, SSH, SELinux config, crypto policy) — ~200 signals

**UMRS-relevant signal highlights (sysctl category — directly maps to posture catalog):**

| Signal | CCE | NIST Controls | Relevance to UMRS |
|---|---|---|---|
| `sysctl_kernel_kexec_load_disabled` | CCE-89232-3 | CM-6 | Already cited in plan; maps to kexec signal |
| `sysctl_kernel_randomize_va_space` | CCE-87876-9 | CM-6(a), SC-30, SC-30(2) | ASLR — likely in posture catalog |
| `sysctl_kernel_dmesg_restrict` | CCE-89000-4 | SI-11(a), SI-11(b) | Kernel log access restriction |
| `sysctl_kernel_kptr_restrict` | CCE-88686-1 | CM-6(a), SC-30 | Kernel pointer address restriction |
| `sysctl_kernel_unprivileged_bpf_disabled` | CCE-89405-5 | AC-6, SC-7(10) | Unprivileged BPF access |
| `sysctl_kernel_yama_ptrace_scope` | CCE-88785-1 | SC-7(10) | ptrace scope restriction |
| `sysctl_net_core_bpf_jit_harden` | CCE-89631-6 | CM-6, SC-7(10) | BPF JIT hardening |
| `sysctl_kernel_exec_shield` | CCE-89079-8 | CM-6(a), SC-39 | Exec shield (NX/SMEP) |
| `sysctl_fs_protected_hardlinks` | CCE-86689-7 | AC-6(1), CM-6(a) | Hardlink protection |
| `sysctl_fs_protected_symlinks` | CCE-88796-8 | AC-6(1), CM-6(a) | Symlink protection |
| `sysctl_kernel_core_pattern` | CCE-86714-3 | SC-7(10) | Core dump storage disable |
| `sysctl_kernel_perf_event_paranoid` | CCE-90142-1 | AC-6 | Unprivileged perf profiling |

**SELinux-specific signals:**
- `selinux_state` (CCE-89386-7): AC-3, AC-3(3)(a), AU-9, SC-7(21) — high severity — UMRS verifies this
- `selinux_policytype` (CCE-88366-0): AC-3, AC-3(3)(a), AU-9, SC-7(21) — medium — UMRS verifies this

**Audit/kmod signals directly relevant to UMRS:**
- `audit_rules_kernel_module_loading_init` (CCE-90172-8): AC-6(9), AU-12(c)
- `audit_rules_kernel_module_loading_finit` (CCE-88638-2): AC-6(9), AU-12(c)
- `audit_rules_kernel_module_loading_delete` (CCE-89982-3): AC-6(9), AU-12(c)

**FIPS/crypto policy signals (relevant to SC-13, FIPS 140-2 environment):**
- `configure_crypto_policy` (CCE-89085-5): AC-17(2), SC-12(2), SC-12(3), SC-13 — high severity
- `configure_bind_crypto_policy` (CCE-86874-5): SC-12(2), SC-13 — high
- `aide_use_fips_hashes` (CCE-90260-1): CM-6(a), SI-7, SI-7(1) — AIDE must use FIPS 140-2 hashes

**Note on STIG check methodology vs. UMRS approach:**
All STIG checks use `check_method: other` or `check_method: sysctl` — meaning they read
configuration files or sysctl values but do NOT compare configured values against live kernel
state. The UMRS contradiction detection approach (configured vs. live) catches cases where a
sysctl.d file sets a value but the running kernel has a different value — a gap the STIG scan
will miss entirely. This is a differentiating capability worth documenting explicitly.

---

## Purpose

Ingest the RHEL 10 STIG playbook (63K lines, 463 unique CCEs, 135 NIST controls, 5389 tasks)
into the RAG as the `scap-stig` collection. This corpus provides:

1. **CCE cross-references** — maps NIST controls to CCE identifiers (e.g., `kexec_load_disabled`
   → `CCE-89232-3` → `NIST-800-53-CM-6`). Our source code and documentation should cite CCEs
   alongside NIST controls where applicable.

2. **Coverage gap analysis** — identifies STIG-mandated hardening checks we may not yet cover
   in the `umrs-platform` posture catalog. Some STIG items will be out of scope (GUI, network
   services), but kernel/sysctl/audit items are directly relevant.

3. **Descriptive text** — task names contain concise, operator-friendly descriptions of what
   each hardening rule does. These may inform our own rationale text and documentation phrasing.

4. **Check methodology comparison** — SCAP/STIG checks examine configuration files (sysctl.d,
   modprobe.d, /etc/ configs) but do NOT compare configuration against live kernel state. Our
   approach (configured vs. live, contradiction detection) is strictly superior. Understanding
   the STIG approach helps us articulate why our method catches false positives that SCAP misses.

---

## Pre-Ingestion: Preprocessing

The raw YAML is 63K lines with massive repetition (each sysctl signal has 5 nearly identical
task blocks). For effective RAG retrieval, preprocess before ingestion:

### Phase 0 — Extract structured signal index

Write a preprocessing script that extracts from the YAML:

```
signal_name | CCE | NIST controls | severity | description | check method | desired value
```

Output: `.claude/references/scap-security-guide/stig-signal-index.md` (markdown table)

This index becomes the primary ingestion document — the RAG can retrieve any signal by
CCE, NIST control, or signal name.

### Phase 0.5 — Extract unique CCE → NIST mapping table

A standalone cross-reference table:

```
CCE-89232-3 | NIST-800-53-CM-6 | sysctl_kernel_kexec_load_disabled | Disable Kernel Image Loading
```

Output: `.claude/references/scap-security-guide/cce-nist-crossref.md`

This is the document that rust-developer and security-auditor will use most — quick lookup
of CCE identifiers for any signal we already cover.

---

## Phase 1 — RAG Ingestion

**Agent:** researcher
**Collection name:** `scap-stig`
**Documents to ingest:**

1. `stig-signal-index.md` (from Phase 0)
2. `cce-nist-crossref.md` (from Phase 0.5)
3. `rhel10-playbook-stig.yml` (full playbook — the RAG chunker will handle it)

Use the `rag-ingest` skill.

---

## Phase 2 — Familiarization (5 agents)

After ingestion, each agent runs corpus-familiarization on the `scap-stig` collection.
Order is parallel where possible.

| Agent | Focus | Priority |
|-------|-------|----------|
| rust-developer | CCE mappings for existing `SignalId` variants; new signal candidates; check methodology comparison | High |
| security-engineer | STIG deployment posture; items affecting SELinux policy, file permissions, audit rules | High |
| security-auditor | Coverage gap analysis vs. posture catalog; CCE annotation debt in source code | High |
| tech-writer | Descriptive text patterns; operator-facing terminology; CCE citation format | Medium |
| senior-tech-writer | Structural integration; where CCE citations belong in Antora modules | Medium |

Each agent updates their MEMORY.md with:
- Key findings from familiarization
- List of CCEs that map to signals they own or document
- Identified gaps or action items

---

## Phase 3 — Cross-Reference Integration

After familiarization, the following integration work is expected:

### 3a. Source Code CCE Annotations

**Agent:** rust-developer (implementation), security-auditor (review)

Add CCE identifiers to `SignalDescriptor` entries in `catalog.rs` where the STIG has a
matching check. New field:

```rust
/// CCE identifier from the RHEL 10 STIG, if this signal has a SCAP equivalent.
pub cce: Option<&'static str>,
```

This makes CCE cross-references available at compile time alongside NIST controls.

### 3b. Documentation CCE Citations

**Agent:** tech-writer

When documenting a signal or hardening check that has a CCE mapping, include the CCE
alongside the NIST control citation. Format: `CCE-89232-3 (NIST SP 800-53 CM-6)`.

### 3c. Coverage Gap Report

**Agent:** security-auditor

Produce a report at `.claude/reports/stig-coverage-gaps.md` listing:
- STIG items we cover (with our `SignalId` → CCE mapping)
- STIG items we could cover (relevant kernel/sysctl/audit items not yet in catalog)
- STIG items out of scope (GUI, network services, package management)
- Items where our check is superior to STIG's (contradiction detection, live vs. configured)

### 3d. Check Methodology Comparison

**Agent:** security-engineer

Document how our configured-vs-live contradiction detection approach differs from and
improves upon the STIG's configuration-file-only checks. This becomes content for the
`docs/modules/architecture/` pages explaining why UMRS catches false positives that
standard SCAP scans miss.

---

## Definition of Done

- [x] Phase 0: Signal index and CCE cross-reference tables generated (451 signals, both files in `scap-security-guide/`)
- [x] Phase 1: `scap-stig` collection ingested into RAG (collection: `scap-security-guide`, 7 chunks)
- [x] Phase 2 (researcher): Corpus familiarization complete — see ingestion notes above
- [ ] Phase 2 (rust-developer): CCE mappings for existing SignalId variants; new signal candidates
- [ ] Phase 2 (security-engineer): STIG deployment posture; SELinux policy, file permissions, audit rules
- [ ] Phase 2 (security-auditor): Coverage gap analysis vs. posture catalog; CCE annotation debt
- [ ] Phase 2 (tech-writer): Descriptive text patterns; CCE citation format
- [ ] Phase 2 (senior-tech-writer): Structural integration; CCE citations in Antora modules
- [ ] Phase 3a: `cce` field added to `SignalDescriptor`; existing signals annotated
- [ ] Phase 3b: Documentation style for CCE citations established
- [ ] Phase 3c: Coverage gap report produced at `.claude/reports/stig-coverage-gaps.md`
- [ ] Phase 3d: Check methodology comparison documented in `docs/modules/architecture/`
- [ ] All agents using CCE citations in new work where applicable
- [ ] RAG chunking fix: re-generate index files with section headings; re-ingest with `--force`

---

## Notes

- The STIG playbook is "based on what is expected" for RHEL 10 — it is not yet the official
  DISA STIG. When the official STIG is released, the researcher should re-ingest and the
  security-auditor should diff for changes.
- The `scap-security-guide/` directory also contains CIS, ANSSI, ISM, HIPAA, PCI-DSS, and
  E8 profiles. These are future ingestion candidates but are NOT in scope for this plan.
- At 63K lines, the raw YAML may hit RAG chunk limits — the preprocessed index files are
  the primary retrieval targets. The full YAML provides fallback context.

## Model Assignments

| Work Item | Agent | Model | Rationale |
|---|---|---|---|
| Phase 0 preprocessing | researcher | **sonnet** | Scripted YAML extraction, no design decisions |
| Phase 1 RAG ingestion | researcher | **sonnet** | Standard ingestion workflow |
| Phase 2 familiarization | all 5 agents | **sonnet** | Corpus reading, knowledge extraction |
| Phase 3a CCE field + annotations | rust-developer | **sonnet** | Follows established `SignalDescriptor` pattern |
| Phase 3b doc citation format | tech-writer | **haiku** | Style decision, no code |
| Phase 3c coverage gap report | security-auditor | **opus** | Cross-referencing two catalogs, judgment calls on scope |
| Phase 3d methodology comparison | security-engineer | **sonnet** | Technical writing, architecture-level |
