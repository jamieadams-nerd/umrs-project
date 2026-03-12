# Procedures — Google Developer Documentation Style Guide

Source: https://developers.google.com/style/procedures
Retrieved: 2026-03-12

## Overview

A procedure is a sequence of numbered steps for accomplishing a task.

## Introductory Sentences

Most procedures should begin with an introductory sentence providing context beyond the section heading. Avoid simply repeating the heading.

**Sentence ending guidance:**
- Use a colon if the introduction immediately precedes the numbered steps
- Use a period if other material (like notes) appears between introduction and steps

**Recommended approaches:**
- "To customize the buttons, follow these steps:"
- "Customize the buttons:"
- "To customize the buttons, do the following:"

**Avoid:** "To customize the buttons:" (incomplete sentence)

## Single-Step Procedures

When a procedure contains only one step, format it as a bulleted list item rather than a numbered list.

**Recommended:** "To clear the entire log, click **Clear logcat**."

## Sub-Steps in Numbered Procedures

In numbered procedures:
- Sub-steps use lowercase letters (a, b, c)
- Sub-sub-steps use lowercase Roman numerals (i, ii, iii)

**Example structure:**
```
1. To add a VM instance, do the following:
   a. Click Create instance.
   b. For Name, enter a name, then do the following:
      i. For Region, specify deployment location.
      ii. For Machine type, select an option.
   c. Click Create.
```

## Order of Multiple Components in a Step

For complex steps, follow this sequence:

1. Describe the action
2. List the command (if needed)
3. Explain placeholders used in the command
4. Provide additional command details (if needed)
5. List command output (if needed)
6. In a separate paragraph, explain the result or additional output

## Multiple Procedures for the Same Task

Document one accessible procedure when possible. If multiple methods exist, prioritize them by:

- Keyboard-only accessibility
- Shortest procedure length
- Audience familiarity with programming language used

Separate different methods using different pages, headings, or tabs rather than listing alternatives.

## Steps with Location Context

State where the action occurs before describing the action.

**Recommended:**
- "In Google Docs, click **File > New > Document**."

**Avoid:** "Click **File > New > Document** in Google Docs."

## Steps with Goals

State the goal or purpose before the action when helpful for clarity.

**Recommended:** "To start a new document, click **File > New > Document**."

**Avoid:** "Click **File > New > Document** to start a new document."

## Optional Steps

Mark optional steps with "Optional:" at the beginning.

**Recommended:** "Optional: Type an arbitrary string..."

**Avoid:** "(Optional) Type an arbitrary string..."

## Key Guidelines Summary

| Guidance | Recommended | Not Recommended |
|----------|-------------|-----------------|
| Imperative verbs | "Clone the repository containing sample data." | "You need the project ID later. Retrieve it." |
| Optional formatting | "Optional: Type..." | "(Optional) Type..." |
| Context placement | "In Cloud Shell, connect to cluster." | Mention tool after the action |
| Purpose statement | "To start a new document, click..." | "Click... to start a new document" |
| "Please" | Omit from instructions | "Please click File > Open" |
| Command introduction | "Deploy the load generator:" | "Run the following command:" |
