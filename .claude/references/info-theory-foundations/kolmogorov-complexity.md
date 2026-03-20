# Kolmogorov Complexity and Algorithmic Information Theory

**Authors:** Andrei N. Kolmogorov, Ray Solomonoff, Gregory Chaitin (independent co-inventors)
**Key Papers:**
- Solomonoff, R.J. (1964). "A formal theory of inductive inference." *Information and Control*, 7(1), 1–22.
- Kolmogorov, A.N. (1965). "Three approaches to the quantitative definition of information." *Problems of Information Transmission*, 1(1), 1–7.
- Chaitin, G.J. (1966). "On the length of programs for computing finite binary sequences." *Journal of the ACM*, 13(4), 547–569.
- Li, M. & Vitányi, P. (2008). *An Introduction to Kolmogorov Complexity and Its Applications*, 3rd ed. Springer.
**Source URL (handbook survey):** https://homepages.cwi.nl/~paulv/papers/handbooklogic07.pdf

---

## Overview

Algorithmic Information Theory (AIT), also called Kolmogorov complexity theory, provides
a definition of the information content of individual strings — without reference to
probability distributions. Where Shannon entropy measures the average bits needed to
encode a source ensemble, Kolmogorov complexity measures the irreducible information
in a specific object.

The Kolmogorov complexity K(x) of a string x is the length of the shortest program
(on a universal Turing machine) that outputs x and halts. Informally: K(x) is the
length of the most compressed description of x achievable by any algorithm.

This resolves a philosophical problem with Shannon entropy: Shannon's entropy is
defined for ensembles (probability distributions), not individual objects. AIT extends
information theory to individual strings and allows reasoning about the complexity of
specific system configurations, individual security contexts, or particular corpus documents.

---

## Key Concepts and Results

### Definition of Kolmogorov Complexity

For a string x ∈ {0,1}*, relative to a universal Turing machine U:

    K_U(x) = min{|p| : U(p) = x}

where |p| is the length of program p. The crucial result is that this is stable up
to an additive constant: for any two universal machines U and U', K_U(x) and K_U'(x)
differ by at most a constant (the size of a translator between the machines). This
makes K(x) a machine-independent concept up to additive constants.

### Conditional Complexity

    K(x|y) = min{|p| : U(p,y) = x}

K(x|y) is the shortest program that produces x given y as auxiliary input. Key identities:

    K(x,y) ≤ K(x) + K(y) + O(log K(x))  (subadditivity)
    K(x) ≤ K(x|y) + K(y) + O(1)
    I(x:y) = K(x) + K(y) - K(x,y) ± O(log K(x,y))  (algorithmic mutual information)

### Incompressibility and Randomness

- A string x is **K-incompressible** (random) if K(x) ≥ |x|: no program shorter than
  x itself can produce it. Most strings are incompressible.
- For any n: there exist 2^n strings of length n, but only 2^(n-c) programs of length
  n-c. So at most 2^(n-c) strings can have K(x) < n-c — the vast majority are
  incompressible.
- **Martin-Löf randomness:** A sequence is random iff it passes all effective statistical
  tests. Equivalent to being incompressible in the Kolmogorov sense.

### Incompressibility Method

The incompressibility method is a proof technique: assume a string x is incompressible
(K(x) ≥ |x|), then show that any algorithm claiming to solve the problem would produce
a shorter description of x, contradiction. Used to prove lower bounds on:
- Sorting complexity
- Data structure space requirements
- Formal language complexity

This is a fundamental technique for proving that certain computational tasks inherently
require high complexity.

### Chaitin's Omega (Ω)

Chaitin's Ω is the halting probability of a universal Turing machine with uniformly
random program:

    Ω = sum_{p halts} 2^{-|p|}

Ω is a real number between 0 and 1. Its binary expansion is maximally random (K(Ω_n) ≥ n
for the first n bits). Ω encodes the answer to all finitely-refutable mathematical
statements. It is definable but uncomputable — a certificate that there are true mathematical
facts that can never be proven.

### Normalized Information Distance

    NID(x,y) = max(K(x|y), K(y|x)) / max(K(x), K(y))

NID is a normalized distance (≥ 0, ≤ 1) that equals 0 iff x and y are identical
(up to a constant), and equals 1 iff x and y share no algorithmic information.
NID is the universal similarity metric — it is an upper bound on all normalized
distances computable by any "admissible" metric.

**Normalized Compression Distance (NCD):** A computable approximation to NID:

    NCD(x,y) = (C(xy) - min(C(x), C(y))) / max(C(x), C(y))

where C denotes the output length of a real compressor (gzip, zstd, bz2). NCD can
cluster documents, detect plagiarism, measure biological sequence similarity, and
identify authorship — without any domain-specific features.

### Minimum Description Length and AIT

