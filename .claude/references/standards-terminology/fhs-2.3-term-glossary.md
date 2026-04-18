# FHS 2.3 — Term Glossary

**Source:** FHS 2.3 (2004) | **Familiarized:** 2026-04-18

Canonical terms from FHS 2.3. Source wins for spelling and definition.

---

## shareable

**Definition:** Files that can be stored on one host and used on others (e.g., via NFS mount).
**Source:** FHS 2.3 Ch 2
**Synonyms:** none (do not use "shared" as a replacement — "shared" is informal)
**Usage:** `/usr`, `/opt` are shareable hierarchies.

---

## unshareable

**Definition:** Files that are not shareable; host-specific (e.g., device lock files, `/etc` configuration).
**Source:** FHS 2.3 Ch 2
**Usage:** `/etc`, `/boot`, `/var/run` are unshareable.

---

## static

**Definition:** Files that do not change without system administrator intervention. Includes binaries, libraries, documentation.
**Source:** FHS 2.3 Ch 2
**Usage:** `/usr` is static. Static files may reside on read-only media.

---

## variable

**Definition:** Files that are not static; change during normal system operation.
**Source:** FHS 2.3 Ch 2
**Usage:** `/var` is the hierarchy for variable files.

---

## add-on application software package (opt package)

**Definition:** Software not part of the base OS distribution, installed into `/opt/<package>` or `/opt/<provider>`.
**Source:** FHS 2.3 §/opt (Ch 3)
**Usage:** UMRS is an add-on application; its static content lives in `/opt/umrs/`.

---

## `/opt/<package>`

**Definition:** A directory under `/opt` named after the software package. Does not require LANANA registration.
**Source:** FHS 2.3 §/opt
**Contrast with:** `/opt/<provider>` (requires LANANA registration of provider name)

---

## `/opt/<provider>`

**Definition:** A directory under `/opt` named after a LANANA-registered provider organization.
**Source:** FHS 2.3 §/opt
**Note:** LANANA = Linux Assigned Names and Numbers Authority (https://www.lanana.org/)

---

## `/etc/opt/<subdir>`

**Definition:** Location for host-specific configuration of add-on packages in `/opt`. The `<subdir>` must match the package name in `/opt`.
**Source:** FHS 2.3 §/etc/opt
**Normative requirement:** "must be installed within" — this is mandatory, not recommended.

---

## `/var/opt/<subdir>`

**Definition:** Location for variable data of add-on packages in `/opt`. The `<subdir>` must match the package name in `/opt`.
**Source:** FHS 2.3 §/var/opt (Ch 5)
**Normative requirement:** "must be installed in" — mandatory.

---

## `/var/run`

**Definition:** Run-time variable data. Files cleared at boot. PID files and UNIX domain sockets stored here.
**Source:** FHS 2.3 §/var/run (Ch 5)
**Note:** Superseded in practice by `/run` (FHS 3.0, systemd convention). On RHEL 10, `/var/run` is a symlink to `/run`.

---

## `/var/lib`

**Definition:** Variable state information that persists across reboots but is not logs or spools. Applications must use a subdirectory.
**Source:** FHS 2.3 §/var/lib (Ch 5)

---

## `/var/cache`

**Definition:** Application cache data. Can be deleted without data loss. Application must recover from deletion.
**Source:** FHS 2.3 §/var/cache (Ch 5)
**Contrast with:** `/var/lib` (state that must not be deleted arbitrarily)

---

## PID file

**Definition:** A file in `/var/run` containing the ASCII decimal process identifier followed by newline. Named `<program-name>.pid`.
**Source:** FHS 2.3 §/var/run
**Format:** 10-byte ASCII decimal, trailing newline. Readers should tolerate leading zeros, extra whitespace, absence of newline.

---

## LANANA

**Definition:** Linux Assigned Names and Numbers Authority. Coordinates namespace assignments for Linux to prevent collisions. Relevant to `/opt/<provider>` directory registration.
**Source:** FHS 2.3 §/opt (rationale)
**URL:** https://www.lanana.org/
**Note:** LANANA does not define UID/GID ranges. UID/GID ranges are defined by LSB §23 and systemd.
