# Compile-Time Contract Enforcement: `#[must_use]`

## Overview

The `#[must_use]` attribute is a compile-time contract enforcement mechanism
that instructs the Rust compiler to emit a warning when a return value is
silently discarded by the caller. In high-assurance systems, this transforms
what would otherwise be a runtime failure into a build-time policy violation.

**Applicable Standards:**
- NIST SP 800-53 Rev 5: SI-10 (Information Input Validation)
- NIST SP 800-53 Rev 5: SA-11 (Developer Testing and Evaluation)
- RTB: Fail Secure / Fail Safe

---

## Motivation

Silent error discard is among the most dangerous failure modes in security
code. When a function returns a `Result` or `Option` and the caller does not
handle it, the failure disappears silently — no log, no panic, no observable
effect. In a system handling CUI under MLS policy, an unhandled label
assignment error or an ignored access control result is a potential security
violation.

`#[must_use]` eliminates this class of defect at compile time, consistent
with the UMRS principle of pushing failure detection as far left as possible.

---

## Usage

### On Functions

Apply to any function whose return value carries security-relevant outcome
information — particularly `Result<T, E>` and `Option<T>` returns.

```rust
// NIST SP 800-53 SI-10, SA-11 | RTB: Fail Secure
#[must_use = "CUI label assignment must be verified before proceeding"]
pub fn assign_label(&self) -> Result<MlsLabel, LabelError> {
    // ...
}
```

If the caller discards this value without handling it, the compiler emits:

```
warning: unused `Result` that must be used
note: CUI label assignment must be verified before proceeding
```

### On Types

Apply to a type when any value of that type should never be silently dropped.
This is appropriate for types that represent pending operations, audit events,
or security decisions.

```rust
// NIST SP 800-53 AU-9 | RTB: Non-repudiation
#[must_use = "audit records must be committed or explicitly discarded"]
pub struct AuditRecord { /* ... */ }
```

---

## Required Pattern

All public API functions in UMRS crates that return `Result`, `Option`, or
any security-relevant type **must** carry `#[must_use]`. The annotation
**must** include a message string explaining why the return value matters.

```rust
// Correct
#[must_use = "returns Err if the MLS range is invalid"]
pub fn validate_range(&self) -> Result<(), RangeError> { ... }

// Non-compliant — annotation present but message missing
#[must_use]
pub fn validate_range(&self) -> Result<(), RangeError> { ... }
```

When a caller intentionally discards a return value, the discard must be
explicit and accompanied by a comment justifying the decision:

```rust
// Intentional discard — error is non-fatal in this context; logged upstream
let _ = self.flush_audit_buffer();
```

---

## Relationship to Other Compile-Time Contracts

`#[must_use]` is one element of UMRS's broader strategy of encoding security
policy into the Rust type system rather than relying on documentation,
runtime assertions, or administrative controls alone.

| Mechanism           | Policy Enforced                                  | Standard        |
|---------------------|--------------------------------------------------|-----------------|
| `#[must_use]`       | Return values must be explicitly handled         | SI-10, SA-11    |
| Ownership / borrow  | Memory and concurrent access safety              | SI-16           |
| `#[non_exhaustive]` | Match arms remain complete as types evolve       | SA-11           |
| Type-state pattern  | Only valid state transitions are expressible     | AC-3, SI-10     |

---

## Security Auditor Checklist

The `security-auditor` agent **must** flag the following as findings:

- [ ] Public function returns `Result` or `Option` without `#[must_use]`
- [ ] `#[must_use]` present but no explanatory message string
- [ ] Caller uses `let _ =` without an accompanying justification comment
- [ ] Security-relevant type lacks `#[must_use]` at the type definition

---

*Section owner: tech-writer agent*
*Last reviewed: see version control*
