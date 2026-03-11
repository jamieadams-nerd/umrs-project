#!/usr/bin/env python3
"""
UMRS RAG Ingestion Pipeline (ChromaDB backend)
================================================
Converts raw reference documents to Markdown, chunks them, embeds them,
and stores them in a ChromaDB vector database for agent search.

Usage:
    python ingest.py                                    # process all new/changed files
    python ingest.py --collection selinux-notebook      # one collection only
    python ingest.py --force                            # reprocess everything
    python ingest.py --summary                          # show database stats and exit
    python ingest.py --drop-collection kernel-docs      # delete collection + manifest entries, then exit

Directory contract:
    .claude/references/<collection>/    <- raw source files (never modified)
    .claude/rag/converted/<collection>/ <- converted .md files
    .claude/rag/manifest.json           <- ingestion state tracking

ChromaDB location:
    Defaults to .claude/rag/chroma/ relative to this script.
    Override with the RAG_CHROMA_PATH environment variable:
        export RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma
"""

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

# ---------------------------------------------------------------------------
# Dependency check
# ---------------------------------------------------------------------------
MISSING = []

try:
    import chromadb
except ImportError:
    MISSING.append("chromadb  →  pip install chromadb --break-system-packages")

try:
    from sentence_transformers import SentenceTransformer
except ImportError:
    MISSING.append("sentence-transformers  →  pip install sentence-transformers --break-system-packages")

try:
    import tiktoken
except ImportError:
    MISSING.append("tiktoken  →  pip install tiktoken --break-system-packages")

try:
    import fitz  # pymupdf
except ImportError:
    MISSING.append("pymupdf  →  pip install pymupdf --break-system-packages")

if MISSING:
    print("ERROR: Missing dependencies:")
    for m in MISSING:
        print(f"  {m}")
    sys.exit(1)

import chromadb
import fitz  # pymupdf
from sentence_transformers import SentenceTransformer
import tiktoken

# ---------------------------------------------------------------------------
# Paths — all relative to this script's location (.claude/rag/)
# ---------------------------------------------------------------------------
SCRIPT_DIR  = Path(__file__).parent.resolve()
RAG_DIR     = SCRIPT_DIR
REFERENCES  = SCRIPT_DIR.parent / "references"
CONVERTED   = RAG_DIR / "converted"
CHROMA_DIR  = Path(os.environ.get("RAG_CHROMA_PATH", str(RAG_DIR / "chroma")))
MANIFEST    = RAG_DIR / "manifest.json"

# ---------------------------------------------------------------------------
# Chunking parameters
# ---------------------------------------------------------------------------
TARGET_TOKENS  = 512
MAX_TOKENS     = 1024
MIN_TOKENS     = 64
OVERLAP_TOKENS = 64

# ---------------------------------------------------------------------------
# Supported source formats → pandoc input format names
# ---------------------------------------------------------------------------
PANDOC_FORMATS = {
    ".rst":  "rst",
    ".html": "html",
    ".htm":  "html",
    ".docx": "docx",
    ".man":  "man",
    ".org":  "org",
}
# Files copied as-is (no pandoc conversion needed)
PASSTHROUGH = {".md", ".markdown", ".txt"}
# Binary or non-text extensions — skip entirely
SKIP_EXTENSIONS = {
    ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".webp",
    ".yaml", ".yml",
    ".dot", ".gv",
    ".odt", ".ods", ".odp",
    ".zip", ".tar", ".gz", ".bz2", ".xz",
    ".so", ".a", ".o", ".ko",
    ".pyc", ".pyo",
    ".csv",
    ".inf",
    ".exceptions",
    ".litmus",
    ".vim",
    ".css", ".js",
}


def sanitize_collection_name(name: str) -> str:
    """ChromaDB collection names: alphanumeric + hyphens, 3-63 chars."""
    sanitized = re.sub(r"[^a-zA-Z0-9-]", "-", name)
    sanitized = re.sub(r"-+", "-", sanitized).strip("-")
    if len(sanitized) < 3:
        sanitized = f"col-{sanitized}"
    return sanitized[:63]


# ---------------------------------------------------------------------------
# Manifest helpers
# ---------------------------------------------------------------------------

