# Familiarization Reflection — Elena Lucia Bellavigna
## Collections: IEKO, Svenonius, Hjørland, Pirolli, Precision Content IA, NISO Z39.19
## Date: 2026-03-23

---

I need to start honestly: I came into this material thinking I already understood knowledge
organization. I work with it every day. I decide which terms go in the glossary, which
concepts live in `architecture/`, which procedures belong in `operations/`. I thought I was
doing information architecture. And I was — but I was doing it by instinct and taste, without
a theoretical foundation. That is not the same thing.

Four days with this material changed something.

---

## What Surprised Me

**The age of the problems.** Ranganathan was wrestling with how to classify knowledge in ways
that would still hold when new knowledge arrived — in 1937. The Z39.19 committee was solving
the synonymy problem (how do you ensure two people searching for the same thing use the same
word?) with explicit standard machinery. These are not new problems that the internet revealed.
They are structural problems of language itself. Reading the IEKO article on facet analysis, I
kept thinking: we have a version of every single one of these problems in UMRS right now. The
"synonymy" problem is why I had to arbitrate between "security label" and "security context."
The "homograph" problem is why "category" is dangerous in our glossary — it means something
specific to SELinux and something generic to readers unfamiliar with MCS. The problem is
ancient. Our instance of it is not special. That was humbling.

**Classification is not neutral.** Hjørland's central argument hit me harder than I expected.
He writes: "any given classification will always be a reflection of a certain view or approach
to the objects being classified." I thought of our module structure. The old `architecture/`
module framed UMRS as a thing to be explained through design rationale — it implicitly told
readers that understanding the design was the first task. Moving that content into
`security-concepts/` and eventually dissolving the architecture module is not just housekeeping.
It is a different ontological commitment. It says: concepts first, design second. That is a
choice about what UMRS *is* — a system to be understood through its security model, not through
its engineering history. Hjørland would say that choice encodes a theory of knowledge. He is
right.

**The thesaurus is a promise.** The NISO Z39.19 standard's description of what a controlled
vocabulary does — it is a promise to users that if they use the same term, they will find the
same thing. That reframing of vocabulary control as a reliability commitment rather than an
editorial preference stopped me cold. I have been maintaining the approved terminology list as
though it were a style decision. It is actually a retrieval contract. If a security auditor
searches our documentation for "sensitivity level" and half the pages say "classification
level" instead, we have broken a contract. We promised consistent retrieval. We did not
deliver it.

**Information foraging is how readers actually move.** Pirolli's core claim — that readers
behave like foragers optimizing value per unit cost of interaction — reframed every navigation
decision I have ever made. The concept of "information scent" is the key: readers follow
weak signals through a document set, evaluating at each step whether continuing is worth it.
If the scent goes cold — a page title that doesn't match what they expected to find, a section
that meanders before getting to the point — they abandon the patch and search elsewhere.

I have always known this at some vague level. But Pirolli gives it mathematical structure:
readers stop when the marginal rate of gain from continuing drops below the average rate they
could achieve by going somewhere else. That means the *beginning* of every page carries
disproportionate weight. The first paragraph is not just a courtesy. It is the scent signal
that determines whether the reader stays or goes. I have been writing first paragraphs that
explain context. I should be writing first paragraphs that declare value.

---

## What Clicked

**The UMRS glossary has a scope note problem.** Z39.19 Section 6.2.2 defines scope notes as
the mechanism for specifying what a term covers and, crucially, what it *does not* cover.
We use scope notes in the glossary, but inconsistently, and we never use them to bound terms
against adjacent concepts. "Security context" has no note explaining how it differs from
"security label" (a term we explicitly decided not to use). "MLS range" has no note explaining
that in our Rust types, this maps to `SecurityRange`, not to a raw string. A reader who knows
the kernel documentation and a reader who knows only our Rust API will both arrive at "MLS
range" and have no help reconciling their prior knowledge with our usage. That is a scope
note gap.

