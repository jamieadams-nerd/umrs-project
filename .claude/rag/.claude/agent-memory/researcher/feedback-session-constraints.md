---
name: Session Tool Constraints
description: Patterns for when Bash/WebFetch are unavailable and how to proceed
type: feedback
---

# Session Tool Constraints

## Bash and WebFetch Unavailable

**Rule:** When both Bash and WebFetch are denied in a session, use WebSearch to gather authoritative content and Write to save synthesized files.

**Why:** Permission denials are session-level and cannot be worked around by other means. The Write tool can bootstrap directories. WebSearch against official domains still retrieves authoritative metadata (URLs, version dates, section structure).

**How to apply:**
1. Attempt Bash first (mkdir + curl + pandoc workflow)
2. If denied, attempt WebFetch
3. If both denied, use WebSearch to confirm canonical URL, version, and section structure
4. Write synthesized files with a clear acquisition note in SOURCE.md documenting the method
5. Include re-fetch curl commands in SOURCE.md for when Bash becomes available
6. Note in SOURCE.md that verbatim text may need verification against live source

**Canada.ca curl tip (from Henri's research):**
- Use Firefox user-agent: `-A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0"`
- French pages: add `-H "Accept-Language: fr-CA,fr;q=0.9"`
- TBS policy docs: add `&section=html` parameter for cleaner extraction
