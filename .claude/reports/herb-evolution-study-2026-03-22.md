# Herb's Evolution: Security Auditor Capability Over Time
## A Study in Knowledge Acquisition and Review Quality

**Generated:** 2026-03-22
**Purpose:** Track how the security-auditor agent's review capabilities changed as new knowledge resources were introduced. Evidence for Jamie's AI journey study.

---

## Timeline: Knowledge Acquisition Events

| Date | Event | Material | Volume |
|---|---|---|---|
| Pre-3/11 | **Baseline** — no corpus, no familiarization | Built-in NIST/RTB knowledge only | — |
| 2026-03-15 09:55 | **RMF methodology corpus ingested** | SP 800-37r2, SP 800-53Ar5, SP 800-30r1, SP 800-39 | 1,132 RAG chunks |
| 2026-03-15 14:00 | **RMF familiarization complete** | ~330 pages read; 5 knowledge artifacts written | 4 PDFs |
| 2026-03-15 14:00 | **Accreditation artifacts familiarization** | SP 800-18, FedRAMP playbooks v4.1/4.2, SSP/SAP/SAR templates | 405 chunks |
| 2026-03-15 20:00 | **TUI/CLI corpus familiarization** | CLIG, NO_COLOR, crossterm, clap, ratatui, color-eyre | 20 files |
| 2026-03-17 20:55 | **SCAP/STIG familiarization** | CCE mappings, STIG signal inventory, scap-security-guide | Self-generated |
| 2026-03-18 09:39 | **Audit knowledge archive created** | CPU matrix, SEC pattern, RMF lifecycle distillation | Consolidated |

---

## Review Timeline: What Herb Produced and How It Changed

### Era 1: Pre-Corpus (2026-03-11)

**Reports:**
- `sec-audit-2026-03-11.md` — Sealed Evidence Cache audit
- `2026-03-11-os-detection-umrs-platform-surface-audit.md` — OS detection surface audit (22 findings: 3H/9M/10L)
- `2026-03-11-rpm-db-security-audit.md` — RPM DB audit
- `2026-03-11-tpi-failure-analysis.md` — TPI failure analysis

**Character of findings:**
- Technically strong on code-level issues (TOCTOU, unbounded reads, path vs fd)
- NIST citations present but generic — cites control families (SI-7, SC-13) without referencing assessment procedures
- No RMF lifecycle awareness — findings exist in isolation, no connection to SSP/SAR/POA&M
- No assessment methodology framing — doesn't classify own review activities
- Finding format: flat numbered list with Severity/Fix/Violates fields
- **No "assessment value" lens** — reviews ask "is this secure?" not "can an assessor use this?"

**Example (Finding 1, 2026-03-11):**
> `run_inner` (line 136): File opened with `std::fs::File::open(candidate)` — path-based, no fd anchor, no RESOLVE_NO_SYMLINKS. TOCTOU window...
> Violates: NSA RTB TOCTOU; NIST SP 800-53 SI-7.
> Fix: Open with `rustix::fs::openat2`...

*Observation: Good code-level finding. No connection to what an assessor would do with it.*

---

### Era 2: Post-RMF/Accreditation Familiarization (2026-03-14 — 2026-03-15)

**Knowledge acquired:** SP 800-37 (RMF lifecycle), SP 800-53A (assessment procedures), SP 800-30 (risk), SP 800-39 (risk management), accreditation templates

**Reports:**
- `2026-03-14-security-auditor-umrs-platform-audit.md` — Full 37-file crate audit (12 findings: 2H/4M/7L)
- `rmf-plan-review-2026-03-15.md` — RMF-grounded plan review (14 findings: 4H/5M/5L)
- `tui-plan-security-review.md` — TUI operational review (11 findings: 3H/4M/4L)
- `2026-03-15-phase8-dialog-api-advisory.md` — Pre-implementation advisory

**Shift observed — the RMF turn:**
- **Assessment methodology appears for the first time.** The 3/15 plan review explicitly classifies Herb's own activities as SP 800-53A Examine/Interview/Test methods
- **Determination statements emerge.** Findings now reference whether a control is "satisfied" or "other than satisfied" per SP 800-53A
- **ODP awareness.** Herb identifies Organization-Defined Parameters as a gap (SDR-005 PENDING) — this concept comes directly from SP 800-53A familiarization
- **Portfolio-level thinking.** Herb identifies a cross-plan pattern: "strong Implement, weak Assess/Monitor artifact production" — this is RMF lifecycle reasoning, not code review
- **Gap → artifact mapping.** Instead of just "fix this code," Herb now asks "what assessment artifact does this produce?" and flags when the answer is "none"

