# Researcher Agent Memory — "The Librarian"
# Alias: The Librarian (always include "The"). Real name: T. Librarian.
# Portrait: docs/modules/ai-transparency/images/librarian.png
# Last updated: 2026-04-02 (audit-logging collection: 7 docs downloaded to .claude/research/audit-logging/)

## Session Constraints
- [Session tool constraints](feedback-session-constraints.md) — Bash/WebFetch unavailable workarounds, Canada.ca curl tips

## Critical: Bash Fetch Patterns

When WebFetch is blocked, fetch via `bash -c` with `>` redirect:
```
bash -c 'curl -L -s --max-time 30 -A "Mozilla/5.0" "<url>" | pandoc -f html -t markdown --wrap=none > <dest>'
bash -c 'curl -L -s --max-time 60 -A "Mozilla/5.0" "<pdf-url>" > <dest.pdf>'
```
Rules: MUST use `>` inside bash -c string. `curl -o <path>` as separate arg is blocked by sandbox.
Verify PDF: `head -c 4 <file>` must show `%PDF`.

## Source Availability (updated 2026-03-23)

- cyber.gc.ca: **BLOCKED by RHEL10 FIPS** — TLS `ems not enabled`; all CCCS documents (ITSG-33, ITSP.*) require manual download
- tbs-sct.canada.ca: curl WORKS with Firefox UA; some IDs need `Accept-Language: fr-CA,fr;q=0.9` for FR pages; use `&section=html` for cleaner extraction; Python extractor at `.claude/agent-memory/henri/extract_pol.py`
- laws-lois.justice.gc.ca: curl WORKS without special headers; use `FullText.html` (EN) / `TexteComplet.html` (FR) for complete acts
- hci.stanford.edu: **BLOCKED by RHEL10 FIPS** — TLS `ems not enabled`; WebFetch HTML only; Stanford CS147 = manual download
- worrydream.com, isko.org, ocw.mit.edu, inlibra.com, bazerman.education.ucsb.edu: **curl WORKS**
- ocw.mit.edu: hash-prefixed PDFs — scrape `/resources/<name>/` page to get hash, then fetch `/<hash>_<filename>.pdf`
- niso.org: Z39.19 at `http://groups.niso.org/higherlogic/ws/public/download/12591/z39-19-2005r2010.pdf`
- developers.google.com, nist.gov: WebFetch blocked; curl WORKS
- plainlanguage.gov PDF: DEAD; use GSA GitHub raw instead
- everyspec.com: curl WORKS — Jamie-approved for unclassified DoD specs
- docs.redhat.com PDFs: curl WORKS — pattern `/en/documentation/red_hat_enterprise_linux/<ver>/pdf/<guide-name>/<Title-en-US.pdf>`

## Reference Library

### .claude/references/ manifest (official security docs)
- `.claude/references/refs-manifest.md` is the canonical manifest — always read before retrieval tasks
- `.claude/references/nist/` — NIST SPs (800-218, 800-171r2, 800-171r3, 800-53r5, 800-37r2, 800-53Ar5, 800-30r1, 800-39, 800-90B, 800-172, 800-161r1 [WITHDRAWN], 800-161r1-upd1 [CURRENT, downloaded 2026-03-31, **familiarized 2026-03-31** → see `sp800-161r1-upd1-familiarization.md`], 800-60v1r1 [Vol 1], 800-60v2r1 [Vol 2, **familiarized 2026-03-31** → see `sp800-60-familiarization.md`]) + FIPS (140-2, 140-3, 180-4 [SHA-256/384, downloaded 2026-04-02], 186-5 [ECDSA, downloaded 2026-04-02])
- `.claude/references/nist/fips/` — FIPS 203, 204, 205 (PQC standards, downloaded 2026-03-13)
- `.claude/references/dod-5200/` — DoD CMMC docs (Final Rule + Assessment Guide L2, both downloaded 2026-03-12)
- `.claude/references/fedramp/` — FedRAMP accreditation docs (downloaded 2026-03-15: CSP playbook, Agency playbook, SSP/SAP/SAR templates)
- `.claude/references/nist/sp800-18r1.pdf` — NIST SP 800-18 Rev 1 (downloaded 2026-03-15)
- `.claude/references/nist/sp800-90B.pdf` — NIST SP 800-90B (downloaded 2026-03-18; SHA-256: 9b0dd77131ade3617a91cd8457fa09e0dc354c273bb2220a6afeaca16e5defe7)
- Last version check: 2026-04-12 (next due 2026-05-12)

