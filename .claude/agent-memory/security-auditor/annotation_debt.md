---
name: Known annotation debt modules
description: Modules and plans with systematic citation gaps identified in past audits
type: project
---

# Known Annotation Debt

## umrs-core::init (plan stage — not yet implemented)

**Audit date:** 2026-03-17
**Report:** `.claude/reports/2026-03-17-umrs-tool-init-compliance-audit.md`

**Systematic gaps:**
- Validator functions (`validate_safe_path`, `validate_lang`, `validate_tz`, etc.) lack
  per-function control citations. Module-level citations exist but individual validators
  are security-critical functions requiring explicit annotation.
- `init_i18n` cites SC-28 (incorrect). Must be updated to NSA RTB RAIN + AU-3 before
  the implementation doc comment is written.
- `init_logging` doc comment cites only AU-3 + AU-12. Missing AU-8, AU-9, SI-11.
- IA-5 appears as a citation for "no secrets in env" — incorrect; use CM-7 + AC-3.

**Status:** Pre-implementation — all gaps are in plan spec, not source code yet.
**Owner:** tech-writer updates plan; coder mirrors in source.

## General Pattern: SC-28 Misuse

SC-28 = Protection of Information at Rest (encryption of stored data).
It has appeared incorrectly in at least two locations in the umrs-tool-init plan.
When reviewing any module that touches i18n, locale, or display output — flag SC-28
if it appears and verify the claim it is attached to.

## General Pattern: IA-5 Misuse

IA-5 = Authenticator Management (password/token lifecycle).
It has appeared incorrectly as a citation for "no secrets in environment variables."
The correct citation for that claim is CM-7 + AC-3. Flag IA-5 in any context that
does not involve actual password/token creation, storage, distribution, or revocation.
