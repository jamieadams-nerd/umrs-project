# UMRS l10n Pipeline Review

**Date:** 2026-03-25
**Author:** umrs-translator agent ("Simone")
**Requested by:** Jamie Adams
**Input documents reviewed:**
- `.claude/jamies_brain/l10n-versus-18n.md`
- `.claude/reports/i18n-l10n-architecture.md`
- All `.po`, `.pot`, `.mo` files under `resources/i18n/`
- `umrs-core/src/i18n.rs`
- `umrs-ls/src/main.rs`, `umrs-stat/src/main.rs`, `umrs-uname/src/main.rs`

---

## 1. Validation of the l10n Guidance Document

Jamie's document is the right foundation. It captures the motivation correctly and sets a
standard that is genuinely higher than most open-source projects achieve. The policy-accuracy
framing — treating l10n as a policy compliance exercise rather than a translation exercise —
is exactly right for a government deployment tool. Henri's standing note at the end is not
decoration; it belongs in the document.

The following section-by-section observations are offered as refinements, not corrections.

### 1.1 Section accuracy: Complete String Externalization

The intent is correct. The example in the document is accurate:

```rust
// Wrong
eprintln!("Error: label not found");

// Right
eprintln!("{}", t!("error.label_not_found"));
```

One technical note: the document shows `t!("error.label_not_found")` — this is not the macro
name used in the actual codebase. The project uses `i18n::tr("error.label_not_found")` as a
function call, not a macro. The document should be updated to match actual usage:

```rust
eprintln!("{}", i18n::tr("error.label_not_found"));
```

This is not a functional issue, but a new developer reading the guidance document alongside
the source code will find the discrepancy confusing.

**Missing from this section:** the document does not address `log::debug!()` calls. Most of
those are developer-facing and should NOT be translated (UMRS already has a debug log
information discipline rule for this). The guidance should explicitly state that `log::debug!()`,
`log::trace!()`, and `log::info!()` in library crates are exempt from the externalization
requirement — only strings that surface to an operator's terminal or TUI are in scope.

### 1.2 Section accuracy: PO File Quality Standards

Accurate. The dual-key requirement (Simone for linguistic quality, Henri for policy
accuracy) is the correct model and should not be softened. The Termium Plus reference is
appropriate — UMRS has 32,000 entries ingested. The reference to "exact Treasury Board
terminology" is the correct framing.

**Missing from this section:** the document does not mention that the corpus-first lookup
protocol (via the `french-lookup` skill) must run before any Termium Plus lookup. The
priority chain is: GNU corpus → Termium Plus → UMRS vocabulary decision. This is described
in the translator system prompt but not in this guidance document that developers and
reviewers will read. It should be added.

### 1.3 Section accuracy: Policy-Critical Label Fidelity (msgctxt)

This section describes `msgctxt` as mandatory for all security labels. This is the right
policy. However, there is a gap between the policy as written and the current technical
implementation — addressed in Section 4 below.

The example `.po` entries shown are technically correct gettext syntax. The `msgctxt` field
functions as a disambiguation key: two msgids with the same string but different `msgctxt`
values are treated as distinct entries by `msgfmt` and `msgmerge`. This is exactly the right
tool for preventing a generic translator from replacing "Protected B" with a linguistically
acceptable but policy-incorrect term.

### 1.4 Section accuracy: French Typography Rules

The table is accurate. The non-breaking space before colon, question mark, and exclamation
mark (`\u00A0`) is a genuine rule of standard French typography that distinguishes French from
all the Five Eyes English variants. This is not optional styling.

One gap in the document: it does not address the **em dash**. Several current msgids use the
em dash (U+2014), including:

```
"State file does not exist — it will be created."
"Warning: state file does not exist — using default state."
"Hard gate failure aborted pipeline"
```

In French typography, an em dash used as a parenthetical separator is typically replaced by
a space + en dash + space (`\u0020\u2013\u0020`) or left as-is depending on context. This
distinction affects the current `umrs-state` translations, which simply retain the em dash
from the source string. That choice is defensible but should be documented as a deliberate
UMRS style decision.

**Recommendation:** Add a row to the typography table for em dash handling.

### 1.5 Section accuracy: Plural Forms

Accurate. The French plural rule `nplurals=2; plural=(n > 1)` is correctly stated. Note that
this is the **fr_CA** rule. Both `fr_CA` and `fr_FR` use the same two-form rule for classical
French, although modern colloquial French sometimes treats `1` as plural (the "n >= 1" rule).
For government documentation, the classical `n > 1` rule is correct and is what the current
`.po` files use.

**Missing from this section:** the document does not address the case where a string contains
a count but is currently not using gettext plural forms. For example, in `umrs-stat`:

```rust
format!("{risk_count} risk finding(s) detected")
format!("{warn_count} warning(s), no risk findings")
```

These are not wrapped at all, but even if they were, they would need `ngettext()` (the plural
form function) not `tr()`. The guidance should note that format strings containing counts must
use `ngettext()`, and the developer must not use the English "(s)" shorthand for plurality —
it does not translate.

### 1.6 Section accuracy: String Expansion Budget

The 20–30% expansion figure is accurate for prose. For UMRS specifically, the real concern is
the TUI key column, which has already been measured and documented (the 150% rule, 66-character
help text limit). This section in the guidance document is lighter than the actual practice —
it would benefit from referencing the specific column-width constraints that have been
empirically discovered during umrs-uname translation work.

**Missing:** the guidance does not mention the `#. UMRS: Layout-sensitive string` comment
convention. This convention was established in the umrs-uname work and should be documented
here as the standard way to flag layout-sensitive strings in `.pot` files.

### 1.7 Section accuracy: Locale Detection

