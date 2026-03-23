#!/usr/bin/bash
# RAG Inventory — displays all collections, documents, chunk counts, and agent assignments.

PATH=/usr/bin:/usr/sbin:/bin
export PATH
readonly PATH

unset LD_PRELOAD LD_LIBRARY_PATH BASH_ENV ENV GLOBIGNORE
unset SHELLOPTS 2>/dev/null || true
LC_ALL=C
export LC_ALL
IFS=$'\n\t'

set -euo pipefail

readonly PROJECT_ROOT="/media/psf/repos/umrs-project"
readonly RAG_DIR="${PROJECT_ROOT}/.claude/rag"
readonly REF_DIR="${PROJECT_ROOT}/.claude/references"
readonly CHROMA_PATH="${RAG_CHROMA_PATH:-/media/psf/repos/ai-rag-vdb/chroma}"

python3 - "${CHROMA_PATH}" "${REF_DIR}" "${RAG_DIR}" "${PROJECT_ROOT}" <<'PYEOF'
import sys
import json
import os
from pathlib import Path

chroma_path = sys.argv[1]
ref_dir = Path(sys.argv[2])
rag_dir = Path(sys.argv[3])
project_root = Path(sys.argv[4])

# --- Agent assignments (manually maintained) ---
AGENT_MAP = {
    "accreditation-artifacts": "Herb (security-auditor)",
    "access-control": "Herb (security-auditor)",
    "cmmc": "Herb (security-auditor)",
    "dod-5200": "Herb (security-auditor)",
    "nist": "Herb (security-auditor), Librarian",
    "rmf-methodology": "Herb (security-auditor)",
    "scap-security-guide": "Herb (security-auditor)",
    "selinux-notebook": "Rusty, Knox",
    "kernel-docs": "Rusty, Knox",
    "rust-security": "Rusty",
    "cpu-extensions": "Rusty",
    "tui-cli": "Rusty",
    "info-theory-foundations": "Rusty",
    "performance-corpus": "Rusty",
    "hci-courses": "Rusty",
    "hci-courses-mit-6831": "Rusty",
    "hci-courses-theory-papers": "Rusty",
    "technical-communication": "Rusty",
    "technical-communication-theory": "Rusty",
    "doc-structure": "Elena, Lucia",
    "rustdoc-book": "Elena, Lucia",
    "asciidoctor-ref": "Elena, Lucia",
    "dita-spec": "Elena, Lucia",
    "tech-writer-corpus": "Elena, Lucia",
    "knowledge-organization": "Elena",
    "knowledge-organization-ieko": "Elena",
    "knowledge-organization-texts": "Elena",
    "information-architecture": "Elena",
    "information-architecture-theory": "Elena",
    "information-architecture-standards": "Elena",
    "nist-pqc": "Librarian",
    "linux-fhs-2-3": "Knox",
    "sage-outreach-corpus": "Sage",
}

# --- Domain groupings ---
DOMAIN_MAP = {
    "Security & Compliance": [
        "nist", "cmmc", "dod-5200", "rmf-methodology", "accreditation-artifacts",
        "scap-security-guide", "access-control",
    ],
    "SELinux & Kernel": [
        "selinux-notebook", "kernel-docs", "linux-fhs-2-3",
    ],
    "Rust & Development": [
        "rust-security", "rustdoc-book", "cpu-extensions", "performance-corpus",
        "tui-cli", "info-theory-foundations",
    ],
    "HCI & Theory": [
        "hci-courses", "hci-courses-mit-6831", "hci-courses-theory-papers",
        "technical-communication", "technical-communication-theory",
    ],
    "Documentation & IA": [
        "doc-structure", "asciidoctor-ref", "dita-spec", "tech-writer-corpus",
        "knowledge-organization", "knowledge-organization-ieko",
        "knowledge-organization-texts", "information-architecture",
        "information-architecture-theory", "information-architecture-standards",
    ],
    "Cryptography": [
        "nist-pqc",
    ],
    "Outreach": [
        "sage-outreach-corpus",
    ],
}

# --- Load ChromaDB ---
try:
    import chromadb
    client = chromadb.PersistentClient(path=chroma_path)
    collections = {}
    for c in client.list_collections():
        name = c.name if hasattr(c, 'name') else str(c)
        try:
            col = client.get_collection(name)
            collections[name] = col.count()
        except Exception:
            collections[name] = "?"
except Exception as e:
    print(f"WARNING: Could not read ChromaDB: {e}", file=sys.stderr)
    collections = {}

# --- Load manifest for dates ---
manifest_path = rag_dir / "manifest.json"
manifest = {}
if manifest_path.exists():
    try:
        with open(manifest_path) as f:
            manifest = json.load(f)
    except Exception:
        pass

def get_last_ingest_date(collection_name):
    """Find most recent ingest date from manifest entries for a collection."""
    latest = None
    for key, val in manifest.items():
        if collection_name in key and isinstance(val, dict):
            d = val.get("ingested") or val.get("date") or ""
            if d and (latest is None or d > latest):
                latest = d
    return latest or "—"

def read_source_md(collection_name):
    """Read SOURCE.md for document details."""
    # Try direct path and slash-variant
    candidates = [
        ref_dir / collection_name / "SOURCE.md",
        ref_dir / collection_name.replace("-", "/", 1) / "SOURCE.md",
    ]
    for p in candidates:
        if p.exists():
            try:
                text = p.read_text(errors="replace")
                # Extract document list (lines starting with - or *)
                docs = []
                for line in text.split("\n"):
                    stripped = line.strip()
                    if stripped.startswith(("- ", "* ", "| ")):
                        docs.append(stripped)
                return docs[:15]  # Cap at 15 lines
            except Exception:
                pass
    return []

# --- Collect all known collection names ---
all_names = set(collections.keys())
for domain_list in DOMAIN_MAP.values():
    all_names.update(domain_list)

# --- Output ---
print("# RAG Reference Library Inventory")
print(f"\nDatabase: `{chroma_path}`")
print(f"Total collections: **{len(collections)}**")
total_chunks = sum(v for v in collections.values() if isinstance(v, int))
print(f"Total chunks: **{total_chunks:,}**")
print()

for domain, domain_collections in DOMAIN_MAP.items():
    print(f"## {domain}")
    print()
    print("| Collection | Chunks | Agent(s) | Last Ingested |")
    print("|---|---|---|---|")
    for name in domain_collections:
        chunks = collections.get(name, "not ingested")
        agent = AGENT_MAP.get(name, "—")
        date = get_last_ingest_date(name)
        print(f"| `{name}` | {chunks} | {agent} | {date} |")
    print()

    # Show documents per collection
    for name in domain_collections:
        source_docs = read_source_md(name)
        if source_docs:
            print(f"**`{name}` documents:**")
            for doc in source_docs:
                print(f"  {doc}")
            print()

# Collections in DB but not in domain map
unmapped = set(collections.keys()) - set(
    name for dl in DOMAIN_MAP.values() for name in dl
)
if unmapped:
    print("## Unmapped Collections")
    print()
    print("| Collection | Chunks |")
    print("|---|---|")
    for name in sorted(unmapped):
        print(f"| `{name}` | {collections[name]} |")
    print()
PYEOF
