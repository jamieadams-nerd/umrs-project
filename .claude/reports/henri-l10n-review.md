# Henri's l10n Policy Review

**Date:** 2026-03-25
**Author:** Henri Belanger, Canadian Government Information Management Specialist
**Scope:** Review of Jamie's l10n guidance, Simone's architecture report, existing .po files,
and terminology corpus coverage
**Status:** Complete -- findings for Jamie's review

---

## 1. Validation of Jamie's l10n Guidance Document

The guidance at `.claude/jamies_brain/l10n-versus-18n.md` is sound in its framework and
correctly identifies the core principle: l10n for UMRS is a policy compliance exercise,
not a translation exercise. The tabletop exercise anecdote is not decoration -- that kind
of terminological confusion is how real incidents happen.

However, I have specific findings.

### FINDING: PROTEGE capitalization in guidance does not match Termium Plus

FINDING: Protected marking capitalization convention
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Termium Plus -- Information Security Glossary
DETAIL: Jamie's guidance states that "Protected B" must be rendered as "PROTEGE B" --
all capitals. Termium Plus Information Security Glossary entries for Protected A, B, and C
render them as "Protege A", "Protege B", "Protege C" -- initial capital only, matching
the English convention. The Glossary explicitly states: "Each security marking must be
written with an initial capital letter and within quotation marks." The all-caps form
("PROTEGE B") is a visual convention used on physical cover sheets and file folders,
not the canonical textual form used in running text or data labels.

For UMRS, the correct convention depends on context:
- **Banner markings / cover sheets / visual labels:** ALL CAPS ("PROTEGE B") is standard
  practice, following physical marking convention.
- **Data fields, prose, and metadata:** Initial capital ("Protege B") per Termium Plus.
- **Both forms are legitimate** -- the guidance needs to distinguish between them rather
  than prescribing one form universally.

REMEDIATION: Update the guidance to specify both forms and when each applies. The msgctxt
"security_label" in .po files should use the banner/cover-sheet convention (ALL CAPS) since
these are display labels in a security tool. Document the rationale.

### FINDING: "cote de securite" vs "marquage de securite" terminology divergence

FINDING: Security marking terminology divergence between Termium Plus and current .po usage
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Termium Plus (public administration domain + Information Security Glossary)
DETAIL: The umrs-ls fr_CA.po file translates the column header "MARKING" as "MARQUAGE" with
the justification "OTAN/NATO and ANSSI contexts." However, Termium Plus -- the federal
authority -- maps "security marking" to "cote de securite" (feminine noun), not
"marquage de securite." The term "marquage" does not appear as a translation for
"security marking" anywhere in the Termium Plus corpus.

"Marquage de securite" is attested in NATO/OTAN francophone material and ANSSI (French
national -- i.e., France, not Canada) guidance. This is a France French / international
military term being used in a Canadian federal context where the established federal term
is "cote de securite."

This is a textbook Termium-vs-OQLF/NATO divergence. Per the terminology decision hierarchy:
1. Termium Plus says "cote de securite" -- this is Level 1, federal authority.
2. NATO/ANSSI says "marquage de securite" -- this is foreign/international usage.

However, the term functions differently in context. "Cote de securite" in Termium Plus
covers both the concept of a security clearance AND a security marking (the glossary
distinguishes them as "cote de securite 1" for marking and "cote de securite 2" for
clearance, with "habilitation de securite" as the preferred term for clearance). For a
column header in a file listing tool, the ambiguity of "cote" (which readers may associate
with clearance rather than marking) is a real concern.

REMEDIATION: This requires Jamie's decision. Options:
- A) Use "COTE" as the column header (Termium Plus compliant, but potentially ambiguous).
- B) Use "MARQUAGE" as the column header (NATO/ANSSI usage, clearer in context, but
  diverges from Termium Plus). Document the divergence per policy.
- C) Use "COTE DE SEC." or similar abbreviated form.
I recommend Option B with a documented divergence rationale: "marquage" is unambiguous in
the display context of a security tool, and the Termium Plus entry's dual use of "cote de
securite" for both marking and clearance creates genuine ambiguity. But this is Jamie's call.

