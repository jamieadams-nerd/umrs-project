---
name: Raw notes in foundations/history pages
description: Several history pages were raw AI conversation transcripts or bullet-note dumps — requires full AsciiDoc rewrite, not just editing
type: feedback
---

Three pages in `docs/modules/ROOT/pages/foundations/history/` were raw AI Q&A transcripts or
draft notes with no proper AsciiDoc structure: `trusted-path-orange.adoc`, `microsoft-nt-orange.adoc`,
and `ibm-zos-os390.adoc`. They contained markdown-style bullet characters (•), all-caps section
headings, AI meta-commentary ("Here is a rewritten version..."), and "just say the word" prompts.

**Why:** These pages were ingested from Jamie's research notes without editorial pass.

**How to apply:** When reading foundations/history pages, always scan for inline AI prompts,
markdown artifacts, and structural issues before assuming the file is ready. Pages that were
Jamie's raw notes often need full rewrites, not just edits. The canonical approach is to keep
the factual content and rebuild the structure entirely in proper AsciiDoc.
