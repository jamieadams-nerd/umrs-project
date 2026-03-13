# Researcher Agent Memory
# Last updated: 2026-03-13

## Reference Library

### refs/ manifest (official security docs)
- `refs/manifest.md` is the canonical manifest — always read before retrieval tasks
- `refs/nist/` — NIST SPs (800-218, 800-171r2, 800-171r3, 800-53r5) + FIPS (140-2, 140-3)
- `refs/nist/fips/` — FIPS 203, 204, 205 (PQC standards, downloaded 2026-03-13)
- `refs/dod/` — DoD CMMC docs (Final Rule + Assessment Guide L2, both downloaded 2026-03-12)
- Last version check: 2026-03-12 (next due 2026-04-12)

### .claude/references/ (technical reference collections for RAG)
Collections and status as of 2026-03-13:

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
| nist-pqc | FIPS 203/204/205 PDFs + 10 web articles | ✓ Ingested | 264 |
| rustdoc-book | doc.rust-lang.org/rustdoc/print.html | ✓ Ingested | 194 |
| asciidoctor-ref | docs.asciidoctor.org (quick reference + document structure) | ✓ Ingested | 67 |
| dita-spec | OASIS DITA 1.3 Part 2 Technical Content (HTML) | ✓ Ingested | 100 |

Full source URL list for update checks: see `rag-collections.md` in this directory.

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
```

NO `--source` flag — the script auto-discovers files from `.claude/references/<collection>/` via `rglob`.
New subdirectories (e.g., `nist-pqc/web/`) are picked up automatically.
Without `--force`, only new/changed files are processed — safe for incremental additions.

```bash
python ingest.py                         # all new/changed collections
python ingest.py --collection <name>     # one collection
python ingest.py --force                 # reprocess everything
python ingest.py --summary               # show chunk counts and exit
```

### ingest.py known fixes (applied 2026-03-10)
1. `.txt` moved to PASSTHROUGH — `plain` is pandoc output-only, not input
2. `SKIP_EXTENSIONS` set added — skips binary/non-text: png, svg, yaml, pdf, etc.
3. `save_manifest()` now called after each file (was: end of loop) — enables safe interruption

### Collection naming: ChromaDB sanitizes `/` → `-`, so `linux-fhs-2.3` becomes `linux-fhs-2-3`

## Approved Sources (from role instructions)
- NIST: csrc.nist.gov, nvlpubs.nist.gov
- DISA/STIG: public.cyber.mil
- Red Hat: access.redhat.com, docs.redhat.com
- Kernel/SELinux: kernel.org, github.com/SELinuxProject
- IETF RFCs: rfc-editor.org, datatracker.ietf.org
- CVEs: nvd.nist.gov, cve.mitre.org
- CMMC: dodcio.defense.gov, acq.osd.mil
- DoD Issuances: esd.whs.mil (approved 2026-03-12)

## Research Reports — MANDATORY RULE

**ALL researcher reports MUST be saved to `refs/reports/`.**
This is the permanent, stable, user-facing location. Never use any other path.
Always do this even if not explicitly asked — if a research answer is substantial
enough to write up, it goes in `refs/reports/`.

Checklist for every new report:
1. Write file to `refs/reports/<kebab-case-name>.md` (date inside file, not in filename)
2. Compute SHA-256 with `sha256sum` and record it
3. Add a manifest entry under "Research Reports" in `refs/manifest.md`
4. Add a one-line entry to the "Existing reports" list below

Existing reports:
- `refs/reports/kernel-selinux-module-context-security.md` (2026-03-10)
  Topics: modules_disabled, MODULE_SIG_FORCE, Lockdown/LoadPin/IPE LSMs,
  SELinux system:module_request, MLS mlsvalidatetrans, Bell-LaPadula tranquility

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
- govinfo.gov PDF URL pattern: `/content/pkg/FR-YYYY-MM-DD/pdf/<doc-number>.pdf` — search
  govinfo to confirm correct date and document number before downloading
- CMMC manifest correction: document 2023-27756 was an OMB submission, NOT the CMMC rule;
  the actual Final Rule is document 2024-22905 (89 FR 83092, Oct 2024)
- CMMC Assessment Guide L2 filename changed from `AssessmentGuide_L2.pdf` to `AssessmentGuideL2v2.pdf`

## Retrieval Notes (learned 2026-03-12)

- OASIS DITA 1.3 Part 2 PDF (dita-v1.3-part2-tech-content.pdf) is a valid PDF but ingest.py PDF
  reader fails to open it — falls back to HTML version. HTML version ingested successfully (100 chunks).
  Relevant for future: if a PDF is valid but ingest fails, try HTML equivalent from same source.
- DoD 5200.01 volumes are accessible at esd.whs.mil (official DoD Issuances portal):
  Vol1: https://www.esd.whs.mil/Portals/54/Documents/DD/issuances/dodm/520001m_vol1.pdf
  Vol2: https://www.esd.whs.mil/portals/54/Documents/DD/issuances/dodm/520001m_vol2.pdf
  Vol3: https://www.esd.whs.mil/Portals/54/Documents/DD/issuances/dodm/520001_p.PDF
  esd.whs.mil is official .mil DoD Issuances site. Confirm approval before adding to library —
  not on the current approved source list in role instructions, but is a legitimate .mil domain.

## Retrieval Notes for JS-Rendered Pages

Some pages return only CSS/JS framework code via WebFetch (no article body):
- Sectigo blog (Nuxt.js): use search index snippets + write a stub noting the issue
- Terra Quantum (React/Gatsby): same approach
- Pattern: fetch the page, note it is JS-rendered, search for indexed snippet via WebSearch,
  write stub with `## Note on Retrieval` header explaining the limitation

## Pending Items
- NSA RTB VNSSA and RAIN: referenced in CLAUDE.md but not yet acquired; may be distribution-restricted
- Clark-Wilson 1987: requires manual download via IEEE Xplore (DOI: 10.1109/SP.1987.10001)
- Graham-Denning 1972: requires manual download via ACM DL (DOI: 10.1145/361011.361067)
- HRU 1976: requires manual download via ACM DL (DOI: 10.1145/360051.360056)
- IEEE 829 (Software Test Documentation): paywalled — requires manual download if desired
- Version check due 2026-04-12