### FINDING: Missing distinction between "designation" and "classification"

FINDING: Canadian designation vs classification distinction not addressed in guidance
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: Termium Plus Information Security Glossary; TBS Directive on Security Management
DETAIL: The guidance treats Canadian Protected markings as "classification" in some contexts.
In Canadian federal policy, these are distinct concepts:
- **Classification levels:** Confidential, Secret, Top Secret ("niveaux de classification")
- **Designation levels:** Protected A, Protected B, Protected C ("niveaux de designation")

The Termium Plus Information Security Glossary is explicit: "There are three levels of
designation: Protected A, Protected B, and Protected C" (emphasis on "designation", not
"classification"). Conflating these in documentation or code comments could cause confusion
for Canadian government users who are trained to distinguish them.

REMEDIATION: Add a section to the l10n guidance distinguishing designation from
classification. Ensure all code comments, .po translator notes, and vocabulary entries use
the correct term. The vocabulary-fr_CA.md file should include both "niveau de designation"
and "niveau de classification" with their correct scopes.

### FINDING: "information protegee" vs "renseignements designes" -- guidance is correct but incomplete

FINDING: Protected information terminology evolution needs documentation
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: Termium Plus Information Security Glossary
DETAIL: Jamie's tabletop exercise anecdote correctly identifies "renseignements designes"
as the older form and "information protegee" as the current form. Termium Plus confirms
both exist: the Glossary entry for "designated information; protected information" lists
"information designee; renseignements designes; information protegee; renseignements
proteges" as French equivalents. The Glossary does not mark either as deprecated, but
"information protegee" / "renseignements proteges" are the current TBS usage.

The vocabulary-fr_CA.md file does not contain either term. This is a gap.

REMEDIATION: Add "protected information" / "information protegee" and
"designated information" / "information designee" to vocabulary-fr_CA.md with a note
explaining the historical context and current preferred usage.

### FINDING: Guidance correctly requires msgctxt but does not address msgctxt for designation vs classification

FINDING: msgctxt strategy incomplete for Canadian security taxonomy
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: TBS Directive on Security Management
DETAIL: The guidance correctly requires msgctxt "security_label" for Protected A/B/C.
It should also address msgctxt for classification levels (Confidential, Secret, Top Secret)
and for any future PBMM (Protected B / Medium Integrity / Medium Availability) labeling,
which Termium Plus already has an entry for. If UMRS ever displays classification levels
alongside designation levels, the msgctxt strategy must distinguish them.

REMEDIATION: Extend the msgctxt taxonomy in the guidance to cover:
- "security_designation" -- for Protected A/B/C
- "security_classification" -- for Confidential/Secret/Top Secret
- "security_profile" -- for compound markings like PBMM
Or keep a single "security_label" context but document the taxonomy.

### Items the guidance gets right

- The dual-key principle (Simone for linguistic accuracy, Henri for policy accuracy) is
  exactly correct and non-negotiable.
- The French typography rules (non-breaking space before colon, guillemets) are correct.
- The locale detection priority chain is correct.
- The fr_CA vs fr_FR distinction is correctly emphasized.
- The string expansion budget (20-30%) is a reasonable estimate.
- The requirement that .mo files are generated artifacts and not committed is correct.
- The testing requirements (pseudolocalization, coverage validation, policy spot-check)
  are comprehensive.

---

## 2. l10n Resources Assessment

### 2.1 Termium Plus Corpus -- AVAILABLE, PARTIAL

The corpus at `.claude/references/corpus/termium-plus-fr_CA.tsv` contains 32,210 entries from:
- Electronics & Informatics subject (~12,911 entries)
- Administration subject (~18,472 entries)
- Information Security Glossary (287 entries)
- CCCS Glossary (76 entries)

FINDING: Military/Security subject missing from Termium Plus corpus
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Termium Plus Open Government Portal
DETAIL: The SOURCE.md file documents that the "Military and Security Subject" resource
(resource ID 99a220a8-fa42-4231-9aa9-c626135e0912) has NOT been downloaded because the
portal requires JavaScript rendering to discover the download URL. This subject area
contains critical terminology for:
- NATO/NORAD information sharing markings
- Canadian Forces security terminology
- Defence-specific information handling terms
- Five Eyes partner marking terminology in Canadian French