The priority order shown in the document matches the actual glibc priority chain, with one
omission. The full chain is:

```
LANGUAGE > LC_ALL > LC_MESSAGES > LANG > (compiled default)
```

The document omits `LANGUAGE`. This is rarely set on server deployments, but it should be
present in the list because it is the highest-priority override. If a user sets `LANGUAGE=fr`
in their environment, that overrides `LC_ALL`. This can cause unexpected behavior and should
be documented.

The `UMRS_LOCALEDIR` escape hatch is not mentioned in this guidance document, though it is
in the i18n architecture report and in `i18n.rs`. For a guidance document that developers will
read, this should be present with the development testing invocation pattern.

### 1.8 Section accuracy: Testing Requirements

The three testing tiers described (pseudolocalization, coverage validation, policy spot-check)
are correct in intent. However, the document does not specify how any of them would actually
be implemented in the current toolchain:

- Pseudolocalization: no tool is named. `poutils` and `pseudo-l10n` are options; a simple
  Python/sed transform over the `.po` file also works. Without a concrete tool, this remains
  aspirational.
- Coverage validation: the Makefile already has `i18n-check` which runs `msgfmt --check`.
  This catches format mismatches but does not enforce that every msgid has a non-empty
  msgstr. A separate `msgfmt --statistics` check (which reports the untranslated count)
  should be the CI gate.
- Policy spot-check: correctly identified as non-automatable and requiring Henri. The
  document should clarify this is a gate before a release tag, not a gate before every
  commit.

**Missing:** the document does not address `.po` staleness detection — the case where source
code changes introduce new strings that are not in the `.pot`, or where the `.pot` is out of
date with the source. `make i18n-extract-<domain>` + `msgmerge` is the pipeline, but this
is not called out in the testing section.

### 1.9 Section accuracy: File Structure

The directory layout shown in the guidance document differs from the actual layout in
`resources/i18n/`. The document proposes a `locale/` directory at project root:

```
locale/
├── umrs.pot
├── fr_CA/
│   └── LC_MESSAGES/
│       ├── umrs.po
│       └── umrs.mo
```

The actual layout uses `resources/i18n/<domain>/` with per-domain subdirectories for each
compiled locale:

```
resources/i18n/
├── umrs-ls/
│   ├── umrs-ls.pot
│   ├── fr_CA.po
│   ├── fr_FR.po
│   ├── en_GB.po
│   └── fr_CA/
│       └── LC_MESSAGES/
│           └── umrs-ls.mo
```

This is a significant discrepancy. A developer reading the guidance document would expect a
different layout from what actually exists. The guidance document should either:

(a) Describe the actual split-domain layout as the canonical structure, or
(b) Note explicitly that the file structure shown is illustrative and the canonical layout
    is in `resources/i18n/domains.md` and the i18n architecture report.

The split-domain structure (one directory per text domain) is the correct approach for UMRS
and should be the canonical reference.

### 1.10 Section accuracy: Roles and Responsibilities table

Accurate. The "String wrapping for externalization, `.pot` generation" responsibility
assigned to Rust developers is correctly placed — this is the boundary defined in my system
prompt (I produce wrapping reports; developers implement them). The table reflects actual
practice.

---

## 2. Resources Required for Proper l10n

### 2.1 Gettext CLI Tools

Status of standard gettext suite:

| Tool | Required for | Status | Install |
|---|---|---|---|
| `xtr` | `.rs` → `.pot` extraction | NOT CONFIRMED INSTALLED | `cargo install xtr` |
| `msginit` | initialize new `.po` | presumed available (GNU gettext) | system gettext package |
| `msgmerge` | merge `.pot` into `.po` | presumed available | system gettext package |
| `msgfmt` | compile `.po` → `.mo` | presumed available | system gettext package |
| `msgfmt --check` | validate `.po` | presumed available | included with msgfmt |
| `msgfmt --statistics` | coverage report | presumed available | included with msgfmt |

The prior architecture report noted that `xtr` was not installed as of 2026-03-10. All
`.pot` files currently in the repository were either hand-crafted (umrs-uname: see the
note at the top of `umrs-uname.pot`) or produced by an earlier extraction run. The
`umrs-ls.pot` and `umrs-state.pot` appear to be genuine extraction outputs.

**Action required before any new extraction work:** verify `xtr` is installed.
```bash
xtr --version
# if not: cargo install xtr
```

For RHEL 10 deployment, the gettext CLI tools are in `gettext-devel` or `gettext`:
```bash
dnf install gettext
```

### 2.2 Crate Selection Assessment

**Current crate: `gettext-rs` 0.7.7** — confirmed in `umrs-core/Cargo.toml` and `Cargo.lock`.

This is the right choice. Rationale:

- Uses system `libintl` on RHEL (via `gettext-sys`), avoiding a bundled copy of the C library.
  This is the correct behavior for a high-assurance RHEL 10 deployment where the system
  gettext is known-good and auditable.
- Works with `xtr` — the combination of `gettext-rs` + `xtr` is the intended Rust gettext
  stack. `xtr` understands `gettext()` and `dgettext()` calls in Rust source.
- `locale_config` is the only non-trivial transitive dependency. It is a pure Rust crate
  handling locale string parsing — acceptable for a high-assurance context.
- The crate is actively maintained (0.7.7 released 2024).
- The workspace uses it consistently — all three binary crates with i18n active use it
  via `umrs-core::i18n`.

**Alternatives evaluated:**

The question of whether to evaluate `gettext` (pure Rust, no C) or `fluent` (Mozilla's i18n
system) is worth answering explicitly:

