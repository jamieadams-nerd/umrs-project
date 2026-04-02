# umrs-c2pa String Inventory — Wrapping Report

Crate: umrs-c2pa
Domain: umrs-c2pa
Date: 2026-04-01
Author: umrs-translator (Simone)

---

## Summary

Total unwrapped user-facing strings: **74**

| File | Count |
|---|---|
| `src/main.rs` | 36 |
| `src/c2pa/report.rs` | 17 |
| `src/c2pa/validate.rs` | 16 |
| `src/c2pa/creds.rs` | 5 |
| `src/c2pa/error.rs` (thiserror) | 5 (see note) |
| `src/c2pa/manifest.rs` | 2 (TrustStatus display strings) |
| `src/c2pa/ingest.rs` | 0 (user-facing: 0; verbose only) |
| `src/c2pa/trust.rs` | 0 (verbose only) |
| `src/c2pa/signer.rs` | 6 (describe_algorithm) |

---

## Notes Before Reading This Report

**1. error.rs — thiserror macros.**
The `#[error("...")]` attribute strings in `error.rs` are rendered by the `thiserror` crate into
`Display` implementations. These strings surface to the user via `.context()` wrapping and
`eprintln!` calls. They need gettext wrapping, but the wrapping strategy is non-obvious — `thiserror`
does not integrate with gettext directly. The developer will need to implement custom `Display` impls
that call `gettext()`. Flag these to the developer as a design decision before wrapping.

**2. validate.rs — ValidationResult messages.**
`ValidationResult::message` strings are user-facing (they are printed by `print_validation_report`),
but they are constructed in library code that returns structured data. The wrapping must happen at
construction time inside `validate.rs`, not at display time in `report.rs`. The developer should
be aware that string context (the `check` field) must remain unwrapped — it is a machine identifier,
not display text.

**3. creds.rs — GeneratedCredentials::summary.**
The `summary` field is a multi-line `format!()` string that is passed back to `main.rs` and printed
with `println!("{}", result.summary)`. This is a composite string with embedded interpolated values.
The developer will need to restructure this into individual translated segments with format arguments,
or replace it with a structured type. Flag as a design decision.

**4. TrustStatus display strings — policy-critical.**
`TRUSTED`, `UNVERIFIED`, `INVALID`, `REVOKED`, `NO TRUST LIST` in `manifest.rs` are security
classification tags. They require `msgctxt` in the `.po` file to prevent generic translation tools
from substituting policy-incorrect terms. These must be reviewed by Henri before any translation
is approved.

**5. Layout-sensitive strings.**
Several strings in `report.rs` and `main.rs` appear inside fixed-width column layouts using
`{:<pad$}` format specifiers. French text runs 20–30% longer — these strings are flagged with
[LAYOUT-SENSITIVE] and require width review before translation is attempted.

**6. verbose! macro strings.**
Verbose strings go to stderr via the `verbose!()` macro. Per project rules, these are
user-visible (operator progress messages). They are included in this inventory. They are
lower priority than CLI output and error messages because they are hidden by default
(`--verbose` required). Discussed further in the Priority section at the end.

---

## File: `src/main.rs`

### Unwrapped strings

---

**Entry 1**
```
file: src/main.rs
line: 147
string: "Failed to load config: {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in .with_context(). Requires custom context wrapper pattern.
```

---

**Entry 2**
```
file: src/main.rs
line: 149
string: "No config file at {} — using defaults"
classification: VERBOSE_MSG
interpolated: yes — file path
macro to use: verbose! wrapping gettext()
```

---

**Entry 3**
```
file: src/main.rs
line: 155
string: "Failed to connect to journald"
classification: ERROR_MSG (panic message via .expect())
interpolated: no
macro to use: gettext()
notes: .expect() panic message — visible on crash, not normal operation.
  Lower priority than interactive output.
```

---

**Entry 4**
```
file: src/main.rs
line: 158
string: "Failed to initialize journald logger"
classification: ERROR_MSG (panic message via .expect())
interpolated: no
macro to use: gettext()
notes: Same as above — panic path.
```

---

**Entry 5**
```
file: src/main.rs
line: 227
string: "File not found: {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in anyhow::bail!(). Surfaces to user via eprintln! in the caller chain.
```

---

**Entry 6**
```
file: src/main.rs
line: 234
string: "Reading detailed manifest store as JSON (includes cert chains)..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 7**
```
file: src/main.rs
line: 236
string: "Reading raw manifest store as JSON..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 8**
```
file: src/main.rs
line: 241
string: "No manifest or read error: {e}"
classification: CLI_OUTPUT (eprintln!)
interpolated: yes — error value
macro to use: gettext() with format argument
notes: This is the primary error path when --json or --detailed-json is used.
  The `{e}` portion should not be translated — only the surrounding message.
  Suggest: eprintln!("{} {e}", gettext("No manifest or read error:"))
```

---

