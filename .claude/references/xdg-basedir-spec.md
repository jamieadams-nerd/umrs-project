# XDG Base Directory Specification

Source: https://specifications.freedesktop.org/basedir-spec/latest/
Canonical version at time of authorship (training data): 0.8
Status: requires_verification_download
Date written: 2026-03-17
Author: Claude (from training knowledge — see verification section below)

---

## Verification

This document was written from training knowledge after an automated fetch of the
freedesktop.org specification was blocked. Jamie should verify it against the live
specification before treating it as authoritative.

**Fetch the current specification:**

```
curl -L -o /tmp/xdg-basedir-spec.html \
  https://specifications.freedesktop.org/basedir-spec/latest/
```

```
curl -L -o /tmp/xdg-basedir-spec-0.8.html \
  https://specifications.freedesktop.org/basedir-spec/basedir-spec-0.8.html
```

Compare against this document and update any discrepancies. Mark status as `downloaded`
and add a SHA-256 checksum once verified.

---

## Overview

The XDG Base Directory Specification defines a set of environment variables that locate
well-known directories for user-specific data, configuration, state, cache, and runtime
files on Unix-like systems. It was developed under the freedesktop.org umbrella and is
widely adopted on Linux desktops, servers, and embedded systems.

The specification separates concerns that historically were conflated under $HOME:

- **Data** — application assets installed by or on behalf of the user
- **Configuration** — user-editable settings files
- **State** — persistent state that is not configuration and not data
- **Cache** — regenerable data; loss is acceptable
- **Runtime** — transient sockets, pipes, and lock files; lifetime tied to the login session

This separation matters for UMRS because different directory types have different security,
backup, and relabeling requirements under SELinux and MLS policy.

---

## Specification Version

The version current in training data is **0.8**, published by freedesktop.org.
Version 0.8 added XDG_STATE_HOME, which was absent in 0.7. Check the source URL for any
subsequent revisions.

---

## Environment Variables

### XDG_DATA_HOME

**Purpose:** Base directory for user-specific data files.

**Default:** $HOME/.local/share

**Requirements:**
- If XDG_DATA_HOME is set, it must be an absolute path. If it is not set or is empty,
  the default $HOME/.local/share must be used.
- Applications should store data under a subdirectory named after the application:
  $XDG_DATA_HOME/<application>/
- The directory should be created with permissions 0700 unless the application has a
  specific reason to share data.

**SELinux / UMRS note:** Files under XDG_DATA_HOME are in the user's home context. If
UMRS tools install data here, the SELinux type must be appropriate for that data's
sensitivity label. MLS-labeled data must not be placed here without a corresponding
security context assignment.

---

### XDG_CONFIG_HOME

**Purpose:** Base directory for user-specific configuration files.

**Default:** $HOME/.config

**Requirements:**
- If XDG_CONFIG_HOME is set, it must be an absolute path. If it is not set or is empty,
  the default $HOME/.config must be used.
- Applications store configuration under $XDG_CONFIG_HOME/<application>/
- Configuration files here are user-editable. Applications must handle missing config
  gracefully (fall through to compiled-in defaults).

**SELinux / UMRS note:** Configuration files under $HOME/.config are typically labeled
user_home_t or a derivative. UMRS tool configuration that affects enforcement decisions
should be read through the trust-gated path (verify kernel subsystem active before
trusting config — CM-6, Trust Gate Rule).

---

### XDG_STATE_HOME

**Purpose:** Base directory for user-specific state data that should persist between
application restarts but is not important enough to be backed up with configuration.

**Default:** $HOME/.local/state

**Requirements:**
- Added in specification version 0.8.
- If XDG_STATE_HOME is set, it must be an absolute path. If not set or empty, the
  default $HOME/.local/state must be used.
- Appropriate contents: history files, recently used files, current state of the
  application window (geometry, open documents), undo history.
- Not appropriate: configuration (use XDG_CONFIG_HOME), cached regenerable data
  (use XDG_CACHE_HOME), temporary runtime files (use XDG_RUNTIME_DIR).

**SELinux / UMRS note:** State files can contain sensitive operational history. Audit
trail data, recent query history, or posture scan results that accumulate across sessions
are candidates for $XDG_STATE_HOME/<app>/. Consider whether this data warrants a
distinct SELinux type rather than the default user_home_t.

