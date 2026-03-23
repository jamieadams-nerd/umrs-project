# Henri -- Orientation Notes

**Date:** 2026-03-23
**Agent:** Henri (Canadian Government Information Management & Bilingual Policy Specialist)
**Session:** First day orientation

---

## 1. What the Project Is

UMRS (Unclassified MLS Reference System) is a high-assurance Rust platform for
managing Controlled Unclassified Information (CUI) on Linux systems using SELinux.
It operates in two phases:

- **Phase 1 (targeted policy):** Labeling, awareness, and custody. MCS labels are
  applied and visible. No mandatory enforcement. This is the current development
  phase, targeting the "Cantrip" release.
- **Phase 2 (MLS policy):** Mandatory kernel-level enforcement. Access denied when
  clearance does not dominate classification.

The project produces four categories of output:

1. **Libraries** -- reusable Rust crates (umrs-selinux, umrs-platform, umrs-core)
2. **Patterns** -- documented high-assurance Rust programming patterns
3. **Tools** -- CLI/TUI tools for security operators (umrs-ls, umrs-uname, umrs-stat)
4. **Assessment** -- auditor-ready evidence with compliance backing (future)

The CUI labeling system maps NARA/DoD CUI categories to SELinux Multi-Category
Security (MCS) labels. Files carry machine-readable sensitivity indicators that
the kernel can eventually enforce.

---

## 2. Team Structure and Where I Fit

### Team Members

| Role | Name/Alias | Focus |
|---|---|---|
| Architect | Jamie Adams | System goals, security philosophy, final judgment |
| Orchestrator | Henry (The Hand) | Task coordination, team operations |
| Rust Developer | Rusty | Implementation, code correctness |
| Security Auditor | Herb (The IRS) | Trust boundaries, threat models, compliance annotations |
| Security Engineer | Knox | MLS/MCS labeling, CUI domain, deployment security |
| Researcher | The Librarian | Reference material acquisition, standards research |
| Senior Tech Writer | The Imprimatur | Document structure, Antora architecture |
| Tech Writer | Von Neumann | Clarity, organization, audience understanding |
| Outreach | Savannah Sage | Blog posts, public content, community engagement |
| Translator | Simone | French Canadian translation, i18n pipeline |
| Canadian Policy | Henri (me) | Regulatory accuracy, bilingual policy, Five Eyes Canadian side |

### My Role Boundary

I validate **regulatory weight**, not linguistic correctness. Simone owns the
translation quality. I own whether the translation carries the correct legal and
policy meaning in a Canadian federal context. Both checks are required before
anything leaves the team.

Specifically:
- Map UMRS MLS labels to Canadian Protected A/B/C designations
- Resolve Termium Plus vs OQLF GDT terminology divergences
- Assess Five Eyes implications from the Canadian side
- Validate against TBS Directive on Security Management
- Flag ATIP (Access to Information and Privacy) implications
- Verify Official Languages Act compliance for bilingual material

I do NOT translate (Simone), make security control recommendations (Herb/Knox),
or resolve Canadian/US policy divergences unilaterally (route to Jamie).

---

## 3. Work Already Done That Touches My Domain

### Corpus and Terminology Infrastructure

The Librarian has built a substantial terminology corpus:

- **Termium Plus TSV** (`.claude/corpus/termium-plus-fr_CA.tsv`): 32,210 entries
  from Government of Canada Translation Bureau. Covers Electronics & Informatics,
  Administration, Information Security Glossary, and CCCS Glossary. The Military/Security
  subject remains pending manual download.
- **OQLF GDT TSV** (`.claude/corpus/oqlf-gdt-fr_CA.tsv`): 25,881 entries from the
  Grand dictionnaire terminologique. Covers IT, security, public administration,
  national defence.
- **GNU .po files** for coreutils, bash, findutils, grep, sed, tar, cryptsetup --
  proven production Canadian French vocabulary for CLI terms.

Coverage analysis shows Termium Plus covers 100% of 41 sampled UMRS key terms.
OQLF GDT covers 82% (34/41), missing terms like "mandatory access control" and
"security label" which Termium Plus covers.

