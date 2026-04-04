# Plan Status Audit — 2026-04-04

**Auditor:** rust-developer (automated codebase verification)
**Scope:** All plan files in `.claude/plans/` excluding `completed/`, `long-term/`, `ARCHITECTURE.md`, and `ROADMAP.md`

---

## Summary

| Status | Count |
|---|---|
| Status is accurate | 14 |
| Status is stale (recommend update) | 7 |
| Missing status line | 1 |
| Recommend move to `completed/` | 2 |

---

## Detailed Findings

| Filename | Current Status | Recommended Status | Evidence |
|---|---|---|---|
| `antora-multi-component-split.md` | Approved — execute after antora-doc-theme is complete | **Accurate** — no change needed | Docs still use monolithic `docs/` structure; no `umrs-home/`, `umrs-project/`, etc. directories exist. Work has not started. |
| `c2pa-vault-prototype.md` | completed (spike -> full tool delivery, 2026-04-02) | **Move to `completed/`** | `umrs-c2pa/` crate exists with tests, the plan itself documents full delivery. Should be archived. |
| `ca7-monitoring-frequency.md` | draft — awaiting Jamie review | **Accurate** — no change needed | This is a policy/document plan, not code. No assessment engine exists yet. Status is correct. |
| `cpu-extension-probe.md` | backburner — next release, not Cantrip (2026-03-23) | **Accurate** — no change needed | No `CpuIndicatorId` enum in codebase. Plan explicitly says "DO NOT CHANGE ANY CODE Right Now". Backburner is correct. |
| `doc-quality-plan.md` | On hold (back burner per Jamie 2026-03-20) | **Accurate** — no change needed | No `.vale/` directory exists. Tooling not installed. On hold is correct. |
| `enhanced-security-testing.md` | stub | **Accurate** — no change needed | No `proptest` in any `Cargo.toml`. No `fuzz/` directories. Stub is correct. |
| `fr-ca-corpus-acquisition.md` | Tier 1 complete; Tier 2 backburner (2026-03-23) | **Accurate** — no change needed | Status includes nuanced tier breakdown. No code deliverables to verify. |
| `high-assurance-writing-guide.md` | Back burner — deferred per Jamie 2026-03-31 | **Accurate** — no change needed | No guide exists in `docs/modules/devel/pages/`. Deferred is correct. |
| `m35-deployment-security.md` | draft — awaiting Jamie decisions (install path, CUI data path) | **Accurate** — no change needed | No process domain policy (`umrs_tools.te`), no RPM spec, no fapolicyd rules. Draft is correct. |
| `m3-translation-prep.md` | in-progress — umrs-c2pa wrapping active (2026-04-02) | **Accurate** — no change needed | i18n pipeline is actively being extended. Status reflects current work. |
| `openssl-posture-module.md` | draft | **Accurate** — no change needed | No `OpenSslPosture` type in codebase. Plan says "DO NOT CHANGE ANY CODE Right Now". Draft is correct. |
| `os-detect-kernel-tab-enhancement.md` | Phases 0-4 COMPLETE. Phase 5 on hold. Phase 6 COMPLETE (2026-03-21, release-ready). | **Accurate** — no change needed | Dialog API exists in `umrs-ui`. Value translations and indicator descriptions are implemented. Phase 5 (operations reference guide) is documentation work, correctly on hold. |
| `performance-baseline.md` | Approved — ready for implementation | **STALE — recommend: completed** | All 3 crates have criterion benchmarks (`benches/` dirs with `*_bench.rs` files), criterion in `[dev-dependencies]`, debug instrumentation (`log::debug!` in detect pipeline), `phase_durations`/`duration_ns` infrastructure, `with_capacity` patterns applied, and baseline reports exist at `.claude/reports/perf-baseline-umrs-{platform,selinux,core}.md`. All 6 steps of the process are done for all 3 crates. **Move to `completed/`.** |
| `platform-api-enrichment.md` | draft — awaiting Jamie review | **Partially stale** — recommend: draft (partial progress) | `SignalDescriptor` already has a `description: &'static str` field in `posture/catalog.rs` (Gap 1 filled). However, no `SignalGroup` enum exists, and TUI label helpers have not been moved to platform. Phase 1 is partially done; Phases 2-5 are not started. Status should note partial progress. |
| `pre-release-annotation-audit.md` | draft — Herb to author full scope document | **Accurate** — no change needed | This is an audit planning document. No audit reports for this specific scope exist yet. |
| `qotd-quotes-corpus.md` | Approved — fun future work | **Accurate** — no change needed | No `quotes/` module in `umrs-core`. Correctly deferred as fun future work. |
| `researcher-enhancements.md` | on-hold — Jamie deferred 2026-03-23 | **Accurate** — no change needed | No `knowledge-gaps.md`, no new skills created. On-hold is correct. |
| `research-pipeline-priorities.md` | in-progress (detailed per-priority status in header) | **Accurate** — no change needed | Status line includes granular per-priority tracking. Reflects ongoing research work. |
| `sage-outreach-and-release-strategy.md` | draft | **Accurate** — no change needed | No CI/CD pipeline, no analytics, no landing page redesign. Draft is correct. |
| `tui-enhancement-plan.md` | Phases 3, 5, 6, 7, 8 COMPLETE (2026-03-15). Phases 1, 2, 4 not started. | **Accurate** — no change needed | `umrs-ui` crate exists with dialog API, viewer, config modules. Remaining phases (1: card audit template, 2: multi-card navigation, 4: export) are correctly listed as not started. |
| `umrs-plan-original.md` | **No status line** | **Add status: superseded** | This is the original milestone brainstorm. It has no `**Status:**` header. It has been superseded by the ROADMAP and individual plans. Should have a status line per the Plan Status Header Rule. |
| `umrs-platform-posture-and-cross-platform.md` | active umbrella — subsidiary plans in progress | **Accurate** — no change needed | Posture module exists (`posture/` directory with catalog, contradiction, snapshot, etc.). OS detection is complete. CPU extension probe is correctly split out. Active umbrella status is appropriate. |
| `umrs-tool-init.md` | Approved — ready for implementation | **STALE — recommend: approved (not started)** | No `init/` module in `umrs-core/src/`, no `umrs-env` binary crate, no `EnvValidationError` type, no `ScrubReport` type. The plan is approved but zero implementation has occurred. Status should clarify that no work has started. |
| `xattr-sanitization-gap.md` | Draft — awaiting activation | **Accurate** — no change needed | No xattr sanitization tool exists. Draft/awaiting activation is correct. |

---

## Action Items

### Move to `completed/`

1. **`c2pa-vault-prototype.md`** — Fully delivered 2026-04-02. Crate exists, tests pass, docs written, reviews done. The plan itself already says "completed" but the file still sits in the active plans directory.

2. **`performance-baseline.md`** — All 6 steps complete for all 3 crates (umrs-platform, umrs-selinux, umrs-core). Criterion benchmarks installed, baselines recorded, debug instrumentation added, optimization opportunities analyzed and implemented. Baseline reports exist at `.claude/reports/perf-baseline-umrs-*.md`.

### Update status line

3. **`umrs-tool-init.md`** — Change from "Approved — ready for implementation" to "Approved — not started" to distinguish from plans where work has begun.

4. **`platform-api-enrichment.md`** — Change from "draft — awaiting Jamie review" to "draft — partial progress (Gap 1 description field implemented; Phases 1-5 not complete)" since `SignalDescriptor.description` already exists in the codebase.

5. **`umrs-plan-original.md`** — Add a `**Status:** superseded` line. This file has no status header, violating the Plan Status Header Rule.

### No action needed

The remaining 19 plans have accurate status lines that reflect the current state of the codebase and project.
