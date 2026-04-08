# Policy Accuracy Review: US-CUI-LABELS.json fr_CA Translations

**Date:** 2026-04-07
**Reviewer:** Henri (Canadian Policy Specialist)
**Source file:** `components/rusty-gadgets/umrs-label/config/us/US-CUI-LABELS.json`
**Scope:** Policy accuracy validation of 143 markings + 10 dissemination controls, fr_CA translations
**Review type:** Policy accuracy (not linguistic accuracy -- Simone owns that)

---

## Summary

| Category | Count |
|---|---|
| ACCURATE | 9 |
| CONCERN | 7 |
| ERROR | 3 |

Overall assessment: Simone's work is strong. The bulk of the catalog is policy-accurate
and uses appropriate federal Canadian French register. The errors and concerns below are
all in the judgment-call and coinage categories, which is exactly where they should be --
the Termium-confirmed translations are clean. Three items require correction before
this catalog can be considered policy-validated.

---

## ACCURATE Findings

### A-1: "Denonciateur" for Whistleblower -- Confirmed Correct

Simone's choice of "denonciateur" is confirmed by Termium Plus at Level 1 authority.
The Termium entry for "U.S. Legislative Protection for Public Sector Whistleblowers"
renders as "La protection accordee par la loi americaine aux **denonciateurs** dans le
secteur public." Multiple additional Termium entries in the public administration domain
consistently use "denonciateur."

Simone's note that "lanceur d'alerte" is fr_FR (France French) is correct. That term
has no Termium Plus entry. Using it would conflate federal Canadian French with
metropolitan French -- a violation of the i18n rules. "Denonciateur" is the right call.

**Verdict:** Sign off. No change required.

### A-2: "Controleur general" for Comptroller General -- Confirmed Correct

Termium Plus confirms "controleur general" in multiple entries including "Bureau du
controleur general" (Office of the Comptroller General). This is the standard federal
Canadian French rendering.

**Verdict:** Sign off. No change required.

### A-3: Operations Security / OPSEC -- Confirmed Correct

The `name` field translates as "Securite de l'exploitation" and the `description` uses
"securite des operations." Termium Plus confirms "securite de l'exploitation" as the
authoritative rendering of "operations security" in the IT domain. However, the
description text uses "securite des operations" which is also acceptable in running
prose as a natural variant. No policy inaccuracy.

**Verdict:** Sign off. No change required.

### A-4: "Infrastructures essentielles" for Critical Infrastructure -- Confirmed Correct

