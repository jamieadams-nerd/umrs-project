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

## System State Terms (umrs-state)

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| Starting [tool-name] | Démarrage de [tool-name] | UMRS decision | No corpus match; "démarrage de" is the standard francophone Linux/POSIX startup-message pattern |
| Enabled (FIPS is Enabled) | activé | corpus:coreutils | Corpus-attested form; FIPS acronym retained untranslated in OTAN/NATO and ANSSI contexts |
| System Purpose | Objet du système | UMRS decision | RHEL concept; "objet du système" used in RHEL francophone subscription tooling |
| state file | fichier d'état | UMRS decision | No corpus match; standard francophone Linux/POSIX term for a persistent state/status file |
| Warning (prefix) | Avertissement | corpus:coreutils | Coreutils and bash corpus: "Warning" → "Avertissement" (well attested) |
| (not set) | (non défini) | UMRS decision | No corpus match; standard francophone Linux/POSIX placeholder for an absent/unset value |
| Unknown key | Clé inconnue | corpus:findutils | findutils: "unknown" → "inconnu(e)"; "clé" is feminine in French → "inconnue" |
| Invalid | invalide | UMRS decision | No corpus match; "invalide" is the standard francophone computing term |
| boolean | booléen/booléenne | UMRS decision | No corpus match; universally attested francophone computing term |
| false (boolean value) | faux | UMRS decision | Standard francophone term for the boolean value false |

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
| hard gate (trust pipeline) | verrou dur | UMRS decision | No corpus match; a mandatory, non-bypassable check in the trust pipeline; "verrou" (lock/gate) is the ANSSI/OTAN term for a security barrier |
| tampering | falsification | UMRS decision | No corpus match; "falsification" is the ANSSI/OTAN fr_CA term for tampering with security-relevant data |
| trust tier | palier de confiance | UMRS decision | No corpus match; "palier" = a defined level in a graded scale; standard ANSSI/OTAN security assessment vocabulary |
| downgrade (trust) | déclassement | UMRS decision | No corpus match; ANSSI fr_CA term for a trust level reduction; distinct from "déclassification" (classification downgrade) |
| contradiction (evidence) | contradiction | UMRS decision | Direct cognate; used when two independent evidence sources disagree on a fact |
| hardening / to harden | durcissement / durcir | UMRS decision | No corpus match; ANSSI/OTAN fr_CA standard term for security hardening of system settings |
| kernel anchor | ancrage noyau | UMRS decision | UMRS-specific term; "ancrage" = anchoring/grounding to a known-good reference point |
| indicator (posture) | indicateur | UMRS decision | No corpus match; "indicateur" is the fr_CA standard for a measured posture signal |
| posture | posture | UMRS decision | Direct cognate; retained as technical term in ANSSI/OTAN security assessment contexts |
| drift (configuration) | dérive | UMRS decision | No corpus match; ANSSI fr_CA term for configuration drift (live state diverging from persisted config) |
| core dump | vidage mémoire | UMRS decision | No corpus match; standard POSIX/Linux fr_CA term for a process memory dump file |
| world-readable | accessible à tous | UMRS decision | No corpus match; fr_CA rendering of the Unix "world-readable" permission idiom |
| accounting (netfilter) | comptabilisation | UMRS decision | No corpus match; per-connection byte/packet counter feature in netfilter; "comptabilisation" is the standard fr_CA infrastructure term |
| profiling (perf) | profilage | UMRS decision | No corpus match; "profilage" is the standard fr_CA computing term for performance profiling |
| digest (cryptographic) | condensé | UMRS decision | No corpus match; ANSSI/ISO fr_CA term for a cryptographic hash/digest value |
| Not Present (absent sentinel) | Non présent | UMRS decision | Used when a cmdline token or blacklist entry is intentionally absent; distinct from "unavailable" (read failure) |
| unavailable (read failure) | indisponible | UMRS decision | Used when a data field could not be read or is absent from the detection result |
| not available (absent field) | non disponible | UMRS decision | Used as a row value when an optional field (e.g., VERSION_ID) is absent from os-release |

---

