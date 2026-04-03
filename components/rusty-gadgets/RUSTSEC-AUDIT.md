# UMRS Security Audit Log

Last audited: 2026-04-03
Tools: cargo-audit 0.21.x, cargo-deny 0.18.x

## How to use this file

This file documents every RustSec advisory, license decision, and dependency
concern that has been evaluated by the team. It is the authoritative record of
what was found, what was accepted, and why.

**When to update:** After every `cargo audit` and `cargo deny check` run. See
the Audit Workflow section at the end of this file.

---

## Active Advisories

### RUSTSEC-2023-0071 — rsa: Marvin Attack timing sidechannel

| Field | Value |
|---|---|
| Status | Accepted risk — no patch available |
| Crate | rsa 0.9.10 |
| Via | c2pa 0.78.8 -> umrs-c2pa |
| CVSS | 5.9 (medium) |
| Date assessed | 2026-04-03 |
| Assessed by | Jamie Adams |

**Finding:** RSA private key timing sidechannel observable over the network.

**Why we are not affected:** The `rsa` crate is pulled in transitively by `c2pa`
for manifest signing. In UMRS, C2PA signing occurs exclusively within the
airgap-native vault ingest pipeline. No RSA operations are exposed to a network
context where timing observation is feasible.

**Resolution:** Monitor RustCrypto/RSA for a constant-time patch. Upgrade `c2pa`
immediately when a patched `rsa` release is available.

**References:**
- https://rustsec.org/advisories/RUSTSEC-2023-0071
- https://people.redhat.com/~hkario/marvin/
- https://github.com/RustCrypto/RSA/issues/19#issuecomment-1822995643

---

### RUSTSEC-2024-0370 — proc-macro-error: unmaintained

| Field | Value |
|---|---|
| Status | Accepted risk — no alternative available upstream |
| Crate | proc-macro-error 1.0.4 |
| Via | c2pa 0.78.8 -> iref -> static-regular-grammar |
| Severity | Unmaintained (no CVE) |
| Date assessed | 2026-04-03 |
| Assessed by | Jamie Adams |

**Finding:** The `proc-macro-error` crate is unmaintained (no commits for 2+ years,
no releases for 4+ years, maintainer unreachable). It also depends on `syn 1.x`,
contributing to duplicate `syn` versions in the dependency tree.

**Why this is acceptable:** This is a build-time proc-macro dependency only. It
generates code at compile time and has zero runtime footprint. The risk surface
is limited to supply chain compromise of the crate itself, which is mitigated by
crates.io's append-only registry model and our `deny.toml` source restrictions.

**Resolution:** Monitor `c2pa` releases for a dependency tree that drops
`static-regular-grammar` or moves to `proc-macro-error2`. Alternatives exist
(`manyhow`, `proc-macro-error2`, `proc-macro2-diagnostics`) but the fix must
come from the upstream `static-regular-grammar` crate, not from UMRS.

**References:**
- https://rustsec.org/advisories/RUSTSEC-2024-0370
- https://gitlab.com/CreepySkeleton/proc-macro-error/-/issues/20

---

## Resolved Advisories

### RUSTSEC-2024-0436 — paste: unmaintained (RESOLVED 2026-04-03)

| Field | Value |
|---|---|
| Status | Resolved — dependency removed |
| Crate | paste 1.0.15 |
| Was via | ratatui-garnish 0.1.0 -> ratatui 0.29.0 |
| Resolution date | 2026-04-03 |

**Resolution:** Removed unused `ratatui-garnish` dependency from `umrs-ui`.
The crate was declared in Cargo.toml but never imported in any source file.
Removing it eliminated ratatui 0.29.0 and its `paste` dependency from the tree.

---

### RUSTSEC-2026-0002 — lru: unsound IterMut (RESOLVED 2026-04-03)

| Field | Value |
|---|---|
| Status | Resolved — dependency removed |
| Crate | lru 0.12.5 |
| Was via | ratatui-garnish 0.1.0 -> ratatui 0.29.0 |
| Resolution date | 2026-04-03 |

**Resolution:** Same as RUSTSEC-2024-0436 above. Removing `ratatui-garnish`
eliminated the transitive `lru 0.12.5` dependency.

---

## License Audit

All dependency licenses have been reviewed and approved. The `deny.toml` allow
list contains:

| License | Crate examples | Assessment |
|---|---|---|
| MIT | Most of the ecosystem | Permissive, no issue |
| Apache-2.0 | serde, tokio, clap | Permissive, no issue |
| Apache-2.0 WITH LLVM-exception | compiler support crates | Permissive, no issue |
| BSD-3-Clause | curve25519-dalek, ed25519-dalek, bcder, subtle, brotli | Permissive, OSI-approved |
| ISC | ring, rustls, rustls-webpki, untrusted | Permissive, OSI-approved |
| Zlib | foldhash, miniz_oxide, png_pong | Permissive, OSI-approved |
| Unicode-3.0 | ICU4X crates, unicode-ident, zerovec family | Permissive, Unicode Consortium |
| CDLA-Permissive-2.0 | webpki-roots | Permissive, Linux Foundation |

### Resolved license issues

**colored (MPL-2.0):** Replaced with `owo-colors` (MIT) on 2026-04-03. The
`colored` crate was used in `umrs-core` for terminal color output. `owo-colors`
is MIT-licensed, zero-allocation, and supports the `NO_COLOR` standard.

---

## Duplicate Crate Warnings

As of 2026-04-03, `cargo deny check bans` reports 20 duplicate crate warnings.
All duplicates are caused by transitive dependencies in the `c2pa` crate tree
pulling older versions of shared ecosystem crates.

These are warning-level, not errors. They increase compile time and binary size
but do not indicate a security or correctness problem.

**Notable duplicates:**
- `hashbrown` (3 versions) — c2pa dep tree vs workspace
- `getrandom` (3 versions) — c2pa dep tree vs workspace
- `syn` (2 versions) — proc-macro-error uses syn 1.x
- `crossterm` (2 versions) — c2pa dep tree
- `winnow`, `toml`, `thiserror`, `rustix`, `nom`, `itertools` — all c2pa-related

**Resolution:** These will reduce naturally as `c2pa` upgrades its own
dependencies. No action required from UMRS.

---

## Audit Workflow

### Files

| File | Purpose | When to update |
|---|---|---|
| `deny.toml` | Policy gate: licenses, bans, advisories, sources | When adding deps, reviewing licenses, or acknowledging advisories |
| `RUSTSEC-AUDIT.md` | Human-readable audit log with rationale | After every audit/deny run |

### Procedure

1. Run `cargo audit` — checks dependencies against RustSec advisories.
2. Run `cargo deny check` — checks licenses, bans, advisories, and sources.
3. For each new finding:
   - Assess severity and applicability to UMRS deployment context.
   - If accepting risk: add ignore entry to `deny.toml` with rationale, add full entry to this file.
   - If fixing: update the dependency, re-run checks, document resolution in this file.
4. For resolved findings: move entry to the Resolved section with resolution date and method.
5. Review this file quarterly or when `c2pa` releases a new version.

### Commands

```bash
cd components/rusty-gadgets

# Check for known vulnerabilities
cargo audit

# Full policy check (licenses, bans, advisories, sources)
cargo deny check

# List all dependency licenses
cargo deny list

# Check for available updates
cargo update --dry-run
```
