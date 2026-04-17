---
name: i18n-l10n-rules
description: >
  Internationalization and localization rules for UMRS: locale detection, gettext wiring,
  .po/.pot file management, French Canadian (fr_CA) translation, security label fidelity,
  and terminology authority. Use this skill when working with i18n, l10n, locale, translation,
  gettext, .po, .pot, French, fr_CA, locale detection, msgid, msgstr, or bilingual content.
  Trigger when the user or agent mentions any of these terms or when writing/reviewing
  user-facing strings, translation files, or locale-sensitive output.
---

## i18n and l10n Rules

Applies when working with i18n, l10n, locale, translation, gettext, `.po`/`.pot` files,
French output, locale detection, or any user-facing string externalization.

---

### Terminology

- **i18n** (internationalization) — the infrastructure that makes multi-language support
  possible: gettext wiring, locale switching, string externalization.
- **l10n** (localization) — the discipline of producing correct, policy-accurate,
  culturally appropriate output for a specific locale.

UMRS has i18n infrastructure. These rules govern l10n correctness.

---

### Linux Locale Environment Variables

The Linux locale system uses a priority chain of environment variables. Understanding
the full chain is mandatory for anyone working on l10n or debugging locale behavior.

#### Variable Definitions

| Variable | Purpose |
|---|---|
| `LC_ALL` | Master override. When set, it overrides every other `LC_*` variable and `LANG`. Use for testing only — never set system-wide in production. |
| `LC_MESSAGES` | Controls the language of user-facing messages (UI strings, error messages, diagnostics). This is the variable gettext consults. |
| `LC_COLLATE` | String sorting order (e.g., `a < b` vs. accent-sensitive ordering). |
| `LC_CTYPE` | Character classification and case conversion (what counts as a letter, digit, etc.). |
| `LC_NUMERIC` | Number formatting (decimal separator: `.` vs. `,`). |
| `LC_TIME` | Date and time formatting. |
| `LC_MONETARY` | Currency formatting. |
| `LANG` | Fallback default. Used when a specific `LC_*` variable is not set. |
| `LANGUAGE` | GNU gettext override. A colon-separated fallback list (e.g., `fr_CA:fr:en`). When set, gettext uses this list instead of `LC_MESSAGES` for message catalog lookup. Does not affect non-message locale categories. |

#### Priority Chain

The effective locale for any category is determined in this order:

1. `LC_ALL` — if set, overrides everything for all categories
2. The specific `LC_*` variable for the category (e.g., `LC_MESSAGES` for messages)
3. `LANG` — fallback when the specific `LC_*` variable is unset

For gettext message lookup specifically, `LANGUAGE` can override the result of
the above chain. This is a GNU extension and a common source of silent misconfiguration.

[RULE] When debugging locale issues, always inspect the full variable set with `locale`.
Never assume that setting `LANG` alone is sufficient to change message output.

---

### UMRS Locale Detection Priority

[RULE] UMRS determines the active locale in this order. The first match wins:

1. Explicit CLI flag: `--locale fr_CA`
2. `LC_ALL` environment variable
3. `LANG` environment variable
4. Default: system locale (typically `en_US.UTF-8` on development systems)

[CONSTRAINT] `fr_CA.UTF-8` is the target Canadian French locale. Never use `fr_FR`.
Canadian French has specific terminology differences that are policy-significant
in government context. This is not a preference — it is a compliance requirement.

[CONSTRAINT] UMRS supports UTF-8 encoding exclusively. Never use ISO-8859-1,
Latin-1, or any other encoding. All locale references must use the `.UTF-8`
suffix (e.g., `fr_CA.UTF-8`, `en_US.UTF-8`). Verify with `locale -a` that
the `.utf8` variant is available on the target system.

[RULE] UMRS does not override or assume the system locale. It respects whatever
the OS is configured to use. On Canadian deployments, the system locale will be
`en_CA` or `fr_CA`. On development systems, `en_US` is typical and correct.

---

### Testing as Canadian French

To simulate a Canadian French system on an English-based development machine,
the full locale environment must be configured — not just `LANG`.

#### Prerequisites

