# A Note on Two Problems in Connexion with Graphs

**Authors:** Edsger W. Dijkstra
**Year:** 1959
**Published in:** Numerische Mathematik, Vol. 1, pp. 269–271
**Source URL:** https://ir.cwi.nl/pub/9256/9256D.pdf

---

## Overview

This three-page paper contains two algorithms. The first is the famous Dijkstra's
shortest path algorithm — the foundational algorithm for finding minimum-cost paths
in weighted graphs with non-negative edge weights. The second is a minimum spanning
tree algorithm (independently discovered; Prim published a similar algorithm the
same year). Together they define the template for greedy graph algorithms that
became the basis of nearly every network routing protocol, graph database traversal,
and heuristic search method developed in the following six decades.

Dijkstra's algorithm is not just practically important — it exemplifies the design
pattern of **priority-based greedy search with an admissibility guarantee**: the
algorithm always extends the cheapest known path, and never revisits a settled node,
yet provably finds optimal paths.

---

## Algorithm Description

### Dijkstra's Shortest Path Algorithm

**Input:** Weighted directed graph G = (V, E, w) with w: E → R≥0; source vertex s
**Output:** dist(v) = minimum-cost path from s to every vertex v

**Algorithm:**
1. Initialize dist(s) = 0; dist(v) = ∞ for all v ≠ s
2. Priority queue Q ← all vertices, priority = dist(v)
3. While Q is non-empty:
   a. Extract vertex u with minimum dist(u) from Q
   b. For each neighbor v of u:
      - If dist(u) + w(u,v) < dist(v):
        - dist(v) ← dist(u) + w(u,v)
        - Update priority of v in Q (decrease-key)
4. Return dist

**Correctness:** By induction. When vertex u is extracted, dist(u) is optimal.
Proof: any path that arrives at u via a vertex w not yet extracted must have
dist(w) ≥ dist(u) (since u was extracted first), so the path through w cannot
be shorter.

**Complexity:**
- Naive array: O(V²) — optimal for dense graphs
- Binary heap: O((V + E) log V) — practical for sparse graphs
- Fibonacci heap: O(E + V log V) — theoretically optimal; Fredman & Tarjan 1987

### Minimum Spanning Tree Algorithm

**Input:** Undirected weighted graph G = (V, E, w)
**Output:** Spanning tree T ⊆ E with minimum total weight

Dijkstra's MST procedure is essentially Prim's algorithm:
1. Start with an arbitrary vertex in T
2. Repeatedly add the minimum-weight edge from T to a vertex not yet in T
3. Until all vertices are included

Greedy correctness follows from the cut property of matroids.

---

## Key Principles Illustrated

### Greedy Optimality with Non-Negative Weights

The critical constraint is w(e) ≥ 0 for all edges. If negative weights are allowed,
Dijkstra's algorithm fails (Bellman-Ford handles negative weights at O(VE) cost).
The non-negativity condition ensures that adding more edges to a path can only
increase its cost — which is what makes greedy extraction of the minimum-cost
frontier vertex correct.

### Priority Queue as Abstract Data Type

The algorithm's efficiency depends entirely on the priority queue implementation:
- **Decrease-key operation:** When a shorter path to v is found, its priority must
  be updated. This is the bottleneck operation.
- Fibonacci heaps provide amortized O(1) decrease-key, making the overall complexity
  O(E + V log V) — but constant factors make binary heaps faster in practice.
- Modern implementations use indexed priority queues (d-ary heaps) for cache efficiency.

### Settled Set Invariant

The "settled" set S contains vertices whose shortest path is finalized. At each step:
- S contains vertices already extracted
- The algorithm only processes each vertex once (O(V) extractions total)
- Once settled, a vertex's distance is never updated

This invariant enables the algorithm to avoid exponential path enumeration — it is
the formal reason the algorithm runs in polynomial time.

---

## UMRS Relevance

### Graph Traversal in Posture Signal Dependency Analysis

When UMRS posture signals have dependencies (e.g., "kernel module loading" signal
depends on "module signing" signal and "secure boot" signal), the dependency graph
can be traversed with Dijkstra-style priority search:
- Edge weights represent the "inference cost" of deriving one signal from others
- Dijkstra finds the most efficient derivation path for any given signal
- The settled-set invariant guarantees each signal is assessed exactly once

### SELinux Policy Graph Analysis

An SELinux policy defines a directed access-allow graph:
- Vertices: types and roles
- Edges: allow rules (with associated permissions as edge labels)
- Shortest path in this graph: minimum number of type transitions needed to reach
  a target context from an initial context

Dijkstra's algorithm can answer questions like:
- "What is the minimal sequence of domain transitions from context A to context B?"
- "Are two types reachable from each other under the current policy?"

This has direct applications in policy audit tooling and minimum-privilege analysis.

### ChromaDB Index Traversal

Vector database query algorithms traverse graph structures:
- HNSW (see separate entry) uses a hierarchical graph; layer-by-layer traversal
  resembles Dijkstra with approximate distance as the priority
- The "settled vertex" concept appears in HNSW as the "visited" set that prevents
  re-expanding nodes

### Reference Library Update Tracking

The manifest tracks document versions and dependencies. When multiple documents
reference the same NIST control, a dependency graph can be built:
- Vertices: documents and controls
- Edges: "cites" relationships
- Dijkstra-style traversal identifies which documents need updating when a control
  changes, in the order they are most directly affected

---

## Key Terminology

- **Shortest path:** Minimum-weight path from source to target vertex
- **Priority queue:** Data structure supporting insert, extract-min, decrease-key
- **Settled vertex:** Vertex whose shortest path is finalized; extracted from Q
- **Decrease-key:** Priority queue operation to update a vertex's distance
- **Spanning tree:** Acyclic subgraph connecting all vertices; MST minimizes weight
- **Relaxation:** The operation of updating dist(v) via a shorter path through u
- **Greedy algorithm:** Makes locally optimal choices at each step; globally optimal
  when the greedy choice property and optimal substructure hold

---

## Cross-References

- **A* Search (Hart et al. 1968)** — A* extends Dijkstra by adding a heuristic estimate
  h(v) of remaining distance; Dijkstra is the special case h(v) = 0
- **Edmonds 1965** — matroid intersection generalizes the greedy MST principle; both
  rely on exchange properties of matroid structure
- **HNSW** — graph-based ANN search uses Dijkstra-like beam search on the navigable
  small-world graph
- **Spectral clustering** — shortest paths in Euclidean graphs relate to graph Laplacian
  eigenstructure; commute-time distance is related to effective resistance
- **Avis & Fukuda** — reverse search is an alternative paradigm: instead of greedy forward
  search, enumerate all solutions by reversing the greedy process
