# TBS Canada.ca Content Style Guide -- Policy Analysis

**Date:** 2026-03-28
**Source files:**
- `.claude/references/tbs-canada-style/canada-content-style-guide-en.md` (3,101 lines)
- `.claude/references/tbs-canada-style/canada-content-style-guide-fr.md` (3,007 lines)
**Authority:** Treasury Board of Canada Secretariat, Directive on the Management of Communications

## Purpose of This Analysis

The TBS Canada.ca Content Style Guide is the closest Canadian federal equivalent to
Simplified Technical English (STE) for French procedural content. This document records
Henri's policy-level findings for Simone's translation work and the broader UMRS bilingual
documentation effort.

---

## 1. Plain Language Rules Parallel to STE Mode

### Sentence Length

- EN: "break up long sentences (optimal is under 15 to 20 words)" (s2.6)
- FR: "decoupez les phrases longues (longueur optimale de moins de 20 mots)" (s2.6)

**Alignment with STE mode:** Our STE mode rule is 20 words max, 10-15 preferred. The TBS
guide says "under 15 to 20 words" in EN and "less than 20 words" in FR. These are
compatible. The FR version is slightly more permissive (it says "less than 20" without
the "15 to" qualifier), which reflects French's natural tendency toward longer syntactic
structures.

### Active Voice

- EN: "Use the active voice" (s2.4.1)
- FR: "Ecrire a la voix active" (s2.4.1)

Both versions mandate active voice. The FR version provides clear examples of active vs
passive rewrites in Canadian French specifically, which is valuable for Simone.

### One Idea Per Sentence / Per Bullet

- EN: "keep sentences to one idea each" (s2.6)
- FR: "limitez-vous a une (1) idee par phrase" (s2.6)

Identical rule. Aligns with STE "one action per step."

### Verbs Over Nominalizations

- EN: "Use verbs instead of nouns formed from verbs" (s2.4.4)
- FR: "Utiliser des verbes plutot que des noms formes a partir de verbes" (s2.4.4)

Both versions provide language-specific examples. The FR examples are directly usable:
- "recommander" not "formuler une recommandation"
- "distribuer" not "faire la distribution"
- "modifier" not "apporter une modification"

### Positive Form

Both versions mandate positive form over negative. Exception: serious/fatal consequences
may use negative form for emphasis.

---

## 2. French-Specific Procedural Writing Guidance

### CRITICAL FINDING: Infinitive Mood for Headings in French

FR section 5.1.1 states:

> "Privilegiez un ton neutre et l'utilisation de l'infinitif lorsque vous redigez
> des titres et des sous-titres."

Translation: "Prefer a neutral tone and the use of the infinitive when writing
titles and subtitles."

**This is the single most important FR-specific rule for UMRS procedural documentation.**

In French, the infinitive mood ("Configurer SELinux") is preferred over the imperative
("Configurez SELinux") for headings and subheadings per TBS guidance. This differs from
the English version, which simply says to use "sentence case" and make titles descriptive
without specifying mood.

**Impact on UMRS:** All French headings in procedural documentation should use infinitive
form. This aligns with standard Canadian federal practice and differs from France French
documentation conventions, which sometimes use the imperative.

### Imperative Mood in List Items (Procedure Steps)

FR section 5.2 provides list writing guidance with this example:

> "si vous utilisez le mode imperatif (commandement) dans le premier element de votre
> liste, reprenez ce mode dans chaque element subsequent"

This means: within a procedural numbered list, use imperative consistently. But the
list's *heading* uses infinitive.

**Pattern for UMRS French procedures:**
- Heading: Infinitive ("Configurer le mode SELinux")
- Steps: Imperative ("Ouvrez le fichier...", "Enregistrez le fichier...")

### Second Person (vous) for Addressing Users

FR section 3.1 mandates "vous" (second person plural / formal) for addressing users.
Exception: "tu" is permitted only for youth campaigns.

For UMRS CLI/TUI output, this means error messages and prompts in French should use
"vous" consistently.

### Obligation vs Recommendation Vocabulary

FR section 3.3:
- Obligation: "devez" (must) -- not "etes legalement tenus de"
- Recommendation/Permission: "pourriez" or "pouvez" (may/might/can)