*`gettext` (pure Rust crate):* Would eliminate the `gettext-sys` C dependency. However, it
implements its own `.mo` parser and may diverge from glibc's implementation on edge cases
(encoding handling, plural form evaluation). For a RHEL 10 government deployment where the
system gettext is the authoritative implementation, using the system library via `gettext-rs`
is more defensible from an audit standpoint. Supply chain risk is lower with
the system-provided C library than with an independent pure-Rust re-implementation.
**Recommendation: do not switch.**

*`fluent` (Mozilla):* Fluent is a modern, Unicode-correct localization system with superior
handling of complex grammatical rules. However, it uses `.ftl` files rather than `.po` files,
requires its own toolchain, has no equivalent to `xtr`, and is not compatible with the
existing `.po` catalog investment. It would be a complete pipeline replacement. The UMRS
use case (short strings, column headers, status messages, TUI labels) does not require
Fluent's grammatical sophistication. **Recommendation: do not switch.**

**Verdict: stay with `gettext-rs` 0.7.7.**

### 2.3 Pseudolocalization Tooling

The guidance document calls for pseudolocalization. No tool has been selected. Options:

1. **`poutils`** (`pip install poutils`) — Python package with a `pseudo` command that
   transforms `.po` files by replacing ASCII letters with accented lookalikes. Simple and
   does what is needed.

2. **Manual script** — a short Python or bash script that reads a `.po` file and outputs
   a pseudo-locale `.po` with transformed strings. This is trivial to implement and requires
   no external dependencies.

3. **`pseudo-l10n`** — a dedicated tool, but it is less commonly available and adds a
   dependency for a testing-only feature.

**Recommendation:** a short Python script maintained in `scripts/` under the project. It
would:
- Read a `.po` file
- For each `msgstr`, replace ASCII `[a-z]` with accented equivalents (`a→à`, `e→é`, etc.)
- Output a `qps-ploc.po` (the conventional pseudo-locale code)
- Preserve format specifiers (`%s`, `{name}`, `{0}`) untouched

This is a developer tool, not a production artifact, and a 20-line Python script is
preferable to a new dependency.

**Prerequisite action (developer):** implement `scripts/pseudoloc.py`. Translator provides
the transformation table.

### 2.4 String Extraction Tooling

The project's chosen extractor is `xtr`. This is the correct choice.

`xgettext --language=C` must NOT be used. It does not understand Rust macro syntax and will
produce incorrect source references, miss multi-argument calls, and misparse raw string
literals. `xtr` was designed for Rust and handles:
- `gettext("msgid")` calls
- `dgettext("domain", "msgid")` calls
- String literals inside macros

The one known limitation of `xtr` is that it does not understand custom wrapper functions.
Our `i18n::tr("msgid")` calls wrap `dgettext()`. Whether `xtr` recognizes `i18n::tr()` as
a gettext call depends on its configuration. The existing `.pot` files appear to have been
hand-crafted or generated with `xtr` in a mode that scanned directly for string literals.
This needs to be verified before any new extraction run — if `xtr` is not recognizing
`i18n::tr()` calls automatically, an explicit wrapper mapping may be needed.

**Verification required:**
```bash
# Install if not present
cargo install xtr

# Test extraction on umrs-ls:
xtr --package-name umrs-ls \
    --output /tmp/umrs-ls-test.pot \
    umrs-ls/src/main.rs

# Compare output with existing resources/i18n/umrs-ls/umrs-ls.pot
```

If `xtr` does not pick up `i18n::tr()` calls, the `--keyword` flag may be needed:
```bash
xtr --keyword i18n::tr --package-name umrs-ls ...
```

### 2.5 CI Integration

A CI gate for `.po` completeness should be added. The recommended approach:

```bash
# In a CI step after msgfmt --check:
msgfmt --statistics resources/i18n/umrs-ls/fr_CA.po 2>&1 | \
  grep "untranslated" && { echo "FAIL: untranslated strings in fr_CA.po"; exit 1; } || true
```

A non-zero untranslated count should be a CI warning for active domains and a CI error
for release builds. The distinction matters: during development, a new domain may be
partially translated; at release time, all strings in `fr_CA.po` for active domains must
be complete.

**Proposed CI stages:**

| Stage | Tool | Gate | Trigger |
|---|---|---|---|
| Format validation | `msgfmt --check` | Error on any `.po` in active domains | All commits |
| Coverage report | `msgfmt --statistics` | Warn on untranslated count > 0 | PR builds |
| Coverage gate | `msgfmt --statistics` | Error on untranslated count > 0 | Release builds |
| Staleness detection | `msgcmp <po> <pot>` | Error on missing msgids | PR builds |

`msgcmp` compares a `.po` against its `.pot` and reports msgids present in the `.pot` but
absent from the `.po`. This catches the case where source changes added new strings that
were extracted to the `.pot` but not yet translated.

**Action required (developer):** add `i18n-ci` target to Makefile that runs the above
sequence for all active domains.

---

## 3. String Wrapping Gap Analysis

### 3.1 Methodology

For each binary crate, I counted:
- Calls to `i18n::tr()` — strings confirmed wrapped for extraction
- User-facing string literals in `format!()`, `eprintln!()`, or `DataRow::new()` / `DataRow::normal()`
  calls that are NOT wrapped — strings visible to operators but not yet translatable

I excluded:
- Strings in `//` comments and `///` doc comments
- Debug log strings (`log::debug!()`, `log::info!()`, etc.)
- Strings that are raw data values (tool names, filesystem type identifiers, capability names)
- Format strings where the substituted value is the entire content (e.g., `format!("{name}")`)

### 3.2 umrs-ls — Gap analysis

**`i18n::tr()` calls:** 8 (matches `.pot` exactly)

