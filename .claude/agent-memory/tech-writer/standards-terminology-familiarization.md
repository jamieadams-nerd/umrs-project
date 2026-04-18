---
name: FHS / LSB / systemd Standards Familiarization
description: Canonical terminology, citation patterns, and deployment-doc rules derived from FHS 3.0, LSB 5.0, and systemd file-hierarchy(7). Updated after Knox's incremental familiarization pass on FHS 3.0 and file-hierarchy(7).
type: project
---

# FHS / LSB / systemd — Technical Writer Familiarization

**Date:** 2026-04-18 (refreshed from FHS 2.3 draft)
**Source artifacts:** FHS 3.0 (fhs-3.0.txt — Linux Foundation, 2015; FreeDesktop, 2025),
LSB 5.0 §23, systemd file-hierarchy(7) man page, Knox's incremental familiarization pass
**Directly verified sections:** FHS 3.0 §3.7.4, §3.7.4.1, §3.7.4.2, §3.13, §3.13.1,
§3.13.2, §5.12, §5.12.1
**Purpose:** Active terminology and citation rules for UMRS deployment documentation.

---

## 1. Collection Summaries

**FHS 3.0** — Reach for this when justifying where UMRS puts files. It is the normative
basis for /opt/umrs/ (§3.13 — add-on application), /etc/opt/umrs/ (§3.7.4 —
host-specific configuration for /opt packages), and /var/opt/umrs/ (§5.12 — variable
data for /opt packages). The matching-subdir requirement (/etc/opt/<subdir> where
<subdir> equals the /opt name) is mandatory, not advisory. Use FHS when an auditor
asks why a path lives where it does.

/etc/keys/umrs/ is FHS-compliant under the §3.7.4.2 exception clause: "If a
configuration file must reside in a different location in order for the package or system
to function properly, it may be placed in a location other than /etc/opt/<subdir>."
Raw key material with strict mode-0700 isolation is the functional requirement that
exception anticipates. Document /etc/keys/umrs/ as FHS-compliant — not as a deviation.

**LSB 5.0 §23** — Reach for this when justifying the umrs system account. LSB §23.3
defines the formal dynamic allocation range for system UIDs (100–499, expressed as
"should") and is the only Linux Foundation-standardized authority for UID ranges. Use
LSB when a claim needs a formal standards body behind it, not just a de facto spec. Also
use LSB to confirm UMRS does not conflict with required or optional account names.

**systemd file-hierarchy(7)** — Reach for this when explaining runtime path semantics,
service-unit directory directives (RuntimeDirectory=, StateDirectory=, CacheDirectory=),
or when an auditor needs to know why /run/umrs/ is created by the service unit rather
than the install script. Also use this to confirm Table 2 (/etc/package/,
/var/lib/package/) applies to /usr/-form packages — not to UMRS, which is an
/opt/-form package. Do not misapply Table 2 slots to UMRS paths.

**systemd UIDS-GIDS** — Reach for this when LSB's 100–499 range feels too narrow (RHEL 10
allocates 201–999) or when explaining early-boot NSS resolvability requirements. systemd
fills the LSB gap from 500–999, mandates that system accounts live in local /etc/passwd,
and establishes /run as the modern runtime path. This is the de facto authority on all
UID decisions above 499.

---

## 2. Top 20 Canonical Terms

