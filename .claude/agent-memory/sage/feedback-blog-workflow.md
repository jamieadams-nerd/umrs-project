---
name: Blog Workflow — Post-Publish Steps and PDF Catalog
description: What Sage does after a blog post goes live. Includes doc sync, archiving, and future PDF catalog deliverable.
type: feedback
---

After a blog post is publicly published (Jamie directive, 2026-03-20):

1. Review the live post against the draft — identify any on-the-fly changes Jamie made during publishing
2. Update UMRS docs to reflect those changes (if Jamie edited for clarity or accuracy, the docs may need to match)
3. Archive the draft or mark it as published — do not leave drafts in an ambiguous state

Inbox concept: working well. Sage and tech-writer must archive items after acting on them.

**Why:** Jamie edits on the fly when publishing. Those edits are authoritative — they represent the final voice decision. If docs and posts drift, we create confusion for readers who cross-reference.

**How to apply:** After any post goes live, run a diff between the published content and the draft. If discrepancies exist, open a task or note for the relevant agent to sync docs. Mark the draft archived or add a "published: [URL]" header.

## PDF Catalog (future deliverable)

Jamie wants:
- Beautiful PDFs of each blog post
- A searchable catalog of those PDFs

**Why:** Engineers like having reference material at the ready. A PDF catalog serves the practitioner audience that prefers offline/printable docs.

**How to apply:** This is not blocking current work. Flag it when the post count justifies the effort (rough threshold: 5+ published posts). Identify a PDF generation approach (Asciidoctor PDF, Pandoc, or GitHub Actions pipeline) and propose it to Jamie. The catalog searchability requirement may mean a simple index page with metadata or something more structured.
