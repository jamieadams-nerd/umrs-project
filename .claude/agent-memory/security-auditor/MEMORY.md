# Security Auditor — Persistent Memory — "The IRS"
# Alias: The IRS (always include "The"). Real name: Herb.
# Portrait: docs/modules/ai-transparency/images/herb-auditor.png

## Topic Files
- [SCAP familiarization](scap_familiarization.md) — CCE mappings, STIG signal inventory, Tier-1 candidates
- [RMF methodology README](rmf-methodology-README.md) — SP800-37, 53A, 30, 39 corpus summary
- [Accreditation artifacts README](accreditation-artifacts-README.md) — SSP/SAP/SAR structures
- [Audit knowledge archive](audit_knowledge_archive.md) — CPU matrix, SEC pattern, RMF lifecycle, resolved items
- [Header terminology review](header-terminology-review.md) — SP 800-53A normative terms for TUI
- [Indicator definitions](indicator-definitions-plain-language.md) — 37 indicators, plain English
- [TUI/CLI corpus](tui-cli-corpus.md) — ratatui, crossterm, clap audit checkpoints

## Recurring Gap Patterns

### pub field defeats immutability claim
Look for `pub records`, `pub entries`, `pub log` on AU-10 types.

### symlink path vs real path in DB queries
RHEL 10 `/etc/os-release` → `/usr/lib/os-release`. Use `path_resolved` for DB queries.

### rusqlite Error Display leaks path info
Wrap and emit only category + error code. Never forward Display verbatim.

### bundled SQLite on FIPS RHEL
`features = ["bundled"]` bypasses RHEL patching. Prefer system SQLite.

### test unwrap() is acceptable
`unwrap()` in `tests/` is fine per project policy. Do not flag.

## Common Incorrect Citations
- "NSA RTB TOCTOU" on path-based checks — narrowing needed; TOCTOU applies only to fd-anchored ops.

## Open HIGH Findings (current)

1. **F-11**: `integrity_check.rs` — T4 via unvalidated `sha2` on FIPS systems. No FIPS gate. SC-13. Owner: coder.
2. **F-07**: `SecureReader::read()` missing `#[must_use]`. Owner: coder.
3. **SecureReader #[must_use] debt**: `new()` has bare #[must_use]; `read()` and `read_with_card()` missing entirely.

### SC-28 Claim Precision
`sealed_cache.rs` overstates SC-28: HMAC seal covers bytes never served (pipeline re-runs). Narrow to "tamper detection". Owner: tech-writer.

## Key Audit Frameworks

### Accreditation Anchors
- UMRS code review = SP 800-53A "Examine"; code is a system design document
- Missing annotation = "Other Than Satisfied" on SA-11
- High risks: remediated within 30 days; no authorization until cleared
- Citations must use canonical SP 800-53 Rev 5 form
- ODP values belong in the SSP, not code annotations (SDR-005 PENDING)

### RMF Anchors
- UMRS audit report = SAR component; finding + remediation = POA&M entry
- UMRS HIGH severity = SP 800-30 High/Very High impact

### Portfolio-Wide Gap
All plans: strong Implement, weak Assess/Monitor artifact production. Runtime outputs not connected to SAR-consumable Examine objects. **Check every new plan for output artifact format.**

## CUI Rules File Patterns (2026-03-30)

- LEI is a valid category abbreviation (General Law Enforcement), not an index group. Index group = "Law Enforcement". Never conflate them. The anti-pattern example in the old rules version was wrong.
- DL ONLY official NARA name is "Dissemination list controlled" — not "Distribution list controlled."
- RELIDO official authority acronym is SFDRA (Senior Foreign Disclosure **and Release** Authority). Check for dropped "and Release."
- ITAR vs EAR does not determine Basic vs Specified for EXPT — both regimes can produce either designation.
- Rules file JSON field table was incomplete: missing `handling_group_id`, `required_warning_statement`, `required_dissemination_control`.

## CUI Catalog Audit Patterns (2026-03-30)

Recurring gaps found in `US-CUI-LABELS.json` v0.3.0 audit:

- `mcs_ranges` metadata must match `labeling_mcs.md` constraint (`c0-c199` for US CUI, not c0-c249)
- `handling_group_id: ""` should be `null` — standardize with other optional fields
- Distribution Statements B–F belong ONLY to CTI (SP-CTI), not to EXPT or any other category
- Basic-tier entries use `CUI//ABBREV` key format (UMRS convention) vs. NARA's bare `CUI` for basic — must be documented in `_metadata.notes` to prevent operator misinterpretation
- Known mandatory warning statements: CVI (6 CFR 27.400), DCNI (10 U.S.C. 128), SSI (49 CFR 15/1520), UCNI (42 U.S.C. 2168), TAX (26 U.S.C. §§ 6103/7213), PCII (6 CFR 29.8), SGI (10 CFR 73.21)
- EXPT warning conflates ITAR (22 CFR 120-130) and EAR (15 CFR 730-774) — these require separate treatments
- RELIDO is a permissive foreign disclosure marking — logically mutually exclusive with NOFORN; add to `mutually_exclusive_with`

## Reports Index
- `2026-03-11-rpm-db-security-audit.md` — RPM findings
- `2026-03-11-os-detection-umrs-platform-surface-audit.md` — detect pipeline
- `2026-03-14-security-auditor-umrs-platform-audit.md` — in-depth platform
- `cpu-matrix-review/security-auditor-review.md` — CPU feature audit
- `tui-plan-security-review.md` — TUI audit card review
- `rmf-plan-review-2026-03-15.md` — RMF portfolio review
- `security-engineer-phase2b-review.md` — posture Phase 2b
- `docs/sage/reviews/2026-03-19-blog-cui-sign-lock.md` — blog accuracy audit
- `security-auditor-tui-review-2026-03-20.md` — TUI v1: 14A/17C/3E
- `security-auditor-tui-review-2026-03-20-v2.md` — TUI v2: 26A/3C/0E; 3 open (C-15v2 MEDIUM, C-T3-STATUS LOW, C-7v2 LOW)
- `code/2026-03-30-us-cui-labels-audit.md` — US-CUI-LABELS.json v0.3.0: 4E/9C; MCS range conflict, EXPT distribution stmt error, 5 missing warning stmts, RELIDO mutex gap; plus rules-file review: 1E/5C (LEI anti-pattern error, DL ONLY name, RELIDO title, JSON fields incomplete)

## TUI Audit Card Patterns
- IndicatorValue for kernel flags → cite SI-7 + CM-6 (NOT SI-3)
- SELinux: `enforce` value `0` = permissive (Inactive), not Active
- `indicator_unavailable` must be visually distinct from `indicator_inactive`
- Evidence display: structured codes (`OK(sha256)`), not narrative strings
- Tool version as HeaderField required for SP 800-53A Examine object
- Header terms: "Assessment" (not Report), "Scope" (not Subject), "Assessed" (not Checked)

## SCAP/STIG Key Checkpoints
- `cce` field on `IndicatorDescriptor` is IMPLEMENTED (2026-03-18); 13 entries populated
- NIST control precision gaps ALL RESOLVED: DmesgRestrict→SI-11, KptrRestrict→SC-30, etc.
- Remaining open items: FipsEnabled CCE-89085-5 (partial coverage), ModuleSigEnforce/SuidDumpable need explanatory comments
- Auditd: 51 rules → 3 composite indicators recommended (not yet implemented)
- UMRS exceeds STIG: 10 CPU mitigations + ModulesDisabled + Lockdown
- Work instruction report: `.claude/reports/2026-03-18-cce-crossref-audit.md`
- Full details: [scap_familiarization.md](scap_familiarization.md)

## TUI/CLI Checkpoints
- NO_COLOR: crossterm handles implicitly; verify no raw `\x1b` bypasses
- `--json` required on structured-data commands
- `ratatui::init()` / `ratatui::restore()` must be paired
- Ratatui v0.30.0: `frame.area()` not `frame.size()`; `block::Title` removed
- Full details: [tui-cli-corpus.md](tui-cli-corpus.md)