[RULE] The target locale must be generated on the system before it can be used.
Even though UMRS is written in Rust, locale resolution depends on the OS-level
locale definitions provided by glibc. Without them, `setlocale` silently falls
back to `POSIX/C` and `.mo` message catalogs never load.

```bash
# RHEL 10 / Fedora
sudo dnf install glibc-langpack-fr

# Ubuntu / Debian
sudo locale-gen fr_CA.UTF-8
sudo update-locale
```

Verify the locale is available:

```bash
locale -a | grep fr_CA
```

Expected output includes `fr_CA.utf8` or `fr_CA.UTF-8`.

#### Setting the Full Environment

[PATTERN] For testing, set all relevant variables to eliminate partial-override bugs:

```bash
export LANG=fr_CA.UTF-8
export LC_ALL=fr_CA.UTF-8
export LANGUAGE=fr_CA:fr:en
```

Using `LC_ALL` ensures no individual `LC_*` variable from the shell profile
leaks through. `LANGUAGE` ensures gettext follows the correct fallback chain.

#### Verifying the Configuration

```bash
locale
```

All categories should show `fr_CA.UTF-8`. If any category shows a different
locale (e.g., `LC_MESSAGES="en_US.UTF-8"`), the test environment is misconfigured.

#### Reverting After Testing

```bash
unset LC_ALL LANGUAGE
export LANG=en_US.UTF-8    # or your system's default
```

---

### Common Misconfiguration Pitfalls

[ANTI-PATTERN] Setting `LANG=fr_CA.UTF-8` without unsetting or overriding
`LC_MESSAGES`. If the shell profile or system default sets `LC_MESSAGES=en_US.UTF-8`,
messages will remain in English even though `LANG` suggests French. Always verify
with `locale` after changing `LANG`.

[ANTI-PATTERN] Missing locale generation. Setting `LANG=fr_CA.UTF-8` on a system
where the `fr_CA` locale is not generated causes silent fallback to `POSIX` or `C`.
The system will not produce French output and may not emit any warning.

[ANTI-PATTERN] The `LANGUAGE` variable silently overriding `LC_MESSAGES`. A user
with `LANGUAGE=en` in their shell profile will never see French messages regardless
of `LANG` or `LC_ALL` settings. This is a GNU gettext behavior, not a POSIX
locale behavior, and it catches experienced administrators off guard.

[ANTI-PATTERN] Compiled `.mo` files not in the correct path. gettext expects
message catalogs at `<basedir>/<locale>/LC_MESSAGES/<domain>.mo`. A file at
`locale/fr_CA/umrs.mo` (missing the `LC_MESSAGES/` directory) will not be found.

[ANTI-PATTERN] Encoding mismatch between the locale and the `.po`/`.mo` files.
UMRS uses UTF-8 exclusively. If a `.po` file is encoded as Latin-1 (ISO 8859-1)
but the locale expects UTF-8, accented characters (critical for French and for
policy terms like PROTEGE B) will render incorrectly or cause runtime errors.

[RULE] All `.po` files must declare `charset=UTF-8` in their header and must be
saved in UTF-8 encoding. No exceptions.

---

### String Externalization

[CONSTRAINT] Zero hardcoded user-facing strings in the Rust codebase. This includes:

- Error messages
- CLI output and diagnostics
- Log messages that surface to users
- Any `format!()` result that reaches a human

Every user-facing string belongs in a `.po` file.

[PATTERN] String expansion budget: French text runs approximately 20-30% longer
than equivalent English. CLI output, formatted tables, and layout-sensitive strings
must accommodate this without truncation or broken alignment. Flag layout-sensitive
strings in the `.pot` file with a comment indicating maximum width.

---

### Security Labels — Policy-Critical Fidelity

[CONSTRAINT] `msgctxt` is mandatory for all security classification labels in `.po` files.
It prevents a generic translation tool from substituting a linguistically acceptable but
policy-incorrect term.

[RULE] Security classification labels must use exact Treasury Board terminology:

| English | French (TB policy) |
|---|---|
| Protected A | PROTEGE A |
| Protected B | PROTEGE B |
| Protected C | PROTEGE C |

Note: The accented forms (PROTEGE with accent aigu on both E characters) are the
correct renderings. The table above uses ASCII for portability. In `.po` files and
all output, the correct Unicode forms with accents must be used.

