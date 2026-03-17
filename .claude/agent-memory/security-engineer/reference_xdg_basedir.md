---
name: XDG Base Directory Specification — security-relevant takeaways
description: Key security facts about XDG directory layout, XDG_RUNTIME_DIR requirements, ~/.local/bin non-spec status, and SELinux context implications for UMRS deployment planning
type: reference
---

Source document: `.claude/references/xdg-basedir-spec.md` (from training knowledge, spec v0.8;
requires verification against https://specifications.freedesktop.org/basedir-spec/latest/)

---

## XDG_RUNTIME_DIR — critical security requirements

The spec's strongest security mandates apply here:

- **Mode must be 0700** — owner only; no group or world access
- **Owned by the user and only the user**
- **Lifetime is the login session** — created at login, removed at final logout
- **Must be on a local filesystem** — network filesystems are prohibited
- **tmpfs strongly preferred** — prevents stale sockets/pipes from surviving reboots
- **Applications must not assume persistence** — use XDG_STATE_HOME for anything that must survive logout

On RHEL 10 with systemd: pam_systemd creates `/run/user/UID` and enforces 0700 at creation.

SELinux type: `user_tmp_t` (or session-specific type, targeted policy). Unix domain sockets
placed here are still subject to type enforcement. For UMRS IPC, the socket type must match
the sensitivity of data exchanged.

If XDG_RUNTIME_DIR is unset: fail, do not silently create a world-readable fallback.

---

## Application compliance rules (all XDG variables)

1. All XDG variables are optional — fall through to defaults if unset or empty.
2. Only honor the variable if it is an **absolute path**; reject relative paths silently.
3. Create subdirectories under the base dir rather than placing files directly in the root.
4. Never fail hard if the directory is absent — create on demand with 0700.
5. Never write to system-wide dirs (XDG_DATA_DIRS, XDG_CONFIG_DIRS) — these are read-only search paths.

**For umrs-tool-init / env validation**: every XDG variable read must be validated as an
absolute path before use. A relative path or empty string from the environment must be
rejected and replaced with the spec default. This is the UMRS XDG trust gate.

---

## ~/.local/bin — NOT in the XDG spec

`~/.local/bin` is a **convention**, not part of XDG Base Directory Specification v0.8.
It is the user-local analog of `/usr/local/bin` in the `~/.local/` prefix hierarchy.

**RHEL 10 / Fedora behavior:** profile.d adds `$HOME/.local/bin` to `$PATH` if the
directory already exists — opt-in by existence, not created automatically.

**Security implications for UMRS deployment:**

1. Do not rely on `~/.local/bin` being in `$PATH` in non-interactive contexts
   (cron, systemd services, PAM-invoked executables). Use absolute paths there.

2. SELinux type under targeted policy: **`user_home_t`**. This means:
   - Cannot be executed in privileged contexts without a specific policy allow rule
   - Not suitable for enforcement-path tools

3. MLS policy: executing from `user_home_t` in a sensitivity-elevated context will
   be blocked. Enforcement-path tools must live in a system directory with correct
   MLS labeling.

4. **Initial UMRS release decision (Jamie confirmed):** Install to `$HOME/.local/bin`.
   This constrains the initial release to display-only / non-privileged operator tools.
   Enforcement-path tools require system installation (`/usr/local/bin` or `/usr/bin`)
   with proper `bin_t` labeling and a policy module.

---

## Directory → SELinux type mapping (targeted policy, RHEL 10)

| Directory            | Typical type   | Notes                                               |
|----------------------|----------------|-----------------------------------------------------|
| ~/.local/share/      | user_home_t    | XDG_DATA_HOME default                               |
| ~/.config/           | user_home_t    | XDG_CONFIG_HOME default                             |
| ~/.local/state/      | user_home_t    | XDG_STATE_HOME default (v0.8+)                      |
| ~/.cache/            | user_home_t    | XDG_CACHE_HOME default                              |
| /run/user/UID/       | user_tmp_t     | XDG_RUNTIME_DIR (pam_systemd); tmpfs                |
| ~/.local/bin/        | user_home_t    | Convention only; NOT executable in privileged ctx   |
| /usr/share/          | usr_t/share_t  | XDG_DATA_DIRS system component                      |
| /etc/xdg/            | etc_t          | XDG_CONFIG_DIRS system component                    |

In MLS environments, any input read from XDG directories (all user_home_t) must be
treated as user-controlled. Validate before use in any enforcement decision (Trust Gate Rule).

---

## Config files under XDG_CONFIG_HOME and XDG_CONFIG_DIRS

These are user-controlled inputs. UMRS tools must apply the Trust Gate Rule:
verify the relevant kernel subsystem is active before trusting configuration read
from `$HOME/.config/` or `/etc/xdg/`. Controls: NIST SP 800-53 CM-6.

---

## Cached security data (XDG_CACHE_HOME)

Cache under `~/.cache/` must never be treated as authoritative for security enforcement
decisions. Any value derived from a privileged kernel query that is cached here must be
re-verified from the kernel source before enforcement use. This applies to cached SELinux
policy decisions, cached kernel attribute readings, etc.

---

## State data (XDG_STATE_HOME)

Posture scan history, audit trail fragments, or recent query results that accumulate across
sessions are candidates for `$XDG_STATE_HOME/<app>/`. Consider whether accumulated
operational history warrants a distinct SELinux type rather than defaulting to `user_home_t`.

---

## Deployment planning implications

| Tool class                                     | Install location      | SELinux type     |
|------------------------------------------------|-----------------------|------------------|
| Display-only, non-privileged operator tools    | ~/.local/bin          | user_home_t (ok) |
| Enforcement-path or elevated-privilege tools   | /usr/local/bin        | bin_t + policy   |
| MLS-labeled or sensitivity-elevated tools      | /usr/bin (packaged)   | bin_t + MLS ctx  |

Initial UMRS release targets `~/.local/bin` — constrains scope to display/reporting tools.
Policy module work for enforcement-path tools must include restorecon and appropriate
`.fc` entries for `/usr/local/bin/umrs-*`.
