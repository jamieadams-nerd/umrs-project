---
name: Event acknowledgement architecture — trust-impact tiering
description: Three-tier event system where acknowledgement is driven by trust impact, not severity. Seven ack-required classes defined. Foundation for future audit/logging subsystem.
type: project
---

## Core Principle

Require acknowledgement when an event threatens trust in: the object, the label, the control plane, the evidence trail, or custody. Severity alone is not the criterion — trust impact is.

**Doctrine:** "Events require acknowledgement when they materially reduce trust in system state, control state, evidence state, or custody state."

## Tier 1 — Must Acknowledge

Seven acknowledgement-required event classes:

1. **INTEGRITY_COMPROMISE** — hash mismatch, signature failure, unexpected label/xattr divergence, audit record integrity failure
2. **AUDIT_ASSURANCE_DEGRADED** — logging pipeline failure, queue overflow, dropped events, signing failure, clock/time-sync failure
3. **SECURITY_POSTURE_DEGRADED** — SELinux disabled/permissive unexpectedly, kernel lockdown disabled, FIPS posture changed, critical kernel setting drift
4. **POLICY_OR_TRUST_CONFIG_CHANGED** — policy reload, category/label taxonomy change, trust anchor change, audit retention rule change
5. **CONFIRMED_OR_HIGH_CONFIDENCE_INTRUSION** — successful unauthorized privileged action, exfiltration indicator, malware detection, tamper with audit components
6. **CUSTODY_OR_LABELING_TRUST_VIOLATION** — data moved outside expected workflow, labels altered unexpectedly
7. **BREAK_GLASS_OR_OVERRIDE_USED** — emergency bypass, vault promotion override, disabling integrity verification

## Tier 2 — Must Notify, Conditional Ack

Become ack-required when: repeats past threshold, affects privileged account, touches protected data, persists beyond interval, or correlation raises hostile confidence.

Examples: repeated auth failures, account lockouts, high AVC volume, suspicious but unconfirmed access attempts.

## Tier 3 — Record Only

Expected denials, routine policy-conformant blocks, normal service restarts, informational posture snapshots.

**Why:** If humans must acknowledge Tier 3 events, they'll acknowledge everything blindly — destroying acknowledgement as a control.

## Acknowledgement Payload (Minimum)

- Who acknowledged, when, role/authority
- Event/case ID
- Disposition: under investigation / accepted operational risk / false positive / maintenance-related / contained / escalated
- Optional notes/ticket reference

Acknowledgement = "A responsible human has reviewed the condition, accepted awareness, and recorded an initial disposition." Not "I clicked OK."

**Status:** DRAFT — future work for the logging/audit subsystem. Not actionable now.
**How to apply:** Rusty uses these classes when implementing the logging/auditing subsystem. Security-auditor validates alignment with NIST AU-5, IR-4/IR-5, SI-4, SI-7.
**Source:** `.claude/jamies_brain/target-mls.txt` (second half)
