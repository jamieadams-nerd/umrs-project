## CUI Phase 1 Language Rule

Applies to ALL agents, ALL documentation, ALL blog posts, ALL tool output.

Phase 1 operates on **targeted SELinux policy**. This provides **labeling, awareness,
and custody** — NOT mandatory enforcement.

### Permitted Language (Phase 1)

- "MCS labels are applied and visible"
- "UMRS labels CUI files with MCS categories"
- "Labels enable awareness and custody tracking"
- "Operators can see what is labeled and how"
- "Labeling supports policy development for future enforcement"

### Prohibited Language (Phase 1)

[CONSTRAINT] The following claims are false under targeted policy and must never appear:

- "UMRS enforces CUI handling"
- "CUI access is controlled by UMRS"
- "UMRS prevents unauthorized access to CUI"
- "Mandatory access control for CUI"
- Any implication that targeted policy provides mandatory enforcement

### Why This Matters

A security professional reading any UMRS content will treat every claim as a factual
statement. If we say "enforces" when we mean "labels," that is an overstated control
claim. It damages credibility with exactly the audience we need to reach. Worse, an
accreditor could flag it as a misrepresentation of the system's security posture.

### Phase 2 Language (when MLS is implemented)

When Phase 2 adds MLS policy with mandatory enforcement, these become accurate:

- "MLS policy enforces mandatory access control on CUI"
- "Access is denied at the kernel level when clearance does not dominate classification"
- "CategorySet evaluation provides non-discretionary enforcement"

### Enforcement

- Sage reviews all public content against this rule before publication
- Tech-writers apply this constraint in all CUI-related documentation
- Rust-developer ensures tool output strings do not use enforcement language
- Security-auditor flags violations during annotation reviews

### Compliance

- NIST SP 800-53 AC-3: Access Enforcement — accurately describing what level of
  enforcement exists prevents false claims
- NIST SP 800-53 PL-4: Rules of Behavior — accurate security documentation
- CMMC SC.L2-3.13.10: CUI system monitoring — output must accurately reflect capability
