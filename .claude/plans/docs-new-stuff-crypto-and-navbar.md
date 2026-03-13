# Plan: docs/new-stuff — Crypto and Navigation Processing

**Source documents**: `docs/new-stuff/crypto.md`, `docs/new-stuff/left-navbar.md`
**Prepared by**: senior-tech-writer
**Date**: 2026-03-13
**Status**: Ready for tech-writer execution (all questions resolved 2026-03-13)

---

## Part 1: crypto.md — Analysis and Disposition

### What the source document is

`docs/new-stuff/crypto.md` is a raw AI conversation transcript, not a finished document.
It contains:

- Two nearly complete iterations of the same 8-section cryptographic baseline document
  (a first pass, then a revised pass that adds PQC algorithms)
- Three conversational prompts from the AI offering further work
  ("If you'd like, I can also produce...", "Good idea. The recent NIST...", "If you want...")
- A clean Markdown third section formatted with `|—|` table separators (malformed AsciiDoc)

Strip all three conversational prompts before using any content.
Use only the **third, clean Markdown section** (starting at line 451) as the primary content
source — it is the most complete and correctly formatted iteration.
The earlier two sections are drafts of the same material and are superseded by it.

### Overlap assessment against existing pages

Four existing pages cover cryptographic content. The table below maps each section of
`crypto.md` against the page that already owns that content.

| crypto.md section | Existing page | Assessment |
|---|---|---|
| 1. Hash Functions | `fips-cryptography-cheat-sheet.adoc` | **Duplicate** — cheat sheet covers SHA-2 and SHA-3 families, with FIPS references and status marks. Cheat sheet is more complete (includes SHAKE, HMAC-SHA-1 restrictions, Ed25519). |
| 2. Symmetric Encryption | `fips-cryptography-cheat-sheet.adoc` | **Duplicate** — cheat sheet covers AES key sizes and 3DES deprecation. |
| 3. AES Cipher Modes | `fips-cryptography-cheat-sheet.adoc` + `crypto-policy-tiers.adoc` | **Duplicate** — both existing pages have AES mode tables. `crypto-policy-tiers.adoc` already has the 4-column tier format. |
| 4. MACs / HMAC | `fips-cryptography-cheat-sheet.adoc` + `crypto-policy-tiers.adoc` | **Duplicate** — cheat sheet adds KMAC128/KMAC256 and GMAC-AES which the source lacks. |
| 5. Digital Signatures | `fips-cryptography-cheat-sheet.adoc` + `crypto-policy-tiers.adoc` + `crypto-post-quantum.adoc` | **Partial duplicate** — classical algorithms covered; PQC variants (ML-DSA-44, ML-DSA-65, ML-DSA-87 with key/sig sizes) already in `crypto-post-quantum.adoc`. |
| 6. Key Agreement / KEM | `fips-cryptography-cheat-sheet.adoc` + `crypto-policy-tiers.adoc` + `crypto-post-quantum.adoc` | **Partial duplicate** — PQC (ML-KEM parameter sets with byte sizes) already in `crypto-post-quantum.adoc`. Classical coverage in cheat sheet and tiers page. |
| 7. KDFs | `crypto-policy-tiers.adoc` | **Duplicate** — tiers page has the same four algorithms (HKDF-SHA-384, HKDF-SHA-256, PBKDF2-SHA-256, NIST Counter KDF) with identical tier assignments. |
| 8. RNG / DRBG | `fips-cryptography-cheat-sheet.adoc` + `crypto-policy-tiers.adoc` | **Duplicate** — cheat sheet adds Dual_EC_DRBG explicit withdrawal which the source lacks. |
| Policy Levels (Preferred/Approved/Legacy/Disallowed) | `crypto-policy-tiers.adoc` | **Duplicate** — this is exactly what `crypto-policy-tiers.adoc` covers, with more precision (four tiers, per-category tables, control mapping). |
| Terminology Glossary | `glossary/pages/index.adoc` | **Already ingested** — Phase 3 work (2026-03-12) populated the glossary from this very source. All 13 terms in the crypto.md glossary were included. |

### Genuine new content in crypto.md

After stripping duplicates, three items in `crypto.md` are new or additive:

**1. Hash use cases expanded (minor)**
The source lists "audit log chaining" and "file verification" as hash function use cases.
The existing pages do not mention audit log chaining in the hash section.
This belongs as a NOTE or bullet addition to `fips-cryptography-cheat-sheet.adoc`
under the Hash Functions section — one line.

