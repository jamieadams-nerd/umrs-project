# FHS 3.0 — Concept Index

**Source:** `fhs-3.0.txt` | **Version:** FHS 3.0 (March 19, 2015) | **Familiarized:** 2026-04-18

---

## Document: Filesystem Hierarchy Standard 3.0

**Short ID:** FHS-3.0

**What it covers:** The current (and final, as of 2026) release of the Filesystem Hierarchy
Standard, published by the Linux Foundation. Supersedes FHS 2.3 (2004). Defines required and
recommended directory structure for Unix-like operating systems. FHS 3.0 is the version
relevant to RHEL 10 deployments: it formally standardizes `/run`, deprecates `/var/run` as
primary (retaining it as a compatibility symlink), and adds `/usr/libexec` as optional. All
decimal section numbers in the `fhs-lsb-uid-gid` skill derive from this version.

**Key terms and concepts:**
- Shareable vs. unshareable files (unchanged from 2.3)
- Static vs. variable files (unchanged from 2.3)
- `/run` — new top-level directory (formalized in 3.0; was `/var/run` in 2.3)
- `/var/run` — now a compatibility symlink to `/run`; programs should migrate to `/run`
- `/opt` — add-on application software packages (§3.13)
- `/etc/opt/<subdir>` — host-specific configuration for `/opt` packages (§3.7.4)
- `/var/opt/<subdir>` — variable data for `/opt` packages (§5.12)
- `/usr/libexec` — binaries run by other programs (optional, new in 3.0; §4.7)
- LANANA-registered provider names for `/opt/<provider>` directories
- The four-quadrant filesystem model (shareable/static, unshareable/static, shareable/variable, unshareable/variable)
- Prohibition on applications creating new top-level root directories

**What it governs:**
- All UMRS path decisions (`/opt/umrs/`, `/etc/opt/umrs/`, `/var/opt/umrs/`, `/run/umrs/`)
- Canonical authority for section-number citations (§3.13, §3.7.4, §5.12, §4.5, §3.15)
- Justification for `/run` over `/var/run` on RHEL 10
- Whether UMRS binaries belong in `/opt/umrs/bin/` (yes — §3.13 `<package>` form)
- Confirmation that `/opt/umrs/` uses the `<package>` form (no LANANA registration required)

**Related documents:**
- FHS 2.3 (2004) — predecessor; use 3.0 for all new work; see diff notes below
- systemd `file-hierarchy(7)` (RHEL 10, systemd 257-23.el10) — extends FHS 3.0 for systemd-managed systems
- LSB 5.0 §23 — UID/GID allocation (FHS does not cover this)
- `fhs-lsb-uid-gid` skill — synthesizes the full compliance chain

---

## Chapter-Level Summaries

### Ch 1 — Introduction (§1)

Purpose and scope unchanged from 2.3. FHS governs interoperability of file placement across
distributions, not local customization. Two-sentence scope: enables software and users to
predict file locations; does not usurp local sysadmin decisions.

---

### Ch 2 — The Filesystem (§2)

The shareable/static classification matrix is unchanged from 2.3. The four-quadrant example
now lists `/var/run` under unshareable/variable — but FHS 3.0 body text clarifies `/var/run`
is superseded by `/run` (§3.15, §5.13). Agents should read the Ch 2 example as historical;
`/run` is canonical.

---

### Ch 3 — Root Filesystem (§3)

Defines all top-level directories. `/run` is now in the required directory list (§3.2),
alongside `bin`, `boot`, `dev`, `etc`, `lib`, `media`, `mnt`, `opt`, `sbin`, `srv`, `tmp`,
`usr`, `var`. This is the most significant structural change from 2.3.

Critical sections for UMRS:

**`/etc`** (§3.7): Host-specific configuration. No binaries.
- **`/etc/opt`** (§3.7.4): Configuration for `/opt` packages. Must use `/etc/opt/<subdir>`
  where `<subdir>` matches the subtree name in `/opt`. No internal structure imposed.

**`/home`** (§3.8): Optional. No program should assume a specific home directory location.

