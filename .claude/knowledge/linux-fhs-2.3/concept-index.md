# FHS 2.3 — Concept Index

**Source:** `fhs-2.3.txt` | **Version:** FHS 2.3 (2004) | **Familiarized:** 2026-04-18

---

## Document: Filesystem Hierarchy Standard 2.3

**Short ID:** FHS-2.3

**What it covers:** Defines the required and recommended directory structure for Unix-like
operating systems. Establishes where applications, distributions, and system administrators
may place binaries, configuration, variable data, and temporary files. Primary normative
source for UMRS filesystem path decisions.

**Key terms and concepts:**
- Shareable vs. unshareable files (cross-host vs. host-specific)
- Static vs. variable files (immutable vs. runtime-modified)
- The four-quadrant filesystem model (shareable/static = `/usr`; unshareable/static = `/etc`; shareable/variable = `/var/mail`; unshareable/variable = `/var/run`)
- `/opt` as the canonical location for add-on application packages
- `/etc/opt/<package>` for host-specific configuration of opt packages
- `/var/opt/<package>` for variable data of opt packages
- LANANA-registered provider names for `/opt/<provider>` directories
- `/var/run` for PID files and UNIX domain sockets (note: superseded by `/run` in FHS 3.0/systemd)
- The prohibition on applications creating top-level root directories
- `/usr` as shareable, read-only data; must not be written during normal operation

**What it governs:**
- All UMRS path decisions (`/opt/umrs/`, `/etc/opt/umrs/`, `/var/opt/umrs/`)
- Justification for placing key material outside `/opt` (FHS allows exceptions where required by function)
- Where UMRS binaries, config, logs, and state live
- Whether `/var/run` or `/run` is used for PID/socket files (version-dependent)

**Related documents:**
- LSB-Core-generic.txt §23 (UID/GID allocation — FHS does not cover this)
- systemd uids-gids.html (runtime path conventions including `/run`)
- `fhs-lsb-uid-gid` skill (synthesizes this corpus for agent decisions)

---

## Chapter-Level Summaries

### Ch 2 — The Filesystem

Establishes the shareable/static classification matrix. This is the theoretical foundation
for why `/usr` is read-only and `/var` is writable. UMRS uses this model: `/opt/umrs/` is
static and shareable; `/var/opt/umrs/` is variable and unshareable (host-specific logs/state).

Key terms: shareable, unshareable, static, variable.

---

### Ch 3 — Root Filesystem

Defines all top-level directories. Critical sections for UMRS:

**`/etc`** (§3.5): Host-specific configuration. No binaries. Static only.
- `/etc/opt` (§3.5.1): Configuration for `/opt` packages goes here as `/etc/opt/<subdir>`.
  The subdir name must match the subtree name in `/opt`.

**`/opt`** (§3.13): Add-on application software.
- Static files go in `/opt/<package>` or `/opt/<provider>`.
- `/opt/bin`, `/opt/doc`, `/opt/include`, `/opt/info`, `/opt/lib`, `/opt/man` are reserved
  for local system administrator use — packages must not use them directly.
- Programs invoked by users must be in `/opt/<package>/bin`.
- Variable data goes to `/var/opt`. Configuration goes to `/etc/opt`.
- LANANA provider registration required for `/opt/<provider>` directory names.

**`/home`** (§3.6): Site-specific; no program should rely on a specific home directory location.

**`/tmp`** (§3.15): No persistence between invocations assumed.

---

### Ch 4 — /usr Hierarchy

`/usr` is shareable, read-only. Large packages must not use a direct subdirectory under `/usr`.
`/usr/local` is for locally-installed software safe from system updates.

**`/usr/share`** (§4.11): Architecture-independent data. `/usr/share/man` for man pages.
Locale-aware man page directory structure: `<mandir>/<locale>/man<section>`.

---

### Ch 5 — /var Hierarchy

Variable data. Key sections:

**`/var/opt`** (§5.10): Variable data for `/opt` packages. Must use `/var/opt/<subdir>`
where subdir matches the package name in `/opt`. No imposed internal structure.

**`/var/log`** (§5.8): Log files. `lastlog`, `messages`, `wtmp` are specified.

**`/var/run`** (§5.11): PID files (`<program-name>.pid`) and UNIX domain sockets.
Cleared at boot. **Note: RHEL 10 uses `/run` (FHS 3.0 convention); `/var/run` is a symlink.**

**`/var/lib`** (§5.5): Variable state information. State that must persist between
invocations and reboots (not logs, not spools).

**`/var/cache`** (§5.2): Application cache data — can be deleted without data loss.

---

### Ch 6 — Linux OS Specific Annex

Adds Linux-specific requirements: `/proc` for kernel/process info, `/dev/null`, `/dev/zero`,
`/dev/tty` required devices, `/lib64` for AMD64 64-bit libraries.