**2. KDF "Typical Use" column entries (minor)**
`crypto.md` provides brief use labels: "High-assurance key derivation", "TLS key schedule",
"Password-based keys", "Protocol key derivation". The existing `crypto-policy-tiers.adoc`
KDF table has a "Typical use" column but its entries are less descriptive.
This is a copy-edit enhancement to `crypto-policy-tiers.adoc`, not new content.

**3. Referenced standards preamble**
The source lists five standards references as an explicit preamble:
- FIPS 140-3 — Cryptographic Module Validation
- FIPS 203 — ML-KEM
- FIPS 204 — ML-DSA
- FIPS 205 — SLH-DSA
- NIST SP 800-131A — Algorithm transitions

`fips-cryptography-cheat-sheet.adoc` has a compliance references table at the bottom.
The cheat sheet already lists all five but does not have a preamble/intro that names them
upfront. A brief "Referenced standards" preamble would improve scannability.
This is a minor enhancement to `fips-cryptography-cheat-sheet.adoc`.

### What to strip entirely

Remove before any use:
- Lines 1–7 (AI framing: "For UMRS documentation, the most useful approach...")
- Lines 184–199 (conversational prompt + bridge: "If you'd like..." through "Good idea...")
- Lines 433–439 (second conversational prompt: "If you'd like, I can also produce...")
- Lines 655–662 (third conversational prompt: "If you want, I can also generate...")
- Lines 442–450 (formatting preamble for the clean Markdown section: "Below is a clean Markdown version...")
- The first two duplicate iterations (lines 1–440, everything before line 451)

### Disposition decisions

| Item | Decision | Target page | Action |
|---|---|---|---|
| All 8 algorithm tables (both iterations) | Duplicate — discard | N/A | Strip |
| Policy levels list (Preferred/Approved/Legacy/Disallowed) | Duplicate — discard | N/A | Strip |
| Terminology glossary (13 terms) | Already ingested in Phase 3 | `glossary/pages/index.adoc` | No action needed |
| AI conversational prompts | Strip — not documentation | N/A | Strip |
| Hash use cases: "audit log chaining", "file verification" | New — minor addition | `fips-cryptography-cheat-sheet.adoc` | Add to Hash Functions section |
| KDF "Typical Use" labels | Enhancement — minor | `crypto-policy-tiers.adoc` | Update KDF table Typical use column |
| Referenced standards preamble | Enhancement — minor | `fips-cryptography-cheat-sheet.adoc` | Add preamble after title |

### Net result

`crypto.md` contains almost no new content not already in the existing pages.
The existing pages are more complete and more precise than the source document.
The three minor enhancements above should be incorporated as targeted edits.
No new page is warranted.

---

## Part 2: left-navbar.md — Analysis and Proposed ROOT/nav.adoc Rewrite

### What Jamie's feedback establishes

1. The current ROOT/nav.adoc is flat and cluttered — top-level items mixed with module links,
   no grouping, no flow.
2. The vision: group items logically, move introductory content up, move AI transparency down,
   surface second-level items from module navs.
3. Jamie's proposed structure is a sketch, not a final spec. This plan refines it.

### Structural problems in the current nav

The current nav has two distinct layers collapsed into one flat list:
- **Site-level pages** (Home, Introduction, What is UMRS, etc.) that live in ROOT/pages/
- **Module entry points** (Architecture, Deployment, Development, etc.)

These are visually indistinguishable in the current nav. There are no grouping headers.
The result is that the navigation does not reflect any of the conceptual groupings
discussed in the doc-vision.

A secondary problem: "AI in This Project" appears between "Getting Started" and the modules —
it reads as if it belongs in the onboarding flow. Jamie's feedback moves it lower, which
is correct. It is background context for auditors, not an onboarding step.

### Principles for the rewrite

**Antora nav constraint**: ROOT/nav.adoc can only link to ROOT/pages/ or use cross-module
xrefs (e.g., `xref:architecture:index.adoc[]`). It cannot surface second-level items
from within module navs directly — those are controlled by the module's own nav.adoc.

**What "second level derived from module sections" means in practice**:
Jamie's intent is that what appears under each module entry in the ROOT nav should
mirror the section groupings that already exist in that module's nav.adoc.
In Antora, this is achieved by the module nav itself (which Antora renders as a
collapsible tree). The ROOT/nav.adoc only needs to list the module entry point;
the module nav handles everything below it.

**Implication**: The ROOT/nav.adoc rewrite is about grouping and ordering the
top-level items. The module navs already define their own second-level structure
and that structure is already good (see assessment below).

### Module nav second-level assessment

