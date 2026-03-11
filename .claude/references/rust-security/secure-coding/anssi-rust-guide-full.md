# Secure Rust Guidelines — Full Content (ANSSI)

Source: https://anssi-fr.github.io/rust-guide/print.html
GitHub: https://github.com/ANSSI-FR/rust-guide
License: Open License 2.0
Retrieved: 2026-03-10

---

## Development Environment

The guide emphasizes using **stable toolchains** exclusively for secure applications, avoiding nightly or beta versions that may contain compiler bugs. It recommends Rustup for toolchain management and highlights that "Tier 1 targets and certified toolchains MUST be used for safety-critical systems."

### Key Tools

**Cargo** serves as the package manager with critical security features:
- Automatic dependency verification through checksums
- Lock files tracking dependency versions
- Protection against supply chain attacks

The guide mandates that "Cargo.lock files MUST be tracked by a version control system" to ensure reproducible builds.

**Clippy** and **Rustfmt** are recommended linting and formatting tools. The document states that "a linter, such as clippy, MUST be used regularly during the development of a secure application."

## Library Management

Dependencies require careful vetting. The guide requires that "each direct third-party dependency MUST be properly validated, and each validation MUST be tracked."

Tools like `cargo-audit` and `cargo-outdated` help identify vulnerabilities and outdated packages. The document specifies that "the cargo-audit tool MUST be used to check for known vulnerabilities in dependencies."

## Language Guarantees and Unsafe Code

Rust's safety guarantees apply only to safe code. The guide emphasizes that "unsafe-free code cannot go wrong" but requires exceptional justification when `unsafe` is necessary.

Valid reasons include FFI, embedded device programming, and performance optimization in small code sections. Otherwise, developers should use `#![forbid(unsafe_code)]` to prevent unsafe code entirely.

When unsafe is unavoidable, it "MUST be encapsulated in such a way that either it exposes safe behavior to the user, in which no safe interaction can result in undefined behavior."

## Integer Operations

The guide addresses overflow behavior differences between debug and release builds:

> "when an arithmetic operation can produce an overflow, the usual operators MUST NOT be used directly. Instead, specialized methods such as `checked_<op>`, `overflowing_<op>`, `wrapping_<op>`, or `saturating_<op>` MUST be used"

## Error Handling

Explicit error handling via `Result` types is strongly preferred. The document warns that "functions or instructions that can cause the code to panic at runtime MUST NOT be used" in library code.

Panic-causing patterns include `unwrap`, `expect`, `assert`, unchecked array access, and large allocations.

## Memory Management

The guide prohibits `std::mem::forget` entirely: "the forget function of std::mem MUST NOT be used." It also cautions against memory leaks from cyclic reference-counted pointers combined with interior mutability.

For sensitive data, developers must recognize that "ensuring security operations at the end of some treatment MUST NOT rely only on the Drop trait implementation."

## Foreign Function Interface (FFI)

FFI requires exceptional care since it crosses safety boundaries. Key requirements include:

- Using `repr(C)` for type compatibility
- Employing pointers rather than references in low-level bindings
- Checking all foreign pointer validity before dereferencing
- Never using enum types directly across FFI boundaries
- Wrapping foreign code in safe abstractions

The document states that "Rust code called from FFI MUST either ensure the function cannot panic, or use catch_unwind or the std::panic module to ensure the Rust code will not abort."

## Standard Library Traits

**Send and Sync** traits require careful implementation. Manual implementations "SHOULD be avoided and, if necessary, MUST be justified and documented."

**Comparison traits** (PartialEq, Eq, PartialOrd, Ord) carry mathematical invariants that the compiler doesn't verify. The guide recommends: "the implementation of standard comparison traits SHOULD be automatically derived with #[derive(...)] when structural equality and lexicographical comparison is needed."

**Drop** implementations must never panic and shouldn't be the sole mechanism for sensitive cleanup operations.

## Naming Conventions

Following Rust API Guidelines is mandatory: "Development of a secure application MUST follow the naming conventions outlined in the rust-guidelines."

The standard pattern uses `UpperCamelCase` for types and traits, `snake_case` for functions and variables, and `SCREAMING_SNAKE_CASE` for constants.

## License

Distributed under the Open License 2.0, permitting reuse with attribution.

---

## Key MUST Requirements Summary

1. Tier 1 targets and certified toolchains MUST be used for safety-critical systems
2. Cargo.lock files MUST be tracked by version control
3. A linter (clippy) MUST be used regularly
4. Each direct third-party dependency MUST be properly validated and tracked
5. cargo-audit MUST be used to check for known vulnerabilities
6. Arithmetic operations that can overflow MUST use checked/overflowing/wrapping/saturating variants
7. Functions that can panic MUST NOT be used in library code
8. std::mem::forget MUST NOT be used
9. Manual Send/Sync implementations MUST be justified and documented
10. Rust code called from FFI MUST ensure no panics or use catch_unwind
11. #![forbid(unsafe_code)] SHOULD be used when unsafe is not needed
12. Development MUST follow Rust API naming conventions
