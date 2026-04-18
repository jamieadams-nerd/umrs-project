# systemd UIDS-GIDS — Knowledge Collection README

**Collection:** systemd-uid-gid
**Familiarization date:** 2026-04-18
**Familiarized by:** Knox (security-auditor)
**Source:** `/DEVELOPMENT/umrs-project/.claude/references/systemd-uid-gid/uids-gids.html`
**Canonical URL:** https://systemd.io/UIDS-GIDS/
**Copyright:** systemd project, 2025. License: LGPL-2.1-or-later

---

## Document Count and Coverage

1 document — 575 lines of HTML — full text of the systemd UID/GID allocation specification.
Complete coverage; no chunking required.

---

## Summary

The systemd UIDS-GIDS specification is the de facto modern standard for UID/GID allocation
on systemd-based Linux distributions, which includes all current RHEL/CentOS Stream releases.
It refines LSB 5.0's broad "100+" definition into precise tiers covering: system users,
regular users, `systemd-homed` managed homes, container host UID mappings, dynamic greeter
sessions, dynamic service users (`DynamicUser=`), `systemd-nspawn` container UID ranges,
and foreign OS image UID ranges.

For UMRS, the critical tier is **1–999 (system users)** and specifically the RHEL 10
implementation of **201–999 (dynamic system allocation via `useradd -r`)**.

The document also specifies NSS resolution requirements: system users MUST be resolvable
without network, before `systemd-udevd.service` and `systemd-tmpfiles-setup.service`.
This mandates that UMRS service accounts live in local `/etc/passwd`.

---

## Artifact Files

| File | Description |
|---|---|
| `concept-index.md` | Full UID range breakdown plus NSS and container manager guidance |
| `cross-reference-map.md` | Agreements, tensions, chains with LSB 5 and FHS 2.3 |
| `style-decision-record.md` | UMRS-specific rulings on systemd compliance |
| `term-glossary.md` | Canonical systemd terms with RHEL 10 implementation evidence |

---

## Significant Gaps and Open Questions

1. **`pkg-config` query method** — systemd documents that UID boundaries can be queried
   via `pkg-config --variable=system_uid_max systemd`. UMRS install scripts could use this
   to programmatically verify the SYS_UID_MAX boundary rather than hardcoding 999.
2. **`DynamicUser=` for future UMRS units** — if any future UMRS systemd unit uses
   `DynamicUser=`, UIDs will be allocated from 61184–65519, not from `/etc/passwd`.
   The NSS synthesis handles this transparently, but operator tooling must be aware.
3. **HIC SVNT LEONES** — the range 2147483648–4294967294 is labeled "here be dragons."
   UIDs above 2^31 are problematic due to `devpts` and `setfsuid()` treating them as
   signed integers. UMRS must never produce UID values in this range.
