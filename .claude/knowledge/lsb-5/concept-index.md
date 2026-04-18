# LSB 5.0 — Concept Index

**Source:** `LSB-Core-generic.txt` (primary), `LSB-Common.txt` | **Version:** LSB 5.0 (2015) | **Familiarized:** 2026-04-18

---

## Document: LSB-Common.txt

**Short ID:** LSB-COMMON

**What it covers:** Common definitions, licensing framework, and normative references
shared across all LSB module documents. Establishes scope and conformance vocabulary.

**Key terms:**
- LSB conformance requirements
- Normative vs. informative references
- GNU Free Documentation License terms

**Governs:** Understanding what "SHALL", "SHOULD", and "MAY" mean across all LSB documents.

---

## Document: LSB-Core-generic.txt

**Short ID:** LSB-CORE

**What it covers:** The primary LSB specification. Covers the runtime environment,
required commands and utilities, filesystem layout (by reference to FHS), and the
critical §23 Users & Groups chapter. This is the document that provides formal standard
authority for UMRS UID/GID allocation decisions.

**Key terms:** See §23 breakdown below.

**Governs:**
- UMRS system account compliance justification
- Required system usernames that UMRS must not conflict with
- UID range within which `useradd -r` should allocate

---

## §23 Users & Groups — Detailed Breakdown

This is the binding content for UMRS system account compliance.

### §23.1 — Preface / User Database Format

The format of user and group databases (e.g., `/etc/passwd`, `/etc/group`) is not
specified by LSB. Programs must use the provided POSIX API (`getpwnam()`, `getpwuid()`,
`getgrnam()`, `getgrgid()`, etc.) to read these databases. Changes must go through
provided commands (`useradd`, `groupadd`, etc.).

**Implication for UMRS:** UMRS must never parse `/etc/passwd` directly. Use POSIX API.

### §23.2 — User & Group Names

**Table 23-1: Required User & Group Names**

| User | Group | Status | Notes |
|---|---|---|---|
| root | root | Required | UID and GID SHALL equal 0 (the only numeric UID/GID assigned by name in §23) |
| bin | bin | Required (legacy) | Included for compatibility only; new applications SHALL NOT use this |
| daemon | daemon | Required (legacy) | Daemons should now run under individual UIDs, not daemon |

**Table 23-2: Optional User & Group Names** (for distributions, not applications)

`adm`, `lp`, `sync`, `shutdown`, `halt`, `mail`, `news`, `uucp`, `operator`, `man`, `nobody`

**Critical policy statement (§23.2):**
> "Applications cannot assume non system user or group names will be defined."

This means UMRS cannot depend on `adm`, `nobody`, or any other optional account existing.

**`operator` group note:** The operator account's group is `root` in Table 23-2, not `operator`.

### §23.3 — User ID Ranges

Full text (verbatim from corpus):
> "The system User IDs from 0 to 99 should be statically allocated by the system, and
> shall not be created by applications."
>
> "The system User IDs from 100 to 499 should be reserved for dynamic allocation by
> system administrators and post install scripts using useradd."

**Key observations:**
- The word "should" is used for the 100–499 dynamic range — a recommendation, not a hard requirement.
- "shall not" (lowercase) for 0–99 creation by applications is closer to a requirement.
- LSB 5.0 defines nothing above 499. Everything above 499 is defined by systemd and distro convention.
- LSB 5.0 does not define 500–999 as a system range. That gap is filled by systemd (extends to 999).

### §23.4 — Rationale

> "The purpose of specifying optional users and groups is to reduce the potential for
> name conflicts between applications and distributions."

---

## Document: LSB-Core-AMD64.txt

**Short ID:** LSB-CORE-AMD64

**What it covers:** Architecture-specific extensions and ABI requirements for the AMD64
(x86_64) platform. Library naming conventions, data model (LP64), calling conventions.
No UID/GID content beyond what Core-generic defines.

**UMRS relevance:** Confirms that AMD64 uses LP64 data model — `uid_t` is 32-bit unsigned,
confirming the 32-bit UID range described in the systemd spec.

---

## Documents: LSB-Desktop-generic.txt, LSB-Desktop-AMD64.txt

**Short ID:** LSB-DESKTOP, LSB-DESKTOP-AMD64

**What it covers:** Desktop integration APIs — GTK, Qt, CUPS, OpenGL, fonts, graphics stack.
321K + 51K lines of GUI framework specifications.

**UMRS relevance:** None for server infrastructure. Not fully read. If UMRS ever gains
a desktop component, this collection becomes relevant.

---

## Document: LSB-Languages.txt

**Short ID:** LSB-LANG

**What it covers:** Language runtime standards — Python, Perl, interpreter locations.
Relates to `/usr/bin/python`, `/usr/bin/perl` paths.

**UMRS relevance:** Low. Confirms standard interpreter paths if UMRS scripts use shebangs.

---

## Document: LSB-TrialUse.txt

**Short ID:** LSB-TRIAL

**What it covers:** Experimental and trial-use specifications not yet normative.
Includes early D-Bus, device access APIs.

**UMRS relevance:** Low. Monitor for any trial items that become normative in future LSB releases.

---

## Document: LSB-Imaging.txt

**Short ID:** LSB-IMAGING

**What it covers:** Imaging API standards (SANE, printer interfaces).

**UMRS relevance:** None for the current UMRS scope.
