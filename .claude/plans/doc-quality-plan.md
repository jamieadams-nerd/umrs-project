# Plan: UMRS Documentation Quality System

**Status:** On hold (back burner per Jamie 2026-03-20) — tooling installed but Vale QA is too much distraction right now. Resume after first release.

## Purpose

This plan establishes a documentation quality feedback loop for UMRS Antora content.
The system enforces terminology consistency, STE compliance on procedural content,
readability standards, and admonition correctness — and produces trend data to measure
improvement over time.

---

## Vale Capability Assessment (2026-03-19)

After reviewing Vale's documentation (https://vale.sh/docs/), these are the key capabilities
that make it a strong fit for UMRS and how they map to our needs.

### Why Vale fits UMRS

- **AsciiDoc is natively supported** — no format conversion needed for our Antora docs
- **Custom styles map directly to our existing rules** — STE mode, admonition hierarchy,
  citation format, approved verbs, and terminology consistency are all already codified
  in `.claude/rules/*.md` files. Vale operationalizes them as machine-enforceable checks
- **It catches mechanical issues before human reviewers spend time** — freeing the auditor,
  tech-writer, and senior-tech-writer to focus on accuracy, threat model correctness, and
  narrative flow rather than "you used 'utilize' again"
- **Rust source comments can be linted** — Vale processes code files, which opens a future
  path for `///` and `//!` doc comment enforcement (out of scope for now; see Future Expansion)

### Vale style mapping to UMRS rule sources

| Proposed Vale Style | Source of Truth | Scope |
|---|---|---|
| `UMRS-STE` | `.claude/rules/ste_mode.md` | Procedural docs — approved verbs, sentence length ≤20 words, active voice, banned ambiguous words |
| `UMRS-Citations` | `.claude/rules/rust_design_rules.md` §Citation Format Rule | All docs — canonical `NIST SP 800-53` (not `NIST 800-53`), `NSA RTB`, `FIPS 140-3`, `CCE-NNNNN-N` forms |
| `UMRS-Terminology` | `approved_terminology.md` + glossary module | All docs — one canonical term per concept, substitution rules for forbidden variants |
| `UMRS-Blog` | Sage feedback memories + outreach style conventions | Blog/whitepaper content — tone, audience accessibility, no unexplained jargon |
| `UMRS-Admonitions` | `.claude/rules/admonition_hierarchy.md` | All AsciiDoc — WARNING/CAUTION/IMPORTANT/NOTE/TIP hierarchy; no inline `Note:` labels |

### Vale rule types we will use

These are the Vale extension points (rule types) most relevant to our needs:

| Extension | UMRS use case |
|---|---|
| `existence` | Detect banned words (utilize, leverage, facilitate), informal admonition labels (`Note:`, `Warning:`), ambiguous terms (appropriate, various, several) |
| `substitution` | Enforce canonical terminology — "security label" → "security context", "driver" → "kernel module", `NIST 800-53` → `NIST SP 800-53` |
| `consistency` | Ensure the same term is used throughout a document — not "kernel module" in one paragraph and "driver" in the next |
| `occurrence` | Enforce sentence length limits on procedural content (≤20 words per STE rule) |
| `capitalization` | Validate heading conventions, acronym formatting |
| `spelling` | Custom vocabulary with Accept/Reject lists for UMRS domain terms |
| `conditional` | Scope-dependent rules — apply STE strictness only to numbered procedure blocks |

### Vocabulary management

Vale's Vocab system maps cleanly to what we need:

- **Accept list:** UMRS-specific terms that spell-checkers would flag as errors — CategorySet,
  SecurityContext, TPI, CUI, MLS, TOCTOU, SELinux, MCS, umrs-selinux, Bell-LaPadula, etc.
- **Reject list:** Terms we have explicitly banned — utilize, leverage, facilitate, security label,
  compartments, MAC policy, plus informal admonition labels

These lists are generated from `approved_terminology.md` and `ste_mode.md`, keeping Vale
in sync with the authoritative sources.

### Integration model

Vale supports pre-commit hooks, CI/CD, and editor plugins (VS Code, Neovim, Zed). Our
plan starts in report-only mode (no blocking), with a path to soft-gate and eventually
hard-gate enforcement as the ruleset matures. The `--no-exit` flag allows pipeline
continuation during the report phase.

Vale's JSON output mode feeds directly into our `history.json` trend tracking, enabling
the quality score metrics and trend comparisons the plan already defines.

### Review routing integration

