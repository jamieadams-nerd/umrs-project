# UID/GID Allocation Compliance Reference — UMRS `umrs` Account

**Date:** 2026-04-17
**Author:** Orchestrator + Jamie Adams
**Purpose:** Authoritative reference for justifying the UMRS `umrs` system account
allocation. Three sources establish the compliance basis; all three must be cited
together because no single source tells the complete story.

---

## The Three Authoritative Sources

### 1. Linux Standard Base 5.0 (LSB) — The Formal Spec

**Location:** `.claude/references/lsb-5/LSB-Core-generic.pdf`, Part IX, Chapter 23

**What it defines:**

| UID Range | LSB 5.0 Classification |
|---|---|
| 0 | Root superuser (UID and GID SHALL equal 0) |
| 1–99 | Statically allocated by the system; SHALL NOT be created by applications |
| 100–499 | Reserved for dynamic allocation by system administrators and post-install scripts using `useradd` |
| 500+ | Not defined by LSB |

**What it does NOT define:**
- LSB 5.0 does not mandate the 100–999 "system" vs 1000+ "human" split
- LSB 5.0 does not define container UID ranges, dynamic service users, or the `nobody` UID
- LSB 5.0 says UIDs 100+ are available for "general use" but provides no further subdivision

**Required users/groups (§23.2, Table 23-1):**

| User | Group | Status |
|---|---|---|
| root | root | Required — all privileges |
| bin | bin | Legacy — new applications should NOT use |
| daemon | daemon | Legacy — daemons should now run under individual UIDs |

**Optional users/groups (§23.2, Table 23-2):**
adm, lp, sync, shutdown, halt, mail, news, uucp, operator, man, nobody

**Key policy statement (§23.2):**
> "Applications cannot assume non-system user or group names will be defined."

### 2. systemd Users/Groups Specification — The Modern Standard

**Location:** `.claude/references/systemd-uid-gid/uids-gids.html`
**URL:** https://systemd.io/UIDS-GIDS/

This is the de facto standard for all systemd-based distributions including RHEL 10.
It refines the LSB's broad "100+" range into specific tiers:

| UID Range | systemd Classification | Resolved By |
|---|---|---|
| 0 | Root superuser | `/etc/passwd` + `nss-systemd` |
| 1–4 | System daemons (distro-assigned) | `/etc/passwd` |
| 5 | TTY group | `/etc/passwd` |
| 6–999 | System daemons (distro-assigned) | `/etc/passwd` |
| 1000–60000 | Regular (human) users | `/etc/passwd` + LDAP/NIS |
| 60001–60513 | Home directories (`systemd-homed`) | `nss-systemd` |
| 60514–60577 | Container host UID mappings | `systemd-nspawn` |
| 60578–60705 | Dynamic greeter sessions | `nss-systemd` |
| 60706–61183 | Unassigned | — |
| 61184–65519 | Dynamic service users (`DynamicUser=`) | `nss-systemd` |
| 65520–65533 | Unassigned | — |
| 65534 | Overflow / `nobody` user | `/etc/passwd` + `nss-systemd` |
| 65535 | Reserved (16-bit `-1` sentinel) | Unavailable |
| 524288–1879048191 | Container UID ranges (`--private-users=pick`) | `nss-systemd` |
| 2147352576–2147418111 | Foreign OS image mappings | `nss-systemd` |
| 4294967295 | Reserved (32-bit `-1` sentinel) | Unavailable |

**Critical notes from systemd spec:**
- System users (0–999) MUST be resolvable without network — required before `systemd-udevd.service` starts
- Regular users do NOT need resolvable identities during early boot
- UIDs above 2^31 should be avoided due to kernel limitations in `devpts` and related subsystems
- Dynamic service users (61184–65519) are chosen below the 16-bit boundary for container compatibility

### 3. RHEL 10 `login.defs` — The Implementation Evidence

**Location:** `/etc/login.defs` on RHEL 10 / CentOS Stream 10

This is what `useradd` actually reads. It implements the systemd specification:

```
SYS_UID_MIN               201
SYS_UID_MAX               999
UID_MIN                  1000
UID_MAX                 60000
SYS_GID_MIN               201
SYS_GID_MAX               999
GID_MIN                  1000
GID_MAX                 60000
SUB_UID_MIN           524288
SUB_UID_MAX        600100000
SUB_UID_COUNT          65536
SUB_GID_MIN           524288
SUB_GID_MAX        600100000
SUB_GID_COUNT          65536
```

**Note:** RHEL 10 sets `SYS_UID_MIN=201` (not 100), reserving 100–200 for distro
static assignments. The `useradd -r` command allocates from 201–999.

---

## UMRS `umrs` Account Justification

### Account Creation

```bash
useradd -r -s /sbin/nologin umrs
```

### Compliance Checklist

| Requirement | Source | UMRS Status |
|---|---|---|
| UID in dynamic system range | LSB 5.0 §23.3 (100–499), systemd (100–999), RHEL 10 (201–999) | ✓ `useradd -r` allocates from SYS_UID_MIN–SYS_UID_MAX |
| No login shell | systemd spec: system users do not log in | ✓ `-s /sbin/nologin` |
| No home directory | Best practice for file-owner accounts | ✓ `-r` flag skips home directory creation |
| Locked password | Security requirement for non-interactive accounts | ✓ `useradd -r` locks password by default (`!` in `/etc/shadow`) |
| Resolvable without network | systemd: system users must resolve before `systemd-udevd` | ✓ Entry in local `/etc/passwd`, no LDAP/NIS dependency |
| Name does not conflict with LSB required/optional names | LSB 5.0 §23.2 Tables 23-1, 23-2 | ✓ `umrs` is not in either table |
| Name does not conflict with distro conventions | LSB 5.0 §23.4 rationale | ✓ `umrs` is application-specific, not a distro service name |

### Why Three Sources Are Required

- **LSB 5.0 alone** is insufficient — it only defines up to UID 499 and says nothing about login shells, home directories, or password locking
- **systemd alone** is insufficient — it is not a formal standard body; LSB provides the ISO/POSIX-adjacent authority
- **RHEL 10 `login.defs` alone** is insufficient — it is implementation, not specification; an auditor needs the normative reference behind the implementation choice
- **Together** they form a complete chain: formal standard (LSB) → modern specification (systemd) → implementation evidence (RHEL 10 `login.defs`)

### LANANA Note

The Linux Assigned Names and Numbers Authority (https://www.lanana.org/) coordinates
namespace assignments to prevent collisions. LANANA does not publish a standalone
UID/GID policy document — its role is registration, not specification. The actual range
definitions come from LSB and systemd. LANANA's LSB registries cover provider names,
package names, and init script names, but not UID/GID ranges directly.

---

## Deployment Documentation Cross-References

The following deployment pages should cite this reference:

- `docs/modules/deployment/pages/umrs/5a-users-and-types/dac-users-groups.adoc` — primary
- `docs/modules/operations/pages/admin-install.adoc` — installation procedure
- `components/rusty-gadgets/scripts/umrs-install.sh` — the `useradd -r` invocation

## NIST / CMMC Control Mapping

| Control | Relevance |
|---|---|
| NIST SP 800-53 AC-2 | Account Management — system account with defined lifecycle |
| NIST SP 800-53 AC-6 | Least Privilege — no login, no shell, no home, locked password |
| NIST SP 800-53 CM-6 | Configuration Settings — UID range follows authoritative guidance |
| NIST SP 800-53 IA-5 | Authenticator Management — password locked, no interactive auth |
| CMMC AC.L2-3.1.1 | Limit system access to authorized users — non-interactive service account |
