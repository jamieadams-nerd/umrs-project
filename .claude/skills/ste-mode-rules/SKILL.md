---
name: ste-mode-rules
description: >
  Simplified Technical English (STE) rules for writing numbered procedure steps in UMRS
  documentation. Use this skill when working with STE, Simplified Technical English,
  procedure steps, numbered steps, step-by-step content, deployment procedures, installation
  steps, or French procedures. Trigger when writing or editing numbered procedural content
  in .adoc files, operational runbooks, troubleshooting sequences, or build instructions.
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
admonition hierarchy applies to **all UMRS documentation**. Every admonition in every `.adoc`
file must follow the hierarchy defined in `.claude/rules/admonition_hierarchy.md`.

See `rules/admonition_hierarchy.md` for the full hierarchy table, decision rules, and examples.

Use AsciiDoc admonition syntax for warnings, notes, cautions, and important notices. Place
admonitions adjacent to the step or paragraph they apply to, not at the end of a section.

Do not use informal labels such as `Note:`, `Warning:`, or `IMPORTANT —` in plain Markdown
style. This project uses Antora. Use the block admonition form.

## Canadian French Procedural Rules

The structural rules above (sentence length, one action per step, active voice, consistent
terminology, admonition hierarchy) apply equally to French procedural content. The following
French-specific conventions supplement them.

**Authority:** TBS Canada.ca Content Style Guide (Guide de rédaction du contenu du site
Canada.ca) + Bureau de la traduction conventions. Vocabulary: TERMIUM Plus (primary),
OQLF GDT (fallback).

### Mood conventions

- **Headings and subheadings:** use infinitive mood ("Configurer SELinux", not "Configurez SELinux").
  This is TBS policy (section 5.1.1 of the French guide), not a preference.
- **Numbered procedure steps within lists:** use imperative mood consistently
  ("Ouvrez le fichier", "Exécutez la commande").

### Address and register

- Address users with "vous" (formal), never "tu".
- Follow epicene writing conventions per Bureau de la traduction ("rédaction épicène"),
  not OQLF's "écriture inclusive."

### Number and date formatting

- Currency: `100 $` (space before symbol, symbol after number)
- Percentage: `20 %` (space before percent sign)
- Time: `16 h 30` (spaces around `h`)
- Dates: `31 juillet 2016` (day month year, month lowercase)

### Approved French verb list

A French approved verb list parallel to the English list must be sourced from TERMIUM Plus
translations. Until the full list is compiled (Simone builds, Henri validates), use the
TERMIUM Plus translation of each English approved verb. When TERMIUM offers multiple
translations, prefer the shortest and most direct form.

### Ambiguous words to avoid (French)

Apply the same principle as the English list — avoid hedging and vague terms:

```
plusieurs (use a specific number)
pourrait / devrait (use "doit" for mandatory, state the condition for optional)
approprié / adéquat (state the specific requirement)
éventuellement (ambiguous in fr_CA — means "possibly", not "eventually")
```

## Activation

STE mode is always active when writing or editing numbered procedure steps in any `.adoc`
file. There is no explicit on/off switch. If the content is a numbered procedure, these
rules apply. If it is conceptual or explanatory text, they do not.

## Agent Behavior

When writing or editing numbered procedure steps:

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