**Entry 9**
```
file: src/main.rs
line: 248
string: "Reading chain of custody as JSON..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 10**
```
file: src/main.rs
line: 251
string: "No manifest or read error: {e}"
classification: CLI_OUTPUT (eprintln!)
interpolated: yes — error value
macro to use: gettext() with format argument
notes: Duplicate of entry 8 — same message, same treatment. One msgid for both.
```

---

**Entry 11**
```
file: src/main.rs
line: 255
string: "Computing SHA-256 digest..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 12**
```
file: src/main.rs
line: 257
string: "Failed to hash file: {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in .with_context().
```

---

**Entry 13**
```
file: src/main.rs
line: 258
string: "SHA-256: {}"
classification: VERBOSE_MSG
interpolated: yes — hash value
macro to use: verbose! wrapping gettext()
```

---

**Entry 14**
```
file: src/main.rs
line: 261
string: "Signing mode — ingesting file into UMRS chain of custody..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 15**
```
file: src/main.rs
line: 263
string: "Security marking: {}"
classification: VERBOSE_MSG
interpolated: yes — marking string
macro to use: verbose! wrapping gettext()
```

---

**Entry 16**
```
file: src/main.rs
line: 268
string: "Ingest failed for: {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in .with_context().
```

---

**Entry 17**
```
file: src/main.rs
line: 269
string: "Signed output written to: {}"
classification: VERBOSE_MSG
interpolated: yes — file path
macro to use: verbose! wrapping gettext()
```

---

**Entry 18**
```
file: src/main.rs
line: 271
string: "Reading chain of custody from signed output..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 19**
```
file: src/main.rs
line: 273
string: "Failed to read chain from signed output"
classification: ERROR_MSG
interpolated: no
macro to use: gettext()
notes: Used in .with_context().
```

---

**Entry 20**
```
file: src/main.rs
line: 274
string: "Chain contains {} entries"
classification: VERBOSE_MSG
interpolated: yes — integer count
macro to use: verbose! wrapping gettext() — consider ngettext for plural
notes: French pluralization rule: nplurals=2; plural=(n > 1). Zero is singular in French.
  This string is a candidate for ngettext("Chain contains {} entry", "Chain contains {} entries", n).
```

---

**Entry 21**
```
file: src/main.rs
line: 278
string: "Read-only mode — inspecting existing chain of custody..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 22**
```
file: src/main.rs
line: 282
string: "Failed to read chain from: {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in .with_context().
```

---

**Entry 23**
```
file: src/main.rs
line: 283
string: "Chain contains {} entries"
classification: VERBOSE_MSG
interpolated: yes — integer count
macro to use: verbose! wrapping ngettext()
notes: Same msgid as entry 20 — same plural handling applies.
```

---

**Entry 24**
```
file: src/main.rs
line: 294
string: "Running configuration preflight checks..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 25**
```
file: src/main.rs
line: 296
string: "{} checks completed"
classification: VERBOSE_MSG
interpolated: yes — integer count
macro to use: verbose! wrapping ngettext()
notes: Candidate for plural: "{} check completed" / "{} checks completed"
```

---

**Entry 26**
```
file: src/main.rs
line: 312
string: "Failed to write config to: {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in .with_context().
```

---

**Entry 27**
```
file: src/main.rs
line: 314
string: "Config template written to: {}"
classification: CLI_OUTPUT (println!)
interpolated: yes — file path
macro to use: gettext() with format argument
```

---

**Entry 28**
```
file: src/main.rs
line: 421
string: "Generating credentials in: {}"
classification: VERBOSE_MSG
interpolated: yes — directory path
macro to use: verbose! wrapping gettext()
```

---

**Entry 29**
```
file: src/main.rs
line: 426
string: "Failed to create directory: {}"
classification: ERROR_MSG
interpolated: yes — directory path
macro to use: gettext() with format argument
notes: Used in .with_context().
```

---

**Entry 30**
```
file: src/main.rs
line: 430
string: "Credential generation failed"
classification: ERROR_MSG
interpolated: no
macro to use: gettext()
notes: Used in .with_context().
```

---

**Entry 31**
```
file: src/main.rs
line: 443
string: "{} already exists at {}. Remove it first or choose a different --output directory."
classification: ERROR_MSG
interpolated: yes — filename, path
macro to use: gettext() with format arguments
notes: Used in anyhow::bail!(). The option name "--output" is a CLI flag — keep untranslated.
```

---

**Entry 32**
```
file: src/main.rs
line: 449
string: "signing.key already exists at {}. Remove it first or choose a different --output directory."
classification: ERROR_MSG
interpolated: yes — path
macro to use: gettext() with format argument
notes: "signing.key" is a filename — keep untranslated. "--output" is a CLI flag — keep untranslated.
```

---

**Entry 33**
```
file: src/main.rs
line: 456
string: "Failed to write {}"
classification: ERROR_MSG
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Used in .with_context(). Two instances (lines 456 and 458) share the same msgid.
```

---

**Entry 34**
```
file: src/main.rs
line: 469
string: "Files written:"
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: This is a section header above two indented lines.
  French expansion of the label may require review if alignment is added later.
```

---

