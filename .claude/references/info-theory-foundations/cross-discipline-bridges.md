# Cross-Discipline Bridges: Information Theory, Graph Theory, and Search

**Purpose:** This document synthesizes the intersections between the corpus documents,
identifies where the theoretical frameworks reinforce each other, and maps the combined
insights to UMRS architectural decisions.

**Date:** 2026-03-20

---

## Bridge 1: Information Theory ↔ Graph Theory

### LDPC Codes and Tanner Graphs

Low-Density Parity-Check (LDPC) codes (covered in MacKay ITILA) achieve near-Shannon-limit
performance precisely because their structure is a sparse bipartite graph — the Tanner graph.
The sparsity of this graph enables belief propagation decoding, which runs in linear time.

**Key intersection:** The graph structure (sparsity, cycle-freeness) determines the information-
theoretic performance (distance from Shannon limit). This is the deepest known connection between
graph theory and information theory: the code is the graph.

**UMRS application:** The posture signal dependency graph is a Tanner graph: signals are
variable nodes, compliance rules are check nodes, and belief propagation on this graph computes
the probability that each signal is compliant given noisy observations. If the dependency graph
is sparse (few interdependencies), BP runs efficiently and converges to correct posteriors.

### Network Information Theory

Network information theory (Ahlswede et al.; Koetter-Médard) studies information transmission
over networks where nodes can code (combine) rather than just route:
- Multi-source, multi-sink problems on directed graphs
- The min-cut max-flow theorem extends to network coding: the Shannon capacity from source to
  sink equals the minimum graph cut capacity (Ford-Fulkerson × information theory)
- Linear network coding achieves capacity for single-source multicast

**UMRS application:** The MLS security lattice defines a directed graph with "channel capacity"
at each edge. Bell-LaPadula no-write-down is a zero-capacity cut from high to low security levels.
Network coding's min-cut theorem tells us the maximum information flow that can leak through
covert channels — a formal measure of policy strength.

### Spectral Graph Theory and Information Flow

The graph Laplacian's eigenvalues (spectral clustering, von Luxburg) directly encode information
flow bottlenecks:
- The Cheeger constant h(G) bounds the minimum-conductance cut — the weakest link in the
  information flow network
- The Fiedler value λ_2 = gap(G) determines the mixing time of random walks: τ_mix ~ 1/λ_2
- Large λ_2 (spectral gap) → fast mixing → efficient information diffusion

