# Paths, Trees, and Flowers

**Authors:** Jack Edmonds
**Year:** 1965
**Published in:** Canadian Journal of Mathematics, 17, 449–467
**Availability:** Not freely available as PDF; held by Cambridge University Press.
  Canonical reference: Edmonds, J. (1965). "Paths, trees, and flowers." *Canad. J. Math.*, 17, 449–467.
  Secondary access: Pulleyblank, W.R. (2009). "Edmonds, matching and the birth of polyhedral combinatorics."
  *Documenta Mathematica, Extra Volume ISMP*, 181–197.

---

## Overview

"Paths, Trees, and Flowers" is one of the most influential papers in theoretical computer
science and combinatorial optimization. It presents the first polynomial-time algorithm
for maximum matching in general graphs, and — more importantly — introduces the conceptual
framework that defined the boundary between "efficient" (polynomial-time) and "inefficient"
(exponential-time) algorithms, which Edmonds called "good algorithms" and which later
became the complexity class P.

The paper's contributions are threefold:
1. **The blossom algorithm:** The first polynomial-time algorithm for maximum matching
   in non-bipartite graphs
2. **The polynomial guarantee:** An explicit argument that the algorithm runs in
   polynomial time — one of the first such arguments in the literature
3. **The polyhedral view:** The matching polytope is characterized by a system of
   linear inequalities, connecting combinatorial algorithms to linear programming

Edmonds did not stop here. His subsequent papers on matroids and matroid intersection
created the general theory within which most combinatorial optimization algorithms live.
"Paths, Trees, and Flowers" is the founding document of this tradition.

---

## The Maximum Matching Problem

**Input:** Undirected graph G = (V, E)
**Output:** Maximum matching M ⊆ E (maximum set of edges with no shared vertex)

### Why General Graphs are Hard (Compared to Bipartite)