Quality reports and violation reviews are stored in the new review routing structure:

- Documentation quality reports → `docs/imprimatur/reviews/YYYY-MM-DD-quality-report.md`
- Blog/outreach quality reports → `docs/sage/reviews/YYYY-MM-DD-quality-report.md`
- Violation review decisions remain in `.claude/metrics/doc-quality/` (operational data)

### Future expansion (not in current scope)

- **`UMRS-SrcComments` style** — lint Rust `///` and `//!` doc comments for module doc
  checklist compliance, tiered annotation expectations, and internal reference prohibition.
  Requires custom scoping work for `.rs` files. Decision deferred.
- **`script` extension** — Vale supports custom Tengo scripts for validation logic that
  exceeds what YAML rules can express. Potential use: verifying that every `## Compliance`
  section contains at least one canonical citation.

The system starts in **report-only mode**. Gate enforcement is configurable once the
ruleset matures through reviewed violation cycles. Every violation review is an
opportunity to grow the ruleset organically from real findings.

**Primary executor:** senior-tech-writer
**Supporting agents:** tech-writer (applies fixes), researcher (acquires Vale style guides
if needed), rust-developer (installs tooling)

---

## Background

UMRS documentation covers complex security domain content — MLS lattice mathematics,
SELinux policy enforcement, FIPS cryptographic boundaries, kernel-level trust enforcement.
This complexity makes documentation quality both harder to achieve and more critical to
maintain. An operator reading a poorly written procedure under pressure is a safety risk.

The project already has two well-developed quality foundations:

- `ste_mode.md` — STE rules for procedural content, admonition hierarchy, approved verbs
- `approved_terminology.md` — canonical terms, forbidden variants, abbreviation rules

This plan operationalizes those foundations into an automated quality pipeline.

---

## Scope

### In scope

- All `.adoc` files under `docs/modules/`
- Vale ruleset generated from existing project terminology and STE rules
- Readability scoring per content type
- Violation review workflow with ruleset refinement loop
- Quality history and trend tracking
- Configurable gate mode (report | soft | hard)

### Out of scope

- Source code comments (separate concern, governed by `rules/assurance_rules.md`; see Future Expansion in Vale assessment)
- `docs/sage/inbox/` and `docs/imprimatur/inbox/` scratch content (not yet promoted)
- `.claude/references/` documents (third-party, not authored by UMRS)
- French translations (umrs-translator scope; separate quality pass if desired later)

---

## Tooling Stack

| Tool | Purpose | Scope |
|---|---|---|
| Vale | Terminology enforcement, vocabulary, abbreviation rules, admonition structure | All `.adoc` files |
| write-good | Passive voice detection, sentence length, weasel words | Procedural content (`.adoc` procedure blocks) |
| textstat (Python) | Flesch Reading Ease, Flesch-Kincaid Grade, Gunning Fog Index | All prose sections |
| Custom Python runner | Orchestrates all three tools, classifies content type, produces unified report | Pipeline driver |

### Installation

| Tool | Root needed? | Install method |
|---|---|---|
| Vale | No | `curl -sfL https://install.Vale.sh \| sh -s -- --dir ~/.local/bin` |
| write-good | Yes (global npm) | `sudo npm install -g write-good` |
| textstat | Yes (system pip) | `sudo pip install textstat --break-system-packages` |
| PyYAML (for gen script) | Yes (system pip) | `sudo pip install pyyaml --break-system-packages` |

Vale installs as a standalone binary to `~/.local/bin` — no root, no package manager.
The other tools require root for global installation.

Verify availability before first run:

```bash
vale --version
write-good --version
python3 -c "import textstat; print('textstat ok')"
```

---

## Vale Ruleset Structure

Vale rules are generated directly from existing project knowledge. Nothing is invented
from scratch — every rule traces back to `ste_mode.md` or `approved_terminology.md`.

```
.vale/
  .vale.ini                    ← Vale configuration
  styles/
    UMRS/
      Terminology.yml          ← Generated from approved_terminology.md
      ForbiddenVerbs.yml       ← Generated from ste_mode.md "Avoid" list
      AmbiguousWords.yml       ← Generated from ste_mode.md ambiguous words list
      Abbreviations.yml        ← First-use and format rules from approved_terminology.md
      Admonitions.yml          ← Admonition hierarchy enforcement
      PassiveVoice.yml         ← Procedural content only
      SentenceLength.yml       ← Procedural content only (20-word max)
    write-good/                ← Vale-compatible write-good integration
  vocab/
    UMRS/
      accept.txt               ← Approved terms (not flagged as spelling errors)
      reject.txt               ← Forbidden variants
```

