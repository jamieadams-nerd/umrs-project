# Google Developer Documentation Style Guide — Key Principles

**Source:** https://developers.google.com/style
**Date compiled:** 2026-03-20
**Phase:** 2C — Technical writing & documentation science

---

## Overview

Google's style guide for developer documentation. Not all rules apply to UMRS (Google has different constraints), but the core principles are widely adopted and well-reasoned.

---

## Key Principles

### 1. Be conversational but not slangy

- Write as if explaining to a colleague
- Use "you" to address the reader directly
- Avoid overly formal constructions
- Don't try to be funny (humor doesn't translate well)

### 2. Use present tense and active voice

- "The function returns a Result" not "The function will return a Result"
- "Run the command" not "The command should be run"
- Active voice is especially important in procedures (aligns with UMRS STE rules)

### 3. Be specific, not vague

- "Takes approximately 5 minutes" not "Takes a few minutes"
- "Requires RHEL 10 or later" not "Requires a recent Linux distribution"
- "Returns an error if the file does not exist" not "May fail under certain conditions"

### 4. Use second person

- "You can configure..." not "Users can configure..." or "One can configure..."
- Exception: API reference docs can use third person for method descriptions

### 5. Write short sentences

- Target 20-25 words per sentence
- If a sentence has more than one "and" or "or", split it
- Aligns with UMRS STE rules (20-word max for procedure steps)

### 6. Use standard American English

- "color" not "colour"
- "behavior" not "behaviour"
- Consistent throughout the project

---

## Terminology Principles

1. **Use the same term for the same concept** — don't alternate between "module", "package", and "library" if they mean the same thing
2. **Define terms before first use** — or link to a glossary
3. **Avoid proprietary terms when standards exist** — "container" not "Docker" when you mean OCI containers
4. **Use product names correctly** — "SELinux" not "selinux" or "Selinux"

---

## Formatting Principles

1. **Code in code font** — any term that appears in code should be in backticks
2. **UI elements in bold** — button names, menu items
3. **File paths and commands in code font** — `/etc/selinux/config`, `sestatus`
4. **Lists for 3+ parallel items** — don't embed lists in prose
5. **Tables for structured comparisons** — easier to scan than paragraphs

---

## What UMRS Should Adopt

- Active voice and present tense (already in STE rules)
- Specific numbers over vague quantities
- Consistent terminology (already enforced)
- Code font for all technical terms
- Short sentences (already in STE rules)

## What UMRS Should Adapt

- Google's voice is deliberately neutral. UMRS (per Jamie's direction) should be more energetic and engaged — "explain like an engineer, not a salesperson"
- Google avoids first person. UMRS blog posts should use first person for authenticity
- Google's guide assumes a large team. UMRS has one author — the voice can be more personal

## Sources

- [Google Developer Documentation Style Guide](https://developers.google.com/style)
- [Google Style Guide — Word list](https://developers.google.com/style/word-list)