| Module | Current sections | Assessment |
|---|---|---|
| Architecture | Design Rationale, Security Model, MLS Label Model, Existing OS Technologies, Justification and Case Studies | Good structure. "Existing OS Technologies" could be renamed "Historical Context" to match Jamie's "Historical Stuff?" sketch, but this is an architecture/nav.adoc decision, not a ROOT change. |
| Security Concepts | (flat list: Security Model Overview, Reference monitor, NSA RTB, Integrity and provenance, Truth and trust) | No grouping headers. Could benefit from section grouping, but that is an architecture decision for the tech-writer, not part of this ROOT nav task. |
| Deployment | Platform Overview, Filesystem Layout, RHEL 10 (5 pages), Ubuntu, Assurance Enhancements (4 pages) | Good. "Assurance Enhancements" section matches Jamie's sketch. |
| Developer Guide | High-Assurance Patterns (5 pages), Platform Internals, Language Guides, Build & Workspace | Good. Jamie wants "Understanding the UMRS API" — this maps to Platform Internals + Language Guides, possibly needing a new section header. Flag for tech-writer. |
| High-Assurance Pattern Library | Architectural Patterns, Coding Techniques, Observability, Process, Deep Dives | Good. Already well-organized. |
| UMRS Tools | (tool pages + Prototype Crates + Tool Operations) | Minimal content. No grouping needed yet. |
| Operations | Operations, Troubleshooting, Signing and Chain of Custody, Integrity, Administration, Tools | Good. No changes needed. |
| Logging and Audit | (flat list, 6 pages) | No grouping headers. Acceptable given small size. |
| Reference | SELinux Context Model, MLS, MLS Display, CUI & Policy, Cryptographic Baseline | Good. Well-organized. |

### Proposed ROOT/nav.adoc

The proposed rewrite below is not a file edit — it is the target specification for the
tech-writer to implement. The structure follows Jamie's sketch, with these refinements:

1. "Pattern Library" is a module in its own right. In Jamie's sketch it appears under
   Development. However, the patterns module has its own nav and is linked from devel/nav.adoc
   already. Keeping it as a top-level entry in ROOT/nav.adoc preserves discoverability.
   Recommendation: keep as top-level, but position it adjacent to Development.

2. "Logging and Audit" is a functional area, not a sub-section of Operations. It stays
   top-level adjacent to Operations.

3. A visual blank line (empty `*` comment block) in AsciiDoc nav creates a section break.
   Antora renders these as visual separators between nav groups.

```asciidoc
* xref:index.adoc[Home]
* xref:getting-started.adoc[Getting Started]

* Introduction
** xref:introduction.adoc[Introduction]
** xref:what-is-high-assurance.adoc[What is High Assurance?]
** xref:scope-and-audience.adoc[Scope and Audience]

* xref:architecture:index.adoc[Architecture]
* xref:security-concepts:index.adoc[Security Concepts]

* xref:deployment:index.adoc[Deployment]

* xref:devel:index.adoc[Development]
* xref:patterns:index.adoc[High-Assurance Pattern Library]

* xref:umrs-tools:index.adoc[UMRS Tools]

* xref:operations:index.adoc[Operations]
* xref:logging-audit:index.adoc[Logging and Audit]

* xref:reference:index.adoc[Reference]
* xref:glossary:index.adoc[Glossary]

* xref:ai-transparency.adoc[AI in This Project]
* xref:legal-notices.adoc[Legal Notices]
* xref:release-notes.adoc[Release Notes]
```

### Rationale for each structural decision

**Home + Getting Started at top**: These are the entry points for all three audiences.
No barrier to access.

**Introduction group** (collapsible implied by Antora rendering): Groups the
"What is X?" explanatory pages that orient new readers before they dive into modules.
Avoids having five flat items that compete with module links.

Note on `introduction.adoc` vs `what-is-umrs.adoc`: **Jamie has decided to merge these.**
The content from `what-is-umrs.adoc` should be incorporated into `introduction.adoc`,
which becomes the single Introduction page. `what-is-umrs.adoc` is removed from the nav
but not deleted from disk.

**Architecture + Security Concepts together**: Related conceptual modules. Security Concepts
content is tightly coupled to Architecture (cross-module xrefs throughout).

**Deployment on its own**: Installation and configuration deserves clear vertical separation.

**Development + Pattern Library together**: Patterns are a developer resource. Keeping them
adjacent (but separate) follows Jamie's sketch while preserving the pattern module's
independent identity.

**UMRS Tools on its own**: Jamie's note — "purpose: security posture checks" — suggests this
is a distinct operational category, not a sub-section of Development or Operations.

**Operations + Logging and Audit together**: Both are runtime/post-deployment concerns.

**Reference + Glossary together**: Lookup resources.

**AI, Legal, Release Notes at bottom**: Background context and administrative pages.
These should not compete with functional navigation.