For **bipartite** graphs, maximum matching reduces to max-flow (König's theorem, 1931).
The key tool is augmenting paths: a matching is maximum iff there is no augmenting path
(alternating path from an unmatched vertex to another unmatched vertex).

For **general (non-bipartite) graphs**, the difficulty is odd cycles — a path can spiral
around an odd cycle and reach an unmatched vertex in a way that creates what appear to be
augmenting paths that don't actually improve the matching.

### Blossoms

The central concept is the **blossom**: an odd-length cycle (2k+1 edges) that can be
"shrunk" to a single vertex without changing whether a maximum matching exists. The blossom
operation replaces the odd cycle with a pseudonode:
- The original graph G is replaced by a contracted graph G/B
- A maximum matching in G/B can be lifted to a maximum matching in G
- This avoids the spurious augmenting paths caused by odd cycles

The blossom operation reduces the problem of matching in general graphs to matching in
graphs without odd cycles (bipartite graphs), applied recursively.

### The Blossom Algorithm

1. Find a maximal matching M (greedy initialization)
2. Build an "alternating forest" F: BFS/DFS from unmatched vertices along alternating edges
3. If an augmenting path is found: augment M and restart
4. If a blossom B is found (two same-label vertices in F connected by an edge):
   - Contract B to a pseudonode
   - Recurse on the contracted graph
5. If neither augmenting path nor blossom: M is maximum

**Complexity:** O(V³) in Edmonds' original; O(E √V) in the Micali-Vazirani 1980 improvement.

### The Matching Polytope

Edmonds proved that the convex hull of all matchings in graph G is characterized by:
- x_e ≥ 0 for all edges e
- sum_{e∋v} x_e ≤ 1 for all vertices v (degree constraint)
- sum_{e∈E(S)} x_e ≤ ⌊|S|/2⌋ for all odd sets S (blossom inequality)

This is the matching polytope — a complete characterization of the fractional relaxation
that has integral extreme points coinciding with integer matchings. The blossom inequalities
are the distinguishing feature: they cut off fractional solutions that exploit odd cycles.

---

## Matroids and the General Framework

Edmonds' broader contribution (in companion papers) is the theory of **matroids**:

### Matroid Definition

A matroid M = (E, I) is a set system where:
- I is a nonempty family of "independent" subsets of E
- **Hereditary:** If A ∈ I and B ⊆ A, then B ∈ I
- **Exchange property:** If A, B ∈ I and |A| > |B|, then ∃ x ∈ A\B such that B ∪ {x} ∈ I

Examples:
- **Graphic matroid:** E = edges of a graph, I = forests (acyclic subsets)
- **Linear matroid:** E = columns of a matrix, I = linearly independent subsets
- **Uniform matroid:** I = all subsets of size ≤ k

### Greedy Algorithm on Matroids

The fundamental theorem of matroids: a greedy algorithm (always add the cheapest
independent element) finds the minimum-weight maximum independent set in ANY matroid.
This unifies:
- Kruskal's MST algorithm (graphic matroid)
- Huffman coding (abstract matroid structure)
- Many scheduling and assignment algorithms

### Matroid Intersection

Two matroids on the same ground set E can be intersected: I1 ∩ I2 = {S : S ∈ I1 and S ∈ I2}.
Edmonds showed that the maximum-weight common independent set can be found in polynomial time.
Matroid intersection subsumes bipartite matching, arborescences, and many assignment problems.

Matroid intersection is the boundary of tractability: three-matroid intersection is NP-hard
in general. Knowing whether a problem is a two-matroid intersection problem tells us whether
it is polynomial-time solvable by greedy-style methods.

---

## The "Good Algorithm" Concept

Edmonds' casual remark in the paper is historically significant:
"I am not aware of a good algorithm for [general matching] in the sense of one whose
running time is bounded by a polynomial in the size of the problem."

At the time, this was informal. It crystallized into the formal definition of the complexity
class P (polynomial time) and the P vs. NP question (formalized by Cook 1971 and Karp 1972).
Edmonds is widely credited with identifying "polynomial time" as the right formal notion of
computational efficiency.

---

## UMRS Relevance

### Policy Assignment as Matching

SELinux type enforcement can be modeled as a matching problem:
- Left vertices: processes (subjects, with their context needs)
- Right vertices: resources (objects, with their labels)
- Edges: allowed access relations
- Maximum matching: the largest set of simultaneously satisfiable access requests

Maximum matching finds the most permissive policy compatible with the security lattice —
the starting point for minimum-privilege analysis (reduce edges until matching is not
affected).

### Matroid Structure of Policy Constraints

The constraints on a valid SELinux policy have matroid-like structure:
- **Graphic matroid:** The "type transition graph" (type→role→user relationships) is acyclic
  in compliant policies (no circular type transitions)
- **Partition matroid:** Each security level allows at most k active subjects (rate limiting)
- **Transversal matroid:** Each clearance level maps to a set of allowable types

Recognizing these as matroid constraints means greedy algorithms solve the minimum-policy
problem optimally — the minimum set of allow rules that satisfies all required access is
findable in polynomial time.

### Greedy Signal Assessment Order

The set of posture signals that can be assessed independently (without data dependencies)
forms an independence system. If this system is a matroid, then the optimal assessment
order (minimizing total evaluation time while covering maximum informational coverage)
is found by greedy algorithms.

### Blossom as Pattern in Dependency Graphs

In posture signal dependency graphs, a "blossom" structure corresponds to a circular
dependency — a cycle of signals where each depends on the previous. Edmonds' contraction
technique provides the right mental model for handling circular dependencies: contract the
cycle, resolve it as a unit, then expand.

### Matching Polytope and Constraint Certification

The matching polytope characterization (integer solutions = matching solutions) is the
model for UMRS compliance certification:
- Each NIST control constraint is a halfspace in the policy space
- The intersection of all constraints defines the compliance polytope
- An SELinux policy is compliant iff it is an integer vertex of this polytope
- The blossom inequalities have analogs: "odd-cycle" constraints that cut off fractional
  (partial-compliance) solutions

---

## Key Terminology

- **Matching:** Set of edges with no shared vertex
- **Maximum matching:** Largest cardinality matching
- **Augmenting path:** Alternating (unmatched-matched-...) path between unmatched vertices
- **Blossom:** Odd-length cycle in the alternating forest; the contraction trick handles odd cycles
- **Matroid:** Abstract structure where greedy algorithms work; generalizes graphic + linear independence
- **Exchange property:** Allows greedy augmentation in matroids
- **Matroid intersection:** Common independent set of two matroids; polynomial-time solvable
- **Matching polytope:** Convex hull of all matchings; characterized by blossom inequalities
- **"Good algorithm":** Edmonds' informal term for polynomial-time algorithm; precursor to class P

---

## Cross-References

- **Dijkstra 1959** — both use the "augmenting" paradigm: Dijkstra augments shortest path
  estimates; Edmonds augments matchings via augmenting paths
- **Avis & Fukuda** — reverse search on the bases of a matroid enumerates all maximal matchings
- **A* 1968** — the admissibility of A* relies on the same optimal substructure that makes
  greedy algorithms work on matroids
- **Spectral clustering** — the graph Laplacian eigenvalues bound matching numbers; spectral
  gap and maximum matching size are related via Cheeger's inequality
- **Shannon 1948** — maximum matching maximizes information content in bipartite graphs
  (channel coding problems are matching problems in disguise)
