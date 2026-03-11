# UMRS French (fr_CA) Vocabulary Reference

Canonical translations for UMRS technical terms. Maintained by the `umrs-translator` agent.

**Update policy**: Add or revise an entry here whenever a new term decision is made during translation work.
When a term has no established francophone equivalent, retain the English term and add a note explaining why.

---

## Directory Listing Column Headers (umrs-ls)

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| MODE (file permissions) | DROITS | corpus:coreutils | coreutils consistently uses "droits" for permission/mode strings; no exact column-header match found, but "droits" is corpus-informed |
| MARKING (SELinux marking) | MARQUAGE | UMRS decision | See Security and Access Control section |
| OWNER:GROUP | PROPRIO:GROUPE | corpus:coreutils | coreutils chown.c: "[OWNER][:[GROUP]]" → "[PROPRIO][:[GROUPE]]"; adapted to column-header style |
| SIZE | TAILLE | corpus:tar | tar genfile.c: "SIZE" → "TAILLE"; also coreutils "Size" → "Taille" |
| MODIFIED (mtime column) | MODIFIÉ | UMRS decision | No corpus match for "MODIFIED"; ANSSI/POSIX francophone standard for last-modification timestamp |
| NAME | NOM | corpus:coreutils | coreutils who.c and tar tar.c/genfile.c: "NAME" → "NOM" |
| <restricted> (placeholder) | <restreint> | corpus:bash | bash builtins/common.c: "restricted" → "restreint"; angle-bracket format preserved |

---

## Storage and Resource Terms (umrs-logspace)

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| Resource Pool | Groupe de ressources | UMRS decision | No corpus match; refers to a storage partition/disk group; "groupe de ressources" is standard francophone infrastructure terminology |
| Mount point | Point de montage | corpus:coreutils | coreutils find-mount-point.c: "mount point" → "point de montage" (both %m format string and ~msgid entries confirm this term) |
| Total space | Espace total | UMRS decision | No corpus match; "espace total" is standard francophone POSIX/Linux usage for total disk capacity |
| Free space | Espace libre | UMRS decision | No corpus match; "espace libre" is standard francophone POSIX/Linux usage for available disk capacity |
| Lifecycle | Cycle de vie | UMRS decision | No corpus match; "cycle de vie" is attested ANSSI/OTAN terminology for data retention lifecycle |

---

## Security and Access Control

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| access control | contrôle d'accès | corpus:coreutils | |
| access denied | accès refusé | UMRS decision | Standard Unix/Linux francophone term; no direct GNU corpus match; widely attested in POSIX, ANSSI, OTAN/NATO documentation |
| mandatory access control | contrôle d'accès obligatoire | UMRS decision | No corpus match; attested ANSSI/OTAN; ISO/IEC 7498-2 |
| audit event | événement d'audit | UMRS decision | |
| category set | ensemble de catégories | UMRS decision | |
| kernel module | module noyau | UMRS decision | |
| MLS range | plage MLS | UMRS decision | MLS retained as acronym |
| policy enforcement | application de la politique | UMRS decision | |
| reference monitor | moniteur de référence | UMRS decision | |
| security context | contexte de sécurité | corpus:coreutils | Corpus uses contexte de sécurité for SELinux label strings throughout |
| security label | étiquette de sécurité | UMRS decision | |
| security marking (column header) | MARQUAGE | UMRS decision | OTAN/NATO and ANSSI francophone term for classification marking; preferred over "étiquette" for the human-readable marking string column; no corpus match |
| sensitivity label | étiquette de sensibilité | UMRS decision | |
| restricted (placeholder) | restreint | corpus:bash | bash corpus: msgid "restricted" → msgstr "restreint"; applied in angle-bracket placeholder format as <restreint> |
