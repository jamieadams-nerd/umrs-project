# Google Developer Documentation Style Guide — Procedures, Formatting, Accessibility

**Source:** https://developers.google.com/style
**Procedures:** https://developers.google.com/style/procedures
**Text-formatting summary:** https://developers.google.com/style/text-formatting
**Accessibility:** https://developers.google.com/style/accessibility
**Link text:** https://developers.google.com/style/link-text
**Tables:** https://developers.google.com/style/tables
**Code samples:** https://developers.google.com/style/code-samples
**Retrieved:** 2026-03-16 (via WebSearch — developers.google.com blocked WebFetch)

---

## Procedures

A procedure is a sequence of numbered steps for accomplishing a task.

- Present procedures as numbered steps.
- Start each step with an imperative verb.
- Steps should be complete sentences and follow parallel structure.
- Provide an introductory sentence with context before the numbered list.
- Single-step procedures are formatted as bulleted lists (not numbered).
- Sub-steps use lowercase letters and Roman numerals.

**For complex steps, document in this order:**
1. The action
2. The command
3. Placeholders
4. Explanation
5. Output

**Multiple methods:** If there's more than one way to complete a task, document one procedure
accessible for all readers. If all methods are accessible, pick the shortest and simplest. If you
need to document multiple ways, separate them in different pages, headings, or tabs.

**Optional steps:** At the beginning of an optional step, type *Optional* followed by a colon.

**Keyboard shortcuts in procedures:** (Added 2025) Guidance on including keyboard shortcuts
within procedure steps is on the Procedures page.

---

## Text Formatting

### Bold
Use bold formatting (`<b>` or `**`) only for:
- UI elements
- Run-in headings, including at the beginning of notices

### Italics
Italicize words for:
- Emphasis
- Term definitions
- Titles of full-length works
- Mathematical variables
- Version variables

### Underline
Underline only link text.

### Ampersands
Don't use ampersands (`&`) as conjunctions or shorthand for *and*. Use *and* — including in
headings and navigation.

### Code font
Put code-related text in code font. This includes names of command-line utilities.

### Semantic HTML
Use semantic HTML or Markdown to control text style — for example, `<code>` tags in HTML or
backticks in Markdown — instead of manually styling text with a monospace font.

---

## Accessibility

Write documentation that is accessible to all readers, including those with disabilities.

### Directional language
Don't use directional language to refer to a position in a document. For example, the text isn't
"below" if it's being read by a screen reader. Instead, use *earlier*, *preceding*, or
*following*.

### Images
- Provide an `alt` attribute for every image.
- Use alt text that adequately summarizes the intent of each image.
- If the image is purely decorative, use empty alt text.
- Don't use images of text, code samples, or terminal output. Use actual text.

### Procedures
In a procedure, make each instruction a list item. Use lists to make it easier for the reader to
follow the steps.

### Interactive elements
Introduce an interactive element (such as a button that expands and collapses) in the text
preceding the element.

### Color and contrast
Pick colors that respect accessible color contrast ratios (4.5:1 for text). Don't use
`visibility:hidden` or `display:none` — both styles hide information from screen readers.

---

## Link Text

- Use short, unique, descriptive phrases that provide context for the material you're linking to.
- Don't force links to open in a new tab or window.
- When a cross-reference is a link, don't put the link text in quotation marks.
- Link to the most relevant page on a site. Link to the most relevant heading on a page.
- Avoid providing multiple links that do the same job.
- Underline link text, and don't underline non-link text.
- Make visited links change color. Use color-blind-friendly color changes.
- People who use screen readers often use them to scan a page to hear just the links. Use
  informative link text.

---

## Tables

- Introduce tables with a complete sentence that describes the purpose of the table.
- Use sentence case in table headings.
- Write concise headings. Don't end with punctuation, including a period, an ellipsis, or a colon.
- Use table headings for the first column and the first row only.
- Avoid tables in the middle of a numbered procedure.
- Avoid using tables for page layout, code snippets, long one-dimensional lists, or within
  numbered procedures.
- Ensure responsiveness for various viewport sizes.
- Use proper HTML elements: `caption`, `th`, and `scope`.

---

## Code Samples

- Adhere to language-specific indentation guidelines, typically using spaces (two per level).
- Wrap lines at 80 characters or fewer, especially for narrower displays.
- Format code blocks as preformatted text using `<pre>` in HTML or four-space indentation in
  Markdown.
- Indicate omitted code with language-specific comments, not ellipses.
- Don't use three dots or the ellipsis character (…) to indicate omitted code.
- If a code block contains an omission, don't format the block as click-to-copy.

---

## Lists

- Use numbered lists for sequences.
- Use bulleted lists for most other lists.
- Use description lists for pairs of related pieces of data.
- Introduce lists with a complete sentence.
- Maintain parallel structure (syntax/grammar) across all list items.
- Don't use a list to show only one item.
- Capitalization and end punctuation depend on the type of list and its contents.

---

## Titles and Headings

- Use sentence case for document titles and section headings.
- For titles of shorter works (articles, episodes), put titles in quotation marks unless they're
  part of a link.
- Tag headings with heading elements (e.g., `<h1>`, `<h2>` in HTML or `#`, `##` in Markdown).
- Use a level-1 heading for the page title or main content heading.
- Don't skip levels of the heading hierarchy (e.g., don't jump from `<h1>` to `<h3>`).
