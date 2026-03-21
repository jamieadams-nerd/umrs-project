# CUI Phase 1 Language Directive

**Date:** 2026-03-21
**Status:** approved
**Author:** Herb (The IRS), endorsed by Jamie
**ROADMAP Goals:** G3 (CUI & Five Eyes), G7 (Public Project)
**Tech Lead:** Herb (security-auditor) — authored; Sage enforces for public content
**LOE:** N/A — this is a constraint, not implementation work
**Applies to:** ALL agents, ALL documentation, ALL blog posts, ALL tool output

---

## The Rule

Phase 1 operates on **targeted SELinux policy**. This provides **labeling, awareness,
and custody** — NOT mandatory enforcement.

### Permitted Language (Phase 1)

- "MCS labels are applied and visible"
- "UMRS labels CUI files with MCS categories"
- "Labels enable awareness and custody tracking"
- "Operators can see what is labeled and how"
- "Labeling supports policy development for future enforcement"

### Prohibited Language (Phase 1)

- "UMRS enforces CUI handling" — **NEVER before Phase 2**
- "CUI access is controlled by UMRS"
- "UMRS prevents unauthorized access to CUI"
- "Mandatory access control for CUI" (true only in Phase 2 / MLS policy)
- Any implication that targeted policy provides mandatory enforcement

### Why This Matters

A security professional reading any UMRS content will treat every claim as a factual
statement. If we say "enforces" when we mean "labels," that is an **overstated control
claim**. It damages credibility with exactly the audience we need to reach. Worse, an
accreditor could flag it as a misrepresentation of the system's security posture.

Phase 2 adds MLS policy with mandatory enforcement. THAT is when enforcement language
becomes accurate. Until then: labels, awareness, custody.

### Phase 2 Language (when MLS is implemented)

- "MLS policy enforces mandatory access control on CUI"
- "Access is denied at the kernel level when clearance does not dominate classification"
- "CategorySet evaluation provides non-discretionary enforcement"

---

## Enforcement

- Sage reviews all public content against this directive before publication
- Tech-writers apply this constraint in all CUI-related documentation
- Rust-developer ensures tool output strings do not use enforcement language
- Security-auditor flags violations during annotation reviews

---

## Compliance

- NIST SP 800-53 AC-3: Access Enforcement — accurately describing what level of
  enforcement exists prevents false claims
- NIST SP 800-53 PL-4: Rules of Behavior — accurate security documentation
- CMMC SC.L2-3.13.10: CUI system monitoring — output must accurately reflect capability