### `.vale.ini`

```ini
StylesPath = .vale/styles
MinAlertLevel = warning

[*.adoc]
BasedOnStyles = UMRS

# Procedural content rules — applied to numbered list blocks only
# Vale does not natively scope to numbered lists; runner pre-processes
# procedure blocks into temporary files for STE rule application
```

### `Terminology.yml` — Generated from `approved_terminology.md`

```yaml
# Auto-generated from .claude/agent-memory/doc-team/approved_terminology.md
# Regenerate with: python3 .claude/scripts/gen-vale-terminology.py
extends: substitution
message: "Use '%s' instead of '%s'. See approved_terminology.md."
level: warning
ignorecase: true
swap:
  'security label': 'security context'
  'selinux context': 'security context'
  'context(?!\s+of\s+a)': 'security context'  # avoid over-matching
  'sensitivity label': 'sensitivity level'
  'clearance level': 'sensitivity level'
  'classification level': 'sensitivity level'
  'compartments': 'category set'
  'label range': 'MLS range'
  'MLS level': 'MLS range'
  'security monitor': 'reference monitor'
  'policy monitor': 'reference monitor'
  'log entry': 'audit event'
  'log record': 'audit event'
  'MAC policy': 'mandatory access control'
  'enforced policy': 'mandatory access control'
  'permission denied': 'access denied'
  'access blocked': 'access denied'
  'driver': 'kernel module'
  'kernel extension': 'kernel module'
  'CUI data': 'CUI'
  'sensitive data': 'CUI'
  'HA(?!-Sign)': 'high-assurance'
```

### `ForbiddenVerbs.yml` — From `ste_mode.md`

```yaml
extends: existence
message: "Avoid '%s' in procedural content. Use an approved technical verb."
level: warning
tokens:
  - facilitate
  - leverage
  - 'perform(?!\s+a\s+\w+\s+operation)'
  - utilize
```

### `AmbiguousWords.yml` — From `ste_mode.md`

```yaml
extends: existence
message: "Avoid ambiguous word '%s'. Write a definite statement."
level: warning
tokens:
  - appropriate
  - '\bmay\b'
  - '\bmight\b'
  - proper
  - several
  - various
```

### `Admonitions.yml` — From `ste_mode.md` admonition hierarchy

```yaml
extends: existence
message: "Use AsciiDoc block admonition syntax, not inline labels. See ste_mode.md admonition hierarchy."
level: error
tokens:
  - '^Note:'
  - '^Warning:'
  - '^IMPORTANT —'
  - '^Caution:'
  - '^TIP —'
```

---

## Content Type Classification

The quality runner classifies each `.adoc` file before applying rules. This ensures
STE sentence-length and passive-voice rules apply only to procedural content.

| Content type | Detection heuristic | Rules applied |
|---|---|---|
| `procedure` | File path contains `/deployment/`, `/operations/`, or page contains 3+ numbered list blocks | All rules including STE sentence length and passive voice |
| `architecture` | File path contains `/architecture/` or `/patterns/` | Terminology + abbreviations + admonitions; readability flagged but not enforced |
| `reference` | File path contains `/reference/` or `/cryptography/` | Terminology + abbreviations + admonitions |
| `developer` | File path contains `/devel/` | All rules; sentence length target relaxed to 30 words |
| `glossary` | File path contains `/glossary/` | Terminology only |

Content type is recorded in every quality report entry.

---

## Quality Metrics and Targets

### Readability Targets by Content Type

Initial targets are **aspirational**, not enforced, until the baseline run establishes
realistic starting points.

| Content type | Flesch Reading Ease | Flesch-Kincaid Grade | Gunning Fog |
|---|---|---|---|
| Procedures | ≥ 60 | ≤ 10 | ≤ 10 |
| Developer guides | ≥ 40 | ≤ 12 | ≤ 14 |
| Architecture / patterns | ≥ 30 | ≤ 14 | ≤ 16 |
| Reference | ≥ 35 | ≤ 13 | ≤ 15 |
| Glossary | ≥ 50 | ≤ 11 | ≤ 12 |

NOTE: A Fog Index of 14–16 on architecture content covering Bell-LaPadula dominance
math or MLS lattice theory is domain-appropriate and should not be treated as a failure.
The targets flag outliers for review — they do not mechanically reject complex content.