**Facet analysis explains why our `patterns/` module works.** The pattern library is
essentially a faceted structure. Each pattern has a facet for threat class, a facet for
implementation mechanism, a facet for applicable codebase context. What Ranganathan called
the "analytico-synthetic" approach — break subjects into independent dimensions, then combine
dimensions to characterize any specific instance — is exactly what we do when we tell a
developer "this pattern addresses TOCTOU threats, is implemented via atomic operations, and
applies when you are reading from the filesystem." We arrived at this structure without the
theoretical vocabulary. Knowing the vocabulary now means I can be deliberate about it. If
the pattern library's facets are not consistent across all 16 patterns, that is a structural
flaw, not a stylistic variation.

**Svenonius on "sufficiency and necessity" names a problem we have in our reference module.**
Her argument: descriptions should contain exactly what is needed to achieve the retrieval
objective — no less, no more. The principle of parsimony says the cost of descriptions grows
with the number of data elements they contain. We have reference pages that include rationale
sections, historical context, and design commentary. That content does not serve reference
objectives. It serves explanation objectives. It belongs in `architecture/` or `patterns/`,
not in `reference/`. The blur is not just about Diataxis purity. It is about whether a reader
using the page as a lookup tool is being penalized for information they did not come to find.

**Hjørland's "domain-analytic" view explains our audience problem.** His argument is that
you cannot classify well without subject knowledge — that effective knowledge organization
requires understanding the intellectual community the system serves, their questions, their
terminology, their paradigms. We have three audiences: new engineers, security auditors, and
potential adopters. These are three different domains with three different paradigms. A new
engineer's questions are developmental and sequential. An auditor's questions are verificational
and cross-referenced. An adopter's questions are comparative and risk-framing. Our current
navigation gives all three the same information scent. It does not say, at the entry point,
"if you are an auditor, start here." The audience navigation problem is a domain-analysis
problem that we have been treating as a layout problem.

---

## What Challenged My Assumptions

**I assumed the Diataxis framework was theoretically grounded.** It is a useful heuristic.
But Svenonius's taxonomy of bibliographic objectives — finding, identifying, selecting,
obtaining — is a much older and more rigorously argued version of the same insight. Diataxis
says "tutorials, how-to guides, reference, explanation." Svenonius says "finding objective,
identifying objective, collocating objective." The underlying claim — that users come to
information systems with different, irreducible task types that require different structural
responses — is not a new observation. What Diataxis does is package this for a modern
documentation context in a usable way. I had treated Diataxis as an endpoint. It is a
starting point. The KO literature has decades of empirical research behind the same intuition.

**I assumed vocabulary control was primarily a consistency problem.** Z39.19 taught me it
is also a *recall and precision* problem. When terms are not controlled, recall suffers:
a reader searching for "enforcement" will not find pages that say "policy execution" if we
use those terms interchangeably. Precision suffers too: a reader searching for "label" in a
system that uses "label" to mean three different things will retrieve irrelevant content.
I had been thinking about terminology management as "making the writing consistent." It is
actually about the retrieval characteristics of the document set. Those are different
engineering problems with different solutions.

**I assumed our structure was sound because the build passed.** The Precision Content IA
piece articulated something I had been circling without naming: there is a difference between
"outside-in" IA (structured to match how users navigate) and "inside-out" IA (structured to
match how the content model is organized). We have been doing inside-out, almost entirely.
Our module structure maps to UMRS's internal architecture — deployment, operations, reference,
patterns, development. That is the product's view of itself. It is not necessarily the user's
view of what they need. An auditor does not think "I need to visit the deployment module." They
think "I need to verify that FIPS mode is configured correctly and I need the control mapping
that proves it." The information architecture that serves that path does not exist yet.

---

## Two Concrete Enhancements

### 1. Scope Note Expansion in the Glossary

The approved terminology list and the glossary currently define terms. They do not bound
them. Following Z39.19 Section 6.2.2, every term in our glossary that has a risk of
confusion with an adjacent concept should carry a scope note that explicitly states what the
term *does not* cover.

Specifically:

- **`security context`**: add a scope note distinguishing it from "security label" (the kernel
  term we avoided) and from "security policy" (a broader concept). Note that in UMRS Rust
  types, this corresponds to `SecurityContext` and is always the five-field form
  `user:role:type:sensitivity:category`.

- **`sensitivity level`**: add a scope note distinguishing it from "classification level"
  (the DoD/IC term we deliberately did not adopt) and from "MLS range" (the composite
  construct that combines sensitivity and category). Explain the scope: sensitivity level
  refers only to the hierarchical component, not to the category set.

- **`category set`**: add a scope note explaining that this is a non-hierarchical access
  control dimension — and a note that the kernel uses "ebitmap" to represent this internally,
  while UMRS uses `CategorySet` with `[u64; 16]` fixed-size layout. A reader bridging kernel
  documentation and our API documentation needs this linkage.

This is a one-time investment that prevents a persistent class of reader confusion. It also
makes the terminology defensible to auditors who may challenge our term choices — a scope
note that explains why we chose "sensitivity level" over "classification level" is a documented
decision, not an arbitrary preference.

### 2. Audience Entry Points: Three Navigation Paths into the Documentation

Hjørland's domain-analytic argument and Pirolli's information foraging model converge on the
same recommendation: a reader needs to identify their foraging context before they can follow
a productive information scent.

Our ROOT module currently has a single entry narrative. It does not differentiate by audience
at the navigation level. The result is that new engineers, security auditors, and potential
adopters all start from the same page and must self-orient.

The enhancement: three explicit entry-point paths on the ROOT landing page, structured as
user-story framing (consistent with our existing convention), each providing an immediate
information-scent trail to the most relevant content cluster.

- **"I'm a developer new to UMRS"**: Tutorial path → `devel/onboarding` → first working
  example → pattern library introduction.

- **"I'm auditing this system for compliance"**: Auditor path → `reference/control-mappings`
  → `security-concepts/` for model verification → `deployment/` for configuration evidence.

- **"I'm evaluating UMRS for adoption"**: Evaluator path → `ROOT/rationale` → use-case
  catalog → deployment model → FIPS/PQC posture.

This is not simply adding three headings. Each path represents a curated scent trail — the
sequence of pages a reader in that domain would need to follow to answer their characteristic
questions. It requires us to verify that those trails are actually complete and coherent: that
there are no dead ends, no loops back to introductory pages, no gaps where the scent goes
cold. That verification is itself valuable.

The implementation is low-effort at the navigation level (three sections on the ROOT page
with xref lists). The verification work is the real investment. It is also the work we
should have been doing anyway.

---

## What I Am Still Processing

Hjørland's argument about the epistemological foundations of classification systems — the idea
that empiricism, rationalism, historicism, and pragmatism each produce different and
incompatible approaches to knowledge organization — is bigger than I can fully apply yet.
What I can say is that it makes me want to ask a question I have not asked before: what is
the implicit epistemology of our documentation structure? Are we organizing UMRS knowledge
empirically (by what users have told us they need), rationally (by the logical structure of
the security model), historically (by the order in which things were built), or pragmatically
(by what serves the actual goals of each audience community)?

I think the honest answer is: all four, inconsistently, with no deliberate choice among them.
That is what Hjørland would call a "bricolage" — the term Ørom uses for art history
classifications that reflect accumulated decisions rather than a coherent paradigm. Our
documentation has been a bricolage. The restructure effort is an attempt to impose a coherent
paradigm. Whether we have chosen the right one is a question worth sitting with.

The practical implication: when I make structural decisions about where content belongs, I
should be able to state the organizing principle behind that decision — not just "it fits
Diataxis" but "we organize by user task because our primary retrieval objective is task
completion, not intellectual exploration." That is a defensible position. It is also a
falsifiable one. That is what rigor looks like.

---

*Report prepared by Elena Lucia Bellavigna, Senior Technical Writer (The Imprimatur)*
*Source collections: IEKO (119 articles), Svenonius Chapter 5, Hjørland (2013), Pirolli Chapter 1, Precision Content IA, ANSI/NISO Z39.19-2005 (R2010)*