[RULE] Every security label `msgstr` must include a `.po` comment citing the
authoritative policy source (e.g., Treasury Board Directive on Security Management).

### Translation Boundary — Specification Vocabulary vs Prose

[RULE] **Translate prose; preserve specification vocabulary.** This applies to all
UMRS tools, not just individual crates.

The boundary is whether an authoritative bilingual form exists:

| Category | Action | Examples |
|---|---|---|
| **Canadian government markings with TB-defined bilingual forms** | Translate using exact TB terminology | Protected A → PROTÉGÉ A |
| **Field labels and prose messages** | Translate to `fr_CA` | "Signed at" → "Signé le" |
| **Report headers and section titles** | Translate | "Chain of Custody" → "Chaîne de possession" |
| **Industry specification vocabulary with no Termium entry** | Keep English in all locales | TRUSTED, UNVERIFIED, INVALID, REVOKED, NO TRUST LIST |
| **CLI operational status codes** | Keep English in all locales | [PASS], [FAIL], [WARN], [INFO], [SKIP] |
| **Algorithm identifiers and technical constants** | Keep English | es256, SHA-256, PEM, JUMBF |

**Rationale (Henri, 2026-04-01):**
- The Official Languages Act requires bilingual service delivery but does not extend
  to machine-status codes or industry-standard technical vocabulary in CLI output.
- CSE Assemblyline and CCCS indicator tools follow the same pattern: English status
  codes, bilingual prose.
- Translating terms where Termium Plus is silent produces Level 5 (agent judgment)
  vocabulary — an invention, not a translation. This creates Five Eyes interoperability
  friction with no corresponding compliance benefit.
- When Termium Plus acquires entries for a term (e.g., if C2PA vocabulary is added),
  revisit and translate at that time.

[RULE] When in doubt about whether a term is specification vocabulary or translatable
prose, check Termium Plus. If no entry exists, keep English and document the decision.

---

### French Typography Rules

[RULE] French punctuation rules are not optional stylistic choices. They are
correct French and must be applied in all French output:

| Rule | English | French |
|---|---|---|
| Colon | `Error:` | `Erreur\u00a0:` (non-breaking space before colon) |
| Question mark | `Ready?` | `Pret\u00a0?` (non-breaking space before question mark) |
| Exclamation | `Done!` | `Termine\u00a0!` (non-breaking space before exclamation) |
| Quotation marks | `"text"` | `\u00ab\u00a0text\u00a0\u00bb` (guillemets with non-breaking spaces) |

The non-breaking space (`\u00a0`) is mandatory. A regular space will cause
line-break artifacts in terminal output.

---

### Plural Forms

[RULE] French pluralization rules differ from English. The `.po` file header
must declare the correct plural form:

```po
"Plural-Forms: nplurals=2; plural=(n > 1);\n"
```

Note: In French, zero is singular (`0 fichier`), unlike English where zero is
plural (`0 files`). gettext handles this correctly only if the plural form
declaration is present and correct.

---

### Terminology Authority

[RULE] Terminology decisions follow this hierarchy (stop at the first authoritative match):

1. **Termium Plus** — federal authority for all government material
2. **GNU `.po` files** — proven production vocabulary for CLI terms
3. **OQLF GDT** — Quebec standard; applies where Termium Plus is silent
4. **Simone's documented vocabulary list** — prior UMRS decisions
5. **Agent judgment** — last resort; always document rationale

[RULE] Divergence between Termium Plus (level 1) and OQLF GDT (level 3) is a finding.
Document it. Do not silently resolve it.

[RULE] Federal Canadian French and Quebec French are not the same register.
Apply the correct standard based on context. Conflating them is a finding.

---

### Roles and Responsibilities

| Responsibility | Owner |
|---|---|
| i18n infrastructure, gettext wiring | Jamie |
| String inventory and extraction reports | Simone |
| String wrapping in Rust source (gettext macros) | Rust developers (Rusty) |
| `.pot` template extraction from wrapped source | Simone (via Makefile) |
| French translation (`msgstr`) | Simone |
| Policy accuracy validation (TB, CCCS, Termium Plus) | Henri |
| Typography and linguistic correctness | Simone + Henri |
| l10n test suite | Rust developers (Rusty) |
| Pre-release policy spot-check | Henri |

