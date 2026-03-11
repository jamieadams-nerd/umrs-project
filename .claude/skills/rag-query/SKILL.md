---
name: rag-query
description: >
  Searches the UMRS RAG reference library using semantic search. Trigger this
  skill whenever any agent — rust-developer, security-engineer, tech-writer,
  security-auditor, or others — is working on a topic related to SELinux, MLS
  policy, Linux kernel internals, filesystem standards, IMA, dm-crypt, Linux
  capabilities, extended attributes, CUI handling, or any topic likely covered
  by the reference library. Also trigger when the user asks to "search the
  docs", "check the reference library", "look up X in the RAG", "what do our
  references say about X", or when answering a technical question would benefit
  from grounding in authoritative source material. When in doubt, search —
  it is fast and the results improve answer quality significantly.
---

# RAG Query Skill

Semantic search over the UMRS reference library via `query.py`.

## Path

```
.claude/rag/query.py
```

## Basic Usage

```bash
# Search all collections — best for cross-cutting topics
python .claude/rag/query.py "your query here"

# Search a specific collection
python .claude/rag/query.py "your query" --collection selinux-notebook

# Search multiple specific collections
python .claude/rag/query.py "your query" --collection kernel-docs linux-fs-standards

# Return more results (default is 5)
python .claude/rag/query.py "your query" --top-k 10

# JSON output — use when you need to process results programmatically
python .claude/rag/query.py "your query" --json

# See available collections and chunk counts
python .claude/rag/query.py --list-collections
```

## Query Writing Guidelines

Search is semantic (meaning-based), not keyword-based. Write queries as
natural phrases describing the concept you need, not as keyword strings.

| Instead of | Use |
|---|---|
| `CAP_MAC_ADMIN` | `capability required to change selinux security context` |
| `mls write` | `mls policy write equality constraint enforcement` |
| `xattr inode` | `where are extended attributes stored relative to the inode` |
| `dm-crypt` | `full disk encryption key management dm-crypt` |

## Collection Reference

| Collection name | Contents |
|---|---|
| `kernel-docs` | Linux kernel internals, LSM hooks, namespaces, capabilities |
| `linux-fs-standards` | Filesystem standards, inode structure, xattr storage, VFS |
| `selinux-notebook` | SELinux policy, MLS constraints, type enforcement, contexts |

Add new collections as the researcher agent ingests them.

## Interpreting Results

Each result includes:
- **Score** — similarity 0.0–1.0. Above 0.85 is highly relevant. Below 0.6
  may be tangential.
- **Collection** — which reference directory the chunk came from
- **Source** — the original file path relative to `.claude/references/`
- **Section** — the Markdown heading the chunk falls under
- **Text preview** — first 300 chars of the chunk

## Workflow for Technical Agents

When answering a question that touches the reference domains:

1. Run a query against the relevant collection(s)
2. Review the top results — check score and section heading first
3. If top scores are below 0.7, try a rephrased query
4. Cite sources in your answer: collection + source file + section
5. If no relevant results found, say so — do not hallucinate citations

## Example Queries by Agent Role

**rust-developer**
```bash
python .claude/rag/query.py "selinux label type for CUI files" --collection selinux-notebook
python .claude/rag/query.py "inode security xattr storage ext4"
```

**security-engineer**
```bash
python .claude/rag/query.py "mls range write constraint policy rule"
python .claude/rag/query.py "linux capability MAC override selinux" --collection kernel-docs
python .claude/rag/query.py "extended attribute security namespace kernel"
```

**security-auditor**
```bash
python .claude/rag/query.py "SELinux policy type enforcement allow rule" --collection selinux-notebook
python .claude/rag/query.py "integrity measurement architecture IMA xattr"
```

**tech-writer**
```bash
python .claude/rag/query.py "selinux context transition newrole runcon"
python .claude/rag/query.py "filesystem extended attributes user namespace"
```

## Error Handling

| Error | Cause | Fix |
|---|---|---|
| `Database not found` | chroma/ dir missing | Ask researcher agent to run ingest.py |
| `Collection not found` | Wrong name or not yet ingested | Run `--list-collections` to see valid names |
| `No results found` | Query too narrow or collection empty | Broaden query or search all collections |
