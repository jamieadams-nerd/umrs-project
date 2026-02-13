# Rationale for a Strongly-Typed SELinux Modeling Library

Traditional SELinux tooling and userland integrations frequently represent
security contexts and labels as unstructured strings. While this approach is
flexible, it introduces significant risk in high-assurance systems where
correctness, determinism, and policy fidelity are critical.

A strongly-typed SELinux modeling library replaces string representations with
validated, domain-specific data types. Each component of a security context is
modeled as an independent primitive with explicit construction rules and
behavioral semantics aligned to SELinux policy logic.

The objective is not abstraction for its own sake, but the elimination of
entire classes of labeling, parsing, and enforcement errors before they can
manifest operationally.

More percisely, the objective of this library is to provide strongly-typed data models
representing SELinux constructs so that userland applications can interact with
the SELinux ecosystem more precisely, safely, and deterministically.

This includes modeling:

* Security contexts
* Sensitivity levels
* Category sets
* MLS levels and ranges
* Users, roles, and types

The library improves how software:

* Constructs labels
* Parses labels
* Validates label components
* Performs dominance math
* Pre-validates context transitions
* Generates policy-aware artifacts

before interacting with the kernel.

Where feasible, this modeling layer operates as an alternative interface to
traditional SELinux userland libraries, including:

* libselinux
* libsepol
* libsemanage


## Operational Value Proposition
By modeling SELinux constructs strongly in userland, systems gain:

* Compile-time validation of label structures
* Deterministic category math
* Safer context construction
* Reduced runtime labeling errors
* Improved audit and tooling fidelity

This enhances interaction with the SELinux subsystem without altering its enforcement core.

## Scope and Non-Interference Statement
This project operates strictly in **userland**.

This library does **not** modify, replace, recompile, or interfere with SELinux
kernel code, kernel policy engines, or mandatory access control enforcement
mechanisms in any way.

All SELinux enforcement logic remains entirely within the Linux kernel security subsystem 
exactly as implemented upstream. Including:

* Access Vector Cache (AVC) decisions
* Policy rule evaluation
* MLS/MCS dominance enforcement
* Type enforcement
* Constraint evaluation
* Kernel labeling operations

The kernel remains the sole authority for:
* Security context validation
* Policy enforcement
* Label transition decisions
* Access control outcomes



## 2. Security Context Decomposition 

A **security context** in SELinux is the
full identity label assigned to a subject (like a process) or an object (like a
file). It defines the security attributes that SELinux uses to make access
control decisions.

A context typically includes:

- **User** — the SELinux identity associated with the subject or object
- **Role** — the functional domain the subject operates within
- **Type** — the primary enforcement attribute that defines what interactions are allowed
- **Sensitivity level** — the classification level (e.g., s0, s1, s2…)
- **Categories** — optional compartment groupings for MLS/MCS isolation

So instead of just asking “Does this process have permission?”, SELinux evaluates:

- Who is the process?
- What role is it operating in?
- What type of resource is it accessing?
- Is its clearance level sufficient?
- Do its categories dominate the target?

All of that information comes from the **security context** — which is why it’s
the core data structure behind SELinux policy enforcement.

A full SELinux security context is composed of:

```
user : role : type : sensitivity : categories
```

A real world example of the `/etc/hosts` file on a system might look like this:
```
system_u:object_r:net_conf_t:SystemLow
```

and my current user's security context is as follows (`id -Z`):
```
unconfined_u:unconfined_r:unconfined_t:SystemLow-s0:c0.c1023
```

In MLS form, the sensitivity and category set together constitute the security
label (or MLS label).

By decomposing this structure into strongly-typed primitives, each field becomes:

- Independently validated
- Explicitly modeled
- Immune to string formatting defects
- Safe for compositional security logic

### 3. Compile-Time Error Elimination

String-based label handling allows invalid states to exist undetected until runtime.

Examples include:

• Invalid suffixes (missing _u, _r, _t)
• Malformed sensitivity identifiers
• Out-of-range categories
• Incorrect delimiter ordering
• Duplicate category declarations

A strongly-typed model prevents these conditions at construction time.

If an object exists, it is valid.

