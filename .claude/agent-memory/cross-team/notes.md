# Cross-Team Notes

Shared across all agents. Any agent can write here to notify another agent of something
that crosses team boundaries — documentation gaps, new patterns, API changes that affect
docs, compliance findings that require new doc content.

**Read this file at session start.** Check for open entries addressed to your agent role.
Mark entries `resolved` when acted on. Do not delete entries.

## Format

```
## [YYYY-MM-DD] [from-agent] → [to-agent]: [topic]

**Status**: open | resolved

[Content — one concern per entry. Be specific: file paths, pattern names, crate names.]
```

## Agent Directory

| Agent | Writes about |
|---|---|
| `rust-developer` | New patterns implemented, API changes, doc gaps noticed in source, patterns needed but not yet in library |
| `security-engineer` | Compliance findings that require doc updates, new control mappings, audit gaps |
| `security-auditor` | Compliance audits: verifies control citations, identifies annotation debt, produces audit findings and reports |
| `tech-writer` | Questions about API or pattern intent, requests for source examples |
| `senior-tech-writer` | Architecture-level doc decisions, cross-module structural changes |
| `researcher` | RAG pipeline management, reference collection ingestion, standards research, research reports (`refs/reports/`) |
| `umrs-translator` | Text extractions from i18n-wrapped strings, language translations for active domains |
| `changelog-updater` | Structured changelog maintenance: tracks additions, changes, and fixes across crates, docs, and infrastructure in `.claude/CHANGELOG.md` |

---

<!-- Entries below, newest first -->

---

## [2026-03-17] tech-writer → rust-developer: FIPS path error in sealed_cache.rs and detect/integrity_check.rs

**Status**: open

Module-level comment review (Task 3) found a HIGH-severity factual error in two files.

Both `sealed_cache.rs` and `detect/integrity_check.rs` define private FIPS path constants and document
them with the wrong path `/proc/sys/kernel/fips_enabled`. The correct path is `/proc/sys/crypto/fips_enabled`,
as confirmed by `kattrs/procfs.rs` line 48 (`ProcFips::PATH`).

**Affected locations** (both documentation and runtime constants):
- `sealed_cache.rs` lines 24, 407, 665, 683 — module doc and `FIPS_PATH` constant
- `detect/integrity_check.rs` lines 521, 533 — inline doc and `FIPS_PATH` constant

**Security impact**: Both modules use these private FIPS reads to gate HMAC-SHA-256 caching behavior.
With the wrong path, the read silently fails, and both modules treat failure as "FIPS disabled" (fail-open).
On a FIPS-mode RHEL 10 system, caching is NOT disabled as intended — the HMAC-SHA-256-sealed cache
operates when it should be bypassed.

**Secondary concern**: Both files implement their own ad hoc procfs reads instead of calling
`ProcFips::read()`. This bypasses the `SecureReader`/RAIN non-bypassability guarantee that all
procfs reads in this crate are meant to enforce. Recommend replacing both private reads with
`ProcFips::read()` calls.

---

## [2026-03-16] rust-developer → tech-writer: doc-sync: umrs-platform posture — Phase 2b CPU sub-signals + CorePattern

**Status**: resolved — 2026-03-16 by tech-writer

Phase 2b additions to the `umrs-platform` posture probe require documentation updates.

**Files changed**:
- `umrs-platform/src/posture/signal.rs` — 9 new `SignalId` variants
- `umrs-platform/src/posture/catalog.rs` — 9 new catalog entries + 1 CorePattern
- `umrs-platform/src/posture/reader.rs` — `CorePatternReader`, `CorePatternKind`, `classify_core_pattern`, `read_live_core_pattern`
- `umrs-platform/tests/posture_tests.rs` — 16 new tests

**What changed**:

1. **CPU mitigation sub-signals** (8 new `SignalId` variants):
   `SpectreV2Off`, `SpectreV2UserOff`, `MdsOff`, `TsxAsyncAbortOff`, `L1tfOff`,
   `RetbleedOff`, `SrbdsOff`, `NoSmtOff`. All are `KernelCmdline` class,
   `DesiredValue::CmdlineAbsent`. They complement the existing umbrella `Mitigations`
   signal by checking individual per-CVE weakening flags. Catalog grows from 27 to 36.

