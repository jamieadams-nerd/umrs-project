# Red Hat Modular Documentation Reference Guide

Source: https://redhat-documentation.github.io/modular-docs/
Retrieved: 2026-03-12

## Introduction

The Modular Documentation Reference Guide provides instructions for authoring documentation based on modular structures and user stories. This approach enables writers to create reusable, independently meaningful content chunks that can be combined flexibly.

## Core Concepts

### What Modular Documentation Is

Modular documentation consists of independent, self-contained information units called "modules" that writers combine into larger collections called "assemblies." As the guide explains, "An assembly can also include other assemblies. A module should not contain another module."

The fundamental principle is that each module must deliver standalone value—readers should understand it without additional context, even when reading it separately.

### What Modular Documentation Is Not

The guide clarifies three common misconceptions:

1. **Not fragmented content**: Modules must be meaningful units, not arbitrary text fragments
2. **Not disconnected collections**: Modules require organization through assemblies based on user stories
3. **Not exclusively linear**: Modular content supports flexible delivery—from lean articles to comprehensive books

## Module Types

### Concept Modules

These "understand" modules provide descriptions and explanations users need to comprehend and use a product.

**Structure:**
- Noun-phrase titles
- Concise introductory paragraph
- Descriptive body content (may include lists, tables, graphics)
- Optional additional resources section

**Key guideline**: "Avoid including instructions to perform an action. Action items belong in procedure modules."

### Procedure Modules

These task-oriented modules contain step-by-step instructions for accomplishing a single action.

**Required sections:**
- Title (gerund phrase, e.g., "Creating guided decision tables")
- Introduction with context
- Numbered steps in imperative voice

**Optional sections:**
- Prerequisites (bulleted list, always plural heading)
- Verification/Results
- Troubleshooting steps
- Next steps
- Additional resources

**Important constraint**: Do not add extra subheadings beyond those specified.

### Reference Modules

These provide lookup data users might need but shouldn't memorize—commands, configuration options, default settings.

**Structure:**
- Short introduction explaining the reference material
- Strictly organized body content (alphabetically, by category, or in tables)
- Logical scanning structure for quick information retrieval

**Guidelines**: Organize information using lists or tables for maximum scannability.

## Text Snippets

Snippets are reusable text sections stored separately—not standalone modules. They lack structural module elements like anchor IDs or H1 headings.

**Valid snippet types:**
- Paragraphs or sentence fragments
- Single steps or step sequences
- Tables or lists
- Notes (disclaimers, guidance, warnings)

**Implementation**: Prefix filenames with `snip-` or `snip_`, then use include directives to embed them.

## Assembly Structure

An assembly collects related modules into a larger unit addressing a specific user story.

**Required components:**
- Title (gerund phrase for task-based; noun phrase for reference)
- Introduction explaining the user outcome
- Constituent modules

**Optional sections:**
- Prerequisites applicable to all modules
- Additional resources

**Module inclusion method**: Use AsciiDoc include directives with leveloffset attributes to establish hierarchy:

```
include::file1.adoc[leveloffset=+1]
include::file2.adoc[leveloffset=+2]
```

## File Naming and Anchors

### File Names

Use prefixes to identify content type:
- `con-` or `con_`: Concept modules
- `proc-` or `proc_`: Procedure modules
- `ref-` or `ref_`: Reference modules
- `assembly-` or `assembly_`: Assemblies

Example: `con-guided-decision-tables.adoc` or `proc_creating-guided-decision-tables.adoc`

### Anchors

Every module requires an anchor in the format `[id="filename_{context}"]` where the context variable enables content reuse. "Module anchors are necessary so that Asciidoctor can identify the module when the module is reused or cross-referenced."

Example:
```
[id="guided-decision-tables_{context}"]
= Guided Decision Tables
```

## Module Reuse

When reusing modules across multiple assemblies, define the `:context:` variable immediately above each include statement.

Cross-reference reused modules using the format: `xref:anchor-name_context-variable-name[]`

## Nesting Assemblies

When including assemblies within assemblies, save and restore the context variable to prevent duplicate ID errors:

**At assembly start:**
```
ifdef::context[:parent-context: {context}]
```

**At assembly end:**
```
ifdef::parent-context[:context: {parent-context}]
ifndef::parent-context[:!context:]
```

## Key Terminology

**User Story**: A brief description of something users accomplish to achieve a goal, typically following the pattern "As a [user type], I want [action] so that [benefit]."

**Assembly**: A collection of modules combined with an introduction explaining the assembly's purpose—essentially the documentation realization of a user story.

**Module**: An independent, self-contained information unit with clear title and organization, reusable across multiple contexts.

## Conversion Workflow

Converting existing documentation involves:

1. Understanding modular principles and terminology
2. Identifying top-level user stories aligned with the customer product lifecycle
3. Defining supporting user stories for each lifecycle phase
4. Creating assemblies around validated user stories
5. Building modules to support each assembly
6. Auditing content for consistency and completeness

The customer product lifecycle phases include: Plan, Install, Configure and Verify, Develop and Test, Manage, Monitor and Tune, Upgrade and Migrate, and Troubleshoot.