**Entry 35**
```
file: src/main.rs
line: 473
string: "Next step — add these to your umrs-c2pa.toml:"
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: "umrs-c2pa.toml" is a filename — keep untranslated within the translated string.
```

---

**Entry 36**
```
file: src/main.rs
line: 477
string: "  # After your CA signs the CSR, replace signing.csr with the signed cert:"
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: Inline TOML comment presented as guidance. "signing.csr" and "signing.pem" are filenames.
```

---

**Entry 37**
```
file: src/main.rs
line: 486
string: "Then run: umrs-c2pa creds validate"
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: "umrs-c2pa creds validate" is a command invocation — keep untranslated within the string.
  Suggest: format!("{} umrs-c2pa creds validate", gettext("Then run:"))
```

---

**Entry 38**
```
file: src/main.rs
line: 494
string: "Validating configured signing credentials..."
classification: VERBOSE_MSG
interpolated: no
macro to use: verbose! wrapping gettext()
```

---

**Entry 39**
```
file: src/main.rs
line: 501
string: "Credential Validation"
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: This is a section header. The Unicode separator line on line 502 is
  56 characters wide (counted by repeat of U+2501). French translation of "Credential Validation"
  is approximately 24–28 characters — within the 56-char box. Low risk, but note for review.
```

---

**Entry 40**
```
file: src/main.rs
line: 517
string: "{failures} check(s) failed."
classification: CLI_OUTPUT (println!)
interpolated: yes — integer count
macro to use: gettext() / ngettext()
notes: The "(s)" pattern is an English pluralization shortcut — not acceptable in French.
  Must use ngettext("1 check failed.", "{n} checks failed.", n).
  French: "1 vérification a échoué." / "{n} vérifications ont échoué."
```

---

**Entry 41**
```
file: src/main.rs
line: 519
string: "To generate new credentials: umrs-c2pa creds generate --output /path/to/certs/"
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: The command "umrs-c2pa creds generate --output /path/to/certs/" should be
  preserved untranslated as a code example.
```

---

**Entry 42**
```
file: src/main.rs
line: 522
string: "All checks passed."
classification: CLI_OUTPUT (println!)
interpolated: no
macro to use: gettext()
```

---

---

## File: `src/c2pa/report.rs`

### Unwrapped strings

---

**Entry 43**
```
file: src/c2pa/report.rs
line: 44
string: "\nChain of Custody — {path}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — file path
macro to use: gettext() with format argument
notes: The em-dash " — " is French-compatible (French also uses em-dash as a separator).
  The leading "\n" is a formatting artifact — do not include in the msgid.
  Suggest splitting: println!("\n{} — {path}", gettext("Chain of Custody"))
[LAYOUT-SENSITIVE]: Section header. French "Chaîne de traçabilité" is longer than
  "Chain of Custody" (19 vs 17 chars). Low risk given no fixed width here, but flag.
```

---

**Entry 44**
```
file: src/c2pa/report.rs
line: 45
string: "SHA-256: {sha256}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — hash string
macro to use: gettext() with format argument
notes: "SHA-256" is a technical designator — do not translate the algorithm name.
  Suggest: format!("{} {sha256}", gettext("SHA-256:"))
```

---

**Entry 45**
```
file: src/c2pa/report.rs
line: 49
string: "  (no C2PA manifest found)"
classification: REPORT_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: "C2PA" is a proper name — do not translate.
```

---

**Entry 46**
```
file: src/c2pa/report.rs
line: 66
string: "Self-signed certificate — not issued by a trusted CA"
classification: REPORT_OUTPUT (footnote string inserted into BTreeMap)
interpolated: no
macro to use: gettext()
notes: This string is a footnote explanation stored in a BTreeMap and printed later.
  The wrapping must happen at insertion time, not at print time. The BTreeMap key
  (the trust label) is derived from TrustStatus::Display — see entry 75.
```

---

**Entry 47**
```
file: src/c2pa/report.rs
line: 74
string: "No trust list configured — trust could not be evaluated"
classification: REPORT_OUTPUT (footnote string)
interpolated: no
macro to use: gettext()
notes: Same as entry 46 — wrapping at insertion time.
```

---

**Entry 48**
```
file: src/c2pa/report.rs
line: 89
string: "Signed at : {} UTC"
classification: REPORT_OUTPUT (println!)
interpolated: yes — timestamp string
macro to use: gettext() with format argument
[LAYOUT-SENSITIVE]: "Signed at :" is a field label in a tabular layout at column position ~22.
  French: "Signé le :" — comparable width. Note the French colon rule requires a non-breaking
  space before the colon: "Signé le\u00a0:". The developer must accommodate this in the format string.
  Maximum label width here is governed by the pad variable (14 or 16 chars). Review needed.
```

---

**Entry 49**
```
file: src/c2pa/report.rs
line: 90
string: "Signed at : no timestamp provided"
classification: REPORT_OUTPUT (println!)
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: Same column as entry 48. Same non-breaking space colon rule applies.
```

---

