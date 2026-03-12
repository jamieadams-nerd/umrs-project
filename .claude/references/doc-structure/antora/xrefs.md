# Cross-References (Xrefs) in Antora

Source: https://docs.antora.org/antora/latest/page/xref/
Retrieved: 2026-03-12

## Overview

Cross-references (xrefs) enable linking between pages in Antora documentation. An xref macro converts source file references into published URL links during the build process.

## Xref Macro Structure

"An xref macro begins with the macro's name, `xref`, followed by a single colon (`:`)"

The basic components are:
- **Macro prefix**: `xref:`
- **Resource ID**: The target page's identifier
- **Fragment** (optional): Element IDs for deep links, starting with `#`
- **Square brackets**: `[]` containing optional link text

## Creating Page Links

To create a cross-reference:

1. Open the current page where you want the link to appear
2. Enter `xref:` at the desired location in your content
3. Assign the target page's resource ID
4. Add square brackets with optional link text: `[link text]`

### Example

For pages in the same component version and module, only the filename is needed:

```
See the xref:modes.adoc[] for more options.
```

## Key Concepts

**Target page**: The source file being referenced by another page.

**Current page**: The source file containing the xref macro.

**Resource ID coordinates**: When the target page belongs to a different module, component, or version, additional coordinates must be specified.

**Default reference text**: If no link text is provided and no fragment is used, the target page's reference text displays automatically.

**Fragment**: A "deep link" to specific sections within the target page using the `#` symbol.

## Important Note

"Always prefer the xref macro to make references to resources outside of the current page" rather than using shorthand xref syntax, as this clarifies distinctions between internal and external references.