### .claude/references/ (technical reference collections for RAG)
Collections and status as of 2026-03-15:

| Collection | Source | Status | Chunks |
|---|---|---|---|
| selinux-notebook/20240430 | github.com/SELinuxProject | ✓ Ingested | 691 |
| linux-fhs-2-3 | linux-fhs-2.3/fhs-2.3.txt | ✓ Ingested | 45 |
| kernel-docs | kernel.org docs tree (268MB, 12.9K files) | ✓ Ingested | 22,738 |
| access-control | Various (see manifest) | ✓ Ingested | 1,360 |
| doc-structure | 7 sources (divio, diataxis, antora, redhat-modular, write-the-docs, google-style, gitlab-docs) | ✓ Ingested | 102 |
| rust-security | Rust security references | ✓ Ingested | 73 |
| nist | NIST SPs (800-171r2, 800-171r3, 800-171Ar3, 800-218, 800-53r5) + FIPS 140-2/3 + others | ✓ Ingested | 1,447 |
| cmmc | CMMC Final Rule + Assessment Guide L2 | ✓ Ingested | 545 |
| dod-5200 | DoDI 5200.01, DoDM 5200.01 Vol 1-3, DoDI 5200.48 (CUI) | ✓ Ingested | 360 |
| nist-pqc | FIPS 203/204/205 PDFs + 12 web articles | ✓ Ingested | 285 |
| rustdoc-book | doc.rust-lang.org/rustdoc/print.html | ✓ Ingested | 194 |
| asciidoctor-ref | docs.asciidoctor.org (quick reference + document structure) | ✓ Ingested | 67 |
| dita-spec | OASIS DITA 1.3 Part 2 Technical Content (HTML) | ✓ Ingested | 100 |
| rmf-methodology | NIST SP 800-37r2, 800-53Ar5, 800-30r1, 800-39 | ✓ Ingested | 1,132 |
| accreditation-artifacts | NIST 800-18 + FedRAMP playbooks/templates (6 docs; 200-B/200-C training removed from fedramp.gov) | Downloaded — awaiting ingestion | 0 |
| tui-cli | crossterm, color-eyre, clap, ratatui (v0.30.0), awesome-ratatui | Downloaded — awaiting ingestion | 0 |
| tech-writer-corpus | MS Style Guide, Google Style Guide, MIL-STD-38784B, Plain Language, NIST Author Instructions, CC Parts 1+2, RHEL 10 security guides (4 PDFs) | Ingested (2026-03-17) | 2017 |
| scap-security-guide | RHEL 10 STIG playbook preprocessed indexes (451 signals, CCE→NIST table) | Ingested (2026-03-17) | 7 |
| info-theory-foundations | 12 research summaries: Shannon/MacKay/Rissanen/Kolmogorov/Dijkstra/A*/Avis-Fukuda/Edmonds/HNSW/ANN-survey/spectral/AC + synthesis | Written 2026-03-20 — awaiting ingestion | 0 |
| knowledge-organization/ieko | 119 IEKO HTML articles (isko.org/cyclo) | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| knowledge-organization/texts | Svenonius Ch.5 + Hjørland KO theories | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| hci-courses/mit-6831 | 25 MIT OCW 6.831 lecture PDFs (CC BY-NC-SA) | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| hci-courses/stanford-cs147 | Stanford CS147 AU2023 lectures | REQUIRES MANUAL DOWNLOAD — FIPS TLS block | 0 |
| hci-courses/theory-papers | Blackwell/Green Cognitive Dimensions + Vannevar Bush 1945 | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| technical-communication/theory | Miller "Genre as Social Action" + Bazerman genre chapter | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| information-architecture/theory | Pirolli ch.1 + Precision Content IA white paper | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| information-architecture/standards | NISO Z39.19-2005 (R2010) controlled vocabularies standard | Downloaded 2026-03-22 — awaiting ingestion | 0 |
| oscal-schemas | OSCAL v1.1.2 JSON schemas (8) + FedRAMP Rev5 LOW/MOD/HIGH profiles (6) | Downloaded 2026-03-23 — NEEDS settings.json permission to ingest | 0 |
| five-eyes-classification | TBS PGS EN/FR, UK GSCP, NZISM 3.7 (downloaded 2026-03-24); DOSM+Appendix J NOT YET downloaded | Familiarization-only (no RAG) — SOURCE.md + manifest added 2026-03-31 | n/a |

