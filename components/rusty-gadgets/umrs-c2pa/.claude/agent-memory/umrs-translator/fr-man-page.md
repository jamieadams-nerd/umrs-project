---
name: French man page — umrs-c2pa
description: Translation decisions, term choices, and Henri D3/D4 rulings applied to the fr_CA man page for umrs-c2pa
type: project
---

Output file: `components/rusty-gadgets/umrs-c2pa/docs/fr/umrs-c2pa.1`
Source file: `components/rusty-gadgets/umrs-c2pa/docs/umrs-c2pa.1`
Date written: 2026-04-01

## Term table applied (Jamie-authorized, corpus empty)

| English | fr_CA used |
|---|---|
| certificate | certificat |
| manifest | manifeste |
| trust | confiance |
| chain of custody | chaîne de possession |
| provenance | provenance |
| private key | clé privée |
| signing (n.) | signature |
| sign (v.) | signer |
| timestamp authority | autorité d'horodatage |
| validate | valider |
| credential | justificatif d'identité |
| inspection | inspection |
| trust anchor | ancre de confiance |
| algorithm | algorithme |
| file | fichier |
| configuration | configuration |
| security marking | marquage de sécurité |
| crate (Rust) | caisse |

## Henri D3/D4 decisions applied

- TRUSTED, UNVERIFIED, INVALID, REVOKED, NO TRUST LIST — retained in English (C2PA spec vocabulary)
- [PASS], [FAIL], [WARN], [INFO], [SKIP] — retained in English (operational status codes)
- es256, es384, es512, ps256, ps384, ps512, ed25519 — retained in English (identifiers)
- Command syntax, subcommand names, flag names — retained in English (only descriptions translated)

## French typography applied

- Non-breaking space before `:`, `?`, `!` via `\~`
- Section headings: NOM, SYNOPSIS, DESCRIPTION, OPTIONS, SOUS-COMMANDES, ALGORITHMES,
  LISTES DE CONFIANCE, FICHIER DE CONFIGURATION, DÉMARRAGE RAPIDE, CODES DE RETOUR,
  FICHIERS, ENVIRONNEMENT, CONSIDÉRATIONS DE SÉCURITÉ, NORMES, AUTEURS, LICENCE, VOIR AUSSI
- Formal register: "vous" (not used directly in this man page; imperative mood for steps)
- Infinitive mood for section headings (e.g., "Générer un nouveau certificat...")

## Notes

- "p. ex." used for "e.g." (Canadian French convention)
- "drapeau" used for command-line "flag" (standard CLI term in fr_CA)
- "répertoire" for "directory"
- "courant" for "current" (as in répertoire courant)
- Crate name "caisse" used once where natural language reference needed (THIRD-PARTY-NOTICES context)
