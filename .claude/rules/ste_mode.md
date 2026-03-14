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

Use AsciiDoc admonition syntax for warnings, notes, cautions, and important notices. Place
admonitions adjacent to the step they apply to, not at the end of the procedure.

```asciidoc
NOTE: Reboot is required for the change to take effect.

WARNING: This action stops the service. Schedule downtime before continuing.

CAUTION: This step is irreversible. Verify the backup before proceeding.

IMPORTANT: Verify the file exists before continuing.
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
