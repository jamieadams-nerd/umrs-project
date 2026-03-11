## High Assurance Pattern Adoption Rule

- Maintain a documented list of high assurance patterns.
- Review the list when designing new features.
- Identify opportunities to apply documented patterns.
- Notify when a high assurance pattern may be applicable.
- Do not apply patterns automatically.
- Apply patterns when approved.
- Example: Consider memory zeroization when handling secrets.

## Pattern Execution Measurement Rule

- When a high assurance pattern is implemented, record execution time in debug mode (`#[cfg(debug_assertions)]` in Rust).
- Use a standardized timing mechanism.
- Use std::time::Instant and .elapsed() for duration measurement.
- Log timing at pattern completion.
- Include the pattern name in the log entry.
- Include a clear completion message.
- Use precise units (e.g., microseconds, milliseconds).
- Do not log timing in non-debug modes unless required.
- Example: Dual-path validation completed in 842 µs.

## Must-Use Contract Rule

- All public functions that return `Result`, `Option`, or a security-relevant type must carry `#[must_use]`.
- The `#[must_use]` annotation must include a message string explaining why the return value matters.
- Security-relevant types (pending operations, audit events, security decisions) must carry `#[must_use]` at the type definition.
- Bare `#[must_use]` without a message is non-compliant — always include the reason.
- When a caller intentionally discards a return value, the discard must use `let _ =` with a justification comment.
- Controls: NIST SP 800-53 SI-10, SA-11 / RTB: Fail Secure.

## Validate at Construction Rule

- Security-relevant types must validate all invariants in their constructor.
- Constructors return `Result` — if construction succeeds, the value is proven valid.
- Downstream code receives the type, not a `Result` — the error was handled at the boundary.
- No downstream re-validation is needed or expected; the type carries proof of validity.
- Fields must be private to prevent direct construction that bypasses validation.
- Controls: NIST 800-218 SSDF PW.4.1 / NSA RTB RAIN.

## Trust Gate Rule

- Never trust configuration files (`/etc/`) when the kernel has not confirmed the subsystem is active.
- Gate config reads behind a kernel status check (e.g., only read SELinux config if kernel confirms SELinux is enabled).
- If the kernel says a subsystem is disabled, return `None` — do not guess from config.
- Controls: NIST 800-53 CM-6.

## Security Findings as Data Rule

- Security findings must be represented as enum variants, not log strings.
- Callers must be able to query, filter, count, and match findings programmatically.
- Log lines, if any, are derived from the enum — not the other way around.
- Example: `SecurityObservation::Setuid` is a matchable value; "WARNING: file is setuid" is not.
- Controls: NIST 800-53 AU-3.

## Compile-Time Path Binding Rule

- Kernel attribute paths and expected filesystem magic must be bound to the type as associated constants (e.g., `StaticSource` trait).
- There must be no runtime parameter for the path or magic — the compiler verifies correct values.
- The default trait method must route through the provenance-verified read path, making verification non-bypassable.
- Controls: NSA RTB RAIN (Non-Bypassability).

## Fixed-Size Deterministic Layout Rule

- Security-relevant bitmasks and classification structures must use fixed-size, stack-allocated arrays.
- Zero heap allocation ensures zero allocator influence on timing or layout.
- Document the endianness and bit ordering for serialization stability.
- Example: `CategorySet` uses `[u64; 16]` with little-endian bit ordering.
- Controls: NSA RTB (Deterministic Execution).