---

## Quality History Structure

```
.claude/metrics/
  doc-quality/
    baseline.md              ← First run — the anchor for all trend comparisons
    history.json             ← Machine-readable trend data (all runs)
    YYYY-MM-DD-report.md     ← Human-readable report per run
    YYYY-MM-DD-review.md     ← Violation review decisions + ruleset changes
  config/
    quality-gate.md          ← Current gate mode and thresholds
    thresholds.md            ← Per-content-type readability targets
```

### `history.json` Schema

```json
{
  "runs": [
    {
      "date": "YYYY-MM-DD",
      "gate_mode": "report",
      "pages_scanned": 142,
      "vale_violations": {
        "total": 23,
        "terminology": 8,
        "admonitions": 3,
        "forbidden_verbs": 4,
        "ambiguous_words": 8
      },
      "readability": {
        "procedure": { "flesch": 62, "kincaid": 9.8, "fog": 10.1 },
        "architecture": { "flesch": 38, "kincaid": 13.2, "fog": 15.4 },
        "developer": { "flesch": 44, "kincaid": 11.9, "fog": 13.7 }
      },
      "quality_score": 87,
      "notes": "Baseline run — ruleset v0.1"
    }
  ]
}
```

### `quality-gate.md` — Gate Configuration

```markdown
# Documentation Quality Gate Configuration

## Current mode: report

## Available modes
- `report`  — scan and report; no blocking; all violations logged for review
- `soft`    — scan and report; agent flags violations to Jamie; task marked incomplete
- `hard`    — scan and report; task cannot complete if violations exceed thresholds

## Threshold for soft/hard mode (not active until mode changes)
Vale errors (not warnings): 0
Vale warnings: ≤ 5 per page
Readability Fog Index (procedures): ≤ 12
Quality score: ≥ 85

## Mode change protocol
Jamie approves mode change after reviewing two consecutive runs
where violations are predominantly dismissed (not added to ruleset).
That indicates the ruleset has stabilized and enforcement is appropriate.
```

---

## Violation Review Workflow

This is the ruleset refinement loop. Every report run produces a review document
where each violation category is evaluated and a decision is recorded.

### Review Document Format

`.claude/metrics/doc-quality/YYYY-MM-DD-review.md`

```markdown
# Violation Review — YYYY-MM-DD

## Summary
Run date: YYYY-MM-DD
Pages scanned: N
Total violations: N
Decisions made: N added | N dismissed | N deferred | N pending

## Violation Decisions

| # | Rule | Violation type | Example location | Example text | Decision | Action |
|---|---|---|---|---|---|---|
| 1 | Terminology | "security label" used | crypto-post-quantum.adoc:47 | "the security label" | add-to-ruleset | Terminology.yml updated |
| 2 | SentenceLength | 28 words in step | deploy.adoc:step 4 | "Before running..." | deferred | Rewrite task created |
| 3 | Fog Index 15.2 | Architecture page | selinux-enforcement.adoc | full page | dismissed | Domain-appropriate complexity |
| 4 | ForbiddenVerb | "utilize" | devel/patterns.adoc:12 | "utilize the pattern" | add-to-ruleset | Already in ruleset — false positive |

## Decision Key
- `add-to-ruleset` — violation confirmed; rule added or strengthened
- `dismissed` — not a real violation for this content type or context
- `deferred` — real violation; fix task created; not yet resolved
- `pending` — needs Jamie decision before action
- `false-positive` — rule fired incorrectly; rule needs refinement

## Ruleset Changes This Review
- Terminology.yml: added "security label" → "security context" swap (was missing variant)
- AmbiguousWords.yml: added "some" to token list (new finding)

## Fix Tasks Created
- [ ] Rewrite step 4 in deploy.adoc — sentence too long (28 words)
- [ ] Replace "utilize" in devel/patterns.adoc:12
- [ ] First-use abbreviation missing for "IMA" in operations/ima-setup.adoc
```

### Decision Rules

| Finding | Default decision |
|---|---|
| Terminology violation confirmed against `approved_terminology.md` | `add-to-ruleset` |
| STE violation in a procedure block | `add-to-ruleset` if clear; `deferred` if rewrite complex |
| Readability score on architecture/patterns content | `dismissed` unless extreme outlier (Fog > 18) |
| Readability score on procedure content | `deferred` with rewrite task |
| Admonition format violation | `add-to-ruleset` — no exceptions |
| False positive (rule too broad) | `false-positive` — refine the rule |

