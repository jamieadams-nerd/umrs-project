# Rationale for a Strongly-Typed SELinux Modeling Library

## Overview

This library provides a strongly-typed Rust model of SELinux security
context components, including:

- SELinux users
- Roles
- Types
- Sensitivity levels
- Categories
- Category sets
- MLS levels and clearance ranges

Rather than representing SELinux labels as unstructured strings, each
component is modeled as a validated, domain-specific data type.

This design is intentional and driven by correctness, safety, and
assurance requirements common to high-security and policy-driven systems.

---

## 1. Compile-Time Error Prevention

### The problem with string-based labels

Traditional SELinux handling often treats labels as raw strings:
system_u:system_r:secret_t:s3:c0,c4

system_u:system_r:secret_t:s3:c0,c4



This approach permits entire classes of errors that are not detected
until runtime or policy enforcement, including:


- Misspelled suffixes (`system_u` vs `system_user`)
- Invalid category identifiers
- Malformed sensitivity syntax
- Incorrect ordering of label components
- Missing or extra delimiters


These errors are frequently discovered late—during policy load,
access decisions, or system operation.


---


### Strong typing as a solution


By modeling label components as typed values:


```rust
SelinuxUser
SelinuxRole
SelinuxType
SensitivityLevel
Category
CategorySet

invalid values cannot be constructed:

SelinuxUser::new("system")        // invalid: missing `_u`
Category::new(5000)               // invalid: out of range
SensitivityLevel::new(9999)       // invalid: policy bound

Errors are detected immediately at construction time, not downstream
during enforcement.

2. Validation at Construction Time

Each primitive enforces its own validation rules.

Primitive	Validation Enforced
User	ASCII, no whitespace, _u suffix
Role	ASCII, no whitespace, _r suffix
Type	ASCII, no whitespace, _t suffix
Category	Numeric range (e.g., c0–c1023)
SensitivityLevel	Numeric classification bounds

This guarantees a core invariant:

If a value exists, it is valid.

Downstream code can rely on this invariant without re-validating.

3. Elimination of Category Math Errors

MLS and MCS labels rely on category bitmaps for compartmentalization.

String-based handling introduces risks such as:

Duplicate categories

Ordering-dependent comparisons

Parsing ambiguity

Incorrect superset logic

By modeling categories as a fixed bitmap (CategorySet), operations such as
membership, intersection, union, and dominance are:

Deterministic

Constant-time

Immune to string parsing errors

Example:

if subject_categories.dominates(object_categories) {
    // subject has access
}
4. Type-Safe Dominance and Clearance Logic

Security decisions depend on lattice mathematics:

Sensitivity dominance

Category superset relationships

Clearance range containment

Strong typing encodes these relationships explicitly:

if subject_level.dominates(object_level) {
    allow_access();
}

This prevents subtle logic errors such as:

Lexical string comparison instead of numeric ordering

Mishandled empty category sets

Inverted dominance checks

5. Prevention of Context Construction Errors

String-based context construction is error-prone:

format!("{user}:{role}:{type}:{level}")

This allows:

Missing fields

Improper ordering

Invalid combinations

Strongly-typed composition enforces structure:

SecurityContext::new(user, role, type_id, level)

Invalid contexts cannot be instantiated.

6. Compile-Time API Contracts

Rust’s type system and attributes allow additional guarantees:

#[must_use] prevents ignored security checks

const fn enables compile-time label modeling

FromStr ensures validated parsing

Display guarantees canonical serialization

Together, these features turn SELinux handling into a contract-driven API
rather than ad-hoc string manipulation.

7. Policy Evolution Safety

SELinux policies evolve over time:

New categories

Expanded sensitivity ranges

Updated naming constraints

Strong typing localizes change:

Validation rules are updated in one place

Consumers remain unaffected

No widespread string-handling rewrites are required

8. Memory Safety and Determinism

Rust’s ownership and type system ensures:

No buffer overflows

No malformed string slicing

No undefined bitmap behavior

Deterministic label operations

These properties are particularly valuable in:

High-assurance systems

Auditing and forensics tooling

Cross-domain and multi-level security solutions

9. Alignment with Kernel Semantics

The library mirrors SELinux kernel lineage constructs conceptually:

Kernel Concept	Rust Model
ebitmap	CategorySet
Sensitivity	SensitivityLevel
MLS level	MlsLevel
Context	SecurityContext

All implementations are original Rust code aligned at the semantic level
only—no kernel or userland source code has been copied or translated.


10. Improved Developer Ergonomics

Instead of parsing labels:

parse_label("s3:c0,c4")?

developers work with explicit structures:

let mut cats = CategorySet::new();
cats.insert(Category::new(0)?);
cats.insert(Category::new(4)?);


let level = MlsLevel::new(SensitivityLevel::new(3)?, cats);

This improves:

Readability

Debuggability

Static analysis

Testability

Summary

A strongly-typed SELinux modeling library provides:

Compile-time prevention of invalid labels

Deterministic dominance and clearance logic

Elimination of string parsing errors

Policy-aligned construction guarantees

Safer and more expressive APIs

In high-assurance environments, these properties significantly reduce
the risk of mislabeling, improper access decisions, and downstream
policy enforcement failures.