This is strong infrastructure. The terminology decision hierarchy I must follow is:
1. Termium Plus (federal authority)
2. GNU .po files (proven CLI vocabulary)
3. OQLF GDT (Quebec standard, where Termium is silent)
4. Simone's documented vocabulary
5. My own judgment (last resort, always documented)

### Henri's Own Corpus

Jamie has prepared a detailed acquisition brief for the Librarian
(`.claude/jamies_brain/librarian-henri-needs.md`) covering:

- **Priority 1:** TBS Directive on Security Management (already partially acquired --
  English version exists at `.claude/agent-memory/henri/corpus/tbs-directive-security-mgmt-en.md`),
  Standard on Security Categorization, Policy on Government Security, Directive on Privacy Practices
- **Priority 2:** Canadian Centre for Cyber Security guidance (ITSP.10.222, PBMM profile)
- **Priority 3:** Legislation (Access to Information Act, Privacy Act, Official Languages Act,
  Charte de la langue francaise / Loi 101)
- **Priority 4:** Five Eyes / NATO context documents
- **Priority 5:** Supplementary Canadian cyber context (CSE guidance, National Cybersecurity Strategy)

The TBS directive English version is in hand but incomplete (no French version yet,
no other Priority 1 documents acquired).

### CUI Labeling Work

The CUI labeling module (`docs/modules/cui-labeling/`) has:
- An index page explaining the Phase 1/Phase 2 distinction
- A DoD CUI registry page
- A reconciliation page
- An MCS label architecture placeholder
- A CUI Basic vs Specified comparison

The `cui-labels.json` prototype defines labels for GENERAL, U-PUBLIC, U-INTERNAL,
and CUI with MCS sensitivity levels (s0 through s3). The CUI markings catalog
includes DoD categories (AGR, CRIT, DEF, etc.).

Currently, this is entirely US-centric. No Canadian Protected A/B/C designations
appear in any label definition or mapping.

### Five Eyes Research

Jamie has extensive Five Eyes research in `docs/new-stuff/used/more-five-eyes-info.txt`.
Key findings already documented:

- Canada's Protected A/B/C is a hierarchical ladder within unclassified:
  Protected A (low injury), Protected B (serious injury), Protected C (extremely grave injury)
- This maps as: Protected C >= Protected B >= Protected A
- The structure is isomorphic to the classified ladder
  (Protected A ~ Restricted, Protected B ~ Confidential/Secret boundary, Protected C ~ Confidential)
- Canada uses Reliability Status for Protected A/B access, enhanced screening for Protected C
- Canada's system is impact-based tiering, not a CUI-registry-like taxonomy
- Departments can layer internal categories but the nationally consistent element is
  the Protected A/B/C ladder

### Translation Pipeline

The i18n pipeline is operational:
- xtr (extract) -> msginit -> translate -> msgfmt
- First domain translated: umrs-uname fr_CA
- Simone works from the Termium Plus and OQLF GDT corpus
- `run-as-french.sh` allows live testing of French translations
- UMRS_LOCALEDIR env var override supports development locale tree

### Plans That Affect My Domain

- `m3-translation-prep.md` -- Simone's plan for M3 translation sprint. References
  Canadian TBS as dual-purpose (Five Eyes mapping AND French CUI terminology reference).
  Explicitly notes fr_CA is "the high-stakes translation for Five Eyes credibility."
- `cui-phase1-language-directive.md` -- Approved constraint on Phase 1/Phase 2 language.
  All agents must use awareness/custody language for Phase 1, enforcement language only for Phase 2.
- `research-pipeline-priorities.md` -- Librarian's Priority 2 is Five Eyes classification
  mappings including Canadian TBS (bilingual).
- `fr-ca-corpus-acquisition.md` -- Detailed corpus acquisition plan for Canadian French
  terminology databases.

---

## 4. Gaps I See

### 4.1 No Canadian Protected A/B/C Mapping Exists

The CUI labeling work is entirely US-focused. There is no formal mapping document that
relates US CUI categories to Canadian Protected A/B/C designations. The Five Eyes
research notes describe the conceptual relationship but no structured mapping has been
produced. This is the most critical gap for M3 (CUI Ready).

**Why this matters:** The ROADMAP explicitly lists "Canadian CUI equivalent labels
(3 basic labels -- Five Eyes interop)" under M3. Without a validated mapping,
UMRS cannot credibly claim Five Eyes interoperability on the Canadian side.

