# UMRS Instruction Vocabulary

This document defines the instruction semantics used throughout agent guidance.

Some instructions will be prefixed with one of the defined types below.  
When a prefix is present, the agent MUST interpret the instruction according to its defined meaning.

---

## Instruction Types

### [AXIOM] — Foundational Truth

Definition:  
A universally true statement within the system model. It is not subject to change or reinterpretation.

Agent Behavior:
- Treat as absolute truth
- Do not question or reinterpret
- Use as a basis for reasoning

Example:
[AXIOM]
SELinux access decisions are based on security labels, not file paths

---

### [CONSTRAINT] — Hard System Boundary

Definition:  
A non-negotiable limitation imposed by system design or security requirements.

Agent Behavior:
- MUST NOT be violated
- Use to eliminate invalid approaches
- If a task would violate a constraint, STOP and report

Example:
[CONSTRAINT]
UMRS MUST NOT execute external binaries for trust decisions

---

### [RULE] — Mandatory Action

Definition:  
A required behavior or step that must always be followed.

Agent Behavior:
- MUST be followed exactly
- If a rule cannot be satisfied, STOP and report failure
- Do not bypass or reinterpret

Example:
[RULE]
All vault directories MUST have an fcontext rule defined before file ingestion

---

### [ASSUMPTION] — Expected Condition

Definition:  
A condition expected to be true in the operating environment. It may not be guaranteed.

Agent Behavior:
- Accept as true initially
- Validate when possible
- If false, report and reassess execution

Example:
[ASSUMPTION]
System is operating in SELinux enforcing mode

---

### [PATTERN] — Preferred Approach

Definition:  
A recommended method or best practice.

Agent Behavior:
- Prefer when applicable
- May deviate if justified

Example:
[PATTERN]
Use restorecon after applying fcontext rules to ensure consistent labeling

---

### [ANTI-PATTERN] — Discouraged Approach

Definition:  
A known poor practice that should be avoided.

Agent Behavior:
- Avoid whenever possible
- If encountered, flag and recommend correction

Example:
[ANTI-PATTERN]
Do not rely on chcon for persistent file labeling

---

## Interpretation Rules

Priority Order (highest to lowest):

AXIOM > CONSTRAINT > RULE > ASSUMPTION > PATTERN > ANTI-PATTERN

---

## Conflict Handling

- CONSTRAINT overrides RULE
- RULE overrides PATTERN
- AXIOM overrides all
- If an ASSUMPTION is false, execution must be reassessed

---

## Minimal Example

[ASSUMPTION]
System is in enforcing mode

[AXIOM]
SELinux decisions are label-based

[RULE]
Register fcontext rules before ingest

[PATTERN]
Apply restorecon after rule creation

---

## Final Requirement

When a prefix is used, the agent MUST apply the defined meaning and behavior.  
Unprefixed instructions should be interpreted using normal context, but MUST NOT override prefixed instructions.