**Entry 50**
```
file: src/c2pa/report.rs
line: 95
string: "Issuer    : {}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — issuer name
macro to use: gettext() with format argument
[LAYOUT-SENSITIVE]: "Issuer    :" is a field label padded to column alignment. French: "Émetteur :"
  — 8 chars + colon vs 6 chars + colon. May disrupt column alignment.
  The developer should use a fixed field-width constant for all labels.
  Non-breaking space required before French colon.
```

---

**Entry 51**
```
file: src/c2pa/report.rs
line: 98
string: "Alg       : {}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — algorithm string
macro to use: gettext() with format argument
[LAYOUT-SENSITIVE]: "Alg" is a technical abbreviation. Recommend keeping "Alg" untranslated
  (it refers to "Algorithm" — a proper technical term). Alternatively: "Algo      :"
  French uses "algorithme" → "Algo" is an acceptable abbreviation in fr_CA technical contexts.
  Henri must confirm. Non-breaking space before colon required.
```

---

**Entry 52**
```
file: src/c2pa/report.rs
line: 105
string: "Generator : {}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — generator name + version
macro to use: gettext() with format argument
[LAYOUT-SENSITIVE]: "Generator" (9 chars) vs French "Générateur" (10 chars + possible abbreviation).
  Column alignment at risk. Non-breaking space before colon.
```

---

**Entry 53**
```
file: src/c2pa/report.rs
line: 109
string: "Marking   : {}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — security marking string
macro to use: gettext() with format argument
notes: "Marking" refers to a CUI/security marking. Henri must confirm the French
  term. Candidate: "Marquage  :" (TERMIUM lookup required — do not translate without corpus check).
[LAYOUT-SENSITIVE]: Non-breaking space before French colon required.
```

---

**Entry 54**
```
file: src/c2pa/report.rs
line: 129
string: "Hash consistency : PASS — file unchanged across all signing events"
classification: REPORT_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: "PASS" is a status indicator — see discussion on policy-critical terms below.
  PASS/FAIL should use msgctxt to prevent colloquial translation.
[LAYOUT-SENSITIVE]: Long string in a loose tabular layout. French will be longer.
  Max expansion: "Cohérence du hachage : PASS — fichier inchangé pour tous les événements de signature"
  Approximately 40% longer. The developer must allow for wrapping or wider display.
```

---

**Entry 55**
```
file: src/c2pa/report.rs
line: 131
string: "Hash consistency : N/A  — no prior manifest (first signature)"
classification: REPORT_OUTPUT (println!)
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: Same column as entry 54. "N/A" is an international abbreviation — keep as-is.
```

---

**Entry 56**
```
file: src/c2pa/report.rs
line: 133
string: "UMRS action      : {}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — C2PA action label string (e.g. "c2pa.acquired")
macro to use: gettext() with format argument
notes: The action label (the interpolated value) is a C2PA spec identifier — do not translate.
  Only "UMRS action" is the translatable label.
[LAYOUT-SENSITIVE]: Colon alignment with entries 54–55.
```

---

**Entry 57**
```
file: src/c2pa/report.rs
line: 134
string: "UMRS output      : {}"
classification: REPORT_OUTPUT (println!)
interpolated: yes — file path
macro to use: gettext() with format argument
[LAYOUT-SENSITIVE]: Colon alignment with entries 54–56.
```

---

**Entry 58**
```
file: src/c2pa/report.rs
line: 136
string: "UMRS identity    : ephemeral self-signed cert (test mode — UNTRUSTED)"
classification: REPORT_OUTPUT (println!)
interpolated: no
macro to use: gettext()
notes: "UNTRUSTED" is a security status label — requires msgctxt.
  "UMRS identity" label — colon alignment.
[LAYOUT-SENSITIVE]: Colon alignment with entries 54–57.
```

---

**Entry 59**
```
file: src/c2pa/report.rs
line: 155
string: "[OK]  "
classification: REPORT_OUTPUT (used as tag in println!)
interpolated: no
macro to use: gettext()
notes: Status tag in the validation report. Used inside format string at line 161.
  All five status tags (OK, WARN, FAIL, INFO, SKIP) must be translated consistently.
  These are not security classification labels — they are check result indicators.
  Discuss with Henri whether these should have msgctxt or are general-purpose.
[LAYOUT-SENSITIVE]: These are 6-character padded tags used for column alignment.
  Translations must not exceed 6 characters including brackets, or the column will break.
  Recommend keeping English tags with gettext providing an alternative:
  msgid "[OK]  " → msgstr "[OK]  " (may keep English status tags as international convention).
  DECISION REQUIRED before wrapping.
```

---

**Entry 60**
```
file: src/c2pa/report.rs
line: 156
string: "[WARN]"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: 6-character width. See note on entry 59.
```

---

**Entry 61**
```
file: src/c2pa/report.rs
line: 157
string: "[FAIL]"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: 6-character width. See note on entry 59.
```

---

**Entry 62**
```
file: src/c2pa/report.rs
line: 158
string: "[INFO]"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: 6-character width. See note on entry 59.
```

---

