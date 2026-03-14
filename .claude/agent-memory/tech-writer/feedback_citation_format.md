---
name: NIST citation format standard
description: Approved citation format for NIST controls in umrs-platform source and documentation
type: project
---

The correct abbreviated citation for NIST controls is **`NIST SP 800-53`** (with `SP` for Special Publication).

Approximately half the `umrs-platform` source files use the bare form `NIST 800-53` (missing `SP`). The correct form is established by `evidence.rs`, `confidence.rs`, `sealed_cache.rs`, and all `detect/` phase modules.

**Why:** Consistency across all documentation and source comments. The `SP` designator is the correct abbreviated form of "Special Publication".

**How to apply:** When writing or reviewing any compliance annotation — whether in source `//!` doc comments, Antora pages, or reports — always use:
- `NIST SP 800-53 SI-7` (not `NIST 800-53 SI-7`)
- `NIST SP 800-218` (not `NIST 800-218`)
- `NIST SP 800-53r5` when a revision-specific citation is needed

Other frameworks have their own abbreviations and do not need `SP`:
- NSA RTB RAIN, NSA RTB VNSSA (no prefix needed)
- CMMC L2 (no change needed)
- FIPS 140-2/140-3 (no change needed)

A sweep edit across ~30 occurrences in `umrs-platform/src/` was identified as needed in the 2026-03-14 module comment review.