| # | Canonical Form | Definition (one line) |
|---|---|---|
| 1 | SYS_UID_MIN | RHEL 10 login.defs key; value 201; lower bound for useradd -r allocations |
| 2 | SYS_UID_MAX | RHEL 10 login.defs key; value 999; upper bound for useradd -r allocations |
| 3 | SUB_UID_MIN | Sub-UID range start for user namespace mapping; authoritative value is 524288, not 100000 |
| 4 | system user | A UID 1–999 account used for daemon privilege separation; not a human; must resolve without network |
| 5 | regular user | A UID 1000–60000 account mapping to an actual human; may use LDAP/NIS |
| 6 | /run/ | Canonical runtime path on RHEL 10; a top-level tmpfs; /var/run is a symlink to it |
| 7 | /var/run | FHS 3.0 term for runtime variable data; on RHEL 10 this is a symlink pointing to /run/ |
| 8 | add-on application software package | FHS 3.0 §3.13.1 term for software installed under /opt; UMRS is an add-on application |
| 9 | /opt/<package> | FHS 3.0 §3.13.2 form for a package-named directory under /opt; does not require LANANA registration |
| 10 | /opt/<provider> | FHS 3.0 §3.13.2 form for a provider-named directory under /opt; requires LANANA registration |
| 11 | LANANA | Linux Assigned Names and Numbers Authority; manages namespace assignments; does not define UID/GID ranges |
| 12 | shareable | FHS term: files that can be stored on one host and used on others (e.g., /opt, /usr) |
| 13 | static | FHS term: files that do not change without sysadmin intervention; can reside on read-only media |
| 14 | variable | FHS term: files that change during normal system operation; live under /var |
| 15 | DynamicUser= | systemd unit directive allocating a transient UID from 61184–65519; do not use for umrs service |
| 16 | nss-systemd | systemd NSS module that synthesizes records for UIDs 0 and 65534; does NOT synthesize the umrs account |
| 17 | useradd -r | LSB-specified command for creating system accounts; reads SYS_UID_MIN/SYS_UID_MAX from login.defs |
| 18 | nobody | UID 65534; overflow/unmappable user; use nobody for both user and group name (not nogroup) |
| 19 | HIC SVNT LEONES | systemd label for UIDs >= 2147483648; avoid — kernel treats these as signed 32-bit and fails |
| 20 | tty group | GID 5 hard assignment in systemd; must be constant across all systemd systems |
| 21 | RuntimeDirectory= | systemd.exec(5) directive; creates /run/<name>/ at service startup and removes it on stop; use for /run/umrs/ |
| 22 | StateDirectory= | systemd.exec(5) directive; creates /var/lib/<name>/ and manages ownership; applies to /usr/-package state |
| 23 | CacheDirectory= | systemd.exec(5) directive; creates /var/cache/<name>/; application must tolerate flush |
| 24 | /var/lib/package/ | file-hierarchy(7) Table 2 slot for persistent private state — applies to /usr/-form packages, NOT to /opt/-form packages such as UMRS |
| 25 | /etc/package/ | file-hierarchy(7) Table 2 slot for system-specific config — applies to /usr/-form packages, NOT to UMRS |

---

## 3. Section-Citation Patterns

When deployment documentation cites these standards, use FHS 3.0 decimal section numbers.
The previous guidance to use prose form only is obsolete — it applied when only FHS 2.3
was on hand. FHS 2.3 references are retained for historical notes only.

**FHS 3.0:** Use decimal section numbers. Verify against fhs-3.0.txt before writing any
citation. Confirmed mappings (read directly from source on 2026-04-18):

| Section | Path | Heading in spec |
|---|---|---|
| §3.7 | /etc | Host-specific system configuration |
| §3.7.1 | /etc | Purpose |
| §3.7.4 | /etc/opt | Configuration files for /opt |
| §3.7.4.1 | /etc/opt | Purpose |
| §3.7.4.2 | /etc/opt | Requirements (exception clause lives here) |
| §3.8 | /home | User home directories (optional) — NOT /etc/opt |
| §3.13 | /opt | Add-on application software packages |
| §3.13.1 | /opt | Purpose |
| §3.13.2 | /opt | Requirements (package vs provider forms) |
| §4.5 | /usr/include | Header files — NOT related to /opt or key material |
| §5.12 | /var/opt | Variable data for /opt |
| §5.12.1 | /var/opt | Purpose |

Correct citation forms:
- "FHS 3.0 §3.13" when citing the /opt hierarchy
- "FHS 3.0 §3.7.4" when citing /etc/opt
- "FHS 3.0 §3.7.4.2 exception clause" when justifying /etc/keys/umrs/
- "FHS 3.0 §5.12" when citing /var/opt

Wrong citation forms (never use):
- "FHS §3.8 (/etc/opt)" — §3.8 is /home
- "FHS §4.5" in any key-material or /opt/ context — §4.5 is /usr/include

