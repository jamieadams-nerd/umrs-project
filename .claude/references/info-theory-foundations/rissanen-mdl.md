# Minimum Description Length (MDL) Principle

**Authors:** Jorma Rissanen (primary; related work by Peter Grünwald, Paul Vitányi, Ming Li)
**Key Papers:**
- Rissanen, J. (1978). "Modeling by shortest data description." *Automatica*, 14, 465–471.
- Rissanen, J. (1983). "A universal prior for integers and estimation by minimum description length." *Annals of Statistics*, 11(2), 416–431.
- Rissanen, J. (1996). "Fisher information and stochastic complexity." *IEEE Transactions on Information Theory*, 42(1), 40–47.
- Grünwald, P. (2007). *The Minimum Description Length Principle*. MIT Press.
**Source URL (survey):** https://homepages.cwi.nl/~pdg/ftp/mdlintro.pdf

---

## Overview

The Minimum Description Length principle is a formal framework for model selection and
statistical inference grounded in information theory. The core idea: **the best model
of data is the one that produces the shortest total description of the model plus the
data given the model.** MDL operationalizes Occam's razor as a precise information-
theoretic criterion.

MDL originated in Rissanen's 1978 paper as a formalization of Akaike's information
criterion (AIC) and later Schwarz's Bayesian information criterion (BIC). Unlike those
heuristics, MDL has a rigorous information-theoretic foundation in universal coding theory.

---

## Core Principle

### Two-Part Codes

Given data D and a class of models M = {M1, M2, ...}, MDL selects the model that
minimizes:

    L(M) + L(D | M)

where:
- L(M) = code length needed to describe the model itself
- L(D|M) = code length needed to describe the data given the model

This is the two-part (crude) MDL criterion. The trade-off is explicit: complex models
have high L(M) but can achieve low L(D|M); simple models have low L(M) but may require
more bits to describe residuals.

### Normalized Maximum Likelihood (NML)

The refined MDL criterion uses the normalized maximum likelihood distribution:

    p_NML(x^n | M) = max_θ p(x^n | θ, M) / C_n(M)

where C_n(M) is the normalization constant (sum over all data sequences of their
maximum likelihood). The NML code length is:

    L_NML(x^n | M) = -log p_NML(x^n | M) = -log max_θ p(x^n|θ,M) + log C_n(M)

The term log C_n(M) is the **stochastic complexity** — a data-independent model
complexity measure. It naturally captures the effective number of free parameters
in a way that AIC/BIC do not.

### Parametric Complexity / COMP

For parametric models, the stochastic complexity grows as (k/2) log(n/2π) + log Vol(θ)
for smooth models with k parameters and n data points. This matches the BIC penalty
asymptotically but differs in finite-sample regimes.

The volume term log Vol(θ) captures the Fisher information geometry of the parameter
space — models with parameters that are harder to distinguish from each other have
lower parametric complexity because they "code" fewer distinguishable distributions.

---

## Key Results

### Universal Coding and MDL

Rissanen showed that no code can achieve expected code lengths shorter than the
stochastic complexity without prior knowledge of the true distribution. Universal
codes (Lempel-Ziv, PPM, context-tree weighting) achieve the stochastic complexity
asymptotically.

This creates a direct link between:
- **Model selection:** Choose the model class with lowest stochastic complexity
  for the observed data
- **Universal compression:** Use the model to compress data; the compressed length
  equals the stochastic complexity up to lower-order terms

### MDL vs. Bayesian Model Selection

Bayesian model selection uses the marginal likelihood (evidence) p(D|M), which
integrates over parameters. MDL with NML uses the normalized maximum likelihood.
These coincide when priors are chosen to match the Fisher information metric (Jeffreys
prior). For misspecified models, MDL has better behavior because it does not assume
the true model is in the comparison set.

### Prequential (Predictive) MDL

Rissanen's prequential version accumulates prediction losses:

    L_pred(D, M) = -sum_{i=1}^{n} log p(x_i | x_1...x_{i-1}, M)

This is the sequential prediction loss — how many bits would be needed to encode
each data point given past observations under model M. Prequential MDL connects to
online learning and is equivalent to the Shtarkov (NML) criterion asymptotically.

---

## UMRS Relevance

### Model Selection for Posture Analysis

MDL provides a principled framework for deciding how many distinct posture signal
categories are needed in the UMRS catalog:

- **Too few categories:** L(D|M) is high because many distinct configurations map
  to the same category, requiring extra bits to distinguish
- **Too many categories:** L(M) is high because the model itself is complex
- **MDL optimum:** The number of categories that minimizes total description length,
  avoiding both underfitting (too coarse) and overfitting (too fine-grained)

In practice: when adding a new posture signal or splitting an existing category,
ask whether the total MDL cost decreases. If not, the split does not improve the
model's explanatory power.

### Canonical Security Context Representations

SELinux security contexts (user:role:type:level) have a combinatorial structure.
MDL provides the criterion for:
- **Finding canonical forms:** The canonical representation is the one that achieves
  the shortest description of the observed context distribution
- **Deduplication:** Two contexts are "informationally equivalent" if replacing one
  with the other does not increase description length
- **Policy compression:** An SELinux policy is a model of the access control space;
  MDL selects the minimal policy that explains observed access patterns

### RAG Collection Curation

When deciding whether to add a new document to a RAG collection, MDL provides the
criterion: does adding the document reduce the total description length of all queries
the collection must answer? Documents that are redundant (already covered by other
collection members) increase L(M) without decreasing L(D|M).

This is operationally useful for managing corpus size and avoiding redundancy in the
UMRS reference library.

### Anomaly Detection in System State

MDL-based anomaly detection: encode normal system behavior as a model M_normal.
An observation O is anomalous if L(M_normal) + L(O|M_normal) > L(M_anomaly) +
L(O|M_anomaly) for some simpler anomaly model M_anomaly. Equivalently, O is
anomalous if it requires more bits to describe under the normal model than under
a model specialized to anomalies.

This is directly applicable to posture contradiction detection — contradictions are
configurations that are expensive to describe under the "compliant system" model.

---

## Key Terminology

- **Description length:** Number of bits required to encode an object using a code
- **Two-part code:** Encode model then residuals; sum is the MDL criterion
- **Stochastic complexity:** Optimal (minimum) code length for data under a model class
- **NML (Normalized Maximum Likelihood):** Minimax-optimal predictive distribution
- **Parametric complexity:** Data-independent complexity of a model class
- **Prequential code:** Sequential prediction-based description length
- **Fisher information:** Riemannian metric on parameter space; governs MDL geometry
- **Universal code:** Code that achieves near-entropy compression without knowing source

---

## Cross-References

- **Shannon 1948** — entropy is the foundation; MDL code lengths are Shannon optimal codes
- **MacKay ITILA** — Chapter 28 covers MDL and Bayesian model comparison; treats them as dual
- **Kolmogorov complexity** — Kolmogorov complexity K(x) is the idealized (uncomputable) limit
  of MDL; MDL provides computable approximations via model classes
- **Spectral clustering** — MDL can be used to select the number of clusters (model order),
  complementing spectral gap heuristics
- **HNSW** — HNSW graph construction implicitly minimizes a description length: the graph
  that enables shortest-path search with fewest edges