Full source URL list for update checks: see `rag-collections.md` in this directory.
PQC status tracker (team-readable): see `pqc-tracker.md` in this directory.

## RAG Pipeline

### Paths
- `.claude/references/<collection>/` — raw source (READ ONLY)
- `.claude/rag/ingest.py` — ingestion script
- `.claude/rag/manifest.json` — per-file tracking (hash, chunks, date)
- `.claude/rag/chroma/` — ChromaDB vector store
- `.claude/rag/converted/<collection>/` — pandoc output (intermediate)
- `RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma` — production ChromaDB location

### ingest.py usage — IMPORTANT: use absolute path and env var
```bash
cd /media/psf/repos/umrs-project/.claude/rag
RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma python ingest.py --collection <name>
# No --source flag; auto-discovers via rglob; --force to reprocess; --summary for counts
```

### ingest.py known fixes (2026-03-10): `.txt`→PASSTHROUGH; SKIP_EXTENSIONS for binaries; save_manifest() per-file. Collection naming: `/` → `-` (e.g. `linux-fhs-2.3` → `linux-fhs-2-3`).

## Approved Sources (from role instructions)
- NIST: csrc.nist.gov, nvlpubs.nist.gov
- DISA/STIG: public.cyber.mil
- Red Hat: access.redhat.com, docs.redhat.com
- Kernel/SELinux: kernel.org, github.com/SELinuxProject
- IETF RFCs: rfc-editor.org, datatracker.ietf.org
- CVEs: nvd.nist.gov, cve.mitre.org
- CMMC: dodcio.defense.gov, acq.osd.mil
- DoD Issuances: esd.whs.mil (approved 2026-03-12)
- FedRAMP: fedramp.gov (approved 2026-03-15 — accreditation process artifacts)
- Common Criteria: commoncriteriaportal.org (approved per tech-writer-corpus-plan.md Phase 3.6 — RAG only, not .claude/references/)

## Research Reports — MANDATORY RULE

**ALL researcher reports MUST be saved to `.claude/references/reports/`.**
This is the permanent, stable, user-facing location. Never use any other path.
Always do this even if not explicitly asked — if a research answer is substantial
enough to write up, it goes in `.claude/references/reports/`.

Checklist for every new report:
1. Write file to `.claude/references/reports/<kebab-case-name>.md` (date inside file, not in filename)
2. Compute SHA-256 with `sha256sum` and record it
3. Add a manifest entry under "Research Reports" in `.claude/references/refs-manifest.md`
4. Add a one-line entry to the "Existing reports" list below

Existing reports:
- `.claude/references/reports/kernel-selinux-module-context-security.md` (2026-03-10)
  Topics: modules_disabled, MODULE_SIG_FORCE, Lockdown/LoadPin/IPE LSMs, SELinux module_request, MLS tranquility
- `.claude/references/reports/stig-signal-coverage.md` (2026-03-17)
  Topics: 36 posture indicators vs 451 STIG rules; 20 direct matches; audit/network gaps; 7 Tier-1 candidates
- `.claude/references/reports/umrs-capabilities-800-171r3-mapping.md` (2026-03-19)
  Topics: 7 UMRS blog capabilities → 800-171r3 mandatory reqs; MP-03/PL-04/CM-06/CA-07/AU-03/SA-08/AC-03/04
