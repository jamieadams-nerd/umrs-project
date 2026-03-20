# Guest-Coder Exercise 2 — CUI Cross-Label Access Check
## Date: 2026-03-20
## Task: cui_access_check.rs — simulated process clearance vs file labels

---

## Executive Summary

Example compiles, runs, and correctly demonstrates the Phase 1 vs Phase 2 teaching moment. The intern successfully navigated the umrs-selinux API chain (`SecureDirent` → `SelinuxCtxState` → `SecurityContext` → `MlsLevel` → `CategorySet` → `dominates()`). Key finding: `SecurityContext::dominates()` is a `todo!()` stub — the intern had to use `CategorySet::dominates()` directly.

---

## What the Example Does

1. Sets up test files with CUI labels via `chcon`
2. Reads each file's security context via `SecureDirent::from_path()`
3. Builds a simulated process clearance (`CategorySet` with only c10/PROCURE)
4. Checks `CategorySet::dominates()` for each file
5. Prints AUTHORIZED or DENIED with reason
6. Attempts to READ a DENIED file — and succeeds (the teaching moment)

---

## The Key Finding — Phase 1 vs Phase 2

The program says DENIED for `specs.txt` (CUI//CTI/EXPT), but `cat specs.txt` succeeds. The intern documented their understanding:

In targeted policy with `unconfined_t`, MCS categories are metadata — the kernel does not enforce them on unconfined processes. The `dominates()` check is awareness, not enforcement. In MLS policy (Phase 2), the kernel would return EACCES.

**This is the single most important lesson about UMRS Phase 1.**

---

## API Findings

### HIGH

**H1: `SecurityContext::dominates()` is a `todo!()` stub**
The natural API path (`security_context.dominates(&other)`) panics at runtime. The intern had to discover that `CategorySet::dominates()` works at the lower level. This needs either implementation or prominent documentation.

### MEDIUM

**M1: The API chain is long but navigable**
`SecureDirent` → `SelinuxCtxState` → `SecurityContext` → `MlsLevel` → `CategorySet` → `dominates()` — 5 hops. The intern found the path but noted it would benefit from a convenience method or a documented "recipe."

**M2: MlsLevel confusion**
There's a `context::MlsLevel` and `mls::level::MlsLevel` — the docs warn about this but it still caused confusion.

**M3: No `getcon()` equivalent**
Cannot read the calling process's own security context. Had to simulate a clearance by constructing a `CategorySet` manually.

---

## Documentation Quality Ratings

| Dimension | Score |
|---|---|
| Discoverability | 6/10 |
| Clarity | 7/10 |
| Completeness | 6/10 |
| Examples | 7/10 |

---

## Files Produced

- `components/rusty-gadgets/umrs-selinux/examples/cui_access_check.rs` — compiles, clippy clean, runs correctly