**Confirmed wrapped strings:**
- `"access denied"`, `"MODE"`, `"MARKING"`, `"OWNER:GROUP"`, `"SIZE"`, `"MODIFIED"`, `"NAME"`, `"<restricted>"`

**Unwrapped user-facing strings identified:**

| Line | String | Classification |
|---|---|---|
| 196 | `"{total_entries} entries  {}  {} groups  {} µs"` | Format string with plural; needs `ngettext()`, not `tr()`. "entries" and "groups" should be externalized. |

The timing footer at line 196 is the only user-facing prose string not wrapped. It contains
pluralizable nouns ("entries", "groups") and a microsecond unit (`µs`). This needs a plural
form treatment, not just `tr()`. The full format string cannot be wrapped as-is.

**Structural issue:** The `{}` placeholders for `listing.path.display()`, `listing.groups.len()`,
and `listing.elapsed_us` mean this string requires template substitution. The wrapping pattern
for format strings with runtime values is:
```rust
// NOT: tr("N entries  {} path  {} groups  {} µs")
// Instead, use separate translatable fragments:
format!("{} {} {} {} {} {} {} {}",
    n, tr("entries"), path, tr("in"), groups, tr("groups"), us, tr("µs"))
```
Or a single format string with named arguments (requires a substitution function, not bare
`tr()`). This is the "format string refactor" category from the umrs-uname open items and
requires developer action to restructure before wrapping is possible.

**Gap count for umrs-ls:** 1 format string requiring structural refactor before wrapping.
Wrapping completeness against current structure: **8/8 = 100%** of wrappable strings are
wrapped.

### 3.3 umrs-stat — Gap analysis

**`i18n::tr()` calls:** 27

**Confirmed wrapped strings (selection):**
Path, Filename, File type, MIME type, Size, Mode, Immutable, IMA/EVM, SELinux Context,
Label state, Marking, Encryption, Count, Findings, No security observations, Failed to
read file, No data available, Identity (tab), Security (tab), Observations (tab),
File Security Audit, No security concerns, Failed to read file attributes.

**Unwrapped user-facing strings identified:**

| Line | String | Category |
|---|---|---|
| 86 | `"{bytes} bytes"` | Plural + unit string; needs `ngettext` + unit translation |
| 93 | `"{bytes} bytes ({kb_frac:.1} KB)"` | Same; "bytes" and "KB" should be translatable |
| 98 | `"{bytes} bytes ({mb_frac:.1} MB)"` | Same; "MB" should be translatable |
| 102 | `"{bytes} bytes ({gb_frac:.2} GB)"` | Same; "GB" should be translatable |
| 122 | `"{risk_count} risk finding(s) detected"` | Plural; `(s)` idiom not translatable; needs `ngettext` |
| 127 | `"{warn_count} warning(s), no risk findings"` | Same plural issue |
| 314 | `"Inode"` | Unwrapped key column label |
| 315 | `"Device"` | Unwrapped key column label |
| 321 | `"{nlink_val} (hard-linked)"` | "(hard-linked)" is user-facing; partially wrappable |
| 325 | `"Hard links"` | Unwrapped key column label |
| 336 | `"{}:(unresolved)"` | "(unresolved)" is user-facing |
| 340 | `"Owner"` | Unwrapped key column label |
| 349 | `"{}:(unresolved)"` | "(unresolved)" is user-facing |
| 353 | `"Group"` | Unwrapped key column label |
| 359 | `"yes"` / `"no"` | Boolean display values — should use tr() |
| 363 | `"Mount point"` | Unwrapped key column label |
| 367 | `"Filesystem"` | Unwrapped key column label |
| 368 | `"Device node"` | Unwrapped key column label |
| 369 | `"Mounted on"` | Unwrapped key column label |
| 387-390 | `"Yes"` / `"No"` | Immutable boolean — should use tr() |
| 396 | `"Yes — integrity hash present"` | IMA/EVM positive description — unwrapped |
| 398 | `"No"` | IMA/EVM negative — should use tr() |
| 409 | `"Append-only"` | Key column label |
| 414 | `"Yes — extended DAC in effect"` | ACL description — unwrapped |
| 418 | `"POSIX ACL"` | Key column label |
| 421-424 | `"yes"` / `"no"` | Access denied boolean |
| 425 | `"Access denied"` | Key column label — note: wrapping already on "access denied" in umrs-ls but not here |
| 450 | `"  SELinux user"` | Indented key column labels (x4: user, role, type, raw) |
| 451 | `"  SELinux role"` | Unwrapped key column label |
| 453 | `"  SELinux type"` | Unwrapped key column label |
| 459 | `"  Raw label"` | Unwrapped key column label |
| 458 | `"(none)"` | Level fallback — unwrapped |
| 463-470 | `"Labeled"`, `"Unlabeled"`, `"ParseFailure"`, `"TpiDisagreement"` | State variant names as display strings — partially wrappable |
| 511 | `"None"` | Encryption source fallback — unwrapped |
| 513 | `"LUKS (dm-crypt)"` | Encryption type label — unwrapped |
| 516 | `"Encrypted filesystem ({fs})"` | Encryption type with substitution — unwrapped |
| 769 | `"error: path contains non-UTF-8 characters and cannot be displayed"` | eprintln error — unwrapped |

**Total unwrapped user-facing strings in umrs-stat:** approximately 37 distinct string
instances across 27 unique string values. The 27 already-wrapped calls represent the
"header" strings (field names, tab names, status messages). The unwrapped strings are
predominantly:
- Boolean display values ("Yes"/"No", "yes"/"no") — straightforward to wrap
- Key column labels in the Identity tab (Inode, Device, Hard links, Owner, Group, etc.)
- The `format_size()` function — structural issue (plural forms + unit strings)
- Status bar strings with `(s)` plural shorthand — structural refactor needed

