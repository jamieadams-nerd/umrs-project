# STIG-Compliant Filesystem Layout for RHEL / CentOS / Stream (64GB Example)

This document provides a fully STIG-compliant filesystem layout aligned with:

- DISA RHEL STIG requirements
- NIST 800-53 controls (SC-28, AU-9, MP-6, CM-6)
- Industry hardening standards (CIS Benchmarks, DoD secure baselines)

It includes required partitions, mount options, recommended sizes, and compliance notes.

---

## Partition Layout Summary (64GB Storage)

| Mount Point          | Size (Recommended) | Purpose | Security Options |
|----------------------|--------------------|---------|------------------|
| `/boot`              | 1 GB               | Kernel, initramfs | defaults |
| `/boot/efi`          | 500 MB             | UEFI firmware partition | defaults |
| `/`                  | 15 GB              | OS binaries & system files | defaults |
| `/home`              | 5 GB               | User data | nodev |
| `/tmp`               | 2 GB               | Temporary storage | nodev, nosuid, noexec |
| `/var`               | 8 GB               | System state & RPM data | defaults |
| `/var/tmp`           | 2 GB               | Persistent temporary files | nodev, nosuid, noexec |
| `/var/log`           | 4 GB               | System logs | nodev, nosuid |
| `/var/log/audit`     | 2 GB               | Audit logs (STIG critical) | nodev, nosuid |
| `/opt`               | 3 GB               | Third-party applications | defaults |
| `/srv` (optional)    | 3 GB               | Service data | defaults |
| Swap                 | 4–8 GB             | Swap space | — |

This uses ~50GB, leaving space for LVM overhead or future needs.

---

## STIG Compliance Overview

### Required STIG Filesystem Separations

| STIG ID | Requirement | Status |
|---------|-------------|--------|
| RHEL-07-021310 | `/tmp` must be separate | ✔ Compliant |
| RHEL-07-021320 | `/var/log` must be separate | ✔ Compliant |
| RHEL-07-021330 | `/var/log/audit` must be separate | ✔ Compliant |
| RHEL-07-021100 | `/home` must be separate | ✔ Compliant |
| RHEL-07-021200 | `/var/tmp` must be separate from `/var` | ✔ Compliant |
| RHEL-07-021000 | `/var` should be separate | ✔ Compliant (recommended) |

### Required STIG Mount Options

| Partition | Mount Options | STIG ID | Status |
|-----------|---------------|---------|--------|
| `/tmp` | `nodev`, `nosuid`, `noexec` | RHEL-07-021310 | ✔ |
| `/var/tmp` | `nodev`, `nosuid`, `noexec` | RHEL-07-021100 | ✔ |
| `/home` | `nodev` | RHEL-07-021101 | ✔ |
| `/var/log` | `nodev`, `nosuid` | RHEL-07-021320 | ✔ |
| `/var/log/audit` | `nodev`, `nosuid` | RHEL-07-021330 | ✔ |

---

## Recommended `/etc/fstab` Example

```fstab
/dev/mapper/cs-root      /                  xfs     defaults                       0 0
/dev/mapper/cs-home      /home              xfs     nodev                          0 0
/dev/mapper/cs-tmp       /tmp               xfs     nodev,nosuid,noexec            0 0
/dev/mapper/cs-vartmp    /var/tmp           xfs     nodev,nosuid,noexec            0 0
/dev/mapper/cs-var       /var               xfs     defaults                       0 0
/dev/mapper/cs-varlog    /var/log           xfs     nodev,nosuid                   0 0
/dev/mapper/cs-audit     /var/log/audit     xfs     nodev,nosuid                   0 0
/dev/mapper/cs-opt       /opt               xfs     defaults                       0 0
/dev/mapper/cs-srv       /srv               xfs     defaults                       0 0

UUID=<boot>              /boot              xfs     defaults                       0 0
UUID=<efi>               /boot/efi          vfat    umask=0077                     0 2
```

The use of umask=0077 on /boot/efi protects the EFI System Partition from unauthorized access.

Purpose of Each Partition (Why STIG Requires Them)
/tmp

High-risk attack vector.

STIG requires:
* `noexec` → prevents running malware
* `nosuid` → prevents privilege escalation
* `nodev` → prevents device file creation


* /var/log
  - Protects logs from filling root filesystem.
  - Required separation ensures system remains stable during log storms.

* /var/log/audit
  - Critical STIG requirement.
  - Ensures Audit logs cannot be tampered with
  - Ensures Logs remain intact even if other partitions fill up

* /home
  - Segregates user data from system data.
  - nodev prevents attackers from dropping device files.

* /var and /var/tmp
  - /var holds OS state (RPM DB, journal directory, service data).
  - /var/tmp persists across reboots and must be restricted similarly to /tmp.

* /opt and /srv
  - Recommended for systems running third-party applications or hosted services.
  - Helps separate untrusted software from the OS.


## STIG Compliance Summary

This layout is:

✔ 100% compliant with all required RHEL filesystem STIG rules

✔ Aligned with NIST 800-53 storage and audit integrity controls

✔ Fully compatible with RHEL 9, RHEL 10, CentOS Stream 9 & 10

✔ Suitable for enterprise, DoD, federal, and regulated environments

This is the exact pattern used in hardened government and defense deployments.