## OS Detection and Trust Terms (umrs-uname)

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| OS Information (tab) | Informations système | UMRS decision | No corpus match; standard fr_CA for a system identity/status data panel |
| Kernel Security (tab) | Sécurité noyau | UMRS decision | No corpus match; ANSSI/OTAN fr_CA rendering of "kernel security" |
| Trust / Evidence (tab) | Confiance / Preuves | UMRS decision | No corpus match; ANSSI fr_CA for the trust classification and evidence chain tab |
| OS Detection Audit | Audit de détection OS | UMRS decision | "Audit de détection" follows ANSSI/OTAN audit report title conventions |
| Label Trust | Confiance | UMRS decision | Shortened 2026-03-23 from "Confiance de l'étiquette" (25 chars) after live TUI testing showed column jamming. "Confiance" (9 chars) is unambiguous: value column carries full classification. |
| Trust Tier | Palier | UMRS decision | Shortened 2026-03-23 from "Palier de confiance" (19 chars). "Palier" alone is unambiguous: value column carries the full tier label (e.g., T4 — Ancré à l'intégrité). |
| Downgrade Reasons | Motifs | UMRS decision | Shortened 2026-03-23 from "Motifs de déclassement" (22 chars). "Motifs" (6 chars) is unambiguous: value is either a count or the "no downgrade" positive message. |
| Evidence Records | Preuves | UMRS decision | Shortened 2026-03-23 from "Enregistrements de preuves" (26 chars). "Preuves" (7 chars) is unambiguous: value is always an integer count. |
| Catalog Baseline | Base de référence du catalogue | UMRS decision | No corpus match; "base de référence" is the ANSSI/OTAN fr_CA for a security baseline |
| Probe Used | Sonde utilisée | UMRS decision | No corpus match; "sonde" = software detection probe |
| Platform Facts | Faits de plateforme | UMRS decision | No corpus match; the count of independent package substrate facts confirmed |
| No Assessment | Sans évaluation | UMRS decision | No corpus match; indicators that could not be assessed because the kernel node was unreadable |
| BOOT INTEGRITY (group) | INTÉGRITÉ AU DÉMARRAGE | UMRS decision | No corpus match; ANSSI fr_CA standard for boot-time integrity controls |
| CRYPTOGRAPHIC POSTURE (group) | POSTURE CRYPTOGRAPHIQUE | UMRS decision | Direct compound; "posture cryptographique" used in ANSSI/OTAN assessment contexts |
| KERNEL SELF-PROTECTION (group) | AUTO-PROTECTION DU NOYAU | UMRS decision | No corpus match; "auto-protection" is the ANSSI fr_CA rendering of self-protection mechanisms |
| PROCESS ISOLATION (group) | ISOLATION DES PROCESSUS | UMRS decision | No corpus match; standard fr_CA for OS process isolation controls |
| FILESYSTEM HARDENING (group) | DURCISSEMENT DU SYSTÈME DE FICHIERS | UMRS decision | See "hardening" entry; longer than English — FLAG for developer width review |
| MODULE RESTRICTIONS (group) | RESTRICTIONS DE MODULES | UMRS decision | No corpus match; standard fr_CA for kernel module loading restrictions |
| NETWORK AUDITING (group) | AUDIT RÉSEAU | UMRS decision | No corpus match; ANSSI fr_CA for network traffic auditing controls |
| Configured: (prefix) | Configuré : | UMRS decision | fr_CA typography: non-breaking space before colon (U+00A0) required |
| from (source file prefix) | source | UMRS decision | In context: "Configuré : <value> (source <file>)"; "source" is clearer than "depuis" for a file attribution |
| OS Detection (report name) | Détection OS | UMRS decision | No corpus match; TUI header report name; "OS" retained as OTAN/NATO acronym |
| Platform Identity and Integrity | Identité et intégrité de plateforme | UMRS decision | No corpus match; TUI header report subject; ANSSI/OTAN phrasing for platform identity assessment |
| (no data) | (aucune donnée) | UMRS decision | Fallback row label for an invalid tab index condition |
| (invalid tab index) | (index d'onglet invalide) | UMRS decision | Fallback row value for an invalid tab index condition |
| X readable — all hardened | X lisibles — tous durcis | UMRS decision | Kernel Security summary: all-hardened case; "lisibles/durcis" masculine plural agree with "indicateurs" |
| X readable — Y hardened, Z not hardened | X lisibles — Y durcis, Z non durcis | UMRS decision | Kernel Security summary: mixed-result case; same gender agreement |
| newer than catalog baseline | est plus récent que la base de référence du catalogue | UMRS decision | Version comparison message for catalog_baseline_row(); corpus:tar confirmed "plus récent" pattern |
| older than catalog baseline | est plus ancien que la base de référence du catalogue | UMRS decision | Version comparison message for catalog_baseline_row(); "ancien" opposite of "récent" |
| update your kernel | mettez à jour votre noyau | UMRS decision | Remediation instruction; imperative formal register; "votre" = formal second person for operators |

| Recommended (indicator label) | Recommandé | corpus:Termium Plus | Appears as "[ Recommandé : <value> ]" in data_panel.rs for unhardened indicators. Termium Plus attests "recommandé(e)" as fr_CA past-participle adjective. Masculine form used to agree with "réglage" (setting) implied by context. Lives in umrs-uname domain (library string rendered by binary). |

---

## Kernel Posture Annotation Terms (umrs-platform display.rs)

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| enabled (boolean indicator) | activé | corpus:coreutils | SELinux context: "activé" well attested; lowercase form for inline use |
| disabled (boolean indicator) | désactivé | corpus:coreutils | From "(désactivé par...)" in coreutils; lowercase form for inline use |
| Enabled (FIPS state label) | Activé | corpus:coreutils | Capitalised state-label form; distinct msgid from lowercase "enabled" |
| Disabled (FIPS state label) | Désactivé | corpus:coreutils | Capitalised state-label form; distinct msgid from lowercase "disabled" |
| Not Present (absent sentinel) | Non présent | UMRS decision | See Security and Access Control section |
| ASLR disabled | ASLR désactivé | corpus:coreutils + UMRS | ASLR acronym retained; "désactivé" corpus-confirmed |
| partial randomization | randomisation partielle | UMRS decision | ANSSI fr_CA technical term for partial ASLR |
| full ASLR | ASLR complet | UMRS decision | ASLR acronym retained |
| pointers visible | pointeurs visibles | UMRS decision | kptr_restrict = 0; kernel pointers readable by all |
| hidden from unprivileged | masqués aux non privilégiés | UMRS decision | kptr_restrict = 1 |
| hidden from all users | masqués à tous les utilisateurs | UMRS decision | kptr_restrict = 2 |
| unprivileged BPF allowed | BPF non privilégié autorisé | UMRS decision | BPF acronym retained |
| restricted to CAP_BPF | restreint à CAP_BPF | corpus:bash + UMRS | "restreint" corpus:bash confirmed; CAP_BPF is a Linux capability — retained |
| unrestricted | sans restriction | UMRS decision | Opposite of "restreint"; ptrace scope = 0 |
| children only | enfants uniquement | UMRS decision | ptrace scope = 1 |
| admin only | administrateurs uniquement | UMRS decision | ptrace scope = 2; CAP_SYS_PTRACE required |
| no attach | attachement interdit | UMRS decision | ptrace scope = 3; complete prohibition |
| world-readable | accessible à tous | UMRS decision | Unix permission idiom; dmesg_restrict = 0 |
| restricted | restreint | corpus:bash | msgid "restricted" → msgstr "restreint"; bash builtins/common.c confirmed |
| loading allowed | chargement autorisé | corpus:bash + UMRS | "chargement" corpus:bash confirmed; "autorisé" UMRS decision |
| loading locked | chargement verrouillé | corpus:bash + UMRS | "chargement" corpus:bash; "verrouillé" for the one-way locked state |
| allowed | autorisé | UMRS decision | No corpus exact match for bare "allowed"; UMRS decision |
| fully disabled | entièrement désactivé | corpus:coreutils + UMRS | "désactivé" corpus-confirmed; sysrq = 0 |
| all functions enabled | toutes les fonctions activées | corpus:coreutils + UMRS | "activées" corpus-confirmed; sysrq = 1 |
| no core dumps | aucun vidage mémoire | UMRS decision | suid_dumpable = 0 |
| core dumps enabled | vidages mémoire activés | corpus:coreutils + UMRS | "activés" corpus-confirmed |
| readable by root only | lisibles par root uniquement | UMRS decision | suid_dumpable = 2; "root" retained as technical username |
| not protected | non protégé | UMRS decision | protected_symlinks/hardlinks = 0 |
| protected | protégé | UMRS decision | protected_symlinks/hardlinks = 1 |
| partial protection | protection partielle | UMRS decision | protected_fifos/regular = 1 |
| fully protected | entièrement protégé | UMRS decision | protected_fifos/regular = 2 |
| accounting off | comptabilisation désactivée | UMRS decision | nf_conntrack acct = 0; feminine agreement with "comptabilisation" |
| accounting on | comptabilisation activée | corpus:coreutils + UMRS | "activée" corpus-confirmed; feminine form |
| fully open | entièrement ouvert | UMRS decision | perf_event_paranoid <= -1; no restrictions |
| kernel profiling allowed | profilage noyau autorisé | UMRS decision | perf_event_paranoid = 0 |
| user profiling allowed | profilage espace utilisateur autorisé | UMRS decision | perf_event_paranoid = 1 |

---

## C2PA / Provenance Terms (umrs-c2pa)

| English | French (fr_CA) | Source | Notes |
|---|---|---|---|
| tamper-evident (assertion) | révélatrice de falsification | UMRS decision | Termium Plus "tamper-evident" → "à preuve d'effraction" (physical packaging connotation, not suitable). Henri E-3 finding: the phrase must convey *detection*, not *prevention*. "Révélatrice de falsification" = reveals/detects tampering. The assertion does not block modification; it makes modification detectable via signature invalidation. Policy-significant distinction. |
| verification (cryptographic) | vérification | corpus:tar + corpus:cryptsetup + Termium Plus | Fully attested across corpus. Termium Plus "signature verification" → "vérification de signature"; "data integrity verification" → "vérification de l'intégrité des données". No ambiguity. |
| dual digest (SHA-256 + SHA-384) | double condensé | UMRS decision | No corpus or Termium entry for "dual digest" as a compound. "Condensé" is Termium Plus attested ("message digest" → "condensé de message"). "Double" is the standard fr_CA quantifier for a paired output. Used in DESCRIPTION section of man page as "deux condensés (SHA-256 et SHA-384)" for maximum clarity, and in the VÉRIFICATION DES CONDENSÉS section heading. |
| CNSA 2.0 | CNSA 2.0 | proper name | NSA Commercial National Security Algorithm Suite 2.0. No French equivalent. Retained as proper name in all locales. Referenced in NORMES section as "Suite cryptographique commerciale nationale 2.0 (NSA)" for informational gloss only. |
| FIPS-safe (algorithm qualifier) | FIPS-safe | spec vocabulary | Appears in enriched Algorithm line: `ES256  ECDSA / P-256 (prime256v1) / SHA-256 / 256-bit / FIPS-safe`. No Termium Plus entry. Henri D4 principle: specification vocabulary with no authoritative bilingual form retained in English. |
| crate (Rust dependency unit) | crate | UMRS decision | No Termium Plus entry. No OQLF GDT entry. "Caisse" (wooden box) is informal Rust community usage with no authority behind it. Jamie decision (2026-04-01): keep "crate" as an English loan word following the pattern of keeping technical terms from English-only ecosystems. Rust tooling is not yet in the Translation Project. |
| chain of custody | chaîne de possession | corpus:Termium Plus + OQLF GDT | Both Termium Plus (legal/forensic) and OQLF GDT agree. Confirmed by Henri C-2. |
| credential (PKI signing) | justificatif d'identité de signature | corpus:Termium Plus | Closest Termium Plus match for "credential" in IT security domain. Connotation leans toward identity documents; in C2PA context the credential is a certificate-and-key pair. Henri C-1: defensible, no change required. |
| trust anchor (PKI) | ancre de confiance | corpus:Termium Plus | Termium Plus standard for PKI trust anchor. |
| security marking (C2PA manifest field) | marquage de sécurité | corpus:Termium Plus | Termium Plus standard; preferred over "étiquette de sécurité" in CUI/C2PA context to avoid SELinux terminology collision. |
