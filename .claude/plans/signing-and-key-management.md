# Signing and Key Management Plan

**Date:** 2026-04-17
**Status:** draft — parked (consolidation of scattered items)
**Author:** Jamie (consolidated by orchestrator)
**ROADMAP Goals:** G1 (Platform Awareness), G4 (Assessment Engine), G5 (Security Tools)
**Milestones:** M3.5 (Full Deployment), M4 (Assessment)
**Tech Lead:** Knox (security-engineer) for SELinux policy; Rusty (rust-developer) for tooling
**LOE:** Large (multi-session)

---

## Purpose

Consolidate all signing, key management, and cryptographic custody work items into one
plan. These items were scattered across session notes, cross-team notes, memory files,
and deferred lists. Nothing here is actively being worked — this plan parks the work
cleanly so it can be picked up as a unit.

---

## Authoritative Specification

`components/rusty-gadgets/selinux/KEY-MANAGEMENT-DIRS.md` (UMRS-SEC-KM-001)
— FHS 3.0 + NIST SP 800-57 Rev 5 + SP 800-53 Rev 5 + CMMC L2.

This is Jamie's design. All work in this plan implements that spec.

---

## 1. Key-Material Directory Architecture (LANDED)

Architecture decided 2026-04-16. Keys live OUTSIDE `/opt/umrs` per FHS §4.5 vs
NIST CM-6/SA-8 resolution:

| Path | Purpose | SELinux Type |
|---|---|---|
| `/etc/keys/umrs/sealing/` | Sealing keys (at rest) | `umrs_seal_key_t` |
| `/etc/keys/umrs/signing/` | Active signing keys | `umrs_sign_key_t` |
| `/etc/keys/umrs/wrapping/` | Key-encryption keys | `umrs_kek_t` |
| `/etc/keys/umrs/staging/` | Pre-activation keys | `umrs_sign_key_t` (reuse) |
| `/var/lib/umrs/keys/active/` | Runtime session keys | `umrs_session_key_t` |
| `/var/lib/umrs/keys/suspended/` | Temporarily unusable | `umrs_retired_key_t` (reuse) |
| `/var/lib/umrs/keys/retired/` | Deactivated, awaiting destroy | `umrs_retired_key_t` |

Config: `/etc/opt/umrs/key-policy.toml` (the ONLY key-related file in the FHS add-on tree).

All 5 types bound to `umrs_secret_type` attribute.

---

## 2. Open Decisions (Jamie)

These were flagged by Knox at end of 2026-04-16 session:

### 2a. Staging type reuse
`/etc/keys/umrs/staging/` uses `umrs_sign_key_t` instead of a dedicated
`umrs_staging_key_t`. Phase 2 may need separation if pre-active state
requires distinct audit visibility.

### 2b. Suspended type reuse
`/var/lib/umrs/keys/suspended/` uses `umrs_retired_key_t`. Same consideration
— if suspended keys need different access rules than retired keys.

### 2c. Container type for `/etc/keys/umrs/`
Currently labeled `umrs_config_ro_t` but NOT bound to `umrs_config_type`
attribute. Binding it would block Phase-1 unconfined signing manager via
existing `neverallow`.

### 2d. `umrs_sign_t` process domain
Dedicated SELinux domain for the signing manager — Phase 2. `neverallow` over
`umrs_secret_type` is gated behind this domain existing.

---

## 3. `umrs-sign-mgr.sh` Script Work

**Location:** `components/rusty-gadgets/scripts/umrs-signing/umrs-sign-mgr.sh`
**Install target:** `/opt/umrs/bin/umrs-sign-mgr` (`.sh` suffix drops on install)

### 3a. Already done (2026-04-16)
- Smart-quote / en-dash syntax damage repaired
- Shebang fixed to `#!/usr/bin/bash`
- PATH lock + env sanitization added
- Strict mode + IFS added
- Self-check block added

### 3b. Still needed
- **BASE_DIR** still defaults to retired `/etc/opt/umrs/pki/` — must update to
  `/etc/keys/umrs/` and distribute per-profile key directories across
  `sealing/`, `signing/`, `wrapping/` per the new architecture
- **8 signing profiles** (audit, syslog, journal, logrotate, archive, config,
  rpmdb, swrel) — confirm which are active Phase 1 vs deferred
- **Ownership policy** — `umrs:umrs` vs `root:umrs 2750` for private key dirs
- **syslog logging** — TODO in script header (rsyslog `logger` calls per
  `secure_bash_rules.md` Logging Rule)
- **`key-policy.toml`** — ship the actual config file, not just the fcontext

### 3c. Documentation
- `umrs-signing-README.adoc` body still references legacy `/etc/pki` prose —
  needs holistic revision when sign-mgr paths are finalized

---

## 4. `umrs-shred.sh` Script

**Location:** `components/rusty-gadgets/scripts/umrs-shred/umrs-shred.sh`
**Install target:** `/opt/umrs/bin/umrs-shred`

Mostly ready. Minor items:
- Shebang already fixed to `#!/usr/bin/bash`
- Good hygiene (strict mode, env sanitization, umask, noclobber)
- No key-material dependencies
- Install: `chmod 0755`, owner `umrs:umrs`

---

## 5. Installer Updates

`umrs-install.sh` currently creates `/etc/keys/umrs/*` as `root:root 0700`.
Does NOT create `/var/lib/umrs/keys/*` — fcontext is declared, creation is lazy.

When sign-mgr work resumes:
- Decide if `/var/lib/umrs/keys/{active,suspended,retired}/` should be
  pre-created by installer or lazy-created by tools
- Ensure `restorecon` runs on new key dirs after creation

---

## 6. Staging Pipeline

`xtask stage.rs` already handles nested scripts (2026-04-16 work):
- Recurses `scripts/` one level
- Strips `.sh` suffix on copy
- Detects duplicate stem names across flat + nested
- 4 tests covering the behavior

No further staging work needed unless new scripts are added.

---

## 7. Key Destruction Service

NIST SP 800-57 §5.3 + NIST SP 800-88 (MP-6) require cryptographic erasure for
`retired/` keys. Noted in KEY-MANAGEMENT-DIRS.md §6.2 resolution. This is
Phase 2+ work — no implementation exists yet.

---

## 8. Related Items (Not In Scope But Adjacent)

- **C2PA vault pipeline** — uses signing keys but has its own plan
  (`c2pa-vault-prototype.md`)
- **IMA/EVM signing** — kernel integrity subsystem, separate from UMRS app-level signing
- **`umrs-secadm` Linux group + SELinux user** — future; forward-looking NOTE exists
  in `dac-users-groups.adoc` and installer comment

---

## Execution Order (When Resumed)

1. Jamie resolves open decisions (§2a–2d)
2. Update `umrs-sign-mgr.sh` BASE_DIR and profile-to-directory mapping
3. Ship `key-policy.toml`
4. Revise `umrs-signing-README.adoc`
5. Test end-to-end on VM
6. Address suspended/staging type separation if Jamie decides to split
7. Key destruction service (Phase 2+)