**Entry 63**
```
file: src/c2pa/report.rs
line: 159
string: "[SKIP]"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
[LAYOUT-SENSITIVE]: 6-character width. See note on entry 59.
```

---

**Entry 64**
```
file: src/c2pa/report.rs
line: 170
string: "  All checks passed ({warnings} warning(s)). Configuration is ready."
classification: REPORT_OUTPUT (println!)
interpolated: yes — integer count
macro to use: gettext() / ngettext() for the warning count
notes: The "(s)" plural shortcut is unacceptable in French. Split into two strings:
  one for singular warning, one for plural. Or use ngettext.
```

---

**Entry 65**
```
file: src/c2pa/report.rs
line: 172
string: "  All checks passed. Configuration is ready."
classification: REPORT_OUTPUT (println!)
interpolated: no
macro to use: gettext()
```

---

**Entry 66**
```
file: src/c2pa/report.rs
line: 175
string: "  {failures} check(s) failed. Configuration is NOT ready."
classification: REPORT_OUTPUT (println!)
interpolated: yes — integer count
macro to use: ngettext()
notes: Same "(s)" issue as entries 40 and 64. Must use plural forms.
  "NOT ready" — emphasis. French: "n'est PAS prête" — discuss register with Henri.
```

---

---

## File: `src/c2pa/validate.rs`

### Unwrapped strings

These are all `ValidationResult::message` strings — they surface to the user via
`print_validation_report` in `report.rs`. Wrapping must occur at the construction
site (inside `validate.rs`), not at the print site.

---

**Entry 67**
```
file: src/c2pa/validate.rs
line: 85
string: "Skipped — requires both cert and key files to be readable"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: no
macro to use: gettext()
```

---

**Entry 68**
```
file: src/c2pa/validate.rs
line: 104-106
string: "No certificate configured — ephemeral self-signed cert will be used (test mode). \
         Manifests will be marked UNTRUSTED by external validators."
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: no
macro to use: gettext()
notes: "UNTRUSTED" is a security status label — requires msgctxt.
  This is a multi-line string literal in source (backslash continuation). Treat as one msgid.
```

---

**Entry 69**
```
file: src/c2pa/validate.rs
line: 115
string: "Field is empty"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: no
macro to use: gettext()
```

---

**Entry 70**
```
file: src/c2pa/validate.rs
line: 130-132
string: "File not found: {}"
classification: REPORT_OUTPUT (ValidationResult message — cert_chain check)
interpolated: yes — file path
macro to use: gettext() with format argument
notes: Shared msgid with other "File not found: {}" instances. One msgid for all.
```

---

**Entry 71**
```
file: src/c2pa/validate.rs
line: 137
string: "Cannot read: {e}"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — error value
macro to use: gettext() with format argument
```

---

**Entry 72**
```
file: src/c2pa/validate.rs
line: 143
string: "Valid PEM at {}"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — file path
macro to use: gettext() with format argument
```

---

**Entry 73**
```
file: src/c2pa/validate.rs
line: 149
string: "File is not valid PEM: {}"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — file path
macro to use: gettext() with format argument
```

---

**Entry 74**
```
file: src/c2pa/validate.rs
line: 208-210
string: "ed25519 is not reliably available on FIPS-enabled systems. \
         Recommended: es256, es384, or es512."
classification: REPORT_OUTPUT (ValidationResult message — algorithm check)
interpolated: no
macro to use: gettext()
notes: Algorithm names (ed25519, es256, es384, es512) are technical identifiers — keep untranslated.
```

---

**Entry 75**
```
file: src/c2pa/validate.rs
line: 217
string: "'{alg}' is not allowed. Use one of: {}"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — algorithm name, allowed list
macro to use: gettext() with format arguments
notes: Algorithm names are identifiers — not translated.
```

---

**Entry 76**
```
file: src/c2pa/validate.rs
line: 234
string: "TSA endpoint reachable: {url}"
classification: REPORT_OUTPUT (ValidationResult message — internet feature)
interpolated: yes — URL
macro to use: gettext() with format argument
notes: Conditional on `#[cfg(feature = "internet")]`.
```

---

**Entry 77**
```
file: src/c2pa/validate.rs
line: 237
string: "TSA endpoint did not respond: {url} ({e})"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — URL, error
macro to use: gettext() with format arguments
```

---

**Entry 78**
```
file: src/c2pa/validate.rs
line: 242-244
string: "TSA configured ({url}) but network feature is disabled — timestamps will be unsigned"
classification: REPORT_OUTPUT (ValidationResult message — non-internet feature)
interpolated: yes — URL
macro to use: gettext() with format argument
```

---

**Entry 79**
```
file: src/c2pa/validate.rs
line: 251-253
string: "No trust lists configured — all manifests will show NO TRUST LIST. \
         See docs/trust-maintenance.md to set up trust anchors."
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: no
macro to use: gettext()
notes: "NO TRUST LIST" is a security status label — requires msgctxt.
  "docs/trust-maintenance.md" is a file path — keep untranslated within the string.