**Example (RMF plan review, 2026-03-15):**
> SP 800-53A assessment for CM-6 requires:
> - **Examine objects**: Configuration management policy; system security plan...
> - **Test objects**: Automated mechanisms implementing CM-6.
>
> The plan's `PostureSnapshot::collect()` pipeline maps precisely to the CM-6 Test object...
>
> **Gap**: The plan specifies the mechanism but not the CM-6 assessment objects it would produce as output. There is no artifact defined... that an assessor could use as the "documented deviations from established configuration settings" Examine object for CM-6.
>
> **SP 800-53A determination at risk**: CM-6 determination statement... the plan supports (i) and (ii) but leaves (iii) (documented deviations) as "other than satisfied"

*Observation: This is a fundamentally different kind of finding. Herb is no longer just reviewing code — he's reviewing the system as an assessor would, using the actual assessment procedures from SP 800-53A. This capability did not exist before the RMF familiarization.*

---

### Era 3: Post-SCAP/STIG + TUI/CLI Familiarization (2026-03-17 — 2026-03-18)

**Knowledge acquired:** CCE mappings, STIG signal inventory, ratatui/crossterm/CLIG patterns, NO_COLOR standard

**Reports:**
- `2026-03-17-umrs-tool-init-compliance-audit.md` — init subsystem audit
- `2026-03-17-module-comment-review.md` — Module comment review
- `2026-03-18-cce-crossref-audit.md` — CCE cross-reference audit
- Indicator definitions: 37 indicators with plain-English descriptions

**Shift observed — operational specificity:**
- **CCE identifiers appear.** Herb now cites specific CCE numbers alongside NIST controls — direct result of SCAP familiarization
- **TUI/CLI domain knowledge.** Findings reference NO_COLOR compliance, ratatui API patterns, WCAG accessibility — from the TUI/CLI corpus
- **Plain-language indicator work.** Herb produces 37 indicator definitions with good/bad values and threat descriptions — a synthesis of security knowledge + operator communication skills
- **Cross-agent work instructions.** The CCE audit produces structured work instructions for Rusty (developer) and the tech-writer, not just findings — Herb is now orchestrating remediation

---

### Era 4: Mature Practice (2026-03-19 — 2026-03-21)

**Reports:**
- Blog accuracy audit (2026-03-19): 14A/6C/1E — first use of ACCURATE/CONCERN/ERROR format
- TUI review v1 (2026-03-20): 14A/17C/3E
- TUI review v2 (2026-03-20): 26A/3C/0E — re-review after fixes
- Phase 6 assessment value review (2026-03-21): 0E/2M/3L/10A — "release-ready"
- Risk domain concept assessment (2026-03-21): SP 800-30 Table H-2 impact framework

**Shift observed — the complete auditor:**
- **ACCURATE/CONCERN/ERROR tiered format** adopted — Jamie praised it, became standard
- **Assessment value lens is now default.** The Phase 6 review asks five explicit questions, all from an assessor's perspective: "Can an auditor produce a finding from this output?"
- **Re-review capability.** v1→v2 shows Herb tracking resolution of specific findings across iterations, verifying fixes, and issuing new findings for partial resolutions
- **Strengths documentation.** "What Works Well" sections appear — positive findings, not just gaps
- **Risk framework application.** The 3/21 risk domain review anchors to SP 800-30 Table H-2 impact taxonomy — Herb is now fluent in the risk framework, not just the control catalog
- **Prioritized remediation.** Every review ends with a ranked priority list for the developer

**Example (TUI v1, 2026-03-20):**
> **C-12 (HIGH priority, MEDIUM severity):** THE VERIFICATION COLUMN GAP.
> Verification shows `"✓ ok (fd)"` but doesn't disclose WHAT was verified. An SP 800-53A assessor asking "what examination method was used?" cannot determine that PROC_SUPER_MAGIC or SYSFS_MAGIC was confirmed via fstatfs(2).