**LSB:** Decimal section numbers are stable and correct for LSB 5.0.
- Correct: "LSB 5.0 §23.3", "LSB 5.0 §23.2 Table 23-1"

**systemd file-hierarchy(7):** Cite by table name or prose section (no decimal section numbers).
- Correct: "file-hierarchy(7) Table 2" when citing the /usr/-package slots
- Correct: "per file-hierarchy(7), RuntimeDirectory= creates /run/<name>/ at service startup"

**RHEL 10 login.defs:** Always cite as implementation evidence, never as the normative source.
- Correct: "RHEL 10 /etc/login.defs (implementation evidence)"

---

## 4. Three-Source Citation Rule

Any deployment documentation claim about UMRS UID/GID compliance requires all three
sources. LSB alone is insufficient (only "should"; tops out at 499). systemd alone is
insufficient (de facto, not formal). RHEL 10 login.defs alone is insufficient
(implementation, not specification).

Required citation block for UID compliance claims:

> Per LSB 5.0 §23.3, UIDs 100–499 are reserved for dynamic allocation by system
> administrators using useradd. The systemd UIDS-GIDS specification extends the
> system user range to 1–999 and requires early-boot local resolution. RHEL 10 implements
> this as SYS_UID_MIN=201, SYS_UID_MAX=999 in /etc/login.defs.

Audit response pattern (if an auditor challenges a UID in 500–999):
Cite systemd and RHEL 10 login.defs. LSB says "should" not "shall" for the 100–499
range — the constraint is a recommendation, not a hard requirement. A UID in 201–499
satisfies all three sources simultaneously and is preferable when an auditor may raise this.

---

## 5. Knox's 5-Item Punch List — Lucia Action Items

Apply these corrections when touching deployment docs. Each item corrects a pattern
identified by Knox during incremental familiarization of FHS 3.0 and file-hierarchy(7).

1. **§3.8 to §3.7.4 correction**: Any documentation citing "FHS §3.8 (/etc/opt)" must be
   changed to "FHS 3.0 §3.7.4 (/etc/opt)". §3.8 in FHS 3.0 is /home, not /etc/opt.

2. **§4.5 key-material citations are wrong**: Any documentation citing "FHS §4.5" in a
   key-material or /opt/ package-autonomy context must be changed to "FHS 3.0 §3.13.2"
   plus NIST CM-6/SA-8. §4.5 in FHS 3.0 is /usr/include, entirely unrelated.
   NOTE: key-material-trees.adoc §6.3 cites "FHS §4.5 autonomy" — this section number
   is wrong. Flagged to Jamie for correction. The argument in §6.3 is architecturally
   correct; only the citation is wrong.

3. **package form — state explicitly**: UMRS uses the <package> form under /opt
   (no LANANA registration). Any page discussing /opt layout that omits this distinction
   is incomplete. LANANA registration applies only to the <provider> form (§3.13.2).

4. **/var/opt/umrs/ internal layout naming**: When directory-purpose-matrix.adoc
   documents the internal layout of /var/opt/umrs/, use the state/cache/log naming
   convention (SDR-FH-001 in Knox's artifacts). Cite both FHS 3.0 §5.12 (normative
   requirement) AND file-hierarchy(7) Table 2 (naming convention reference), noting that
   Table 2 formally applies to /usr/-packages but the naming convention is adopted for
   consistency.

5. **/run/umrs/ creation mechanism**: /run/umrs/ is created by RuntimeDirectory=umrs
   in the UMRS service unit, not by umrs-install.sh. file-hierarchy(7) explicitly
   cites RuntimeDirectory= as the correct mechanism for creating /run/<name>/ at
   service startup. Do not document the install script as responsible for /run/umrs/.

---

## 6. Section-Number Verification Table

Populated by reading fhs-3.0.txt directly on 2026-04-18. Do not update from memory or
distillation — open the source file to re-verify before adding a new entry.