**Key formula (Cheeger's inequality):** h²/2 ≤ λ_2 ≤ 2h

**UMRS application:** In a covert channel analysis of SELinux policy, the information flow
graph's Cheeger constant bounds the maximum information leakage per unit time. Policies with
large Cheeger constants are more vulnerable to timing covert channels.

---

## Bridge 2: Graph Theory ↔ Search

### Greedy Algorithms on Matroids

Edmonds (1965) established that greedy algorithms are optimal precisely on matroids —
structures with the exchange property. Dijkstra's shortest path algorithm (1959) is greedy
and optimal for graphs with non-negative weights. The connection:
- The set of "non-dominated path prefixes" (Dijkstra's settled set) is a matroid basis
- Dijkstra's optimality proof is a matroid proof: the greedy augmentation is correct
  because the matroid exchange property holds

A* (1968) extends this: the admissibility of the heuristic is the condition that makes
the greedy search optimal despite using estimated costs.

**Key synthesis:** Dijkstra → A* → HNSW forms a progression:
- Dijkstra: exact, no heuristic (h=0)
- A*: exact, with admissible heuristic (h ≤ h*)
- HNSW: approximate, with learned structure (skip-list hierarchy replaces h)

Each step trades some guarantee (optimality → high recall) for efficiency (fewer expansions).
The information content of the heuristic determines where on the optimality-efficiency curve
the algorithm sits.

### Reverse Search as the Complement of Forward Search

Avis & Fukuda's reverse search and Dijkstra/A* are dual paradigms:
- Forward search: find one optimal solution efficiently
- Reverse search: enumerate all solutions in bounded memory

Together they provide the complete picture of the solution space:
- A* answers "what is the minimum?"
- Reverse search answers "how many solutions are within ε of the minimum?"
- The ratio (number near-optimal solutions) / (total solutions) is an entropy measure

**UMRS application:** For SELinux policy analysis, A* finds the minimal set of type
enforcement rules that satisfies a given security requirement. Reverse search enumerates
all such minimal sets, enabling the operator to choose based on secondary criteria
(manageability, auditability, least surprise).

### HNSW as Approximate A* on Learned Graphs

HNSW's layer-by-layer search can be interpreted as A* with:
- The entry point of each layer as the heuristic starting state
- The upper-layer distance as h(n) — an estimate of how far the true nearest neighbor is
- The beam width ef as a relaxation of A*'s exact expansion

The critical difference: A*'s heuristic is designed by a human (or hand-crafted); HNSW's
"heuristic" is learned from the data structure during index construction. The skip-list
hierarchy emerges from random layer assignment + greedy edge selection — it is a form of
unsupervised heuristic learning.

---

## Bridge 3: Information Theory ↔ Search

### Entropy-Driven Search Ordering

In any branching search, the optimal order to try alternatives is determined by their
information content — the Shannon entropy of which branch contains the solution:
- Try the branch with highest probability first (most likely to succeed)
- This minimizes expected search depth (E[depth] is minimized by probability-ordered search)
- The minimum expected depth equals the entropy of the solution distribution H(solution)

**This connects A* and information theory directly:** A*'s f(n) = g(n) + h(n) is an
approximation to the log-likelihood ratio of the optimal path. The admissibility condition
h ≤ h* ensures that the algorithm never eliminates the optimal solution from consideration —
analogous to ensuring the likelihood ratio threshold is conservative enough.

### Compression as Search Space Reduction

Any compression of data is equivalent to pruning a search tree:
- Huffman coding assigns shorter codes to more probable outcomes → fewer branches needed
  to reach common outcomes
- Arithmetic coding implicitly navigates a trie; compressed output is the search path
- LZ77 compression builds a dictionary that prunes the space of all possible next symbols
  to those observed before — dramatically reducing search depth

**MDL as search policy:** Rissanen's MDL selects the model that most compresses the data.
This is equivalent to selecting the search space pruning strategy that leaves the fewest
branches while still containing the true model. MDL = optimal search compression.

### Rate-Distortion ↔ Approximate Search

The rate-distortion trade-off (MacKay ITILA; Shannon 1948) and the recall-speed Pareto
frontier of ANN search (HNSW, ANN survey) are the same mathematical structure:
- R(D): minimum bits (cost) to represent a source within distortion D
- Recall(QPS): maximum recall (quality) achievable at query speed QPS (cost)

Both are decreasing monotone curves (higher cost → better quality). Both have concave
shape (diminishing returns). Both have fundamental limits (Shannon limit; brute-force recall).

**Practical implication:** Tuning ef in HNSW is rate-distortion coding for vector search.
The optimal ef is the knee of the recall-QPS curve, analogous to the operational point on
the R(D) curve.

### Entropy of the Retrieval Distribution

Given a query q and collection D, the entropy of the relevance distribution
H(relevant document | q) measures how difficult the retrieval problem is:
- H = 0: one document is overwhelmingly relevant → retrieval is easy
- H = log|D|: all documents are equally relevant → retrieval is as hard as guessing

High-entropy queries are the hard cases for RAG. They correspond to queries where many
documents are relevant — requiring more retrieved chunks and more sophisticated synthesis.

**UMRS application:** When a NIST control query returns high-entropy results (many equally-
relevant chunks from many documents), the RAG system should increase K (retrieved chunks)
and flag the query as "requires synthesis, not just lookup."

---

## Bridge 4: Kolmogorov Complexity ↔ All Three Domains

### Kolmogorov as the Unified Foundation

Kolmogorov complexity provides the deepest theoretical unification:
- **Compression:** K(x) is the ultimate compression limit; MDL and Shannon entropy are
  computable approximations
- **Graph complexity:** K(graph) measures the intrinsic description length of a graph;
  random graphs have K(G) ≈ |G| (incompressible)
- **Search complexity:** The number of steps in any search is bounded below by K(target|start)
  / K(step) — the algorithmic information needed to reach the target from the start

**UMRS application:** The algorithmic complexity K(SELinux_policy) is the irreducible
description length of the policy. Any policy representation larger than K(policy) contains
redundancy that can be eliminated. This is the formal basis for minimum-privilege analysis:
remove all policy rules without increasing K(policy) — the remaining rules are information-
theoretically necessary.

### NCD (Normalized Compression Distance) as Universal Similarity

NCD provides a computable, domain-independent similarity measure applicable to:
- **Document similarity:** NCD(doc1, doc2) using gzip/zstd
- **Graph similarity:** NCD(adjacency_matrix1, adjacency_matrix2)
- **Posture signal correlation:** NCD(signal_value_sequence1, signal_value_sequence2)

NCD is the compression-distance analog of cosine similarity — both measure shared structure,
but NCD uses algorithmic information (compression) while cosine uses geometric projection.

**UMRS application:** For reference library management, NCD quickly identifies near-duplicate
documents without any domain-specific feature engineering. For posture signal deduplication,
NCD identifies signals that carry redundant information across observed system states.

---

## Bridge 5: Analytic Combinatorics ↔ Everything

### AC as the Analysis Tool for All Other Methods

Analytic combinatorics provides the mathematical machinery to analyze the behavior of all
algorithms in this corpus:

| Algorithm | AC Analysis Tool | Key Result |
|---|---|---|
| Huffman coding | GF for prefix trees | Expected code length = H + O(1) |
| Quicksort | EGF for BSTs | E[comparisons] = 2(n+1)H_n - 4n |
| Dijkstra | GF for shortest paths | Mean path length in random graphs |
| A* | GF for search trees | Expected nodes expanded = f(h quality) |
| HNSW | GF for small-world graphs | Expected hops = O(log N / log log N) |
| Spectral clustering | Characteristic polynomial of L | Eigenvalue distribution |

AC is the lens that reveals the quantitative behavior hidden in the qualitative algorithm descriptions.

### Phase Transitions and Security

AC reveals phase transitions in combinatorial structures — points where qualitative behavior
changes abruptly as a parameter crosses a threshold. Classic examples:
- Random graph connectivity: at edge density p = log(n)/n, the graph becomes connected (sharp)
- SAT satisfiability: at clause/variable ratio α ≈ 4.267, 3-SAT transitions from satisfiable
  to unsatisfiable (sharp)

**UMRS application:** Security compliance has a phase-transition structure:
- Below a threshold of compliant signals, the system is qualitatively non-compliant
- Above the threshold, it is qualitatively compliant
- Near the threshold, small configuration changes produce large compliance swings

AC techniques can locate this phase transition and characterize the "compliance cliff" —
the set of signal values where the system is marginally compliant and most sensitive to change.

---

## Unified Insights for UMRS Architecture

### The Three Cs: Compression, Clustering, Search

Every UMRS retrieval operation involves all three:
1. **Compression** (information theory): documents are encoded as embeddings — lossy
   compression from text to vectors. Shannon entropy bounds information loss.
2. **Clustering** (spectral methods): similar documents cluster in embedding space.
   Spectral gap determines cluster separation quality.
3. **Search** (graph algorithms): HNSW navigates the cluster structure to find similar
   embeddings. A* admissibility provides the theoretical guarantee that would make search
   exact (HNSW approximates this guarantee).

Optimizing RAG requires optimizing all three simultaneously — they are not independent.
The rate-distortion trade-off at the embedding stage affects cluster quality, which
affects HNSW search quality.

### MDL as Corpus Management Policy

Treat the RAG corpus as a probabilistic model M for answering queries. Then:
- Adding a new document decreases L(queries|M) (queries are better answered)
- But increases L(M) (corpus is larger, harder to maintain)
- MDL selects the corpus size that minimizes L(M) + L(queries|M)

This operationalizes corpus management: add documents when they decrease the total
description length; remove documents when they are redundant (increase L(M) without
decreasing L(queries|M)).

### Matroid Structure of Compliance Constraints

NIST SP 800-53 controls have a dependency structure:
- Some controls are independent (can be implemented in any order)
- Some controls build on others (CM-6 requires CM-2; AU-12 requires AU-9)

The independence structure (which controls can be satisfied simultaneously without conflict)
is a matroid if and only if the greedy "implement easiest controls first" strategy achieves
maximum coverage. Edmonds' matroid theory tells us when this greedy strategy works —
and when it fails (when the structure is three-matroid or higher, requiring more sophisticated
algorithms like integer programming).

### Reverse Search for Policy Space Exploration

When UMRS's assessment engine identifies a non-compliant configuration, the operator needs
to know: "what are all the ways to fix this?"

Reverse search enumerates all minimum-change remediation paths:
- Define "canonical solution" as the fully-compliant configuration
- Define "parent" of a configuration C as C with one more violation (moving away from compliant)
- Reverse search on the violations graph enumerates all minimal fix sets

This is strictly more informative than A*'s single optimal remediation path — it gives the
operator the full solution space to choose from.

---

## Key Cross-Discipline Terms

| Concept | Information Theory | Graph Theory | Search |
|---|---|---|---|
| "Distance" | KL divergence / H | Shortest path length | f(n) cost |
| "Optimal" | H(X) bits | Min-cost path | f* value |
| "Approximate" | Rate-distortion D | ANN recall@K | Weighted A* factor w |
| "Redundancy" | H(X|Y) → 0 | Graph edge cut → 0 | Cycle in search tree |
| "Structure" | Entropy gradient | Laplacian eigenvalues | Heuristic h(n) |
| "Limit" | Shannon capacity C | Max-flow | Brute-force recall |
