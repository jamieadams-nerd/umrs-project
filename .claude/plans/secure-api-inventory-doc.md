# Secure API Inventory Documentation

**Status:** in-progress — file-operations page drafted 2026-04-20; remaining groups pending.
**Date:** 2026-04-20
**Author:** Jamie + Claude
**ROADMAP Goals:** G7 (Public Project), G4 (Assessment Engine)
**Milestones:** M4 (Public Release)

---

## Purpose

Produce a developer-facing reference catalog under `docs/modules/devel/pages/` that contrasts every **traditional (std / POSIX / third-party)** method of performing a task with the **UMRS high-assurance equivalent**. Each row records whether the secure call emits an `EvidenceBundle` / `EvidenceRecord` and its trust-tier relationship to the platform `TrustLevel` (T0–T4).

The goal is blunt: **stop duplicating high-assurance building blocks.** The catalog surfaces existing APIs so agents and humans reach for them first, and exposes gaps (where no UMRS equivalent exists yet) as explicit TODO rows rather than silent re-implementation.

---

## Scope (v1 — core packages only)

- `libs/umrs-core`
- `libs/umrs-selinux`
- `libs/umrs-platform`
- `libs/umrs-hw`

Binaries (`umrs-ls`, `umrs-stat`, `umrs-label`, `umrs-uname`, `umrs-c2pa`) consume these APIs but do not define new primitives — they are reference consumers, not inventory sources.

---

## Deliverables

One AsciiDoc page per task group in `docs/modules/devel/pages/`:

| # | Page | Covers |
|---|---|---|
| 1 | `file-operations-task.adoc` | Reading files (regular, procfs, sysfs, kernel-security), directories, xattrs, DAC metadata, SELinux labels, symlinks, filesystem magic |
| 2 | `process-and-env-task.adoc` | `init_tool`, env scrubbing, `Command::env_clear`, child-process spawning, logging init |
| 3 | `crypto-and-integrity-task.adoc` | Digest computation (SHA-256/384), HW RNG, package-integrity checks, signature verification |
| 4 | `labels-and-mac-task.adoc` | SELinux context parsing, MCS translation, catalog lookup, marking validation |
| 5 | `timestamps-and-evidence-task.adoc` | RDTSC/RDTSCP hardware timestamps, `EvidenceRecord`, `EvidenceBundle`, `ConfidenceModel` |
| 6 | `state-and-config-task.adoc` | Loading/saving serialized state, reading config files, template expansion |

Page 1 drafted first as the template; subsequent pages follow the same column layout.

## Column layout (every page)

| Task | Traditional method | UMRS secure method | Evidence bundle? | Trust range |
|---|---|---|---|---|

- **Evidence bundle?** — `yes` / `no` / `partial` (emits record but no bundle).
- **Trust range** — `TrustLevel` tier the call can participate in (T0–T4). Variant names are OS-detection-oriented; interpretation is "the highest tier a confidence model can reach given this call and its provenance gates."

## Navigation

New section added to `docs/modules/devel/nav.adoc`:

```
* Secure API Inventory
** xref:file-operations-task.adoc[File operations]
** (remaining pages added as they land)
```

---

## Gaps surfaced by the v1 inventory

Captured verbatim from the first-page scan (2026-04-20). These become follow-up tasks, not silent re-implementations:

1. No wrapper for `listxattr(2)` / `flistxattr(2)`.
2. No dedicated `readlink(2)` wrapper (resolution implicit in `EvidenceRecord::path_resolved`).
3. No write-side APIs: `open(O_WRONLY)`, `write`, `mkdir`, `unlink`, `chmod`, `chown`, xattr set — all absent. Current inventory is 100% read-only.
4. `EvidenceBundle` is only consumed by `umrs-platform`'s detection pipeline. `umrs-selinux`, `umrs-c2pa`, and `umrs-hw` file operations do not emit bundles.

These are the highest-value backlog items the inventory produces.

---

## Execution order

1. ~~Inventory file operations (Explore agent)~~ done 2026-04-20.
2. Draft `file-operations-task.adoc` — done 2026-04-20.
3. Jamie reviews page 1 for accuracy and layout.
4. On sign-off: repeat for pages 2–6.
5. Add gap rows as tasks on the task board as each page lands.

---

## Notes on naming

Page filenames follow `<group>-task.adoc`. AsciiDoc (`.adoc`) per project doc convention — Jamie's earlier example used `.md` but all pages in `docs/modules/devel/pages/` are `.adoc`.