Jamie reviews `pending` items only. All other decisions are senior-tech-writer authority.

---

## Implementation Phases

### Phase 1 — Tooling and Ruleset Bootstrap

**Executor:** rust-developer installs tools; senior-tech-writer generates Vale rules

1. Install Vale, write-good, textstat on RHEL10 VM
2. Create `.vale/` directory structure
3. Generate `Terminology.yml` from `approved_terminology.md` using generation script
4. Generate `ForbiddenVerbs.yml` and `AmbiguousWords.yml` from `ste_mode.md`
5. Write `Admonitions.yml` manually (small, precise ruleset)
6. Create `SentenceLength.yml` scoped to procedure content
7. Create `.claude/metrics/` directory structure
8. Write `quality-gate.md` in report-only mode
9. Write `thresholds.md` with initial aspirational targets

**Output:** Functional Vale installation, ruleset v0.1, metrics directory ready

### Phase 2 — Baseline Run

**Executor:** senior-tech-writer

1. Run full quality scan across all `docs/modules/**/*.adoc`
2. Produce `baseline.md` — this is the anchor for all future trend comparisons
3. Produce `history.json` with first entry
4. Produce first `YYYY-MM-DD-report.md`
5. Do NOT fix anything yet — baseline must reflect current state

**Output:** Baseline established; first quality score recorded

### Phase 3 — First Violation Review

**Executor:** senior-tech-writer with Jamie review of `pending` items

1. Produce `YYYY-MM-DD-review.md` from baseline violations
2. For each violation category: decide add-to-ruleset | dismissed | deferred | pending
3. Update Vale ruleset for all `add-to-ruleset` decisions
4. Create fix task list for all `deferred` items
5. Update `quality-gate.md` with any threshold adjustments from review findings
6. Re-run scan with updated ruleset — record as run #2 in `history.json`

**Output:** Ruleset v0.2; first fix task list; first trend data point

### Phase 4 — Fix Cycle and Trend Tracking

**Executor:** tech-writer (fixes); senior-tech-writer (re-scan and review)

1. tech-writer works through deferred fix task list
2. senior-tech-writer re-scans after each batch of fixes
3. Each scan appends to `history.json`
4. After two clean review cycles (violations predominantly dismissed),
   propose mode change to `soft` gate for Jamie approval

**Output:** Improving trend line; ruleset stabilizing; gate mode upgrade candidate

### Phase 5 — Gate Mode Upgrade (when ready)

**Executor:** senior-tech-writer proposes; Jamie approves

1. Present trend data showing ruleset stability
2. Jamie approves upgrade to `soft` mode
3. Update `quality-gate.md`
4. Agents begin flagging gate failures on task completion
5. Repeat review cycle; upgrade to `hard` mode when `soft` proves stable

---

## Vale Rule Generation Script

The generation script ensures Vale rules stay synchronized with
`approved_terminology.md` as the terminology list grows.

**Path:** `.claude/scripts/gen-vale-terminology.py`

```python
#!/usr/bin/env python3
"""
Generate Vale Terminology.yml from approved_terminology.md.
Run after any update to approved_terminology.md.

Usage: python3 .claude/scripts/gen-vale-terminology.py
Output: .vale/styles/UMRS/Terminology.yml
"""
import re
import yaml
from pathlib import Path

TERMINOLOGY_FILE = Path(".claude/agent-memory/doc-team/approved_terminology.md")
OUTPUT_FILE = Path(".vale/styles/UMRS/Terminology.yml")

def parse_terminology(path: Path) -> dict:
    swaps = {}
    in_table = False
    for line in path.read_text().splitlines():
        if line.startswith("| Use |"):
            in_table = True
            continue
        if in_table and line.startswith("|---"):
            continue
        if in_table and line.startswith("|"):
            cols = [c.strip().strip("`") for c in line.split("|")[1:-1]]
            if len(cols) >= 2 and cols[0] and cols[1]:
                preferred = cols[0]
                forbidden = cols[1]
                if forbidden and forbidden != "—" and forbidden != "(no variation)":
                    for variant in forbidden.split(","):
                        variant = variant.strip().strip("`")
                        if variant:
                            swaps[variant] = preferred
        elif in_table and not line.startswith("|"):
            in_table = False
    return swaps

def generate_vale_rule(swaps: dict) -> dict:
    return {
        "extends": "substitution",
        "message": "Use '%s' instead of '%s'. See approved_terminology.md.",
        "level": "warning",
        "ignorecase": True,
        "swap": swaps,
    }