```

---

**Entry 80**
```
file: src/c2pa/validate.rs
line: 269-271
string: "File exists: {}"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — file path
macro to use: gettext() with format argument
```

---

**Entry 81**
```
file: src/c2pa/validate.rs
line: 276-278
string: "EKU config found: {}"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — file path
macro to use: gettext() with format argument
```

---

**Entry 82**
```
file: src/c2pa/validate.rs
line: 294-296
string: "OCSP responder configured: {url} (not yet implemented — skeleton only)"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — URL
macro to use: gettext() with format argument
notes: "(not yet implemented — skeleton only)" is an advisory — appropriate for translation.
```

---

**Entry 83**
```
file: src/c2pa/validate.rs
line: 320
string: "Valid PEM at {} ({} certificate(s))"
classification: REPORT_OUTPUT (ValidationResult message)
interpolated: yes — path, integer count
macro to use: gettext() / ngettext() for certificate count
notes: "(s)" plural issue again. Use ngettext:
  "Valid PEM at {} ({} certificate)" / "Valid PEM at {} ({} certificates)"
```

---

---

## File: `src/c2pa/creds.rs`

### Unwrapped strings

The `summary` field in `GeneratedCredentials` is a multi-line composite string built
with `format!()`. It is printed by `main.rs` via `println!("{}", result.summary)`.
This is a structural wrapping problem — the developer must restructure this before
gettext wrapping can be applied cleanly.

---

**Entry 84**
```
file: src/c2pa/creds.rs
line: 107-114
string: "Generated CSR + private key\n\
         Algorithm : {alg_str} (ECDSA {curve_name}, {key_bits}-bit)\n\
         Subject   : O={org}, CN={org} (UMRS C2PA Signing)\n\
         \n\
         Submit the CSR to your Certificate Authority for signing.\n\
         Keep the private key safe — it cannot be regenerated."
classification: CLI_OUTPUT (via result.summary in main.rs)
interpolated: yes — algorithm, curve name, key bits, organization name
macro to use: DESIGN DECISION REQUIRED — see below
notes: This multi-line composite string must be restructured before wrapping.
  Suggested approach: replace summary: String with a structured type, or
  print each line individually from main.rs using separate gettext calls.
  The interpolated technical values (algorithm strings, org name) must not be translated.
[LAYOUT-SENSITIVE]: Each line is a colon-aligned label-value pair. Same risk as report.rs labels.
```

---

**Entry 85**
```
file: src/c2pa/creds.rs
line: 124-133
string: "Generated self-signed certificate + private key\n\
         Algorithm : {alg_str} (ECDSA {curve_name}, {key_bits}-bit)\n\
         Subject   : O={org}, CN={org} (UMRS C2PA Signing — self-signed)\n\
         Validity  : {validity_days} days from now\n\
         \n\
         Self-signed certificates will show as UNVERIFIED by external\n\
         validators. For trusted status, submit a CSR to a recognized CA\n\
         or add your org's root to the trust anchors."
classification: CLI_OUTPUT (via result.summary in main.rs)
interpolated: yes — algorithm, curve, bits, org, validity days
macro to use: DESIGN DECISION REQUIRED — same as entry 84
notes: "UNVERIFIED" is a security status label — requires msgctxt.
  "days" is a plural candidate: ngettext("{} day from now", "{} days from now", n).
[LAYOUT-SENSITIVE]: Same colon-alignment issue as entry 84.
```

---

**Entry 86**
```
file: src/c2pa/creds.rs
line: 157-160
string: "No cert_chain or private_key configured. \
         Run `inspect creds generate` to create them, \
         then set the paths in umrs-c2pa.toml."
classification: CLI_OUTPUT (printed via creds validate command)
interpolated: no
macro to use: gettext()
notes: Backtick-wrapped command "inspect creds generate" should remain untranslated.
  "umrs-c2pa.toml" is a filename — keep untranslated.
```

---

**Entry 87**
```
file: src/c2pa/creds.rs
line: 164
string: "cert_chain is set but private_key is missing"
classification: CLI_OUTPUT (CredCheck message)
interpolated: no
macro to use: gettext()
notes: "cert_chain" and "private_key" are TOML field names — keep untranslated as identifiers.
  Only the surrounding message text is translated.
```

---

**Entry 88**
```
file: src/c2pa/creds.rs
line: 170
string: "private_key is set but cert_chain is missing"
classification: CLI_OUTPUT (CredCheck message)
interpolated: no
macro to use: gettext()
notes: Same as entry 87.
```

---

---

## File: `src/c2pa/error.rs`

### thiserror display strings — design decision required

These `#[error("...")]` strings are rendered by `thiserror` into `Display` impls.
They surface to users via `.context()` chain and `eprintln!` calls. Standard
gettext wrapping cannot be applied directly to `#[error]` attributes.

**Required developer action before wrapping:**
The developer must replace `#[error]` attributes with custom `Display` implementations
that call `gettext()`. This is a non-trivial change that affects the error type API.
Present this as a tracked work item, not a simple wrapping task.

---

