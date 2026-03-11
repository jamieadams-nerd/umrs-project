# Rust Unsafe Code Guidelines

Source: https://github.com/rust-lang/unsafe-code-guidelines (README)
Book: https://rust-lang.github.io/unsafe-code-guidelines
Retrieved: 2026-03-10

---

The purpose of this repository is to collect and discuss questions that come up when writing unsafe code. It is primarily used by the [opsem team](https://github.com/rust-lang/opsem-team/) to track open questions around the operational semantics.

## Current Status

The [Unsafe Code Guidelines Reference "book"][ucg_book] is a past effort to systematize consensus on these questions. Most of it has been archived, but the [glossary](https://rust-lang.github.io/unsafe-code-guidelines/glossary.html) remains a useful resource.

Current consensus is documented in:
- [t-opsem FCPs](https://github.com/rust-lang/opsem-team/blob/main/fcps.md)
- The [Rust Language Reference](https://doc.rust-lang.org/reference/index.html)

## Relationship to Rustonomicon

The [Rustonomicon] is a draft document discussing unsafe code, intended to be brought into agreement with the UCG content. It explains *how* to write unsafe Rust code, whereas UCG is a *reference* for the memory model.

[Rustonomicon]: https://doc.rust-lang.org/nightly/nomicon/

## Key Topics

The UCG tracks questions around:
- Pointer provenance and aliasing rules
- Memory model for unsafe Rust
- Layout guarantees for types
- Validity invariants for values
- Soundness of unsafe abstractions

## Glossary Reference

The glossary at https://rust-lang.github.io/unsafe-code-guidelines/glossary.html remains the active, non-archived resource. Key concepts include:
- Undefined Behavior (UB)
- Validity invariant vs. safety invariant
- Pointer provenance
- Place (memory location)
- Value (typed interpretation of bytes)

## Licensing

Apache 2.0 / MIT dual license.
