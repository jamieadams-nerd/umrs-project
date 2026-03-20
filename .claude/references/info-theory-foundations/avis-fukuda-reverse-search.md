# Reverse Search for Combinatorial Optimization

**Authors:** David Avis, Komei Fukuda
**Year:** 1996 (core paper: 1993 for vertex enumeration version)
**Key Papers:**
- Avis, D. & Fukuda, K. (1996). "Reverse search for enumeration." *Discrete Applied Mathematics*, 65(1–3), 21–46.
- Avis, D. & Fukuda, K. (1993). "A pivoting algorithm for convex hulls and vertex enumeration of arrangements and polyhedra." *Discrete & Computational Geometry*, 8(1), 295–313.
**Source URL:** http://cgm.cs.mcgill.ca/~avis/doc/avis/AF96a.pdf

---

## Overview

Reverse search is a paradigm for exhaustive enumeration of combinatorial objects
(vertices of a polytope, spanning trees, bases of a matroid, maximal cliques, etc.)
using bounded space — specifically, O(n) space rather than the potentially exponential
space required by naive backtracking or BFS over the solution space.

The key insight: many enumeration problems have a natural "parent" function that
maps each solution to a unique "simpler" solution. This defines a spanning tree over
the solution space rooted at a canonical (easiest-to-find) solution. Reverse search
performs DFS over this tree without storing the tree — it reconstructs parent-child
relationships on the fly.

Reverse search is orthogonal to forward greedy search (Dijkstra, A*): where greedy
algorithms find one optimal solution efficiently, reverse search exhaustively
enumerates all solutions in bounded memory. This makes it the canonical method when
the task is to characterize the entire solution space rather than optimize.

---

## Algorithm Description

### Setup

Let F be the set of all feasible solutions (e.g., all vertices of a polytope, all
spanning trees of a graph). Assume:
1. A polynomial-time algorithm to find one "canonical" solution f* ∈ F (base case)
2. A function parent(f): F → F ∪ {∅} that maps each non-root solution to a unique
   "parent" solution closer to f* (defines an implicit tree T over F rooted at f*)
3. A polynomial-time algorithm to enumerate all "children" of any solution f:
   children(f) = {f' ∈ F : parent(f') = f}
4. An adjacency test: given f and a candidate f', check whether f' is an actual child

### The Reverse Search Tree

The parent function induces a spanning tree T over F:
- Root: f* (canonical solution)
- Edges: (parent(f), f) for each f ≠ f*
- DFS traversal of T visits every solution in F exactly once

The tree structure is "reverse" because the search goes from children to parent
in the parent-finding direction, but traverses in the forward (root-to-leaf)
direction during DFS.

### Enumeration Algorithm

```
procedure reverse_search(f):
  output f
  for each candidate adjacent solution f' adjacent to f:
    if parent(f') = f:    // f' is a child of f
      reverse_search(f')

// Called as: reverse_search(f*)
```

**Key property:** No solution is stored more than once; the implicit tree is
traversed without materializing it. Space complexity: O(depth of T × cost per step).

### Parent Function Design

