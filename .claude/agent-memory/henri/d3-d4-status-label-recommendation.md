# D3 and D4: Status Tag and Trust Label i18n Recommendation

**Agent:** Henri (Canadian Policy Specialist)
**Date:** 2026-04-01
**Scope:** Project-wide pattern for status indicators and security trust labels in CLI output
**Routes to:** Jamie (decision authority)

---

## D3: Status Tag Column Width

### Recommendation: Option A -- Keep English Tags in All Locales

**Rationale by source:**

**1. TBS Canada.ca Content Style Guide (s4.1)**

The TBS guide prohibits all-caps only for *emphasis* purposes. It explicitly
permits all-caps for abbreviations and standardized codes. Status tags like
`[PASS]`, `[FAIL]`, `[WARN]` fall into the category of standardized operational
codes. The TBS guide does not require translation of machine-status indicators.

**2. Federal government CLI tool precedent**

There is no TBS directive that governs CLI status output formatting specifically.
The Directive on the Management of Communications applies to "public-facing
websites and digital services." CLI tools for security operators are not
public-facing web content. The Official Languages Act (OLA) requires that
federal services be available in both official languages, but this obligation
applies to *communication with the public* and *language of work* -- not to
machine-status codes displayed within an operator's terminal.

Canadian federal tools that produce structured output (CSE's Assemblyline
malware analysis platform, for example) use English-language status codes
internally. The pattern in Canadian federal security tooling is:

- Prose messages, error descriptions, help text: bilingual (OLA requirement)
- Machine-status codes, log severity levels, validation tags: English (operational standard)

This is the same convention as HTTP status codes, syslog severity levels,
and SELinux access vector labels -- none of which are translated.

**3. Five Eyes interoperability**

Audit logs and configuration validation reports are cross-national artifacts.
A US analyst reviewing a Canadian system's validation output must be able to
parse `[PASS]` and `[FAIL]` without a translation table. Translating these
tags creates a barrier to interoperability with no corresponding policy benefit.

**4. Column alignment is not cosmetic**

Status tags in fixed-width CLI output serve an accessibility function. Operators
scanning a validation report rely on column alignment to identify failures rapidly.
Dynamic-width columns or padded translations both degrade scannability. This is
an operator safety concern in time-sensitive security contexts, not a formatting
preference.

### What must be translated (OLA compliance)

The *messages* adjacent to the status tags must be translated. The tags themselves
are codes. The pattern is:

```
English:
  [PASS] trust_anchors: Valid PEM at config/C2PA-TRUST-LIST.pem (18 certificates)

French:
  [PASS] trust_anchors : PEM valide dans config/C2PA-TRUST-LIST.pem (18 certificats)
```

Note the non-breaking space before the colon in the French output per French
typography rules. The check name (`trust_anchors`) is a machine identifier and
is not translated. The message text is translated.

### Why not Option B or C

**Option B (translate tags):** Creates a Five Eyes interoperability problem and
a column alignment problem with no corresponding OLA obligation. The OLA does
not require translation of machine-status codes. This would be over-compliance
that degrades operational utility.

**Option C (translate but enforce max width):** Constraining French abbreviations
to 6 characters produces terms that are neither standard French vocabulary nor
recognizable operational codes. `[REUSSI]` is 6 characters but requires dropping
the accent (`REUSSI` vs `REUSSI`). `[ECHEC]` works but `[AVERT]` is not a standard
French abbreviation for "avertissement." The result would be a set of invented
codes that are neither English-standard nor French-standard -- the worst of both
approaches.

### FINDING: Status tags are operational codes, not prose

```
FINDING: CLI status tags ([PASS], [FAIL], [WARN], [INFO], [SKIP]) are operational codes
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: Official Languages Act, s21-22; TBS Directive on the Management of Communications
DETAIL: The OLA obligation to provide federal services in both official languages applies
  to communication with the public and language of work. Machine-status codes in CLI output
  are operational identifiers analogous to HTTP status codes, syslog levels, and SELinux
  access vector labels. No federal policy instrument requires their translation. Translating
  them would create Five Eyes interoperability friction without satisfying any compliance
  requirement.
REMEDIATION: Keep English status tags in all locales. Translate adjacent prose messages.
  Document this decision as the project-wide pattern.
```

