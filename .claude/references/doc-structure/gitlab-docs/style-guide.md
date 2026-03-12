# GitLab Documentation Style Guide

Source: https://docs.gitlab.com/development/documentation/styleguide/
Retrieved: 2026-03-12

## Core Documentation Principles

### The GitLab Voice

Documentation should be "concise, direct, and precise" with content that's "easy to search and scan." The tone balances conversational warmth with brevity—friendly yet succinct.

### Single Source of Truth

GitLab documentation functions as the authoritative reference for product implementation, usage, and troubleshooting. This approach prevents information silos and enables users to find comprehensive product details in one location.

### Docs-First Methodology

Contributors should prioritize adding information to documentation rather than repeating it elsewhere. When questions arise, link to existing documentation. For missing information, create a merge request to add it.

## Language Standards

### Writing Principles

- **Active voice**: Prefer active construction for clarity and translatability
- **Customer perspective**: Focus on user benefits rather than what GitLab created
- **Trust-building**: Avoid marketing language like "easily" or "simply"; use facts and specifics instead
- **Clarity**: Eliminate unnecessary words while maintaining completeness

### Grammar Requirements

- Write in US English with US grammar conventions
- Use contractions for friendly tone in tutorials and UI text
- Avoid contractions with proper nouns (write "Terraform is," not "Terraform's")
- Spell out numbers zero through nine; use digits for 10+
- Format dates as "month day, year" (e.g., January 3, 2026)

### Localization Considerations

Documentation should support global audiences through:

- Clear, direct language suitable for machine translation
- Avoiding phrases with hidden subjects ("there is/are")
- Eliminating ambiguous pronouns
- Using lists and tables instead of complex sentences
- Maintaining consistent feature naming and terminology

## Markdown and Technical Formatting

### Basic Markdown

All documentation uses Markdown processed by Hugo's Goldmark engine. Content is tested with markdownlint and Vale for consistency.

**Structural Guidelines:**
- Insert blank lines between paragraphs and different markup types
- Split long lines at approximately 100 characters for readability
- Start each new sentence on a new line
- Use HTML comments for author notes (not to hide content)

### Heading Structure

- Include a `title` attribute in metadata; this becomes the H1
- Don't add an H1 in Markdown
- Increment heading levels sequentially (no skipping levels)
- Limit headings to H5 maximum; move complex content to new pages
- Use sentence case for topic titles
- Leave blank lines before and after headings

### Code Formatting

**Inline Code**: Use single backticks for filenames, parameters, keywords, short inputs/outputs, and UI text entries.

**Code Blocks**: Use triple backticks with syntax highlighting.

**Keyboard Commands**: Use the `<kbd>` tag. Spell out full key names (Shift, Command, Control).

## Text Formatting

### Bold Text

Use bold for:
- UI element labels (buttons, checkboxes, settings, menus, pages, tabs)
- Navigation paths

### Lists

**Unordered Lists**: Use dashes (`-`), not asterisks. Leave blank lines before and after.

**Ordered Lists**: Use `1.` for all items (Hugo renders sequentially). Use for procedures requiring specific sequence.

**List Guidelines:**
- Make all items parallel in structure
- Capitalize first word of each item
- Apply consistent punctuation
- Use periods only for complete sentences
- Add colon after introductory phrase

### Tables

**Creation Standards:**
- Include no empty cells (use N/A or None)
- Place description columns rightmost when possible
- Maintain consistent column spacing
- Use sentence case for column titles

## Links

### Link Guidelines

- Use inline links over reference links
- Avoid duplicate links on the same page
- Don't include links in headings
- Keep links on single lines (no hard line wraps)
- Limit to 15 external page links per page
- Use relative file paths for same-repository links

### Link Text Standards

Follow these patterns:
- "For more information, see [descriptive text](link.md)"
- "To [accomplish task], see [descriptive text](link.md)"

**Avoid:**
- "Learn more about..."
- "Click here"
- Generic "this page" references
- Words like "documentation" in link text

## Capitalization

**Topic Titles**: Use sentence case.

**UI Text**: Match exact interface capitalization; use sentence case for all-caps text.

**Feature Names**: Use lowercase unless added to markdownlint configuration as proper nouns.

**Product Tiers**: Capitalize (GitLab Free, GitLab Ultimate).

## Alert Boxes

Use sparingly; never place two consecutively.

**Types:**
- **Note**: Supplementary information
- **Warning**: Deprecated features or data loss risks
- **Flag**: Feature availability details
- **Disclaimer**: Forward-looking statements about unreleased features

Format: `> [!type]` with brief content.

## Screenshots

**Capture Standards:**
- Show only relevant portions
- Include realistic, diverse fake data
- Maximum 1000 pixels wide, 500 pixels tall
- PNG format (not JPEG)
- Compressed to 100 KB or less

## Vale and Testing

Documentation uses Vale for style rule enforcement and markdownlint for Markdown validation.

## Topic Types

GitLab uses topic types (concepts, tasks, references, tutorials) to organize documentation. This structure helps users find information quickly and addresses findability and contributor perspective issues.