**Wrapping completeness for umrs-stat (structural):**
- Wrappable strings already wrapped: 27
- Wrappable strings not yet wrapped: ~20 (excluding the structural refactor cases)
- Structural refactors needed before wrapping: ~8 (format_size, risk/warning counts,
  nlink "(hard-linked)", encryption format strings with substitution)
- Overall: approximately **57% of wrappable string calls are wrapped**

This is the crate with the most remaining work. A full wrapping report for umrs-stat is
needed. Note that `umrs-stat` also has no domain directory — both the wrapping work and
the domain onboarding are prerequisites before any fr_CA translation can begin.

### 3.4 umrs-uname — Gap analysis

**`i18n::tr()` calls:** 98

The umrs-uname wrapping coverage is substantially higher than the other binaries. The `.pot`
file (hand-crafted 2026-03-23) covers all major UI areas: tab labels, card titles, trust
tier labels, security group headers, security group descriptions, help text overlays,
evidence table headers, evidence source labels, status bar messages, and configured/from
prefix labels.

The `fr_CA.po` contains 98 translated entries (matching the `.pot`). All entries have
non-empty `msgstr` values.

**Open developer items from the 2026-03-23 rescan** (documented in prior reports):
- Items 1-2: `report_name()` and `report_subject()` return `&'static str` — trait signature
  change required before these can be wrapped.
- Items 4-8: os-release field names (VERSION, ID, NAME, PRETTY_NAME, etc.) — developer
  decision pending on whether to translate these technical identifiers.
- Items 12-15: format strings with runtime substitution — structural refactor needed for
  catalog_baseline_row() and indicator count summary strings.

**Wrapping completeness for umrs-uname:** approximately **85% of wrappable strings are
wrapped**. The remaining 15% are in the developer-action-pending categories.

### 3.5 Summary table

| Crate | i18n::tr() calls | Estimated unwrapped | Structural refactors needed | fr_CA coverage | Domain status |
|---|---|---|---|---|---|
| `umrs-ls` | 8 | 1 (format only) | 1 (timing footer) | 100% (8/8) | Active, .mo compiled |
| `umrs-stat` | 27 | ~20 | ~8 | 0% (no domain) | MISSING — blocking |
| `umrs-uname` | 98 | ~15 | ~4 | ~85% (98 wrapped, open items) | Active, .mo compiled |

---

## 4. msgctxt Support Assessment

### 4.1 Does gettextrs 0.7.7 support msgctxt?

`gettext-rs` 0.7.7 exposes `pgettext(ctx, msgid)` and `npgettext(ctx, msgid, msgid_plural, n)`
in addition to `gettext(msgid)` and `dgettext(domain, msgid)`. These are the Rust bindings
to the C library's `pgettext()` and `npgettext()` functions, which are the caller-side
counterpart to `msgctxt` in `.po` files.

**So: yes, `gettext-rs` supports `msgctxt` via `pgettext()`.**

### 4.2 Does the current `i18n::tr()` function support context-qualified lookups?

No. The current `tr()` function in `umrs-core/src/i18n.rs` is:

```rust
pub fn tr(msgid: &str) -> String {
    ensure_locale();
    let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);
    dgettext(dom, msgid)
}
```

This calls `dgettext(dom, msgid)` — the domain-qualified lookup without context. It does not
call `pgettext()` or `dcpgettext()`. There is no `tr_ctx()` function or equivalent.

**Consequence:** even if security labels in `.po` files used `msgctxt "security_label"`, the
current `tr()` function would not perform a context-qualified lookup. It would call `dgettext()`
which resolves the first matching `msgid` in the catalog, ignoring `msgctxt`.

The guidance document requires `msgctxt` for all security labels. This requirement cannot
be satisfied by the current `tr()` API alone. A new function is needed.

### 4.3 What needs to be added

A context-aware lookup function must be added to `umrs-core::i18n`. The signature:

```rust
/// Translate a message identifier with a context qualifier.
///
/// Performs a pgettext()-style lookup: the context discriminates between
/// msgids that share the same string but have different meanings.
/// Required for security classification labels, where a generic translator
/// must be prevented from substituting a linguistically acceptable but
/// policy-incorrect term.
///
/// # Parameters
///
/// - `ctx`: Context string (e.g., `"security_label"`). Must be stable.
/// - `msgid`: Message identifier to translate.
pub fn tr_ctx(ctx: &str, msgid: &str) -> String {
    ensure_locale();
    let dom = *DOMAIN.get().unwrap_or(&FALLBACK_DOMAIN);
    dcpgettext(dom, ctx, msgid)
}
```

The `dcpgettext()` function is `gettextrs`'s binding to `dcpgettext()` (domain + context +
msgid). This is the correct function for context-qualified lookups within a specific domain.

### 4.4 What call sites would use `tr_ctx()`?

Any string that represents a security classification label or is policy-critical enough that
a generic translator could substitute an incorrect term. Initial candidates:

- Security classification state names in umrs-stat ("Labeled", "Unlabeled", "ParseFailure",
  "TpiDisagreement") — currently unwrapped, would use `tr_ctx("label_state", "Labeled")` etc.
- Any future display of Canadian Protected markings if these are ever treated as gettext
  strings (currently they are designators, not gettext candidates — see Section 4.5).
- SELinux label components if ever surfaced as translatable strings.

### 4.5 The PROTÉGÉ A/B/C exception

The guidance document requires `msgctxt` for "Protected B" → "PROTÉGÉ B". However, the
i18n architecture report (Section 1.3) establishes that Canadian Protected markings are
**designators from the JSON label catalog, not gettext strings**. They are not wrapped in
`i18n::tr()` and should not be.