Termium Plus uses "infrastructures essentielles" consistently across multiple entries
(Protection des infrastructures essentielles, plan de protection des infrastructures
essentielles, etc.). This is the standard Canadian government term. Simone has applied
it consistently across all Critical Infrastructure index group entries (CUI//CRIT,
CUI//DCRIT, CUI//SP-CEII, CUI//SP-PCII, etc.).

**Verdict:** Sign off. No change required.

### A-5: "Application de la loi" for Law Enforcement -- Confirmed Correct

Termium Plus confirms "application de la loi" as the standard federal term (e.g.,
"Guide provisoire des operations d'application de la loi" for "Interim Law Enforcement
Operations Manual"). Simone uses this consistently across all Law Enforcement index
group entries.

**Verdict:** Sign off. No change required.

### A-6: "Grand jury federal" for Federal Grand Jury -- Confirmed Correct

Simone correctly identifies this as a loan phrase for an institution with no Canadian
equivalent. The French legal tradition does not have grand juries. Retaining the English
institutional name with a French determiner ("du grand jury federal") is the standard
approach for US-specific legal concepts in Canadian federal French translation. There is
no Termium Plus entry for this term, making it a Level 5 judgment call, but the judgment
is sound and well-documented.

**Verdict:** Sign off. Document as Level 5 terminology decision.

### A-7: "Pornographie juvenile" for Child Pornography -- Confirmed Correct

Termium Plus confirms "pornographie juvenile" in the entry for "Loi concernant la
declaration obligatoire de la pornographie juvenile sur Internet." This is the
authoritative Canadian federal term.

Note: Canadian federal law has shifted toward "child sexual abuse material" / "materiel
d'exploitation sexuelle d'enfants" in recent years, but the NARA CUI category name is
"Child Pornography" and Simone correctly translated the NARA name as given. If NARA
renames the category, the translation should follow.

**Verdict:** Sign off. No change required.

### A-8: "Informations relatives a la loi SAFETY Act" -- Confirmed Correct

Retaining the US statute name in English is correct. SAFETY Act is a proper noun
(Support Anti-terrorism by Fostering Effective Technologies Act). Translating the
acronym would be a policy error.

**Verdict:** Sign off. No change required.

### A-9: Dissemination Controls -- Generally Accurate

The 10 dissemination controls are well-translated. Specific notes:
- "Entrepreneur" for contractor is Termium-confirmed (Level 1)
- "Diffusion" for dissemination is standard federal usage
- "Secret professionnel de l'avocat" for attorney-client privilege is Canadian common
  law terminology
- "Aucune diffusion a l'etranger" for NOFORN captures the operational meaning

**Verdict:** Sign off with one concern noted separately (C-5).

---

## CONCERN Findings

### C-1: "Beneficiaire de l'asile" for Asylee -- Acceptable but Undocumented

**Recommendation:** Simone's distinction between "beneficiaire de l'asile" (person
granted asylum) and "demandeur d'asile" (asylum seeker) is semantically correct
and important. However, this is a Level 5 terminology decision -- no Termium Plus
entry exists for "asylee" or "beneficiaire de l'asile."

The CUI category name is "Asylee" (the person with granted status), not "Asylum
Seeker." Simone's translation correctly distinguishes the two. However, the NARA
description field says "Records related to asylum seekers **and** asylee status
determinations" -- it covers both. The fr_CA description correctly handles this
by using both terms ("demandeurs d'asile" and "beneficiaire de l'asile").

Document this as a Level 5 decision in the approved vocabulary list. If Termium Plus
acquires an entry for this immigration term, revisit.

### C-2: "Personne placee sous contrainte judiciaire" for Committed Person -- Acceptable but Long

**Recommendation:** This is a Level 5 decision. The term is semantically accurate --
"committed person" in US law refers to someone placed under legal restraint through
judicial or law enforcement processes. The French rendering captures the judicial
placement concept. However, at 5 words it is unusually long for a `name` field that
will appear in UI labels.

Consider whether a shorter form exists. Canadian mental health law uses "personne sous
garde" (person in custody) in some contexts, but this does not fully capture the
judicial commitment concept. The current translation is policy-accurate; the length
is a UI concern, not a policy concern. Flag to Jamie for UI review.

### C-3: "Registre telephonique / dispositif de reperage" for Pen Register / Trap & Trace -- Acceptable

**Recommendation:** This is a Level 5 decision. Pen registers and trap-and-trace
devices are US-specific surveillance instruments defined in 18 U.S.C. Ch. 206.
Canada's equivalent (Part VI of the Criminal Code, "Interception of Private
Communications") uses different legal vocabulary. Simone's functional translation
captures the operational meaning without claiming a false Canadian legal equivalence.

Document as Level 5. The slash construction mirrors the English original. No policy
error, but note that a Canadian operator may not immediately recognize the US legal
instruments from the French rendering alone. Consider adding a translator's note in
the catalog description if this matters for operator comprehension.

### C-4: UMRS Coinage "avec traitement de base" for Baseline Handling -- Acceptable with Reservation

**Recommendation:** This phrase appears in many description fields (CUI//CVI, CUI//DCNI,
CUI//FNC, CUI//FSEC, CUI//GENETIC, CUI//HLTH, CUI//ID, CUI//INF, CUI//INTEL,
CUI//INV, etc.) as the translation for "under baseline handling."

Simone's rationale (TBS privacy directive usage of "traitement") has merit -- TBS does
use "traitement" in data handling contexts. However, "traitement de base" could also
read as "basic processing" in French, which introduces a semantic ambiguity. The English
"baseline handling" refers to the standard set of CUI safeguards per 32 CFR 2002, not
to data processing.

An alternative: "avec les mesures de protection de base" (with baseline safeguards) is
more explicit but longer. Or "avec les garanties CUI de base" which would match the
phrasing already used elsewhere in the descriptions ("au-dela des garanties CUI de
base").

This is not an error -- the meaning is recoverable in context -- but it is a coinage
that should be recorded in the approved vocabulary list with a note about the potential
ambiguity. Route to Jamie for decision on whether to standardize on one phrasing.

### C-5: "Produit du travail de l'avocat" for Attorney Work Product -- Acceptable in Common Law Context

**Recommendation:** The attorney work product doctrine is a common law concept that
exists in Canadian common law provinces. Simone's translation is defensible. However,
in Quebec civil law ("droit civil"), the concept maps to "secret professionnel" more
broadly, and "produit du travail" is not standard Quebec legal French.

Since this is a US CUI catalog (not a Canadian catalog), and since the translation
targets fr_CA federal usage (not Quebec civil law), Simone's rendering is acceptable.
But document this as a potential Five Eyes friction point: if this catalog is ever
displayed on a Quebec-based system, the term may confuse Quebec-trained legal
professionals.

### C-6: UMRS Coinage "(precisee/precisees)" for NARA "Specified" Designation

**Recommendation:** This appears as a parenthetical suffix on all Specified category
names: "Procedures administratives (precisees)", "Budget (precise)", etc. The
grammatical agreement pattern (masculine/feminine/plural) is correctly applied
throughout.

"Precise" is not the worst choice -- Termium Plus does use "precisees" as an
adjective in "tables de traitement precisees." However, in Canadian federal legal
French, "specifie" or "designe" would be closer to the NARA intent. "Specified"
in the CUI context means "the governing authority has specified additional
requirements" -- the sense is closer to "vise" (targeted by regulation) or
"designe" (designated) than to "precise" (clarified/detailed).

Alternatives to consider:
- "(vise)" / "(visee)" -- "targeted by specific regulation"
- "(designe)" / "(designee)" -- "designated" (closer to the NARA designation concept)
- "(avec exigences specifiques)" -- explicit but long

This is not an error because the meaning is recoverable and no Termium Plus entry
exists for the NARA "Specified" designation. Document as Level 5 coinage. Route
to Jamie for decision.

### C-7: Word Order in "Protection des renseignements personnels generale" (CUI//PRVCY)

**Recommendation:** The `name` field for CUI//PRVCY reads "Protection des
renseignements personnels generale." The adjective "generale" modifies "protection"
but is separated from it by the prepositional phrase "des renseignements personnels,"
which creates an awkward reading. The specified variant (CUI//SP-PRVCY) has the same
issue: "Protection des renseignements personnels generale (precisee)."

Linguistically this is Simone's domain, not mine. But from a policy standpoint, the
question is whether "generale" could be misread as modifying "personnels" (general
personal information) rather than the protection regime (general privacy). In the
context of CUI categories, this distinction matters -- it is the privacy *category*
that is general, not the personal information.

A clearer rendering might be: "Protection generale des renseignements personnels."
Flag to Simone for linguistic judgment on the adjective placement.

---

## ERROR Findings

### E-1: "Securite de l'exploitation" vs "securite des operations" Inconsistency in OPSEC Entry

**Severity:** Low
**Key:** `CUI//OPSEC`

The `name` field correctly uses "Securite de l'exploitation" (Termium Level 1), but the
`description` field uses "securite des operations" in the phrase "compromettrait la
securite des operations." While both are comprehensible, using two different French
terms for the same English concept ("operations security") within a single catalog
entry creates an internal inconsistency.

**Recommended fix:** Change the description's "securite des operations" to "securite de
l'exploitation" to match the name field and the Termium Plus authoritative term:

- **Current:** `"compromettrait la securite des operations"`
- **Corrected:** `"compromettrait la securite de l'exploitation"`

### E-2: "RELIDO" Description Translation -- "renseignements sans restrictions" is Inaccurate

**Severity:** Medium
**Key:** `RELIDO` (dissemination control)

The English source says "uncaveated intelligence material." Simone's translation renders
this as "renseignements sans restrictions." This is policy-inaccurate.

"Uncaveated" in the intelligence community means "without caveats" (without handling
restrictions or access limitations beyond the base classification). This is a specific
IC term. "Sans restrictions" (without restrictions) overstates the meaning -- the
material still has a classification and handling requirements; it simply lacks
additional caveats (such as NOFORN, ORCON, etc.).

**Recommended fix:**
- **Current:** `"renseignements sans restrictions"`
- **Corrected:** `"renseignements sans reserve"` or `"renseignements non assortis de reserves"`

"Reserve" in Canadian federal intelligence French corresponds to "caveat" -- it is the
term used when a producing agency attaches conditions to further dissemination. This is
a Level 5 decision (no Termium entry for "uncaveated") but "sans reserve" is the
established Five Eyes Canadian intelligence vocabulary for this concept.

This matters because a Canadian intelligence professional reading "sans restrictions"
on a Five Eyes-shared document could interpret it as material with no handling
requirements at all, which is not what RELIDO conveys.

### E-3: "Divulgable par le responsable de la divulgation de l'information" for RELIDO Name -- Overly Literal

**Severity:** Low
**Key:** `RELIDO` (dissemination control, `name` field)

The English name is "Releasable by Information Disclosure Official." Simone's
translation is technically accurate word-for-word, but the result is a 10-word name
field that does not match how Canadian government French handles these concepts.

"Information Disclosure Official" refers to a "Senior Foreign Disclosure and Release
Authority" (SFDRA) in the IC context. The description field correctly explains this.
However, in the `name` field, a more concise rendering would be:

- **Current:** `"Divulgable par le responsable de la divulgation de l'information"`
- **Suggested:** `"Divulgable par le responsable de la divulgation"`

The trailing "de l'information" is redundant -- in the CUI context, the official's
role is understood to concern information disclosure. Removing it shortens the name
without losing policy meaning.

This is a Low severity concern because the current text is not wrong, just unnecessarily
verbose for a name field.

---

## Terminology Decision Log

Per project rules, every terminology decision that diverges from Termium Plus must be
documented. The following Level 5 decisions from this catalog require entry in the
approved vocabulary list:

| English | fr_CA | Level | Rationale |
|---|---|---|---|
| Asylee | Beneficiaire de l'asile | 5 | No Termium entry; distinguishes from demandeur d'asile |
| Committed Person | Personne placee sous contrainte judiciaire | 5 | No Termium entry; captures judicial placement concept |
| Federal Grand Jury | Grand jury federal | 5 | Loan phrase; no Canadian legal equivalent |
| Pen Register / Trap & Trace | Registre telephonique / dispositif de reperage | 5 | No Termium entry; functional translation of US legal instruments |
| SAFETY Act Information | Informations relatives a la loi SAFETY Act | 5 | US statute proper noun retained |
| Whistleblower | Denonciateur | 1 | Termium-confirmed; "lanceur d'alerte" is fr_FR |
| RELIDO | Divulgable par le responsable de la divulgation | 5 | US IC-specific; no Canadian equivalent |
| Attorney Work Product | Produit du travail de l'avocat | 5 | Common law concept; no Termium entry |
| Comptroller General | Controleur general | 1 | Termium-confirmed |
| (Specified) suffix | (precise/precisee) | 5 | No NARA equivalent in Termium; closest match |
| baseline handling | avec traitement de base | 5 | TBS "traitement" usage; potential ambiguity noted |
| uncaveated | sans reserve | 5 | Henri recommendation; Canadian IC vocabulary |

---

## Divergence Findings (Termium vs OQLF)

No Termium-vs-OQLF divergences were identified in this review. All Termium-confirmed
terms were used where available. No OQLF GDT consultation was required because Termium
provided authoritative entries for all terms that had corpus coverage.

---

## Five Eyes Policy Observations

### F-1: US CUI Catalog Displayed on Canadian Systems

When this catalog is rendered on a fr_CA system, operators will see French translations
of US-specific legal concepts (grand jury, pen register, SAFETY Act). These concepts
have no Canadian equivalents. The translations correctly convey the operational meaning
without implying Canadian legal parallels.

However, in a Five Eyes information-sharing context where Canadian operators receive
US-originated CUI material, the French renderings must not be mistaken for Canadian
legal categories. Consider whether the catalog display should include a visible
indicator that these are US program markings, not Canadian Protected designations.
This is a UI/UX concern for Jamie, not a translation correction.

### F-2: NOFORN Implications for Five Eyes

The NOFORN translation ("Aucune diffusion a l'etranger") is accurate but carries
special weight in a Five Eyes context. NOFORN on US CUI material means it cannot be
shared with Canada, even though Canada is a Five Eyes partner. The translation
correctly conveys this absolute restriction. A Canadian operator seeing this marking
in French should understand that the material is not releasable to them.

This is an information-sharing policy observation, not a translation finding.

---

## Strengths Worth Preserving

1. **Consistent register throughout.** Simone maintained federal Canadian French
   register across all 143 markings without dropping into metropolitan French or
   Quebec French. This is difficult to sustain over a catalog this large.

2. **Correct use of "entrepreneur" for contractor.** Termium-confirmed. Many
   translators default to "contractant" (a false cognate). Simone avoided this.

3. **Correct application of "infrastructures essentielles."** The US uses "critical
   infrastructure" where Canada uses "infrastructures essentielles." Simone applied
   the Canadian standard consistently, which is correct -- this is a translation
   for Canadian audiences, not a transliteration.

4. **Grammatical agreement on the "(precise/precisee/precises/precisees)" suffix.**
   Every Specified category entry correctly agrees in gender and number with the
   preceding noun. Over 50+ entries, this is a nontrivial consistency achievement.

5. **Proper handling of US statute names.** SAFETY Act, FISA, HIPAA, FERPA, GINA --
   all correctly retained as English proper nouns. No false translations of statute
   names.

---

## Remediation Owner Summary

| Priority | Finding | Owner | Action |
|---|---|---|---|
| 1 | E-2: RELIDO "sans restrictions" | Simone (fix) + Henri (validate) | Change to "sans reserve" or equivalent |
| 2 | E-1: OPSEC inconsistency | Simone | Align description to Termium term |
| 3 | E-3: RELIDO name verbosity | Simone | Shorten name field |
| 4 | C-4: "traitement de base" coinage | Jamie (decide) | Standardize phrasing across catalog |
| 5 | C-6: "(precise)" coinage | Jamie (decide) | Accept or choose alternative |
| 6 | C-7: PRVCY word order | Simone (linguistic judgment) | Reorder if linguistically warranted |
| 7 | C-1 through C-5 | Henri | Document Level 5 decisions in approved vocabulary |

All findings route to Jamie for final decision per standard policy finding protocol.

---

**Review status:** CONDITIONAL PASS -- three errors require correction, seven concerns
require documentation or decision. The bulk of the catalog (130+ entries) passes policy
review without modification.

Henri
Canadian Policy Specialist, UMRS Project
