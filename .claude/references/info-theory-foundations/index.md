# Information Theory, Graph Theory & Search Foundations — Corpus Index

**Collection:** `info-theory-foundations`
**Location:** `.claude/reference/info-theory-foundations/`
**Created:** 2026-03-20
**Purpose:** Canonical papers and texts underlying RAG indexing, canonical representations,
vector search, and MLS lattice theory in UMRS.

---

## Document Catalog

### Phase 1A — Core Information Theory

| File | Title | Author(s) | Year | One-Line Description |
|---|---|---|---|---|
| `shannon-1948.md` | A Mathematical Theory of Communication | Claude E. Shannon | 1948 | Founding paper defining entropy, channel capacity, and the limits of lossless compression and reliable communication |
| `mackay-itila.md` | Information Theory, Inference, and Learning Algorithms | David J.C. MacKay | 2003 | Comprehensive modern synthesis of information theory, Bayesian inference, and machine learning including LDPC codes and belief propagation |
| `rissanen-mdl.md` | Minimum Description Length Principle | Jorma Rissanen et al. | 1978–2007 | MDL operationalizes Occam's razor as an information-theoretic model selection criterion; the computable approximation to Kolmogorov complexity |
| `kolmogorov-complexity.md` | Kolmogorov Complexity and Algorithmic Information Theory | Kolmogorov, Solomonoff, Chaitin | 1964–1966 | Algorithmic information theory defining the irreducible description length of individual strings; foundation for NCD and the incompressibility method |

### Phase 1B — Graph Theory & Search Foundations

| File | Title | Author(s) | Year | One-Line Description |
|---|---|---|---|---|
| `dijkstra-1959.md` | A Note on Two Problems in Connexion with Graphs | Edsger W. Dijkstra | 1959 | Classical shortest path algorithm; the greedy template for all polynomial-time graph algorithms and the foundation of heuristic search |
| `astar-1968.md` | A Formal Basis for the Heuristic Determination of Minimum Cost Paths | Hart, Nilsson, Raphael | 1968 | A* algorithm extending Dijkstra with an admissible heuristic; proven optimal when h(n) ≤ h*(n); bridges estimation theory and graph search |
| `avis-fukuda-reverse-search.md` | Reverse Search for Combinatorial Optimization | David Avis, Komei Fukuda | 1996 | Bounded-memory enumeration of all combinatorial solutions via DFS on an implicit parent-function tree; complement to forward greedy search |
| `edmonds-1965.md` | Paths, Trees, and Flowers | Jack Edmonds | 1965 | Blossom algorithm for maximum matching; introduced matroids and the concept of polynomial-time ("good") algorithms; founding paper of polyhedral combinatorics |

### Phase 1C — Modern Graph-Based ANN Search

| File | Title | Author(s) | Year | One-Line Description |
|---|---|---|---|---|
| `hnsw-2016.md` | Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs | Malkov, Yashunin | 2016 | HNSW algorithm underlying ChromaDB and most vector databases; hierarchical skip-list graph achieving sub-linear ANN query time with high recall |
| `ann-survey-2021.md` | A Comprehensive Survey and Experimental Comparison of Graph-Based Approximate Nearest Neighbor Search | Wang et al. | 2021 | Systematic taxonomy and benchmark of ANN methods (HNSW, NSG, DiskANN, NGT); algorithm selection guide by dataset scale and recall target |

### Phase 1D — Cross-Discipline Bridges & Analysis

| File | Title | Author(s) | Year | One-Line Description |
|---|---|---|---|---|
| `spectral-clustering-luxburg.md` | A Tutorial on Spectral Clustering | Ulrike von Luxburg | 2007 | Graph Laplacian eigenvectors for data clustering; Cheeger inequality connecting spectral gap to information flow bottlenecks; convergence theory |
| `analytic-combinatorics-flajolet-sedgewick.md` | Analytic Combinatorics | Flajolet, Sedgewick | 2009 | Generating functions and complex analysis for exact/asymptotic algorithm analysis; AC provides the quantitative framework for all other corpus algorithms |

### Synthesis Documents

| File | Description |
|---|---|
| `cross-discipline-bridges.md` | Synthesis of intersections between the five theoretical areas; maps combined insights to UMRS architecture decisions |
| `index.md` | This file |

---

## Concept Index

### By UMRS Component

**RAG Pipeline (ChromaDB / HNSW):**
- HNSW algorithm: `hnsw-2016.md`
- ANN algorithm selection: `ann-survey-2021.md`
- Retrieval entropy / rate-distortion: `shannon-1948.md`, `mackay-itila.md`
- Corpus organization via clustering: `spectral-clustering-luxburg.md`