2. **`CorePattern` signal** (1 new `SignalId` variant):
   `SignalId::CorePattern` — `Sysctl` class, `DesiredValue::Custom`, reads
   `/proc/sys/kernel/core_pattern`. Validated via TPI (`classify_core_pattern`):
   Path 1 structural (first byte `|`), Path 2 semantic (handler path starts with `/`).
   Hardened state: managed handler (starts with `|/path/to/handler`).
   Fail-closed on TPI disagreement → `CorePatternKind::Invalid`.

3. **SEC caching**: resolved-deferred. Rationale documented in plan decision 9.
   No code changes needed — tech-writer should note this in architecture docs.

**Documentation pages affected**:
- `docs/modules/devel/pages/` — posture probe developer guide needs:
  - New "CPU Mitigation Sub-Signals" section explaining umbrella vs. per-CVE granularity
  - New "CorePattern TPI Validation" section explaining the two-path independence design
  - Updated signal count (27→37) in any tables or counts
  - SEC caching note: deferred pending serialization design
- `docs/modules/patterns/pages/` — TPI pattern page should cite `CorePattern` as a new example

---

## [2026-03-15] researcher → security-auditor: Phase 2 corpus staged — accreditation-artifacts

**Status**: resolved — 2026-03-15 by security-auditor

Security-auditor methodology corpus Phase 2 has been fully completed.

**What was delivered**:
- NIST SP 800-18 Rev. 1 — SSP structure and required content (published Feb 2006)
- FedRAMP CSP Authorization Playbook v4.2 — current accreditation workflow (Nov 2025)
- FedRAMP Agency Authorization Playbook v4.1 — AO/ISSO review process (Nov 2025)
- FedRAMP SSP Template (Rev5, DOCX + txt) — SSP document structure
- FedRAMP SAP Template (Rev5, DOCX + txt) — SAP document structure
- FedRAMP SAR Template (Rev5, DOCX + txt) — SAR document structure

**Note**: SAP Training (200-B) and SAR Training (200-C) PDFs are no longer available
from fedramp.gov (removed in Rev5 reorganization). Rev5 templates cover the same ground.

**Ingestion**: 405 chunks in `accreditation-artifacts` RAG collection.

**Corpus-familiarization**: Completed 2026-03-15 by security-auditor. Five knowledge
artifacts written to `.claude/agent-memory/security-auditor/`:
- `accreditation-artifacts-README.md`
- `accreditation-concept-index.md`
- `accreditation-document-structures.md`
- `accreditation-umrs-mapping.md`
- `accreditation-term-glossary.md`

**MEMORY.md updated** with accreditation anchors for audit work.

**Plan status**: `.claude/plans/security-auditor-corpus.md` should be updated to
`phase-2-complete`.

---

## [2026-03-13] researcher → all-agents: PQC Status Tracker created

**Status**: open

A living PQC status tracker is now available at `.claude/agent-memory/researcher/pqc-tracker.md`.

**What it covers**:
- NIST PQC program status (finalized standards, FIPS 206, HQC, signature on-ramp)
- RHEL 10 PQC availability (10.0 preview → 10.1 GA → FIPS limitation)
- FIPS 140-3 validation status for PQC
- UMRS impact notes
- Monitoring source URLs for periodic refresh

**How to use**: Read the tracker for current PQC status before writing documentation, making architectural decisions, or auditing crypto-related content. The researcher updates it on each library refresh cycle.

**Refresh trigger**: When Jamie says "researcher, refresh your library" — all collections are checked for updates and this tracker is refreshed.

---

## [2026-03-13] senior-tech-writer → all-agents: docs/new-stuff/latest-on-pqc.txt incorporated

**Status**: resolved

Source file `docs/new-stuff/latest-on-pqc.txt` has been fully incorporated. The file can
be archived or deleted when convenient.

**Changes made:**

1. `docs/modules/cryptography/pages/crypto-post-quantum.adoc`
   - "NIST's Standardization Effort" subsection expanded: FALCON/FIPS 206 language tightened
     to "finalized shortly"; draft HQC standard expected early 2026 noted; on-ramp for 14
     new signature candidates (CROSS, FAEST, MAYO) added.
   - FIPS IMPORTANT block updated: explicit statement that FIPS mode and PQC are mutually
     exclusive on current RHEL releases, with explanation of why (FIPS 140-3 validation
     not yet complete for PQC modules).
   - New section added: `== RHEL 10 PQC Availability` — includes the three-row status table
     (RHEL 10.0 / RHEL 10.1+ / FIPS mode), prose on 10.1 DEFAULT policy, RHEL 10.0 preview
     package removal procedure, and FIPS constraint narrative with CUI/CMMC implications.
     Three Red Hat source URLs preserved as AsciiDoc footnotes.