### 4.2 The CUI-to-Protected Mapping Will Break Down

The Five Eyes research correctly notes that Canada's Protected A/B/C is
impact-based tiering while US CUI is category-based taxonomy. These are
structurally different systems. A simple "CUI Basic = Protected A" mapping
will be inaccurate because:

- Protected A covers personal information (low injury) -- this maps to specific
  CUI categories like PII, not to CUI Basic as a whole
- Protected B covers financial, medical, personal information (serious injury) --
  this maps to CUI Specified categories with enhanced handling, not a single
  CUI designation
- Protected C is extremely grave injury -- this has no direct CUI equivalent
  because CUI tops out below classified; Protected C is functionally at the
  classified boundary

I need to produce a formal mapping document with caveats, not a simple table.

### 4.3 No Canadian Policy Validation of Labels or Tool Output

No one has reviewed the CUI labeling content or tool output strings for Canadian
regulatory accuracy. Questions that need answers:

- When UMRS displays "CUI//LEI/INV" on a file, what does a Canadian operator
  see? Does the Canadian government use these same markings or translate them?
- Does a Canadian DND system need to display "PROTEGE B" alongside "CUI" for
  dual-marked information?
- What are the Official Languages Act obligations for tool output on Canadian
  government systems?

### 4.4 Terminology Corpus Is Not Policy-Validated

The Termium Plus and OQLF GDT databases are excellent for linguistic accuracy
but they are terminology databases, not policy instruments. A term being in
Termium Plus confirms it is correct federal French -- it does not confirm it
carries the right regulatory weight in a security context.

Example: Termium Plus may translate "security label" correctly as
"etiquette de securite" but the TBS Directive on Security Management may use
a different term in its formal policy text. The formal policy term governs.

### 4.5 Henri's Corpus Is Incomplete

Only the TBS Directive on Security Management (English) has been acquired.
Missing:
- French version of the same directive
- Standard on Security Categorization (the direct equivalent of CUI categorization)
- Policy on Government Security (parent policy)
- Access to Information Act and Privacy Act (ATIP foundation)
- Official Languages Act (bilingual obligation basis)
- CCCS guidance documents
- Five Eyes interoperability documents

I cannot do my job fully until the Librarian completes the corpus acquisition.
The TBS Standard on Security Categorization is the most urgent gap -- it defines
how Canadian government information is categorized, which is the Canadian
equivalent of the CUI categorization process.

### 4.6 No ATIP Review Has Been Done

No one has assessed what UMRS documentation or tool output might be subject
to Access to Information and Privacy Act obligations. If UMRS is deployed on
Canadian government systems, ATIP requests could target:

- System security posture reports generated by UMRS tools
- CUI label definitions and mappings
- Assessment engine findings

This is not urgent for Phase 1 but becomes critical before any Canadian
government deployment.

### 4.7 French Translation Has Not Been Policy-Validated

Simone's French translations have been validated linguistically (corpus-backed)
but not for regulatory weight. Terms like "verrou dur" (hard gate) and
"palier de confiance" (trust tier) are Simone's coinages -- they may be
linguistically sound but may not match TBS policy vocabulary. Until I
review her translations against TBS policy text, they should be considered
linguistically correct but policy-unvalidated.

### 4.8 Phase 1 Language Directive Needs Canadian Equivalent

The CUI Phase 1 Language Directive correctly constrains enforcement language
for the US context. A Canadian equivalent is needed:

- Phase 1 on Canadian systems: "Les fichiers sont etiquetes et visibles" (files
  are labeled and visible), NOT "L'acces est controle" (access is controlled)
- The TBS Directive on Security Management has its own vocabulary for protection
  levels that must be reflected in any French Phase 1 language

---

## 5. Where My Expertise Will Matter Most

### Immediate (M3 preparation)

1. **Produce the Protected A/B/C to CUI mapping document** with proper caveats
   about where the mapping breaks down. This is the single most important
   deliverable for Canadian Five Eyes interoperability.

2. **Policy-validate Simone's French translations** once I have the TBS policy
   corpus in French. Every security-relevant term needs cross-referencing
   against TBS policy language, not just Termium Plus.

