# Performance Engineering Corpus Plan

**Created:** 2026-03-15
**Status:** ALL PHASES COMPLETE (2026-03-16). RAG ingestion done (549 chunks). Corpus familiarization by rust-developer completed 2026-03-16 (4 knowledge artifacts, 5 key findings). Ready to archive.
**Source:** `.claude/jamies_brain/perf-research.txt`
**ROADMAP Goals:** G5 (Security Tools — performance-aware tooling), G8 (High-Assurance Patterns)
**Agent:** researcher
**Depends on:** Nothing — can start when prioritized

---

## Purpose

Build a performance engineering knowledge corpus covering three layers:
1. **Algorithmic performance** — data structures, complexity, search/graph optimization
2. **Systems performance** — cache, memory layout, CPU behavior, profiling
3. **Rust-specific optimization** — allocation, iterators, SIMD, benchmarking

The corpus feeds the rust-developer agent's ability to write performant code and the
tech-writer's ability to document performance characteristics.

---

## Phase 1 — Rust-Specific Performance (Highest Priority)

| # | Source | URL | Format | Notes |
|---|--------|-----|--------|-------|
| 1.1 | The Rust Performance Book | `nnethercote.github.io/perf-book/` | Web | Best Rust perf guide; profiling, allocation, SIMD, iterators |
| 1.2 | Rust Compiler Performance Guide | `rustc-dev-guide.rust-lang.org/profiling.html` | Web | Flamegraphs, perf tools, profiling methodology |

**Save to:** `.claude/references/performance-corpus/rust-performance/`

---

## Phase 2 — Systems Performance

| # | Source | URL | Format | Notes |
|---|--------|-----|--------|-------|
| 2.1 | What Every Programmer Should Know About Memory (Drepper) | `people.freebsd.org/~lstewart/articles/cpumemory.pdf` | PDF | CPU caches, NUMA, latency, prefetching — legendary |
| 2.2 | Brendan Gregg Linux Performance | `brendangregg.com/linuxperf.html` | Web | CPU profiling, flame graphs, kernel perf analysis |
| 2.3 | Agner Fog Optimization Manuals | `agner.org/optimize/` | PDF set | Instruction latency, microarchitecture, branch prediction, vectorization |

**Save to:** `.claude/references/performance-corpus/systems-performance/`

---

## Phase 3 — Benchmarking & Profiling Tools

| # | Source | URL | Format | Notes |
|---|--------|-----|--------|-------|
| 3.1 | Criterion.rs | `github.com/bheisler/criterion.rs` | GitHub README + docs | Statistical benchmarking for Rust |
| 3.2 | flamegraph-rs | `github.com/flamegraph-rs/flamegraph` | GitHub README | CPU hotspot detection |

**Save to:** `.claude/references/performance-corpus/profiling/`

---

## Phase 4 — RAG Ingestion & Knowledge Extraction

1. Ingest all materials into RAG collection `performance-corpus`
2. rust-developer agent runs corpus-familiarization
3. Extract structured knowledge index: techniques, anti-patterns, tool recommendations

---

## Deferred (Future Phases)

These were listed in the source material but are lower priority:

- MIT Open Algorithms Course materials
- Competitive programming algorithm repos (KACTL)
- High-performance Rust codebases as reference (ripgrep, tokio, rayon, tantivy)
- Abseil performance tips (C++ but applicable concepts)
- Hands-On Concurrency with Rust (lock-free, atomics)

---

## Post-Phase Hygiene

- Log each phase to `.claude/logs/task-log.md`
- Update this plan with phase status
- Notify Jamie when corpus is ready for familiarization