The EN version distinguishes between "must" (legal) and "need to" (administrative).
The FR version collapses this into "devez" for both. This is a simplification that
Simone should be aware of -- if we need to distinguish legal from administrative
obligation in French, we cannot rely on the TBS verb guidance alone and may need
explicit context.

---

## 3. Accessibility Requirements Affecting TUI/CLI Output

### Screen Reader Compatibility

Both versions emphasize screen reader support. Key rules:
- Avoid all-caps except for abbreviations and military operation names (s4.1)
- Avoid italics for emphasis (accessibility concern for dyslexia)
- Use bold sparingly
- Underline only for links
- Never rely on colour alone to convey information

**Impact on UMRS TUI:** Already aligned with our NO_COLOR policy. The TBS all-caps
prohibition reinforces that UMRS should not use all-caps for emphasis in CLI output
(including French output).

### Hyphen/Dash and Screen Reader Behavior

FR section 4.1.4 notes that screen readers often skip hyphens in year ranges, making
"2024-2025" hard to parse audibly. The FR guide recommends "de 2024 a 2025" or
"exercice 2024 a 2025" instead.

EN section 4.1.5 allows en dashes in fiscal year ranges as an exception, but also
prefers "to" in most contexts.

**Impact on UMRS:** Any date ranges in CLI/TUI output should use words ("to"/"a")
rather than dashes, for both accessibility and bilingual consistency.

### Table Accessibility

Both versions provide extensive table accessibility guidance:
- Always include column/row headers
- Simplest possible structure
- No blank cells without explanation
- No reliance on color/texture alone

---

## 4. Bilingual Content Presentation Rules

### Official Languages Act Compliance (s1.3)

Content must:
- Be professionally translated
- Reflect Canadian writing conventions in English AND French
- Include fully bilingual images, multimedia, and transcripts

The phrase "reflect Canadian writing conventions" is significant. This means:
- Federal Canadian French, not France French or Quebec French (for federal content)
- Canadian English, not American English

### Alphabetical Lists (s5.2.4)

"Si vous presentez du contenu francais par ordre alphabetique, presentez aussi le
contenu traduit en anglais par ordre alphabetique pour offrir la meme experience
intuitive."

Each language version must independently make sense -- do not just mirror the order
of the other language. If items are alphabetical in French, they must be re-sorted
alphabetically in English, and vice versa.

### Words in Transition (s4.11)

EN version standardizes: "website", "web page", "web", "email", "online"
FR version notes: "Cette regle s'applique en anglais plutot qu'en francais."
The FR guide defers to Cles de la redaction for French equivalents.

---

## 5. EN/FR Divergences Simone Should Know

### 5.1 Heading Mood (Most Important)

- EN: No specific mood requirement for headings
- FR: INFINITIVE required for headings and subheadings

This is a structural difference, not just a translation difference. Simone must not
simply translate imperative English headings to imperative French headings. The French
headings must be converted to infinitive form.

### 5.2 Contractions (s4.5)

- EN: Contractions encouraged ("you'll", "we've", "don't")
- FR: "La forme contractee en francais s'applique par defaut." (Contractions are
  the default in French -- articles and prepositions contract automatically: l', d', qu')

No action required -- French contractions are grammatically mandatory, not stylistic.

### 5.3 Number Formatting

- EN: "$100" (dollar sign before number, no space)
- FR: "100 $" (number before dollar sign, with non-breaking space)
- EN: "20%" (no space before percent)
- FR: "20 %" (non-breaking space before percent sign)
- EN: "July 31, 2016"
- FR: "31 juillet 2016" (no comma between date and year)
- EN: "4:30 pm"
- FR: "16 h 30" (24-hour format with "h" separator)
- EN ordinals: "10th, 50th" (same text size)
- FR ordinals: "2e, 400e" (superscript e/es/er)

These formatting differences are well-known but must be consistently applied in any
UMRS bilingual output.

### 5.4 Inclusivity Approach

- EN: "Make gender-inclusive writing your standard practice." References English
  inclusive writing guidelines.
- FR: "La redaction epicene devrait etre votre pratique habituelle." Uses the term
  "redaction epicene" (epicene writing) which is the federal Canadian French standard
  for gender-inclusive language.

This is important: the federal French standard uses "redaction epicene," NOT "ecriture
inclusive" in the OQLF sense or the more radical typographic forms (like middot or
point median). Simone should use epicene forms per the Bureau de la traduction guidance,
not Quebec-specific inclusive writing conventions.