def load_manifest() -> dict:
    if MANIFEST.exists():
        with open(MANIFEST) as f:
            return json.load(f)
    return {}


def save_manifest(manifest: dict):
    with open(MANIFEST, "w") as f:
        json.dump(manifest, f, indent=2)


def file_hash(path: Path) -> str:
    h = hashlib.sha256()
    h.update(path.read_bytes())
    return h.hexdigest()[:16]


# ---------------------------------------------------------------------------
# Step 1: Conversion — raw file → .md
# ---------------------------------------------------------------------------

def convert_pdf_to_markdown(src: Path, dst: Path) -> bool:
    """
    Extract text from a PDF using pymupdf and write as Markdown.
    Each page is introduced with an H2 heading (## Page N) so the
    chunker can split on section boundaries.
    Original source file is never modified.
    Returns True on success.
    """
    try:
        doc = fitz.open(str(src))
    except Exception as exc:
        print(f"    WARNING: could not open PDF {src.name}: {exc}")
        return False

    parts = []
    for page_num, page in enumerate(doc, start=1):
        text = page.get_text().strip()
        if text:
            parts.append(f"## Page {page_num}\n\n{text}")

    doc.close()

    if not parts:
        print(f"    WARNING: no text extracted from {src.name} (scanned/image PDF?)")
        return False

    dst.write_text("\n\n".join(parts), encoding="utf-8")
    return True


def convert_to_markdown(src: Path, dst: Path) -> bool:
    """
    Convert src to Markdown and write to dst.
    Original source file is never modified.
    Returns True on success.
    """
    dst.parent.mkdir(parents=True, exist_ok=True)
    suffix = src.suffix.lower()

    if suffix in PASSTHROUGH:
        dst.write_text(src.read_text(encoding="utf-8", errors="replace"))
        return True

    if suffix == ".pdf":
        return convert_pdf_to_markdown(src, dst)

    pandoc_fmt = PANDOC_FORMATS.get(suffix)
    cmd = ["pandoc", "--to", "markdown", "--wrap=none", str(src), "-o", str(dst)]
    if pandoc_fmt:
        cmd = ["pandoc", "--from", pandoc_fmt, "--to", "markdown",
               "--wrap=none", str(src), "-o", str(dst)]

    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"    WARNING: pandoc failed on {src.name}: {result.stderr.strip()}")
        return False
    return True


# ---------------------------------------------------------------------------
# Step 2: Hybrid chunking — .md → list of chunk dicts
# ---------------------------------------------------------------------------

def count_tokens(text: str, enc) -> int:
    return len(enc.encode(text))


def split_markdown_sections(text: str) -> list[dict]:
    """
    Split Markdown on H1/H2/H3 boundaries.
    Code blocks and tables are kept intact within their section.
    """
    sections = []
    current_heading = "Preamble"
    current_lines = []
    in_code_block = False

    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("```") or stripped.startswith("~~~"):
            in_code_block = not in_code_block

        if not in_code_block and re.match(r'^#{1,3}\s+', line):
            if current_lines:
                sections.append({
                    "heading": current_heading,
                    "text": "\n".join(current_lines).strip()
                })
            current_heading = line.lstrip("#").strip()
            current_lines = [line]
        else:
            current_lines.append(line)

    if current_lines:
        sections.append({
            "heading": current_heading,
            "text": "\n".join(current_lines).strip()
        })

    return [s for s in sections if s["text"].strip()]


def chunk_section(section: dict, enc, overlap_text: str = "") -> list[dict]:
    """
    Split one section into token-bounded chunks with overlap.
    """
    chunks = []
    text = section["text"]

    if count_tokens(text, enc) <= MAX_TOKENS:
        body = (overlap_text + "\n\n" + text).strip() if overlap_text else text
        return [{"heading": section["heading"], "text": body}]

    paragraphs = re.split(r'\n{2,}', text)
    current_parts = []
    current_tokens = 0

    if overlap_text:
        current_parts.append(overlap_text)
        current_tokens = count_tokens(overlap_text, enc)

    for para in paragraphs:
        para_tokens = count_tokens(para, enc)
        if current_tokens + para_tokens > TARGET_TOKENS and current_parts:
            chunks.append({
                "heading": section["heading"],
                "text": "\n\n".join(current_parts).strip()
            })
            overlap_words = " ".join(current_parts[-1].split()[-50:])
            current_parts = [overlap_words, para] if overlap_words else [para]
            current_tokens = count_tokens("\n\n".join(current_parts), enc)
        else:
            current_parts.append(para)
            current_tokens += para_tokens

    if current_parts:
        chunks.append({
            "heading": section["heading"],
            "text": "\n\n".join(current_parts).strip()
        })

    return chunks


