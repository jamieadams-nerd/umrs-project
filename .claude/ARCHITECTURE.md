# UMRS Architecture Reference

This file contains workspace layout, crate dependency constraints, module maps, and
architectural review triggers extracted from `CLAUDE.md` during the 2026-03-18
housekeeping pass. It is the single source of truth for structural and architectural
constraints.

---

## Workspace Layout

```
components/rusty-gadgets/   ← Cargo workspace root (production crates only)
  umrs-selinux/             ← PRIMARY CRATE (SELinux MLS reference monitor)
  umrs-hw/                  ← Hardware timestamp isolation (workspace unsafe boundary)
  umrs-platform/            ← Low-level OS/kernel layer (depends on umrs-hw only)
  umrs-core/                ← Shared formatting, i18n, timing utilities
  umrs-ls/                  ← First tool: security-enriched directory listings
  umrs-logspace/            ← Audit trail and logging
  umrs-state/               ← System state introspection
  umrs-tui/                 ← TUI framework and audit card binaries
  xtask/                    ← Build automation (fmt/clippy/test)
components/rust-prototypes/ ← Prototype workspace (out of scope for active development)
  cui-labels/               ← CUI label definitions (JSON-serializable)
  kernel-files/             ← Kernel attribute file parsing
  mcs-setrans/              ← MCS → human-readable category translation
  vaultmgr/                 ← Secret/vault management
components/platforms/rhel10/ ← SELinux policy modules (non-Rust)
components/tools/           ← Shell signing tools
docs/                       ← Antora documentation (architecture, devel, patterns, operations)
refs/                       ← Third-party standards (NIST, DoD) — see refs/manifest.md
```

**Note:** `components/rust-prototypes/` was split from `rusty-gadgets/` on 2026-03-11.
It has no `xtask`; use `cargo build`/`cargo test` directly. All agents should ignore
this workspace unless explicitly directed by Jamie.

---

## Crate Dependency Rules

These dependency directions are **fixed architectural constraints** and must never be violated
during coding, refactoring, or the addition of new features. Reversing or adding to these
directions is prohibited without an explicit architectural decision.

| Crate | Allowed dependencies (workspace) |
|---|---|
| `umrs-hw` | None — the workspace's unsafe isolation boundary; no workspace deps |
| `umrs-platform` | `umrs-hw` only — no dependencies on `umrs-selinux` or `umrs-core` |
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
