---
name: Citation mapping conventions
description: Correct control citations for common security patterns in UMRS source code and docs
type: reference
---

# Control Citation Mapping Conventions

## Environment variable security

| Claim | Correct citation | Wrong citations to avoid |
|---|---|---|
| "No secrets in environment variables" | CM-7 (Least Functionality) + AC-3 (Access Enforcement) + CWE-526 | IA-5 is WRONG (IA-5 = authenticator lifecycle) |
| "Environment is observable via /proc/<pid>/environ" | AC-3 (Access Enforcement) | — |
| "Denylist strips what is not needed" | CM-7 (Least Functionality), CM-6 (Configuration Settings) | — |
| "Validated accessor pattern replaces raw env access" | SI-10 (Information Input Validation), SI-7 (Software Integrity) | — |
| "Audit trail of stripped/failed variables" | AU-3 (Content of Audit Records), AU-12 (Audit Record Generation) | — |

## i18n and locale

| Claim | Correct citation | Wrong citations to avoid |
|---|---|---|
| "Locale-appropriate output for operators" | AU-3 (intelligible audit records), SA-8 (engineering principles) | SC-28 is WRONG (SC-28 = protection of data AT REST) |
| "Domain derived from /proc/self/exe (provenance-safe)" | NSA RTB RAIN (non-bypassability), NIST SP 800-218 SSDF PW.4 | — |

## Logging

| Claim | Correct citation | Notes |
|---|---|---|
| Audit record content | AU-3 | Required on all logging modules |
| Audit record generation | AU-12 | Required on all logging modules |
| Timestamps | AU-8 | Required when journald is used |
| Protection of audit information | AU-9 | Required when discussing journal sealing or backend degradation |
| Debug information discipline | SI-11 | Required when variable data from env/config is near log output |

## Path safety

| Claim | Correct citation | Notes |
|---|---|---|
| Ownership/permission validation | SI-7 (Software Integrity), NSA RTB RAIN | |
| Traversal prevention | SI-10 (Input Validation), SI-7 | |
| Symlink chain validation | SI-7, NSA RTB RAIN | TOCTOU is the threat |

## Process isolation / subprocess

| Claim | Correct citation | Notes |
|---|---|---|
| Clean subprocess environment | CM-7, SI-7, CERT ENV03-C | |
| No TCP in air-gapped deployment | SC-7 (Boundary Protection), CM-7 | |
| Least privilege in subprocess env | AC-6, CM-7 | |

## Type safety / safe code

| Claim | Correct citation | Notes |
|---|---|---|
| `#![forbid(unsafe_code)]` compile-time proof | NIST SP 800-218 SSDF PW.4, NSA RTB | |
| Fail-closed construction | NIST SP 800-218 SSDF PW.4.1, NSA RTB RAIN | |
| `#[must_use]` on Result/security types | NIST SP 800-53 SI-10, SA-11 | RTB: Fail Secure |