**`/opt`** (§3.13): Add-on application software. The canonical UMRS installation point.
- Static files in `/opt/<package>` or `/opt/<provider>` (LANANA-registered).
- `/opt/bin`, `/opt/doc`, `/opt/include`, `/opt/info`, `/opt/lib`, `/opt/man` reserved for sysadmin use.
- User-invocable programs must be in `/opt/<package>/bin`.
- Variable data → `/var/opt`. Configuration → `/etc/opt`.
- No package files outside `/opt`, `/var/opt`, `/etc/opt` except where function requires otherwise.
- `<package>` form (e.g., `/opt/umrs`) does NOT require LANANA registration.

**`/run`** (§3.15): **NEW in FHS 3.0.** Run-time variable data. Flushed at boot.
- PID files: `/run/<program-name>.pid`
- UNIX domain sockets must be placed in `/run` or a subdirectory.
- Supersedes `/var/run` (§5.13 relegates `/var/run` to compatibility-only).

**`/srv`** (§3.17): Data for services. Clarified in 3.0 as site-specific, no imposed structure.

---

### Ch 4 — /usr Hierarchy (§4)

`/usr` is shareable, read-only. Key section:

**`/usr/include`** (§4.5): Standard C include files. No change from 2.3.

**`/usr/libexec`** (§4.7): **New optional section in FHS 3.0.** Binaries run by other programs
(not directly invoked from shell). Useful for daemon helper binaries that should not appear
in `$PATH`. This is the FHS 3.0 authorization for the pattern used by systemd's `/usr/lib/<package>/`.

**`/usr/share`** (§4.11): Architecture-independent data. Man pages, documentation, locale data.

---

### Ch 5 — /var Hierarchy (§5)

**`/var/opt`** (§5.12): Variable data for `/opt` packages. Must use `/var/opt/<subdir>`;
subdir matches package name in `/opt`. No imposed internal structure. "Except where superseded
by another file in /etc" (unusual edge case; key material exception is a real-world example).

**`/var/run`** (§5.13): **Now deprecated as primary.** FHS 3.0 explicitly states that
`/var/run`'s functions have moved to `/run`. Valid to implement as symlink to `/run`.
UMRS on RHEL 10 should use `/run/umrs/` — `/var/run` → `/run` is the RHEL 10 implementation.

**`/var/lib`** (§5.8): Variable state information. State persisting between reboots.
**`/var/cache`** (§5.5): Application cache data — deletable without data loss.
**`/var/log`** (§5.10): Log files.

---

### Ch 6 — OS Specific Annex (§6.1 Linux)

Adds Linux-specific requirements: `/proc`, `/sys`, `/dev/null`, `/dev/zero`, `/dev/tty`.
Largely unchanged from 2.3.

---

## Key Diffs: FHS 3.0 vs FHS 2.3

| Topic | FHS 2.3 | FHS 3.0 | Impact |
|---|---|---|---|
| `/run` | Not a top-level dir; use `/var/run` | Top-level, required (§3.2, §3.15) | UMRS uses `/run/umrs/`, not `/var/run/umrs/` |
| `/var/run` | Primary runtime data directory | Compatibility symlink to `/run` (§5.13) | Cite §3.15 for new work; §5.13 explains why |
| `/usr/libexec` | Not mentioned | Optional (§4.7) | Available for daemon helper binaries |
| `/srv` | Brief mention | Clarified: site-specific, no imposed structure (§3.17) | No impact on UMRS |
| Section numbering | No decimal section numbers in text | Full decimal numbering (§3.7.4, §3.13, §5.12, etc.) | All skill/doc citations must use 3.0 numbers |
| `hwclock adjtime` | `/etc/adjtime` | Moved to `/var/lib/hwclock/adjtime` (§5.8.6, noted as change from FHS 2.1) | Informational |
| Copyright year | 1994–2004 | 1994–2015 | Document currency |

**Decision guidance for agents:** When choosing between citing FHS 2.3 or 3.0:
- `/run` vs `/var/run` → cite FHS 3.0 §3.15 (2.3 is wrong for modern systems)
- `/opt`, `/etc/opt`, `/var/opt` → cite FHS 3.0 §3.13, §3.7.4, §5.12 (content unchanged, but 3.0 has proper section numbers)
- Historical baseline or legacy system discussion → cite FHS 2.3
- All new work on RHEL 10 → FHS 3.0 is the authoritative source