---

### XDG_CACHE_HOME

**Purpose:** Base directory for user-specific non-essential (cached) data.

**Default:** $HOME/.cache

**Requirements:**
- If XDG_CACHE_HOME is set, it must be an absolute path. If not set or empty, the
  default $HOME/.cache must be used.
- Cache data may be deleted at any time without loss of functionality. Applications
  must tolerate a missing or empty cache directory.
- Applications store cache under $XDG_CACHE_HOME/<application>/

**SELinux / UMRS note:** Because cache data may be regenerated, it is lower risk for
MLS purposes. However, a cache that stores the result of a privileged query (e.g., cached
SELinux policy decisions, cached kernel attribute readings) must not be treated as
authoritative. UMRS tools should re-verify from the kernel source rather than trusting
cached values for any security enforcement decision.

---

### XDG_RUNTIME_DIR

**Purpose:** Base directory for user-specific runtime files and other file objects such
as sockets, named pipes, and lock files.

**Default:** None specified by the standard. The runtime system (PAM, systemd) is expected
to set this variable and create the directory.

**Requirements — these are the most security-critical rules in the specification:**

1. **Ownership:** The directory must be owned by the user and only the user.
2. **Mode:** The directory must have mode 0700 (owner read/write/execute only).
3. **Lifetime:** The directory must be bound to the login session. It must be created
   when the user logs in and removed when the last session for the user ends.
4. **Filesystem:** The directory should be on a local filesystem. It must not be stored
   on a network filesystem.
5. **tmpfs preferred:** The directory is ideally on a memory-backed filesystem (tmpfs)
   so that runtime files do not survive reboots. This prevents stale sockets and lock
   files from persisting across reboots.
6. **No guarantee of persistence:** Applications must not rely on files in
   XDG_RUNTIME_DIR surviving beyond the session. If persistence is needed, use
   XDG_STATE_HOME instead.
7. **Application subdirectories:** Applications should use a subdirectory
   $XDG_RUNTIME_DIR/<application>/ rather than placing files directly in the root.

**systemd implementation (pam_systemd):**
On systemd-based systems, pam_systemd creates XDG_RUNTIME_DIR automatically at login,
sets it to /run/user/UID (where UID is the numeric user ID), and tears it down at final
logout. The 0700 mode requirement is enforced by pam_systemd at creation time.

**What happens if XDG_RUNTIME_DIR is unset:**
If the variable is unset or the directory does not exist, applications should fall back
to a temporary directory approach. However, they must not silently create a world-readable
fallback — failing is safer than creating an insecure substitute.

**SELinux / UMRS note:** /run/user/UID is labeled user_tmp_t or a session-specific type
under targeted policy. Unix domain sockets placed here are subject to SELinux type
enforcement in addition to the DAC 0700 check. For UMRS tools that communicate via
sockets, the socket type must be appropriate for the sensitivity of data exchanged.

---

### XDG_DATA_DIRS

**Purpose:** Ordered set of system-wide directories to search for data files, in addition
to XDG_DATA_HOME.

**Default:** /usr/local/share:/usr/share

**Requirements:**
- Colon-separated list of absolute paths.
- If not set or empty, the default above applies.
- XDG_DATA_HOME is always searched first, before any directory in XDG_DATA_DIRS.
- Directories are searched in order; the first match wins.
- The standard does not define a merging strategy — the first file found is used.

**Typical values on RHEL 10 / Fedora:**
/usr/local/share:/usr/share (default), sometimes extended to include /usr/share/gnome
or flatpak data paths by the desktop session.

---

### XDG_CONFIG_DIRS

**Purpose:** Ordered set of system-wide directories to search for configuration files,
in addition to XDG_CONFIG_HOME.

**Default:** /etc/xdg

**Requirements:**
- Colon-separated list of absolute paths.
- If not set or empty, the default /etc/xdg applies.
- XDG_CONFIG_HOME is always searched first, before any directory in XDG_CONFIG_DIRS.
- Directories are searched in order; the first match wins.