**Entry 89**
```
file: src/c2pa/error.rs
line: 29
string: "IO error: {0}"
classification: ERROR_MSG (thiserror Display)
interpolated: yes — IO error value
macro to use: DESIGN DECISION — custom Display impl needed
```

---

**Entry 90**
```
file: src/c2pa/error.rs
line: 32
string: "C2PA error: {0}"
classification: ERROR_MSG (thiserror Display)
interpolated: yes — c2pa error value
macro to use: DESIGN DECISION — custom Display impl needed
```

---

**Entry 91**
```
file: src/c2pa/error.rs
line: 35
string: "Config error: {0}"
classification: ERROR_MSG (thiserror Display)
interpolated: yes — config error value
macro to use: DESIGN DECISION — custom Display impl needed
```

---

**Entry 92**
```
file: src/c2pa/error.rs
line: 38
string: "Signing error: {0}"
classification: ERROR_MSG (thiserror Display)
interpolated: yes — signing error value
macro to use: DESIGN DECISION — custom Display impl needed
```

---

**Entry 93**
```
file: src/c2pa/error.rs
line: 44
string: "Algorithm '{0}' is not in the FIPS-safe allowed set"
classification: ERROR_MSG (thiserror Display)
interpolated: yes — algorithm name
macro to use: DESIGN DECISION — custom Display impl needed
notes: Algorithm name is a technical identifier — untranslatable component.
```

---

**Entry 94**
```
file: src/c2pa/error.rs
line: 47
string: "Refusing to overwrite previously signed file: {0}"
classification: ERROR_MSG (thiserror Display)
interpolated: yes — file path
macro to use: DESIGN DECISION — custom Display impl needed
```

---

---

## File: `src/c2pa/manifest.rs`

### TrustStatus display strings — policy-critical

These strings are rendered by `TrustStatus::Display` and appear in the chain-of-custody report.
They are security classification indicators and **require `msgctxt`** in the `.po` file.

---

**Entry 95**
```
file: src/c2pa/manifest.rs
line: 78
string: "TRUSTED"
classification: REPORT_OUTPUT (security status label)
interpolated: no
macro to use: gettext() — but ONLY after Henri confirms the French equivalent
notes: [POLICY-CRITICAL] This label indicates cryptographic trust chain verification.
  msgctxt REQUIRED: "C2PA trust status label"
  Henri must confirm the authoritative French term before any translation is attempted.
  Candidate (subject to corpus lookup): "FIABLE" or "DE CONFIANCE"
  TERMIUM lookup required before committing.
```

---

**Entry 96**
```
file: src/c2pa/manifest.rs
line: 79
string: "UNVERIFIED"
classification: REPORT_OUTPUT (security status label)
interpolated: no
macro to use: gettext() — with msgctxt
notes: [POLICY-CRITICAL] This label indicates the signer is not on a trust list.
  msgctxt REQUIRED: "C2PA trust status label"
  Candidate: "NON VÉRIFIÉ" — subject to corpus lookup.
  Do not use "INCONNU" (unknown) — different semantic meaning.
```

---

**Entry 97**
```
file: src/c2pa/manifest.rs
line: 80
string: "INVALID"
classification: REPORT_OUTPUT (security status label)
interpolated: no
macro to use: gettext() — with msgctxt
notes: [POLICY-CRITICAL] This label indicates signature verification failure or hash mismatch.
  This is the most security-sensitive label in the codebase.
  msgctxt REQUIRED: "C2PA trust status label"
  Candidate: "INVALIDE" — well-established French term. Corpus lookup required.
```

---

**Entry 98**
```
file: src/c2pa/manifest.rs
line: 81
string: "REVOKED"
classification: REPORT_OUTPUT (security status label)
interpolated: no
macro to use: gettext() — with msgctxt
notes: [POLICY-CRITICAL] This label indicates certificate revocation.
  msgctxt REQUIRED: "C2PA trust status label"
  Candidate: "RÉVOQUÉ" — standard PKI term in French. Corpus lookup required.
```

---

**Entry 99**
```
file: src/c2pa/manifest.rs
line: 82
string: "NO TRUST LIST"
classification: REPORT_OUTPUT (security status label)
interpolated: no
macro to use: gettext() — with msgctxt
notes: [POLICY-CRITICAL] This label indicates no trust list was configured.
  msgctxt REQUIRED: "C2PA trust status label"
  Candidate: "AUCUNE LISTE DE CONFIANCE" — subject to corpus lookup.
  Note: this label also appears inside footnote text in report.rs (entries 46, 47, 79).
  Those instances are embedded in natural-language sentences — different treatment from
  the standalone display label here.
```

---

---

## File: `src/c2pa/signer.rs`

### describe_algorithm strings

These strings are returned by `describe_algorithm()` and printed as `ValidationResult::message`
strings. They surface to the user in the config validation report.

---

