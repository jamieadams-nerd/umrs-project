---
name: umrs-c2pa operator review findings
description: First-time operator evaluation of umrs-c2pa tool and documentation (2026-04-02); 9 findings including systemd prerequisite, trust status clarity, silent fallback behavior
type: reference
---

# umrs-c2pa First-Time Operator Review

**Date:** 2026-04-02
**Reviewer:** Guest Administrator (RHEL sysadmin, zero prior UMRS context)
**Scope:** Binary (v0.1.0), docs (umrs-c2pa.adoc + man page), hands-on testing

## Findings Summary

| Severity | Count | Issues |
|----------|-------|--------|
| HIGH | 1 | Systemd prerequisite missing from Prerequisites list |
| MEDIUM | 4 | Trust status clarity, silent ephemeral fallback, missing high-assurance rationale, config validation help text |
| LOW | 4 | JSON flag naming ambiguity, help text argument documentation, confirmation prompts, file-optional behavior |

## Critical Findings

### HIGH: Systemd Prerequisite Missing (docs, lines 40–47 vs. 34–36)

The Quick Start section lists:
- `umrs-c2pa` binary installed to PATH
- A TOML configuration file

But the WARNING (line 34) says: "umrs-c2pa requires a Linux system with systemd... macOS and non-systemd Linux are not supported."

**Risk:** Operator on Alpine/minimal container installs binary, follows quick start, fails at runtime on journald init.

**Fix:** Move systemd requirement to Prerequisites list.

### MEDIUM: Trust Status UNVERIFIED vs. NO_TRUST_LIST Not Clearly Distinguished

When trust lists are configured but a signer (OpenAI) is not in the list, images show `UNVERIFIED`, not `NO_TRUST_LIST`. Documentation explains both statuses but does not warn that major AI platforms may not be in the C2PA consortium list.

**Impact:** Operator might assume trust list is broken or misconfigured.

**Fix:** Add NOTE explaining that `UNVERIFIED` is normal for signers outside the C2PA consortium.

### MEDIUM: Silent Fallback to Ephemeral Mode

If operator sets `cert_chain` but forgets `private_key`, config loads silently and tool enters ephemeral mode. All manifests are marked `UNVERIFIED`. No warning during config load.

**Impact:** In large ingest pipeline, untrusted manifests might go unnoticed for days.

**Fix:** Move silent-fallback WARNING to CAUTION block immediately after `private_key` field in config docs.

## Hands-On Test Results

✓ All core operations successful:
- config generation, validation
- creds generation, validation
- signing without marking (c2pa.acquired)
- signing with marking (--marking "CUI//SP-CTI//NOFORN")
- re-signing (c2pa.published, hash consistency PASS)
- inspection with formatted output, --json, --chain-json, --detailed-json
- verbose mode (clear progress output)
- error handling (file not found, invalid TOML, overwrite safeguard)

✓ Security implementation solid:
- Keys created with atomic 0600 permissions
- Zeroizing buffers mentioned
- FIPS algorithm enforcement
- Journald fallback to stderr

⚠ Single concern: Operator confidence level is "good with docs nearby" rather than "confident standalone." High-assurance "why" is missing from Purpose section.

## Operator Usability Grade

| Category | Grade | Notes |
|----------|-------|-------|
| CLI intuitiveness | A | Flag names are clear; help text is complete |
| Error messages | A | Actionable, specific, exit codes are meaningful |
| Documentation clarity | B+ | Comprehensive but assumes some C2PA familiarity |
| Configuration safeguards | B- | Silent fallback is a risk in large deployments |
| Security implementation | A | FIPS, key protection, tamper detection all correct |

## Full Report

See `.claude/reports/admin-reports/2026-04-02-umrs-c2pa-operator-eval.md`
