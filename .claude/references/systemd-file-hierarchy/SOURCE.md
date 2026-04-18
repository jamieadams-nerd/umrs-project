# Source: systemd file-hierarchy(7)

## Document Information

| Field | Value |
|---|---|
| Full title | file-hierarchy(7) — File system hierarchy overview |
| Issuing authority | systemd project (freedesktop.org) |
| Installed version | systemd 257 (257-23.el10-g881b59a) |
| Retrieval date | 2026-04-18 |
| Retrieval method | `man 7 file-hierarchy | col -bx` (RHEL 10 installed man page) |

## Files

### file-hierarchy.txt

| Field | Value |
|---|---|
| Source | Local system: RHEL 10, systemd 257-23.el10 |
| SHA-256 | 3131f8042829040660a44014b75a60fb0cfd8b8837874478cc206237eb140390 |
| Size | 42700 bytes (694 lines) |
| Content description | RHEL 10 installed file-hierarchy(7) man page; covers /run, /var/lib, /etc/systemd/, /usr/lib/systemd/, credentials directories, state/cache separation, and XDG-derived path conventions under systemd |

## Upstream HTML Status

Both canonical upstream URLs returned HTTP 418 (bot-blocked):
- https://www.freedesktop.org/software/systemd/man/latest/file-hierarchy.html
- https://www.freedesktop.org/software/systemd/man/file-hierarchy.html

The locally installed man page (RHEL 10, systemd 257) is the ground-truth for the UMRS deployment target and is authoritative for this project.

## systemd Version Details

```
systemd 257 (257-23.el10-g881b59a)
+PAM +AUDIT +SELINUX -APPARMOR +IMA +IPE +SMACK +SECCOMP -GCRYPT -GNUTLS +OPENSSL
+ACL +BLKID +CURL +ELFUTILS +FIDO2 +IDN2 +KMOD +LIBCRYPTSETUP +LIBCRYPTSETUP_PLUGINS
+LIBFDISK +PCRE2 +PWQUALITY +P11KIT +TPM2 +BZIP2 +LZ4 +XZ +ZLIB +ZSTD
+BPF_FRAMEWORK +BTF +XKBCOMMON +UTMP +SYSVINIT +LIBARCHIVE
```

## Relevance

Complements FHS 3.0 and LSB 5.0 with systemd-specific path semantics:
- `/run` — ephemeral runtime data (systemd-native, supersedes /var/run)
- `/var/lib` — persistent state data for services
- `/etc/systemd/` — local systemd configuration
- `/usr/lib/systemd/` — vendor-supplied systemd units
- Credentials directories — runtime secret injection
- State vs cache separation — authoritative for UMRS service layout decisions

## Manual Re-fetch Command

To re-capture the man page from a future RHEL system:

```bash
man 7 file-hierarchy | col -bx > file-hierarchy.txt
systemctl --version >> file-hierarchy.txt
```
