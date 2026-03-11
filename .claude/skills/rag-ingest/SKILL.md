---
name: rag-ingest
description: >
  Ingests new or updated reference material into the UMRS RAG database for the
  researcher agent. Trigger this skill whenever the user asks to: add a
  collection to the RAG, ingest new documents, update the reference library,
  index new material, or says things like "add X to the RAG", "ingest the new
  collection", "the researcher should index this", or "update the reference
  database". Also triggers when the researcher agent detects a directory in
  .claude/references/ that is not yet present in the database.
---

# RAG Ingest Skill

Manages ingestion of reference documents into the ChromaDB RAG database.

## Paths (always absolute from repo root)

```
.claude/references/<collection>/   ← raw source docs (READ ONLY)
.claude/rag/ingest.py              ← ingestion script
.claude/rag/chroma/                ← ChromaDB database
.claude/rag/manifest.json          ← tracks what has been ingested
```

## Step 1 — Detect What Needs Ingesting

Before running anything, compare what exists on disk vs what is in the database:

```bash
# List directories in references/
ls .claude/references/

# List what is already in the database
python .claude/rag/query.py --list-collections
```

Collections present in `.claude/references/` but absent from
`--list-collections` output are NEW and need ingesting.

Collections present in both may have UPDATED files — ingest.py handles
this automatically via manifest hash checking.

Report your findings to the user before proceeding:
- "Found 1 new collection not yet in the database: nist-sp800-53"
- "Found 3 collections already indexed. Checking for updates."

## Step 2 — Ingest

### New single collection
```bash
python .claude/rag/ingest.py --collection <directory-name>
```

### All new/updated collections at once
```bash
python .claude/rag/ingest.py
```

### Force full reindex of a collection
```bash
python .claude/rag/ingest.py --collection <directory-name> --force
```

## Step 3 — Verify

After ingestion completes, always confirm results:

```bash
python .claude/rag/query.py --list-collections
```

Report the chunk counts per collection to the user.

## Step 4 — Notify the Team

After successful ingestion report a brief summary:

```
Ingested: <collection-name>
Files processed: N
Chunks stored: N
Collections now available: [list all]
```

## Error Handling

| Symptom | Likely cause | Fix |
|---|---|---|
| `Database not found` | ingest hasn't been run yet | Run `ingest.py` without `--collection` first |
| `WARNING: no chunks produced` | file too short or binary | Check file content, add to SKIP_EXTENSIONS if binary |
| `pandoc failed` | unsupported format or corrupt file | Check file, convert manually to .md first |
| Hangs on a file | binary file not in SKIP_EXTENSIONS | Ctrl+C, add extension to SKIP_EXTENSIONS in ingest.py |

## Long Running Ingests

For large collections (kernel-docs or any collection over 1000 files),
always run detached so the process survives session exit:

nohup python3 .claude/rag/ingest.py --collection <name> \
  > .claude/rag/ingest.log 2>&1 &

echo "PID: $!"

Report the PID to the user so they can monitor or kill if needed.


## Notes

- Raw source files in `.claude/references/` are NEVER modified
- Re-running on an unchanged file is safe — manifest hash check skips it
- Converted `.md` files land in `.claude/rag/converted/<collection>/`
- The database is a directory at `.claude/rag/chroma/` — back it up with git or cp