The parent function is the algorithm's creative core. For vertex enumeration
of polytopes, Avis & Fukuda use a pivot operation (Bland's rule) as the parent
step — a lexicographically smallest sequence of pivots from f back to f*. For
spanning tree enumeration, parent(T) = T with one edge swapped to reduce a
potential function. The choice of parent function determines tree balance and
per-node work.

---

## Key Properties

### Completeness

Every solution in F is output exactly once. Proof: the parent function defines
a unique tree; DFS on a tree visits every node exactly once.

### Space Efficiency

Space: O(n) where n is the size of a single solution representation. Compare with:
- BFS over solution space: O(|F|) — exponential in the worst case
- Naive backtracking without duplicate detection: revisits solutions

Reverse search achieves exponential solution enumeration with polynomial space.

### Parallelism

The tree structure supports parallel enumeration: different subtrees can be
assigned to different processors with no coordination overhead (the tree is
implicitly defined; subtrees don't share solutions). Parallel reverse search
achieves near-linear speedup on enumeration problems.

### Applications

Reverse search applies to:
- **Vertex and facet enumeration of polytopes** — convex hull algorithms
- **Spanning tree enumeration** — complete tree listing in O(n + output) space
- **Maximal clique enumeration** — polynomial space enumeration
- **Triangulation enumeration** — of point sets and manifolds
- **Basis enumeration for matroids** — the general combinatorial setting

---

## Connection to Information Theory and Search

### Enumeration vs. Optimization

Reverse search and A*/Dijkstra are complementary:
- A* finds one optimal solution efficiently (focused search)
- Reverse search finds all solutions exhaustively (complete enumeration)
- Information-theoretically: A* answers the query "what is the minimum?" while
  reverse search answers "what is the full distribution over solutions?"

The full distribution is needed when:
1. Multiple near-optimal solutions exist and trade-offs matter
2. Counting solutions is required (e.g., "#SAT" problems)
3. The entire feasible region must be characterized for safety verification

### Implicit Backtracking and Non-Redundancy

A key challenge in enumeration is avoiding re-visiting solutions. Reverse search
solves this by encoding the path back to the root (via the parent function)
within each solution representation — no external visited set is needed. This is
a form of self-organizing data structure: the enumeration "remembers" where it
has been by following the implicit tree structure.

### Connections to MCMC and Sampling

Reverse search enumerates all solutions; MCMC sampling draws from the solution
distribution. The two are connected:
- Reverse search can compute partition functions exactly (by counting)
- The implicit tree from reverse search can guide MCMC samplers toward under-
  explored regions
- Importance sampling weights can be derived from the tree structure

---

## UMRS Relevance

### Complete Posture Space Enumeration

When validating UMRS posture assessment logic, it may be necessary to enumerate
all feasible combinations of signal values — the full configuration space F.
Reverse search makes this tractable when F is exponentially large:
- Define a canonical configuration (e.g., all signals at their default values)
- Define parent(config) as a configuration one step simpler
- Reverse search visits all feasible configurations in O(|signals|) space

This enables exhaustive testing of contradiction detection logic across all
possible signal combinations — a formal verification approach.

### SELinux Policy Space Analysis

The set of all satisfying access control matrices for a given security requirement
is a combinatorial search problem:
- F = {policies that satisfy the security requirement}
- Reverse search enumerates all such policies
- This gives the full space of compliant policy options, enabling minimum-privilege analysis

For example: what is the complete set of minimal SELinux type enforcement rules
that permit a specific application to run while satisfying a given NIST control
constraint? Reverse search characterizes this set exhaustively.

### MLS Lattice Enumeration

The set of all valid MLS label assignments satisfying a multi-level security
lattice is exponentially large. Reverse search can enumerate:
- All valid clearance combinations for a set of subjects
- All label assignments satisfying Bell-LaPadula for a given information flow graph
- All minimal policies that enforce a given information flow policy

### Catalog Completeness Verification

The UMRS posture catalog can be modeled as a combinatorial object. Reverse search
can verify that all required signal combinations are accounted for — no feasible
configuration is missing a catalog entry. The canonical solution is the fully-specified
catalog; the parent function reduces configurations by removing one requirement at a time.

### Reference Library Gap Detection

In corpus management (Phase 3C skill): enumerate all pairs (document, NIST control)
and verify that each control required by UMRS has at least one document in the library
addressing it. Reverse search on the "document × control" bipartite graph identifies
all maximal uncovered controls.

---

## Key Terminology

- **Reverse search:** Enumeration by DFS on an implicit tree defined by a parent function
- **Parent function:** Maps each non-root solution to a unique simpler solution
- **Implicit tree:** Tree over the solution space defined by the parent function; not stored
- **Vertex enumeration:** Finding all extreme points of a convex polytope
- **Spanning tree enumeration:** Complete listing of all spanning trees of a graph
- **Canonicalization:** Mapping solutions to unique representatives; used to define the root
- **Backtracking:** Alternative enumeration using explicit stack; less space-efficient
- **Matroid basis:** Combinatorial generalization of vector basis; reverse search applies

---

## Cross-References

- **Dijkstra 1959** — complementary: forward greedy finds one optimal solution;
  reverse search enumerates all solutions in the feasible set
- **A* 1968** — A* with optimal heuristic explores the tree of partial solutions;
  reverse search explores the tree of complete solutions
- **Edmonds 1965** — Edmonds' matroid theory provides the abstract framework;
  matroids are exactly the structures where greedy and enumeration both work cleanly
- **Shannon 1948** — counting solutions is equivalent to computing entropy of the
  solution distribution; reverse search computes this count exactly
- **Spectral clustering** — spectral methods partition graphs; reverse search
  enumerates all partitions, enabling comparison of spectral solution quality
