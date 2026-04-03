# Enhanced Security Testing Plan

**Status:** stub
**ROADMAP:** G2 (High-Assurance Platform), G4 (Testing & Validation)
**Source:** `.claude/research/secure-code/workflow.md`, `.claude/research/secure-code/newchecks.md`

---

## What we already have

- `cargo xtask clippy` — pedantic + nursery, zero warnings enforced
- `cargo xtask test` — all crate tests, external test directory structure
- `cargo deny check` — licenses, bans, advisories, sources
- `RUSTSEC-AUDIT.md` — human-readable audit trail
- High-assurance patterns in code: TPI, validate-at-construction, fail-closed,
  FD-anchored I/O, typed security primitives, `#![forbid(unsafe_code)]`
- Negative testing discipline on parsers and validators (ongoing practice)

---

## 1. Property Testing

**Tool:** `proptest`
**Requires:** stable toolchain
**Priority:** High — ready to start now

### Why this is first

UMRS has typed security primitives with algebraic invariants. Property testing
hammers those invariants with random inputs. This is the highest-value testing
technique for our domain that we aren't doing yet.

### Where it hits hardest

| Target | Invariant |
|---|---|
| `SecurityContext` | parse -> serialize -> parse == identity |
| `CategorySet` | bit ordering invariant; union/intersection laws hold |
| `MlsLevel` / `MlsRange` | dominance is a partial order (reflexive, antisymmetric, transitive) |
| `Sensitivity` | bounded within 0..=3 after construction |
| Posture signals | normalization is idempotent |
| `setrans.conf` parser | round-trip fidelity |

### What to build

- Add `proptest` as dev-dependency to `umrs-selinux`, `umrs-platform`, `umrs-labels`
- Write `tests/proptests/` directories per crate
- Focus on round-trip identity and algebraic laws, not random inputs

---

## 2. Fuzzing

**Tool:** `cargo-fuzz` (libfuzzer-based)
**Requires:** nightly toolchain (rustup)
**Priority:** High — but blocked on rustup decision
**Prerequisite:** Resolve rustup vs dnf-installed Rust on RHEL 10

### Why this is second

UMRS ingests structured input from kernel interfaces, config files, and JSON
catalogs. Fuzzing finds parser bugs that humans and property tests miss — edge
cases in encoding, truncation, and malformation.

### Fuzz targets (by value)

1. `SecurityContext::from_str()` — TPI dual-path parser
2. `/proc` and `/sys` readers — kernel boundary ingestion
3. CUI label JSON parsing — catalog ingestion
4. `setrans.conf` parsing
5. `os-release` parsing

### What to build

- Install rustup, add nightly toolchain
- `fuzz/` directory per crate with target definitions
- Seed corpus from existing test fixtures
- Short regression runs (30s per target) when CI exists

---

## 3. Everything else — folded into existing workflows

These items from the research notes have value but don't need their own plan
layers. They fold into work we already do or decisions we haven't made yet.

| Item | Where it belongs | Action |
|---|---|---|
| cargo-geiger | Audit workflow (`RUSTSEC-AUDIT.md`) | Run once, document baseline, re-run on dep changes |
| Negative testing | `.claude/rules/test_structure_rules.md` | Formalize as checklist for new parsers/validators |
| STRIDE threat modeling | One-time exercise | Output feeds fuzz targets and property test lists above |
| Miri + sanitizers | Low value with `forbid(unsafe_code)` | Revisit only if `umrs-hw` grows or new unsafe deps appear |
| CI gates | When CI pipeline exists | Run clippy, deny, test, plus short fuzz regression |
| Coverage (tarpaulin) | Periodic spot-check | Use to find missing negative tests, not to chase % |

---

## What the research notes validated

The research notes recommended several patterns that UMRS already implements.
This section exists so we don't re-derive them later.

- Typed state elimination (CategorySet, Sensitivity, SecurityContext)
- Single canonical constructor path (validate-at-construction rule)
- Enums over booleans (SignalId, SecurityObservation, etc.)
- Total functions with Result (no unwrap, denied by clippy)
- Flat control flow (let-else, early return)
- Executable invariants (debug_assert in HA patterns)
- Named assurance patterns (TPI, TOCTOU-safe reads, trust gates)
- Defense-in-depth redundancy preserved (dual-path validation)
