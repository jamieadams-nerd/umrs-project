= Librarian Acquisition Brief: Henri Corpus
:doctype: article
:toc:
:toclevels: 2
:icons: font

== Mission

Acquire, process, and integrate the training corpus for the Henri
agent (Canadian Government Specialist). Henri requires authoritative
Canadian federal policy documents, bilingual terminology resources,
and cybersecurity guidance from the Canadian Centre for Cyber
Security. All documents should be acquired in both English and
French where available — the bilingual pairing is itself training
material for Henri's regulatory terminology mapping function.

Store processed corpus under:
`.claude/agent-memory/henri/corpus/`

Index all acquisitions in:
`.claude/agent-memory/henri/corpus-index.md`

== Priority 1 — Treasury Board Secretariat Policy
_(Acquire first. These are Henri's foundational policy instruments.)_

=== Directive on Security Management (2019)
* URL: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32611
* French: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32611
* Why: Defines Protected A/B/C designations, handling requirements,
  and departmental obligations. Henri's equivalent of NIST SP 800-53.
* Format: Fetch both language versions. Store as
  `tbs-directive-security-mgmt-en.md` and
  `tbs-directive-security-mgmt-fr.md`

=== Standard on Security Categorization
* URL: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=32614
* French: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=32614
* Why: Defines how Canadian government information is categorized.
  Direct equivalent of the CUI categorization process. Maps to
  UMRS labeling decisions.
* Format: Both languages. Store as
  `tbs-standard-security-categorization-en.md` and
  `tbs-standard-security-categorization-fr.md`

=== Policy on Government Security
* URL: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578
* French: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=16578
* Why: Parent policy above the Directive. Establishes the framework
  within which all Canadian government security decisions are made.
* Format: Both languages.

=== Directive on Privacy Practices
* URL: https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=18309
* French: https://www.tbs-sct.canada.ca/pol/doc-fra.aspx?id=18309
* Why: Privacy Act obligations that affect information handling
  and release. Essential for ATIP context.
* Format: Both languages.

== Priority 2 — Canadian Centre for Cyber Security (CCCS)
_(Acquire second. Technical security baseline Henri needs to
validate UMRS alignment.)_

=== CCCS Guidance Publications Index
* URL: https://www.cyber.gc.ca/en/guidance
* French: https://www.cyber.gc.ca/fr/orientation
* Action: Retrieve the publications index. Identify and acquire
  the following specific documents:

==== ITSP.10.222 — Network Security Zones
* Search: "ITSP.10.222" on https://www.cyber.gc.ca/en/guidance
* Why: Canadian government network security zone baseline.
  Context for how Canadian systems are architected around
  security boundaries — directly relevant to UMRS MLS zones.
* Format: Both languages where available.

==== PBMM Profile (Protected B Medium Integrity Medium Availability)
* Search: "PBMM" or "Protected B cloud profile" on CCCS site
* Why: The Canadian government cloud security baseline. If UMRS
  is deployed in a Canadian government cloud context this is
  the required baseline. Henri needs this to validate alignment.
* Note: PBMM is periodically updated. Acquire the current version
  and record the retrieval date in the corpus index.
* Format: Both languages where available.

==== CSE/CCCS Cyber Security Guidance for IT Practitioners
* URL: https://www.cse-cst.gc.ca/en/information-and-tools/tools
* Why: Establishes the technical security guidance authority
  Henri references alongside TBS policy.
* Action: Retrieve the tools and guidance index. Acquire any
  documents relevant to information classification, handling,
  or system security.

== Priority 3 — Legislation
_(Acquire third. Legislative basis for Henri's policy authority.)_

=== Access to Information Act
* URL: https://laws-lois.justice.gc.ca/eng/acts/A-1/
* French: https://laws-lois.justice.gc.ca/fra/lois/A-1/
* Why: Canadian FOIA equivalent. Henri needs to understand what
  triggers ATIP obligations and how information classification
  affects releasability.
* Format: Both languages. Current consolidated version.
* Store as: `atia-en.md` and `atia-fr.md`

=== Privacy Act
* URL: https://laws-lois.justice.gc.ca/eng/acts/P-21/
* French: https://laws-lois.justice.gc.ca/fra/lois/P-21/
* Why: Governs personal information held by federal institutions.
  Intersects with Protected A designation for personal information.
* Format: Both languages. Current consolidated version.

=== Official Languages Act
* URL: https://laws-lois.justice.gc.ca/eng/acts/O-3.01/
* French: https://laws-lois.justice.gc.ca/fra/lois/O-3.01/
* Why: Legislative basis for bilingual documentation requirements.
  Henri needs to understand when bilingual output is legally
  required versus recommended.
* Format: Both languages.

=== Charte de la langue française (Quebec — Loi 101)
* URL: https://www.legisquebec.gouv.qc.ca/en/document/cs/C-11
* French: https://www.legisquebec.gouv.qc.ca/fr/document/lc/C-11
* Why: Legislative basis for OQLF authority. Henri must understand
  why the OQLF GDT carries legal weight in Quebec contexts —
  it is backed by legislation, not merely style preference.
  This is why federal Termium Plus and OQLF GDT can produce
  findings when they diverge.
* Format: Both languages.

== Priority 4 — Five Eyes / NATO Context
_(Acquire fourth. Interoperability framework Henri uses to flag
Canadian/US divergences.)_

=== Canadian government unclassified Five Eyes documentation
* Search for any publicly available TBS or CCCS guidance on
  information sharing with allied partners
* Key divergence to document for Henri:
  - US: CUI framework (NARA registry, 32 CFR Part 2002)
  - Canada: Protected A/B/C (TBS Directive on Security Management)
  - Approximate mapping: CUI Basic ≈ Protected A/B;
    CUI Specified ≈ Protected B/C depending on category
  - Henri's job: flag where this mapping breaks down
* Note: Do not acquire classified material. Henri operates
  entirely in the unclassified space.

=== NATO STANAG 2014 — Marking of Classified Military Documents
* Search: "STANAG 2014" publicly available version
* Why: Canada is a NATO member. These markings appear on shared
  Five Eyes/NATO material Henri may need to validate against.
* Acquire unclassified public version only.

== Priority 5 — Supplementary Canadian Cyber Context

=== Canadian Cybersecurity Strategy (2018)
* URL: https://www.publicsafety.gc.ca/cnt/rsrcs/pblctns/ntnl-cbr-scrt-strtg/index-en.aspx
* French: https://www.publicsafety.gc.ca/cnt/rsrcs/pblctns/ntnl-cbr-scrt-strtg/index-fr.aspx
* Why: Strategic context within which CCCS guidance sits.
  Gives Henri the policy rationale behind technical requirements.

=== Communications Security Establishment (CSE) Public Guidance
* URL: https://www.cse-cst.gc.ca/en/information-and-tools
* Why: CSE is the Canadian signals intelligence and information
  assurance authority — equivalent function to NSA for Canadian
  government systems. Public guidance documents establish
  standards Henri needs to recognize.
* Action: Retrieve the public guidance index and acquire
  any documents relevant to information protection and
  system security.

== Processing Instructions

=== Bilingual Pairing
For every document acquired in both languages, store the pair
together in the corpus index with explicit cross-references.
The bilingual pairing is training material — Henri learns how
TBS renders its own policy terminology in both official languages.
That is the authoritative source for regulatory term mapping.

=== Corpus Index Entry Format
For each document acquired, create an entry in
`.claude/agent-memory/henri/corpus-index.md`:

```markdown
## [Document Title]
- **Authority:** [TBS / CCCS / CSE / Justice Canada / etc.]
- **Retrieved:** [YYYY-MM-DD]
- **English:** [filename-en.md]
- **French:** [filename-fr.md]
- **Relevance:** [one sentence — what Henri uses this for]
- **Key sections:** [section numbers or headings most relevant
  to Henri's work]
```

=== Terminology Extraction
After acquiring TBS policy documents, extract all defined terms
and their French equivalents into a supplementary TSV:
`henri-tbs-terms-fr_CA.tsv`

Format:
```
english_term[TAB]french_term[TAB]source[TAB]authority_level
```

This supplements Simone's Termium Plus and OQLF GDT TSVs with
policy-specific terminology that may not appear in the general
terminology databases.

=== Conflict Flagging
If any acquired document uses terminology that conflicts with
Simone's existing Termium Plus or OQLF GDT entries, flag the
conflict in the corpus index with both terms and their sources.
These conflicts are Henri's working material — they are the
cases where his judgment matters most.

== Reconciliation Task

After acquisition, produce a one-page summary for Henri:

. Total documents acquired by priority tier
. Any documents unavailable or requiring alternative sources
. Key terminology conflicts identified during processing
. Recommended additions based on what you found during acquisition
  (the Librarian's judgment — flag anything that looks important
  that was not on this list)

File the summary at:
`.claude/agent-memory/henri/corpus-acquisition-summary.md`

== Notes

* All TBS policy documents are publicly available in both official
  languages at tbs-sct.canada.ca. Use `web_fetch` directly on
  the English and French URLs provided.
* Justice Canada legislation is available at laws-lois.justice.gc.ca
  in both languages. Always acquire the current consolidated version.
* CCCS documents may require navigating the guidance index to find
  current versions. Record the exact URL and retrieval date for
  every document — CCCS updates guidance periodically.
* Rate limit requests to government sites. One request per second
  minimum. These are public servants' servers, not CDNs.

_Brief authored for The Librarian by The Hand.
Cross-reference Henri's corpus with Simone's existing corpus
before filing the acquisition summary. Note overlaps and gaps._
