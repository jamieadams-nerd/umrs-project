---
name: pipeline gaps and blocking issues
description: Known gaps in the UMRS l10n pipeline as of 2026-03-25, with severity and required actions
type: project
---

## Gap 1 — tr_ctx() missing (BLOCKING for msgctxt policy)

**Why:** Jamie's l10n guidance requires msgctxt for all security labels. gettextrs 0.7.7
exposes `pgettext()` and `dcpgettext()`, but `umrs-core::i18n` only exposes `tr()` which
calls `dgettext()`. No context-qualified lookup is available at call sites.

**How to apply:** Any wrapping report for security classification strings must specify
`i18n::tr_ctx("security_label", msgid)` rather than `i18n::tr(msgid)`. Developer must
add the function first. Document the new function in a wrapping report to developer.

## Gap 2 — umrs-stat: no domain directory, ~20 unwrapped strings (HIGH PRIORITY)

**Why:** `umrs-stat/src/main.rs:748` calls `i18n::init("umrs-stat")` but there is no
domain directory, no `.pot`, no `fr_CA.po`. fr_CA operators see all English strings.

**Also:** ~20 additional user-facing strings are not yet wrapped:
- Boolean values ("Yes"/"No", "yes"/"no") — straightforward tr() candidates
- Key column labels: Inode, Device, Hard links, Owner, Group, Mount point, Filesystem,
  Device node, Mounted on, Append-only, POSIX ACL, Access denied, SELinux user/role/type,
  Raw label, "(none)"
- Encryption type strings: "None", "LUKS (dm-crypt)", "Encrypted filesystem ({fs})"
- State labels: "Labeled", "Unlabeled", "ParseFailure", "TpiDisagreement"
- The eprintln! error message at line 769 (non-UTF-8 path)

**Structural refactors needed (developer must do before wrapping):**
- `format_size()` at lines 86/93/98/102 — "bytes"/"KB"/"MB"/"GB" need ngettext + unit tr()
- Status bar format strings at 122/127 — "(s)" plural shorthand not translatable; needs ngettext
- nlink "(hard-linked)" at line 321 — partially wrappable

**How to apply:** Produce umrs-stat wrapping report. Do not begin fr_CA translation until
domain directory exists and at least the straightforward strings are wrapped.

## Gap 3 — umrs-uname help text overlays not translated

**Why:** Four multi-line help text blocks (Tab 0, Tab 1, Tab 2, fallback navigation) have
empty `msgstr ""` in fr_CA.po. These are the most complex strings — ASCII box-drawing,
fixed-width column alignment, ~150 lines total. 66-character line limit applies per line.

**How to apply:** This is the next major translation task for umrs-uname. Do not attempt
to translate all four blocks in one session — work one tab at a time. Developer must
review any line flagged as exceeding 66 characters.

## Gap 4 — umrs-uname .pot is hand-crafted and may be stale

**Why:** `.pot` was hand-crafted 2026-03-23. The `fr_CA.po` has been updated since,
resulting in more entries in the `.po` than in the `.pot`. This is an inversion of the
correct workflow. Running `msgmerge` will mark ~19 extra entries as obsolete (#~) unless
the `.pot` is first updated.

**How to apply:** Before running msgmerge on umrs-uname, do a fresh xtr extraction and
compare the new `.pot` against the current one. Only then run msgmerge.

## Gap 5 — domains.md was stale

**What:** As of 2026-03-25, domains.md was updated to reflect actual state (15 rows).
It now lists: umrs-ls, umrs-platform, umrs-uname, umrs-state, umrs-logspace as active;
umrs-stat as blocking gap; umrs-ui as decision pending; umrs-df/ps/tester as reserved;
umrs-labels/selinux/hw/cui as not onboarded.

The incorrect umrs-core entry (no domain needed) was noted and will be removed once
Jamie confirms.

## Gap 6 — No pseudolocalization tooling

**Why:** Guidance document requires pseudolocalization for layout testing. No tool selected.
Recommendation: implement `scripts/pseudoloc.py` (20-line Python). Developer action pending.

## Gap 7 — No CI gate for .po completeness

**Why:** Makefile has `i18n-check` (msgfmt --check) but no coverage gate. No `msgcmp`
staleness check. Proposed: `i18n-ci` Makefile target with msgfmt --statistics + msgcmp.
Developer action pending.
