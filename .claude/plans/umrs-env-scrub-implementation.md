# Plan: Environment Scrubber Implementation (`umrs-core::init::scrub`)

**Status:** draft — awaiting Jamie's review and approval
**Date:** 2026-04-13
**Author:** Opus (session lead) with Rusty (familiarized 2026-04-13)
**ROADMAP alignment:** G2 (Platform Library), G4 (Tool Ecosystem)
**Parent plan:** `.claude/plans/umrs-tool-init.md` — this plan refines Sub-Phases 1a–1c with design decisions locked after the Librarian's research was completed.

---

## Authoritative References

All implementation must align with these documents. Citations in source code and `ScrubReport` output must point back to them.

| Reference | Location | Why it matters here |
|---|---|---|
| Librarian's env attack-prevention report | `.claude/references/reports/2026-04-12-env-attack-prevention.md` | Ground truth for tier lists, CVE rationale, secure-execution behavior, implementation notes |
| Env sanitization rules | `.claude/rules/env_sanitization_rules.md` | Distilled axioms/constraints/rules; Tier 1/2/3 tables; CWE↔NIST mapping |
| High-assurance pattern rules | `.claude/rules/high_assurance_pattern_rules.md` | `#[must_use]` discipline, validate-at-construction, security-findings-as-data, debug log information discipline |
| Rust design rules | `.claude/rules/rust_design_rules.md` | `forbid(unsafe_code)`, citation format, system-state-read prohibition |
| Parent plan | `.claude/plans/umrs-tool-init.md` | Phase ordering, test targets, file inventory |

---

## Design Decisions Locked (2026-04-13)

Three open questions surfaced during Rusty's familiarization. Jamie approved the leans below.

### D1 — PATH Symlink Handling (resolve-and-stat)

**Decision:** `validate_safe_path()` resolves symlinks and stats the target using the fd-anchored pattern already used elsewhere in `umrs-selinux` / `umrs-platform`. Refusing symlinks outright would false-positive on RHEL 10's `/usr/bin → /bin` and other legitimate platform links.

**Why this is safe:** Fd-anchored resolution prevents TOCTOU substitution between the readlink call and the stat. See NSA RTB (Non-Bypassability, TOCTOU) and NIST SP 800-53 SI-7 — the pattern is already a project standard for `/proc`, `/sys`, and xattr reads.

**Implementation note:** Reuse `SecureReader` / `openat` primitives. Do not re-implement path resolution; route through existing umrs-platform helpers. This is also a `[CONSTRAINT]` from `rust_design_rules.md` System State Read Prohibition.

### D2 — `ScrubFinding` Severity Axis

**Decision:** Add a severity enum to `ScrubFinding`:

```rust
pub enum Severity {
    /// Presence alone is a security finding. Cannot be legitimate.
    /// Examples: LD_PRELOAD, GLIBC_TUNABLES, BASH_ENV, PYTHONPATH.
    Critical,
    /// Tier 1 by attack-surface lineage but commonly set for legitimate
    /// reasons. Report as finding; do not trigger page-the-operator alerts.
    /// Examples: TMPDIR, TZDIR when set to a validated path.
    Advisory,
    /// Tier 2 — value failed validation. Severity derives from the validator.
    ValidationFailure,
}
```

**Why:** Operator alert fatigue is a real failure mode. The Librarian report is explicit that TMPDIR is a loader-injection vector *and* extremely commonly set. Uniform severity forces operators to either treat every scrub report as an incident (unsustainable) or learn to ignore them (defeats the control). The severity axis preserves the full finding surface while letting downstream tooling triage.

**Cross-reference:** NIST SP 800-53 AU-3 (Content of Audit Records — finding data must be structured, not prose), security-findings-as-data rule in `high_assurance_pattern_rules.md`.

### D3 — Platform-Posture Sidecar on `ScrubReport`

**Decision:** `ScrubReport` carries a sibling `PlatformPosture` struct that annotates findings with patch status, rather than embedding platform-specific metadata in the finding itself:

```rust
pub struct ScrubReport {
    pub findings: Vec<ScrubFinding>,
    pub posture: PlatformPosture,   // sidecar — does not alter findings
    // ...
}

pub struct PlatformPosture {
    pub glibc_version: Option<String>,
    pub cve_2023_4911_patched: Option<bool>,
    pub kernel_release: Option<String>,
    pub at_secure_active: bool,
    pub no_new_privs_set: bool,
}
```

