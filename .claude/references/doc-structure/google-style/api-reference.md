# API Reference Code Comments — Google Developer Documentation Style Guide

Source: https://developers.google.com/style/api-reference-comments
Retrieved: 2026-03-12

## Overview

This guide provides standards for documenting APIs through source code comments. Complete API references must include descriptions for all public classes, methods, constants, and members.

## Documentation Requirements

**Essential elements** that require descriptions:

- Every class, interface, struct, and similar API members
- All constants, fields, enums, and typedefs
- Every method with parameter, return value, and exception documentation

**Strongly recommended practices:**

- Include a 5-20 line code sample at the top of each unique page
- Format all API names, classes, methods, and parameters in code font with links
- Enclose string literals in double quotation marks (e.g., `"wrap_content"`)
- Match class name spelling exactly as it appears in code
- Avoid pluralizing class names; instead use "plural noun" format (e.g., "Intent objects")

## Class Documentation

Begin with a single sentence stating the class's intended purpose—information not evident from the name alone. Avoid:

- Repeating the class name
- Phrases like "this class will/does"
- Abbreviations with periods that could prematurely terminate summaries

**Example:** "A primary toolbar within the activity that may display the activity title..."

## Method Documentation

### Description Guidelines

Start with a verb describing what the method performs:

- **Boolean getters:** "Checks whether..."
- **Non-boolean getters:** "Gets the..."
- **Operations with data:** "Adds a new [item] and returns the..."
- **Setters/updates:** "Sets the...", "Updates the..."
- **Deletions:** "Deletes the..."
- **Callbacks:** "Called by..." followed by implementation guidance

### Parameters

- Capitalize first word; end with period
- Begin non-booleans with "The" or "A"
- For booleans: state behavior for both true and false conditions
- Specify defaults using format: "Default:"

### Return Values

Keep descriptions brief:

- Non-booleans: "The [description]"
- Booleans: "True if...; false otherwise"

### Exceptions

- With auto-inserted "Throws": Begin with "If..."
- Without auto-insertion: Begin with "Thrown when..."

## Deprecation Guidelines

Always specify the replacement. Include the first deprecation version if tracked.

**Examples:**
- "Deprecated. Use #CameraPose instead."
- "Deprecated. Access this field using the `getField` method."

Explain why deprecation occurred and guide developers on updating their code.
