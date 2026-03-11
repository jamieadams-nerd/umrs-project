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
