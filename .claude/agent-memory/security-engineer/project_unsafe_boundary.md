---
name: unsafe_isolation_boundary
description: How this project handles the unsafe_code forbid constraint when unsafe is genuinely required
type: project
---

`#![forbid(unsafe_code)]` is set in every crate root and CANNOT be overridden by `#[allow]`.
This is a compile-time proof, not a policy — the Rustonomicon confirms `forbid` beats `allow`.

**Why:** NIST SP 800-218 SSDF PW.4 / NSA RTB RAIN. An auditor can verify mechanically.

The established pattern for isolating genuinely-unsafe code is the `umrs-hw` crate:
- `umrs-hw` is the workspace's unsafe isolation boundary — no forbid, just deny, with targeted allows
- It has no workspace dependencies, minimizing its attack surface
- All other crates depend on its safe public API, never on the unsafe internals

**How to apply:** When a new feature requires `unsafe` (e.g., `std::env::set_var`/`remove_var`
for environment scrubbing, RDTSCP for timestamps), create a new isolated crate rather than
adding `#[allow(unsafe_code)]` to an existing `#![forbid]` crate. This is the same isolation
model as `umrs-hw`. When reviewing plans that propose `#[allow(unsafe_code)]` overrides inside a
`#![forbid]` crate, flag this as HIGH — it cannot compile.

The `umrs-tool-init` environment scrubbing plan (2026-03-17) requires this decision:
Option A = new `umrs-init` crate (recommended, mirrors umrs-hw precedent).
Option B = contract-documented single-threaded precondition with best-effort enforcement.
Jamie's decision pending.
