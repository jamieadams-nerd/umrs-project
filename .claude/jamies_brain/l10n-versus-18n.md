# UMRS Localization (l10n) Guidance

*From the desk of Jamie Adams, Principal Architect*  
*Reviewed by Henri Bélanger and Simone*

---

## Terminology First

Before diving in, let's be precise:

- **i18n** (internationalization) — the *infrastructure* that makes multi-language support possible. UMRS has this. gettext is wired in, locale switching works, the foundation is solid.
- **l10n** (localization) — the *discipline* of making that infrastructure produce correct, policy-accurate, culturally appropriate output for a specific locale. This document is about that.

We have good i18n. This document is about doing l10n *right*.

---

## Why l10n Matters More for UMRS Than Most Projects

Most projects treat l10n as a translation exercise. For UMRS it is a **policy compliance exercise**.

A generic translator would render "Protected B" as "Protégé B." Henri knows it must be **PROTÉGÉ B** — capitalized, accented, per current Treasury Board policy. That distinction is not cosmetic. In a real government deployment, a terminological inconsistency is not a UX bug. It is a potential disclosure incident.

Henri has lived this. The 2016 Canadian-US tabletop exercise where "renseignements désignés" (five years out of policy) versus "information protégée" caused two working groups to reach completely different conclusions from what everyone believed was the same scenario — ninety minutes of confusion that in a real operation would have been a disclosure incident. That is the stakes.

---

## What Full l10n Adherence Requires

### 1. Complete String Externalization

Zero hardcoded user-facing strings anywhere in the Rust codebase. This includes:

- Error messages
- CLI output and diagnostics
- Log messages that surface to users
- Any `format!()` result that reaches a human

Henri and Simone will audit every `println!`, `eprintln!`, and user-facing `format!()` call. If it touches a user, it belongs in a `.po` file.

```rust
// Wrong — hardcoded string
eprintln!("Error: label not found");

// Right — externalized
eprintln!("{}", t!("error.label_not_found"));
```

---

### 2. PO File Quality Standards

`msgstr` translations must meet UMRS standards — not machine translated, not approximated:

- All terminology must align with **Termium Plus** entries (32,000 already ingested)
- Security classification labels must use **exact Treasury Board terminology**
- Simone produces the translations; Henri validates policy accuracy

This is not a division of labour — it is a dual-key requirement. Linguistic correctness and policy correctness are separate checks.

---

### 3. Policy-Critical Label Fidelity

Security labels are not ordinary strings. They require `msgctxt` annotations and explicit policy commentary so translators — human or AI — have the context needed to get them right:

```po
#. UMRS: Security label — must match TB policy exact terminology
#. Reference: Treasury Board Directive on Security Management, Appendix J
#: src/labels.rs:42
msgctxt "security_label"
msgid "Protected B"
msgstr "PROTÉGÉ B"
```

```po
#. UMRS: Security label — Protected A per TB policy
#: src/labels.rs:38
msgctxt "security_label"
msgid "Protected A"
msgstr "PROTÉGÉ A"
```

```po
#. UMRS: Security label — Protected C per TB policy
#: src/labels.rs:46
msgctxt "security_label"
msgid "Protected C"
msgstr "PROTÉGÉ C"
```

The `msgctxt` field is mandatory for all security labels. It prevents a generic translation tool from substituting a linguistically acceptable but policy-incorrect term.

---

### 4. French Typography Rules

French is not English with different words. French has different punctuation rules that affect rendered output:

| Rule | English | French |
|------|---------|--------|
| Colon | `Error:` | `Erreur\u00a0:` (non-breaking space before) |
| Question mark | `Ready?` | `Prêt\u00a0?` (non-breaking space before) |
| Exclamation | `Done!` | `Terminé\u00a0!` (non-breaking space before) |
| Quotation marks | `"text"` | `«\u00a0text\u00a0»` (guillemets with spaces) |

These are not optional stylistic choices. They are correct French. Henri will notice.

---

### 5. Plural Forms

French pluralization rules differ from English. gettext handles this, but `.po` files must define plural forms explicitly:

```po
msgid "one file processed"
msgid_plural "{n} files processed"
msgstr[0] "un fichier traité"
msgstr[1] "{n} fichiers traités"
```

Do not assume English plural logic applies. It does not.

---

### 6. String Expansion Budget

French text runs approximately **20–30% longer** than equivalent English. CLI output, formatted tables, and layout-sensitive strings must accommodate this without truncating or breaking alignment.

If a string is layout-sensitive, flag it in the `.pot` file:

```po
#. UMRS: Layout-sensitive string — max ~40 chars for column alignment
#: src/output.rs:88
msgid "Label applied successfully"
msgstr "Étiquette appliquée avec succès"
```

---

### 7. Locale Detection

Respect system locale conventions. Detection order:

```rust
// Priority order for locale detection
// 1. Explicit CLI flag: --locale fr_CA
// 2. LC_ALL environment variable
// 3. LANG environment variable
// 4. Default: en_CA
```

`fr_CA` is the target locale — not `fr_FR`. Canadian French has specific terminology differences that matter for government policy context. This is Henri's domain.

---

### 8. Testing Requirements

l10n correctness requires its own test suite — separate from unit tests, separate from integration tests.

**Pseudolocalization** — replace all strings with accented equivalents to catch hardcoded assumptions and layout breaks:

```
"Label applied" → "Làbél àpplïéd"
```

Run this before any release to surface layout issues before they reach translation.

**Coverage validation** — every `msgid` in the `.pot` file must have a corresponding `msgstr` in the `fr_CA` `.po` file. A missing translation is a build warning. In a policy-sensitive context, it may be a build error.

**Policy spot-check** — Henri reviews all security classification labels against current TB policy before any tagged release. This is not optional and cannot be automated.

---

### 9. File Structure

```
locale/
├── umrs.pot              # Master template — generated from source
├── fr_CA/
│   └── LC_MESSAGES/
│       ├── umrs.po       # French Canadian translations (Simone + Henri)
│       └── umrs.mo       # Compiled binary (generated, not committed)
└── en_CA/
    └── LC_MESSAGES/
        ├── umrs.po       # English Canadian baseline
        └── umrs.mo
```

The `.mo` files are generated artifacts. They are not committed to the repository.

---

## Roles and Responsibilities

| Responsibility | Owner |
|---------------|-------|
| i18n infrastructure, gettext wiring | Jamie |
| String wrapping for externalization, `.pot` generation | Rust developers |
| French translation (`msgstr`) | Simone |
| Policy accuracy validation (TB, CCCS, Termium Plus) | Henri |
| Typography and linguistic correctness | Simone + Henri |
| l10n test suite | Rust developers |
| Pre-release policy spot-check | Henri |

---

## Henri's Standing Note

> "A tool that renders PROTÉGÉ B correctly is not doing something extra. It is doing the minimum required to be taken seriously by a Canadian government user. Everything else we build on top of that minimum."

*— Henri Bélanger, Canadian Government Specialist*

---

*This document is a living reference. Updates require Henri's sign-off on any changes to the policy accuracy section.*


