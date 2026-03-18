---
name: SCAP/STIG Corpus Familiarization Findings
description: Structural integration recommendations for CCE citations in the Antora module system, following Phase 2 familiarization of the SCAP/STIG corpus (451 RHEL 10 STIG signals, CCE → NIST cross-reference)
type: reference
---

# SCAP/STIG Corpus Familiarization — senior-tech-writer findings

**Date:** 2026-03-17
**Plan:** `.claude/plans/scap-stig-corpus-plan.md` (Phase 2)
**Source files read:**
- `.claude/references/scap-security-guide/stig-signal-index.md` (451 signals, columns: Signal Name, CCE, NIST Controls, Severity, Description, Check Method)
- `.claude/references/scap-security-guide/cce-nist-crossref.md` (451 unique CCEs, columns: CCE, NIST Controls, Signal Name, Description)

---

## 1. What the corpus contains

451 RHEL 10 STIG signals, each with:

- A `signal_name` — machine identifier (e.g., `sysctl_kernel_kexec_load_disabled`)
- A `CCE` identifier — globally unique SCAP identifier (e.g., `CCE-89232-3`)
- One or more `NIST controls` — the 800-53 controls the signal satisfies (e.g., `CM-6`)
- A `severity` — high, medium, or low
- A `description` — terse operator-facing phrase (can inform doc phrasing)
- A `check_method` — sysctl, audit-rule, file-check, package-check, cmdline, service-check, other

The CCE identifier is the bridge: it links a STIG check to its NIST control, and from there to UMRS's own compliance citations.

---

## 2. Structural integration recommendation

### 2a. Primary home: `docs/modules/reference/` — standalone CCE cross-reference page

**Decision: Yes, create a dedicated reference page.**

A CCE cross-reference belongs in `reference/` for three reasons:

1. **Diataxis type is Reference.** The user need is lookup — "what CCE corresponds to this signal?" or "what signals satisfy AC-3?" That is an information-retrieval task, not a learning or doing task. Reference is led by the product; the table structure mirrors the corpus structure exactly.

2. **Audience separation.** The CCE cross-reference is an auditor artifact. Operators do not use CCE identifiers day-to-day. Placing it in `reference/` lets operators stay in `operations/` and `deployment/` while auditors go directly to the compliance section of `reference/`.

3. **Existing precedent.** `reference/pages/compliance-frameworks.adoc` already serves as the governance registry. A CCE cross-reference page is a natural sibling — it extends the compliance section with a lookup table rather than a registry of frameworks.

**Proposed file:** `docs/modules/reference/pages/stig-cce-crossref.adoc`

**Proposed nav placement:** In `reference/nav.adoc`, under the existing `* CUI & Policy` section, after `compliance-frameworks.adoc`:

```asciidoc
* CUI & Policy
** xref:compliance-frameworks.adoc[Compliance frameworks registry]
** xref:stig-cce-crossref.adoc[RHEL 10 STIG — CCE cross-reference]
** xref:cui/cui-descriptions.adoc[CUI category descriptions (c0–c9)]
** xref:cui/cui-category-abbreviations.adoc[CUI category abbreviations]
```

**Page structure:**
- Brief intro: what SCAP/CCE identifiers are and why they matter (2–3 sentences, no more)
- Note on the STIG source version (pre-official; caveated accordingly)
- Filterable table: CCE | NIST Controls | Signal Name | Severity | Description
- Consider splitting into alphabetical subsections (A–C, D–G, etc.) — this also resolves the known RAG chunking limitation in the corpus plan

**What NOT to put on this page:** implementation notes, posture probe internals, Rust code. Those belong elsewhere.

### 2b. Secondary placement: inline in `docs/modules/reference/pages/kernel-probe-signals.adoc`

**Decision: Yes, add CCE citations inline — but selectively.**

`kernel-probe-signals.adoc` is the authoritative reference for every signal the UMRS posture probe evaluates. When a UMRS signal has a STIG equivalent, the CCE identifier belongs alongside the NIST control citation on that signal's entry.

This is the most useful placement for an auditor who wants to trace a specific UMRS posture check to its STIG equivalent. They should not have to cross-reference two pages to get the NIST + CCE combination.

**Format (to establish as a standard for Phase 3b):**

```asciidoc
*Compliance:* NIST SP 800-53 CM-6 | CCE-89232-3 (RHEL 10 STIG)
```

Or in a table column if the page uses tabular format for compliance citations.

This format was proposed in the corpus plan (Phase 3b): `CCE-89232-3 (NIST SP 800-53 CM-6)`. I prefer the format above because it leads with the governing standard (NIST) rather than the cross-reference identifier (CCE). The CCE is a lookup key, not the primary authority.

**Scope of inline additions:** Only signals where UMRS has a confirmed match to a STIG signal. Do not add CCE citations to signals where the match is uncertain — partial or inferred mappings introduce audit risk.

### 2c. NOT in `devel/`

`devel/compliance-annotations.adoc` explains *how to write* compliance annotations in source code. It should reference the CCE cross-reference page by xref when telling developers to add CCE identifiers alongside NIST controls. It should NOT contain the CCE table itself. Devel is oriented toward developer practice, not auditor lookup.

