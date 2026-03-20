# A Comprehensive Survey and Experimental Comparison of Graph-Based Approximate Nearest Neighbor Search

**Authors:** Mengzhao Wang, Xiaoliang Xu, Qiang Yue, Yuxiang Wang
**Year:** 2021
**Source URL:** https://arxiv.org/abs/2101.12631
**Published in:** arXiv:2101.12631 (VLDB 2021 Journal Track)

---

## Overview

This survey provides the most comprehensive systematic comparison of graph-based approximate
nearest neighbor (ANN) search methods as of 2021. It covers the algorithmic landscape from
early NSW methods through HNSW, NSG (Navigating Spreading-out Graphs), DiskANN, NGT-panng,
and hybrid methods combining graph with quantization or hashing.

The survey's value is threefold:
1. **Unified taxonomy:** All graph-based ANN methods are mapped to a common framework
   of index construction and search phases, enabling principled comparison
2. **Empirical benchmark:** Experiments on 9 real-world datasets covering multiple
   embedding types, dimensions (25–960), and scales (1M–1B vectors)
3. **Trade-off analysis:** Systematic characterization of the recall-speed-memory
   Pareto frontier for each algorithm

---

## Unified Framework for Graph-Based ANN

All graph-based ANN methods can be decomposed into:

### Phase 1: Index Construction

**Graph initialization:** How the initial graph is built (random, brute-force k-NN, etc.)
**Neighbor selection:** Which neighbors to connect to each node:
- k-NN graph: connect to k nearest neighbors (accurate but expensive)
- RNG/MRNG (Relative Neighborhood Graph / Monotonic RNG): connect to neighbors
  that form a "well-spread" neighborhood covering different directions
- HNSW heuristic: connect if no existing neighbor is closer to the candidate

**Connectivity refinement:** Iterative passes to fix isolated nodes and improve routing
**Diversification:** Ensuring neighbors span different directions (like HNSW heuristic)

### Phase 2: Index Search

All methods use some form of beam search:
1. Start from entry point (random, fixed, or selected by upper-level graph)
2. Maintain candidate queue Q (size ef) sorted by distance to query
3. Expand nearest unvisited candidate; add its neighbors to Q if close enough
4. Stop when no unvisited candidate is closer than furthest in Q
5. Return top-K from Q

Differences are in: entry point selection, graph structure affecting traversal paths,
early stopping criteria, and whether upper-level navigation is used (HNSW-style).

---

## Algorithm Taxonomy

### Tier 1: Flat Graph Methods

**NSW (Navigable Small World):** Malkov et al. 2014. Single-layer navigable graph.
Monotone search, O(log² N) query. Superseded by HNSW but foundational.

**KGraph:** Wei Dong et al. 2011. Approximate k-NN graph via "neighbor of neighbor"
heuristic. Excellent recall but slow construction.

**NGT (Neighborhood Graph and Tree):** Yahoo Japan Research. Uses ANNG (Approximate
Nearest Neighbor Graph) + ONNG (Optimized NG). Competitive on moderate scales.

### Tier 2: Hierarchical Methods

**HNSW:** Malkov & Yashunin 2016/2020. Hierarchical skip-list-style graph.
Currently the best all-around performer. (See separate HNSW entry.)

**NSG (Navigating Spreading-out Graph):** Fu et al. 2019. Single-layer but with
Monotonic Relative Neighborhood Graph (MRNG) structure: edges must provide monotone
improvement toward query. Key insight: the "monotone search network" property ensures
greedy search always makes progress toward the query point.

NSG achieves better recall at same M than HNSW on many datasets because MRNG
gives better graph structure. Construction: offline (requires a k-NN graph as input)
and slower than HNSW.

### Tier 3: Disk-Based Methods

**DiskANN / Vamana:** Microsoft Research 2019. Graph designed for SSD storage.
Uses a modified NSG with large M (compressed) stored on disk; cache-friendly access
patterns. Enables billion-scale ANN on a single machine with ~64GB RAM.

The Vamana algorithm is the index construction part:
- Build a random graph
- Iteratively prune/add edges to enforce monotone search property
- Result: robust-beam-search graph with ~70–150 connections per node

### Tier 4: Hybrid Methods

**SPTAG:** Microsoft Research. Combines k-d trees for entry point selection with
BKT (Balanced k-means Tree) for initial navigation, then graph search. Used in
Bing search for billion-scale image retrieval.

**ScaNN (Google):** Asymmetric quantization + tree-based partitioning. Not pure
graph-based but competitive on many benchmarks.

---

## Key Empirical Findings

### Recall-Speed Pareto Frontier (from experiments)

On SIFT-1M (128-dim, 1M vectors), QPS at recall@10 = 0.999:
1. **NSG:** ~40,000 QPS (fastest at high recall)
2. **HNSW:** ~25,000 QPS
3. **NGT-panng:** ~20,000 QPS
4. **KGraph:** ~10,000 QPS

At recall@10 = 0.95:
1. **HNSW:** ~200,000 QPS (best at moderate recall — benefits from hierarchical routing)
2. **NSG:** ~150,000 QPS

**Key finding:** HNSW wins at low-to-moderate recall targets; NSG wins at high recall.
For RAG workloads (recall@10 ≈ 0.95–0.99), HNSW is typically the right choice.

### Effect of Dimensionality

All graph methods degrade gracefully as dimension increases (unlike tree-based methods
that become unusable above ~20 dims). At 960 dimensions (GloVe-like):
- Recall drops by 5–15% at fixed ef compared to 128-dim
- Compensated by increasing M or ef