The link between Rissanen's MDL and Kolmogorov complexity:
- K(x) is the idealized (uncomputable) MDL — the shortest description achievable by
  any algorithm
- Rissanen's MDL provides computable approximations by restricting to parametric
  model classes
- The coding theorem: -log P_U(x) ≤ K(x) + O(1), where P_U is the universal prior
  (Solomonoff measure). The universal prior assigns probability 2^{-K(x)} to each
  string, matching its compressibility.

### Algorithmic Mutual Information

    I(x:y) = K(y) - K(y|x) = K(x) - K(x|y) ± O(log K(x,y))

This measures the algorithmic information that x and y share — how much knowing x
allows compression of y. Unlike Shannon mutual information, this is defined for
individual strings rather than distributions.

---

## UMRS Relevance

### Canonical Representation of Security Contexts

AIT provides the theoretical justification for canonical security context forms:
- The canonical form of a security context class is the shortest description achievable
  by an effective procedure — the K-complexity of the equivalence class
- Two security context strings that compute to the same access control decision share
  high algorithmic mutual information I(x:y) — they can be normalized to a canonical form
- The complexity K(policy) of an SELinux policy file bounds the minimum size of any
  equivalent policy representation

### Deduplication in Reference Library

NCD is directly applicable to UMRS reference library management:
- Compute NCD(doc1, doc2) using zstd to measure document similarity
- Documents with NCD < 0.1 are near-duplicates and should be merged or one dropped
- Documents with NCD > 0.9 are informationally independent — both should be kept
- NCD clustering produces a natural taxonomy of the reference library without any
  domain-specific feature engineering

This is a practical tool for corpus quality assurance (Phase 3C in the acquisition plan).

### Anomaly Detection via Compression

The incompressibility method yields a practical anomaly detector:
- Concatenate a new system state observation O with a corpus of normal observations N
- Compute C(N+O) - C(N) (incremental compression gain)
- If O is anomalous, it shares little information with N, so C(N+O) ≈ C(N) + C(O):
  the incremental gain is small
- If O is normal, it shares structure with N, so C(N+O) << C(N) + C(O): large gain
- Anomaly score = C(O) / (C(N+O) - C(N))

This is model-free and requires no labeled training data — only a corpus of normal observations.

### MLS Lattice and Information Flow

Kolmogorov complexity provides a rigorous basis for defining information flow in MLS:
- The information content of a classified document at level L is K(document|level)
  bits relative to the level context
- Information flow from level L1 to level L2 (where L1 dominates L2) is the algorithmic
  mutual information I(doc_L1 : channel_output) detectable by a subject at level L2
- Covert channels are exactly those channels that transmit I(signal:response) > 0
  bits between security levels despite policy prohibitions

This connects directly to Bell-LaPadula policy enforcement: the goal is to ensure
I(high_data : low_observable) = 0, which is the algorithmic definition of non-interference.

### Corpus Comparison for RAG Collections

When deciding whether a new RAG collection adds novel information:
- NCD(new_collection, existing_corpus) measures how much novel structure the new
  collection contributes
- If NCD is low, the new collection is largely redundant with existing material
- If NCD is high, the collection brings genuinely new information worth ingesting

### Minimum Viable Policy

AIT bounds the minimum viable SELinux policy: the shortest effective policy is the
one with complexity K(policy) closest to K(access_matrix), where K(access_matrix) is
the irreducible complexity of the intended access control decisions. Policy bloat
(rules that could be subsumed by simpler rules) increases K(policy) beyond the
information-theoretic minimum.

---

## Key Terminology

- **Kolmogorov complexity K(x):** Length of shortest program producing string x
- **Universal Turing machine:** Reference machine for K(x) definition; choice matters only up to a constant
- **Incompressible string:** K(x) ≥ |x|; no shorter description exists
- **Martin-Löf randomness:** Passing all effective statistical tests; equivalent to incompressibility
- **Normalized Information Distance (NID):** Universal similarity metric in [0,1]
- **Normalized Compression Distance (NCD):** Computable approximation to NID
- **Algorithmic mutual information:** Shared algorithmic structure between two strings
- **Chaitin's Ω:** Maximally random real number; encodes halting problem
- **Solomonoff prior:** Universal prior assigning 2^{-K(x)} to each string x
- **Incompressibility method:** Proof technique using randomly-chosen strings

---

## Cross-References

- **Shannon 1948** — ensemble (probabilistic) information theory; AIT extends to individual strings
- **MacKay ITILA** — Bayesian inference connection; Solomonoff prior as ultimate Bayesian prior
- **MDL/Rissanen** — MDL is the computable (restricted) version of AIT; K(x) is the idealized limit
- **Spectral clustering** — Graph complexity can be measured algorithmically; K(graph) bounds
  the minimum description of a community structure
- **HNSW** — The HNSW graph has computable complexity; near-optimal HNSW minimizes K(graph)
  subject to navigability constraints
