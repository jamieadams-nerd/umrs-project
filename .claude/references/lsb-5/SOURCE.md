# LSB 5.0 Specification Series

**Source:** Linux Foundation — Linux Standard Base 5.0
**URL:** https://refspecs.linuxfoundation.org/LSB_5.0.0/
**Retrieved:** 2026-04-17
**Method:** curl with Firefox user-agent
**Integrity:** All files verified as valid PDF (%PDF magic bytes)

## Documents

| File | Description |
|---|---|
| LSB-Core-generic.pdf | Core specification (architecture-independent) — includes UID/GID allocation blocks |
| LSB-Core-AMD64.pdf | Core specification (AMD64-specific) |
| LSB-Common.pdf | Common definitions shared across all LSB modules |
| LSB-Desktop-generic.pdf | Desktop specification (architecture-independent) |
| LSB-Desktop-AMD64.pdf | Desktop specification (AMD64-specific) |
| LSB-Languages.pdf | Runtime language specifications |
| LSB-Imaging.pdf | Printing/scanning specifications |
| LSB-TrialUse.pdf | Trial use specifications |

## Key Section for UMRS

**LSB-Core-generic.pdf, Part IX, Chapter 23 "Users & Groups"** — defines UID/GID allocation:
- §23.2: Required (root, bin, daemon) and optional (adm, lp, nobody, etc.) user/group names
- §23.3 User ID Ranges:
  - 0–99: statically allocated by the system; SHALL NOT be created by applications
  - 100–499: reserved for dynamic allocation by sysadmins and post-install scripts using `useradd`
- UMRS `umrs` account: created with `useradd -r` → falls in the 100–499 dynamic system range (correct per LSB 5.0)

## Not Downloaded

Architecture-specific Core documents for IA32, IA64, PPC32, PPC64, S390, S390X were not
downloaded — AMD64 is the project's target architecture. Available at the same base URL
if needed.