**UMRS note:** System-wide configuration under /etc/xdg falls under the Trust Gate Rule.
UMRS tools must verify the relevant kernel subsystem is active before trusting any
configuration read from this path.

---

## Summary Table

| Variable          | Default                    | Purpose                        | Persists reboots | Backup-worthy |
|-------------------|----------------------------|--------------------------------|-----------------|---------------|
| XDG_DATA_HOME     | ~/.local/share             | User data                      | Yes             | Yes           |
| XDG_CONFIG_HOME   | ~/.config                  | User configuration             | Yes             | Yes           |
| XDG_STATE_HOME    | ~/.local/state             | Persistent non-config state    | Yes             | Optional      |
| XDG_CACHE_HOME    | ~/.cache                   | Regenerable cache              | Yes             | No            |
| XDG_RUNTIME_DIR   | (set by PAM/systemd)       | Session-scoped runtime files   | No              | No            |
| XDG_DATA_DIRS     | /usr/local/share:/usr/share | System-wide data search path  | Yes             | N/A           |
| XDG_CONFIG_DIRS   | /etc/xdg                   | System-wide config search path | Yes             | N/A           |

---

## The ~/.local/bin Question

### Is ~/.local/bin part of the XDG Base Directory Specification?

**No.** ~/.local/bin is not defined by the XDG Base Directory Specification version 0.8.
The specification defines only the data, config, state, cache, and runtime directory
variables listed above. It does not define a user-local binary directory.

~/.local/bin is a **convention** — a widely adopted one, but a convention. Its status
as a de facto standard comes from several sources, described below.

### What the FHS says

The Filesystem Hierarchy Standard 3.0 defines /usr/local/bin as the place for locally
installed binaries for all users. It does not address per-user binary directories. The
FHS predates the widespread adoption of ~/.local/ as a user-local prefix.

### The ~/.local/ prefix as a user-local analog of /usr/local/

The directory tree ~/.local/ functions as a user-local analog of the /usr/local/ hierarchy:

```
/usr/local/share   ->   ~/.local/share   (XDG_DATA_HOME)
/usr/local/state   ->   ~/.local/state   (XDG_STATE_HOME, added in XDG 0.8)
/usr/local/bin     ->   ~/.local/bin     (convention, not XDG spec)
/usr/local/lib     ->   ~/.local/lib     (convention)
```

This parallel structure is documented informally in freedesktop.org discussions and is
referenced in various XDG-related specifications (like the XDG menu specification), but
~/.local/bin itself has never been formally added to the Base Directory Specification.

### What RHEL 10 / Fedora say

**Fedora** includes ~/.local/bin in the default $PATH for interactive bash sessions
via /etc/profile.d/ scripts (specifically the path construction in the system profile).
This has been the case since approximately Fedora 36. The intent is to allow users to
install Python scripts via pip install --user, pipx, and similar user-space package
managers without root.

**RHEL 10 (based on Fedora upstream)** follows the same convention. The system bash
profile adds $HOME/.local/bin to $PATH if the directory exists.

**The relevant profile logic (typical):**

```bash
# /etc/profile or /etc/profile.d/sh.local
if [ -d "$HOME/.local/bin" ] && [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    PATH="$HOME/.local/bin:$PATH"
fi
```

This is an opt-in by existence model: the directory is only added to $PATH if it
already exists, so it does not pollute $PATH for accounts that do not use it.

### What systemd says

systemd-related tooling (particularly systemd-path and documentation for user systemd
services) refers to ~/.local/bin as the canonical user executable prefix. The
systemd.exec(5) manpage and user service documentation assume that user-installed
binaries live in ~/.local/bin. The pam_systemd session setup does not create
~/.local/bin automatically — that is left to the user or user-space package managers.

### What pip / pipx / uv say

Python user installs (pip install --user) place scripts in ~/.local/bin by default on
Linux. This is by far the most common way the directory gets created on developer and
operator workstations.

### Security implications for UMRS deployment

For UMRS tools:

1. **Do not rely on ~/.local/bin being in $PATH in non-interactive contexts.** Cron
   jobs, systemd services, and PAM-invoked executables may not have the profile-sourced
   $PATH. Use absolute paths.

2. **If UMRS provides user-local binaries, ~/.local/bin is the correct prefix.** It
   aligns with the ~/.local/ hierarchy convention and is what Fedora/RHEL expect.

