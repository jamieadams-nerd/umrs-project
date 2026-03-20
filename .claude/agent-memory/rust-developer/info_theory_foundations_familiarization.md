---
name: Info Theory Foundations Corpus Familiarization
description: Active knowledge of the info-theory-foundations RAG collection (238 chunks, 14 files) — concepts, UMRS connections, design implications
type: project
---

## Collection Overview

14 files, grouped into four phases:
- Phase 1A: Core information theory (Shannon, MacKay, Rissanen MDL, Kolmogorov)
- Phase 1B: Graph theory and search (Dijkstra, A*, Avis-Fukuda reverse search, Edmonds)
- Phase 1C: Modern ANN search (HNSW, ANN survey)
- Phase 1D: Cross-discipline bridges (spectral clustering, analytic combinatorics, synthesis)

---

## Key Concepts by Document

### Shannon 1948 — entropy, channel capacity, mutual information, source coding theorem
- H(X) = -sum p(x) log2 p(x) — irreducible bits of uncertainty
- Mutual information I(X;Y) = H(X) - H(X|Y) — ideal relevance measure for RAG
- Covert channels = residual channel capacity that policy doesn't eliminate
- UMRS: MLS Bell-LaPadula = zero-capacity channel from high→low security levels

### MacKay ITILA — Bayesian inference = dual of information theory
- Belief propagation (BP) on factor/Tanner graphs — efficient marginal inference
- Rate-distortion R(D): minimum bits for representation within distortion D
- Bayesian model selection (Occam's razor) — complex models penalized by evidence
- UMRS: posture catalog = probabilistic model; contradictions = low-posterior states

### Rissanen MDL — minimum description length as model selection criterion
- Best model = shortest (model + data|model) code length
- NML (normalized maximum likelihood) = minimax-optimal
- UMRS: MDL for deciding how many posture signal categories to have; corpus curation criterion

### Kolmogorov Complexity — irreducible description length of individual strings
- K(x) = shortest program producing x; machine-independent up to additive constant
- NID / NCD: normalized compression distance — universal similarity without domain features
- Anomaly detection: C(N+O) - C(N) small → O shares little structure with normal corpus N
- UMRS: NCD for reference library deduplication; covert channels = I(high_data:low_observable) > 0

### Dijkstra 1959 — greedy shortest path, foundational graph algorithm
- Non-negative weights required; settled-set invariant
- UMRS: signal dependency graph traversal; SELinux policy type-transition shortest paths

### A* 1968 — optimal heuristic search with admissibility guarantee
- f(n) = g(n) + h(n); admissible h never overestimates; consistent = triangle inequality
- Higher-quality heuristic → exponentially fewer nodes expanded
- UMRS: compliance remediation path finding (min-change path to compliant state)

### Avis-Fukuda Reverse Search — exhaustive enumeration in O(n) space
- DFS on an implicit parent-function tree over solution space; visits each solution once
- Complement to A*: A* finds one optimum; reverse search enumerates all solutions
- UMRS: enumerate all minimal SELinux rule sets satisfying a security requirement

### Edmonds 1965 — matroids, maximum matching, polynomial-time complexity (class P)
- Greedy algorithms are optimal exactly on matroids (exchange property)
- Blossom algorithm for max matching; matroid intersection is polynomial-time solvable
- UMRS: SELinux policy assignment as matching; compliance constraints may have matroid structure

### HNSW 2016 — hierarchical navigable small world graph for ANN search
- Skip-list hierarchy: upper layers for long-range routing, bottom for local precision
- Parameters: M (connections/node), ef_construction, ef (query beam width)
- Recall-speed trade-off = rate-distortion analog
- UMRS: ChromaDB backend; separate collections per security level (HNSW has no per-doc ACL)

### ANN Survey 2021 — systematic comparison of graph ANN methods
- HNSW wins at recall@10 = 0.95; NSG wins at 0.999
- Below ~10,000 vectors, brute force competes with HNSW
- DiskANN for billion-scale; HNSW for streaming inserts
- UMRS: current per-collection sizes (<50K chunks) — HNSW defaults are appropriate

### Spectral Clustering — graph Laplacian eigenvalues = cluster structure
- L = D - W; eigenvalue multiplicity of 0 = number of connected components
- Cheeger's inequality: h²/2 ≤ λ_2 ≤ 2h (Fiedler value bounds conductance)
- Spectral gap λ_{k+1} - λ_k → principled choice of k (number of clusters)
- UMRS: MLS lattice spectral analysis; posture signal correlation clustering; RAG corpus organization

### Analytic Combinatorics — generating functions for exact/asymptotic algorithm analysis
- Symbolic method: structural description → GF equation → asymptotics mechanically
- Phase transitions in combinatorial structures (SAT, random graphs)
- UMRS: compliance has a "phase transition" — small config changes cause large compliance swings near threshold

### Cross-Discipline Bridges (synthesis) — unified insights
- Three Cs: Compression (embedding) → Clustering (spectral) → Search (HNSW); all interdependent
- MDL as corpus management policy: add document iff it decreases total description length
- Rate-distortion = recall/speed Pareto frontier (operational equivalence)
- Network information theory: MLS lattice min-cut = max info-flow bounds covert channel strength
- Belief propagation on posture signal dependency graph computes compliance posteriors

---

## Design Implications for UMRS

### RAG Pipeline
- ChromaDB HNSW defaults (M=16, ef=50) appropriate at current scale (<50K chunks/collection)
- One ChromaDB collection per security level — HNSW has no per-document access control
- Increase ef to 200 for high-precision queries; ~4× latency cost
- MDL criterion for corpus curation: add a document only if it reduces total query description length
- High-entropy retrieval results → increase K retrieved chunks; flag for synthesis not lookup

### MLS Policy Analysis (Phase 2)
- MLS lattice is a directed graph; Bell-LaPadula no-write-down = zero-capacity channel cut
- Cheeger constant of MLS information flow graph bounds covert channel leakage per unit time
- A* can find minimum-length type transition paths in SELinux policy graph
- Reverse search can enumerate all minimal compliant policy rule sets

### Posture Catalog
- Signal dependency graph should be sparse (Tanner graph); enables BP-style inference
- MDL criterion for catalog categories: minimize L(model) + L(data|model)
- Compliance has a phase-transition structure near threshold — report the "compliance cliff"
- Contradiction detection → low-posterior states under Bayesian system-state model

### Reference Library Management
- NCD (zstd-based) for near-duplicate detection without domain features
- Reverse search on (document × control) bipartite graph to detect uncovered controls
- Spectral clustering over document similarity for ChromaDB collection structuring

### Canonical Security Contexts
- MDL/Kolmogorov: canonical form = shortest achievable description of the equivalence class
- Two contexts are informationally equivalent if replacing one doesn't increase description length
- AIT bounds minimum viable SELinux policy: K(policy) closest to K(access_matrix)

---

## Cross-References Within Corpus

Shannon → MacKay (modernization) → Rissanen MDL (computable approximation) → Kolmogorov (idealized limit)
Dijkstra → A* (heuristic extension) → HNSW (approximate, learned hierarchy)
Edmonds (matroids) ↔ Dijkstra (greedy optimality proof is a matroid proof)
Spectral → HNSW (navigability requires high Fiedler value)
AC → all (quantitative analysis tool for every other algorithm)