def chunk_document(md_path: Path, collection: str, source_file: str, enc) -> list[dict]:
    """
    Full chunking pipeline for one Markdown file.
    Returns list of chunk dicts ready for embedding.
    """
    text = md_path.read_text(encoding="utf-8", errors="replace")
    sections = split_markdown_sections(text)

    all_chunks = []
    overlap_text = ""
    chunk_index = 0

    for section in sections:
        if count_tokens(section["text"], enc) < MIN_TOKENS:
            continue

        for chunk in chunk_section(section, enc, overlap_text):
            token_count = count_tokens(chunk["text"], enc)
            if token_count < MIN_TOKENS:
                continue

            all_chunks.append({
                "collection":    collection,
                "source_file":   source_file,
                "section":       chunk["heading"] or "",
                "chunk_index":   chunk_index,
                "text":          chunk["text"],
                "token_count":   token_count,
                "date_ingested": datetime.now(timezone.utc).isoformat()
            })

            words = chunk["text"].split()
            overlap_text = " ".join(words[-OVERLAP_TOKENS:]) if len(words) > OVERLAP_TOKENS else ""
            chunk_index += 1

    return all_chunks


# ---------------------------------------------------------------------------
# Step 3: ChromaDB storage
# ---------------------------------------------------------------------------

def get_chroma_client() -> chromadb.PersistentClient:
    CHROMA_DIR.mkdir(parents=True, exist_ok=True)
    return chromadb.PersistentClient(path=str(CHROMA_DIR))


def get_or_create_collection(client: chromadb.PersistentClient, dir_name: str):
    return client.get_or_create_collection(
        name=sanitize_collection_name(dir_name),
        metadata={"hnsw:space": "cosine", "directory": dir_name}
    )


def delete_chunks_for_file(collection, source_file: str):
    """Remove all chunks for a given source file before re-ingesting."""
    results = collection.get(where={"source_file": source_file})
    if results and results["ids"]:
        collection.delete(ids=results["ids"])
        print(f"    Removed {len(results['ids'])} stale chunks for {source_file}")


def embed_and_store(collection, chunks: list[dict], model: SentenceTransformer):
    """Embed and insert chunks into ChromaDB."""
    if not chunks:
        return

    texts      = [c["text"] for c in chunks]
    embeddings = model.encode(texts, show_progress_bar=False, normalize_embeddings=True)

    collection.add(
        ids        = [f"{c['source_file']}::chunk::{c['chunk_index']}" for c in chunks],
        embeddings = [e.tolist() for e in embeddings],
        documents  = texts,
        metadatas  = [
            {
                "collection":    c["collection"],
                "source_file":   c["source_file"],
                "section":       c["section"],
                "chunk_index":   c["chunk_index"],
                "token_count":   c["token_count"],
                "date_ingested": c["date_ingested"]
            }
            for c in chunks
        ]
    )


# ---------------------------------------------------------------------------
# Main ingestion loop
# ---------------------------------------------------------------------------

