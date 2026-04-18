# systemd file-hierarchy(7) — Concept Index

**Source:** `file-hierarchy.txt` | **Version:** systemd 257-23.el10 (RHEL 10) | **Familiarized:** 2026-04-18
**Captured from:** Local RHEL 10 man page — deployment-target ground truth

---

## Document: file-hierarchy(7) — File System Hierarchy Overview

**Short ID:** FH-SYSTEMD

**What it covers:** The systemd project's definition of a "generalized, minimal, modernized
subset" of FHS and hier(7) that governs file system layout on systemd-managed systems. This is
the RHEL 10 implementation document — not a standard, but the de facto specification for what
actually happens on RHEL 10. It explicitly cites FHS 3.0 as its upstream standard and extends
it with systemd-specific conventions: credentials directories, state vs. cache separation,
XDG user directory integration, and per-package runtime/state/cache/log table slots.

**Key terms and concepts:**
- `/run/` — tmpfs for system runtime data, sockets, PID files (always writable, flushed on boot)
- `/run/user/<uid>/` — per-user runtime directory, mounted as tmpfs, accessed via `$XDG_RUNTIME_DIR`
- `/var/lib/` — persistent private system data (state)
- `/var/cache/` — persistent but deletable system cache data
- `/var/log/` — persistent system logs
- `/usr/lib/<package>/` — private static vendor resources (including internal binaries)
- `/usr/lib/arch-id/` — public shared libraries (`$libdir`)
- `/usr/share/factory/etc/` — pristine vendor copies of `/etc/` files (for comparison/recovery)
- `/usr/share/factory/var/` — pristine vendor copies of `/var/` files
- State vs. cache distinction — state cannot be deleted without data loss; cache can
- `StateDirectory=`, `CacheDirectory=`, `RuntimeDirectory=`, `LogsDirectory=` — systemd unit directives for automated directory management
- `tmpfiles.d(5)` — mechanism for creating runtime directories at boot
- Node type discipline: device nodes in `/dev/` only; sockets/FIFOs in `/run/` only
- `nosuid`, `nodev` on `/tmp/` and `/var/tmp/`

**What it governs:**
- RHEL 10 runtime path behavior (what exists, when, and with what properties)
- Whether to use `StateDirectory=` vs `RuntimeDirectory=` in UMRS systemd units
- Location of UNIX domain sockets for UMRS daemons (`/run/umrs/`)
- The per-package slot tables that define where system-package files live
- State vs. cache separation in `/var/opt/umrs/` subdirectory design
- `/dev/shm/` warnings relevant to any UMRS shared-memory usage

**Related documents:**
- FHS 3.0 — upstream standard that file-hierarchy(7) extends and refines
- systemd UIDS-GIDS — same project; covers UID/GID allocation conventions
- `fhs-lsb-uid-gid` skill — synthesizes the full compliance chain
- `selinux-rules` skill — SELinux labels for the paths defined here

---

## Section-Level Summaries

### GENERAL STRUCTURE

Covers root, `/boot/`, `/efi/`, `/etc/`, `/home/`, `/root/`, `/srv/`, `/tmp/`.

Key UMRS-relevant notes:
- `/etc/` "may or may not be read-only" — applications must fall back to defaults if config is missing
- `/tmp/` — always use `mkstemp(3)` / `mkdtemp(3)`; small files only; use `/var/tmp/` for larger files

---

### RUNTIME DATA

**`/run/`**: tmpfs, flushed at boot, writable from earliest boot. Primary location for:
- System-level sockets and PID files
- Any data that must be writable before `/var/` becomes available

**`/run/log/`**: Runtime system logs (available before `/var/log/` is accessible)

**`/run/user/<uid>/`**: Per-user runtime directories, tmpfs. Access via `$XDG_RUNTIME_DIR`, not direct path.

**UMRS implication:** UMRS daemon sockets → `/run/umrs/`. Use `RuntimeDirectory=umrs` in the
systemd unit to have systemd create `/run/umrs/` automatically at service start.

---

### VENDOR-SUPPLIED OPERATING SYSTEM RESOURCES

**`/usr/`**: Vendor OS resources. Usually read-only. Not to be modified by sysadmin except
during package operations.

**`/usr/bin/`**: User-callable binaries in `$PATH`. Daemon binaries should NOT be here —
use `/usr/lib/<package>/` instead.

**`/usr/lib/`**: Static, private vendor data including internal executables.

**`/usr/lib/arch-id/`** (`$libdir`): Public shared libraries. Architecture-specific.