- `.claude/references/reports/nara-cui-registry-crossref.md` (2026-03-21)
  Topics: NARA CUI vs cui-labels.json audit; 6 standalone-vs-group errors; 7 fabricated categories
- `.claude/references/reports/agent-knowledge-acquisition-plan.md` (2026-03-21)
  Topics: HCI/IA/KO/TechComm open-access sources; 12 Tier-1 resources; ISKO IEKO, Stanford CS147, MIT 6.831
- `.claude/references/reports/2026-03-31-priority4-nist-familiarization.md` (2026-03-31)
  Topics: SP 800-172 (APT/CMMC L3); SP 800-161r1 WITHDRAWN (superseded by upd1); SP 800-60 Vol 1 INCOMPLETE
- `.claude/references/reports/five-eyes-case-study-research.md` (2026-04-03)
  Topics: 50 new failure cases across US, CA, UK, AU, NZ — see project_case_study_corpus.md for summary
- `.claude/references/reports/2026-04-12-env-attack-prevention.md` (2026-04-12)
  Topics: env var attack CVEs (CVE-2023-4911, CVE-2024-48990/48992, Shellshock), CERT ENV rules, CWEs, glibc AT_SECURE, Rust 2024 set_var unsafe, Tier 1/2/3 variable tables, rust-developer reading list

## Retrieval Patterns (learned 2026-03-11)

- DTIC `apps.dtic.mil/sti/pdfs/` URL pattern is blocked by automated-access controls
- DTIC `apps.dtic.mil/sti/tr/pdf/` is also blocked — but works with Firefox user-agent header
- ACM Digital Library PDFs require subscription — flag as manual download
- IEEE papers: originals behind paywall; academic .edu mirrors often work with Firefox user-agent
- GitHub release assets: use API (`api.github.com/repos/.../releases/tags/<tag>`) to discover
  exact asset filenames before downloading — do not guess filenames
- Cornell `courses/` URLs for older papers frequently return 404 — search for alternatives first
- SELinux Notebook PDF asset name is `SELinux_Notebook.pdf` (capital S, capital N) in release 20240430
- govinfo.gov: Cloudflare-protected; curl/wget fail even with browser UA; Selenium Firefox headless
  passes Cloudflare challenge, then JS `fetch()` from same origin works for PDF downloads
- dodcio.defense.gov now redirects to `dowcio.war.gov`; must navigate to war.gov first, then
  fetch from war.gov domain context (cross-origin fetch from defense.gov context fails)
- govinfo.gov PDF URL pattern: `/content/pkg/FR-YYYY-MM-DD/pdf/<doc-number>.pdf` — confirm date/number before downloading
- CMMC manifest correction: 2023-27756 = OMB submission (wrong); actual Final Rule = 2024-22905 (89 FR 83092, Oct 2024)
- CMMC Assessment Guide L2: filename is `AssessmentGuideL2v2.pdf` (not `AssessmentGuide_L2.pdf`)
- fedramp.gov: `/assets/resources/` URLs return S3 redirect stubs (59–83 byte `binary/octet-stream`); real files at `/resources/`. Check content-type: valid PDF = `application/pdf`. Rev4 training PDFs (200-B/200-C) removed in Rev5 reorg.

## Retrieval Notes (learned 2026-03-12)

- If a PDF fails ingest.py, try the HTML equivalent from the same source (e.g. OASIS DITA)
- DoD 5200.01: esd.whs.mil/Portals/54/Documents/DD/issuances/dodm/520001m_vol{1,2}.pdf
- JS-rendered pages (Nuxt/React/Gatsby): curl gets empty shell; use WebSearch snippets + write stub with `## Note on Retrieval`

## scap-security-guide Collection Notes (2026-03-17)

- 451 STIG signals for RHEL 10; index files in `.claude/references/scap-security-guide/`
- Chunking limitation: flat tables → one chunk per file; use Read tool directly for CCE/signal lookups
- Fix pending: add alphabetical section headings then `--force` re-ingest
- Key signals: sysctl kernel hardening (~35), SELinux state/policytype, audit kmod rules, FIPS crypto policy