This shifts failure detection from runtime enforcement to compile-time or instantiation-time validation — a critical assurance improvement.

---

### 4. Construction-Time Validation

Each primitive encodes its own policy and structural invariants.

Examples:

SELinux User
• ASCII only
• No whitespace
• Must end in _u
• Length bounded

Role
• ASCII only
• Must end in _r

Type
• ASCII only
• Must end in _t

Sensitivity Level
• Numeric bounded classification
• Policy-aligned range enforcement

Category
• Numeric bounded compartment identifier

This localized validation prevents malformed labels from ever entering system workflows.

---

### 5. Deterministic Category Modeling

MLS category sets are implemented in the kernel as ebitmap structures — sparse bitmaps representing compartment membership.

String representations introduce risks:

• Ordering ambiguity
• Duplicate entries
• Inefficient comparison logic
• Parsing complexity

A strongly-typed CategorySet modeled as a fixed bitmap provides:

• Constant-time membership tests
• Deterministic dominance checks
• Efficient union/intersection operations
• No duplicate category states

This aligns directly with kernel lineage semantics while remaining safe and performant in userland.

---

### 6. Correct Dominance and Clearance Logic

MLS enforcement depends on lattice mathematics:

• Sensitivity dominance
• Category superset relationships
• Clearance range containment

String comparison cannot safely represent these operations.

Strong typing enables explicit security math:

subject_level dominates object_level
subject_categories dominate object_categories

This prevents:

• Lexical comparison errors
• Inverted access logic
• Mishandled empty sets
• Partial dominance miscalculations

---

### 7. Safe Security Context Composition

Constructing contexts via string concatenation is inherently unsafe:

user:role:type:level:categories

Errors may include:

• Missing fields
• Field misordering
• Invalid combinations
• Improper serialization

Strong typing enforces structure at construction:

SecurityContext(user, role, type, level)

Invalid contexts cannot be created, serialized, or enforced.

---

### 8. API Contract Enforcement

Rust’s type system enables additional assurance guarantees:

• must_use prevents ignored security decisions
• const functions enable compile-time modeling
• FromStr enforces validated parsing
• Display ensures canonical serialization

This transforms SELinux handling from text processing into a contract-bound security API.

---

### 9. Policy Evolution Resilience

SELinux environments evolve:

• Category ranges expand
• Sensitivity levels adjust
• Naming rules tighten

Strong typing localizes adaptation:

Validation logic changes once
Consumers remain unaffected
No widespread string rewrites occur

This reduces maintenance cost and policy drift risk.

---

### 10. Memory Safety and Operational Assurance

Rust’s guarantees eliminate classes of vulnerabilities common in legacy label tooling:

• Buffer overflows
• Unsafe string slicing
• Bitmap memory corruption
• Undefined comparison behavior

These properties are especially valuable in:

• Cross-domain solutions
• High-assurance guards
• Auditing pipelines
• Forensic record systems

---

### 11. Kernel Lineage Alignment

The model reflects SELinux conceptual structures without copying implementation:

Kernel Concept → Strongly-Typed Analog

ebitmap → CategorySet
Sensitivity → SensitivityLevel
MLS Level → MlsLevel
Context → SecurityContext

This preserves operator familiarity while modernizing safety guarantees.

---

### 12. Developer and System Benefits

Operational advantages include:

• Compile-time validation
• Deterministic enforcement math
• Reduced parsing complexity
• Improved readability
• Easier testing and fuzzing
• Policy-aligned APIs

Most importantly:

Invalid labels become unrepresentable states.

---

### 13. Strategic Assurance Impact

In high-assurance and regulated environments, labeling errors are not cosmetic — they are security boundary failures.

A strongly-typed SELinux library provides:

• Early failure detection
• Label correctness guarantees
• Enforcement math integrity
• Safer policy integration

This directly supports system accreditation, auditability, and cross-domain trust models.

---

If you want to extend this later, the next natural sections would be:

• Clearance range modeling rationale
• Sensitivity/category lattice diagrams
• Mapping to Bell-LaPadula / Biba
• Guard enforcement examples

But as a foundational justification report, this captures the “why” behind the architecture you’re building.