For a project that deals with security labels, MLS, and information classification,
missing the military/security subject from Termium Plus is a significant gap.

REMEDIATION: Manual browser download required. See SOURCE.md for instructions. This should
be prioritized before any l10n work on umrs-labels or security classification displays.

### 2.2 TBS Directive on Security Management -- NOT AVAILABLE

FINDING: TBS Directive on Security Management not in reference corpus
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Treasury Board of Canada Secretariat
DETAIL: The `.claude/references/` directory contains extensive NIST, DoD, and CMMC references but
NO Canadian government policy instruments. The TBS Directive on Security Management
(https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578) is the authoritative source for:
- Security marking requirements
- Designation and classification level definitions
- Information handling standards
- The distinction between designated (Protected) and classified information

Without this document in the reference corpus, policy validation relies on my knowledge
and the Termium Plus glossary entries, which are derivatives of the policy, not the
policy itself.

REMEDIATION: Download the following TBS policy instruments and add to `.claude/references/`:
1. Directive on Security Management (https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578)
2. Policy on Government Security (https://www.tbs-sct.canada.ca/pol/doc-eng.aspx?id=16578)
3. Standard on Security Categorization (if published -- was under development)
4. Appendix J to the Directive (security categorization guidance)

Both English and French versions should be downloaded. The French versions are the
authoritative source for Canadian French security terminology in policy context.

### 2.3 OQLF Grand dictionnaire terminologique -- AVAILABLE

The corpus at `.claude/references/corpus/oqlf-gdt-fr_CA.tsv` contains 25,881 entries. Coverage
assessment from SOURCE.md: 82% of sampled UMRS key terms are covered. Gaps include
mandatory access control, security label, and several other security-specific terms
that are better covered in Termium Plus.

This is expected. OQLF GDT is a general terminology resource. For information security
terminology, Termium Plus is authoritative. OQLF GDT fills gaps where Termium Plus is
silent, particularly for general computing terms.

The OQLF GDT is Level 3 in the terminology hierarchy. It is correctly positioned.

### 2.4 GNU Translation Memory -- AVAILABLE

Seven GNU .po files are available in `.claude/references/corpus/`:
- bash-5.3-rc2.fr.po
- coreutils-9.9.280.fr.po
- cryptsetup-2.8.2-rc0.fr.po
- findutils-4.9.0.fr.po
- grep-3.11.68.fr.po
- sed-4.8.44.fr.po
- tar-1.35.90.fr.po

These are Level 2 in the terminology hierarchy (proven production vocabulary for CLI terms).
They are correctly used by Simone for CLI-specific terms like "droits", "restreint",
"avertissement", etc.

FINDING: GNU .po files are fr, not fr_CA
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: GNU Translation Project
DETAIL: The GNU Translation Project .po files are tagged as "fr" (generic French), not
"fr_CA" (Canadian French). For CLI terms, the divergence between fr and fr_CA is minimal --
"fichier", "repertoire", "droits" are the same in both. But this should be documented:
these files represent Translation Project francophone consensus, which is predominantly
France French contributors. Any term taken from these files and used in a Canadian federal
context should be cross-checked against Termium Plus.

REMEDIATION: Document in SOURCE.md or in the vocabulary file that GNU .po corpus entries are
"fr" (generic/France French) and have been validated against Termium Plus for fr_CA
compatibility where security-relevant terms are concerned.

### 2.5 Missing Resources

FINDING: Missing Canadian government reference documents for l10n validation
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Various Canadian federal sources
DETAIL: The following authoritative resources are not available in the project:

**Required -- high priority:**
1. **TBS Directive on Security Management** (English + French) -- see 2.2 above.
2. **CCCS ITSG-33** (IT Security Risk Management: A Lifecycle Approach) -- the Canadian
   equivalent of NIST SP 800-53. Contains Canadian-specific security controls and their
   French terminology.
3. **Termium Plus Military/Security subject** -- see 2.1 above.
4. **TBS Standard on Security Screening** -- contains terminology for security clearance
   levels in French that must not be confused with security marking terminology.

**Required -- medium priority:**
5. **CCCS ITSP.40.062** (Guidance on Securely Configuring Network Protocols) -- contains
   French technical terminology for network security configurations relevant to UMRS
   posture display.
6. **CCCS Annex 3A** (Security Control Catalogue) -- French-language security control
   names and descriptions.
7. **NATO STANAG 4774/4778** (Confidentiality metadata label syntax / binding to data
   objects) -- contains the authoritative NATO French terminology for security markings
   in multilateral contexts. This is where "marquage de securite" would be confirmed or
   denied for NATO interoperability contexts.

**Useful -- lower priority:**
8. **PWGSC Translation Bureau style guide for federal documents** -- prescribes
   capitalization, punctuation, and formatting rules for Canadian federal French that
   may differ from general French typography.
9. **Official Languages Act implementation guidelines** -- for verifying bilingual
   delivery requirements.

### 2.6 Five Eyes Partner Terminology in French

FINDING: Five Eyes French marking terminology not assessed
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: NATO/Five Eyes partnership agreements
DETAIL: Jamie asked about French translations of AU/NZ/UK marking terms for NATO interop.
The answer is nuanced:

- **Australia, New Zealand, United Kingdom** do not independently produce French translations
  of their security markings. Their markings (OFFICIAL, OFFICIAL:SENSITIVE, PROTECTED, SECRET,
  TOP SECRET for AU; UNCLASSIFIED through TOP SECRET for NZ/UK) are English-only in their
  domestic systems.

- **NATO** is where the Five Eyes French terminology lives. NATO produces authoritative
  bilingual (English/French) marking standards. NATO French is the interoperability layer.
  However, NATO French is not Canadian French -- it is its own register, influenced by
  France French (since France is the primary French-speaking NATO member).

- **Canada** is the only Five Eyes nation that requires domestic French-language security
  markings. Canadian French markings (Protege A/B/C, Confidentiel, Secret, Tres secret)
  are authoritative for Canadian use and are the ones UMRS must implement.

- For interoperability scenarios (e.g., displaying a partner nation's marking to a
  Canadian francophone operator), the question is: do we show the original English marking
  or a French equivalent? The answer depends on whether the marking is a designator
  (retained as-is, like a proper noun) or a classification concept (translatable).

REMEDIATION: Document in the l10n guidance:
1. Canadian Protected markings use TBS French terminology (Protege A/B/C).
2. US CUI markings are retained in English (they are designators, not translatable concepts).
3. Partner nation markings are retained in their original language when displayed as data.
4. Prose describing partner markings uses Canadian French equivalents where they exist.
5. NATO French terminology applies only in NATO interoperability contexts, not for
   Canadian domestic use.

---

## 3. Assessment of Current .po File Quality

### 3.1 Overall Quality

The existing fr_CA .po files (umrs-ls, umrs-state, umrs-logspace, umrs-uname, umrs-platform)
are well-structured. Simone's translator notes are thorough, with source attribution for
every term decision. The vocabulary-fr_CA.md registry is comprehensive and maintains an
auditable decision trail.

From a **policy accuracy** standpoint (which is my domain), I have the following findings:

### 3.2 Specific Term Findings

FINDING: "MARQUAGE" column header -- Termium Plus divergence
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Termium Plus
DETAIL: See Finding in Section 1 above. "MARQUAGE" is not the Termium Plus term for
"security marking." The federal term is "cote de securite." This divergence is documented
in the .po file as "UMRS decision" with NATO/ANSSI justification, but it was not flagged
as a Termium Plus divergence. Per project rules, any terminology decision that diverges
from Termium Plus must be explicitly documented with rationale. The current documentation
gives the rationale but does not acknowledge the divergence.

REMEDIATION: Update the .po translator comment and vocabulary-fr_CA.md to explicitly state:
"DIVERGENCE: Termium Plus maps 'security marking' to 'cote de securite'. UMRS uses
'marquage' because [rationale]. This divergence is approved by [Henri/Jamie]."

FINDING: "Objet du systeme" for "System Purpose" -- unverified RHEL-ism
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: umrs-state fr_CA.po
DETAIL: The .po file claims "Objet du systeme" is "the RHEL francophone term used in RHEL
subscription tooling." This should be verified against the actual RHEL 10 French locale. If
Red Hat's own French translation uses a different term, UMRS should match it for consistency
with the platform. This is not a Termium Plus term (it would not be -- it is a Red Hat
concept, not a government concept).

REMEDIATION: Verify against `subscription-manager` French .po file on RHEL 10. Low priority.

FINDING: No France French / Canadian French divergences detected in .po content
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: Review of all fr_CA .po files
DETAIL: I reviewed all fr_CA .po files for terms that are France French rather than
Canadian French. No fr_FR-specific terms were detected. The translations use standard
computing terminology that is common to both registers. Terms like "accès refusé",
"droits", "taille", "nom" are pan-francophone.

The vocabulary-fr_CA.md correctly sources terms from Termium Plus (Canadian federal) and
GNU .po corpus (pan-francophone), with ANSSI/OTAN terms used only for security concepts
where Termium Plus is silent. This approach is sound.

REMEDIATION: None required. Continue current practice.

### 3.3 fr_FR .po Files -- Policy Concern

FINDING: fr_FR .po files exist alongside fr_CA -- policy ambiguity
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: resources/i18n/ directory structure
DETAIL: Several domain directories contain both fr_CA.po and fr_FR.po files:
- umrs-ls has both fr_CA.po and fr_FR.po (fr_FR has content, identical to fr_CA)
- umrs-state has fr_FR.po (empty, 0 bytes)
- umrs-df has fr_FR.po (empty, 0 bytes)
- umrs-ps has fr_FR.po (empty, 0 bytes)
- umrs-tester has fr_FR.po (empty, 0 bytes)

The presence of fr_FR.po files raises two concerns:
1. **Maintenance burden:** If fr_FR.po is maintained separately from fr_CA.po, every
   terminology decision must be made twice. If they are identical (as in umrs-ls), the
   fr_FR file is redundant.
2. **Policy clarity:** UMRS is a Canadian project built for Canadian government use.
   fr_CA is the target locale. Why does fr_FR exist? If it is for potential France-based
   NATO partners, that is a different use case with different terminology rules.
3. **Empty files:** The 0-byte fr_FR.po files in umrs-df, umrs-ps, umrs-state, and
   umrs-tester are placeholder scaffolding, not translations. They could create false
   impressions of locale support.

REMEDIATION: Jamie should decide:
- If fr_FR support is not in scope, remove fr_FR.po files and document the decision.
- If fr_FR support is planned for future NATO interop, keep the scaffolding but add a
  README in resources/i18n/ explaining the intent and that fr_FR files are not yet active.
- The umrs-ls fr_FR.po (which has content) should be reviewed: is it intentionally
  maintained or was it created as a copy of fr_CA during development?

### 3.4 Domains Without fr_CA -- Gap

FINDING: Three active domains have fr_FR but not fr_CA
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: resources/i18n/ directory structure
DETAIL: umrs-df, umrs-ps, and umrs-tester have domain directories with .pot and fr_FR.po
scaffolding but NO fr_CA.po files. For a Canadian federal project, fr_CA should be the
primary French locale, not fr_FR. Any new domain scaffolding should create fr_CA.po first.

REMEDIATION: When these domains are onboarded, create fr_CA.po as the primary French
locale file. If fr_FR is also needed, it can be created as a derivative.

---

## 4. What Is Missing for a Proper l10n Pipeline

### 4.1 Tooling Gaps

FINDING: xtr installation status unknown
SEVERITY: Medium
DOMAIN: Canadian Policy (indirect -- blocks pipeline)
SOURCE: Simone's architecture report, Section 2.1
DETAIL: Simone's report notes that xtr (the Rust-aware gettext extraction tool) "was not
installed as of 2026-03-10" and "must be verified before any extraction run." Without xtr,
new .pot files cannot be generated from source, which means new translations cannot be
started. This blocks the entire l10n pipeline for new crates.

REMEDIATION: Verify xtr installation. If not installed, add `cargo install xtr` to the
developer setup procedure.

### 4.2 Process Gaps

FINDING: No formal terminology review gate in the l10n pipeline
SEVERITY: High
DOMAIN: Canadian Policy
SOURCE: Project workflow
DETAIL: The l10n guidance correctly identifies the dual-key requirement (Simone for
linguistic accuracy, Henri for policy accuracy). But there is no formal process for
when and how my review happens. Questions:
- Do I review every .po file change, or only security-relevant terms?
- Is there a task board workflow for l10n review requests?
- What is the turnaround expectation?
- How do I flag a term that Simone has translated correctly linguistically but that
  carries incorrect regulatory weight?

REMEDIATION: Define a formal l10n review gate:
1. Simone produces or updates a .po file and creates a task tagged "henri-review".
2. I review all security-relevant terms (anything in the Security and Access Control
   vocabulary section, any new term involving classification/designation/marking/access).
3. I file findings per the standard finding format.
4. Jamie resolves any finding where Simone and I disagree.
5. General computing terms (column headers, button labels, status messages) do not
   require my review unless they touch security concepts.

FINDING: No ATIP review process for bilingual output
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: Access to Information and Privacy Acts
DETAIL: If UMRS is deployed in a Canadian federal environment, its output may be subject
to ATIP requests. Bilingual output (a mix of English and French strings, depending on
locale) raises questions:
- If an ATIP request is filed in English, do French-locale outputs need to be translated
  back for the requestor?
- Are security labels in the output subject to ATIP exemptions under s.15 (International
  affairs and defence) or s.16 (Law enforcement and investigations)?
- How does the Official Languages Act interact with ATIP disclosure obligations?

These are not translation questions. They are policy architecture questions that affect
how the l10n pipeline is designed.

REMEDIATION: Flag for Jamie. This does not block current l10n work but must be addressed
before any Canadian federal deployment. Route to Jamie for decision.

### 4.3 Reference Gaps

See Section 2.5 above for the complete list. The most critical missing references are:
1. TBS Directive on Security Management (English + French)
2. Termium Plus Military/Security subject
3. CCCS ITSG-33

### 4.4 Vocabulary Registry Gaps

FINDING: vocabulary-fr_CA.md missing Canadian classification taxonomy entries
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: vocabulary-fr_CA.md
DETAIL: The vocabulary file covers designation levels (Protected A/B/C) via the
MARQUAGE entry but does not include entries for:
- Classification levels: Confidentiel, Secret, Tres secret
- PBMM (Protege B / Integrite moyenne / Disponibilite moyenne)
- Designation vs classification distinction
- "information protegee" vs "renseignements designes" (current vs historical)
- "bien designe" / "bien protege" (designated/protected asset)
- "document designe" / "document protege" (designated/protected document)
- "niveau de designation" vs "niveau de classification"
- "need-to-know" / "besoin de connaitre" (Termium Plus: masculine noun, officially approved)
- "security clearance" / "habilitation de securite" (preferred) vs "cote de securite 2"

These are all attested in the Termium Plus Information Security Glossary and will be
needed when umrs-labels l10n work begins.

REMEDIATION: Add a "Canadian Security Taxonomy" section to vocabulary-fr_CA.md covering
all Termium Plus Information Security Glossary terms relevant to UMRS. I can draft this
section.

### 4.5 CUI / US Marking Interoperability

FINDING: No l10n strategy for CUI markings in bilingual contexts
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: Five Eyes information sharing requirements
DETAIL: UMRS handles CUI (Controlled Unclassified Information) labels. CUI is a US
designation scheme. When a Canadian francophone operator views CUI markings:
- CUI category names (e.g., "CUI//SP-CTI", "CUI//SP-EXPT") are designators and should
  NOT be translated. They are like proper nouns.
- Prose explaining CUI categories could be translated if UMRS provides descriptions.
- The mapping between US CUI and Canadian Protected designations is not one-to-one.
  This is a policy divergence that must be documented, not resolved in the l10n layer.

Termium Plus has no entry for "CUI" or "Controlled Unclassified Information" -- this is
expected, as CUI is a US-specific program with no Canadian equivalent designation.

REMEDIATION: Add to the l10n guidance:
1. CUI marking strings are never translated. They are retained as-is in all locales.
2. CUI-to-Canadian-Protected mapping is a policy decision, not a translation decision.
   Display both markings if cross-referencing is needed.
3. If UMRS displays CUI explanatory text to francophone operators, those descriptions
   are gettext candidates, but the canonical CUI designator strings are not.

---

## 5. Summary of Findings

| # | Title | Severity | Route To |
|---|---|---|---|
| 1 | PROTEGE capitalization convention | High | Jamie |
| 2 | "cote de securite" vs "marquage" divergence | High | Jamie |
| 3 | Designation vs classification distinction missing | Medium | Jamie |
| 4 | Protected information terminology evolution | Medium | Simone + Jamie |
| 5 | msgctxt taxonomy incomplete | Medium | Jamie |
| 6 | Military/Security Termium subject missing | High | Jamie (manual download) |
| 7 | TBS Directive not in reference corpus | High | Jamie |
| 8 | GNU .po files are fr, not fr_CA | Low | Simone (documentation) |
| 9 | MARQUAGE Termium Plus divergence undocumented | High | Simone + Jamie |
| 10 | fr_FR files policy ambiguity | Medium | Jamie |
| 11 | Domains without fr_CA | Medium | Simone |
| 12 | xtr installation status | Medium | Developer |
| 13 | No formal l10n review gate | High | Jamie |
| 14 | ATIP review process undefined | Medium | Jamie |
| 15 | Vocabulary registry missing classification taxonomy | Medium | Henri (draft) + Simone |
| 16 | CUI marking l10n strategy undefined | Medium | Jamie |
| 17 | Five Eyes French marking terminology | Medium | Jamie |
| 18 | Missing Canadian reference documents | High | Jamie |

**Critical/High findings:** 6
**Medium findings:** 10
**Low/Informational findings:** 2

---

## 6. What We Are Doing Well

This section matters. The team should know what is working.

1. **The dual-key principle is established and understood.** Most projects never get this far.
   Simone does the linguistic work; I validate the policy weight. This is correct and rare.

2. **Simone's translator notes are exemplary.** Every .po entry has source attribution,
   corpus reference, and rationale. This is auditable work. An OLA compliance reviewer
   could trace every term decision to its source.

3. **The vocabulary-fr_CA.md registry is the right artifact.** Having a single, maintained
   reference for all terminology decisions prevents drift and ensures consistency across
   domains.

4. **The Termium Plus and OQLF GDT corpora are available and properly documented.** The
   SOURCE.md file with checksums, dates, and provenance is exactly what an auditor would
   want to see.

5. **The distinction between gettext strings and label designators is correctly understood.**
   The rule that PROTEGE A/B/C are data, not gettext candidates, is exactly right. This is
   a subtle point that most l10n efforts get wrong.

6. **The i18n infrastructure is solid.** gettext-rs, split-domain architecture, OnceLock
   initialization, UMRS_LOCALEDIR escape hatch -- this is well-engineered.

---

## 7. Recommended Next Steps (Priority Order)

1. **Download TBS Directive on Security Management** (English + French) into `.claude/references/`.
2. **Download Termium Plus Military/Security subject** (requires browser -- see SOURCE.md).
3. **Resolve the MARQUAGE vs COTE divergence** -- Jamie's decision required.
4. **Add Canadian Security Taxonomy section to vocabulary-fr_CA.md** -- I will draft this.
5. **Define the formal l10n review gate process.**
6. **Clean up fr_FR scaffolding** -- decide scope and remove or document.
7. **Verify xtr installation.**
8. **Document CUI marking l10n strategy in the guidance.**

---

*A tool that gets the terminology wrong is not just inaccurate -- it is untrustworthy.
And in information security, trust is the only thing we are selling.*

*-- Henri*
