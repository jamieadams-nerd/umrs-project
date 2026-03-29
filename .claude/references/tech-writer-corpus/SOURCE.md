# Tech-Writer Corpus — Collection Index

**Collection:** tech-writer-corpus
**Created:** 2026-03-16
**Purpose:** Style guides, government writing standards, and domain reference materials for
RAG augmentation of the tech-writer and senior-tech-writer agents. Supports accurate
terminology, register, and structure for Five Eyes / federal security audience documentation.

**RAG collection name (ChromaDB):** tech-writer-corpus
**Plan:** `.claude/plans/tech-writer-corpus-plan.md`

---

## Sub-collections

### style-guides/microsoft/
Microsoft Writing Style Guide — 13 pages covering:
- Welcome and brand voice
- Top 10 tips for style and voice
- Bias-free communication
- Procedures and step-by-step instructions
- Word choice and simple sentences
- Scannable content, headings, lists
- Capitalization
- Global communications

Status: Phase 1.2 complete (2026-03-16)
Source: https://learn.microsoft.com/en-us/style-guide/

### style-guides/google/
Google Developer Documentation Style Guide — 2 files covering:
- About, highlights, core principles
- Procedures, text formatting, accessibility, link text, tables, code samples, lists

Status: Phase 1.1 partial (2026-03-16) — WebFetch blocked; content from WebSearch summaries.
Source: https://developers.google.com/style

### gov-standards/
Government and defense writing standards — 3 files:
- Federal Plain Language Guidelines (Revision 1, May 2011) — content from WebSearch summaries
- NIST Technical Series Publications Author Instructions — content from WebSearch summaries
- MIL-STD-38784B — REQUIRES MANUAL DOWNLOAD (see placeholder file)

Status: Phase 2 substantially complete (2026-03-16); MIL-STD-38784B needs manual acquisition.
Sources: plainlanguage.gov, nist.gov, quicksearch.dla.mil (CAC required)

---

### domain-refs/rhel-security-guide/
RHEL 10 official security documentation — 4 PDFs from docs.redhat.com:
- `rhel10-security-hardening.pdf` — FIPS, crypto policy, AIDE, Keylime, LUKS/Clevis, fapolicyd
- `rhel10-using-selinux.pdf` — SELinux core, MLS (Ch6), MCS (Ch7), custom policy, containers
- `rhel10-risk-reduction-recovery.pdf` — Audit system (Ch7), security monitoring (Ch8)
- `rhel10-kernel-management.pdf` — IMA/EVM (Ch24), module signing (Ch22), Secure Boot

Status: Phase 3.4 complete (2026-03-17) — 772 new chunks ingested; total collection: 2017 chunks
Sources: docs.redhat.com (approved source)

---

## Phase Status (as of 2026-03-17)

| Phase | Items | Status |
|-------|-------|--------|
| Phase 1.1 | Google Style Guide | Partial — curl+pandoc verbatim acquired (2026-03-16) |
| Phase 1.2 | Microsoft Writing Style Guide | Complete (2026-03-16) |
| Phase 2 | Gov standards (Plain Language, NIST Author, MIL-STD-38784B) | Substantially complete; MIL-STD manual download pending |
| Phase 3.4 | RHEL 10 Security Guides | Complete (2026-03-17) |
| Phase 3.5 | SELinux Project Notebook | CHECK — exists at .claude/references/selinux-notebook/ |
| Phase 3.6 | Common Criteria Parts 1 & 2 | Complete (2026-03-16) |
| Phase 4 | Apple, DigitalOcean, Mailchimp style guides | Not started |
| Phase 5 | NASA writing guidance | Not started |

---

## Pending Actions

1. **MIL-STD-38784B:** Requires manual download via CAC-authenticated ASSIST, OR user approval
   of everyspec.com as an approved source.

2. **Google Style Guide:** developers.google.com was blocked by WebFetch during acquisition.
   Try direct WebFetch in a future session, or use `curl` if Bash permissions are restored.

3. **Phase 3 check:** Before acquiring Phase 3 items, verify which already exist in
   `.claude/references/nist/`, `.claude/references/selinux-notebook/`, and `.claude/references/`.

4. **RAG ingestion (Phase 6):** When sufficient material is collected, run:
   ```bash
   cd /media/psf/repos/umrs-project/.claude/rag
   RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma python ingest.py --collection tech-writer-corpus
   ```
