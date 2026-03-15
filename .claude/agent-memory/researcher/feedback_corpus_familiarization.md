---
name: Post-ingestion familiarization requirement
description: After RAG ingestion, the target agent must run corpus-familiarization before using the material
type: feedback
---

After any RAG collection is ingested, the agent the corpus was built for must run the `corpus-familiarization` skill on the new collection before doing any work with it.

**Why:** If an agent doesn't know what's in the RAG at a high level, they won't know the right questions to ask. Passive retrieval is not enough — agents need active knowledge of the material.

**How to apply:** After every `rag-ingest`, notify the target agent and have them run `corpus-familiarization` as their first task. This applies to all agents, not just the researcher.
