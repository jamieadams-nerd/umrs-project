This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

All Rust work happens under `components/rusty-gadgets/`. Run these from that directory:

```bash
cd components/rusty-gadgets

# Format
cargo xtask fmt

# Lint (pedantic + nursery, -D warnings — must be clean)
cargo xtask clippy

# Test all workspace crates
cargo xtask test

# Test a single crate
cargo test -p umrs-selinux

# Run a specific integration test file
cargo test -p umrs-selinux --test category_tests

# Run an example
cargo run -p umrs-selinux --example show_status
cargo run -p umrs-selinux --example ls_ha -- -la /root

# Build docs
cargo doc -p umrs-selinux --no-deps --open
```

`cargo xtask` is an alias defined in `.cargo/config.toml` that runs the `xtask` workspace crate.

---

## General Workflow

- Planning mode. Decide upon primary features or new components.
- **Before implementing any new type, trait, or module**, search the entire workspace for
  existing equivalents. Duplication requires explicit written justification. Reuse is the default.
- Identify any opportunities to use high-assurance patterns.
- Implement code and write test cases.
- Must pass all test cases and strict clippy findings fixed.
- A worthy example or two should be written.
- Documentation updated. This includes rust API documentation and the developer guide in /docs.
  - High-assurance pattern information is applicable.
  - And a use case example to identify its use as a building block.

## Claude will NEVER

- Never git commit or push
- Never delete documentation. If it is duplicate, redundant or useless information ask me. We can
  either delete it or merge it.

## Technology Stack

- High-assurance platform with SELinux (targeted policy) and future MLS work.
- Rust is the primary language on RHEL10 with some work on Ubuntu.

## Environment Context

Understanding the deployment environment is essential to making correct architectural decisions:

- **Target OS**: Red Hat Enterprise Linux 10 (SELinux enforcing, MLS or targeted policy)
- **FIPS mode**: assumed active — `ProcFips` is a first-class kernel attribute in this codebase;
  any cryptographic operation must use FIPS 140-2/140-3 validated primitives
- **Network posture**: isolated or near-isolated systems; assume no outbound network access from
  deployed binaries; no DNS, no TLS client calls from library code
- **Audit exposure**: code and design decisions are subject to government/DoD review; every
  choice must be traceable to a requirement (NIST control, RTB rule, or explicit design decision)
- **Data sensitivity**: handles CUI (Controlled Unclassified Information) and MLS-labeled data;
  treat any value derived from a security label as potentially sensitive

---

## Critical Coding Rules

- **All public items** need NIST control, CMMC, or RTB annotation in doc comments
- **No unsafe** — `#![forbid(unsafe_code)]` is set in every crate root; this is a compile-time
  proof, not a policy. `#![forbid]` cannot be overridden by any inner `#[allow]`, making it
  mechanically verifiable by an auditor (NIST 800-218 SSDF PW.4, NSA RTB)
- **Avoid FFI** — always prefer pure Rust
- **TPI parsing** — `SecurityContext` uses two independent parsers (`nom` + `FromStr`) and fails closed on any disagreement
- **Rustfmt** — 100-char max width, 4-space indent, Unix newlines

---
## Shell Tools — Hard Rule
- Always use `rg` (ripgrep) instead of `grep` for all search operations.
- `rg` is available at `~/.cargo/bin/rg`.
- NEVER use the built-in Search or Read tools. ALL agents, no exceptions.
- ALL searching MUST use `rg` via Bash.
- ALL file reading MUST use `Bash(cat:*)` or `Bash(rg:*)`.

### Path Rules — Hard Rule
- NEVER use absolute paths starting with `/media/psf/` — this mount is not traversable by subprocesses.
- NEVER append any path argument to `rg`. Always `cd` first, then run `rg` without a path.
- If unsure of CWD, run `pwd` first before any file operation.
- Correct:   `cd components/rusty-gadgets && rg --no-heading -n --hidden --smart-case --glob '!target/*' --glob '!.git/*' <pattern>`
- INCORRECT: `rg <args> <pattern> /media/psf/repos/umrs-project/components/rusty-gadgets/`

