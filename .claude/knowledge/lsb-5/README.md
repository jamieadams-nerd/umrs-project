# LSB 5.0 — Knowledge Collection README

**Collection:** lsb-5
**Familiarization date:** 2026-04-18
**Familiarized by:** Knox (security-auditor)
**Source directory:** `/DEVELOPMENT/umrs-project/.claude/references/lsb-5/`

---

## Document Inventory

| File | Lines | Type | Priority for UMRS |
|---|---|---|---|
| `LSB-Common.txt` | 1,409 | Common definitions, LSB licensing/scope | Low |
| `LSB-Core-generic.txt` | 70,945 | Core standard — **§23 UID/GID is here** | HIGH |
| `LSB-Core-AMD64.txt` | 21,058 | AMD64 architecture extensions to Core | Medium |
| `LSB-Desktop-generic.txt` | 321,339 | Desktop APIs — not relevant to UMRS server | Low |
| `LSB-Desktop-AMD64.txt` | 51,609 | AMD64 desktop extensions | Low |
| `LSB-Imaging.txt` | 5,518 | Imaging API standard | Low |
| `LSB-Languages.txt` | 21,498 | Language runtime standards (Python, etc.) | Low |
| `LSB-TrialUse.txt` | 57,083 | Experimental/trial-use specifications | Low |

**Note:** LSB-Desktop-generic.txt (321K lines) was not fully read — it is a GUI/desktop
standard with no relevance to UMRS server infrastructure. The UID/GID content,
filesystem, and account management material is entirely in LSB-Core-generic.txt §23.

---

## Summary

LSB 5.0 (Linux Standard Base, 2015, Linux Foundation) is the formal standard body
specification for Linux distributions. It defines binary compatibility, required commands,
filesystem layout (by reference to FHS), and — critically for UMRS — the authoritative
UID/GID allocation ranges in Chapter 23.

For UMRS purposes, the binding content is:
- **§23.2** — Required and optional user/group names (Tables 23-1 and 23-2)
- **§23.3** — UID ranges: 0–99 statically allocated, 100–499 dynamic system allocation
- **§23.4** — Rationale for optional names

LSB 5.0 does NOT define the 1000+ regular user split; that is systemd's refinement.
LSB 5.0 does NOT mandate login shells, home directories, or password locking for system
accounts; those constraints come from systemd spec and security best practice.

---

## Artifact Files

| File | Description |
|---|---|
| `concept-index.md` | §23 chapter breakdown plus scope of other modules |
| `cross-reference-map.md` | Agreements, tensions, chains with FHS and systemd |
| `style-decision-record.md` | UMRS-specific rulings on LSB compliance |
| `term-glossary.md` | Canonical LSB terms, especially §23 account terminology |

---

## Significant Gaps and Open Questions

1. **LSB 5.0 defines 100–499 as dynamic system range.** systemd refines this to 100–999.
   RHEL 10 implements 201–999 via `SYS_UID_MIN=201`. The existing compliance report
   handles this three-source chain correctly.
2. **LSB §23 uses "should"** for the UID range allocations — these are recommendations,
   not absolute requirements ("SHALL"). This weakens LSB alone as a compliance basis;
   the three-source chain (LSB + systemd + RHEL 10 login.defs) is necessary.
3. **`bin` and `daemon` legacy accounts**: LSB §23.2 marks both as legacy. Modern
   applications should use individual UIDs. UMRS correctly uses the `umrs` dedicated
   account rather than `daemon`.
