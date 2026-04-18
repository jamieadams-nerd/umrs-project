# FHS 3.0 — Term Glossary

**Source:** FHS 3.0 (March 19, 2015, Linux Foundation) | **Familiarized:** 2026-04-18

---

## Core Filesystem Classification Terms

### Shareable files
- **Definition:** Files that can be stored on one host and used on others.
- **Source:** FHS 3.0 §2
- **Synonyms:** none
- **Examples:** `/usr` (binaries, libraries), `/opt` (add-on software), `/var/mail`
- **Usage note:** Contrast with *unshareable files*. This distinction motivates the `/usr` (shared) vs `/etc` (host-specific) separation.

### Unshareable files
- **Definition:** Files that are not shareable; host-specific content.
- **Source:** FHS 3.0 §2
- **Examples:** `/etc` (host configuration), `/boot` (host kernel)

### Static files
- **Definition:** Files that do not change without system administrator intervention. Includes binaries, libraries, documentation.
- **Source:** FHS 3.0 §2
- **Usage note:** Static files can be stored on read-only media. `/usr` and `/opt` are static hierarchies.

### Variable files
- **Definition:** Files that are not static; changed during normal system operation.
- **Source:** FHS 3.0 §2
- **Examples:** `/var` contents (logs, state, spool), `/run` contents (PID files, sockets)
- **Usage note:** Variable files need separate backup schedules from static files.

---

## Directory Terms

### /run
- **Definition:** A top-level tmpfs directory for run-time variable data: PID files, socket files, and similar transient data. Flushed at each boot. Always writable.
- **Source:** FHS 3.0 §3.15 (NEW in 3.0 — not present in FHS 2.3)
- **Deprecated variant:** `/var/run` (see below)
- **Usage note:** RHEL 10 uses `/run` as primary; `/var/run` is a symlink to `/run`. For UMRS, use `/run/umrs/` for PID files and UNIX sockets.

### /var/run
- **Definition:** Historical location for run-time variable data. Compatibility symlink to `/run` in FHS 3.0-era systems.
- **Source:** FHS 3.0 §5.13
- **Deprecated:** Yes — for new work. FHS 3.0 explicitly states functions moved to `/run`.
- **Usage note:** Do not use `/var/run` in new UMRS code or documentation. Acceptable in legacy system references.

### /opt
- **Definition:** Reserved for the installation of add-on application software packages. Static files only.
- **Source:** FHS 3.0 §3.13
- **Requirements:** Static files in `/opt/<package>` or `/opt/<provider>`. Variable data → `/var/opt`. Config → `/etc/opt`.

### /opt/\<package\>
- **Definition:** Directory tree for a named add-on software package. The `<package>` form does NOT require LANANA registration.
- **Source:** FHS 3.0 §3.13.1
- **Contrast with:** `/opt/<provider>` (LANANA-registered provider name)
- **UMRS application:** `/opt/umrs/` uses the `<package>` form. No LANANA registration required.

### /opt/\<provider\>
- **Definition:** Directory tree for a LANANA-registered provider offering multiple packages.
- **Source:** FHS 3.0 §3.13.1
- **Usage note:** UMRS does NOT use this form.

### /etc/opt/\<subdir\>
- **Definition:** Host-specific configuration files for add-on packages installed in `/opt`. The `<subdir>` must match the subtree name in `/opt`.
- **Source:** FHS 3.0 §3.7.4
- **Requirements:** No internal structure imposed. If a config file must reside elsewhere to function, exception is permitted.
- **UMRS application:** `/etc/opt/umrs/` — configuration for the `umrs` package.

### /var/opt/\<subdir\>
- **Definition:** Variable data for packages in `/opt`. The `<subdir>` matches the package name in `/opt`.
- **Source:** FHS 3.0 §5.12
- **Requirements:** No internal structure imposed.
- **UMRS application:** `/var/opt/umrs/` — logs, state, vault data for the `umrs` package.

### /usr/libexec
- **Definition:** Binaries intended to be run by other programs, not directly from a shell. Optional directory, new in FHS 3.0.
- **Source:** FHS 3.0 §4.7
- **Deprecated variants:** None (did not exist in FHS 2.3)
- **Usage note:** Use for daemon helper binaries that should not appear in `$PATH`. systemd's `/usr/lib/<package>/` serves the same purpose and is the modern convention on RHEL 10.

### /usr/include
- **Definition:** Standard C include files for the system.
- **Source:** FHS 3.0 §4.5
- **Usage note:** The skill cites "FHS 4.5 vs NIST CM-6/SA-8" for the `/etc/keys/umrs/` key material exception. FHS §4.5 covers `/usr/include` (C headers), which is UNRELATED to key material placement. The skill's citation of §4.5 for this rationale is a category error — §4.5 does not justify key separation. The actual FHS basis is the general `/opt` exception clause in §3.13.2: "No other package files may exist outside the /opt, /var/opt, and /etc/opt hierarchies except for those package files that must reside in specific locations within the filesystem tree in order to function properly." Key material under `/etc/keys/` qualifies under this exception by function, not by §4.5.

### LANANA
- **Definition:** Linux Assigned Names and Numbers Authority. Registers provider names for use in `/opt/<provider>` directory trees.
- **Source:** FHS 3.0 §3.13.1
- **Usage note:** Only the `<provider>` form requires LANANA registration. The `<package>` form (UMRS's form) does not.

---

## Changes from FHS 2.3 Terminology

| Term | FHS 2.3 | FHS 3.0 | Change |
|---|---|---|---|
| `/run` | Not defined; use `/var/run` | Top-level required directory (§3.2, §3.15) | Newly standardized |
| `/var/run` | Primary runtime directory | Compatibility symlink to `/run` (§5.13) | Demoted |
| `/usr/libexec` | Not defined | Optional (§4.7) | Added |
| Section numbers | Not present (chapter/prose only) | Full decimal numbering | All citations must now use decimal IDs |

---

## Do Not Confuse

- `/etc/opt` (FHS §3.7.4) vs `/etc/keys` (NIST CM-6/SA-8 exception, not FHS-governed)
- `/opt/<package>` (no registration) vs `/opt/<provider>` (LANANA registration required)
- `/run` (FHS 3.0 primary) vs `/var/run` (compatibility symlink — same path on RHEL 10, but cite `/run`)
- `FHS §4.5 /usr/include` vs the `/opt` exception clause in §3.13.2 (do not cite §4.5 for key material decisions)
