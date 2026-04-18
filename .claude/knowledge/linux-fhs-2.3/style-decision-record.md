# FHS 2.3 — Style Decision Record

**Familiarized:** 2026-04-18

---

## SDR-FHS-001: FHS 2.3 vs FHS 3.0 Version Decision

**Tension:** The corpus on disk is FHS **2.3** (2004). The `fhs-lsb-uid-gid` skill
claims "FHS 3.0 paths." FHS 3.0 (2015, Linux Foundation) exists and is the version
applicable to modern RHEL 10 deployments. Key practical difference: FHS 3.0 formally
adds `/run` as a top-level directory; FHS 2.3 only specifies `/var/run`.

**Option A — Replace with FHS 3.0:**
Download the FHS 3.0 PDF/HTML from https://refspecs.linuxfoundation.org/fhs.shtml,
extract to text, replace `fhs-2.3.txt` (or add alongside), update the skill to cite 3.0
section numbers, re-run familiarization.

**Option B — Keep FHS 2.3 as documented baseline:**
Acknowledge that UMRS documentation cites 2.3. For RHEL 10 deployment, note that
`/var/run` → `/run` (symlink) makes this a non-issue in practice. Update the skill to
correctly cite 2.3 section numbers and flag the version.

**Current ruling:** RESOLVED 2026-04-18 -- Option A selected. FHS 3.0 acquired, extracted,
and familiarized. FHS 3.0 is now the preferred citation for all new UMRS work. FHS 2.3
is retained in this directory for historical context only.

See .claude/knowledge/linux-fhs-3.0/ for the current authoritative artifacts.

---

## SDR-FHS-002: Section Number Citations in Skill

**Tension:** The fhs-lsb-uid-gid skill uses section numbers like "FHS 3.13 (/opt)",
"FHS 3.8 (/etc/opt)", "FHS 5.12 (/var/opt)" that do not exist in FHS 2.3. These appear
to be FHS 3.0 decimal-numbered section references applied to content that exists in 2.3
but without those numbers.

**Ruling:** RESOLVED 2026-04-18. FHS 3.0 confirmed (Option A above). Section numbers
verified against FHS 3.0 text:
- section 3.13 (/opt): CORRECT
- section 5.12 (/var/opt): CORRECT
- section 3.7.4 (/etc/opt): CORRECT -- note the skill said "3.8 (/etc/opt)" which is WRONG;
  /etc/opt is at section 3.7.4 (a subsection of 3.7 /etc), not section 3.8 (/home).
  Corrected in skill update 2026-04-18.
- section 4.5 (/usr/include) cited for key material: WRONG citation; see SDR-FHS3-002 in
  linux-fhs-3.0/style-decision-record.md. Corrected in skill update 2026-04-18.

**Remediation owner:** coder (skill file updated 2026-04-18)

---

## SDR-FHS-003: LANANA Provider Registration

**Tension:** FHS 2.3 §/opt distinguishes between `/opt/<package>` (no registration) and
`/opt/<provider>` (requires LANANA registration). UMRS uses `/opt/umrs/` without LANANA
registration.

**Ruling:** `/opt/umrs/` is the `<package>` form, not the `<provider>` form. The `<package>`
form does not require LANANA registration. This is compliant. Documentation should
explicitly state UMRS uses the `<package>` form to preempt auditor questions.

**Remediation owner:** tech-writer

---

## SDR-FHS-004: Runtime Data — `/var/run` vs `/run`

**Tension:** FHS 2.3 specifies `/var/run`. RHEL 10 / systemd uses `/run`.

**Ruling:** For UMRS on RHEL 10, use `/run/umrs/` for any runtime sockets or PID files.
`/var/run` is a symlink to `/run` on RHEL 10; both paths work, but `/run` is canonical
for modern systems and avoids confusion when auditors check FHS version compliance.
This ruling holds regardless of FHS version decision in SDR-FHS-001.

**Remediation owner:** coder (if any UMRS code hardcodes `/var/run`)