### String Inventory → Wrapping Workflow

[RULE] The i18n workflow for a new or existing crate follows this sequence:

1. **Simone** scans the source and produces a string inventory report —
   classifying each user-facing string by type (error, CLI output, verbose,
   report), noting interpolated values, flagging layout-sensitive strings,
   and identifying security labels that need `msgctxt`.
2. **Rusty** wraps the identified strings in gettext macros in the Rust source,
   guided by Simone's inventory. Rusty does not choose translations or decide
   terminology — only applies the mechanical wrapping.
3. **Simone** extracts the `.pot` template from the wrapped source using the
   Makefile l10n targets (`make pot` or equivalent). The extraction tooling
   is Simone's domain, not Rusty's.
4. **Simone** translates the `msgid` entries into `fr_CA` `msgstr` values,
   consulting Termium Plus and the approved vocabulary list.
5. **Henri** validates policy-critical translations (security labels, TB terms).

[CONSTRAINT] Simone does not modify Rust source code. Rusty does not write
French translations. The boundary is the `.pot` file: Rusty produces it,
Simone consumes it.

[RULE] Linguistic correctness and policy correctness are separate, independent checks.
Both must pass before any localized output leaves the team. Simone owns linguistic
accuracy. Henri owns policy accuracy. Neither substitutes for the other.

---

### Deployment Verification — Canadian VMs

Jamie will provision two RHEL 10 virtual machines installed with Canadian locale
selections during OS installation:

- **VM 1:** English (Canada) → `LANG=en_CA.UTF-8`
- **VM 2:** Français (Canada) → `LANG=fr_CA.UTF-8`

These VMs are the acceptance environment for l10n validation. All UMRS tools
must be deployed and tested on both VMs before any release that includes
French localization. The full team participates:

- **Rusty:** verify tools build, run, and produce correct output on both VMs
- **Henri:** validate that security labels, TB terminology, and policy-critical
  strings render correctly on the `fr_CA` VM
- **Simone:** verify typography (guillemets, non-breaking spaces, accents),
  plural forms, and string expansion on the `fr_CA` VM
- **Knox:** verify that locale switching does not degrade security posture
  (no label fallback to English on the French VM, no silent `POSIX` fallback)

This is not a substitute for automated l10n tests — it is the final
acceptance gate that proves UMRS works as a real Canadian system, not
just an American system with `LANG` overridden.

### Testing Requirements

[RULE] l10n correctness requires its own validation, separate from unit and integration tests:

- **Pseudolocalization** — replace all strings with accented equivalents to catch
  hardcoded assumptions and layout breaks before translation begins.
- **Coverage validation** — every `msgid` in the `.pot` file must have a corresponding
  `msgstr` in the `fr_CA` `.po` file. Missing translations are build warnings.
  In a policy-sensitive context, missing translations may be build errors.
- **Policy spot-check** — Henri reviews all security classification labels against
  current Treasury Board policy before any tagged release. This is not optional
  and cannot be automated.

---

### File Structure

[RULE] Locale files follow the standard gettext directory structure:

```
locale/
  umrs.pot                        # Master template (generated from source)
  fr_CA/
    LC_MESSAGES/
      umrs.po                     # French Canadian translations (Simone + Henri)
      umrs.mo                     # Compiled binary (generated, not committed)
  en_CA/
    LC_MESSAGES/
      umrs.po                     # English Canadian baseline
      umrs.mo                     # Compiled binary (generated, not committed)
```

[RULE] `.mo` files are generated artifacts. They are not committed to the repository.

---

### Five Eyes Considerations

[ASSUMPTION] When UMRS operates in a Five Eyes information-sharing context,
locale-specific security labels must be rendered in the locale of the receiving
nation's system, not the originating nation. A Canadian system displaying US CUI
labels must still use Treasury Board terminology for the Canadian-equivalent tier,
with the US designation shown as a cross-reference — not as the primary label.

[RULE] When Canadian and US information handling label terminology diverges in
translated output, flag the divergence explicitly with both positions stated.
Route to Jamie for decision. Do not resolve unilaterally.