### 5.5 FR-Specific List Punctuation

FR section 4.1.3 on lists:
- Dependent list items (completing a lead-in sentence) begin lowercase
- Independent list items begin uppercase
- No period, exclamation, or question mark at end of list items

Same as EN in principle, but the FR guide is more explicit about when to use
lowercase vs uppercase based on grammatical dependency.

### 5.6 Noun Strings

- EN s2.4.5 explicitly warns against noun strings ("Food Animal Carcass Post-mortem
  Evaluation Standards")
- FR version omits this subsection -- noun strings are not a structural problem in
  French due to the language's use of prepositions ("normes d'evaluation post-mortem
  des carcasses d'animaux de boucherie")

---

## 6. Policy Findings

### FINDING: FR Heading Mood Convention

```
FINDING: French headings must use infinitive mood per TBS
SEVERITY: Medium
DOMAIN: Canadian Policy
SOURCE: TBS Canada.ca Content Style Guide FR, section 5.1.1
DETAIL: The TBS guide mandates infinitive mood ("Configurer") not imperative
  ("Configurez") for French headings and subheadings. If UMRS French documentation
  uses imperative headings, it does not conform to TBS web content standards. This
  is not a linguistic error -- it is a policy-level convention that carries regulatory
  weight under the Directive on the Management of Communications.
REMEDIATION: Simone should use infinitive headings in all French procedural content.
  STE mode should note that while English procedure headings may use imperative,
  French procedure headings must use infinitive per TBS.
```

### FINDING: Epicene Writing Standard for Federal French

```
FINDING: Federal French uses epicene writing, not Quebec inclusive writing
SEVERITY: Low
DOMAIN: Canadian Policy
SOURCE: TBS Canada.ca Content Style Guide FR, section 1.2
DETAIL: The TBS guide uses "redaction epicene" as the standard for gender-inclusive
  French, referencing Bureau de la traduction guidelines. This is distinct from
  OQLF's "ecriture inclusive" conventions and from France's typographic approaches.
  For UMRS French content, the federal epicene standard applies.
REMEDIATION: Informational. Simone should follow Bureau de la traduction epicene
  guidelines, not OQLF inclusive writing conventions, for UMRS French documentation.
```

### FINDING: Obligation Vocabulary Divergence EN/FR

```
FINDING: EN must/need-to distinction collapses in FR
SEVERITY: Informational
DOMAIN: Canadian Policy
SOURCE: TBS Canada.ca Content Style Guide EN s3.3 vs FR s3.3
DETAIL: The EN version distinguishes "must" (legal obligation) from "need to"
  (administrative requirement). The FR version uses "devez" for both, losing the
  distinction. If UMRS needs to distinguish legal from administrative requirements
  in French output, additional context must be provided because the verb alone
  is insufficient.
REMEDIATION: No immediate action. Flag to Jamie if UMRS French output needs to
  distinguish legal vs administrative obligations.
```

---

## 7. Applicability to UMRS

The TBS guide is mandatory for "public-facing websites and digital services" under the
Directive on the Management of Communications. UMRS is a reference system, not a
deployed GC service. Therefore:

- The TBS guide is **advisory** for UMRS, not mandatory.
- However, following TBS conventions positions UMRS French documentation correctly
  for any future federal deployment.
- The infinitive heading rule, number formatting, and epicene writing standards are
  the most actionable items for Simone.
- The plain language rules reinforce and are compatible with our existing STE mode.

## 8. Terminology Hierarchy Note

This analysis identified one Termium Plus vs OQLF divergence area:

- "Redaction epicene" (Termium/Bureau de la traduction) vs "ecriture inclusive" (OQLF)

Per the Terminology Decision Hierarchy, Termium Plus (Level 1) governs. The OQLF term
is noted but does not override for federal material. This is documented, not silently
resolved.