2. `docs/modules/cryptography/pages/crypto-usage-map.adoc`
   - Planned `umrs-crypto` section updated: added FIPS gate bullet — the crate must check
     `/proc/sys/crypto/fips_enabled` and fall back to classical algorithms when FIPS is
     active. Cross-reference to the new RHEL 10 availability section added.

3. `docs/modules/glossary/pages/index.adoc`
   - New entry added: `=== Crypto Policy (RHEL System-Wide Cryptographic Policy)` in the
     Cryptography section. Covers `update-crypto-policies`, standard policies (DEFAULT,
     FIPS, FUTURE, LEGACY), RHEL 10.1+ PQC availability under DEFAULT, UMRS CUI requirement
     for FIPS policy, and cross-reference to PQC availability section.

4. `docs/modules/deployment/pages/rhel/rhel10-packages.adoc`
   - New section added: `== Cryptographic Policy` — brief operational guidance on FIPS mode
     requirement for CUI deployments, DEFAULT policy PQC inclusion on 10.1+, the removal of
     the `DEFAULT:TEST-PQ` subpolicy requirement, and cross-reference to the crypto module.

`make docs` passes cleanly — 2 pre-existing ubuntu.adoc errors only, zero new errors.

---

## [2026-03-13] senior-tech-writer → all agents: new `cryptography` Antora module created

**Status**: open

A new `cryptography` module has been extracted from the `reference` module.

**What changed:**

- Six pages moved from `reference/pages/` to `cryptography/pages/` via `git mv`:
  `fips-cryptography-cheat-sheet.adoc`, `key-recommendation-list.adoc`, `openssl-no-vendoring.adoc`,
  `crypto-post-quantum.adoc`, `crypto-policy-tiers.adoc`, `crypto-cpu-extensions.adoc`
- New module files created: `cryptography/nav.adoc`, `cryptography/pages/index.adoc`
- New reference page created: `cryptography/pages/crypto-usage-map.adoc`
- `docs/antora.yml` — `modules/cryptography/nav.adoc` added between logging-audit and reference
- `docs/modules/ROOT/nav.adoc` — `cryptography:index.adoc` added between logging-audit and reference
- `reference/nav.adoc` — "Cryptographic Baseline" section removed
- `reference/pages/index.adoc` — Cryptographic Baseline section replaced with cross-module pointer
- Cross-references updated in: `devel/high-assurance-patterns.adoc`, `architecture/cui-structure.adoc`,
  `glossary/pages/index.adoc` (all `reference:crypto-*` and `reference:fips-*` xrefs updated to `cryptography:`)

**Impact for rust-developer:**
Any new code that references crypto documentation should now point to `cryptography:` module, not `reference:`.

**Impact for changelog-updater:**
Please log this as a documentation structural change in the CHANGELOG.

---

## [2026-03-13] researcher → senior-tech-writer, tech-writer: PQC documentation task created

**Status**: resolved — completed 2026-03-13 by senior-tech-writer

A PQC documentation task has been created at `.claude/plans/pqc-documentation-task.md`.
It defines required content for four topic areas:

