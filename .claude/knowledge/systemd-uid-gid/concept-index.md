# systemd UIDS-GIDS — Concept Index

**Source:** `uids-gids.html` | **Version:** systemd 2025 | **Familiarized:** 2026-04-18

---

## Document: uids-gids.html

**Short ID:** SYSTEMD-UIDS

**What it covers:** Complete UID/GID allocation model for systemd-based Linux distributions.
Covers special Linux UIDs, distribution UID ranges, systemd-specific ranges, container
allocation, NSS resolution requirements, and container manager recommendations.

**Governs:**
- System user UID range boundaries on RHEL 10 (1–999)
- NSS resolvability requirements for early-boot system accounts
- Container UID range allocation (`systemd-nspawn --private-users=pick`)
- `DynamicUser=` service user allocation (61184–65519)
- `tty` group GID = 5 (hard requirement for `devpts` at early boot)

---

## Section: Special Linux UIDs

Four UIDs with kernel-level special meaning:

| UID | Meaning |
|---|---|
| 0 | `root` superuser |
| 65534 | `nobody` / overflow — where subsystems map unmappable users (16-bit filesystems, NFS, user namespacing) |
| 65535 | 16-bit `(uid_t) -1` — invalid, pre-Linux-2.4 legacy sentinel |
| 4294967295 | 32-bit `(uid_t) -1` — invalid, `setresuid()`/`chown()` treat -1 as "do not change" |

**Note on `nobody` naming:** Some distributions call the `nobody` group `nogroup`. The
systemd spec notes this but expresses a preference against it. For consistency, UMRS
documentation should use `nobody` for both user and group name.

---

## Section: Special Distribution UID Ranges

The fundamental system/regular split:

| Range | Class | Notes |
|---|---|---|
| 1–999 | System users | Non-human; daemon privilege separation; MUST resolve without network |
| 1000–65533 and 65536–4294967294 | Regular (human) users | May use LDAP/NIS; resolved at late boot |

The 999/1000 boundary is systemd's compile-time default. `systemd` strongly discourages
changing this boundary. The boundary can be queried at runtime via:
```
pkg-config --variable=system_uid_max systemd
```
Returns: `999` on an unmodified RHEL 10 system.

---

## Section: Special systemd GIDs

systemd defines one hard GID assignment:
- **GID 5 = `tty` group** — must be constant because it is encoded in `devpts` mount
  parameters during earliest boot, before NSS lookups are possible.

All other systemd-defined group GIDs do not need to be constant across systems.
The current list is in `sysusers.d/basic.conf`.

---

## Section: Special systemd UID Ranges

The full table (from the canonical summary in the HTML document):

| UID/GID Range | Hex | Count | Purpose | Defined By | Listed In |
|---|---|---|---|---|---|
| 0 | 0x00000000 | 1 | `root` | Linux | `/etc/passwd` + `nss-systemd` |
| 1–4 | 0x00000001–0x00000004 | 4 | System users | Distributions | `/etc/passwd` |
| 5 | 0x00000005 | 1 | `tty` group | systemd | `/etc/passwd` |
| 6–999 | 0x00000006–0x000003E7 | 994 | System users | Distributions | `/etc/passwd` |
| 1000–60000 | 0x000003E8–0x0000EA60 | 59001 | Regular users | Distributions | `/etc/passwd` + LDAP/NIS |
| 60001–60513 | 0x0000EA61–0x0000EC61 | 513 | Human users (homed) | systemd | `nss-systemd` |
| 60514–60577 | 0x0000EC62–0x0000ECA1 | 64 | Host users mapped into containers | systemd | `systemd-nspawn` |
| 60578–60705 | 0x0000ECA2–0x0000ED21 | 128 | Dynamic greeter users | systemd | `nss-systemd` |
| 60706–61183 | 0x0000ED22–0x0000EEFF | 478 | Unused | — | — |
| 61184–65519 | 0x0000EF00–0x0000FFEF | 4336 | Dynamic service users (`DynamicUser=`) | systemd | `nss-systemd` |
| 65520–65533 | 0x0000FFF0–0x0000FFFD | 14 | Unused | — | — |
| 65534 | 0x0000FFFE | 1 | `nobody` | Linux | `/etc/passwd` + `nss-systemd` |
| 65535 | 0x0000FFFF | 1 | 16-bit `(uid_t) -1` (invalid) | Linux | — |
| 65536–524287 | 0x00010000–0x0007FFFF | 458752 | Unused | — | — |
| 524288–1879048191 | 0x00080000–0x6FFFFFFF | ~1.88B | Container UID ranges (`systemd-nspawn`) | systemd | `nss-systemd` |
| 1879048192–2147352575 | 0x70000000–0x7FFDFFFF | ~268M | Unused | — | — |
| 2147352576–2147418111 | 0x7FFE0000–0x7FFEFFFF | 65536 | Foreign OS image UIDs | systemd | `nss-systemd` |
| 2147418112–2147483647 | 0x7FFF0000–0x7FFFFFFF | 65536 | Unused | — | — |
| 2147483648–4294967294 | 0x80000000–0xFFFFFFFE | ~2.15B | HIC SVNT LEONES (avoid) | — | — |
| 4294967295 | 0xFFFFFFFF | 1 | 32-bit `(uid_t) -1` (invalid) | Linux | — |

**"Unused" caveat:** "Unused" means no well-established pre-defined purpose between Linux,
distributions, and systemd. Other packages may allocate from these ranges.

---

## Section: Figuring Out UID Boundaries

Runtime query method:
```sh
pkg-config --variable=system_uid_max systemd     # → 999
pkg-config --variable=greeter_uid_min systemd    # → 60578
pkg-config --variable=greeter_uid_max systemd    # → 60705
pkg-config --variable=dynamic_uid_min systemd    # → 61184
pkg-config --variable=dynamic_uid_max systemd    # → 65519
pkg-config --variable=container_uid_base_min systemd  # → 524288
pkg-config --variable=container_uid_base_max systemd  # → 1878982656
pkg-config --variable=foreign_uid_base systemd   # → 2147352576
```

RHEL 10 implementation values from `/etc/login.defs`:
- `SYS_UID_MIN=201`, `SYS_UID_MAX=999`
- `UID_MIN=1000`, `UID_MAX=60000`
- Sub-UID range: 524288–600100000 (for user namespace mapping)

---

## Section: NSS Resolvability Requirements

**System users (1–999):** MUST be resolvable without network, at earliest boot, before:
- `systemd-udevd.service`
- `systemd-tmpfiles-setup.service`

This means: system accounts MUST live in local `/etc/passwd` (or a local cache available
in the initrd). LDAP/NIS providers are prohibited for system users unless they maintain
a local cache that is available before networking is up.

**Regular users (1000+):** Must be resolvable by the time `nss-user-lookup.target` is
reached. May use networked databases.

---

## Section: Container Manager Guidance

Key rules for container UID allocation:
1. Assign at least 65536 UIDs per container (must include UID 65534 = nobody)
2. Prefer exactly 65536 UIDs per container
3. Align container base UID so lower 16 bits are zero (enables efficient `chown` operations)
4. Check NSS (`getpwuid()` + `lckpwdf()`) before picking a range to avoid collisions
5. Provide an NSS module that makes allocated UIDs visible in the user database

Container UID math:
```
INTERNAL_UID = EXTERNAL_UID & 0x0000FFFF
CONTAINER_BASE_UID = EXTERNAL_UID & 0xFFFF0000
EXTERNAL_UID = INTERNAL_UID | CONTAINER_BASE_UID
```