def ingest_collection(
    dir_name: str,
    client: chromadb.PersistentClient,
    manifest: dict,
    model: SentenceTransformer,
    enc,
    force: bool = False
):
    src_dir = REFERENCES / dir_name
    dst_dir = CONVERTED / dir_name

    if not src_dir.exists():
        print(f"  WARNING: {src_dir} does not exist — skipping")
        return

    src_files = sorted([
        f for f in src_dir.rglob("*")
        if f.is_file()
        and not f.name.startswith(".")
        and f.suffix.lower() not in SKIP_EXTENSIONS
    ])

    print(f"\n[{dir_name}] {len(src_files)} source file(s) found")
    chroma_col = get_or_create_collection(client, dir_name)

    for src in src_files:
        manifest_key  = str(src.relative_to(REFERENCES))
        current_hash  = file_hash(src)

        if not force and manifest_key in manifest:
            if manifest[manifest_key]["hash"] == current_hash:
                print(f"  SKIP (unchanged): {src.name}")
                continue
            print(f"  UPDATE:           {src.name}")
        else:
            print(f"  NEW:              {src.name}")

        # Step 1 — convert
        md_path = dst_dir / src.relative_to(src_dir).with_suffix(".md")
        if not convert_to_markdown(src, md_path):
            continue

        # Step 2 — chunk
        chunks = chunk_document(md_path, dir_name, manifest_key, enc)
        if not chunks:
            print(f"    WARNING: no chunks produced from {src.name}")
            continue

        # Step 3 — store
        delete_chunks_for_file(chroma_col, manifest_key)
        embed_and_store(chroma_col, chunks, model)

        manifest[manifest_key] = {
            "hash":          current_hash,
            "converted_to":  str(md_path.relative_to(RAG_DIR)),
            "chunks_stored": len(chunks),
            "collection":    dir_name,
            "ingested_date": datetime.now(timezone.utc).isoformat()
        }
        save_manifest(manifest)  # persist after each file — safe for incremental / interrupted runs

        print(f"    → {len(chunks)} chunks stored")


def drop_collection(client: chromadb.PersistentClient, dir_name: str):
    """Delete a ChromaDB collection and remove its manifest entries."""
    col_name = sanitize_collection_name(dir_name)
    existing = [c.name for c in client.list_collections()]
    if col_name in existing:
        client.delete_collection(col_name)
        print(f"  Deleted ChromaDB collection: {col_name}")
    else:
        print(f"  Collection not found in ChromaDB (already absent): {col_name}")

    manifest = load_manifest()
    prefix = dir_name + "/"
    removed = [k for k in manifest if k == dir_name or k.startswith(prefix)]
    for k in removed:
        del manifest[k]
    save_manifest(manifest)
    print(f"  Removed {len(removed)} manifest entry/entries with prefix '{dir_name}'")


def print_summary(client: chromadb.PersistentClient):
    print("\n--- Database Summary ---")
    collections = client.list_collections()
    if not collections:
        print("  (empty)")
    for col in collections:
        print(f"  {col.name}: {col.count()} chunks")
    print(f"\n  Database : {CHROMA_DIR}")
    print(f"  Manifest : {MANIFEST}")


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="UMRS RAG ingestion pipeline")
    parser.add_argument("--collection", help="Process one collection by directory name")
    parser.add_argument("--force", action="store_true", help="Reprocess all files")
    parser.add_argument("--summary", action="store_true", help="Show DB stats and exit")
    parser.add_argument("--drop-collection", metavar="NAME",
                        help="Delete a collection from ChromaDB and remove its manifest entries, then exit")
    args = parser.parse_args()

    print(f"RAG database : {CHROMA_DIR}")
    client = get_chroma_client()

    if args.drop_collection:
        print(f"Dropping collection: {args.drop_collection}")
        drop_collection(client, args.drop_collection)
        print("Done.")
        return

    if not REFERENCES.exists():
        print(f"ERROR: {REFERENCES} not found")
        print("Expected: .claude/references/<collection>/")
        sys.exit(1)

    if args.summary:
        print_summary(client)
        return

    collections = (
        [args.collection] if args.collection
        else [d.name for d in sorted(REFERENCES.iterdir())
              if d.is_dir() and not d.name.startswith(".")]
    )

    if not collections:
        print("No collections found in .claude/references/")
        sys.exit(0)

    print(f"Collections to process: {collections}")
    print("\nLoading embedding model (all-MiniLM-L6-v2)...")
    print("First run downloads ~90MB to ~/.cache/huggingface/ — cached after.")

    model = SentenceTransformer("all-MiniLM-L6-v2")
    enc   = tiktoken.get_encoding("cl100k_base")
    manifest = load_manifest()

    for col in collections:
        ingest_collection(col, client, manifest, model, enc, force=args.force)

    print_summary(client)


if __name__ == "__main__":
    main()
