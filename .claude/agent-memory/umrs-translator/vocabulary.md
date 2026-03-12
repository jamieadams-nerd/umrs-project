# UMRS Translator Vocabulary

UMRS-specific glossary. Overrides GNU corpus for UMRS-specific context.

| English | French | Source | Notes |
|---|---|---|---|
| access control | contrôle d'accès | corpus:coreutils | Confirmed match |
| access denied | accès refusé | UMRS decision | No GNU corpus match; standard Unix/Linux francophone; POSIX/ANSSI/OTAN attested |
| audit event | événement d'audit | UMRS decision | No corpus match found |
| category set | ensemble de catégories | UMRS decision | No corpus match found |
| kernel module | module noyau | UMRS decision | No corpus match found |
| mandatory access control | contrôle d'accès obligatoire | UMRS decision | No corpus match; attested ANSSI/OTAN; ISO/IEC 7498-2 |
| MLS range | plage MLS | UMRS decision | MLS retained as acronym |
| policy enforcement | application de la politique | UMRS decision | No corpus match found |
| reference monitor | moniteur de référence | UMRS decision | No corpus match found |
| security context | contexte de sécurité | corpus:coreutils | Preferred term. Full five-part SELinux label: user:role:type:sensitivity_level:category_set. Corpus uses this for SELinux label strings throughout. |
| sensitivity level | niveau de sensibilité | UMRS decision | The s0–s15 hierarchical component. Preferred over "sensitivity label" (étiquette de sensibilité), which is colloquial and imprecise. |

## Removed / Non-preferred Terms

| Removed | Replacement | Reason |
|---|---|---|
| security label (étiquette de sécurité) | security context (contexte de sécurité) | "Security label" is a colloquial synonym for security context. Use the precise term. |
| MLS label (étiquette MLS) | security context (contexte de sécurité) | Generic "MLS label" maps to the full security context. Use security context for precision. |
| sensitivity label (étiquette de sensibilité) | sensitivity level (niveau de sensibilité) | "Sensitivity label" is non-preferred. The correct term is "sensitivity level" — the s0–s15 hierarchical component of a security context. |
