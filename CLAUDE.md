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
  either delete it merge it.

## Technology Stack
- High-assurance platform with SELinux (targeted policy) and future mls work.
- RUST is the primary language on RHEL10 with some work on Ubuntu


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

## Workspace Layout

```
components/rusty-gadgets/   ← Cargo workspace root
  umrs-selinux/             ← PRIMARY CRATE (SELinux MLS reference monitor)
  umrs-core/                ← Shared formatting, i18n, timing utilities
  umrs-logspace/            ← Audit trail and logging
  umrs-state/               ← Prototypes: System state introspection
  cui-labels/               ← CUI label definitions (JSON-serializable)
  kernel-files/             ← Prototypes: Kernel attribute file parsing
  mcs-setrans/              ← Prototypes: MCS → human-readable category translation
  vaultmgr/                 ← Prototypes: Secret/vault management
  xtask/                    ← Build automation (fmt/clippy/test)
components/platforms/rhel10/ ← SELinux policy modules (non-Rust)
components/tools/           ← Shell signing tools
```

More specifically.
```
.
├── build-tools
│   └── antora
├── components
│   ├── apache-mls-cui
│   ├── integrity     
│   │   └── aide         <-- probably should be moved to deployment or operations.
│   ├── platforms        <--  Operating system specifics
│   │   └── rhel10       <-- Things like setrans.conf for MLS labeling
│   ├── rusty-gadgets    <-- Primary source
│   │   ├── target
│   │   ├── cui-labels    <-- SELinux markings/labels for controlled unclass info
│   │   ├── kernel-files  <--+ Prototypes for stuff that's now in selinux
│   │   ├── mcs-setrans   <--+ Proto type tools
│   │   ├── umrs-logspace <-- prototype work
│   │   ├── vaultmgr      <!-- Prototype for ingesting new files
│   │   ├── umrs-state    <!-- Prototype to capture state information
│   │   ├── umrs-ls       <!-- First tool to show directory listings
│   │   ├── umrs-core     <-- Uses selinux and platform. It is used by tools
│   │   ├── umrs-selinux  <-- Imports platform
│   │   ├── umrs-platform <-- Low level
│   │   └── xtask
│   ├── tools              <-- Non-ruse prototype tools
│   │   ├── git-signing
│   │   ├── umrs-shred
│   │   └── umrs-signing
│   └── umrs-python
│       └── umrs
├── docs              <--  Primary documentation Antora
│   ├── images
│   ├── modules
│   │   ├── admin
│   │   ├── architecture <-- Background, history, and why things are the way they are.
│   │   ├── deployment   <-- Operating system configurations
│   │   ├── devel        <-- Developer guides
│   │   ├── operations   <-- Maintaining or day-to-day of an HA system
│   │   ├── reference
│   │   └── ROOT
│   ├── _scratch
│   │   └── notes
│   └── _vendor
│       └── ui
├── help          <-- placeholder?
├── man           <-- placeholder?
├── refs
│   ├── dod
│   └── nist
│       └── fips
└── resources
    └── i18n       <== possible internationalization. 
        └── umrs-tester
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

## Reference Documents

Third-party standards and guidance documents are stored in `refs/` at the repo root.
The manifest at `refs/manifest.md` tracks each document's version, download date, source
URL, and SHA-256 checksum. When asked, Claude Code will check source URLs for newer
versions and summarize changes.

Two documents in the manifest require manual browser download (DoD portals block curl).
See `refs/manifest.md` for instructions.

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

These patterns derive from NSA RTB VNSSA, NIST 800-53, NIST CMMC 2.0, NIST 800-171, and
NIST 800-218 (SSDF). They are **not optional** — apply them wherever a security decision,
parse operation, or I/O operation is involved. When planning or reviewing code, Claude Code
must proactively flag where these patterns could be applied but have not been.

### TPI — Two-Path Independence
Parse or validate using two independent methods (e.g., `nom` + `FromStr`). If the results
disagree, fail closed. This eliminates single-point-of-failure in any security-relevant parse.
Applies to: context parsing, any input that drives an authorization or classification decision.

### TOCTOU Safety
Anchor all I/O operations to a single open `File` handle (via `rustix` fd-based syscalls).
Never re-open a resource by path for a second operation — the filesystem state may have changed
between calls.

### Fail-Closed
On any ambiguity, parse error, or disagreement between independent paths, deny access and
surface the error. Never silently succeed with a degraded or default result in a security context.

### Provenance Verification (NIST 800-53 SI-7)
Before trusting data from a kernel pseudo-filesystem (`selinuxfs`, `procfs`), verify the
filesystem magic via `statfs`. This prevents spoofing via bind-mounts or malicious overlays.

### Loud Failure
Errors are visible and auditable — never swallowed. Use `log::warn!` or `log::error!` when
a security-relevant operation degrades or fails, even if the caller handles the error.

### Non-Bypassability (RAIN)
Security checks must always be invoked. Use private constructors, newtype wrappers, and
module boundaries to ensure callers cannot accidentally skip validation.

### Secure Arithmetic (SSDF PW 4.1)
For any integer arithmetic on security-relevant values (sensitivity levels, category IDs,
bitmask indices), use `checked_*`, `saturating_*`, or `wrapping_*` operations explicitly.
Do not rely on debug-mode overflow panics or release-mode wrapping behavior for correctness.
Note: Rust release builds disable overflow checks by default — consider enabling
`overflow-checks = true` in `[profile.release]` for binaries in this codebase.

### Zeroize Sensitive Data (SSDF PW 4.1, NIST 800-53 MP-4)
Any type that holds secrets, credentials, key material, or classified data must implement
`Zeroize` (or `ZeroizeOnDrop`) using the `zeroize` crate. For types where the `Debug` output
must also be redacted, consider the `secrecy` crate (`Secret<T>` prints `[REDACTED]`).
Applies to: `vaultmgr` secret types, any buffer holding raw key bytes or cleartext credentials.

### Constant-Time Comparisons (SSDF PW 4.1)
Any comparison of security-relevant byte sequences (tokens, MACs, credentials) must use
constant-time equality to prevent timing side-channels. Use the `subtle` crate
(`ConstantTimeEq`). Standard `==` leaks timing information proportional to the position of
the first differing byte.

### Error Information Discipline (NIST 800-53 SI-12)
Error messages must never contain security labels, key material, credentials, or classified
data. Return structured error types; keep sensitive context in audit logs (where access is
controlled), not in user-visible error strings.

### Bounds-Safe Indexing (SSDF PW 4.1)
Prefer `.get(i)` over `[i]` for security-relevant array access. Never assume index validity;
treat out-of-bounds as a security event, not a logic error.

### Supply Chain Hygiene (SSDF PO 1.2, PW 4.3)
Every new external crate is an attack surface. Before adding a dependency:
- Prefer crates with minimal transitive dependency trees
- Check with `cargo audit` for known vulnerabilities
- Assess maintenance status and provenance
- Prefer crates that are FIPS-compatible or agnostic
- `Cargo.lock` is committed and treated as an auditable artifact

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

---

## Design Invariants

- Follow and use techniques in NSA RTB VNSSA and RAIN
- TOCTOU safe and elimination of implicit state (deterministic execution) among others
- Always try to satisfy NIST 800-53 Security Controls
- Let NIST CMMC 2.0 Level 1, 2, 3 and NIST 800-171 drive design choices
- Follow NIST 800-218 Secure Software Development Framework
- Types validate at construction; raw strings never cross module boundaries
- TOCTOU safety is achieved by anchoring all operations to a single `File` handle (`rustix` fd-based syscalls), never re-opening by path
- `CategorySet` layout is deterministic (`[u64; 16]`, little-endian bit ordering); serialized form must remain stable
- `SecureDirent` records findings as `SecurityObservation` enum values (data), not log strings
- Prefer easy to read code over fancier, syntactic candy — explicit `if/else` and `match` over functional chaining
