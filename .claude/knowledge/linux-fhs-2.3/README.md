# Linux FHS 2.3 — Knowledge Collection README

**Collection:** linux-fhs-2.3
**Familiarization date:** 2026-04-18
**Familiarized by:** Knox (security-auditor)
**Source:** `/DEVELOPMENT/umrs-project/.claude/references/linux-fhs-2.3/fhs-2.3.txt`

---

## CRITICAL VERSION NOTICE

The file on disk is **FHS 2.3** (copyright 1994–2004, editors Russell/Quinlan/Yeoh).
The `fhs-lsb-uid-gid` skill claims "FHS 3.0 paths." **This is incorrect.**

FHS 3.0 was released in 2015 by the Linux Foundation/LSB workgroup. It is not in the corpus.
A decision is required: download FHS 3.0 and replace this file, or document 2.3 as the intentional baseline.
See `style-decision-record.md` for the formal placeholder.

---

## Document Count and Coverage

1 document — 2,711 lines — full text of the Filesystem Hierarchy Standard version 2.3.

Chapters covered:
- Ch 1: Introduction and scope
- Ch 2: The Filesystem (shareable/static classification model)
- Ch 3: Root Filesystem (`/bin`, `/boot`, `/dev`, `/etc`, `/etc/opt`, `/home`, `/lib`, `/media`, `/mnt`, `/opt`, `/root`, `/sbin`, `/srv`, `/tmp`)
- Ch 4: The `/usr` Hierarchy
- Ch 5: The `/var` Hierarchy (including `/var/opt`)
- Ch 6: Linux OS Specific Annex
- Ch 7: Appendix

---

## Summary

FHS 2.3 defines the canonical filesystem layout for Unix-like systems. It establishes where
applications may install files (`/opt/<package>`), where configuration goes (`/etc/opt/<package>`),
where variable data goes (`/var/opt/<package>`), and prohibits applications from creating
top-level root directories. The UMRS path layout (`/opt/umrs/`, `/etc/opt/umrs/`, `/var/opt/umrs/`)
is directly grounded in this standard.

FHS 2.3 does **not** define UID/GID allocation — that is LSB §23's domain.

---

## Artifact Files

| File | Description |
|---|---|
| `concept-index.md` | Per-chapter concept summaries and key terms |
| `cross-reference-map.md` | Agreements, tensions, chains, and gaps with LSB 5 and systemd |
| `style-decision-record.md` | Project-specific rulings including the FHS 2.3 vs 3.0 decision placeholder |
| `term-glossary.md` | Canonical FHS terms with definitions and source sections |

---

## Significant Gaps and Open Questions

1. **Version gap**: FHS 2.3 (2004) vs FHS 3.0 (2015). Key 3.0 additions include formalizing `/run` as a top-level directory (FHS 2.3 still specifies `/var/run`). UMRS deployment using `/run` under RHEL 10 is governed by 3.0, not 2.3.
2. **`/run` absent**: FHS 2.3 specifies `/var/run` for runtime data. RHEL 10 uses `/run` (a 3.0/systemd convention). The skill currently cites FHS section numbers that may not be accurate for 2.3.
3. **LANANA registration**: FHS 2.3 requires `/opt/<provider>` be a LANANA-registered name. The `umrs` name is not verified as LANANA-registered. This may be a compliance gap.
