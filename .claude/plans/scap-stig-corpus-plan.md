# SCAP/STIG Corpus Ingestion Plan

**Created:** 2026-03-16
**Status:** Ready to execute
**Source:** `.claude/references/scap-security-guide/rhel10-playbook-stig.yml`
**ROADMAP Goals:** G2 (Security Posture Assessment), G5 (Security Tools), G8 (High-Assurance Patterns)
**Agent:** researcher (ingestion), then rust-developer + security-engineer + security-auditor + tech-writer + senior-tech-writer (familiarization)

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

- [ ] Phase 0: Signal index and CCE cross-reference tables generated
- [ ] Phase 1: `scap-stig` collection ingested into RAG
- [ ] Phase 2: All 5 agents complete familiarization, MEMORY.md updated
- [ ] Phase 3a: `cce` field added to `SignalDescriptor`; existing signals annotated
- [ ] Phase 3b: Documentation style for CCE citations established
- [ ] Phase 3c: Coverage gap report produced
- [ ] Phase 3d: Check methodology comparison documented
- [ ] All agents using CCE citations in new work where applicable

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
