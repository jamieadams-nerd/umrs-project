---
name: Internal type names in operator display
description: Rust enum variant names appear verbatim in operator-visible display strings, requiring UMRS internals knowledge to interpret
type: project
---

Observed in umrs-tui (2026-03-20): LabelClaim, UntrustedLabelCandidate, TrustedLabel,
IntegrityVerifiedButContradictory appear as the leading text in the Label Trust field.
Trust tier descriptions use PROC_SUPER_MAGIC and "mount topology" language.

**Why:** Display strings are built directly from the Rust match arms without a separate
operator-facing label mapping.

**How to apply:** When reviewing any display string that originates from an enum match,
check whether the displayed text is a code artifact (variant name, internal constant,
internal term) or genuine operator vocabulary. Flag MEDIUM if the operator would need
to read source or design docs to interpret it.