---

## D4: TrustStatus Display Labels

### Question 1: Should trust labels be translated in fr_CA output?

**Recommendation: No. Keep English trust labels in all locales.**

The five trust status labels (`TRUSTED`, `UNVERIFIED`, `INVALID`, `REVOKED`,
`NO TRUST LIST`) are security-relevant indicators. They carry a specific
meaning that operators are trained to recognize. They appear in:

- Chain-of-custody reports (screen output)
- JSON output (`--json` mode)
- Audit logs (if integrated with syslog)

These labels function as a controlled vocabulary for trust evaluation. They
are not prose descriptions. They are closer to the CUI banner markings
(`CUI//SP-CTI`) than to error messages.

**Policy basis:**

1. **No Canadian federal standard defines French equivalents for C2PA trust
   status.** The C2PA specification (Coalition for Content Provenance and
   Authenticity) is an English-language industry standard. There is no
   Termium Plus entry for "trust status" in the C2PA sense. There is no
   CCCS guidance on rendering C2PA validation results in French.

2. **The TBS Standard on Security Categorization defines Protected A/B/C
   terminology in both languages because those are Canadian government
   designations.** C2PA trust status is not a Canadian government designation.
   It is an industry-standard provenance indicator. The obligation to translate
   Canadian government security markings does not extend to third-party
   standard vocabulary.