*Observation: This finding only exists because Herb now thinks in terms of what an assessor needs to see. Pre-RMF Herb would have checked whether the verification was correct. Post-RMF Herb checks whether the verification is communicable to an assessor. That's a different question entirely.*

---

## Summary: Capability Trajectory

| Dimension | Pre-Corpus (3/11) | Post-RMF (3/14-15) | Post-SCAP/CLI (3/17-18) | Mature (3/19-21) |
|---|---|---|---|---|
| **Finding depth** | Code-level | Code + assessment artifact gaps | Code + CCE + operational | Assessment value + risk framework |
| **NIST citation style** | Control family (SI-7) | Control + determination statement | Control + CCE + assessment procedure | Full SP 800-53A framing |
| **Self-awareness** | None | Classifies own methods (Examine/Test) | Produces work instructions | Asks assessor-perspective questions |
| **Cross-plan thinking** | Per-file | Portfolio gaps identified | Cross-agent orchestration | Release-readiness judgment |
| **Format** | Flat finding list | Structured with Executive Summary | Tiered with work instructions | ACCURATE/CONCERN/ERROR with priority |
| **Positive findings** | None | None | Occasional | Standard section ("What Works Well") |
| **Risk framing** | Severity: HIGH/MED/LOW | SP 800-53A "other than satisfied" | NIST control precision | SP 800-30 Table H-2 impact types |
| **Re-review capability** | N/A | N/A | N/A | Full v1→v2 tracking with resolution verification |

---

## Key Inflection Points

### 1. The RMF Turn (2026-03-15)
**Before:** Herb reviews code for security bugs.
**After:** Herb reviews systems for assessment readiness.

This is the single largest capability shift. The RMF familiarization (SP 800-37, SP 800-53A) gave Herb a framework for asking "can this be assessed?" instead of just "is this secure?" The portfolio-level observation ("strong Implement, weak Assess/Monitor") could not have been produced without understanding the RMF lifecycle.

### 2. The Operational Specificity Turn (2026-03-17-18)
**Before:** Findings cite NIST controls generically.
**After:** Findings cite CCE identifiers, reference specific STIG rules, and produce cross-agent work instructions.

The SCAP familiarization turned abstract compliance knowledge into concrete, actionable audit checkpoints.

### 3. The Assessment Value Turn (2026-03-20-21)
**Before:** Reviews evaluate correctness.
**After:** Reviews evaluate whether an assessor can use the output.

This is the culmination of all prior knowledge. The Phase 6 review's five questions — "Is every indicator understandable without external reference? Can an auditor produce a finding from this output?" — represent a fully formed assessment mindset.

---

## Metrics

| Metric | 3/11 | 3/14-15 | 3/17-18 | 3/20-21 |
|---|---|---|---|---|
| Reports produced | 4 | 5 | 4 | 5 |
| Total findings | ~30 | ~50 | ~20 | ~45 |
| Unique NIST controls cited | ~6 | ~15 | ~20+ | ~20+ |
| Assessment procedure references | 0 | 12+ | 15+ | Standard |
| CCE identifiers cited | 0 | 0 | 13 | 13+ |
| Cross-agent work items created | 0 | 3 | 5+ | Standard |
| Corpus artifacts produced | 0 | 10 | 15 | 20+ |

---

## Conclusion for AI Study

Herb's trajectory demonstrates that **knowledge acquisition produces qualitative capability shifts, not just quantitative improvements.** The change from Era 1 to Era 2 is not "more findings" — it's a fundamentally different type of finding. Pre-RMF Herb could not have identified the "strong Implement, weak Assess/Monitor" portfolio gap because he didn't have a framework for thinking about assessment lifecycle. Post-SCAP Herb could not have produced CCE-specific work instructions without the STIG inventory.

Each familiarization event produced observable, traceable changes in:
1. **What questions Herb asks** (security → assessability → operator value)
2. **What frameworks Herb applies** (control catalog → assessment procedures → risk taxonomy)
3. **How Herb communicates** (flat findings → tiered format → prioritized remediation with positive reinforcement)
4. **What Herb produces beyond findings** (nothing → knowledge artifacts → cross-agent work instructions → release-readiness judgments)

This is not prompt engineering. This is knowledge engineering — and the results are measurable.