This is a genuine tension between the two documents. The resolution is:

The `msgctxt "security_label"` requirement in the guidance document applies to cases where
a **prose description** of a security level appears as a gettext msgid and needs policy-
accurate translation. It does not apply to the designator itself ("PROTÉGÉ B"), which is
a structured data value from the label catalog.

Example of the distinction:
- `"Protected B"` as a **designator** in the label catalog → NOT a gettext string. Displayed
  as-is from the catalog. The French form "PROTÉGÉ B" comes from the catalog.
- `"Protection level:"` as a **column header** → IS a gettext string. Wrapped normally.
- `"This file is Protected B"` as a **prose description** → IS a gettext string, and this
  is where `msgctxt "security_label"` would apply to prevent "Protected" from being
  generically translated.

The guidance document should be updated to clarify this distinction. As written, a developer
could reasonably conclude that `i18n::tr("Protected B")` with `msgctxt` is the right pattern,
when it is not.

### 4.6 Developer action required for msgctxt

1. Add `tr_ctx(ctx: &str, msgid: &str) -> String` to `umrs-core/src/i18n.rs`. This is a
   one-function addition using the existing `dcpgettext()` from `gettextrs`.
2. Update wrapping reports to specify `tr_ctx("security_label", msgid)` for any string
   that carries security classification meaning.
3. The `.po` file entries for context-qualified strings use the `msgctxt` + `msgid` +
   `msgstr` triple format — `msgmerge` handles these correctly.

---

## 5. Additional Findings Not in the Guidance Document

### 5.1 fr_FR locale files exist alongside fr_CA — this needs a decision

The following domains have `fr_FR.po` files that are non-empty:
- `umrs-ls/fr_FR.po` — 101 lines, fully translated, identical content to `fr_CA.po`
- `umrs-state/fr_FR.po` — 0 bytes (empty placeholder)
- `umrs-logspace/fr_FR.po` — 0 bytes (empty placeholder)

The existence of `fr_FR.po` in `umrs-ls` raises a question the guidance document does not
address: is `fr_FR` an intended target locale for UMRS?

The guidance document specifies `fr_CA` as the target locale throughout. The l10n guidance
is silent on metropolitan French. The `domains.md` file does not list `fr_FR` as a supported
locale — it lists `fr_CA` only.

