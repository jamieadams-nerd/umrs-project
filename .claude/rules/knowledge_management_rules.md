## Knowledge Management Rules

Applies to The Librarian (researcher agent) and any agent involved in
acquiring, ingesting, or managing the UMRS reference library.

### Acquisition Rules

[RULE] Every acquired document must have a `SOURCE.md` manifest in its
collection directory recording: source URL, retrieval date, retrieval method,
and content description.

[RULE] Verify document integrity after download:
- PDFs: `head -c 4 <file>` must show `%PDF`
- HTML-to-markdown conversions: spot-check for empty or stub content
- Check `content-length` and `content-type` headers — reject redirect stubs

[RULE] All substantial research reports go to `.claude/references/reports/` with a
`.claude/references/refs-manifest.md` entry and SHA-256 checksum.

[CONSTRAINT] Only fetch from approved sources (listed in Librarian's memory).
New sources require Jamie's approval before first use.

### Ingestion vs Familiarization Decision

Not every reference needs RAG ingestion. Choose based on the material:

| Scenario | Action |
|---|---|
| Large corpus, many agents need it | Ingest into RAG, then familiarize the primary agent |
| Small/medium doc, one agent owns it | Familiarize only — agent reads and distills |
| Reference searched for specific passages | Ingest — RAG excels at retrieval |
| Reference applied as principles | Familiarize only — active knowledge, not retrieval |

[RULE] Do not default to ingesting everything. Each collection adds storage
cost and query noise. Ask: "Will agents need to search this, or just know it?"

[RULE] After RAG ingestion, the primary beneficiary agent MUST run the
`corpus-familiarization` skill before using the material. Without
familiarization, the agent has passive retrieval but no active knowledge.

### Collection Naming

[RULE] Collection names use kebab-case: `tui-cli`, `selinux-notebook`, `nist-pqc`.
ChromaDB sanitizes `/` to `-`, so avoid slashes in collection names.

[PATTERN] Use hierarchical directory names for related sub-collections:
`hci-courses/mit-6831`, `knowledge-organization/ieko`. The RAG collection
name flattens these: `hci-courses-mit-6831`.

### Storage Layout

- `.claude/references/<collection>/` — raw source documents (read-only after acquisition)
- `.claude/rag/manifest.json` — per-file tracking (hash, chunks, date)
- `.claude/rag/chroma/` — local ChromaDB (development)
- `RAG_CHROMA_PATH` env var — production ChromaDB location
- `.claude/references/nist/` — NIST standards (SPs, FIPS)
- `.claude/references/dod-5200/` — DoD standards and CMMC
- `.claude/references/fedramp/` — FedRAMP templates
- `.claude/references/reports/` — Librarian's research reports
- `.claude/references/refs-manifest.md` — provenance manifest for third-party standards

[CONSTRAINT] Never modify files in `.claude/references/` after acquisition.
They are the source of truth for re-ingestion.

### Refresh and Version Tracking

[RULE] Check `.claude/references/refs-manifest.md` source URLs for newer versions monthly
(or when Jamie requests a refresh).

[PATTERN] When a document is updated upstream:
1. Download the new version alongside the old (do not overwrite)
2. Summarize changes for the team
3. Re-ingest with `--force` after Jamie confirms
4. Update chunk counts in Librarian's memory and `rag-collections.md`
5. Post a cross-team note if the update affects documented content

### Retrieval Patterns

[PATTERN] When `curl` or `WebFetch` fails:
- Try Firefox user-agent: `-A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0"`
- For French pages: add `-H "Accept-Language: fr-CA,fr;q=0.9"`
- For government portals blocked by FIPS TLS: flag as manual download in `SOURCE.md`
- Never silently skip a failed download — document the failure and the workaround

[ANTI-PATTERN] Do not synthesize document content from search results and
present it as the original. If verbatim fetch fails, flag it and provide
re-fetch commands for the next session with appropriate access.

### Decommissioning

[RULE] Do not delete RAG collections without Jamie's approval. If a collection
is superseded, mark it in the manifest and leave it until Jamie confirms removal.
