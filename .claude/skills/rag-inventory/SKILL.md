---
name: rag-inventory
description: >
  Display complete RAG database inventory with collection details, document
  versions, chunk counts, and agent assignments. Use when the user asks to
  "show the RAG", "list collections", "what's in the reference library",
  "RAG inventory", "RAG status", or wants to see what documents are ingested
  and which agents have familiarized with them.
---

# RAG Inventory Skill

Shows the full contents of the UMRS RAG reference library.

## Usage

Run the inventory script:

```bash
cd /media/psf/repos/umrs-project && bash .claude/skills/rag-inventory/rag-inventory.sh
```

## Output

A formatted markdown table grouped by domain, showing:

- Collection name and chunk count
- Documents in each collection (with version/date if available from SOURCE.md)
- Agent assignment (which agent has familiarized with the collection)
- Ingestion status

## Notes

- Reads ChromaDB at `RAG_CHROMA_PATH` (default: `/media/psf/repos/ai-rag-vdb/chroma`)
- Reads SOURCE.md files from `.claude/references/<collection>/`
- Reads manifest.json from `.claude/rag/manifest.json`
