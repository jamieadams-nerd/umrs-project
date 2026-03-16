# Performance Corpus Knowledge Artifacts

Generated from corpus familiarization pass on 2026-03-16.
Source: `.claude/references/performance-corpus/` (549 RAG chunks)

## Files

- **concept-index.md** — Categorized performance techniques: allocations, I/O, iterators, types, hashing, build config, profiling, systems
- **cross-reference-map.md** — Maps UMRS crates to applicable techniques and techniques to source documents
- **style-decision-record.md** — Decisions made based on corpus findings (SipHash default, profiling-first, forbid(unsafe) constraints, BufReader mandatory, PGO timing)
- **term-glossary.md** — Key performance terms with definitions and UMRS context

## Key Findings

1. SipHash is correct for UMRS — never substitute FxHashMap for security-relevant maps
2. Heap allocation patterns (with_capacity, Cow, buffer reuse) directly applicable to umrs-ls
3. Bounds-check elimination via iterators aligns with forbid(unsafe_code)
4. PGO viable for tool binaries as medium-term optimization
5. Corpus gap: no getdents64/readdir coverage — use kernel-docs collection for that
