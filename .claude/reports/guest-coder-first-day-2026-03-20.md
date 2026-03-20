# Guest-Coder First Day Report — Pass 1 (Cold)
## Date: 2026-03-20
## Task: Write `system_summary.rs` using only public docs and rustdoc

---

## Executive Summary

The intern wrote a working, compiling example that covers 2 of 4 tasks cleanly (OS detection, package check) and documents the friction points for the other 2 (SELinux file context, IMA/EVM status). The example is well-structured, honors NO_COLOR, and includes compliance annotations. The friction points are precisely documented with `// STUCK:` comments.

**Result:** Example compiles, runs, produces real output on CentOS Stream 10. Shows T3 confidence, 22/34 indicators hardened, 6 critical-impact findings.

---

## Findings

### HIGH — API gaps that blocked the task

**H1: KernelRelease inaccessible via public API**
`KernelRelease` is exported and documented but not reachable through `DetectionResult`. The intern expected `result.kernel_release` or similar. The kernel version string is locked behind the detection internals with no public accessor.

**H2: No cross-crate pointer from umrs-platform to umrs-selinux**
The SELinux file context task was a dead end. `umrs-platform` rustdoc never mentions `umrs-selinux`. An outside developer has no way to discover that security contexts are in a different crate without reading source code or asking someone.

**H3: No IMA/EVM variant in IndicatorId**
The intern scanned all 36 `IndicatorId` variants and found no IMA or EVM indicator. Notable gap for a posture tool that aims to cover kernel security state.

### MEDIUM — API ergonomics friction

**M1: No `Display` impl on `TrustLevel`, `OsName`, `OsId`, `VersionId`**
The intern had to write manual `match` arms to format these types. Every consumer will write the same boilerplate.

**M2: `DetectionError` — no `Display` impl, struct variant shape unclear**
`PidCoherenceFailed` is a struct variant whose fields aren't previewed in the module doc. Caused a compile error that required guessing the field names.

**M3: `IndicatorDescriptor` has no short `label` field**
Only `rationale` (a full sentence) is available. For CLI/TUI column headers, consumers need a short label but must truncate the rationale themselves.

**M4: `is_installed` returns bare `bool`**
No distinction between "package absent" and "RPM database unreadable." Consumer can't tell the difference between "not installed" and "query failed."

---

## What Worked Well

- `OsDetector::detect()` — simple, intuitive, worked first try
- `PostureSnapshot::collect()` — clean one-liner that returns structured data
- `is_installed()` — easy to find and call (aside from the bool-only return)
- The existing `posture_demo` example provided a useful pattern to follow
- rustdoc for the posture module was thorough enough to navigate IndicatorId variants

## What Was Missing

- **Getting Started page** — no "start here" entry point in the developer guide
- **Cross-crate discovery** — no way to know umrs-selinux exists from umrs-platform docs
- **Display impls** — basic formatting for security types should be in the library, not consumer code
- **Error type documentation** — variant shapes and fields need to be visible in rustdoc summaries

---

## Documentation Ratings (1-10)

| Dimension | Score | Notes |
|---|---|---|
| Discoverability | 4 | No getting started page, no cross-crate pointers |
| Clarity | 6 | What exists is clear, but there are significant gaps |
| Completeness | 5 | OS detection well-covered; SELinux/IMA invisible from platform crate |
| Examples | 5 | posture_demo exists and helps; need more task-oriented examples |

---

## Implications for Tutorial (Pass 2)

The tutorial MUST address:
1. Which crate to use for which task (crate selection guide)
2. The `OsDetector::detect()` → query pattern (this works great, lead with it)
3. Cross-crate workflow: platform for posture, selinux for file contexts
4. How to format/display the typed results (or add Display impls first)

---

## Files Produced

- `components/rusty-gadgets/umrs-platform/examples/system_summary.rs` — working example with STUCK comments
