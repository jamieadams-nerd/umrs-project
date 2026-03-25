---
name: workflow feedback and confirmed patterns
description: What works, what to avoid, and confirmed patterns from sessions to date
type: feedback
---

## Confirmed working patterns

**xtr is not yet verified installed.** All .pot files so far were either hand-crafted
(umrs-uname) or produced by earlier extraction. Always check `xtr --version` at session
start before assuming it is available.

**The fr_CA.po for umrs-uname has more entries than the .pot.** This is because the .po
was extended after the hand-crafted .pot was created. Running msgmerge will obsolete ~19
entries unless the .pot is regenerated first. Always regenerate .pot before msgmerge.

**Comment ordering in .po files:** TRANSLATOR comments must come BEFORE the #: source
reference line. Order: `# TRANSLATOR:` → `#.` (extracted comment) → `#:` source → msgid.
The umrs-logspace/fr_CA.po had comments after #: — this was fixed 2026-03-25.

**Header template typo to watch:** `a.k.a,` (comma) vs `a.k.a.` (period). The correct
form is `a.k.a.` — umrs-ls/fr_CA.po was corrected 2026-03-25.

**The `t!("...")` macro is not used in this project.** The guidance document shows `t!()`
but actual source uses `i18n::tr()`. Always use `i18n::tr("msgid")` in wrapping reports.

**Library strings resolve in the calling binary's domain (Option A).** umrs-ui strings
appear in the calling binary's .po (e.g., "Recommended" is in umrs-uname/fr_CA.po not
in a umrs-ui domain). This is the current implicit choice. Document it as Option A in
domains.md once Jamie confirms.

**Column brevity constraint:** key column labels for TUI should be ≤8 characters where
possible. Exceptions documented (PROPRIO:GROUPE = 14 chars — inherent to two-field format).
Length testing at 150% expansion is the rule — measure actual TUI width before committing.

**Security label designators are NOT gettext strings.** "PROTÉGÉ B" comes from the JSON
label catalog, not from gettext. Do not wrap designators in i18n::tr(). Only surrounding
prose labels (column headers, field names) are translatable via gettext.

## What to avoid

**Do not use `msgctxt` in .po files until `tr_ctx()` exists in umrs-core::i18n.**
gettextrs supports pgettext/dcpgettext, but the project's `tr()` function calls dgettext()
only. Adding msgctxt to .po files without a corresponding tr_ctx() call site would have
no effect at runtime. Wait for developer to add tr_ctx() first.

**Do not run msgmerge on umrs-uname without first regenerating the .pot from source.**
The existing .pot is hand-crafted and lags behind the current source + fr_CA.po.

**Do not use xgettext for Rust source.** Use xtr only. xgettext does not understand Rust
macro syntax and will produce wrong source references.
