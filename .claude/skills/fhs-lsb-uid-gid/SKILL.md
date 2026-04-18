---
name: fhs-lsb-uid-gid
description: >
  FHS 3.0 filesystem hierarchy, LSB 5.0 UID/GID allocation, systemd UID/GID conventions,
  and RHEL 10 login.defs ranges for UMRS system account and path decisions. Use this skill
  when working with FHS, LSB, UID, GID, useradd, login.defs, /opt/umrs, system account,
  filesystem hierarchy, dynamic system account, /etc/keys, /etc/opt/umrs, or /var/opt/umrs.
  Trigger when making decisions about filesystem paths, system user creation, or UID/GID
  allocation for UMRS deployment.
---

## FHS / LSB / UID-GID Quick Reference

Version: updated 2026-04-18 -- FHS 3.0 section numbers verified against corpus; SUB_UID_MIN
corrected to 524288; /etc/opt citation corrected from section 3.8 to section 3.7.4;
/etc/keys citation corrected from section 4.5 to section 3.13.2 exception clause.

This skill provides the key data from four authoritative sources plus the UMRS compliance
report, so agents can make filesystem and account decisions without reading full PDFs.

---

### Authoritative Sources

1. **FHS 3.0** -- `.claude/references/linux-fhs-3.0/` (March 19, 2015, Linux Foundation)
2. **systemd file-hierarchy(7)** -- `.claude/references/systemd-file-hierarchy/` (RHEL 10, systemd 257-23.el10)
3. **LSB 5.0** -- `.claude/references/lsb-5/` (Chapter 23, Section 23.3)
4. **systemd UIDS-GIDS** -- `.claude/references/systemd-uid-gid/`
5. **RHEL 10 `/etc/login.defs`** -- system defaults for account creation
6. **Full compliance report** -- `.claude/references/reports/2026-04-17-uid-gid-compliance-reference.md`

---

### UID/GID Range Tables

#### LSB 5.0 (Ch 23 Section 23.3)

| Range | Purpose |
|---|---|
| 0-99 | Statically allocated by the distribution |
| 100-499 | Dynamic allocation for system accounts by the distribution |
| 500+ | Available for user accounts (legacy; modern systems use 1000+) |

#### systemd UID/GID Allocation

| Range | Purpose |
|---|---|
| 0 | root |
| 1-99 | Statically allocated by the distribution |
| 100-999 | Dynamic system users and groups |
| 1000-60000 | Regular (human) user accounts |
| 60001-60513 | systemd DynamicUser (container UIDs) |
| 60514-61183 | Unassigned |
| 61184-65519 | systemd DynamicUser (additional range) |
| 65520-65533 | Unassigned |
| 65534 | nobody (unmapped, overflow) |
| 65535 | Reserved (do not use; 16-bit sentinel) |

#### RHEL 10 `/etc/login.defs`

| Parameter | Value |
|---|---|
| `UID_MIN` | 1000 |
| `UID_MAX` | 60000 |
| `SYS_UID_MIN` | 201 |
| `SYS_UID_MAX` | 999 |
| `GID_MIN` | 1000 |
| `GID_MAX` | 60000 |
| `SYS_GID_MIN` | 201 |
| `SYS_GID_MAX` | 999 |
| `SUB_UID_MIN` | 524288 |
| `SUB_UID_MAX` | 600100000 |

**Key implication:** RHEL 10 reserves 0-200 for static distribution accounts and allocates
system accounts dynamically in 201-999. UMRS system accounts created with `useradd -r`
will land in this range.

---

### UMRS FHS 3.0 Path Layout

| Path | Purpose | FHS 3.0 Section |
|---|---|---|
| `/opt/umrs/` | UMRS binaries and static data | FHS 3.0 section 3.13 (/opt) |
| `/opt/umrs/bin/` | Executable tools | FHS 3.0 section 3.13 |
| `/opt/umrs/share/umrs/` | Architecture-independent data | FHS 3.0 section 3.13 |
| `/etc/opt/umrs/` | Configuration files | FHS 3.0 section 3.7.4 (/etc/opt) |
| `/var/opt/umrs/` | Variable data (logs, state, vaults) | FHS 3.0 section 5.12 (/var/opt) |
| `/run/umrs/` | Runtime PID files and UNIX domain sockets | FHS 3.0 section 3.15 (/run) |
| `/etc/keys/umrs/` | IMA/EVM key material | FHS 3.0 section 3.13.2 exception + NIST CM-6/SA-8 |

**Why `/etc/keys/umrs/` is outside `/opt`:** FHS 3.0 section 3.13.2 states: "No other package
files may exist outside the /opt, /var/opt, and /etc/opt hierarchies **except for those
package files that must reside in specific locations within the filesystem tree in order to
function properly**." IMA/EVM tooling uses well-known paths under `/etc/keys/` and `/etc/ima/`.
Key material therefore qualifies for this exception by function. Additionally, NIST SP 800-53
CM-6 (Configuration Settings) and SA-8 (Security Engineering Principles) require isolating
trust-critical material from operational configuration. The FHS exception clause provides
permission; the NIST controls provide the security engineering rationale.

**Note:** The /opt/umrs/ installation uses the `<package>` form (not `<provider>`). The
`<package>` form does NOT require LANANA provider registration. Documentation should state
this explicitly to preempt auditor questions (FHS 3.0 section 3.13.1).

---

### Decision Guidance

- **Creating a system account:** Use `useradd -r -s /sbin/nologin -d /var/opt/umrs umrs`.
  The `-r` flag causes allocation in the SYS_UID range (201-999 on RHEL 10).
- **Choosing a static UID:** Avoid. Let the system allocate dynamically unless a specific
  UID is required for cross-system consistency (e.g., NFS, container images). If a static
  UID is needed, document the choice and avoid the 0-200 range (distribution-reserved).
- **Path decisions:** Always use the FHS layout above. Do not create UMRS paths outside
  this structure without explicit justification.
- **Runtime directory:** Use `RuntimeDirectory=umrs` in the systemd service unit. systemd
  will create `/run/umrs/` with correct ownership before service start and remove it on stop.
- **Variable data layout:** Subdivide `/var/opt/umrs/` following systemd conventions:
  state/, cache/, log/, vaults/ (see fhs-lsb-systemd-CHAIN.md for rationale).
- **Citing both /run sections:** When justifying `/run/umrs/`, cite FHS 3.0 section 3.15
  (establishes /run) and section 5.13 (explains /var/run is now a compatibility symlink).