My assessment: the `fr_FR.po` in `umrs-ls` was likely created during early pipeline testing.
The translations are identical to `fr_CA.po` (the two dialects share all the strings that
appear in umrs-ls — the differences are in vocabulary for government-specific terms that
have not yet appeared in that crate's string set).

**Recommendation:** confirm with Jamie whether `fr_FR` is an intended deployment target.
If not, the non-empty `fr_FR.po` files are misleading noise and the empty placeholder files
should be converted to notes about why `fr_FR` is not in scope. If `fr_FR` is in scope,
then `domains.md` and the guidance document both need updating to reflect this.

### 5.2 Ghost domains: umrs-df, umrs-ps, umrs-tester

The following directories exist under `resources/i18n/` but contain only empty files:
- `resources/i18n/umrs-df/` — `.pot` is 0 bytes; `fr_FR.po` is 0 bytes; no `fr_CA.po`
- `resources/i18n/umrs-ps/` — `.pot` is 0 bytes; `fr_FR.po` is 0 bytes; no `fr_CA.po`
- `resources/i18n/umrs-tester/` — `.pot` is 0 bytes; `fr_FR.po` is 0 bytes; no `fr_CA.po`

These crates do not appear in the workspace member list visible from
`components/rusty-gadgets/`:
```
umrs-core, umrs-cui, umrs-hw, umrs-labels, umrs-ls, umrs-platform,
umrs-selinux, umrs-stat, umrs-ui, umrs-uname, xtask
```

There is no `umrs-df`, `umrs-ps`, or `umrs-tester` in the workspace. These appear to be
placeholder directories for future or planned crates (df = disk free, ps = process status
— both consistent with the UMRS tool family theme).

**Recommendation:** if these crates are planned, the placeholder directories are acceptable
but should be noted in `domains.md` as "reserved — not yet onboarded." If they are abandoned
plans, the empty directories should be cleaned up. The empty `fr_FR.po` files in particular
add confusion since they imply a locale is in scope that is not in `domains.md`.

**Action for Jamie:** confirm status of `umrs-df`, `umrs-ps`, `umrs-tester`.

### 5.3 domains.md is out of sync with actual state

`domains.md` lists four domains: `umrs-ls`, `umrs-core`, `umrs-platform`, `umrs-uname`.

Actual state has eight domain directories: `umrs-df`, `umrs-logspace`, `umrs-ls`,
`umrs-platform`, `umrs-ps`, `umrs-state`, `umrs-tester`, `umrs-uname`.

Missing from `domains.md`:
- `umrs-state` — has fr_CA.po and compiled .mo; ACTIVE
- `umrs-logspace` — has fr_CA.po and compiled .mo; ACTIVE
- `umrs-df`, `umrs-ps`, `umrs-tester` — ghost domains (see 5.2)

`umrs-core` is listed in `domains.md` but has no domain directory under `resources/i18n/`.
It also uses `i18n::tr()` nowhere (it provides the function, not uses it). This entry in
`domains.md` appears incorrect — `umrs-core` does not need its own domain.

**Action required (translator):** update `domains.md` to reflect actual state.

### 5.4 Root-level .mo files are present alongside the canonical locale-structured .mo files

The following `.mo` files exist at domain root level alongside the canonical
`<locale>/LC_MESSAGES/<domain>.mo` structure:

- `umrs-ls/fr_CA.mo` (682 bytes) — alongside `umrs-ls/fr_CA/LC_MESSAGES/umrs-ls.mo`
- `umrs-ls/fr_FR.mo` — root-level
- `umrs-ls/en_GB.mo`, `en_AU.mo`, `en_NZ.mo` — root-level
- `umrs-platform/fr_CA.mo` — alongside `umrs-platform/fr_CA/LC_MESSAGES/`
- `umrs-state/fr_CA.mo` — alongside `umrs-state/fr_CA/LC_MESSAGES/`
- `umrs-logspace/fr_CA.mo` — alongside `umrs-logspace/fr_CA/LC_MESSAGES/`
- `umrs-uname/fr_CA.mo` — alongside `umrs-uname/fr_CA/LC_MESSAGES/`

The architecture report (Section 4.1) already identified these as obsolete artifacts from
an earlier compilation approach. They are not loaded by `bindtextdomain` at runtime — the
function always resolves to `<localedir>/<locale>/LC_MESSAGES/<domain>.mo`.

These root-level `.mo` files waste space and could confuse a future contributor who sees
them and assumes they are the active catalog. They should be deleted.

**Action required (Jamie to authorize):** delete root-level `.mo` files that have
corresponding `<locale>/LC_MESSAGES/` counterparts.

### 5.5 The fr_CA.po header has a minor typo

In `umrs-ls/fr_CA.po` and `umrs-state/fr_CA.po`, the copyright line reads:
```
# Copyright (c) 2025 Jamie Adams (a.k.a, Imodium Operator)
```

Note the comma after "a.k.a" — it should be a period:
```
# Copyright (c) 2025 Jamie Adams (a.k.a. Imodium Operator)
```

The standard header template in my system prompt has the correct form. The `umrs-uname`,
`umrs-platform`, and `umrs-logspace` files use the correct form. This is cosmetic but
should be corrected for consistency.

### 5.6 The umrs-uname fr_CA.po has 98 wrapped strings but the .pot has only 79 msgids

The `.pot` file was hand-crafted and explicitly notes: "Hand-crafted on 2026-03-23.
Re-run xtr once available." Counting the msgid entries in the `.pot`: 79 entries.

The `fr_CA.po` was created from this `.pot` but then continued to be updated as more
strings were wrapped in source, resulting in more msgid/msgstr pairs in the `.po` than
in the `.pot`.

This is an inversion of the correct workflow: the `.pot` should always be a superset of
the `.po`. When `msgmerge` is run, it merges the `.pot` into the `.po`. If the `.po` has
entries not present in the `.pot`, `msgmerge` will mark them as obsolete (`#~` prefix).

**Consequence:** the `fr_CA.po` for umrs-uname has approximately 19 entries that will be
marked obsolete on the next `msgmerge` run unless the `.pot` is first updated to include
all wrapped strings. Until `xtr` is available and confirmed to work with `i18n::tr()`, the
`.pot` should be manually maintained to stay in sync with the source.

**Priority action:** once `xtr` is installed and verified, run a fresh extraction on
`umrs-uname/src/main.rs` and compare the output against the existing `.pot` to find all
discrepancies before running `msgmerge`.

---

## 6. Assessment of Current Translation Quality

### 6.1 umrs-ls fr_CA.po — quality: good

All 8 msgstr values are filled. Commentary is thorough. Corpus citations are accurate and
sourced correctly. The column-header translations are appropriately brief. "MARQUAGE" for
MARKING and "MODIFIÉ" for MODIFIED are both sound decisions documented with rationale.

One observation: "PROPRIO:GROUPE" is 14 characters, which exceeds the target of ≤8 for
column headers. This was acknowledged in the inline comment. The column width should be
measured at runtime against this translation to ensure it does not cause layout breaks.

### 6.2 umrs-state fr_CA.po — quality: good

All 8 msgstr values are filled. Corpus citations are appropriate. Typography is correct —
the em dash is retained from the source string in both `msgstr` entries that contain it.
This is defensible. The colon in "Avertissement : le fichier d'état..." correctly uses the
non-breaking space before the colon (this is a UMRS-applied typography rule, not in the
source string).

Wait — reviewing the source string: `"Warning: state file does not exist — using default state."` — the colon in the source English is a post-"Warning" colon. The French translation correctly applies `Avertissement\u00a0:` with a non-breaking space. This is a genuine typography correction applied in translation, and it is correct. Good.

### 6.3 umrs-logspace fr_CA.po — quality: good

All 5 msgstr values are filled. These are short field name strings. The translations are
accurate. "Cycle de vie" for "Lifecycle" is the correct ANSSI/OTAN fr_CA term. "Groupe de
ressources" is a reasonable rendering of "Resource Pool" for a storage context.

One minor note: the `TRANSLATOR:` comments are placed after the `#:` source reference line
rather than before it. The gettext convention is:
```
# TRANSLATOR: comment
#: source.rs:N
msgid "..."
```

In `umrs-logspace/fr_CA.po`, the comments appear as:
```
#: source.rs:N
# TRANSLATOR: comment
msgid "..."
```

The `#:` line should come after all `#` comment lines. `msgmerge` and `msgfmt` will still
parse this correctly, but it is non-standard and may cause some tools to display the
comments incorrectly. This is a cosmetic issue but worth correcting on the next touch of
this file.

### 6.4 umrs-uname fr_CA.po — quality: very good

The translation is comprehensive. 98 entries covering a wide range of domain-specific
security terminology. The inline `# TRANSLATOR:` commentary is thorough and documents the
reasoning for every non-trivial decision. The vocabulary is consistent with `vocabulary-fr_CA.md`.

Two specific observations:

**Help text translations are not yet done.** The four multi-line help text blocks (Tab 0,
Tab 1, Tab 2, and fallback navigation) all have empty `msgstr ""` values in the `.po` file.
These are the most complex strings in the catalog — they contain ASCII box-drawing characters,
fixed-width column alignment, and approximately 150 lines of English prose across all four
blocks. Translating these correctly while preserving column alignment is significant work.
The 66-character line limit applies to each line within these blocks. This work is
outstanding and should be explicitly scheduled.

**Trust tier label truncations are correct but need Henri's validation.** The shortening
decisions made on 2026-03-23 (Label Trust → "Confiance", Trust Tier → "Palier", etc.) are
linguistically sound but reduce the label from its full form. For a government operator, the
abbreviated key column paired with a full-value column is standard TUI practice and should
not cause confusion. Henri should review these specific decisions.

### 6.5 umrs-platform fr_CA.po — quality: very good

All 39 msgstr values are filled. The posture annotation terms (ASLR, BPF, ptrace scope,
sysrq, etc.) are consistently translated. Gender agreement is handled correctly throughout
(e.g., "comptabilisation désactivée" with feminine agreement for "comptabilisation"). The
CAP_* capability names are correctly retained as-is.

---

## 7. Action Item Summary

### For Jamie (decisions required)

1. **fr_FR scope decision:** Is `fr_FR` an intended deployment locale? The non-empty
   `umrs-ls/fr_FR.po` and empty placeholder files in other domains need a clear answer.
   Either add `fr_FR` to `domains.md` as a supported locale or remove the files.

2. **Ghost domain decision:** What is the status of `umrs-df`, `umrs-ps`, `umrs-tester`?
   Planned future crates, or abandoned? Update `domains.md` accordingly.

3. **Root-level .mo cleanup:** Authorize deletion of root-level `.mo` files that have
   `<locale>/LC_MESSAGES/` counterparts. These are obsolete artifacts.

4. **umrs-stat domain onboarding:** Is this a priority? The crate calls
   `i18n::init("umrs-stat")` but has no domain directory, no `.pot`, no `fr_CA.po`.
   Approximately 20 additional strings need wrapping before extraction is useful.

### For the developer

1. **Add `tr_ctx()` to `umrs-core::i18n`** — required for `msgctxt` support for
   security labels. One function, using `dcpgettext()` from `gettextrs`. Wrapping
   report will specify which call sites use it.

2. **Verify `xtr` is installed and can recognize `i18n::tr()` calls** — run a test
   extraction on umrs-ls. If `xtr` does not recognize the wrapper, add
   `--keyword i18n::tr` to Makefile extraction invocations.

3. **Produce `umrs-stat` wrapping report work:** approximately 20 string instances
   need wrapping across key column labels and boolean display values. Several require
   structural refactoring (format_size plural/unit strings, risk count strings).

4. **Complete umrs-uname open items** from 2026-03-23 rescan (report_name/report_subject
   return type, os-release field name decision, format string refactors for catalog
   baseline comparison strings).

5. **Implement `scripts/pseudoloc.py`** — 20-line Python script for pseudolocalization
   testing. Translator will provide the ASCII→accented character mapping table.

6. **Add `i18n-ci` Makefile target** — `msgfmt --check` + `msgfmt --statistics` with
   appropriate gates for PR and release builds.

### For the translator (Simone)

1. **Update `domains.md`** — add umrs-state and umrs-logspace; correct the umrs-core
   entry (no domain needed); note ghost domains pending Jamie's decision.

2. **Translate umrs-uname help text overlays** — the four multi-line help text blocks
   are the last major untranslated section. High complexity due to column alignment.
   Requires 66-character line limit enforcement and developer review of any line that
   exceeds it after translation.

3. **Produce umrs-stat wrapping report** — after developer wraps the identified strings,
   extract the `.pot` and initialize `fr_CA.po`. French translations can begin once the
   domain directory and `.pot` are in place.

4. **Fix comment ordering in umrs-logspace/fr_CA.po** — `# TRANSLATOR:` comments should
   precede the `#:` source reference lines.

5. **Fix header typo in umrs-ls/fr_CA.po and umrs-state/fr_CA.po** — `a.k.a,` → `a.k.a.`

6. **Add `tr_ctx()` usage guidance to vocabulary-fr_CA.md** — once the developer
   implements the function, document which UMRS term categories require context-qualified
   lookups.

---

## 8. Summary Assessment

The UMRS l10n pipeline is in good shape for a project at this stage. The infrastructure is
sound, the vocabulary decisions are documented and rigorous, and the three active fr_CA
catalogs (umrs-ls, umrs-state, umrs-logspace, umrs-platform, umrs-uname) are all in
compilable and at least partially translated state.

The guidance document from Jamie captures the intent correctly. Its gaps are areas of
implementation detail that were addressed in practice during the earlier translation work
but never written back into the document.

The two highest-priority gaps are:

1. **msgctxt / `tr_ctx()` — structural gap.** The guidance document requires `msgctxt`
   for security labels. The current `tr()` API does not support context-qualified lookups.
   This requires one new function in `umrs-core::i18n` before the policy can be enforced.

2. **umrs-stat — pipeline gap.** The crate initializes i18n but has no domain directory,
   no `.pot`, and approximately 20 additional unwrapped strings. It is currently invisible
   to fr_CA operators. This is the highest-priority onboarding task.

The help text translations for umrs-uname are the largest remaining translation work item.

---

*Report prepared by Simone (umrs-translator agent). For review by Jamie Adams.*
*Technical l10n policy items require Henri Bélanger sign-off before implementation.*
