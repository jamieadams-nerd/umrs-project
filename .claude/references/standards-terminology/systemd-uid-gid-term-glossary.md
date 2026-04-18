# systemd UIDS-GIDS — Term Glossary

**Source:** systemd UIDS-GIDS (2025), https://systemd.io/UIDS-GIDS/ | **Familiarized:** 2026-04-18

RHEL 10 `/etc/login.defs` values are included as implementation evidence where relevant.

---

## system user (systemd)

**Definition:** A user in the UID range 1–999 that does not map to an actual human.
Used as security identities for system daemons to implement privilege separation with minimal privileges.
**Source:** systemd UIDS-GIDS §"Special Distribution UID Ranges"
**NSS requirement:** Must be resolvable without network, before `systemd-udevd.service`.
**RHEL 10 implementation:** `SYS_UID_MIN=201`, `SYS_UID_MAX=999` — dynamic allocation range.
The 0–200 range is reserved by RHEL for static distro accounts.
**Synonyms:** "service user" (informal, acceptable)
**Contrast with:** regular user (1000+)

---

## regular user (systemd)

**Definition:** A user in the UID range 1000–60000 (and 65536–4294967294) who maps to an
actual human. May be stored in remote databases (LDAP, NIS). Must be resolvable by
`nss-user-lookup.target`.
**Source:** systemd UIDS-GIDS §"Special Distribution UID Ranges"
**RHEL 10 implementation:** `UID_MIN=1000`, `UID_MAX=60000`

---

## nobody (systemd)

**Definition:** UID 65534. The "overflow" UID. Subsystems map unmappable users here —
e.g., filesystems supporting only 16-bit UIDs, NFS, user namespacing.
`nss-systemd` synthesizes a user record for UID 65534 even if not in `/etc/passwd`.
**Source:** systemd UIDS-GIDS §"Special Linux UIDs"
**Note:** Some distributions call the `nobody` group `nogroup`. The spec notes this
but does not endorse it. Use `nobody` for both user and group name for consistency.

---

## overflow UID

**Synonym for:** nobody (UID 65534)
**Source:** systemd UIDS-GIDS §"Special Linux UIDs"

---

## nss-systemd

**Definition:** A glibc NSS (Name Service Switch) module provided by systemd. Synthesizes
user database records for UIDs 0 (root) and 65534 (nobody) even without `/etc/passwd`
entries. Also synthesizes records for `DynamicUser=` allocations (61184–65519) and
`systemd-homed` home directories (60001–60513).
**Source:** systemd UIDS-GIDS (throughout)
**Implication for UMRS:** The `umrs` account (in the 201–999 range) is NOT synthesized by
`nss-systemd` — it must be in local `/etc/passwd`.

---

## DynamicUser=

**Definition:** A systemd unit directive that allocates a transient UID from 61184–65519
for the lifetime of the service. The UID does not persist in `/etc/passwd`. `nss-systemd`
synthesizes the record while the service runs.
**Source:** systemd UIDS-GIDS §"Special systemd UID Ranges" (item 3)
**Hex range:** 0xEF00–0xFFEF
**UMRS ruling:** Do not use for the `umrs` service unit. See SDR-SYSTEMD-001.

---

## systemd-homed

**Definition:** A systemd service for managing user home directories. UIDs 60001–60513
are allocated to `systemd-homed`-managed accounts (hash of username for stability).
**Source:** systemd UIDS-GIDS §"Special systemd UID Ranges" (item 1)
**UMRS relevance:** None directly, but the 60001–60513 range is off-limits for other purposes.

---

## systemd-nspawn

**Definition:** A lightweight container runtime. Uses `--private-users=pick` to allocate
a 64K UID block from 524288–1879048191. Container base UIDs have lower 16 bits = 0.
**Source:** systemd UIDS-GIDS §"Special systemd UID Ranges" (item 4)

---

## tty group GID = 5

**Definition:** The `tty` group must have GID 5 on all systemd systems. This is the only
hard GID numeric assignment in the systemd spec. Required because it is encoded in
`devpts` mount parameters during earliest boot before NSS is available.
**Source:** systemd UIDS-GIDS §"Special systemd GIDs"
**Canonical spelling:** `tty` (lowercase)

---

## nss-user-lookup.target

**Definition:** A systemd passive target unit used as a synchronization point between
user database providers (LDAP client, etc.) and consumers (login services). Regular users
must be resolvable by the time this target is reached. Services providing the user database
are ordered `Before=` this target; services consuming it are ordered `After=`.
**Source:** systemd UIDS-GIDS §"Notes on resolvability of user and group names"

---

## lckpwdf() / ulckpwdf()

**Definition:** POSIX functions to acquire/release a lock on the password database.
systemd and container managers use this during UID allocation to prevent race conditions
when picking UIDs and writing them to NSS databases.
**Source:** systemd UIDS-GIDS §"Special systemd UID Ranges" (DynamicUser allocation note)
**Implication for UMRS:** If UMRS ever writes to `/etc/passwd` or `/etc/group` programmatically,
it must use `lckpwdf()` + `ulckpwdf()` or delegate to `useradd`/`usermod`.

---

## HIC SVNT LEONES

**Definition:** Latin: "Here be dragons." Used in the systemd spec to label the UID range
2147483648–4294967294 (0x80000000–0xFFFFFFFE). This range should be avoided because
kernel subsystems including `devpts` and `setfsuid()` treat UIDs as signed 32-bit integers
and cannot correctly handle values ≥ 2^31.
**Source:** systemd UIDS-GIDS summary table
**UMRS ruling:** UIDs ≥ 2147483648 must be rejected at input boundaries. See SDR-SYSTEMD-003.

---

## sub-UID / sub-GID (RHEL 10 implementation)

**Definition (RHEL 10 `login.defs`):**
- `SUB_UID_MIN=524288`, `SUB_UID_MAX=600100000`, `SUB_UID_COUNT=65536`
- `SUB_GID_MIN=524288`, `SUB_GID_MAX=600100000`, `SUB_GID_COUNT=65536`

These are the ranges used for user namespace UID/GID subordinate mappings (rootless containers).
**Note:** The `SUB_UID_MIN=524288` value overlaps with systemd's container UID range
(524288–1879048191). This is a potential collision point when UMRS hosts both rootless
containers and systemd-nspawn containers. Monitor for conflicts.
**Source:** RHEL 10 `/etc/login.defs` (implementation evidence, not in systemd spec itself)