### Standard Invocations
- General search: `rg --no-heading -n --hidden --smart-case <pattern>`
- Source code search: `cd components/rusty-gadgets && rg --no-heading -n --hidden --smart-case --glob '!target/*' --glob '!.git/*' <pattern>`
- Source code workspace is `components/rusty-gadgets/` — scope searches there unless told otherwise.


## Settings Files — Hard Rule
- There is ONE settings file: `.claude/settings.json`
- NEVER create `.claude/settings.local.json` or any other settings variant.
- If you need to add a permission or env var, edit `.claude/settings.json` directly.
- This rule applies to ALL agents without exception.

## Hard Rule — Tool Selection
- The built-in `Search` tool is DISABLED for this project.
- To search files, ALWAYS run: `Bash(rg --no-heading -n --hidden --smart-case --glob '!target/*' --glob '!.git/*' <pattern> [path])`
- Never use Search(pattern:...). Never. Use rg.

---

## Workspace Layout

```
components/rusty-gadgets/   ← Cargo workspace root
  umrs-selinux/             ← PRIMARY CRATE (SELinux MLS reference monitor)
  umrs-platform/            ← Low-level OS/kernel layer (no workspace deps)
  umrs-core/                ← Shared formatting, i18n, timing utilities
  umrs-ls/                  ← First tool: security-enriched directory listings
  umrs-logspace/            ← Audit trail and logging
  umrs-state/               ← Prototypes: System state introspection
  cui-labels/               ← CUI label definitions (JSON-serializable)
  kernel-files/             ← Prototypes: Kernel attribute file parsing
  mcs-setrans/              ← Prototypes: MCS → human-readable category translation
  vaultmgr/                 ← Prototypes: Secret/vault management
  xtask/                    ← Build automation (fmt/clippy/test)
components/platforms/rhel10/ ← SELinux policy modules (non-Rust)
components/tools/           ← Shell signing tools
docs/                       ← Antora documentation (architecture, devel, patterns, operations)
refs/                       ← Third-party standards (NIST, DoD) — see refs/manifest.md
```

## Crate Dependency Rules

These dependency directions are **fixed architectural constraints** and must never be violated
during coding, refactoring, or the addition of new features. Reversing or adding to these
directions is prohibited without an explicit architectural decision.

| Crate | Allowed dependencies (workspace) |
|---|---|
| `umrs-platform` | None — no dependencies on `umrs-selinux` or `umrs-core` |
| `umrs-selinux` | `umrs-platform` only |
| `umrs-core` | `umrs-platform` and `umrs-selinux` |

**Enforcement**: Before adding any `path = "../..."` dependency to a `Cargo.toml`, verify it
does not violate the table above. If a proposed design requires a direction not listed here,
stop and raise it with the developer before proceeding.

---

## umrs-selinux Module Map

Key modules (all under `src/`):

| Module | Purpose |
|---|---|
| `context.rs` | `SecurityContext`: full SELinux label with dual-path TPI parsing |
| `category.rs` | `CategorySet`: 1024-bit `[u64; 16]` bitmask for MLS categories |
| `sensitivity.rs` | `SensitivityLevel`: s0–s15 hierarchical levels |
| `mls/` | MLS level + range types; lattice dominance math |
| `mcs/` | Multi-Category Security translation and color coding |
| `secure_dirent.rs` | `SecureDirent`: TOCTOU-safe, security-enriched directory entry |
| `xattrs.rs` | `SecureXattrReader`: fd-anchored xattr access via `rustix` |
| `posix/` | `Uid`, `Gid`, `Inode`, `FileMode`, Linux identity resolution |
| `utils/kattrs.rs` | KATTRS: provenance-checked kernel attribute reader |
| `status.rs` | SELinux kernel enable/MLS-enable status queries |

Integration tests live exclusively in `tests/` — never inline.

---

## Clippy Policy

`lib.rs` enables `#![warn(clippy::pedantic)]` and `#![warn(clippy::nursery)]`. The guiding principle:

