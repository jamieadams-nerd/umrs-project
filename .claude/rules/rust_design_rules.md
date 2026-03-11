## Tool Security Posture Rule

- Design tool functionality from a security posture perspective.
- Design tool output from a security posture perspective.
- Implement features that evaluate trust, integrity, labeling, enforcement, or provenance.
- Do not implement functionality that is purely administrative unless instructed.

## Layered Separation Rule

- When building high-assurance tools, enforce layered separation.
- Separate data collection, storage, reading, and presentation.
- Do not combine write-path logic with read-path or display logic.
- Display layers (CLI, TUI, GUI) must depend only on read interfaces.
- Design layers to support different access control policies.
- Example: umrs-state writes JSON; a separate reader parses it; CLI/TUI/GUI display parsed state only.

## Presentation Tone Rule

- Maintain high-assurance functionality.
- Maintain integrity.
- Make tools and documentation approachable.
- Encourage engagement and exploration.
- Informal elements are permitted (e.g., ASCII art, visual effects).
- Informal elements must not reduce correctness.
- Informal elements must not reduce clarity.
- Informal elements must not reduce security posture.

## Source Comment Discipline Rule

- Preserve code readability.
- Do not place security control citations on every enum, field, or line item.
- Do not use excessive /// comments at fine granularity.
- Place security control mappings at module, struct, or major component level.
- Use parent documentation blocks to reference related items.
- Keep inline comments focused on behavior and intent.

## Control Flow Readability Rule

- Avoid long combinator or method chains when they reduce readability.
- Break complex chains into intermediate variables when intent is unclear.
- Prefer explicit control flow for multi-step logic.
- Do not refactor solely for compactness.
- Performance or security requirements may justify chaining.
- If chaining is required, keep transformations understandable and scoped.
