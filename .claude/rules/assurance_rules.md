## Security Control Alignment Rule

- Design features to satisfy applicable security controls where feasible.
- When a feature supports a control, document the mapping.
- Document mappings in Rust doc comments (///) at module or item level.
- Document mappings in project documentation.
- Document mappings in architectural or design documentation when applicable.
- Security control alignment is a design objective — map to controls wherever feasible, but do not block a sound implementation solely because a specific control citation cannot be identified.

## Security and Reliability Priority Rule

- Security is a primary requirement.
- Reliability is a primary requirement.
- High assurance is a primary requirement.
- Performance is required but secondary to security and reliability.
- Implement high assurance patterns efficiently.
- Do not sacrifice security or reliability for performance.
- Do not introduce unnecessary performance degradation.