if __name__ == "__main__":
    swaps = parse_terminology(TERMINOLOGY_FILE)
    rule = generate_vale_rule(swaps)
    OUTPUT_FILE.parent.mkdir(parents=True, exist_ok=True)
    with open(OUTPUT_FILE, "w") as f:
        yaml.dump(rule, f, default_flow_style=False, allow_unicode=True)
    print(f"Generated {OUTPUT_FILE} with {len(swaps)} substitution rules.")
```

Run after any update to `approved_terminology.md`:

```bash
cd /path/to/umrs-project
python3 .claude/scripts/gen-vale-terminology.py
```

---

## Quality Report Format

`.claude/metrics/doc-quality/YYYY-MM-DD-report.md`

```markdown
# Documentation Quality Report — YYYY-MM-DD

**Ruleset version:** v0.N
**Gate mode:** report
**Pages scanned:** N
**Run type:** scheduled | triggered | baseline

---

## Summary Scores

| Content type | Pages | Flesch RE | FK Grade | Fog Index | Vale violations | Score |
|---|---|---|---|---|---|---|
| Procedures | N | N | N | N | N | N/100 |
| Architecture | N | N | N | N | N | N/100 |
| Developer | N | N | N | N | N | N/100 |
| Reference | N | N | N | N | N | N/100 |
| **Overall** | **N** | **N** | **N** | **N** | **N** | **N/100** |

---

## Vale Violations

### Terminology (N violations)
| File | Line | Found | Expected |
|---|---|---|---|

### Admonitions (N violations)
| File | Line | Issue |
|---|---|---|

### Forbidden Verbs (N violations)
| File | Line | Found |
|---|---|---|

### Ambiguous Words (N violations)
| File | Line | Found |
|---|---|---|

---

## Readability Outliers

Pages outside target range for their content type:

| File | Content type | Fog Index | Target | Delta |
|---|---|---|---|---|

---

## Trend vs. Previous Run

| Metric | Previous | This run | Delta |
|---|---|---|---|
| Total violations | N | N | ±N |
| Overall score | N | N | ±N |
| Procedure Fog Index | N | N | ±N |

---

## Recommended Actions

1. [Auto-generated list of top violation categories]
2. [Readability outliers requiring review]
3. [Any items requiring Jamie decision]

---

*Next scheduled run: YYYY-MM-DD*
*Review document: YYYY-MM-DD-review.md (to be produced)*
```

---

## Senior-Tech-Writer Agent Updates

Add to `agents/senior-tech-writer.md`:

```markdown
## Documentation Quality Responsibilities

The senior-tech-writer is the documentation quality authority. This includes:

- Running the quality scan pipeline after significant documentation additions
- Producing violation review documents for each scan
- Making add-to-ruleset | dismissed | deferred | pending decisions
- Escalating `pending` items to Jamie
- Maintaining `.vale/styles/UMRS/` ruleset files
- Regenerating `Terminology.yml` after any update to `approved_terminology.md`
- Tracking quality history in `.claude/metrics/doc-quality/`
- Proposing gate mode upgrades when trend data supports it

Quality scan triggers:
- After any documentation phase completes (plan-driven work)
- After 10 or more `.adoc` files are modified in a session
- When Jamie says "run a quality scan"
- On a scheduled basis (every 2 weeks or at session start if overdue)
```

---

## Hard Constraints

- Never modify `approved_terminology.md` or `ste_mode.md` without Jamie approval —
  these are the authoritative sources; the Vale ruleset is derived from them, not the reverse
- Never upgrade gate mode without Jamie approval
- Baseline run must complete before any fixes are applied — the baseline is the anchor
- Fix tasks created during review must be tracked in the review document until resolved
- `history.json` is append-only — never modify past entries
- Never touch `.claude/references/`, `knowledge/`, or source code under `components/`
- Never git commit or push

---

## Success Criteria

When this plan is complete:

1. Vale, write-good, and textstat are installed and functional on the RHEL10 VM
2. Vale ruleset v0.1 is generated from existing project terminology and STE rules
3. Baseline quality scores are established across all content types
4. First violation review is complete with ruleset v0.2
5. `history.json` has at least two data points showing trend direction
6. senior-tech-writer can run a quality scan autonomously and produce a report
7. The ruleset refinement loop is functioning: violations → review → ruleset growth
8. Gate mode is configurable and documented; upgrade path to soft/hard is defined