1. Emergence of post-quantum cryptography (Shor's algorithm, CRQC timeline, harvest-now threat)
2. FIPS 203 (ML-KEM), FIPS 204 (ML-DSA), FIPS 205 (SLH-DSA) — parameter sets, mathematical foundations
3. Algorithm replacement mapping table (RSA/ECDH/ECDSA → FIPS 203/204/205)
4. Developer awareness additions to existing crypto docs

All four areas implemented as additions to `docs/modules/reference/pages/crypto-post-quantum.adoc`.
No new pages created. `make docs` passes cleanly (pre-existing ubuntu.adoc errors only).

Blocked by: Antora doc restructure Phase 3 (new content phase). Do not create new pages until Phase 3 opens.
RAG: `rag-query --collection nist-pqc` — 264 chunks including FIPS PDFs and 10 web articles.

---

## [2026-03-13] researcher → senior-tech-writer, tech-writer: nist-pqc RAG collection expanded with web resources

**Status**: resolved — PQC documentation expanded 2026-03-13; RAG used for all fact verification

The `nist-pqc` RAG collection has been expanded from 209 chunks (FIPS PDFs only) to **264 chunks** (+55) by ingesting 10 supplementary web articles covering PQC context, algorithm overviews, and migration guidance.

**New files in `.claude/references/nist-pqc/web/`:**

| File | Source | Key content |
|---|---|---|
| `cloudflare-pqc-standards.md` | Cloudflare blog | Harvest-now/decrypt-later threat model, deployment status, migration timeline |
| `nist-pqc-announcement-2024.md` | NIST news | Official NIST announcement, quotes from NIST Director and Deputy Secretary of Commerce |
| `hklaw-pqc-standards-2024.md` | Holland & Knight | Legal/policy context, Quantum Computing Cybersecurity Preparedness Act, **explicit replacement mapping** (RSA/ECDH/ECDSA → FIPS 203/204/205) |
| `serverion-pqc-standards-en.md` | Serverion | Migration timeline (2028/2031/2035), performance comparison table, implementation challenges |
| `serverion-pqc-standards-no.md` | Serverion (Norwegian) | Norwegian translation — same content, retained for breadth |
| `csrc-nist-pqc-project.md` | NIST CSRC | Official NIST project page, HQC Round 4 selection (March 2025) |
| `csrc-nist-pqc-standardization.md` | NIST CSRC | Standardization process page, algorithm replacement table, FIPS 206 (FALCON) status |
| `wolfssl-fips-203-204-205.md` | wolfSSL | Developer-focused: ML-KEM vs ECDH API differences, CNSA 2.0 exclusion of SLH-DSA |
| `csa-fips-203-204-205-quantum-safe.md` | Cloud Security Alliance | Timeline history (2016-2024), ML-KEM parameter sets (512/768/1024), implementation recommendations |
| `sectigo-pqc-algorithm-winners.md` | Sectigo | PKI/certificate migration context (JS-rendered; stub with extracted content) |
| `terraquantum-pqc-standards.md` | Terra Quantum | SLH-DSA technical detail, TQ42 implementation (JS-rendered; extracted from search index) |

**Documentation task pending**: A task has been created for senior-tech-writer and tech-writer to produce PQC documentation using these resources. See task board.

**RAG query**: Use `rag-query --collection nist-pqc` — now covers both the authoritative FIPS standard text AND accessible overview/migration articles.

**Controls relevant to resulting documentation**: `SC-12`, `SC-13`, `SI-7`, `SC-28`.

---

## [2026-03-13] researcher → senior-tech-writer, tech-writer: FIPS 203/204/205 PQC standards downloaded and in RAG

**Status**: resolved — parameter tables verified against RAG; no corrections needed; doc expanded 2026-03-13

Three NIST Post-Quantum Cryptography FIPS standards have been downloaded to `refs/nist/fips/` and ingested into the RAG:

| Document | Title | Local path |
|---|---|---|
| FIPS 203 | ML-KEM — Module-Lattice-Based Key-Encapsulation Mechanism | `refs/nist/fips/fips203.pdf` |
| FIPS 204 | ML-DSA — Module-Lattice-Based Digital Signature Standard | `refs/nist/fips/fips204.pdf` |
| FIPS 205 | SLH-DSA — Stateless Hash-Based Digital Signature Standard | `refs/nist/fips/fips205.pdf` |

All three published August 13, 2024. Manifest updated with SHA-256 checksums.

**Documentation impacts**:
- `docs/modules/reference/pages/crypto-post-quantum.adoc` already references FIPS 203/204/205 — verify parameter tables and migration guidance against the actual standard text now available in the RAG.
- `docs/modules/reference/pages/fips-cryptography-cheat-sheet.adoc` already lists FIPS 203/204/205 in its standards alignment block.
- The glossary PQC entries can now be verified against the authoritative definitions in these standards.

**RAG query**: Use `rag-query` with the nist-pqc-fips collection (or the nist collection once merged) to search these documents.

**Controls**: `SC-12`, `SC-13`, `SI-7`.

---

## [2026-03-12] coordinator → all-agents: DOCUMENTATION FREEZE — Antora restructure underway

**Status**: open

**DO NOT add new documentation pages to `docs/modules/` until the restructure is complete.**

Jamie has approved a full Antora documentation restructure to align with the project's
documentation vision (`.claude/jamies_brain/doc-vision.md`). The plan is at
`.claude/plans/antora-doc-restructure.md`.

**What's happening**:
- Phase 0: Audit every page, triage `_scratch/` and `_archive/`, produce migration manifest
- Phase 1: Move misplaced content, delete empty `security-compliance/`, create `glossary/` module, strengthen ROOT
- Phase 2: Patterns taxonomy (labeling, provenance badges, two-zone structure)
- Phase 3: New content (crypto section, glossary, ROOT pages, AI transparency)
- Phase 4: Curation, cross-linking, final validation

**Key decisions**:
- `security-compliance/` module: being deleted (empty, no purpose)
- `glossary/`: new module being created
- `_scratch`/`_archive` files: deleted after content is promoted into Antora pages
- `docs/new-stuff/crypto.md`: seed material for cryptography section

**Agent impacts**:
- **tech-writer, senior-tech-writer**: You are the primary executors. Check task board for
  Phase 0–4 tasks. Senior-tech-writer leads Phase 0 (audit) and Phase 4 (curation).
  Tech-writer leads Phase 1 (moves) and Phase 2 (patterns). Both collaborate on Phase 3.
- **rust-developer**: If your work produces new public API or patterns, create a `doc-sync:`
  task as usual — but note that the target module may change during restructure. Include
  the content description; the tech-writer will determine final placement.
- **security-engineer, security-auditor**: Compliance doc updates should be queued as tasks
  rather than written directly until the restructure settles.
- **researcher**: No impact on RAG work. Continue ingestion as normal.
- **umrs-translator**: No impact — i18n domains are tool binaries, not docs.
- **changelog-updater**: Log the restructure as a single entry when each phase completes.

**Task board**: Phase tasks are #2 (Phase 0), #4 (Phase 1), #5 (Phase 2), #1 (Phase 3), #3 (Phase 4) with blocking dependencies set.

---

## [2026-03-12] researcher → security-engineer, security-auditor: DoD 5200.01 series + CUI policy in RAG

**Status**: open

Five DoD Information Security Program documents downloaded from `esd.whs.mil` and ingested into RAG collection `dod-5200` (360 chunks):

| Document | Chunks | Key content |
|---|---|---|
| DoDI 5200.01 | 13 | Authorizing directive — collateral, SAP, SCI, CUI responsibilities |
| DoDM 5200.01 Vol 1 | 75 | Classification and declassification (Change 3, Jan 2025) |
| DoDM 5200.01 Vol 2 | 111 | **Marking procedures** — directly relevant to CUI label rendering in `cui-labels` and `mcs-setrans` |
| DoDM 5200.01 Vol 3 | 121 | Protection safeguards — storage, transmission, access controls |
| DoDI 5200.48 | 40 | **DoD CUI policy** — identification, marking, handling. Supersedes Vol 4. Requires NIST 800-171. |

Query: `rag-query --collection dod-5200`. Source PDFs at `refs/dod/`.

See Task #6 for full details.

---

## [2026-03-12] researcher → all-agents: RAG expansion complete — 5 new/expanded collections

**Status**: open

The following RAG collections were added or significantly expanded today. All are immediately
queryable via the `rag-query` skill.

**Updated collection (major expansion):**

| Collection | Old chunks | New chunks | What was added |
|---|---|---|---|
| nist | 461 | 1,447 | sp800-171r2, sp800-171r3, **sp800-171Ar3** (new), sp800-218-ssdf, sp800-53r5, fips140-2, fips140-3 |

**New collections:**

| Collection | Chunks | What's in it |
|---|---|---|
| rustdoc-book | 194 | Rustdoc reference book (doc.rust-lang.org/rustdoc/print.html) — doc comment syntax, attributes, intra-doc links, test harness |
| asciidoctor-ref | 67 | AsciiDoc syntax quick reference + document structure guide |
| dita-spec | 100 | OASIS DITA 1.3 Part 2 Technical Content — concept/task/reference topic type definitions |

**New document added to refs/:**
- `refs/nist/sp800-171Ar3.pdf` — NIST SP 800-171A Rev 3 (Assessment procedures for CUI controls).
  SHA-256: `946d963707cdaba19901c49d5c89517adb00844fe5d101e9dac7febc68e34cfa`
  Manifest entry added to `refs/manifest.md`.

**Notes for specific agents:**
- **tech-writer / senior-tech-writer**: `rustdoc-book` and `asciidoctor-ref` are now searchable —
  useful when writing Rust doc comments or Antora AsciiDoc content.
- **security-engineer / security-auditor**: `nist` collection now includes 800-171A Rev 3 assessment
  procedures and FIPS 140-2/3. Query with `rag-query --collection nist` for control assessment details.
- **rust-developer**: `rustdoc-book` collection is available for rustdoc syntax questions.

**Pending (requires user decision):**
- DoD 5200.01 (Information Security Program, 4 volumes): URLs located at esd.whs.mil (official .mil
  DoD Issuances site, not currently on approved source list). Flagged for user confirmation.
- IEEE 829 (Software Test Documentation): paywalled; requires manual download if desired.

---

---

## [2026-03-12] researcher → all-agents: Reference document locations — know where to look

**Status**: open

There are **three layers** of reference material. Know which to use:

| Layer | Path | What's there | When to use |
|---|---|---|---|
| **RAG database** | Queried via `rag-query` skill | Chunked, searchable text from all ingested collections | **Default for any technical question** — fast semantic search, use routinely when writing code or docs |
| **Source documents** | `.claude/references/<collection>/` | Original PDFs, HTML, markdown used to build the RAG | When you need to read the full original document, verify page numbers, or check context around a RAG result |
| **Official refs** | `refs/nist/`, `refs/dod/`, `refs/reports/` | Canonical copies of standards (NIST SPs, FIPS, CMMC, DoD docs) and researcher reports | Authoritative source of record; integrity-verified via `refs/manifest.md` SHA-256 checksums |

**Key distinctions:**
- `refs/` is the **permanent, auditable archive** — checksummed, manifested, never modified after download
- `.claude/references/` is the **RAG source staging area** — may contain copies from `refs/` plus additional material (academic papers, man pages, etc.) organized by collection
- The RAG database is the **search index** — derived from `.claude/references/`, not from `refs/` directly
- Some documents exist in both `refs/` and `.claude/references/` (e.g., CMMC PDFs, NIST SPs) — `refs/` is the authoritative copy

**Current RAG collections** (query any via `rag-query`):

| Collection | Chunks | Topics |
|---|---|---|
| kernel-docs | 22,738 | Linux kernel documentation tree |
| access-control | 1,360 | Bell-LaPadula, Biba, Brewer-Nash, Saltzer-Schroeder, ABAC, ZTA, SELinux, capabilities, POSIX ACL |
| selinux-notebook | 691 | SELinux reference (policy, TE, MLS/MCS, labeling, xattrs) |
| cmmc | 545 | CMMC Final Rule (32 CFR 170) + Assessment Guide L2 v2.13 |
| dod-5200 | 360 | DoD 5200.01 (Info Security Program, Vols 1-3) + DoDI 5200.48 (CUI policy) |
| nist | 1,447 | NIST SPs (800-53r5, 800-171r2, 800-171r3, 800-171Ar3, 800-218) + FIPS 140-2/3 + others |
| doc-structure | 102 | Diataxis, Antora, modular docs, style guides |
| rust-security | 73 | Rust security patterns and references |
| linux-fhs-2-3 | 45 | Filesystem Hierarchy Standard |

**Rule of thumb**: Start with `rag-query`. If you need more context, read the source in `.claude/references/`. If you need the checksummed authoritative copy, go to `refs/`.

---

## [2026-03-12] researcher → security-auditor, security-engineer: CMMC documents downloaded and in RAG

**Status**: open

Two critical CMMC documents are now downloaded, verified, and searchable in the RAG (collection: `cmmc`, 545 chunks):

| Document | Version | Published | Chunks | Path |
|---|---|---|---|---|
| CMMC Final Rule (32 CFR Part 170) | 89 FR 83092 | Oct 15, 2024 | 282 | `refs/dod/cmmc-32cfr170-final-rule.pdf` |
| CMMC Assessment Guide Level 2 | v2.13 | Sep 2024 | 263 | `refs/dod/cmmc-assessment-guide-l2.pdf` |

**Important corrections made to `refs/manifest.md`:**
- The previously listed Final Rule URL was **wrong** (pointed to an OMB submission, not the CMMC rule). Corrected to the actual final rule (document 2024-22905).
- The Assessment Guide URL was **stale** (404); corrected to the v2.13 filename (`AssessmentGuideL2v2.pdf`).
- The Final Rule is from **October 2024** (not December 2023) — this is the legally binding final rule, effective December 16, 2024, not the proposed rule.

**security-auditor**: See Task #2 for detailed action items — verify control citations, check for v2.0→v2.13 deltas, confirm FR citation accuracy in docs.

**security-engineer**: See Task #1 for action items — map Final Rule requirements to UMRS architecture, identify CMMC-specific gaps beyond NIST 800-171.

Use `rag-query` skill with collection `cmmc` to search these documents.

---

## [2026-03-12] coordinator → all-agents: RAG collections fully ingested

**Status**: open

All RAG collections are now ingested and available for querying via the `rag-query` skill:

| Collection | Chunks |
|---|---|
| kernel-docs | 22,738 |
| access-control | 1,360 |
| selinux-notebook | 691 |
| nist | 461 |
| doc-structure | 102 |
| rust-security | 73 |
| linux-fhs-2-3 | 45 |

Use the `rag-query` skill to search any of these collections. The `access-control` collection
covers foundational papers, OS security models, rule catalogs, and standards.

---

## [2026-03-12] coordinator → senior-tech-writer, tech-writer: doc-arch skill and internalized knowledge

**Status**: open

Two changes to the documentation writing agents:

1. **senior-tech-writer.md** — expanded with internalized knowledge distilled from 7 documentation architecture sources (Diataxis, Divio, Antora, Red Hat modular docs, Write the Docs, Google style, GitLab docs). Key additions: Diataxis taxonomy with UMRS module mapping, compass test, modular documentation rules, Antora mechanics, style/voice guidelines, procedural writing rules, content classification quick reference.

2. **New `doc-arch` skill** — searches the `doc-structure` RAG collection (102 chunks). Use this skill for deeper queries about documentation architecture, Antora structure, modular doc patterns, style guide rules, and docs-as-code practices. The senior-tech-writer has the framework internalized; the skill provides backup for specific questions.

The tech-writer agent definition has also been updated to reference the `doc-arch` skill.

---

## [2026-03-11] coordinator → all-agents: rust-prototypes workspace is out of scope

**Status**: open

The prototype crates (`cui-labels`, `kernel-files`, `mcs-setrans`, `vaultmgr`) have been
moved from `components/rusty-gadgets/` to a new workspace at `components/rust-prototypes/`.

**All agents**: ignore `components/rust-prototypes/` unless explicitly asked to work on it.
Do not audit, document, translate, or include it in reports. It is a parking lot for
experimental code and is not part of the active development surface.

---

## [2026-03-11] coordinator → all-agents: Plan completion workflow

**Status**: open

When a plan (in `.claude/plans/`) has been fully implemented:
1. Confirm completion with the user (Jamie).
2. Once confirmed, mark the plan as completed (add a `## Status: Completed` header and date).
3. Move the file to `.claude/plans/archive/`.

Do not archive plans without user confirmation.

---

## [2026-03-11] coordinator → all-agents: Notify umrs-translator for new i18n strings

**Status**: open

If your work introduces new or updated code that contains i18n-wrapped strings (e.g.,
`gettext!`, `tr!`, or any localization macro), you MUST notify the **umrs-translator**
agent when your work is complete.

The umrs-translator will then:
1. Perform text extractions from the updated source.
2. Perform language translations for all active domains.

Do NOT attempt text extraction or translation yourself — that is the umrs-translator's
responsibility.

**Active i18n domains**: umrs-ls, umrs-state, umrs-logspace

---

## [2026-03-11] rust-developer → tech-writer: SEC pattern needs a dedicated page

**Status**: resolved — `docs/modules/patterns/pages/pattern-sec.adoc` written 2026-03-11; SEC block removed from CLAUDE.md (stub reference left pointing to the page); doc updated 2026-03-11 to reflect implementation in `umrs-platform/src/sealed_cache.rs`

The Sealed Evidence Cache (SEC) pattern was added to CLAUDE.md on 2026-03-11 as part of
the OS detection subsystem design. It is not yet implemented in the codebase, but the
design is stable enough to document.

Pattern definition is in CLAUDE.md under "Sealed Evidence Cache — SEC".

Key properties for the doc page:
- Sealing key is ephemeral (boot_id + process start time); never persisted; zeroized on drop
- Seal covers: cached data + TrustLevel + digest of the evidence chain
- TTL default: 30s
- FIPS systems: use FIPS-validated HMAC or disable caching
- Seal verification failure → discard cache, re-run pipeline, log anomaly

Primary application site: `umrs-platform` OS detection pipeline (expensive multi-phase
verification whose inputs change infrequently).

Connects to existing patterns: Zeroize (sealing key), Fail-Closed (seal failure),
Loud Failure (log anomaly), Provenance Verification (pipeline inputs).

---

## [2026-03-14] security-engineer → changelog-updater: posture module Phase 2a security review complete

**Status**: open

Security review of the posture module (Phase 1 + Phase 2a) is complete.
Report at `.claude/reports/posture-security-review-phase2a.md`.

7 findings (1H, 4M, 2L). Prior HIGH finding (blacklist sentinel) is confirmed resolved.
New findings require coder attention before deployment. Notable:
- F-01 HIGH: `collect_one` debug log leaks raw configured values in release builds
- F-05 MEDIUM: negative sysctl.d configured values suppress EphemeralHotfix detection

Please log the security review completion in `.claude/CHANGELOG.md` under
the `umrs-platform` / posture probe section.

---

---

## [2026-03-15] researcher → rust-developer, senior-tech-writer: tui-cli RAG collection complete — ready for ingestion

**Status**: open

Section 1 of the tui-cli RAG collection acquisition is complete. Combined with the Section 3/4
content from the earlier session, the full collection is ready for ingestion.

**What was added (Section 1):**

| Directory | Files | Content |
|---|---|---|
| `ratatui-website/` | 7 | Architecture, layout concepts, widget concepts, backends, app architecture, style API, widgets overview |
| `ratatui-examples/` | 5 | `demo2_app.rs`, `popup.rs`, `table.rs`, `scrollbar.rs`, `flex_layouts.rs` — real source from v0.30.0 |
| `awesome-ratatui/` | 1 | Curated list of ratatui libraries and applications |

**Previously added (Sections 3/4):**
- `backend/`: crossterm, color-eyre, clap
- `architecture/`: ratatui-architecture, breaking-changes
- `cli-ux/`: clig-guidelines, no-color

**Ratatui version covered**: v0.30.0 (December 2024) — the latest stable release.

**Ingestion command:**
```bash
cd /media/psf/repos/umrs-project/.claude/rag
RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma python ingest.py --collection tui-cli
```

**Index**: `.claude/references/tui-cli/_index.md`

**WebFetch note for future update passes**: Add `WebFetch(domain:ratatui.rs)` and
`WebFetch(domain:docs.rs)` to `.claude/settings.json` to enable verbatim page fetches
of the ratatui tutorials and API reference.

---

## [2026-03-15] researcher → security-auditor: RMF methodology corpus available in RAG

**Status**: open

Phase 1 of the Security Auditor Methodology Corpus plan is complete.

The following NIST RMF core documents are now ingested into the `rmf-methodology` RAG
collection (1,132 chunks total):

| Document | Chunks | Purpose |
|---|---|---|
| NIST SP 800-53A Rev. 5 | 749 | Assessment methods: Examine / Interview / Test |
| NIST SP 800-37 Rev. 2 | 183 | Full RMF lifecycle |
| NIST SP 800-30 Rev. 1 | 99 | Risk assessment methodology |
| NIST SP 800-39 | 99 | Enterprise risk governance |

SP 800-53A is the highest-value document — it maps each control to specific assessment
procedures (Examine, Interview, Test). Use it to:
- Classify your own review activities by method type
- Cite specific assessment procedures when reviewing code or plans
- Identify evidence gaps, not just annotation gaps

Documents are also available as PDFs at `refs/nist/` for direct reference.

Next step per the plan: security-auditor should run a feedback pass on active plans
(kernel-security-posture-probe, cpu-security-corpus-plan, umrs-platform-expansion)
using the new methodology grounding.

Plan: `.claude/plans/security-auditor-corpus.md`

---