**Correctness and safety lints are law. Aesthetic lints are suppressed when they trade
readability for "idiomatic" style.**

Current suppressions and their rationale:

| Lint | Reason suppressed |
|---|---|
| `unwrap_used` | **Denied** — hard requirement, never allowed |
| `option_if_let_else` | Clippy prefers `.map_or_else()` over plain `if let` — the expanded form is clearer |
| `redundant_closure` | Clippy prefers `foo` over `\|x\| foo(x)` — explicit closures are sometimes clearer at the call site |
| `module_name_repetitions` | `SelinuxUser` in module `selinux` is intentional and clear |
| `missing_errors_doc` | `# Errors` sections on every `Result`-returning fn is excessive noise |
| `missing_panics_doc` | `# Panics` sections for unreachable panics add no value |
| `unreadable_literal` | Underscore grouping in hex/binary bitmasks would obscure their meaning |
| `doc_markdown` | Backtick-wrapping every code-looking term in prose is disruptive |

When a lint fires and the suggested rewrite would reduce clarity, add `#[allow(lint_name)]`
on the function rather than rewriting to the "fancy" form.

---

## Compliance Annotations

Public items need NIST 800-53, CMMC, or NSA RTB annotations in their doc comments, but the
requirement is tiered:

- **Modules** — always include relevant control references in the module-level doc comment
- **Security-critical types and functions** — require explicit control citations (e.g., `NIST 800-53 AC-4`, `NSA RTB RAIN`)
- **Simple accessors and display impls** — no annotation required if the parent type is already annotated

---

## High-Assurance Design Patterns

The full pattern library with threat descriptions, codebase examples, and control citations
lives in `docs/modules/patterns/pages/`. The developer guide at
`docs/modules/devel/pages/high-assurance-patterns.adoc` provides the consolidated narrative.

Enforcement rules for these patterns are in `.claude/rules/high_assurance_pattern_rules.md`
and `.claude/rules/assurance_rules.md`.

---

## Architectural Review Triggers

When any of the following appear during planning or code review, Claude Code must pause and
raise the relevant pattern or concern before proceeding:

| Trigger | Pattern to raise |
|---|---|
| New external dependency | Supply chain hygiene — justify the crate |
| Integer arithmetic on a security value | Secure arithmetic — use checked ops |
| Comparison of tokens, labels, or credentials | Constant-time comparison — use `subtle` |
| Type that could hold secret or classified data | Zeroize / `secrecy` |
| New parser for security-relevant input | TPI — two independent parse paths |
| New file or kernel attribute I/O | TOCTOU safety — fd-anchored access |
| Reading from `/sys/fs/selinux/` or `/proc/` | Provenance verification — check `statfs` magic |
| Error message that includes variable data | Error information discipline — no sensitive data |
| Any cryptographic primitive | FIPS 140-2/3 — confirm the primitive is validated |
| New public API surface | Compliance annotation — add NIST/RTB control citation |
| New crate added to workspace | Add `#![forbid(unsafe_code)]` to its crate root immediately |
| New type, trait, or module proposed | Search workspace for existing equivalents first — duplication requires written justification |
| Expensive verification result reused across calls | SEC — sealed evidence cache with HMAC + TTL |
| Security-relevant fn returns Result/Option | `#[must_use]` with message string |
| New config file read from /etc/ | Trust gate — confirm kernel subsystem is active first |

---

## Agent Directory

| Agent | Responsibility |
|---|---|
| `rust-developer` | New patterns implemented, API changes, doc gaps noticed in source, patterns needed but not yet in library |
| `security-engineer` | Compliance findings that require doc updates, new control mappings, audit gaps |
| `security-auditor` | Compliance audits: verifies control citations, identifies annotation debt, produces audit findings and reports |
| `tech-writer` | Questions about API or pattern intent, requests for source examples |
| `senior-tech-writer` | Architecture-level doc decisions, cross-module structural changes |
| `researcher` | RAG pipeline management, reference collection ingestion, standards research, research reports (`refs/reports/`) |
| `umrs-translator` | Text extractions from i18n-wrapped strings, language translations for active domains |
| `changelog-updater` | Structured changelog maintenance: tracks additions, changes, and fixes across crates, docs, and infrastructure in `.claude/CHANGELOG.md` |

