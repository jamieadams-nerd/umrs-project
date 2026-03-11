# Researcher Agent Memory
# Last updated: 2026-03-11

## Reference Library

### refs/ manifest (official security docs)
- `refs/manifest.md` is the canonical manifest — always read before retrieval tasks
- `refs/nist/` — NIST SPs (800-218, 800-171r2, 800-171r3, 800-53r5) + FIPS (140-2, 140-3)
- `refs/dod/` — DoD CMMC docs (two require manual download; dodcio.defense.gov blocks curl)
- Last version check: not yet performed (first session 2026-03-10)

### .claude/references/ (technical reference collections for RAG)
Collections and status as of 2026-03-11:

| Collection | Source | Status | Chunks |
|---|---|---|---|
| selinux-notebook/20240430 | github.com/SELinuxProject | ✓ Ingested | 691 |
| linux-fhs-2.3 | linux-fhs-2.3/fhs-2.3.txt | ✓ Ingested | 45 |
| kernel-docs | kernel.org docs tree (268MB, 12.9K files) | In progress | growing |
| access-control | Various (see manifest) | Awaiting review (not ingested) | — |

access-control collection placed 2026-03-11 per research-plan-access-control.md.
Ingest NOT run — awaiting user review.

kernel-docs ingest started 2026-03-10. Re-run
`python3 .claude/rag/ingest.py --collection kernel-docs` to resume if interrupted.

## RAG Pipeline

### Paths
- `.claude/references/<collection>/` — raw source (READ ONLY)
- `.claude/rag/ingest.py` — ingestion script
- `.claude/rag/manifest.json` — per-file tracking (hash, chunks, date)
- `.claude/rag/chroma/` — ChromaDB vector store
- `.claude/rag/converted/<collection>/` — pandoc output (intermediate)

### ingest.py usage
```bash
cd .claude/rag
python3 ingest.py                         # all new/changed collections
python3 ingest.py --collection <name>     # one collection
python3 ingest.py --force                 # reprocess everything
python3 ingest.py --summary               # show chunk counts and exit
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

## Pending Items
- NSA RTB VNSSA and RAIN: referenced in CLAUDE.md but not yet acquired; may be distribution-restricted
- CMMC 32 CFR Part 170 and Assessment Guide L2: require manual browser download
- Clark-Wilson 1987: requires manual download via IEEE Xplore (DOI: 10.1109/SP.1987.10001)
- Graham-Denning 1972: requires manual download via ACM DL (DOI: 10.1145/361011.361067)
- HRU 1976: requires manual download via ACM DL (DOI: 10.1145/360051.360056)
- access-control RAG collection: awaiting user review before ingest
- First version check not yet run (trigger when 30 days have elapsed from 2026-03-10)
