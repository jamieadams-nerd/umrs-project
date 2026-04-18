# FHS 3.0 — Style Decision Record

**Familiarized:** 2026-04-18

---

## SDR-FHS3-001: FHS 3.0 is now the preferred citation for all new work

**Context:** FHS 2.3 (2004) was the corpus baseline from initial familiarization. FHS 3.0
(2015) was acquired and familiarized 2026-04-18. RHEL 10 is a 3.0-era system; the section
numbers in the `fhs-lsb-uid-gid` skill are 3.0 decimal section numbers.

**Ruling:** FHS 3.0 is the authoritative citation for all new UMRS work. FHS 2.3 is retained
for historical context only. When a citation is needed in code, documentation, or compliance
artifacts, cite FHS 3.0 with decimal section numbers (e.g., "FHS 3.0 §3.13", "FHS 3.0 §5.12").

**Scope of change:**
- Skill `fhs-lsb-uid-gid/SKILL.md` — update to confirm 3.0 section numbers (done 2026-04-18)
- CHAIN.md — upgrade from trio to quartet (done 2026-04-18)
- FHS 2.3 SDR-FHS-001 placeholder — resolved; Option A selected (done 2026-04-18)
- Any doc or code citing "FHS 2.3 §..." for paths that exist unchanged in 3.0 — should be updated to cite 3.0 on next touch

**Remediation owner:** coder (skill), tech-writer (documentation)

---

## SDR-FHS3-002: Section citation for /etc/keys/umrs/ key material exception

**Tension:** The `fhs-lsb-uid-gid` skill cites "FHS Section 4.5" as justification for placing
key material outside the `/opt` hierarchy. FHS 3.0 §4.5 covers `/usr/include` (C header files)
and has no bearing on key material placement.

**Ruling:** The correct FHS citation for the `/etc/keys/umrs/` exception is the exception
clause in FHS 3.0 §3.13.2:

> "No other package files may exist outside the /opt, /var/opt, and /etc/opt hierarchies
> except for those package files that must reside in specific locations within the filesystem
> tree in order to function properly."

Cryptographic key material used by IMA/EVM tooling qualifies as files that "must reside in
specific locations in order to function properly" — IMA/EVM tools use well-known paths under
`/etc/keys/` and `/etc/ima/`. This exception clause, combined with NIST SP 800-53 CM-6
(Configuration Settings) and SA-8 (Security Engineering Principles), is the correct dual
citation. Remove all references to "FHS §4.5" in the context of key material.

**Remediation owner:** coder (skill update)

---

## SDR-FHS3-003: /run vs /var/run in UMRS code and documentation

**Ruling (inherited from FHS 2.3 SDR-FHS-004, now confirmed by FHS 3.0):**
Use `/run/umrs/` for all runtime data on RHEL 10. FHS 3.0 §3.15 makes `/run` a required
top-level directory; §5.13 explicitly demotes `/var/run` to a compatibility symlink. On RHEL 10,
`/var/run` is a symlink to `/run`. Both paths resolve to the same location, but cite and use
`/run` in new work.

**Citation:** FHS 3.0 §3.15 (for the `/run` requirement), §5.13 (for the `/var/run` deprecation).

**Remediation owner:** coder (if any UMRS code hardcodes `/var/run`), tech-writer (documentation)

---

## SDR-FHS3-004: /opt/umrs uses the <package> form, not <provider>

**Ruling (inherited from FHS 2.3 SDR-FHS-003, now confirmed by FHS 3.0 §3.13.1):**
`/opt/umrs/` uses the `<package>` form. LANANA provider registration is not required.
Deployment documentation must state this explicitly to preempt auditor questions.

**Citation:** FHS 3.0 §3.13.1 — "A package to be installed in /opt must locate its static
files in a separate /opt/<package> or /opt/<provider> directory tree, where <package> is a
name that describes the software package and <provider> is the provider's LANANA registered name."

**Remediation owner:** tech-writer

---

## SDR-FHS3-005: The /opt compliance triad must be cited as a unit

**Ruling:** When justifying UMRS path layout, cite all three FHS 3.0 sections together:
- §3.13 (/opt) — static data
- §3.7.4 (/etc/opt) — configuration
- §5.12 (/var/opt) — variable data

Citing §3.13 alone implies configuration and variable data have no standard home. The triad
is load-bearing for compliance documentation.

**Remediation owner:** tech-writer