### Impact analysis: what changes when ROOT/nav.adoc is rewritten

**Pages moved within ROOT/nav.adoc** (no file moves, xref changes, or content edits required):
- `introduction.adoc` — moves from top-level flat item into Introduction group
- `what-is-umrs.adoc` — moves into Introduction group
- `what-is-high-assurance.adoc` — moves into Introduction group
- `scope-and-audience.adoc` — moves into Introduction group
- `ai-transparency.adoc` — moves from between Getting Started and module links to bottom

**No files move. No xrefs break.**
ROOT/nav.adoc entries are display-layer only. Moving an entry in nav.adoc does not affect
any xref in any page body.

**Pattern Library label change** (optional): Current label is "Pattern Library".
The proposed label is "High-Assurance Pattern Library" to match the module title as
it appears in `patterns/nav.adoc`. This is a cosmetic label change in ROOT/nav.adoc only.

### Second-level items from module navs (Jamie's note)

Jamie's note "beyond the top-level, the second level should be derived from the sections
in the modules" is already satisfied by Antora's collapsible nav behavior. Each module's
nav.adoc defines its own section structure. When a user expands a module link in the sidebar,
they see the module's sections. Nothing in ROOT/nav.adoc needs to change to make this work.

However, one module nav improvement is worth flagging for the tech-writer:

**Security Concepts nav** currently has no section grouping — it is a flat list.
Given the content (5 pages covering related but distinct topics), adding two groups would
help:
- "Foundations" — Security Model Overview, Reference monitor, NSA RTB and RAIN
- "Integrity and Trust" — Integrity and provenance, Truth and trust concepts

This is a `security-concepts/nav.adoc` edit, not a ROOT/nav.adoc edit.
It is a small improvement that the tech-writer can include as part of the navbar work.

---

## Part 3: Task Board Entries for the Tech-Writer

Two tasks should be created on the task board.

---

### Task CTW-NAV-1: Rewrite ROOT/nav.adoc — navigation restructure

**Priority**: High
**Depends on**: Nothing (ROOT pages all exist)

**Goal**: Rewrite `docs/modules/ROOT/nav.adoc` to match the structure defined in this plan
(Part 2, "Proposed ROOT/nav.adoc" section).

**Steps**:

1. Read the current `docs/modules/ROOT/nav.adoc`.
2. Read this plan (`.claude/plans/docs-new-stuff-crypto-and-navbar.md`) Part 2 in full.
3. Implement the proposed nav structure. The exact target is in the "Proposed ROOT/nav.adoc"
   section above. Do not deviate from it without flagging the reason.
4. Before implementing: verify that all pages referenced in the Introduction group exist:
   - `docs/modules/ROOT/pages/introduction.adoc` — confirm exists
   - `docs/modules/ROOT/pages/what-is-high-assurance.adoc` — confirm exists
   - `docs/modules/ROOT/pages/scope-and-audience.adoc` — confirm exists
5. **Merge `what-is-umrs.adoc` content into `introduction.adoc`**:
   - Read both pages. Incorporate the useful content from `what-is-umrs.adoc` into
     `introduction.adoc`. Do not delete `what-is-umrs.adoc` — just remove it from the nav.
   - The merged Introduction page should serve as the single "What is UMRS and why does it
     exist?" orientation page.
6. **Consolidate architecture history sections**: In `docs/modules/architecture/nav.adoc`,
   combine "Existing OS Technologies" and "Justification and Case Studies" into a single
   "Historical Context" section. All pages stay — only the section grouping header changes.
7. Optionally, improve `docs/modules/security-concepts/nav.adoc` by adding two section
   groupings: "Foundations" and "Integrity and Trust" (see Part 2 for grouping details).
   This is optional but recommended.
6. Run `make docs 2>&1` from the repo root. Zero errors required (two pre-existing errors
   in ubuntu.adoc are acceptable; no new errors).
7. Confirm the nav renders correctly by checking the build output — look for broken xref warnings.

**Files to edit**:
- `docs/modules/ROOT/nav.adoc` — primary target
- `docs/modules/ROOT/pages/introduction.adoc` — merge content from `what-is-umrs.adoc`
- `docs/modules/architecture/nav.adoc` — consolidate history sections
- `docs/modules/security-concepts/nav.adoc` — optional grouping improvement

**Do not**:
- Move any page files
- Change any xref in any page body
- Add new pages

---

### Task CTW-CRYPTO-1: Apply minor crypto enhancements from crypto.md source

**Priority**: Low
**Depends on**: Nothing

**Goal**: Apply three targeted enhancements to existing crypto reference pages.
No new pages. No large rewrites. Targeted line-level additions only.