3. **Inventing French equivalents for C2PA trust status creates a
   terminology collision risk.** "INVALIDE" in French has broader connotations
   than `INVALID` in the C2PA context (which specifically means "signature
   verification failed or hash mismatch"). A French operator encountering
   `[INVALIDE]` may interpret it as "the file is corrupt" rather than
   "the cryptographic signature did not verify." The English term is more
   precise because it is the term of art.

### Question 2: If translated, what are the correct TB/CCCS terms?

Not applicable per recommendation above. However, if Jamie overrides this
recommendation, the following analysis applies:

There are no authoritative Termium Plus entries for these specific trust
evaluation terms in the C2PA context. The closest Termium Plus entries are:

| English | Termium Plus candidate | Risk |
|---------|----------------------|------|
| TRUSTED | `de confiance` / `fiable` | "Fiable" means "reliable," not "cryptographically verified." Policy-inaccurate. |
| UNVERIFIED | `non verifie` | Acceptable but generic. Does not convey "signature present but unvalidated." |
| INVALID | `invalide` | Overly broad in French. Could mean "expired," "corrupt," or "unauthorized." |
| REVOKED | `revoque` | Closest match. Termium Plus has `certificat revoque` for revoked certificate. |
| NO TRUST LIST | `aucune liste de confiance` | Too long for column display. No standard abbreviation exists. |

```
FINDING: No Termium Plus authority for C2PA trust status vocabulary in French
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: Termium Plus (searched: trust status, certificate validation, chain of custody)
DETAIL: Termium Plus does not contain entries for C2PA-specific trust evaluation
  terminology. The closest entries are general cryptography and PKI terms that do not
  carry the same specific meaning as the C2PA trust status values. Inventing
  translations without Termium Plus authority would violate the terminology hierarchy
  (Level 5 -- agent judgment as last resort) and would require explicit documentation
  of the rationale for each invented term.
REMEDIATION: Keep English trust labels. If translation is required by Jamie, each
  term requires a formal terminology decision with documented rationale, and all five
  terms need msgctxt in the .po file to prevent automated tool substitution.
```

### Question 3: Five Eyes report sharing -- generating locale or English?

**Recommendation: Reports should render trust labels in English regardless
of generating system locale.**

This is not actually a locale question. It is a controlled vocabulary question.
The trust labels are part of the report's semantic content, not its
presentation language. Consider the parallel:

- A French-locale system displaying US CUI markings renders them in English
  (`CUI//SP-CTI`), not in French. The marking is a controlled vocabulary
  item, not prose.
- A Canadian system displaying Protected B renders it as `PROTECTED B` (EN)
  or `PROTEGE B` (FR) depending on locale because those are Canadian
  government designations with authoritative bilingual forms.
- C2PA trust status has no authoritative French form. Therefore it renders
  in the language of the standard: English.

For the rest of the report (prose headers, field labels, summary text),
locale-appropriate language applies. The trust status values themselves are
invariant across locales.

### Question 4: CSE/CCCS tooling precedent

CSE's Assemblyline platform and CCCS's published indicators of compromise
use English-language status codes internally. The pattern is consistent:

- Severity levels: English (`critical`, `high`, `medium`, `low`)
- Verdict labels: English (`malicious`, `suspicious`, `clean`)
- Confidence indicators: English (`high`, `medium`, `low`)

User-facing messages (descriptions, analyst notes, UI chrome) are bilingual
per OLA requirements. Machine-status vocabulary is English.

This is the established precedent in Canadian federal security tooling.

---

## Summary Decision Table

| Item | Recommendation | Translate? | Rationale |
|------|---------------|-----------|-----------|
| `[PASS]` / `[FAIL]` / `[WARN]` / `[INFO]` / `[SKIP]` | Keep English | No | Operational codes, not prose. OLA does not require translation. |
| `TRUSTED` / `UNVERIFIED` / `INVALID` / `REVOKED` / `NO TRUST LIST` | Keep English | No | C2PA standard vocabulary. No Termium Plus authority for French equivalents. |
| Check names (`trust_anchors`, `cert_chain`, etc.) | Keep English | No | Machine identifiers. |
| Check messages ("Valid PEM at...", "File not found") | Translate | Yes | User-facing prose. OLA obligation. |
| Report headers ("Chain of Custody", "Configuration Validation") | Translate | Yes | User-facing prose. OLA obligation. |
| Field labels ("Signed at", "Issuer", "Alg") | Translate | Yes | User-facing prose. OLA obligation. |
| Security markings (Protected A/B/C, CUI categories) | Translate | Yes | Canadian government designations with authoritative bilingual forms (TB). |

---

## Implementation Notes for the .po File

1. **Status tags and trust labels:** Do not create `msgid`/`msgstr` entries
   for status tags or trust labels. They are not in the translation pipeline.
   They are constants in Rust source.

2. **msgctxt requirement:** If Jamie overrides and requires trust label
   translation, every trust label `msgid` must carry `msgctxt "c2pa_trust_status"`
   to prevent automated translation tools from substituting generic terms.

3. **Layout budget:** The prose messages adjacent to status tags must respect
   the 20-30% expansion budget. The `.pot` file should flag these with
   `#. LAYOUT-SENSITIVE: max width NN chars` comments.

4. **Non-breaking space:** French field labels followed by a colon require
   `\u00a0:` (non-breaking space before colon). This applies to translated
   field labels in the report, not to status tags.

---

## Cross-Reference to Existing Findings

This recommendation is consistent with:

- **setrans.conf language finding** (Appendix J report, finding 4): The
  setrans.conf language question for Canadian markings is a *different*
  question because Protected A/B/C are Canadian government designations
  with authoritative bilingual forms. Status tags and trust labels are not.

- **Five Eyes locale rendering rule** (i18n_l10n_rules.md): The existing
  rule states that locale-specific security labels render in the locale of
  the receiving nation's system. This applies to *government security
  designations* (Protected, CUI), not to industry-standard vocabulary
  (C2PA trust status).

---

## Divergence Documentation

No Termium Plus vs OQLF GDT divergence to document for D3 or D4.
The relevant terms are not in either corpus because they are
English-language industry-standard vocabulary, not government terminology.

This is itself a finding:

```
FINDING: C2PA trust vocabulary has no French-language authority
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: Termium Plus, OQLF GDT (negative search result)
DETAIL: The C2PA trust status vocabulary (TRUSTED, UNVERIFIED, INVALID,
  REVOKED, NO TRUST LIST) has no entry in Termium Plus or OQLF GDT.
  These terms originate from the Coalition for Content Provenance and
  Authenticity specification, which is published in English only.
  Any French rendering would be a UMRS invention without external authority.
REMEDIATION: Use English terms. If Jamie requires French equivalents in
  future, route to Simone for linguistic proposal and Henri for policy
  validation before adoption. Document as Level 5 terminology decision
  (agent judgment) per the terminology hierarchy.
```
