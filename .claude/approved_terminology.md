# Approved Terminology List

Use these terms consistently across all UMRS documentation.
Propose new terms explicitly; add to this list when approved.

**Related files** (all three must remain consistent):
- This file — preferred English forms and abbreviation rules for documentation
- `.claude/agent-memory/umrs-translator/vocabulary.md` — English-to-French mappings for i18n
- `docs/modules/glossary/pages/index.adoc` — reader-facing definitions

When adding a term here, check both related files.
If a translation exists in the translator vocabulary, note it.
If a reader-facing definition is needed, add it to the glossary.

---

## Preferred Terms

| Use | Do not use | Note |
|---|---|---|
| `high-assurance` (adjective) | `HA` | "HA" reads as high-availability in most infrastructure contexts. Always spell out. |
| `high assurance` (noun phrase) | `HA` | Same rule — no abbreviation in prose. |
| `HA-Sign` | (no variation) | This is the tool's proper name. "HA-" is part of the product name, not an abbreviation in general prose. |
| `security context` | `security label`, `context`, `selinux context` | The full five-part SELinux label: `user:role:type:sensitivity_level:category_set`. "Security label" is a colloquial synonym — avoid it for precision. Use `security context` throughout. Translator: *contexte de sécurité*. |
| `sensitivity level` | `sensitivity label`, `clearance level`, `classification level` | The `s0`–`s15` hierarchical component of a security context. "Sensitivity label" is a non-preferred colloquial form. UMRS operates unclassified; avoid classification vocabulary. Translator: *niveau de sensibilité*. |
| `category set` | `categories`, `compartments` | The bitmask component of a security context: a 1024-bit `[u64; 16]` array. Translator: *ensemble de catégories*. |
| `MLS range` | `MLS level`, `label range` | The low–high pair `sensitivity_low:categories_low-sensitivity_high:categories_high` defining a subject's operating range. Translator: *plage MLS*. |
| `reference monitor` | `security monitor`, `policy monitor` | The kernel mechanism that mediates all subject–object access. Translator: *moniteur de référence*. |
| `policy enforcement` | `policy check`, `enforcement check` | Active, kernel-mediated blocking — not logging or advisory. Translator: *application de la politique*. |
| `audit event` | `log entry`, `log record`, `audit entry` | A structured, typed record of a security-relevant action. Use `audit record` only when citing NIST 800-53 AU-3 specifically. Translator: *événement d'audit*. |
| `mandatory access control` | `MAC policy`, `enforced policy` | Spell out on first use; MAC is acceptable abbreviation thereafter. Translator: *contrôle d'accès obligatoire*. |
| `Controlled Unclassified Information` | `CUI data`, `sensitive data` | CUI on first use, then CUI alone. Never "CUI data" (redundant). |
| `access denied` | `permission denied`, `access blocked` | The kernel's decision to reject an access request. Translator: *accès refusé*. |
| `system mediator` | — | The enforcing layer between subjects and objects. Use when describing the reference monitor role abstractly. |
| `kernel module` | `driver`, `kernel extension` | Preferred when referring to loadable kernel objects. Translator: *module noyau*. |

---

## Security Context — Structural Definition

A security context has five components:

```
user : role : type : sensitivity_level : category_set
```

Each component is a distinct validated type in the UMRS codebase.
"Security label" is a colloquial shorthand that people use to mean the full security context.
In UMRS documentation, always use `security context` — never the shorthand.

---

## Abbreviations

| Abbreviation | Expansion | First-use rule |
|---|---|---|
| CUI | Controlled Unclassified Information | Spell out on first use per page |
| MAC | Mandatory Access Control | Spell out on first use per page |
| MLS | Multi-Level Security | Spell out on first use per page |
| TPI | Two-Path Independence | Spell out on first use per page |
| TCB | Trusted Computing Base | Spell out on first use per page |
| FIPS | Federal Information Processing Standard | Spell out on first use per page |
| IMA | Integrity Measurement Architecture | Spell out on first use per page |
| EVM | Extended Verification Module | Spell out on first use per page |
| PQC | Post-Quantum Cryptography | Spell out on first use per page |
| RTB | Raise the Bar (NSA) | Spell out on first use per page |
| AVC | Access Vector Cache | Spell out on first use per page |

---

## Do Not Abbreviate

| Term | Reason |
|---|---|
| high-assurance | "HA" = high-availability in most contexts |
| high availability | Do not abbreviate either term — confusion with high-assurance |
| security context | Do not shorten to "context" — ambiguous without qualifier |
| sensitivity level | Do not shorten to "level" — ambiguous |