**`/usr/share/factory/etc/`** and **`/usr/share/factory/var/`**: Pristine vendor configuration
and variable data. Useful for auditing drift from vendor defaults.

**UMRS implication:** UMRS is an `/opt` package, not a `/usr/lib` package. These paths do not
apply to UMRS binary placement. They document the systemd convention for contrast.

---

### PERSISTENT VARIABLE SYSTEM DATA

**`/var/`**: Persistent, variable. Writable during normal operation. May not be available
during early boot — applications must not assume it is mounted before starting.

**`/var/cache/`**: Non-essential, deletable. Flushing must not break the application — it
may just be slower to rebuild. Use `CacheDirectory=` in systemd units.

**`/var/lib/`**: Persistent private data. Primary slot for persistent state that does not
fit other categories. Use `StateDirectory=` in systemd units.

**`/var/log/`**: Persistent logs. Prefer `sd_journal_print(3)` or `syslog(3)` over writing
files here directly. Use `LogsDirectory=` in systemd units.

**`/var/tmp/`**: Larger temporary files, not flushed on boot. Same `mkstemp` security rules
as `/tmp/`.

**UMRS implication:** Within `/var/opt/umrs/`, adopt the systemd convention:
- `/var/opt/umrs/state/` — persistent state (mirrors `/var/lib/<package>/` convention)
- `/var/opt/umrs/cache/` — cache data (mirrors `/var/cache/<package>/`)
- `/var/opt/umrs/log/` — logs (mirrors `/var/log/<package>/`)

---

### VIRTUAL KERNEL AND API FILE SYSTEMS

`/dev/`, `/dev/shm/`, `/proc/`, `/proc/sys/`, `/sys/`, `/sys/fs/cgroup/` — standard
virtual filesystems. `/dev/shm/` is world-writable; use `/run/` or `$XDG_RUNTIME_DIR` for
security-sensitive shared memory instead.

---

### COMPATIBILITY SYMLINKS

`/bin/` → `/usr/bin/`, `/sbin/` → `/usr/bin/`, `/usr/sbin/` → `/usr/bin/`, `/lib/` → `/usr/lib/`,
`/lib64/` → `$libdir`, **`/var/run/` → `/run/`**.

All these are compatibility symlinks on RHEL 10. Do not rely on them in new code — use the
canonical paths.

---

### HOME DIRECTORY

`~/.cache/`, `~/.config/`, `~/.local/bin/`, `~/.local/lib/`, `~/.local/state/` — XDG
convention for user-installed applications. UMRS is a system-level application; these paths
are informational only.

---

### WRITE ACCESS

**Unprivileged write access** (user processes): `/tmp/`, `/var/tmp/`, `/dev/shm/`, `$HOME`, `$XDG_RUNTIME_DIR`

**Unprivileged system processes:** Only `/tmp/`, `/var/tmp/`, `/dev/shm/`.
For private writable directories in `/var/` or `/run/`: either create before privilege drop,
use `tmpfiles.d(5)`, or use `StateDirectory=` / `RuntimeDirectory=` directives.

**UMRS implication:** The `umrs` service account (non-interactive, no home) must have its
writable directories created via `StateDirectory=` or `RuntimeDirectory=` in the systemd unit,
or pre-created by the install script with correct ownership and SELinux labels.

---

### SYSTEM PACKAGES — Table 1 (Vendor files) and Table 2 (Variable files)

These tables define the canonical slot for each type of system-package file. They apply to
packages installed under `/usr/`, NOT to `/opt` packages (which are governed by FHS §3.13).

| Slot | Purpose |
|---|---|
| `/usr/bin/` | Public executables |
| `/usr/lib/package/` | Private static vendor resources (includes private binaries) |
| `/etc/package/` | System-specific configuration |
| `/run/package/` | Runtime data (flushed at boot) |
| `/var/cache/package/` | Persistent cache |
| `/var/lib/package/` | Persistent private data (primary state slot) |
| `/var/log/package/` | Persistent logs |

**UMRS note:** These slots are the systemd-canonical equivalent of FHS §3.13's opt triad.
When explaining UMRS's `/var/opt/umrs/` layout to auditors familiar with systemd conventions,
map: `/var/opt/umrs/state/` ≈ `/var/lib/package/`; `/var/opt/umrs/cache/` ≈ `/var/cache/package/`.

---

### NODE TYPES

Device nodes: `/dev/` only. Sockets and FIFOs: `/run/` only. "Applications should expect
that a security policy might be enforced on a system that enforces these rules." This is an
explicit SELinux reference — UMRS SELinux policy should enforce this node-type discipline.
