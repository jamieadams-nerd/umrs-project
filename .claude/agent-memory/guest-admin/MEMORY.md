# Guest Administrator Memory

## umrs-c2pa Operator Evaluation (2026-04-02)

- [umrs-c2pa findings](umrs-c2pa-findings.md) — first-time operator review of tool and docs; 9 findings recorded
- **Key issue:** systemd requirement not in Prerequisites; high-assurance "why" underdocumented
- **Strength:** CLI is intuitive, error messages are clear, security implementation is solid
- **Hands-on scope:** config generation, creds gen, signing with/without markings, inspection, trust validation

## Operator Perspective Notes

As an external RHEL sysadmin, I evaluate documentation from the angle: "Would I understand this on day one?" UMRS documentation generally assumes Rust or security engineering background. When domain-specific terms (TPI, MLS, CUI) appear, they should be either:
1. Defined inline with a link to full reference
2. Flagged as a prerequisite (e.g., "familiarity with CUI marking standard")

Tools I've tested so far:
- umrs-c2pa: functional, good UX, security-conscious design
