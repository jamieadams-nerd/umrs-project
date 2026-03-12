# Documentation Topic Types (CTRT) — GitLab

Source: https://docs.gitlab.com/development/documentation/topic_types/
Retrieved: 2026-03-12

## Primary Topic Types

GitLab documentation organizes content into four main topic types:

- **Concept** - Explanatory content that provides understanding
- **Task** - Procedural content focused on how to accomplish goals
- **Reference** - Factual information for lookup and consultation
- **Troubleshooting** - Solutions for common problems and issues

The acronym "CTRT" represents the first letter of each type.

## Page Structure Guidelines

"Even if a page is short, the page usually starts with a concept and then includes a task or reference topic." Pages should combine multiple topic types rather than stand alone.

## Additional Page Types

Beyond the core four, GitLab supports:

- Tutorial
- Get started
- Top-level pages
- Prompt examples
- Glossaries

## Topic Title Best Practices

Effective topic titles should:

- Be "clear and direct. Make every word count"
- Stay under 70 characters when feasible
- Include articles and prepositions
- Follow capitalization guidelines
- Avoid repeating text from earlier headings
- Skip hyphens for separating information

## Related Topics Section

Supplementary content can use a "Related topics" section with brief, scannable link text formatted as an unordered list without periods.

## Content to Avoid

Documentation should not consist solely of links to other pages (except top-level navigation pages) or contain single-sentence topics.

---

## Concept Topic Type

Source: https://docs.gitlab.com/development/documentation/topic_types/concept/

A concept introduces a single feature or concept and should answer two key questions:
- **What** is this?
- **Why** would you use it?

Concept topics provide foundational understanding for those unfamiliar with a subject, focusing on explanation rather than instruction.

### Key Principles

**Avoid including:**
- Instructions on how to use something (that belongs in task topics)
- Links to related tasks (navigation handles this)
- Descriptions of other concepts (create separate concept pages instead)

### Format Guidelines

```
title: Title (a noun, like "Widgets")
---

A paragraph or two explaining what this thing is and why you would use it.

Focus on one concept only.
```

### Title Best Practices

Use nouns for titles. Examples include:
- `Widgets`
- `GDK dependency management`

Prefer noun forms ending in `-ion` over `-ing` forms (e.g., "Object migration" instead of "Migrating objects"). This approach aids translation and aligns with technical writing standards.

**Titles to Avoid:**
- "Overview" or "Introduction" - use specific searchable nouns instead
- "Use cases" - incorporate this information within the concept itself
- "How it works" - use "noun + workflow" format (e.g., "Merge request workflow")

---

## Task Topic Type

Source: https://docs.gitlab.com/development/documentation/topic_types/task/

A task provides step-by-step instructions for completing a procedure in GitLab.

### Format Structure

Tasks follow this pattern:
- **Title**: Begins with an active verb (e.g., "Create a widget")
- **Introduction**: Explains when to use the task
- **Prerequisites**: Lists required conditions (optional)
- **Steps**: Numbered instructions with location then action
- **Result/Next steps**: Describes outcome (optional)

### Title Guidelines

Use the format: "active verb + noun" (e.g., "Create an issue").

### Single-Step Tasks

When a task has only one step, present it as an unordered list item.

### Multiple Methods

If several ways exist to perform a task, document the primary method. When multiple approaches must be included:
- Add topic headings nested under the main task
- List methods in descending order by likelihood
- Order steps from most to least common

### Prerequisites

- Use "Prerequisites" (always plural)
- List applicable user roles first
- Avoid phrases like "Ensure that"