3. **SELinux context:** Binaries in ~/.local/bin under targeted policy are typically
   labeled user_home_t. They cannot be executed in privileged contexts without a policy
   module that allows it. For UMRS operator tools that need elevated SELinux privileges,
   installation to /usr/local/bin (system-wide) with appropriate bin_t labeling is
   safer than relying on the user-home context.

4. **MLS environments:** In MLS policy, executing files from user_home_t in a
   sensitivity-elevated context will almost certainly be blocked. UMRS operator tools
   that run at elevated sensitivity levels must live in a system directory with the
   correct MLS label — not under ~/.local/.

5. **User-space UMRS tooling vs. system UMRS tooling:** User-space utilities that do
   not require elevated privilege (e.g., display-only tools, report viewers) are
   appropriate for ~/.local/bin. Enforcement-path tools belong in /usr/local/bin or
   /usr/bin with proper system labeling.

### Summary: ~/.local/bin

| Aspect                          | Answer                                                    |
|---------------------------------|-----------------------------------------------------------|
| In XDG Base Directory Spec?     | No                                                        |
| De facto convention?            | Yes — widely adopted                                      |
| RHEL 10 $PATH inclusion?        | Yes, if the directory exists (profile.d logic)            |
| systemd user services assume it?| Yes, informally                                           |
| Created automatically?          | No — created on first user-space package install          |
| SELinux context                 | user_home_t (targeted) — not suitable for privileged executables |
| MLS suitability                 | Not suitable for elevated-sensitivity tool execution      |

---

## Application Compliance Requirements

Per the specification, applications must:

1. Treat all XDG variables as optional; fall through to defaults if unset or empty.
2. Only honor the variable if it is an absolute path; ignore relative paths.
3. Create subdirectories under the base directory rather than placing files directly
   in the base directory (except for well-known dotfiles like $XDG_CONFIG_HOME/mimeapps.list).
4. Never fail hard if the directory does not exist — create it on demand with appropriate
   permissions (0700 for user directories is recommended).
5. Never write to system-wide directories (XDG_DATA_DIRS, XDG_CONFIG_DIRS) — these
   are read-only search paths from the application's perspective.

---

## Interaction with SELinux

The XDG directories sit in the user's home hierarchy. Under SELinux targeted policy:

| Directory              | Typical SELinux type   | Notes                                      |
|------------------------|------------------------|--------------------------------------------|
| ~/.local/share/        | user_home_t            | Standard home context                      |
| ~/.config/             | user_home_t            | Standard home context                      |
| ~/.local/state/        | user_home_t            | Standard home context                      |
| ~/.cache/              | user_home_t            | Standard home context                      |
| /run/user/UID/         | user_tmp_t             | Created by pam_systemd; tmpfs              |
| /usr/share/            | usr_t / share_t        | System data dirs                           |
| /etc/xdg/              | etc_t                  | System config dir                          |
| ~/.local/bin/          | user_home_t            | Not executable in privileged contexts      |

In MLS environments, sensitivity labels on files under $HOME are constrained by the
user's clearance. Any tool that reads from XDG directories and uses the result in an
enforcement decision must treat that input as user-controlled and validate accordingly.

---

## References

- XDG Base Directory Specification 0.8:
  https://specifications.freedesktop.org/basedir-spec/basedir-spec-0.8.html
- XDG Base Directory Specification (latest):
  https://specifications.freedesktop.org/basedir-spec/latest/
- Fedora default PATH configuration:
  https://src.fedoraproject.org/rpms/setup (the profile file in the setup package)
- systemd pam_systemd(8):
  https://www.freedesktop.org/software/systemd/man/pam_systemd.html
- Filesystem Hierarchy Standard 3.0:
  https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html

---

## Companion Documents in This Corpus

- linux-fhs-2.3/fhs-2.3.txt — Filesystem Hierarchy Standard 2.3 (full text)
- This document covers user-space directory conventions; FHS covers system-wide layout.

---

Written from training knowledge (Claude, 2026-03-17). Requires verification against
the live specification at https://specifications.freedesktop.org/basedir-spec/latest/
before treating as authoritative. See verification section above.
