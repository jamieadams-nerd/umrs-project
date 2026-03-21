# CA-7 Monitoring Frequency (ODP Table)

**Date:** 2026-03-21
**Status:** draft — awaiting Jamie review
**Author:** Herb (The IRS)
**ROADMAP Goals:** G4 (Assessment Engine), G5 (Security Tools)
**Milestones:** M2 (Assessment Capable) — must be defined before assessment engine
**Tech Lead:** Herb (security-auditor)
**LOE:** Small (~1 session to finalize; document only, no code)

---

## Purpose

Define the Organization-Defined Parameter (ODP) for NIST SP 800-53 CA-7 continuous
monitoring frequency across all UMRS tools. This table is the authoritative reference
for prescribed routine audit scheduling. The assessment engine (M2) needs these values
to emit correct audit records.

---

## Monitoring Frequency Table

| Tool | Check Type | Frequency | Rationale | Escalation Trigger |
|---|---|---|---|---|
| `umrs-state` | Kernel security indicators, SELinux mode | Daily at boot + on-demand | CA-7 ODP: high-impact systems require near-continuous monitoring; daily startup is minimum viable | Any indicator change from last known-good (BootDrift/EphemeralHotfix) → immediate ack required |
| `umrs-state` | Full contradiction scan (live vs configured) | Weekly | Contradiction analysis matches CM-3 change review cycles | New ContradictionKind → operator ack within 72 hours |
| `umrs-ls` | Critical path scan (`/etc`, `/bin`, `/sbin`, `/usr/sbin`) | Weekly | Matches AIDE cadence; CCE-86441-3 references weekly AIDE runs | Unexpected setuid, label change, or xattr mutation → HIGH, immediate ack |
| `umrs-ls` | On-demand (operator-initiated) | As needed | Not a scheduled audit | SecurityObservation findings → log and flag |
| `umrs-state` | FIPS mode verification | On boot only (systemd unit) | FIPS is boot-time; drift impossible within session unless live-patched | FIPS disabled on previously FIPS-enabled host → CRITICAL, immediate escalation |

---

## Escalation Path

| Severity | Acknowledgement Window | Stale Ack Escalation |
|---|---|---|
| CRITICAL | 4 hours | If no ack in journald within 24 hours, generate stale-ack event |
| HIGH | 72 hours | Weekly review flags unacknowledged HIGHs |
| MEDIUM | Weekly review cycle | Batch review |
| LOW | Weekly review cycle | Batch review |

---

## Event Acknowledgement Mapping

- CRITICAL and HIGH findings → **mandatory-acknowledgement tier**
- MEDIUM and LOW findings → **advisory tier**
- journald correlation by `boot_id` is sufficient for ack chain at M2 scope

---

## Dependencies

- Assessment engine must consume these frequencies for audit record cadence
- Event acknowledgement architecture (3-tier system) maps to the escalation tiers
- systemd timer units needed for daily/weekly scheduled runs

---

## Compliance

- NIST SP 800-53 CA-7: Continuous Monitoring — this table IS the ODP
- NIST SP 800-53 CM-3: Configuration Change Control — weekly contradiction cadence
- NIST SP 800-53A: Assessment methodology — frequencies inform assessment planning
- CCE-86441-3: AIDE weekly monitoring cadence alignment
