#!/usr/bin/env python3
"""
UMRS RAG Query Interface
=========================
Semantic search over the UMRS reference library for agent use.

Usage:
    # Search all collections
    python query.py "CAP_MAC_ADMIN selinux context transition"

    # Search a specific collection
    python query.py "mls write equality constraint" --collection selinux-notebook

    # Search multiple specific collections
    python query.py "inode extended attributes" --collection kernel-docs linux-fs-standards

    # Return more results
    python query.py "dm-crypt key management" --top-k 10

    # Output as JSON (for agent consumption)
    python query.py "integrity measurement" --json

    # List available collections
    python query.py --list-collections
"""

import argparse
import json
import os
import sys
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

if MISSING:
    print("ERROR: Missing dependencies:")
    for m in MISSING:
        print(f"  {m}")
    sys.exit(1)

import chromadb
from sentence_transformers import SentenceTransformer

# ---------------------------------------------------------------------------
# Paths
# ---------------------------------------------------------------------------
SCRIPT_DIR = Path(__file__).parent.resolve()
RAG_DIR    = SCRIPT_DIR
CHROMA_DIR = Path(os.environ.get("RAG_CHROMA_PATH", str(RAG_DIR / "chroma")))

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def get_client() -> chromadb.PersistentClient:
    if not CHROMA_DIR.exists():
        print(f"ERROR: Database not found at {CHROMA_DIR}")
        print("Run ingest.py first to build the database.")
        print("Override location with: export RAG_CHROMA_PATH=<path>")
        sys.exit(1)
    print(f"RAG database : {CHROMA_DIR}")
    return chromadb.PersistentClient(path=str(CHROMA_DIR))


def list_collection_names(client: chromadb.PersistentClient) -> list[str]:
    return [col.name for col in client.list_collections()]


def sanitize_collection_name(name: str) -> str:
    """Must match the sanitization logic in ingest.py."""
    import re
    sanitized = re.sub(r"[^a-zA-Z0-9-]", "-", name)
    sanitized = re.sub(r"-+", "-", sanitized).strip("-")
    if len(sanitized) < 3:
        sanitized = f"col-{sanitized}"
    return sanitized[:63]


def query_collection(
    collection,
    embedding: list[float],
    top_k: int
) -> list[dict]:
    """Query a single ChromaDB collection and return ranked results."""
    results = collection.query(
        query_embeddings=[embedding],
        n_results=min(top_k, collection.count()),
        include=["documents", "metadatas", "distances"]
    )

    hits = []
    if not results or not results["ids"] or not results["ids"][0]:
        return hits

    for i, doc_id in enumerate(results["ids"][0]):
        distance = results["distances"][0][i]
        # ChromaDB cosine distance: 0 = identical, 2 = opposite
        # Convert to similarity score 0-1
        similarity = 1.0 - (distance / 2.0)

        metadata = results["metadatas"][0][i] if results["metadatas"] else {}
        document  = results["documents"][0][i] if results["documents"] else ""

        hits.append({
            "id":          doc_id,
            "similarity":  round(similarity, 4),
            "collection":  metadata.get("collection", "unknown"),
            "source_file": metadata.get("source_file", "unknown"),
            "section":     metadata.get("section", ""),
            "chunk_index": metadata.get("chunk_index", 0),
            "token_count": metadata.get("token_count", 0),
            "text":        document
        })

    return hits


def search(
    query_text: str,
    client: chromadb.PersistentClient,
    model: SentenceTransformer,
    collections: list[str] | None = None,
    top_k: int = 5
) -> list[dict]:
    """
    Search one, several, or all collections.
    Results are merged and re-ranked by similarity across all searched collections.
    """
    available = list_collection_names(client)

    if not available:
        print("ERROR: No collections found. Run ingest.py first.")
        sys.exit(1)

    # Determine which collections to search
    if collections:
        # Sanitize user-provided names to match stored names
        targets = [sanitize_collection_name(c) for c in collections]
        # Validate
        missing = [t for t in targets if t not in available]
        if missing:
            print(f"ERROR: Collection(s) not found: {missing}")
            print(f"Available: {available}")
            sys.exit(1)
    else:
        targets = available

    # Embed the query once
    embedding = model.encode(query_text, normalize_embeddings=True).tolist()

    # Query each target collection
    all_hits = []
    for col_name in targets:
        col  = client.get_collection(col_name)
        hits = query_collection(col, embedding, top_k)
        all_hits.extend(hits)

    # Merge and re-rank by similarity descending, return top_k overall
    all_hits.sort(key=lambda h: h["similarity"], reverse=True)
    return all_hits[:top_k]


# ---------------------------------------------------------------------------
# Output formatters
# ---------------------------------------------------------------------------

def print_results(results: list[dict], query: str):
    """Human-readable output for terminal use."""
    print(f'\nQuery: "{query}"')
    print(f"Results: {len(results)}\n")
    print("─" * 72)

    for i, hit in enumerate(results, 1):
        print(f"[{i}] Score: {hit['similarity']:.4f}")
        print(f"    Collection : {hit['collection']}")
        print(f"    Source     : {hit['source_file']}")
        if hit["section"]:
            print(f"    Section    : {hit['section']}")
        print(f"    Chunk      : {hit['chunk_index']}  ({hit['token_count']} tokens)")
        print()

        # Show a preview of the text — first 300 chars
        preview = hit["text"][:300].replace("\n", " ").strip()
        if len(hit["text"]) > 300:
            preview += "..."
        print(f"    {preview}")
        print()
        print("─" * 72)


def print_results_json(results: list[dict], query: str):
    """JSON output for agent consumption."""
    output = {
        "query":   query,
        "count":   len(results),
        "results": results
    }
    print(json.dumps(output, indent=2))


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="UMRS RAG semantic search",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument(
        "query",
        nargs="?",
        help="Search query text"
    )
    parser.add_argument(
        "--collection", "-c",
        nargs="+",
        metavar="NAME",
        help="Collection(s) to search (default: all). "
             "Use directory names e.g. selinux-notebook kernel-docs"
    )
    parser.add_argument(
        "--top-k", "-k",
        type=int,
        default=5,
        metavar="N",
        help="Number of results to return (default: 5)"
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output results as JSON (for agent consumption)"
    )
    parser.add_argument(
        "--list-collections",
        action="store_true",
        help="List available collections and exit"
    )
    args = parser.parse_args()

    client = get_client()

    # --list-collections
    if args.list_collections:
        names = list_collection_names(client)
        if not names:
            print("No collections found. Run ingest.py first.")
        else:
            print("Available collections:")
            for name in names:
                col   = client.get_collection(name)
                count = col.count()
                print(f"  {name}  ({count} chunks)")
        return

    # Require query for search
    if not args.query:
        parser.print_help()
        sys.exit(1)

    # Load model — cached after first run
    model = SentenceTransformer("all-MiniLM-L6-v2")

    results = search(
        query_text  = args.query,
        client      = client,
        model       = model,
        collections = args.collection,
        top_k       = args.top_k
    )

    if not results:
        print("No results found.")
        sys.exit(0)

    if args.json:
        print_results_json(results, args.query)
    else:
        print_results(results, args.query)


if __name__ == "__main__":
    main()

