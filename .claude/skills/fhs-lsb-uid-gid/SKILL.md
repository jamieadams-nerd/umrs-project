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

This skill provides the key data from three authoritative sources plus the UMRS compliance
report, so agents can make filesystem and account decisions without reading full PDFs.

---

### Authoritative Sources

1. **LSB 5.0** — `.claude/references/lsb-5/` (Chapter 23, Section 23.3)
2. **systemd UIDS-GIDS** — `.claude/references/systemd-uid-gid/`
3. **RHEL 10 `/etc/login.defs`** — system defaults for account creation
4. **Full compliance report** — `.claude/references/reports/2026-04-17-uid-gid-compliance-reference.md`

---

### UID/GID Range Tables

#### LSB 5.0 (Ch 23 Section 23.3)

| Range | Purpose |
|---|---|
| 0–99 | Statically allocated by the distribution |
| 100–499 | Dynamic allocation for system accounts by the distribution |
| 500+ | Available for user accounts (legacy; modern systems use 1000+) |

#### systemd UID/GID Allocation

| Range | Purpose |
|---|---|
| 0 | root |
| 1–99 | Statically allocated by the distribution |
| 100–999 | Dynamic system users and groups |
| 1000–60000 | Regular (human) user accounts |
| 60001–60513 | systemd DynamicUser (container UIDs) |
| 60514–61183 | Unassigned |
| 61184–65519 | systemd DynamicUser (additional range) |
| 65520–65533 | Unassigned |
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
| `SUB_UID_MIN` | 100000 |
| `SUB_UID_MAX` | 600100000 |

**Key implication:** RHEL 10 reserves 0–200 for static distribution accounts and allocates
system accounts dynamically in 201–999. UMRS system accounts created with `useradd -r`
will land in this range.

---

### UMRS FHS 3.0 Path Layout

| Path | Purpose | FHS Section |
|---|---|---|
| `/opt/umrs/` | UMRS binaries and static data | FHS 3.13 (/opt) |
| `/opt/umrs/bin/` | Executable tools | FHS 3.13 |
| `/opt/umrs/share/umrs/` | Architecture-independent data | FHS 3.13 |
| `/etc/opt/umrs/` | Configuration files | FHS 3.8 (/etc/opt) |
| `/var/opt/umrs/` | Variable data (logs, state, vaults) | FHS 5.12 (/var/opt) |
| `/etc/keys/umrs/` | IMA/EVM key material | Deliberately outside /opt per FHS 4.5 vs NIST CM-6/SA-8 |

**Why `/etc/keys/umrs/` is outside `/opt`:** FHS Section 4.5 says `/etc/opt/<package>/`
is for add-on package configuration. Key material is not configuration — it is
cryptographic trust anchors. Placing keys under `/etc/keys/` follows the convention
established by IMA/EVM tooling and keeps key material in a path that can be independently
hardened with restrictive SELinux labels and filesystem permissions without affecting
general UMRS configuration access. This separation satisfies NIST SP 800-53 CM-6
(Configuration Settings) and SA-8 (Security Engineering Principles) by isolating
trust-critical material from operational configuration.

---

### Decision Guidance

- **Creating a system account:** Use `useradd -r -s /sbin/nologin -d /var/opt/umrs umrs`.
  The `-r` flag causes allocation in the SYS_UID range (201–999 on RHEL 10).
- **Choosing a static UID:** Avoid. Let the system allocate dynamically unless a specific
  UID is required for cross-system consistency (e.g., NFS, container images). If a static
  UID is needed, document the choice and avoid the 0–200 range (distribution-reserved).
- **Path decisions:** Always use the FHS layout above. Do not create UMRS paths outside
  this structure without explicit justification.