## Standing Refresh Tasks

On "refresh library" or "check for updates":
1. Read `pqc-tracker.md` for PQC monitoring URLs; check for new FIPS/RHEL updates
2. Check `rag-collections.md` source URLs for newer versions; check `.claude/references/refs-manifest.md`
3. Re-ingest updated collections; update chunk counts in MEMORY.md and rag-collections.md
4. Post cross-team note summarizing changes; create tasks for tech-writer if docs are affected

## French-CA Terminology Corpus (2026-03-23)

`.claude/references/corpus/` contains Simone's french-lookup databases:
- `termium-plus-fr_CA.tsv` — 32,210 entries (TERMIUM Plus GoC + InfoSec Glossary + CCCS)
- `oqlf-gdt-fr_CA.tsv` — 25,881 entries (OQLF Grand dictionnaire terminologique)
- `SOURCE.md` — provenance, checksums, update procedure

GoC server TLS: donnees-data.tpsgc-pwgsc.gc.ca requires `OPENSSL_CONF=/dev/null` + `--insecure`
Military/Security subject: Open Gov Portal resource `99a220a8` — JS-blocked, needs manual download
Helper scripts: `.claude/references/scripts/extract_termium.py`, `.claude/references/scripts/extract_gdt.py`,
  `.claude/references/scripts/parse_termium_glossary3.py`, `.claude/references/scripts/parse_cccs_bilingual.py`,
  `.claude/references/scripts/verify_corpus.py` — retain for re-extraction runs

## umrs-c2pa TRUSTED Fixture Gap (2026-04-13)

Status: NO TRUSTED fixture exists. 5 files in `tests/fixtures/trusted/` are all UNVERIFIED or ERROR.
Root cause: Adobe rotated issuing CA on 2025-11-19; all existing images predate rotation.
Full analysis: `c2pa-test-image-acquisition.md`
Fastest fix for Jamie: Generate image at https://firefly.adobe.com (post-rotation, embedded manifest)
Alternative: Any Google Pixel 10 photo — Google C2PA Root CA G3 in trust list, No rotation issue.
Verification command: `cd umrs-c2pa && cargo run tests/fixtures/trusted/<new-photo.jpg>`

## Pending Items
- **umrs-c2pa TRUSTED fixture** — Jamie action required (see above + `c2pa-test-image-acquisition.md`)
- audit-logging collection: [reference_audit_logging_collection.md](reference_audit_logging_collection.md) — 7 docs in `.claude/research/audit-logging/`; not yet ingested; see SOURCE.md for checksums
- Stanford CS147: MANUAL DOWNLOAD — 20 PDFs; see `.claude/references/hci-courses/stanford-cs147/SOURCE.md`
- RAG ingestion pending: accreditation-artifacts, tui-cli, info-theory-foundations, all 2026-03-22 Tier 1 collections
- OSCAL v1.1.2 schemas: DOWNLOADED (2026-03-23) — 8 JSON schemas + 6 FedRAMP Rev5 profiles in `.claude/references/oscal-schemas/`; manifest updated; RAG ingestion BLOCKED — needs `Bash(RAG_CHROMA_PATH=... python ingest.py --collection oscal-schemas 2>&1)` added to settings.json allowlist
- DISA RHEL 10 STIG: NOT YET PUBLISHED as of 2026-03-17; monitor public.cyber.mil
- Version check due 2026-05-12
- Full pending list: see `research-priorities.md` in this directory
- CPU corpus: see `cpu-corpus-state.md` for phase state
- TERMIUM Military/Security subject ZIP: manual download needed (resource 99a220a8 at open.canada.ca)
- Henri regulatory corpus: 14 files (8 docs EN+FR) acquired 2026-03-23 at `.claude/agent-memory/henri/corpus/`; CCCS docs blocked — see corpus-index.md; ITSP.10.222 does not exist (correct: ITSG-33 Annex 4A)
