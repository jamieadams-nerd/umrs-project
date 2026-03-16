---
name: ste_mode
description: "Apply STE (Simplified Technical English) rules when writing or updating numbered, step-by-step procedural content — installation steps, configuration procedures, troubleshooting sequences, build steps, and test instructions. Applies regardless of audience (operators, developers, auditors). Does NOT apply to introductory, conceptual, or explanatory paragraphs in the same document."
---

# STE Mode: Simplified Technical English

## Purpose

This mode defines the writing rules for step-by-step procedural content in UMRS documentation.
STE rules apply only to numbered procedure blocks. Introductory paragraphs, conceptual explanations,
and rationale sections in the same document use normal writing style.

Use this mode for:

- deployment and installation procedures
- configuration sequences
- troubleshooting steps
- build and test instructions
- operational runbooks

Do not apply STE rules to design discussions, architecture overviews, or conceptual explanations.

## Scope

STE governs the numbered steps in a procedure block. It does not govern:

- introductory paragraphs before a procedure
- explanatory notes between procedures
- conceptual or rationale sections
- table descriptions, section headers, or captions

Example of scope boundary:

> SELinux enforces mandatory access control on every process. The policy is loaded at boot time
> and evaluated on every system call. The following procedure configures the enforcement mode.
>
> 1. Open `/etc/selinux/config`.
> 2. Set `SELINUX=enforcing`.
> 3. Save the file.
> 4. Reboot the system.

The paragraph above the numbered list uses normal style. The numbered list uses STE rules.

## Short Sentences

Maximum sentence length in a procedure step: 20 words.

Prefer 10–15 words per step.

Split long steps into two steps.

Incorrect:

> Before performing maintenance operations, ensure that the electrical power source has been
> disconnected from the system.

Correct:

```
1. Switch off electrical power.
2. Disconnect the power source.
```

## Procedure Format

Use numbered steps. Each step contains one action. State the expected outcome after steps that
produce visible output.

Incorrect:

> Remove the cover and inspect the wiring.

Correct:

```
1. Remove the cover.
2. Inspect the wiring.
```

Example with expected outcome:

```
1. Run the following command:

   $ sestatus

   The output shows `SELinux status: enabled`.
```

## Active Voice

Use active voice in procedure steps.

Incorrect:

> The configuration file must be edited before the service is started.

Correct:

```
1. Edit the configuration file.
2. Start the service.
```

## Approved Technical Verbs

Prefer simple, direct verbs. Use the approved list.

```
audit
check
configure
connect
deny
disable
disconnect
enable
enforce
install
label
load
mount
query
relabel
remove
restart
set
start
stop
unload
unmount
validate
verify
```

Avoid:

```
facilitate
leverage
perform
utilize
```

## Avoid Ambiguous Words

Do not use:

```
appropriate
may
might
proper
several
various
```

Write clear, definite statements.

Incorrect:

> The service should start automatically.

Correct:

> The service starts automatically.

## Consistent Terminology

Use the same term for the same concept throughout a document.

Correct — one term, used consistently:

> kernel module
> kernel module loading
> kernel module unloading

Incorrect — multiple terms for the same concept:

> driver
> module
> kernel extension

When a project-specific term requires explanation, define it inline before the first step that
uses it. Example:

> A security context is a colon-delimited string in the form `user:role:type:level` that SELinux
> attaches to every process and file. The following procedure reads the security context of a file.
>
> 1. Run `ls -Z /etc/shadow`.

## AsciiDoc Admonitions

**Scope:** Unlike the other STE rules (which apply only to numbered procedure steps), the
admonition hierarchy below applies to **all UMRS documentation** — architecture, reference,
developer guides, deployment, operations, patterns, and procedures alike. Every admonition
in every `.adoc` file must follow this hierarchy.

Use AsciiDoc admonition syntax for warnings, notes, cautions, and important notices. Place
admonitions adjacent to the step or paragraph they apply to, not at the end of a section.

### Admonition Hierarchy (MIL-STD-38784B adapted for software)

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

**Decision rules:**

- If ignoring it causes a **security vulnerability or makes the system unbootable** → WARNING
- If ignoring it causes **wrong output, service restart, or a gap an auditor would flag** → CAUTION
- If ignoring it causes **a procedure to fail or produce errors** → IMPORTANT
- If it provides **context that helps understanding but skipping it does not break anything** → NOTE
- If it is an **optional shortcut or nice-to-know** → TIP

```asciidoc
WARNING: In enforce mode, a bad IMA rule order makes the system unbootable.

CAUTION: Layer 1 results without Layer 2 checks produce misleading output.

IMPORTANT: Complete IMA/EVM setup before disabling module loading.

NOTE: The UMRS project is still actively developing this feature.

TIP: Use `sestatus -v` for a quick summary of the current SELinux state.
```

Do not use informal labels such as `Note:`, `Warning:`, or `IMPORTANT —` in plain Markdown
style. This project uses Antora. Use the block admonition form.

## Agent Behavior

When STE mode is active:

1. Apply STE rules to numbered procedure steps only. Explanatory and conceptual text in the
   same document is not affected.

2. When updating an existing document that was not written in STE, rewrite all existing
   numbered step content to comply with these rules. Do not limit STE rewrites to new additions.

3. When a step cannot be split without losing technical precision, keep it intact. Add a note
   immediately after the step explaining why it was not split.

4. When a step uses a project-specific term that requires a longer definition, define it inline
   before the first step in the procedure that uses it.

5. Replace vague verbs with approved technical verbs.

6. Remove unnecessary adjectives from procedure steps.

7. Use AsciiDoc admonition syntax for all warnings, notes, cautions, and important notices.
