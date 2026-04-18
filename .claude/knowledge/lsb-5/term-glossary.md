# LSB 5.0 — Term Glossary

**Source:** LSB 5.0 (2015), primarily LSB-Core-generic.txt §23 | **Familiarized:** 2026-04-18

---

## system User ID

**Definition:** A UID from 0 to 499 (LSB definition). 0–99 statically allocated by the system;
100–499 dynamically allocated by system administrators and post-install scripts using `useradd`.
**Source:** LSB §23.3
**Note:** systemd extends "system user" to include 500–999. RHEL 10 implements 201–999 as the dynamic system range.
**Canonical spelling:** "system User ID" (LSB capitalization)

---

## user database

**Definition:** The collection of user identity records (e.g., `/etc/passwd`, LDAP) that
programs must access only through the POSIX API (`getpwnam()`, `getpwuid()`, etc.).
**Source:** LSB §23.1

---

## group database

**Definition:** The collection of group identity records (e.g., `/etc/group`) accessed
only through POSIX API (`getgrnam()`, `getgrgid()`, `getgrouplist()`).
**Source:** LSB §23.1

---

## required user (LSB)

**Definition:** A user that MUST exist on every LSB-conformant system. Only three:
`root`, `bin` (legacy), `daemon` (legacy).
**Source:** LSB §23.2 Table 23-1

---

## optional user (LSB)

**Definition:** A user that MAY exist; if it exists, it must be in the specified group.
These names are for distributions, not applications. Applications must not depend on them.
**Source:** LSB §23.2 Table 23-2
**List:** adm, lp, sync, shutdown, halt, mail, news, uucp, operator, man, nobody

---

## root (LSB)

**Definition:** The required administrative superuser. Both User ID and Group ID SHALL
equal 0 — the only specific numeric UID/GID assignment mandated by LSB §23.
**Source:** LSB §23.2 Table 23-1 and §23.3

---

## bin (LSB, legacy)

**Definition:** A required but legacy user and group. Included for compatibility only.
"New applications should no longer use the bin User ID/Group ID."
**Source:** LSB §23.2 Table 23-1 note a
**Usage note:** Do not use. Create dedicated service accounts instead.

---

## daemon (LSB, legacy)

**Definition:** A required but legacy user and group. Originally used as a generic
unprivileged UID for daemons. "Generally daemons should now run under individual
User ID/Group IDs in order to further partition daemons from one another."
**Source:** LSB §23.2 Table 23-1 note b
**Usage note:** Do not use. UMRS uses the dedicated `umrs` account — this is the correct pattern.

---

## nobody (LSB optional)

**Definition:** Optional user listed in Table 23-2. LSB notes it is "Used by NFS."
Corresponds to UID 65534 in the systemd spec (overflow/unmappable UID).
**Source:** LSB §23.2 Table 23-2 (name only); systemd UIDS-GIDS (numeric assignment)
**Note:** UMRS must not depend on `nobody` existing per the policy statement in §23.2.

---

## useradd (LSB command)

**Definition:** The LSB-specified command for creating user accounts. The `-r` flag
allocates from the system UID range. Changes to user databases should go through
this command, not by direct file editing.
**Source:** LSB Core §19 (useradd command specification), referenced in §23.3
**RHEL 10 behavior:** `useradd -r` allocates from SYS_UID_MIN–SYS_UID_MAX = 201–999

---

## umask / default permissions

**Applicable note from §23.2:** "Applications cannot assume any policy for the default
file creation mask (umask) or the default directory permissions a user may have.
Applications should enforce user only file permissions on private files."
**Source:** LSB §23.2
**Implication for UMRS:** UMRS code that creates sensitive files must explicitly set
restrictive permissions rather than relying on umask.