**Posture Catalog / Signal Assessment:**
- Greedy evaluation ordering (matroids): `edmonds-1965.md`
- A*-guided remediation paths: `astar-1968.md`
- Contradiction detection as Bayesian inference: `mackay-itila.md`
- Complete solution enumeration: `avis-fukuda-reverse-search.md`
- Phase transitions in compliance: `analytic-combinatorics-flajolet-sedgewick.md`

**MLS / SELinux Policy Analysis:**
- Information flow and covert channels: `shannon-1948.md`
- Network capacity and min-cut: `cross-discipline-bridges.md`
- Policy graph traversal: `dijkstra-1959.md`
- Matroid structure of constraints: `edmonds-1965.md`
- Spectral analysis of MLS lattice: `spectral-clustering-luxburg.md`

**Canonical Representations / Deduplication:**
- Shannon entropy as encoding limit: `shannon-1948.md`
- MDL for representation selection: `rissanen-mdl.md`
- NCD for document similarity: `kolmogorov-complexity.md`
- Minimum policy (AIT bound): `kolmogorov-complexity.md`

**Reference Library Management (Corpus 3 skills):**
- MDL for corpus size optimization: `rissanen-mdl.md`
- NCD for duplicate detection: `kolmogorov-complexity.md`
- Gap detection (reverse search): `avis-fukuda-reverse-search.md`
- AC for catalog completeness counting: `analytic-combinatorics-flajolet-sedgewick.md`

### By Theoretical Concept

**Entropy:** `shannon-1948.md` (definition), `mackay-itila.md` (Bayesian connection)
**Compression:** `shannon-1948.md` (source coding), `mackay-itila.md` (practical algorithms),
  `rissanen-mdl.md` (model selection), `kolmogorov-complexity.md` (AIT limit)
**Channel Capacity:** `shannon-1948.md` (definition), `cross-discipline-bridges.md` (network flow)
**Mutual Information:** `shannon-1948.md` (definition), `mackay-itila.md` (belief propagation)
**Graph Laplacian:** `spectral-clustering-luxburg.md` (tutorial), `cross-discipline-bridges.md`
**Greedy Algorithms:** `dijkstra-1959.md` (shortest path), `edmonds-1965.md` (matroids)
**Heuristic Search:** `astar-1968.md` (A*), `hnsw-2016.md` (approximate, learned heuristic)
**Generating Functions:** `analytic-combinatorics-flajolet-sedgewick.md`
**Nearest Neighbor Search:** `hnsw-2016.md` (HNSW), `ann-survey-2021.md` (survey)

---

## Source URLs

| Document | Source URL |
|---|---|
| Shannon 1948 | https://people.math.harvard.edu/~ctm/home/text/others/shannon/entropy/entropy.pdf |
| MacKay ITILA | https://www.inference.org.uk/itprnn/book.pdf |
| MDL/Rissanen | https://homepages.cwi.nl/~pdg/ftp/mdlintro.pdf |
| Kolmogorov complexity | https://homepages.cwi.nl/~paulv/papers/handbooklogic07.pdf |
| Dijkstra 1959 | https://ir.cwi.nl/pub/9256/9256D.pdf |
| A* 1968 | https://www.cs.auckland.ac.nz/courses/compsci709s2c/resources/Mike.d/astarNilsson.pdf |
| Avis & Fukuda | http://cgm.cs.mcgill.ca/~avis/doc/avis/AF96a.pdf |
| Edmonds 1965 | Not freely available (Cambridge University Press); described from literature |
| HNSW 2016 | https://arxiv.org/abs/1603.09320 |
| ANN Survey 2021 | https://arxiv.org/abs/2101.12631 |
| Spectral clustering | https://arxiv.org/pdf/0711.0189 |
| Analytic Combinatorics | https://algo.inria.fr/flajolet/Publications/book.pdf |

---

## RAG Ingestion Notes

**Collection name:** `info-theory-foundations`
**Ingest command:**
```bash
cd /media/psf/repos/umrs-project/.claude/rag
RAG_CHROMA_PATH=/media/psf/repos/ai-rag-vdb/chroma python ingest.py \
  --collection info-theory-foundations
```

**Expected chunk count:** ~800–1200 chunks (12 files × 5–10 pages equivalent each)

**Key concepts to verify after ingestion:**
- Shannon entropy definition is retrievable
- HNSW algorithm description is retrievable
- UMRS relevance sections are included in chunks
- Cross-references appear in multiple chunk contexts

---

## Status

| Phase | Status | Files |
|---|---|---|
| 1A — Core Information Theory | Complete | 4 summary files |
| 1B — Graph Theory & Search | Complete | 4 summary files |
| 1C — Modern ANN | Complete | 2 summary files |
| 1D — Cross-Discipline | Complete | 1 spectral, 1 AC, 1 bridges synthesis |
| RAG Ingestion | Pending | Run ingest command above |
| Corpus Familiarization | Pending | Post-ingestion |