## Team Collaboration & Workflow

**All agents must read `.claude/team-collaboration.md` at session start.** It defines:

- Team structure, role boundaries, and why specialization matters
- The review pipeline (developer → security → architecture → documentation)
- How work enters the system (Jamie's research → `jamies_brain/` → `plans/`)
- Cross-notification responsibilities and post-work handoff rules
- `docs/new_stuff/` routing (senior-tech-writer and tech-writer only)
- Operational rules: notify when idle, remind when blocked, never block silently

---
## Code Navigation & Metadata

### Crate & Dependency Metadata — cargo metadata
- Use `cargo metadata --format-version 1` for structured crate/dependency information.
- Provides: full dependency graph, crate structure, target list, workspace layout.
- Prefer this over manual directory inspection for workspace-level questions.
- Standard invocation: `cargo metadata --format-version 1 | python3 -m json.tool`
- For dependency-only queries (faster): `cargo metadata --format-version 1 --no-deps`

### rust-developer Agent Rules
- Before asking Jamie about crate structure, run `cargo metadata` first.
- Never traverse `target/` for metadata — use `cargo metadata` instead.
- Use `--no-deps` by default; only drop it when external dependency graph is explicitly needed.

---

## Role of Claude Code in This Project

This codebase operates in a high-assurance, heavily scrutinized environment. Claude Code is
expected to function as an **architectural partner**, not just a code writer. This means:

- **Proactively identify** opportunities to apply security patterns, even when not asked
- **Flag compliance gaps** — note when a design does not satisfy a NIST, CMMC, or RTB requirement
  it could satisfy, and propose how to close the gap
- **Challenge trust boundaries** — when a new interface, module, or data path is being designed,
  explicitly reason about what is trusted, what is untrusted, and where the validation boundary sits
- **Raise new patterns** — if a technique from NIST 800-218 SSDF, NSA RTB, or related frameworks
  applies and has not been used, surface it before implementation begins
- **Scrutinize new dependencies** — every new crate is an attack surface and a supply chain risk;
  flag it and assess its suitability before it is added
- **Think in threat models** — for any new feature, ask: what does an adversary gain if this fails?
  What does the system reveal? What can be replayed, forged, or bypassed?

The goal is to seize every opportunity to strengthen the security posture. Keep the developer
on their toes.

---

## Reference Documents

Third-party standards and guidance documents are stored in `refs/` at the repo root.
The manifest at `refs/manifest.md` tracks each document's version, download date, source
URL, and SHA-256 checksum. When asked, Claude Code will check source URLs for newer
versions and summarize changes.

Two documents in the manifest require manual browser download (DoD portals block curl).
See `refs/manifest.md` for instructions.

---

## Performance & Task Tracking

### Task Log — Hard Rule
- Every agent MUST append a one-line entry to `.claude/logs/task-log.md` upon task completion.
- Create the file and directory if it does not exist.
- Format:
  `[YYYY-MM-DD HH:MM] [agent-name] [brief task description] [tools used] [outcome: success/partial/failed] [notes]`

### Examples
```
[2026-03-14 09:15] rust-developer  unwrap() audit  rg,cargo-metadata  success  zero hits confirmed
[2026-03-14 09:30] security-auditor  comment review on umrs-selinux  rg,cat  success  2 annotations added
[2026-03-14 10:00] researcher  NIST SP 800-53 rev5 fetch  WebFetch  partial  rate limited on 2nd doc
```

### What to Record in Notes
- Dead ends or retries
- Tools that failed or were unavailable
- Anything that required Jamie intervention
- Unexpected findings worth remembering

### Benchmark Query
- The canonical baseline query for rust-developer efficiency is:
  `"List all crates, their editions, and internal dependencies."`
- Record tool count and steps in notes when running this query after significant workflow changes.


### End of Session Report
- When the team goes idle, always report a summary of `.claude/logs/task-log.md` 
  entries from the current session before signing off.