**Entry 100**
```
file: src/c2pa/signer.rs
line: 69
string: "ES256  ECDSA / P-256 (prime256v1) / SHA-256 / 256-bit / FIPS-safe"
classification: REPORT_OUTPUT (ValidationResult message via describe_algorithm)
interpolated: no
macro to use: gettext()
notes: All technical identifiers (ES256, ECDSA, P-256, prime256v1, SHA-256, FIPS)
  must not be translated — they are algorithm designators.
  Only "FIPS-safe" might have a French equivalent; the rest are international standards notation.
  Candidate for "FIPS-safe": "conforme FIPS" or "approuvé FIPS" — corpus lookup required.
```

---

**Entry 101**
```
file: src/c2pa/signer.rs
line: 70
string: "ES384  ECDSA / P-384 (secp384r1) / SHA-384 / 384-bit / FIPS-safe"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
notes: Same treatment as entry 100.
```

---

**Entry 102**
```
file: src/c2pa/signer.rs
line: 71
string: "ES512  ECDSA / P-521 (secp521r1) / SHA-512 / 521-bit / FIPS-safe"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
notes: Same treatment as entry 100.
```

---

**Entry 103**
```
file: src/c2pa/signer.rs
line: 72
string: "PS256  RSA-PSS / SHA-256 / 2048+ bit / FIPS-safe"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
notes: Same treatment as entry 100.
```

---

**Entry 104**
```
file: src/c2pa/signer.rs
line: 73
string: "PS384  RSA-PSS / SHA-384 / 2048+ bit / FIPS-safe"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
notes: Same treatment as entry 100.
```

---

**Entry 105**
```
file: src/c2pa/signer.rs
line: 74
string: "PS512  RSA-PSS / SHA-512 / 2048+ bit / FIPS-safe"
classification: REPORT_OUTPUT
interpolated: no
macro to use: gettext()
notes: Same treatment as entry 100.
```

---

---

## Design Decisions Required Before Wrapping

The following items require developer input before the wrapping report can be converted
into source changes. Present these to the developer as tracked decisions.

| # | File | Issue | Decision needed |
|---|---|---|---|
| D1 | `error.rs` | `thiserror` `#[error]` attrs cannot directly use `gettext()` | Developer implements custom `Display` impls per variant |
| D2 | `creds.rs` | `summary` is a composite multi-line `format!()` string | Developer restructures into separate per-line `println!` calls in `main.rs`, or defines a structured result type |
| D3 | `report.rs` L155–159 | `[OK]`, `[WARN]`, `[FAIL]`, `[INFO]`, `[SKIP]` are 6-character padded tags in a column layout | Confirm whether French translations must fit 6 chars, or whether layout adapts |
| D4 | `manifest.rs` | TrustStatus Display strings are security labels | Henri must approve French translations before wrapping; `msgctxt` required for all five |
| D5 | `report.rs` L48–109 | Multiple field labels ("Signed at", "Issuer", "Alg", "Generator", "Marking") use column-aligned `{:<pad$}` layout | Developer must determine whether pad width adjusts dynamically for French or is fixed |

---

## Priority Order for Wrapping Work

1. **ERROR_MSG strings in `main.rs`** — these surface on every failure path and must be correct in French before any deployment. (Entries 1, 5, 12, 16, 19, 22, 26, 29–33.)

2. **REPORT_OUTPUT strings in `report.rs`** — the chain-of-custody report is the primary user-visible output. (Entries 43–66.) Block on D3 and D5 decisions first.

3. **TrustStatus display labels in `manifest.rs`** — policy-critical; Henri must review. (Entries 95–99.) Block on Henri's approval.

4. **ValidationResult messages in `validate.rs`** — config validation report. (Entries 67–83.)

5. **CredCheck messages in `creds.rs`** — credential validation output. (Entries 86–88.) Block on D2 for summary strings.

6. **InspectError display strings in `error.rs`** — block on D1 design decision. (Entries 89–94.)

7. **describe_algorithm strings in `signer.rs`** — lowest urgency; technical strings with limited translation scope. (Entries 100–105.)

8. **VERBOSE_MSG strings** — only visible with `--verbose`; lower urgency than the above. Wrap after all ERROR_MSG and REPORT_OUTPUT strings are done.

---

## Strings Explicitly Excluded from This Report

The following were examined and determined not to need wrapping:

- All `log::debug!`, `log::info!`, `log::warn!` calls — these go to journald only.
- All `log::debug!` format strings in `trust.rs` — machine-readable structured log entries.
- Serde field names and JSON keys in `ingest.rs` and `manifest.rs`.
- MIME type strings in `ingest.rs` — machine-parseable identifiers.
- Algorithm identifier strings in `signer.rs` used as match arms (not for display).
- TOML config keys and field names in `config.rs`.
- C2PA spec identifier strings ("c2pa.acquired", "c2pa.published", "umrs.security-label") — protocol identifiers.
- SEPARATOR and THIN_SEP constants in `report.rs` — purely decorative box-drawing.
- Unicode checkmark/x-mark characters in `main.rs` (U+2714, U+2718) — symbols, not text.
- The `config_template()` function string — this is a file template, not user-facing interactive output. It should be handled as a separate documentation translation task, not inline gettext wrapping.
