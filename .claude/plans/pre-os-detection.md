@rust-developer before writing any new code for the OS detection subsystem (os-detection.md) in umrs-platform, do the following audit first:

1. READ the entire umrs-platform/src/ tree:
   - List every existing module and its public surface (structs, enums, traits, fns)
   - Flag anything that overlaps with the detection design in .claude/skills or CLAUDE.md

2. IDENTIFY reusable existing code. Specifically look for:
   - Any secure file reader (openat2 wrapper, bounded read, FD-based open)
   - Any statx or fstatfs wrapper
   - Any readlinkat wrapper
   - Any SHA-256 / digest utility
   - Any error types that could extend to DetectionError
   - Any evidence or provenance recording types
   - Any existing confidence or trust level types
   - Any capability detection already written (SELinux, FIPS, IMA, etc.)

3. BEFORE writing a single line of new code, produce a reconciliation report:

   For each item in the detection design, state one of:
   - REUSE: existing code at <path> covers this — use it as-is
   - EXTEND: existing code at <path> is close — propose the minimal change needed
   - NEW: nothing exists — write new code
   - MOVE: code exists elsewhere in the workspace that belongs in platform
   - CONSIDER: It's future use by other code. 
     - Even code from above packages such as umrs-selinux or umrs-core
     - Some stuff could be of public use.
     - Use best practices and organize code accordingly. 
     - I am not please with the organization in the original plan. 
     - Simplify if we can. 
     - Be sure to document all security controls in source code comments.

4. RULES:
   - Do not duplicate. If a secure reader already exists, use it everywhere.
   - Do not create new error types if existing ones can be extended.
   - Do not create new evidence/provenance types if existing ones can be reused.
   - Types built during detection (EvidenceRecord, TrustLevel, etc.) must be
     designed for reuse by the rest of platform — not scoped only to detect/.
   - All new types go in the most general location that makes sense
     (e.g., evidence.rs at the crate root if used broadly, not buried in detect/).

5. Only after I approve the reconciliation report, begin writing code.
   Implement one phase at a time (phase0 → phase1 → ...) and stop for review
   between phases.

The detection design document is at: .claude/skills/os-detect-design.md
(or wherever it was placed — check the project outputs or CLAUDE.md for the path).
