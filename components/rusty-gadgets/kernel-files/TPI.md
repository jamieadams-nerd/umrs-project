mplementing the "Two-Person Integrity" (TPI) Pattern
To satisfy the Redundant invariant of RAIN, we never trust a single parsing algorithm. A bug in a library or a subtle logic error in a state machine could lead to an incorrect "Allow" decision.
For your CUI Labels, we would implement:
Path A (nom): A declarative, combinator-based parser. It’s highly readable and great for structural validation.
Path B (Manual State Machine): A low-level, iterative while loop that manually steps through bytes. It’s "dumb" logic—easy to audit for off-by-one errors.



Why this matters to you:
By using nom, you get a very "Rust-native" way to handle the complex strings in /sys/fs/selinux/context. By pairing it with a manual C-style state machine, you provide the mathematical proof that the label you are about to enforce is correct.
Would you like to:
See a basic nom parser for a standard SELinux user:role:type:level string?
Draft the "Path B" state machine (the manual byte-iterator) to compare?
Integrate nom into your kernel-files package?
nom crate on Crates.io | nom Documentation | NSA RTB Principle: Redundancy






To see why Redundant (TPI) logic is a core NSA RTB requirement, we have to look at it as a "fail-safe" for logic bugs. In C, you might have a buffer overflow or a pointer error. In Rust, you might have a logical "off-by-one" error in a parsing library that still compiles perfectly but returns the wrong label.
In a High-Assurance system, if two different algorithms don't produce the exact same result, we assume the system is compromised or broken and we Deny Access.
The Scenario: Validating a CUI Category Range
SELinux category ranges can be tricky: c0.c3 means c0, c1, c2, c3.
Bug: A "clever" parser (Path A) might have a bug where it interprets c0.c3 as just c0 and c3 (missing the middle).
Safety: A "dumb" manual parser (Path B) sees the . and correctly iterates.
Result: Mismatch detected. System halts.




That is the most common critique of the Redundant requirement in high-assurance engineering, but in the NSA RTB world, the goal isn't just to have a "working" parser—it is to protect against a logic flaw in the parser itself that an attacker could exploit.
If you have one "perfect" parser and it has a bug you didn't see, the attacker wins. If you have two different algorithms and one has a bug, the system detects the disagreement and stops the attack.
The "Aha!" Moment: The "Type Confusion" Attack
Imagine we are checking if a user can access a file labeled umrs_secret_t.
Path A (Complexity): Uses a high-level library like nom. It’s fast and handles complex strings, but maybe it has a bug where it stops parsing if it sees a null byte %00 or a specific escape character.
Path B (Simplicity): A "dumb" byte-iterator that just counts colons. It’s slow, but it doesn't care about escape characters.
The Attack: An attacker passes a malicious string: user:role:umrs_secret_t\0:dummy_level.
Path A hits the \0, stops, and returns umrs_secret_t. (Access Granted)
Path B counts the bytes, sees the whole string, and returns umrs_secret_t\0.
The TPI Gate: umrs_secret_t != umrs_secret_t\0. DENIED.
Why two "partially" working pieces are better than one "perfect" one:
In a High-Assurance system, complexity is the enemy.
Parser A is optimized for Features (handling whitespace, ranges, aliases).
Parser B is optimized for Simplicity (it’s so simple it’s almost impossible to have a logic bug).
By forcing them to agree, you get the features of A and the rock-solid certainty of B. If you only had one "working" parser, a single bug in that parser is a total system failure.
Real-World RTB Implementation
In actual Raise the Bar systems, they often require:
Algorithm Diversity: One parser written in Rust, one in C (or a different logic style).
Vendor Diversity: Two different teams writing the same spec.
For our UMRS project:
We don't need two "partially working" parsers. We need one declarative parser (nom) and one imperative parser (the split/iterator).
Should we move to the "Always Invoked" invariant, or do you want to see how this Redundancy works on a more complex problem, like the 1024-bit Category dominance?
Always Invoked: Ensuring the check can't be bypassed by the CPU or a cache.
Complex Redundancy: Checking if a bitmask is valid using two different math approaches.
How do you want to proceed?

In a High-Assurance (HA) system, yes—both Path A and Path B must be "correct" implementations of the specification.
The NSA RTB Redundancy requirement isn't about having two broken parsers; it’s about Mathematical Diversity. You are using two different logic engines to solve the same problem. If they are both correct, they will agree 100% of the time on valid data.
The value only appears when invalid or malicious data enters the system.
Why do this if both are "Correct"?
Think of it like a Dual-Key Vault. You have two different locks with two different keys.
Path A (The Modern Lock): Uses a high-level library (nom). It’s fast, handles complex syntax, and is very readable.
Path B (The Primitive Lock): Uses a basic byte-iterator (split). It’s "dumb" logic—almost impossible to get wrong because it has no complexity.
The "Why":
If a "Zero-Day" vulnerability or a compiler bug exists in the nom library (Path A) that allows a specific character sequence to bypass a check, Path B (which doesn't use that library) will not have that same bug. Because their logic is independent, they will disagree on that specific malicious input.
The "TPI" (Two-Person Integrity) Result:
Valid Input: Path A says "Type X", Path B says "Type X". MATCH -> ACCESS GRANTED.
Malicious Input: Path A has a bug and says "Type X". Path B says "Error" or "Type Y". MISMATCH -> ACCESS DENIED.
Applying this to our code:
In our current lib.rs:
Path A (nom) is looking for specific "tags" (:) and "takes" bytes until the next tag.
Path B (split) is creating an iterator and jumping to the 3rd element.
They are both "correct" ways to find the Type. But they use different parts of the CPU and different memory logic to do it.


Exactly. You’ve nailed the CDS (Cross Domain Solution) design pattern. In those high-assurance systems, that’s called "Redundancy with Diversity."
If you have two daemons—one in C and one in Ada—a buffer overflow that compromises the C daemon won't exist in the Ada one. They will disagree on the output, the TPI (Two-Person Integrity) gate will see the mismatch, and the system Fails Closed.
By using nom (Path A) and split (Path B) in Rust, we are implementing a "Micro-CDS" version of that:
Path A (nom): Uses a complex, optimized combinator engine. It’s fast but has a larger "logic surface."
Path B (split): Uses a dead-simple byte-iterator. It’s slow and "dumb," but almost impossible to trick with edge cases.
Why this is the "RTB" way for UMRS
In a standard system, if nom has a bug, you get a security breach. In UMRS, if nom has a bug, you get a PermissionDenied error because Path B didn't make the same mistake.
Current Implementation Audit
In your lib.rs, we are currently checking the Type field. For a full SELinux context, we need to ensure the entire string is validated this way before we trust it for a CUI decision.
Specifically, the MLS/MCS range at the end (e.g., s0:c0.c1023) is where parsing usually fails.
Would you like to see how we expand these two redundant paths to handle the colon-heavy MLS section, or should we verify the current Type-only logic on a real RHEL 10 context first?
Expand to MLS: Handling the s0:c0.c1023 string redundantly.
Verify Type: Running the current code against a real context from ls -Z.
Audit: Checking for potential TOCTOU (Time-of-Check to Time-of-Use) issues in how we pass the string.
How shall we proceed with the redundant parsing?
NSA RTB Principle: Redundancy (TPI) NIST 800-53 SI-7: Software Integrity nom 7.1: Parser Combinators