Specifically: when Phase 3a (adding `cce` field to `IndicatorDescriptor`) is done, `compliance-annotations.adoc` should add a section explaining when and how to populate the `cce` field. That section xrefs `stig-cce-crossref.adoc` for the lookup table.

---

## 3. Cross-module impact — pages that need CCE citations added

These existing pages will need updates when Phase 3b integration work begins:

| Page | Location | What changes |
|---|---|---|
| `kernel-probe-signals.adoc` | `reference/pages/` | Add CCE to each signal entry that has a STIG match |
| `compliance-annotations.adoc` | `devel/pages/` | Add section on CCE field in `IndicatorDescriptor`; xref CCE page |
| `compliance-frameworks.adoc` | `reference/pages/` | Add SCAP/STIG as a recognized framework with version caveat |
| `posture-probe.adoc` | `devel/pages/` | Note that `IndicatorDescriptor` carries optional CCE field; link to CCE page |
| `posture-probe-internals.adoc` | `devel/pages/` | Same — reference CCE field in catalog struct documentation |

No changes needed in `architecture/`, `deployment/`, or `operations/` modules. CCE identifiers are auditor artifacts, not operator instructions.

---

## 4. Audience routing — how to surface CCEs without cluttering operator content

The core tension: CCE identifiers are meaningful to auditors and irrelevant to operators. The solution is audience-aware layering, not avoidance.

### Rules

**Rule 1: CCE citations appear only in `reference/` and `devel/` modules.**

`operations/` and `deployment/` pages describe what to do and how to do it. They do not cite CCE identifiers. An operator configuring SELinux enforcing mode does not need to know `selinux_state = CCE-89386-7`. They need the procedure.

**Rule 2: CCE is never a primary citation — always accompanies NIST.**

CCE is a lookup index into the STIG. NIST 800-53 is the governing control. Always write `NIST SP 800-53 AC-3 | CCE-89386-7`, never `CCE-89386-7` alone. An auditor must be able to find the primary authority without following the CCE link.

**Rule 3: The standalone CCE cross-reference page is the auditor's entry point.**

Do not distribute the 451-row table across multiple pages. Concentrate it in one place. From other pages, xref into it. This follows the "single source of truth" principle and prevents the table from drifting into multiple inconsistent copies.

**Rule 4: The TUI and CLI do not display CCE identifiers.**

CCE identifiers belong in exported reports and audit packages, not in real-time operator displays. When UMRS adds JSON report export (planned), the CCE field from `IndicatorDescriptor.cce` should appear in the JSON schema. The TUI display remains CCE-free.

### Navigation hint for auditors

The reference module nav should make the CCE cross-reference findable without requiring knowledge of the Antora structure. The existing `* CUI & Policy` section is the correct grouping — compliance-focused auditors already look there for `compliance-frameworks.adoc`. Placing `stig-cce-crossref.adoc` in the same section makes it discoverable by proximity.

---

## 5. Corpus notes for future documentation work

- **UMRS differentiator worth documenting explicitly** (for `architecture/`): All STIG checks use `check_method: other` or `check_method: sysctl` — they read configuration files but do NOT compare configured values against live kernel state. UMRS contradiction detection catches cases the STIG scan misses (e.g., sysctl.d sets a value but the running kernel differs). This belongs in `architecture/` as an explanation of why the dual-check model exists, not in `reference/`.

- **SELinux signals with high audit relevance:** `selinux_state` (CCE-89386-7, AC-3, AC-3(3)(a), AU-9, SC-7(21), high severity) and `selinux_policytype` (CCE-88366-0, same controls, medium) are the two signals an SELinux auditor will ask about first. These should appear prominently in any CCE cross-reference table.

- **FIPS signals:** `configure_crypto_policy` (CCE-89085-5, SC-12/SC-13, high) is the STIG check for the system crypto policy. UMRS operates in FIPS mode by assumption. When this signal is documented in the CCE table, note that UMRS requires the FIPS policy — not just any crypto policy.

- **RAG limitation:** The signal index and CCE cross-reference are stored as single massive chunks in ChromaDB. Targeted CCE/signal queries do not work reliably via RAG semantic search. Direct file reads are the workaround. The fix (re-generating index files with alphabetical section headings) should be prioritized before Phase 3 integration begins, so that rust-developer and security-auditor can do targeted lookups without reading the entire 450-row table.

---

## 6. Action items for senior-tech-writer (Phase 3b)

When Phase 3a (rust-developer adds `cce` field to `IndicatorDescriptor`) is complete:

1. Create `docs/modules/reference/pages/stig-cce-crossref.adoc` — standalone CCE cross-reference page with the full 451-signal table
2. Register it in `reference/nav.adoc` under `* CUI & Policy`
3. Add `SCAP / RHEL 10 STIG` row to `compliance-frameworks.adoc`
4. Add CCE citations inline in `kernel-probe-signals.adoc` for confirmed UMRS signal matches (coordinate with rust-developer for the confirmed list)
5. Add section to `compliance-annotations.adoc` explaining the `cce` field and when to populate it
6. Run `make docs` — confirm zero errors — before declaring any page complete