**Why separate:** Findings must remain uniform and machine-comparable across systems (audit trail integrity — NIST SP 800-53 AU-10). Platform posture is context for the operator, not a property of the finding. Keeping them separate means a report exported from a RHEL 10 system and one exported from a misconfigured Ubuntu system produce directly comparable finding vectors; the posture sidecar explains any divergence in recommended action without contaminating the finding itself.

**Cross-reference:** NIST SP 800-53 AU-10 (audit record integrity), CM-8 (component inventory — glibc version is an inventory fact).

### D4 — Every Finding Self-Explains

**Decision:** Every `ScrubFinding` variant implements `explanation() -> &'static str` and `remediation() -> &'static str`. Every log line emitted by the scrubber includes the CVE/CWE citation when applicable. No terse severity codes alone.

**Why:** Operators will not have the Librarian report in hand when reading scrub output. The output is the teaching surface. An unexplained "`WARN: GLIBC_TUNABLES present`" tells them nothing; "`WARN: GLIBC_TUNABLES present (CVE-2023-4911 vector; RHEL 10 glibc patched per posture report, but presence is still anomalous for a non-setuid tool)`" tells them what to do.

**Cross-reference:**
- NIST SP 800-53 AU-3 (audit records must be actionable) and SI-11 (error handling — meaningful messages, no sensitive leakage)
- `feedback_scrub_report_verbosity.md` (Jamie's explicit direction, 2026-04-13)
- Debug Log Information Discipline rule in `high_assurance_pattern_rules.md` — explanation text MUST NOT include raw env values from Tier 1 variables (loader paths), only variable names and CVE/CWE citations. Rationale: raw LD_PRELOAD values may point to attacker-controlled paths an operator should not blindly re-read from logs.

---

## Type Surface Additions to Parent Plan

These are *additions* to the types already sketched in `umrs-tool-init.md`. They supersede any prior sketch on conflict.

```rust
// In umrs-core/src/init/scrub.rs (Phase 1c)

#[must_use = "A ScrubReport represents a security audit; ignoring it defeats the control"]
pub struct ScrubReport {
    findings: Vec<ScrubFinding>,
    posture: PlatformPosture,
    scrub_duration: Duration,
}

#[must_use = "A ScrubFinding is a security event; it must be surfaced or explicitly consumed"]
pub enum ScrubFinding {
    DangerousLoaderVariable { name: &'static str, cve: Option<&'static str>, severity: Severity },
    SecretPatternVariable   { name: String,       cwe: &'static str,          severity: Severity },
    ValidationFailure       { name: &'static str, validator: &'static str,    severity: Severity },
    SecurityDecisionFromEnv { name: &'static str, severity: Severity }, // CWE-807
}

impl ScrubFinding {
    pub const fn severity(&self) -> Severity;
    pub fn explanation(&self) -> &'static str;   // operator-facing WHY
    pub fn remediation(&self) -> &'static str;   // operator-facing WHAT NEXT
    pub fn cwe(&self) -> &'static str;           // always citable
    pub const fn nist_controls(&self) -> &'static [&'static str];
}
```

Every public item above carries `#[must_use]` with a message (per `high_assurance_pattern_rules.md` Must-Use Contract).

---

## Commentary & Logging Standards

To satisfy D4 and the Error Information Discipline rule in `high_assurance_pattern_rules.md`:

1. **Module `//!` block** must cite: NIST SP 800-53 CM-7, SI-7, SI-10, SI-11, SC-3, AU-3, AU-10, IA-5, SC-28; NIST SP 800-218 SSDF PW.4.1; NSA RTB RAIN; CWE-427/454/526/807/20.

2. **Every Tier 1 variable constant** gets an inline citation comment:
   ```rust
   // glibc ld.so(8) secure-execution stripped variable — NIST SP 800-53 CM-7, SI-7
   // Presence is a CWE-427 (Uncontrolled Search Path Element) finding.
   // Reference: CVE-2023-4911 (GLIBC_TUNABLES buffer overflow).
   const GLIBC_TUNABLES: &str = "GLIBC_TUNABLES";
   ```

3. **Log lines** follow this pattern:
   ```
   [scrub] Critical finding: LD_PRELOAD present
       rationale: dynamic linker preload injection vector (MITRE T1574.006)
       controls:  NIST SP 800-53 CM-7, SI-7; CWE-427
       remediation: unset LD_PRELOAD before invoking UMRS tools; review parent process
   ```
   **Never** log the raw value of a Tier 1 variable. Log only the variable name, CVE/CWE, and remediation text.

4. **Rustdoc examples** on every public item show both the happy path and a finding path so operators and developers can see the shape of real output.

---

## Threat-Model Cross-Reference

Each design decision explicitly maps to a threat. If a reviewer removes a decision, they must show the threat is no longer credible.

| Decision | Counters | Evidence |
|---|---|---|
| Snapshot-at-startup, never mutate parent env | Rust 2024 thread-safety unsoundness; TOCTOU on concurrent getenv | Librarian report §5.1; Rust RFC on unsafe `set_var` |
| Tier 1 list from ld.so(8) + interpreter family | CVE-2023-4911, CVE-2024-48990/48992, Shellshock, NLSPATH overflows, APT41/Ebury/Rocke LD_PRELOAD | Librarian report §1.1–1.11, §4.1, §4.6 |
| `Command::env_clear()` for children | Needrestart-class privilege inheritance | Librarian report §1.2 |
| Symlink resolve-and-stat (fd-anchored) | TOCTOU between readlink and stat on PATH entries | NSA RTB, project pattern |
| Severity axis | Alert fatigue → operators ignore the control | Librarian report §4.1 TMPDIR notes |
| Platform posture sidecar | Cross-system finding comparability under audit | NIST SP 800-53 AU-10 |
| Self-explaining findings | Operator cannot action terse codes | Jamie 2026-04-13; NIST SP 800-53 AU-3 |

---

## Execution Mapping to Parent-Plan Phases

| Parent phase | What this plan adds |
|---|---|
| 1a (simple validators) | No new work. Cite `env_sanitization_rules.md` Tier 2 validator contracts in rustdoc. |
| 1b (complex validators) | D1 — `validate_safe_path()` uses fd-anchored resolve-and-stat via existing umrs-platform primitives. |
| 1c (ScrubReport, SanitizedEnv) | D2 + D3 + D4 — severity axis, platform-posture sidecar, self-explaining findings, commented/cited output. |
| 1d–1f | No change. |
| 2 (`umrs-env` binary) | Default output format surfaces Critical findings prominently, groups Advisory separately, prints posture sidecar below. `--json` emits full structured report. |

---

## Acceptance Criteria

1. All Tier 1 variables from `env_sanitization_rules.md` are tested — one "present" case per variable generating the expected finding.
2. TMPDIR produces `Advisory` severity; LD_PRELOAD produces `Critical`.
3. PATH containing a symlink to a root-owned non-world-writable directory validates successfully.
4. PATH containing a symlink to a world-writable directory fails validation with a citable finding.
5. `ScrubReport::findings` are deterministic in order (alphabetical by variable name) for audit-log reproducibility (NIST SP 800-53 AU-10).
6. `PlatformPosture::at_secure_active` correctly reports `false` for non-setuid test runs.
7. Zero clippy warnings on `pedantic + nursery`.
8. `cargo test -p umrs-core` passes with all new tests under `tests/init/`.
9. No raw Tier 1 variable values appear anywhere in log output (grep test).
10. Every new public item has `#[must_use]` with a descriptive message and a rustdoc example.

---

## Out of Scope

- RAG ingestion of upstream sources (deferred per 2026-04-13 decision; familiarization + rule pointer is sufficient).
- `prctl(PR_SET_NO_NEW_PRIVS)` enforcement from application code — requires `unsafe` or `nix`, conflicts with `forbid(unsafe_code)`. Recommendation deferred to systemd unit hardening (see `env_sanitization_rules.md` Patterns).
- Setuid helpers — not currently planned; if introduced, glibc secure-execution strips variables before program entry and this scrubber becomes defense-in-depth rather than the only defense.

---

## Open Questions for Jamie

1. Should `PlatformPosture` be populated eagerly on every scrub, or lazily on demand? Eager is simpler and more honest; lazy saves ~1 ms at init.
2. Should the `umrs-env` CLI (Phase 2) exit non-zero when any `Critical` finding is present? Leaning yes (treat scrub as a gate in CI/pre-exec hooks), but it makes the tool unusable for inspection on already-compromised systems unless a `--report-only` flag exists.
3. Secret-pattern detection: maintain the list in-tree, or make it configurable per-deployment? Leaning in-tree (tamper-resistant default) with an additive allowlist via config file.

---

## Review Checklist (for Jamie)

- [ ] Design decisions D1–D4 correctly capture intent
- [ ] Threat-model cross-reference is complete
- [ ] Acceptance criteria are testable
- [ ] Cross-references to rules files and Librarian report are accurate
- [ ] Open questions surfaced rather than silently resolved