| Claim | FHS 3.0 section | Verified |
|---|---|---|
| /opt is for add-on application software | §3.13.1 | Yes — 2026-04-18 |
| <package> form needs no LANANA registration | §3.13.2 | Yes — 2026-04-18 |
| /etc/opt/<subdir> for add-on package config | §3.7.4.1 | Yes — 2026-04-18 |
| Exception clause allows /etc/keys/umrs/ | §3.7.4.2 | Yes — 2026-04-18 |
| §3.8 is /home, NOT /etc/opt | §3.8 heading | Yes — 2026-04-18 |
| §5.12 is /var/opt | §5.12.1 | Yes — 2026-04-18 |

Note: §4.5 in FHS 3.0 covers /usr/include. It is not cited in any FHS section dealing
with /opt or key material. Any citation of §4.5 in a deployment-doc context is wrong.

---

## 7. Known Adoc Issues — Do Not Self-Correct

These are errors I found by reading source. They are flagged to Jamie. Do not
independently edit the adoc files without instruction.

**key-material-trees.adoc, Standards Overview table (lines 65–72):**
- Lists §4.5 for /opt/ — wrong. Correct section is §3.13.
- Lists §3.13 for /etc/opt/ — wrong. Correct section is §3.7.4.
The prose and architecture of that page are correct. Only the section-number table
has wrong entries.

**key-material-trees.adoc §6.3 (FHS /opt/ Autonomy vs NIST Configuration Control):**
- Cites "FHS §4.5 autonomy" in two prose locations — wrong. §4.5 is /usr/include.
- Correct citation for package autonomy under /opt is §3.13.2.
- The argument and resolution in §6.3 are architecturally sound; the citation is not.

---

## 8. Intentional Non-Issues — Do Not Flag

**key-material-trees.adoc IMPORTANT box**: /var/lib/umrs/keys/ directories are
"declared and typed in policy but are not created by umrs-install.sh in the current
release." This is documented intent, not an oversight. Session key lifecycle is future
work. Do not flag this when reviewing deployment docs.

**file-hierarchy(7) Table 2 does not apply to UMRS**: Table 2 slots (/etc/package/,
/var/lib/package/) are for /usr/-form packages. UMRS's authority for config and variable
data is FHS §3.7.4 and §5.12. Drafts that apply Table 2 directly to UMRS paths should
be corrected to cite the FHS sections instead.

---

## 9. Settled Architecture Decisions — Do Not Re-Open

- UMRS is an /opt/-form package: /opt/umrs/, /etc/opt/umrs/, /var/opt/umrs/.
  Documented in docs/modules/deployment/pages/umrs/5b-directory-structure/key-material-trees.adoc.
- /etc/keys/umrs/ is FHS-compliant under §3.7.4.2. It is not a deviation.
- /var/lib/umrs/keys/ is the runtime key state path; intentionally declared, not yet
  created by install script. Session key lifecycle is future work.
- No key material under /opt/umrs/ or /var/opt/umrs/ — deliberate NIST CM-6/SA-8
  compliance decision, documented in key-material-trees.adoc §6.3.

---

## 10. Tensions

**T1: /opt/umrs/ — package form vs provider form**
FHS 3.0 §3.13.2 allows both /opt/<package> (no registration needed) and
/opt/<provider> (LANANA registration required). UMRS uses the package form. Documentation
must state this explicitly: "UMRS is installed as an add-on package under /opt/umrs/
per FHS 3.0 §3.13 — the <package> form, which does not require LANANA registration."

**T2: LSB 100–499 vs systemd 1–999 vs RHEL 10 201–999**
All three ranges are compatible for a UID in 201–499. A UID anywhere in 500–999 satisfies
systemd and RHEL 10 but is technically outside LSB's stated recommendation. Flag if UMRS
account ends up above 499 — cite systemd + RHEL 10 and note LSB is advisory ("should").

**T3: file-hierarchy(7) Table 2 vs FHS §5.12**
file-hierarchy(7) Table 2 naming conventions (state, cache, log) are worth adopting
for /var/opt/umrs/ internal layout — but the normative authority for that path is FHS
§5.12, not Table 2. Cite both: §5.12 for the normative requirement, Table 2 for naming
convention rationale.