**Enhancement 1 — fips-cryptography-cheat-sheet.adoc: add referenced standards preamble**

After the title/description/toc attributes block and the compliance annotation comments,
add a brief preamble before the Legend table. The preamble should name the five standards
this reference is aligned with:

```asciidoc
This reference is aligned with the following standards:

* FIPS 140-3 — Security Requirements for Cryptographic Modules
* FIPS 203 — ML-KEM Post-Quantum Key Encapsulation Mechanism
* FIPS 204 — ML-DSA Post-Quantum Digital Signature Algorithm
* FIPS 205 — SLH-DSA Stateless Hash-Based Digital Signature Algorithm
* NIST SP 800-131A Rev.2 — Transitioning the Use of Cryptographic Algorithms and Key Lengths
```

The existing intro paragraph ("This reference covers the approved algorithm landscape...")
already exists. Add the standards list after it, before the Legend table.

**Enhancement 2 — fips-cryptography-cheat-sheet.adoc: hash use cases NOTE**

In the Hash Functions section, add a NOTE admonition after the SHA-1 entry that adds
two use cases not currently mentioned:

```asciidoc
NOTE: Hash functions are also used for audit log chaining and file integrity verification.
SHA-512 is the recommended algorithm for both in high-assurance deployments.
```

**Enhancement 3 — crypto-policy-tiers.adoc: KDF Typical Use column improvement**

The KDF table currently has a "Typical use" column. Update the entries to be more descriptive:

| Algorithm | Current "Typical use" | Improved "Typical use" |
|---|---|---|
| HKDF-SHA-384 | Key material derivation | High-assurance key material derivation |
| HKDF-SHA-256 | Widely deployed | TLS key schedule, general derivation |
| PBKDF2-SHA-256 | Password-based key derivation | Password-based key derivation |
| NIST Counter KDF | Approved KDF for DRBG-derived keys | Protocol key derivation (NIST SP 800-108) |

**Verification**:

1. Run `make docs 2>&1` from the repo root after each edit.
2. Zero new errors. Two pre-existing errors in ubuntu.adoc are acceptable.

**Files to edit**:
- `docs/modules/reference/pages/fips-cryptography-cheat-sheet.adoc`
- `docs/modules/reference/pages/crypto-policy-tiers.adoc`

**Do not**:
- Create new pages
- Rewrite existing sections
- Change any table structure beyond the KDF "Typical use" column entries
- Touch `crypto-post-quantum.adoc` or `key-recommendation-list.adoc`

---

## Part 4: Post-Processing (After Tech-Writer Completes)

After both tasks are verified complete and `make docs` passes:

1. Move `docs/new-stuff/crypto.md` to `docs/new-stuff/used/crypto.md`
2. Move `docs/new-stuff/left-navbar.md` to `docs/new-stuff/used/left-navbar.md`

The senior-tech-writer will handle the file moves after confirming the work is done.

---

## Decisions from Jamie (2026-03-13)

**Q1 — RESOLVED: Merge both pages into Introduction.**
Jamie's direction: `introduction.adoc` and `what-is-umrs.adoc` should be merged into a
single "Introduction" page. Getting Started comes before Introduction and serves as a
routing page — gets people's attention or sends them to a specific section depending on
who they are.

Impact on the proposed nav: The Introduction group simplifies to:
```asciidoc
* Introduction
** xref:introduction.adoc[Introduction]
** xref:what-is-high-assurance.adoc[What is High Assurance?]
** xref:scope-and-audience.adoc[Scope and Audience]
```
The tech-writer must also merge the content of `what-is-umrs.adoc` into `introduction.adoc`
and remove `what-is-umrs.adoc` from the nav. The file itself stays (do not delete docs),
but its content should be incorporated into the merged Introduction page.

**Q2 — RESOLVED: Consolidate into "Historical Context".**
Jamie approves combining "Existing OS Technologies" and "Justification and Case Studies"
into a single "Historical Context" section in `architecture/nav.adoc`. This is iterative —
as more historical context is added, the grouping can be revisited.

**Q3 — RESOLVED: Not a new page. It is a crate orientation section.**
Jamie's intent is NOT a separate API page. It is about orienting developers to the
available UMRS library resources: the major crates (`umrs-platform`, `umrs-selinux`,
`umrs-apparmor` (future), `umrs-core`) and what each provides. This belongs as a section
or subsection within the existing Developer Guide, not a standalone page. The tech-writer
should add a "UMRS Crate Overview" or similar section header in `devel/nav.adoc` that
groups the crate-level orientation content. Content can be developed iteratively.
