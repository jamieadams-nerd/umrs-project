# umrs-c2pa French Man Page -- Policy Accuracy Review

**Reviewer:** Henri (Canadian Policy Specialist)
**Date:** 2026-04-01
**File under review:** `components/rusty-gadgets/umrs-c2pa/docs/fr/umrs-c2pa.1`
**English original:** `components/rusty-gadgets/umrs-c2pa/docs/umrs-c2pa.1`
**Review scope:** Policy accuracy, terminology, TBS conventions, Five Eyes interoperability
**Linguistic accuracy:** Out of scope (Simone's domain)

---

## Summary Table

| Category | Count |
|----------|-------|
| ACCURATE | 14 |
| CONCERN  | 7 |
| ERROR    | 3 |

---

## ACCURATE Findings

### A-1: D3/D4 compliance -- UNVERIFIED kept in English

Line 262: `UNVERIFIED` trust status label is kept in English in the French man page.
Consistent with D3/D4 recommendation. Correct.

### A-2: D3/D4 compliance -- NO TRUST LIST kept in English

Line 175: `NO TRUST LIST` is kept in English. Consistent with D3/D4 recommendation. Correct.

### A-3: Security marking examples kept in English

Line 77: CUI marking examples (`CUI`, `CUI//SP-CTI//NOFORN`) are kept in English within
guillemets. CUI markings are US-standard controlled vocabulary and must not be translated.
Correct.

### A-4: Algorithm table preserved as-is

Lines 146-156: Algorithm names (es256, es384, etc.) and type identifiers (ECDSA, RSA-PSS)
are preserved in English. These are cryptographic standard identifiers. Correct.

### A-5: FIPS reference preserved

Lines 144, 254: FIPS references kept in English. FIPS is a US NIST standard designation.
The French rendering "conformes aux normes FIPS" is appropriate -- it translates the
surrounding prose while keeping the standard name in English. Correct.

### A-6: C2PA kept as proper noun

Throughout: "C2PA" is kept as the English proper noun / consortium name. Correct. There is
no French equivalent; the Coalition for Content Provenance and Authenticity is an
English-language standards body.

### A-7: NIST SP reference preserved

Line 271: `NIST SP 800-53 AU-3` is preserved verbatim. Correct. NIST standards are never
translated.

### A-8: Address form -- "vous" used throughout

Lines 108, 117: Imperative forms use "vous" consistently (e.g., "Soumettre", "définissez").
TBS Content Style Guide requires "vous" (formal) for federal French content. Correct.

### A-9: Date format

Line 15: `1er avril 2026` -- correct Canadian French date format per TBS guide
(day month year, month lowercase, ordinal for first day of month). Correct.

### A-10: Section headings use infinitive mood

Lines 96-97, 107, 137-138: Subcommand descriptions use infinitive mood ("Générer un
nouveau certificat", "Valider les justificatifs", "Exécuter les vérifications"). This
follows TBS s5.1.1 requirement for French headings. Correct.

### A-11: "marquage de sécurité" for security marking

Lines 49, 77, 257: "marquage de sécurité" is used for "security marking." Termium Plus
confirms "marquage" as the correct term for marking in this context (as opposed to
"étiquette" which is more general). Correct.

### A-12: "clé privée" for private key

Lines 97, 243, 251: "clé privée" is Termium Plus standard for "private key" in
cryptographic context. Correct.

### A-13: "certificat auto-signé" for self-signed certificate

Line 107: "certificat auto-signé" is Termium Plus standard. Correct.

### A-14: "ancres de confiance" for trust anchors

Lines 161, 194: "ancres de confiance" for "trust anchors." Termium Plus has "ancre de
confiance" for trust anchor in PKI context. Correct.

---

## CONCERN Findings

### C-1: "justificatif d'identité" for "credential" in PKI context

**Lines:** 128, 204

Simone used "justificatifs d'identité de signature" for "signing credentials."

Termium Plus offers "justificatif d'identité" for "credential" in the general identity
context. However, in PKI context, the credential being validated is not an identity
document -- it is a certificate-and-key pair. The Termium Plus entry for "credential" in
the IT security domain also lists "titre de compétence" and "accréditation," neither of
which applies here.

The issue is that "justificatif d'identité" carries a connotation of an identity document
(like a passport or ID card), whereas in PKI context the "credential" is a cryptographic
key pair bound to a certificate. The English original uses "credentials" in the
colloquial IT sense (cert + key = your credentials).

**Recommendation:** Termium Plus does not have a precise entry for "PKI signing credential"
as a compound noun. "Justificatif d'identité" is the closest Termium Plus match and is
defensible. However, an alternative rendering such as "certificat et clé de signature"
(certificate and signing key) would be more technically precise in context, since the
`creds validate` subcommand specifically checks that the cert and key files exist and match.

This is not an error -- "justificatif d'identité" is a valid Termium Plus term. It is a
concern because the connotation leans toward physical identity documents rather than
cryptographic material. Route to Simone for linguistic judgment on whether the connotation
is problematic in practice.

### C-2: "chaîne de possession" for "chain of custody"

**Lines:** 39-40, 43, 48, 64

Simone used "chaîne de possession" for "chain of custody."

Termium Plus has two relevant entries:
- "chaîne de possession" -- appears in legal/forensic contexts (evidence custody)
- "chaîne de traçabilité" -- appears in some supply chain/food safety contexts

The OQLF GDT lists "chaîne de possession" as the primary translation for "chain of
custody" in legal and forensic contexts. This is the same domain as C2PA (provenance
and custody of digital evidence).

**Recommendation:** "Chaîne de possession" is correct. Both Termium Plus and OQLF GDT
agree on this term for legal/forensic chain of custody. No divergence to document.
This concern is resolved in Simone's favor -- marking it as a concern only because the
review brief asked me to verify it explicitly.

### C-3: "caisse" for "crate" (Rust dependency)

**Line:** 280

"Cet outil dépend de la caisse c2pa d'Adobe" -- "caisse" for Rust "crate."

This is a known terminology problem. GNU `.po` files for Rust tooling do not have an
established French term for "crate" because the Rust compiler and Cargo are not yet
translated into French via the Translation Project. Termium Plus has no entry for
"crate" in the software package sense.

OQLF GDT has no entry either. "Caisse" is a literal translation (wooden crate/box)
that has been used informally in some French Rust community contexts, but it has no
authority behind it.

**Recommendation:** This is a Level 5 terminology decision (agent judgment). Document
the rationale. Options:
1. Keep "caisse" -- informal community convention, no authority
2. Use "bibliothèque" (library) -- technically imprecise (a crate can be a binary)
3. Keep "crate" untranslated as a proper noun -- follows the pattern of keeping
   technical terms from English-only ecosystems

I lean toward option 3 (keep "crate" in English) given that there is no authoritative
French term. Route to Simone for final decision.

### C-4: Missing --detailed-json option in French SYNOPSIS

**Lines:** 19-35 (French SYNOPSIS) vs. lines 7-31 (English SYNOPSIS)

The English SYNOPSIS shows three JSON output modes:
```
.RB [ \-\-json | \-\-detailed\-json | \-\-chain\-json ]
```

The French SYNOPSIS omits the `--detailed-json` and `--chain-json` options from the
SYNOPSIS block and only shows `--sign` on the second synopsis line. The `--marking`
and `--output` options are also missing from the SYNOPSIS (though they appear in the
OPTIONS section).

This is not a policy finding per se, but incomplete documentation of security-relevant
options (especially `--detailed-json` which exposes certificate chains) is a policy
concern because it affects operator awareness of what the tool can disclose.

**Recommendation:** Route to Simone to align the French SYNOPSIS with the English original.

### C-5: Reduced SECURITY CONSIDERATIONS section

**Lines:** 249-262 (French) vs. 274-305 (English)

The English SECURITY CONSIDERATIONS section has 8 bullet points with NIST SP 800-53
control citations. The French version has only 4 bullet points and drops:

1. The atomic file creation detail ("not as a post-write chmod -- eliminating the race
   window") -- reduced to just "mode 0600"
2. The zeroizing buffer detail with NIST SP 800-53 SC-12 citation
3. The O_NOFOLLOW symlink protection detail with NIST SP 800-53 AC-3 citation
4. The config validate permission check detail
5. The journald fallback detail with NIST SP 800-53 AU-5 citation

The dropped items include three NIST control citations (SC-12, AC-3, AU-5) and two
security implementation details (race window elimination, symlink attack prevention).

**Recommendation:** This is a policy concern because security-relevant information is
missing from the French version. An operator reading only the French man page would not
know about the zeroizing buffer, symlink protection, or audit fallback behaviors. The
NIST citations are also missing, which matters for compliance documentation.

Route to Simone to restore the full security considerations content. The NIST citations
should be preserved verbatim (they are standard identifiers, not translatable prose).

### C-6: FIPS 186-5 citation missing from French SECURITY CONSIDERATIONS

**Line:** 254-255 (French) vs. 292-294 (English)

English: "Only FIPS 140-2/140-3 safe algorithms are permitted. ed25519 is excluded by
policy -- unreliable on FIPS-enabled RHEL OpenSSL providers. (NIST SP 800-53 SC-13,
FIPS 186-5)."

French: "Seuls les algorithmes conformes aux normes FIPS 140-2/140-3 sont autorisés.
ed25519 est exclu par politique."

The NIST SP 800-53 SC-13 and FIPS 186-5 citations are dropped. These are standard
identifiers that should be preserved.

**Recommendation:** Restore the control citations. They are not translatable text --
they are standard reference numbers.

### C-7: TRUST LISTS section omits C2PA trust list file names and download URLs

**Lines:** 160-178 (French) vs. 161-203 (English)

The English version names the two specific trust list files (`C2PA-TRUST-LIST.pem` and
`C2PA-TSA-TRUST-LIST.pem`), explains their distinct purposes (signing CA roots vs TSA
CA roots), provides download URLs, and includes a detailed TOML configuration block.

The French version reduces this to a generic description with a truncated TOML block
that uses different placeholder paths (`c2pa-anchors.pem` and `org-roots.pem` instead
of the actual file names). The download URLs are completely absent. The TSA timestamp
verification note ("Verifying an existing TSA timestamp is a local operation; the
internet feature is not required for this") is also dropped.

**Recommendation:** The trust list file names are proper nouns (published by the C2PA
consortium). The download URLs are operational information. Both must be preserved in
the French version. Route to Simone.

---

## ERROR Findings

### E-1: "étiquette de sécurité" used alongside "marquage de sécurité"

**Line:** 74: "étiquette de sécurité" (security label)
**Lines:** 49, 77, 257: "marquage de sécurité" (security marking)

The English original consistently uses "security marking" for the CUI marking embedded
in the C2PA manifest, and "security label" for the JSON field name in `--chain-json`
output. This is a valid distinction in English (marking = the banner text, label = the
data field).

However, the SELinux rules file (`selinux.md`) contains:

> [RULE] Use `security context` in documentation and UI, not `label` or `security label`.
> Use `sensitivity level`, not `sensitivity label`.

This rule applies to SELinux documentation specifically, not to C2PA output. But in the
French translation, line 74 uses "étiquette de sécurité" which translates to "security
label" -- a term that in the UMRS context has a specific SELinux connotation that could
confuse operators.

More importantly, the `--chain-json` output field is literally named `security-label`
in the code. The French man page should either:
1. Keep the field name in English (it is a machine identifier), or
2. Use "marquage de sécurité" consistently and note that the JSON field name is
   `security-label`

**Severity:** Medium -- terminology inconsistency between two security domains (C2PA vs
SELinux) in the same product suite could confuse operators.

**Recommended replacement:** Use "marquage de sécurité" consistently for the CUI marking
concept. When referring to the JSON field name, keep `security-label` as a code literal.

### E-2: "Système de référence MLS non classifié" for UMRS

**Line:** 40

"le Système de référence MLS non classifié (UMRS)"

This translates UMRS as "Unclassified MLS Reference System" in French. The English
original (line 37) says "Unclassified MLS Reference System (UMRS)."

There are two issues:

1. **"Non classifié" is the TBS term for "Unclassified."** In the Canadian federal
   context, "non classifié" has a specific meaning -- information that does not meet
   any Protected or Classified threshold. Using it as part of the product name implies
   a security categorization determination has been made about the system itself, which
   is not the intent. The English "Unclassified" in the product name is a descriptor of
   the MLS level the system operates at, not a Canadian security categorization.

2. **The product name "UMRS" is a proper noun.** The expansion "Unclassified MLS
   Reference System" is an English proper noun that should be kept in English, with a
   French gloss if needed. Translating proper nouns of products creates confusion --
   the acronym UMRS does not map to "SRMNS" or any French acronym.

**Severity:** Medium -- a francophone operator encountering "Système de référence MLS
non classifié" may interpret "non classifié" as a formal TBS security categorization
statement rather than a product name descriptor.

**Recommended replacement:** Keep the English expansion on first use:
"Unclassified MLS Reference System (UMRS)" with no translation. Or use a brief French
gloss: "le projet UMRS (Unclassified MLS Reference System)". The product name is not
subject to OLA translation requirements -- it is a proper noun.

### E-3: "à l'épreuve de la falsification" for "tamper-evident"

**Line:** 82

"assertion ... à l'épreuve de la falsification" (tamper-proof) for English "tamper-evident
assertion" (line 84).

"À l'épreuve de la falsification" means "tamper-proof" or "falsification-proof." The
English says "tamper-evident," which is a weaker and more accurate claim. A tamper-evident
assertion does not prevent tampering -- it *reveals* tampering. A C2PA signed assertion
is tamper-evident: if you modify the marking, the signature verification fails, making
the tampering detectable. It does not prevent the modification from occurring.

This is a policy-significant distinction. Claiming the assertion is "tamper-proof" overstates
the security guarantee of C2PA signing. An operator reading the French man page could
believe that CUI markings cannot be modified once signed, when in fact they can be modified
but the modification will be detected.

Termium Plus entries:
- "tamper-evident" -> "à preuve d'effraction" / "inviolable" (these lean toward physical
  tamper-evidence on packaging)
- "falsification" -> "falsification" (correct term for the attack)

**Severity:** High -- overstates a security guarantee. The difference between
"detects tampering" and "prevents tampering" is load-bearing in a security context.

**Recommended replacement:** "assertion révélatrice de falsification" or "assertion
permettant de détecter toute falsification." The key concept is *detection*, not
*prevention*. Route to Simone for the most natural phrasing that preserves the
"detects, does not prevent" semantics.

---

## Typography Review

### T-1: Non-breaking spaces before colons -- CORRECT

Lines 55, 73, 77, 104, 115, 118, 144, 163, 180, 182: `\~:` used correctly throughout.
The troff `\~` escape produces a non-breaking space. Correct per French typography rules
and i18n_l10n_rules.md.

### T-2: Guillemets with non-breaking spaces -- CORRECT

Line 77: `<<\~CUI\~>>` rendered as `\(lq\~CUI\~\(rq` -- actually rendered as
`<<\~CUI\~>>` with the `p.\~ex.` abbreviation. The guillemets with non-breaking
spaces are correctly implemented.

### T-3: Abbreviation "p. ex." with non-breaking space -- CORRECT

Lines 77, 257: `p.\~ex.` -- correct. The non-breaking space prevents line break
within the abbreviation.

---

## Five Eyes Interoperability Assessment

### Overall: LOW RISK with caveats

The French man page would not cause significant confusion for a francophone Five Eyes
operator, with the following caveats:

1. **E-3 (tamper-evident vs tamper-proof)** is the highest-risk item. An operator in a
   Five Eyes context relying on the French documentation to understand security guarantees
   would overestimate the protection provided by C2PA signing. This must be corrected.

2. **C-5 (reduced security considerations)** means a French-reading operator gets less
   security-relevant information than an English-reading operator. In a Five Eyes context
   where operators may be assigned to either English or French documentation based on
   language preference, this creates an information asymmetry.

3. **E-2 (UMRS product name translation)** is low risk for Five Eyes because Five Eyes
   partners would use the English name "UMRS." But on a Canadian fr_CA system, the
   translated name could cause confusion about whether a categorization determination
   has been made.

4. **Trust labels and status tags in English** -- this is correct per D3/D4 recommendation
   and poses no Five Eyes interoperability risk.

---

## Terminology Divergence Log

Per rules, all terminology decisions diverging from Termium Plus must be documented.

| Term | Termium Plus | Translation Used | Divergence? | Rationale |
|------|-------------|-----------------|-------------|-----------|
| chain of custody | chaîne de possession | chaîne de possession | No | Termium Plus and OQLF agree |
| credential (PKI) | justificatif d'identité | justificatif d'identité | No (but imprecise) | Closest Termium Plus match; see C-1 |
| trust anchor | ancre de confiance | ancre de confiance | No | Termium Plus standard |
| tamper-evident | à preuve d'effraction | à l'épreuve de la falsification | YES | See E-3. Translation means "tamper-proof," not "tamper-evident." Policy-significant. |
| crate (Rust) | No entry | caisse | YES | No Termium Plus or OQLF entry. Level 5 decision. See C-3. |
| security marking | marquage de sécurité | marquage de sécurité | No | Termium Plus standard |
| security label | étiquette de sécurité | étiquette de sécurité | No (but context collision) | See E-1 |
| self-signed certificate | certificat auto-signé | certificat auto-signé | No | Termium Plus standard |

---

## Remediation Owner Summary

| Priority | Finding | Owner | Action |
|----------|---------|-------|--------|
| 1 | E-3: tamper-evident vs tamper-proof | Simone (linguistic) + Henri (policy validation) | Correct "à l'épreuve de la falsification" to convey detection, not prevention |
| 2 | C-5: Reduced security considerations | Simone | Restore all 8 bullet points and NIST citations from English original |
| 3 | E-1: étiquette vs marquage inconsistency | Simone + Henri | Standardize on "marquage de sécurité" for CUI markings |
| 4 | E-2: UMRS product name translation | Jamie (decision) | Decide whether product name expansion should be kept in English |
| 5 | C-4: Missing SYNOPSIS options | Simone | Align French SYNOPSIS with English original |
| 6 | C-6: Missing NIST citations | Simone | Restore SC-13, FIPS 186-5, SC-12, AC-3, AU-5 citations |
| 7 | C-7: Missing trust list details | Simone | Restore file names, download URLs, TSA note |
| 8 | C-3: "caisse" for crate | Jamie (decision) | Decide: keep "caisse", use "crate" as English, or other |
| 9 | C-1: justificatif d'identité | Simone | Assess connotation; consider "certificat et clé de signature" |
| 10 | C-2: chaîne de possession | None | Confirmed correct. No action needed. |

---

## Strengths Worth Preserving

1. **Consistent use of "vous" form** -- TBS-compliant throughout.
2. **Correct date format** -- "1er avril 2026" follows TBS French date convention exactly.
3. **Infinitive mood for headings** -- subcommand descriptions correctly use infinitive
   per TBS s5.1.1.
4. **Non-breaking space typography** -- `\~:` used consistently and correctly before colons.
5. **CUI markings kept in English** -- correct policy decision; CUI is US controlled vocabulary.
6. **Trust status labels kept in English** -- consistent with D3/D4 recommendation.
7. **NIST/FIPS standard identifiers kept in English** -- correct; these are never translated.
8. **Guillemets with non-breaking spaces** -- correct French quotation mark usage.
9. **"marquage de sécurité"** -- correct Termium Plus term for the primary concept.
10. **Algorithm table preserved** -- cryptographic identifiers correctly left untranslated.

---

## Routing

- **E-1, E-2, E-3:** Route to Jamie for decision, then Simone for implementation
- **C-1 through C-7:** Route to Simone for implementation; C-3 and C-4 through Jamie first
- **All policy findings** confirmed within Henri's domain; none cross into security control
  territory (Herb/Knox)
