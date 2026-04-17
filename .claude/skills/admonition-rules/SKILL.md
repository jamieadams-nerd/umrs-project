---
name: admonition-rules
description: >
  MIL-STD-38784B adapted admonition hierarchy for UMRS documentation: WARNING,
  CAUTION, IMPORTANT, NOTE, and TIP with decision rules for correct level
  selection. Use this skill when writing or editing .adoc documentation files
  under docs/. Trigger when the user or agent mentions WARNING, CAUTION, NOTE,
  TIP, IMPORTANT, admonition, or when adding advisory blocks to AsciiDoc pages.
  Also trigger when reviewing documentation for correct admonition usage.
---

## Admonition Hierarchy Rule (MIL-STD-38784B adapted for software)

**Scope:** This rule applies to ALL UMRS documentation — every `.adoc` file, every module,
every document type. It is not gated behind STE mode or any other writing mode. Every agent
that writes or edits documentation must follow this hierarchy unconditionally.

### Hierarchy

The MIL-STD-38784B WARNING/CAUTION/NOTE hierarchy was designed for physical equipment.
This project adapts it for software documentation. Choose the correct level based on the
consequence of ignoring the admonition:

| Level | Use when | Software equivalent of MIL-STD meaning |
|---|---|---|
| **WARNING** | Security breach risk, data loss, or system damage (unbootable, corrupted state) | Physical injury / equipment damage |
| **CAUTION** | Recoverable degradation, service disruption, misleading output, or compliance gap | Equipment damage (repairable) |
| **IMPORTANT** | Action required, prerequisite, or critical sequencing step that must not be skipped | — (UMRS addition) |
| **NOTE** | Supplementary context, clarification, project status advisory | Supplementary information |
| **TIP** | Optional improvement, convenience shortcut, or best practice | — (UMRS addition) |

### Decision Rules

- If ignoring it causes a **security vulnerability or makes the system unbootable** → WARNING
- If ignoring it causes **wrong output, service restart, or a gap an auditor would flag** → CAUTION
- If ignoring it causes **a procedure to fail or produce errors** → IMPORTANT
- If it provides **context that helps understanding but skipping it does not break anything** → NOTE
- If it is an **optional shortcut or nice-to-know** → TIP

### Examples

```asciidoc
WARNING: In enforce mode, a bad IMA rule order makes the system unbootable.

CAUTION: Layer 1 results without Layer 2 checks produce misleading output.

IMPORTANT: Complete IMA/EVM setup before disabling module loading.

NOTE: The UMRS project is still actively developing this feature.

TIP: Use `sestatus -v` for a quick summary of the current SELinux state.
```

### Syntax

Use AsciiDoc admonition syntax only. Place admonitions adjacent to the step or paragraph
they apply to, not at the end of a section.

Do not use informal labels such as `Note:`, `Warning:`, or `IMPORTANT —` in plain Markdown
style. This project uses Antora. Use the block admonition form.
