# Documentation Restructure History

Moved from MEMORY.md 2026-03-18 during housekeeping pass. This is reference material
for understanding how the current structure came to be — not needed in active context.

## Architecture Module — Complete Page List (2026-03-11)

All pages now in `docs/modules/architecture/pages/`:

| Page | Source |
|---|---|
| index.adoc | STW-2 (written 2026-03-10) |
| five-eyes-interop.adoc | STW-3 (written 2026-03-10) |
| mls-history.adoc | from historical/ |
| selinux-history.adoc | from historical/ |
| trusted-path-orange.adoc | from historical/ |
| HACAMS.adoc | from historical/ |
| ring-based-security.adoc | from historical/ |
| ibm-zos-os390.adoc | from historical/ |
| microsoft-nt-orange.adoc | from historical/ |
| one-way-hashes.adoc | from historical/ |
| openssl-no-vendoring.adoc | from historical/ |
| umrs-prog-lang.adoc | from devel/ (copy; devel/ retains its copy) |
| reference-monitor.adoc | from security/ (converted S-1) |
| rtb-vnssa.adoc | from security/ (converted S-1) |
| kernel-files-tpi.adoc | from security/ (converted S-1) |
| library-model.adoc | from security/ (converted S-1) |
| rationale-strongly-typed.adoc | from security/ (converted S-1, major expansion) |
| mls-label-model.adoc | Phase 2 A-1 (new) |
| integrity-and-provenance.adoc | Phase 2 A-2 (new) |
| case-studies.adoc | Phase 2 A-3 (new) |
| cui-structure.adoc | Phase 2 A-4 (new) |
| truth-concepts.adoc | Phase 2 A-5 (fully written 2026-03-14) |
| high-availability-history.adoc | new 2026-03-14 |

## Reference Module — Converted Pages (2026-03-11)

mls-colors.adoc, rhel-selinux-users.adoc, setrans-technical.adoc,
cui-category-abbreviations.adoc, example-setrans-conf.adoc,
fips-cryptography-cheat-sheet.adoc, umrs-mls-registry.adoc,
cui-descriptions.adoc, key-recommendation-list.adoc.

## File Locations After Reorganization (canonical)

| Files | Module | Subdirectory |
|---|---|---|
| rtb-vnssa, integrity-and-provenance, truth-concepts, reference-monitor | security-concepts | pages/ |
| selinux-history, five-eyes-interop, HACAMS, ibm-zos-os390, microsoft-nt-orange, ring-based-security, mls-history, one-way-hashes, trusted-path-orange | architecture | pages/history/ |
| category_set, security_type, role, user, context, sensitivity, booleans, secolor, mcs, mls-colors, rhel-selinux-users, setrans-technical, example-setrans-conf, umrs-mls-registry | reference | pages/selinux/ |
| openssl-no-vendoring, key-recommendation-list, fips-cryptography-cheat-sheet, crypto-post-quantum, crypto-policy-tiers, crypto-cpu-extensions, crypto-usage-map | cryptography | pages/ |
| cui-category-abbreviations, cui-descriptions | reference | pages/cui/ |
| logging-capacity, log-lifecycle-model, log-tuning | logging-audit | pages/ |
| rhel10-installation, rhel10-openscap, rhel10-packages, rhel10-setrans, rhel10-directory-structure | deployment | pages/rhel/ |
| ubuntu.adoc | deployment | pages/ubuntu/ |
| git-commit-signing | devel | pages/ |
| umrs-tooling, umrs-tool-shred, umrs-tool-shred-usage | umrs-tools | pages/ |

## Phase 2 Reorganization (2026-03-11)

| Pages | From | To |
|---|---|---|
| security-model.adoc | ROOT/pages/ | security-concepts/pages/ |
| case-studies.adoc, mls-classified-talk.adoc | architecture/pages/ | architecture/pages/history/ |
| structured-logging.adoc, how-to-structure-log.adoc | operations/pages/ | deployment/pages/ |
| auditing-noise.adoc | operations/pages/ | logging-audit/pages/ |
| TW0-NETIF-JUSTIFICATION.adoc | deployment/pages/ | deployment/pages/dual-network-interface.adoc |

## PQC Documentation Expansion (2026-03-13)

Round 1: Quantum Threat section, KEM/NIKE NOTE, CNSA 2.0 NOTE, Algorithm Replacement Mapping,
NIST IR 8547 deprecation, FIPS provider, SI-7 control mapping.

Round 2 (RHEL 10 availability): FALCON/FIPS 206, HQC, 14-candidate on-ramp, FIPS/PQC mutual
exclusion IMPORTANT block, RHEL 10 PQC Availability section, crypto-usage-map FIPS gate,
glossary Crypto Policy entry, deployment crypto policy section.

## Devel Guide Restructure (2026-03-13)

rust-style-guide.adoc, secure-bash.adoc, secure-python.adoc moved to reference/pages/.
devel/nav.adoc restructured. os-detection-deep-dive.adoc stays in devel/pages/.

## AI Transparency Module (2026-03-15)

13 pages written. nav.adoc covers all 13. Registered in antora.yml. ROOT nav updated.
ROOT ai-transparency.adoc retained as summary stub.

## Phases 3–4 Completion Notes (2026-03-12)

ROOT stubs populated. Glossary 25+ definitions. Crypto reference pages complete.
crypto-cpu-extensions.adoc was stub — replaced by cpu-extensions.adoc (2026-03-16).
Phase 4 plans archived.