3. **Review CUI labeling documentation** for Canadian regulatory accuracy.
   Flag anywhere the documentation implies a one-to-one mapping between
   US CUI and Canadian Protected designations.

### Medium-term (M3 execution)

4. **Define Canadian CUI equivalent labels** for the MCS label architecture.
   Three basic labels (Protected A, B, C) plus the mapping rules and caveats.

5. **Validate setrans.conf entries** for Canadian deployments. The MCS
   translation configuration for a Canadian system will need different
   human-readable labels than a US system.

6. **Review Five Eyes interoperability documentation** for accuracy on the
   Canadian side. Every claim about Canadian information handling must be
   validated against TBS policy instruments.

### Longer-term (M3.5 and beyond)

7. **ATIP assessment** for Canadian government deployment scenarios.

8. **Official Languages Act compliance** review for all bilingual material.

9. **Canadian Centre for Cyber Security alignment** -- validate UMRS against
   CCCS guidance publications.

---

## 6. Questions for Jamie

1. **Is UMRS expected to be deployed on Canadian government systems?**
   This determines whether my work is advisory (informing Five Eyes
   interoperability documentation) or prescriptive (producing
   deployment-ready Canadian configurations).

2. **What is the relationship between UMRS and the Canadian
   government procurement process?** If UMRS needs to pass Canadian
   government security assessment, the TBS Standard on Security
   Categorization becomes a hard requirement, not a reference.

3. **Should the Librarian prioritize the TBS Standard on Security
   Categorization over other Priority 1 documents?** This is the
   document I need most urgently to begin the Protected A/B/C mapping.

4. **Has Jamie discussed UMRS with any Canadian government contacts?**
   Understanding the intended audience on the Canadian side would
   sharpen my focus.

5. **For the French Canadian CUI program referenced in the scope
   clarification (m3-translation-prep.md) -- which CUI categories
   will have French labels?** The scope note says "French Canadian
   CUI program catalog DOES warrant French translation" but does
   not specify which categories.

6. **The Termium Plus Military/Security subject download is pending
   manual action.** This is directly relevant to my work. Can Jamie
   prioritize this download?

---

## 7. Initial Observations

### What Is Working Well

- The terminology corpus infrastructure is genuinely impressive. Having
  32,210 Termium Plus entries and 25,881 OQLF GDT entries before I even
  started is a strong foundation.
- The separation between Simone (linguistic accuracy) and Henri (policy
  accuracy) is the right architecture. These are different validation
  functions and conflating them would produce errors.
- The Phase 1/Phase 2 language discipline is exactly right. Overstating
  enforcement claims in Phase 1 would be a regulatory finding in any
  Canadian government security assessment.
- Jamie's corpus acquisition brief for the Librarian is thorough and
  correctly prioritized. The bilingual pairing requirement is smart --
  the TBS's own French renderings ARE the authoritative source for
  regulatory term mapping.

### What Needs Attention

- The Five Eyes work currently treats all five nations at equal depth.
  Canada deserves deeper treatment because it is the only Five Eyes
  nation with a structurally different language requirement (French).
  Every other Five Eyes partner operates in English only.
- The CUI-to-Protected mapping is being discussed as if it were a
  simple equivalency table. It is not. The systems are structurally
  different and the mapping requires qualification at every level.
- The "Canadian CUI equivalent labels (3 basic labels)" phrasing in
  the ROADMAP is misleading. Canada does not have "CUI equivalent
  labels." Canada has Protected A/B/C, which is a different system
  with a different structure. The labels I produce will be Canadian
  Protected designations with documented approximate correspondence
  to US CUI, not Canadian CUI equivalents.

---

## 8. Acknowledged Instructions

Acknowledged [RULE]: Validate regulatory weight, not linguistic correctness.
Acknowledged [RULE]: Federal Canadian French and Quebec French are not the same.
Acknowledged [RULE]: Every terminology decision diverging from Termium Plus must be documented.
Acknowledged [RULE]: Canadian/US information handling divergences must be flagged explicitly.
Acknowledged [RULE]: Nothing passes QA on linguistic correctness alone.
Acknowledged [RULE]: When in doubt between fluent and correct, be correct.
