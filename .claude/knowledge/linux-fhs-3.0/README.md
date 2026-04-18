# FHS 3.0 — Knowledge Collection

**Collection:** linux-fhs-3.0
**Date of familiarization pass:** 2026-04-18
**Familiarized by:** Knox (security-auditor)
**Source:** `.claude/references/linux-fhs-3.0/fhs-3.0.txt` (and `.pdf`, SHA-256 in SOURCE.md)

---

## Summary

FHS 3.0 (March 19, 2015) is the current and authoritative release of the Filesystem Hierarchy
Standard, published by the Linux Foundation. It supersedes FHS 2.3 (2004). The primary
substantive change relevant to UMRS is the formalization of `/run` as a required top-level
directory and the demotion of `/var/run` to a compatibility symlink. Section numbering was
introduced in 3.0 (decimal, e.g., §3.13, §5.12) — these are the section numbers used by the
`fhs-lsb-uid-gid` skill and all UMRS compliance documentation.

FHS 3.0 is the correct citation for any RHEL 10 filesystem layout decision. FHS 2.3 is
retained in `.claude/knowledge/linux-fhs-2.3/` for historical reference only.

---

## Document Coverage

Documents processed: 1 (`fhs-3.0.txt`, 7 chapters + appendix)

Chapters:
- Ch 1: Introduction (purpose and scope)
- Ch 2: The Filesystem (shareable/static classification model)
- Ch 3: Root filesystem (all top-level directories including `/run`, `/opt`, `/etc`)
- Ch 4: /usr hierarchy (including `/usr/include` §4.5 and new `/usr/libexec` §4.7)
- Ch 5: /var hierarchy (including `/var/opt` §5.12 and `/var/run` §5.13 deprecation)
- Ch 6: Linux OS Annex
- Ch 7: Appendix

---

## Artifact Files

| File | Description |
|---|---|
| `concept-index.md` | Chapter-level summaries and diff table vs FHS 2.3 |
| `term-glossary.md` | Canonical directory terms, deprecated variants, and a MEDIUM citation error caught in the skill |
| `cross-reference-map.md` | Agreements/tensions/chains with systemd `file-hierarchy(7)`; gaps in FHS coverage |
| `style-decision-record.md` | Five SDRs covering version precedence, section citations, `/run` vs `/var/run`, LANANA form, and the opt-triad |

---

## Notable Findings

1. **FHS §4.5 citation error in skill** — The `fhs-lsb-uid-gid` skill cites "FHS §4.5" as
   the basis for placing key material outside `/opt`. §4.5 covers `/usr/include` (C headers).
   The correct citation is the exception clause in §3.13.2. Corrected in the skill update
   (2026-04-18). See SDR-FHS3-002.

2. **All other skill section numbers confirmed correct** — §3.13 (/opt), §3.7.4 and the
   prose reference to §3.8 (which actually references `/home`, not `/etc/opt`), §5.12
   (/var/opt), §3.15 (/run) all land exactly where the skill and compliance documents claim.
   Note: the skill table uses "FHS 3.8 (/etc/opt)" which is slightly imprecise — `/etc/opt`
   is at §3.7.4 (a subsection of §3.7 /etc), not §3.8. §3.8 is `/home`. This is corrected
   in the skill update.

3. **SUB_UID_MIN confirmed as 524288** — Fixed in skill; the old value of 100000 was incorrect.

4. **Tension T1 documented** — The systemd file-hierarchy Table 2 path conventions apply to
   `/usr` system packages, not `/opt` packages. UMRS correctly follows FHS §3.13/§5.12 for
   variable data under `/var/opt/umrs/`, which can be further subdivided following systemd
   conventions without any conflict.

---

## Open Questions

None. All SDR placeholders resolved as of this pass. Jamie's decision (SDR-FHS-001 from the
2.3 SDR) is recorded as resolved: FHS 3.0 is now the preferred citation for all new work.