### Effect of Data Distribution

- **Clustered data** (e.g., face embeddings): NSG and HNSW both perform well; clustered
  structure creates natural hierarchies
- **Uniform random** (adversarial for small-world property): NSG outperforms HNSW because
  NSG's MRNG structure is more robust
- **Intrinsic dimensionality:** Even in 1536-dim spaces, if the true data manifold is
  lower-dimensional, ANN methods perform as if dimensionality is the intrinsic dimension

### Memory vs. Recall

HNSW with M=16: ~100 bytes/vector
NSG with M=32: ~60 bytes/vector (no multi-layer overhead)

For collections of 1M+ vectors, the memory difference becomes significant. NSG's
single-layer design saves ~40% memory at equivalent recall.

---

## Algorithm Selection Guide

| Scenario | Recommended Method | Reason |
|---|---|---|
| <1M vectors, best recall | NSG | Superior graph structure |
| <1M vectors, fast iteration | HNSW | Online insertion, good defaults |
| >1M vectors, disk storage | DiskANN | Designed for SSD access |
| Streaming inserts | HNSW | Only method with efficient online insertion |
| Production, recall@10 = 0.95 | HNSW (ChromaDB default) | Best ecosystem support |
| Research/tuned deployment | NSG | Best Pareto frontier at high recall |

---

## Advanced Topics

### Monotone Search Network (MSN) Property

A key theoretical result: a graph with the MSN property (every edge (u,v) has the
property that v is closer to u's nearest neighbor than u is, for some appropriate
metric) guarantees that beam search always makes progress. NSG explicitly enforces
MSN; HNSW approximates it via the neighbor selection heuristic.

MSN is the ANN analog of A*'s consistency condition — it ensures the greedy traversal
doesn't get stuck in local minima.

### Graph Compression

Several methods use product quantization (PQ) to compress edge weights:
- Store compressed representations of neighbor embeddings for candidate generation
- Re-rank using full precision only for top candidates
- Reduces memory ~4–8× at 1–3% recall cost

This is rate-distortion in practice: quantization compresses neighbors at the cost
of some search accuracy.

### Billion-Scale Challenges

At 1B+ vectors:
- HNSW construction: weeks (impractical)
- NSG + sharding: feasible with distributed construction
- DiskANN: single machine, ~2TB SSD, 256GB RAM
- FAISS IVF-PQ: CPU-efficient for offline batch queries

The survey identifies the billion-scale regime as the key open problem in graph-based ANN.

---

## UMRS Relevance

### ChromaDB Configuration

UMRS RAG uses ChromaDB which defaults to HNSW. Survey findings confirm:
- M=16 (ChromaDB default) is appropriate for collections <1M chunks
- ef=50 (ChromaDB default) gives ~0.95 recall; increase to ef=200 for ~0.999 recall
  at 4× latency cost
- For UMRS's current corpus scale (~25,000 chunks across all collections), HNSW
  with defaults is well within the regime where HNSW performs at its best

### Multi-Collection Architecture Validation

UMRS maintains separate ChromaDB collections per RAG topic. The survey confirms this
is the right architectural decision:
- Cross-collection ANN search would require merging indices, losing the security boundary
- Per-collection HNSW indices have faster construction (smaller N per collection)
- At 1000–50,000 chunks per collection, brute force is actually competitive with HNSW
  (the crossover point is ~10,000 vectors at typical ef values)

### Embedding Dimension Trade-offs

UMRS currently uses various embedding models. Survey guidance:
- For text-embedding-3-small (1536 dims): HNSW handles this well
- Dimensionality reduction (e.g., PCA to 256 dims) before indexing can improve
  throughput with <5% recall cost at moderate scales
- If intrinsic dimensionality of UMRS's security-domain text is ~50–100 (plausible),
  the effective ANN difficulty is much lower than the nominal 1536 dims suggest

### Future: Billion-Scale UMRS

If UMRS ever needs to index compliance documents at scale (e.g., all CVEs + all STIG rules):
- NVD contains ~250,000 CVEs
- At 50 chunks/CVE, that's 12.5M chunks — approaching HNSW's practical limit
- DiskANN would be the right choice at that scale
- Survey's comparison provides the selection criteria

---

## Key Terminology

- **Graph-based ANN:** Using k-NN or navigable graphs for approximate similarity search
- **MRNG (Monotonic Relative Neighborhood Graph):** Graph where edges ensure monotone routing
- **MSN (Monotone Search Network):** Property guaranteeing beam search never gets stuck
- **DiskANN / Vamana:** Disk-optimized graph ANN; billion-scale capable
- **ef:** Query beam width parameter; controls recall-speed trade-off at query time
- **Product Quantization (PQ):** Lossy compression of vectors for memory reduction
- **QPS (Queries Per Second):** Standard ANN performance metric
- **Recall@K:** Fraction of true top-K neighbors returned

---

## Cross-References

- **HNSW 2016** — the primary algorithm covered; this survey contextualizes HNSW in the landscape
- **Dijkstra 1959** — beam search in ANN is Dijkstra with bounded candidate set; all graph ANN
  methods are Dijkstra variants
- **A* 1968** — MSN property in NSG mirrors A*'s consistency condition
- **Shannon 1948** — recall-speed Pareto frontier is the operational rate-distortion curve for ANN
- **Spectral clustering** — graph structure quality (navigability, diameter) relates to spectral
  properties; spectral gap bounds ANN graph quality
