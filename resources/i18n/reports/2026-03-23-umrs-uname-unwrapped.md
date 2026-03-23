# Wrapping Report — umrs-uname

Crate: umrs-uname
Domain: umrs-uname
Date: 2026-03-23

---

## Summary

`umrs-uname/src/main.rs` has approximately 60 `i18n::tr()` call sites already in place.
Seven unwrapped (or incorrectly wrapped) items remain, grouped below.

**i18n crate**: `gettextrs = "0.7"` (same as all other UMRS crates).
**Domain**: `umrs-uname`
**Function to use**: `i18n::tr("msgid")` — already imported via `use umrs_core::i18n`.

---

## BUG — Incorrect domain name at `i18n::init`

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1778
current: i18n::init("umrs-ui");
correct: i18n::init("umrs-uname");
```

The binary is loading the `umrs-ui` gettext catalog at runtime. No `umrs-uname` catalog
is ever bound. Every `i18n::tr()` call in this binary silently falls back to the msgid
(English) because the correct domain name was never passed to `bindtextdomain`. Fix
this before any `.mo` compilation has value.

---

## Item 1 — `label_trust_display`: four unwrapped return strings

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1402–1426
function: label_trust_display()

  string: "UntrustedLabelCandidate — do not use for policy"
  macro to use: i18n::tr("UntrustedLabelCandidate — do not use for policy").to_owned()
    → replace: "UntrustedLabelCandidate — do not use for policy".to_owned()

  string: "LabelClaim — structurally valid; integrity unconfirmed"
  macro to use: i18n::tr("LabelClaim — structurally valid; integrity unconfirmed").to_owned()
    → replace: "LabelClaim — structurally valid; integrity unconfirmed".to_owned()

  string: "TrustedLabel — T4: ownership + digest verified"
  macro to use: i18n::tr("TrustedLabel — T4: ownership + digest verified").to_owned()
    → replace: "TrustedLabel — T4: ownership + digest verified".to_owned()

  string: "Verified w/ Contradiction — T4 integrity + conflict"
  macro to use: i18n::tr("Verified w/ Contradiction — T4 integrity + conflict").to_owned()
    → replace: "Verified w/ Contradiction — T4 integrity + conflict".to_owned()
```

All four are operator-facing trust classification strings displayed in the pinned Trust /
Evidence summary pane. They must be translated.

---

## Item 2 — `indicator_group_rows`: unwrapped "Configured:" prefix

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1112
string: "Configured: {raw} (from {source_file})"
macro to use: format!("{} {} ({} {})", i18n::tr("Configured:"), raw,
                      i18n::tr("from"), source_file)
```

The format string `"Configured: {raw} (from {source_file})"` is not currently wrapped.
The static prefix `"Configured:"` and the static fragment `"from"` should be translated
separately; the dynamic values `raw` and `source_file` are runtime data and must not
appear as msgids.

Note: the parenthesised structure `(from {source_file})` may need restructuring in French
to accommodate word order. The translator will produce a suitable split; the developer
should follow the msgid structure provided in the `.pot`.

---

## Item 3 — Help text Tab 1 (Kernel Security): raw string, not wrapped

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1521–1596
match arm: tab_index == 1
```

Tab 1 help text is a raw `r"..."` string literal passed directly as a `match` arm
return value — it is **not** wrapped with `i18n::tr()`. It should be:

```rust
1 => { i18n::tr(r" KERNEL SECURITY Tab
   ◼ Shows the settings ...
   ...
   Enter, Esc, or q    close this help") }
```

**Width constraint (CRITICAL)**: This string contains fixed-width ASCII layout with
carefully aligned columns (e.g., the `NAVIGATION:` block uses space padding to align
`switch between tabs`, `scroll this help`, etc. to the same column). French text is
15–30% longer. The translator will flag any line that cannot fit within the original
column width and propose an abbreviation for developer review before committing.

---

## Item 4 — Help text Tab 2 (Trust / Evidence): raw string, not wrapped

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1602–1679
match arm: tab_index == 2
```

Tab 2 help text is a raw `r"..."` string literal — not wrapped with `i18n::tr()`.
Same fix pattern as Item 3.

**Width constraint (CRITICAL)**: The `TRUST TIERS:` table has a fixed two-column layout
(tier name left, description right, arrow `⭭` alignment). French tier names and
descriptions will be longer. The translator will produce translations that preserve the
visual structure, with a flag for any line that requires developer review.

---

## Item 5 — Fallback navigation text: raw string, not wrapped

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1684–1689
match arm: _ (fallback)
string: r" NAVIGATION:
   Tab / Shift-Tab     switch between tabs
   j / k  or  ↑ / ↓   scroll this help
   PgDn / PgUp         scroll faster
   Enter, Esc, or q    close this help"
macro to use: i18n::tr(r"...")
```

---

## Note on `const fn help_text_for_tab`

`help_text_for_tab` is declared `const fn`. The Tab 0 arm at line 1487 already calls
`i18n::tr()` inside it, which is a Rust compile error — `i18n::tr()` is a runtime
function and cannot be evaluated in a `const` context. The developer must change
`const fn help_text_for_tab` to a regular `fn` before any i18n wrapping of the
help text arms will compile.

---

## Item 6 — Two "unavailable" fallback strings: unwrapped

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 135
  function: os_name_from_release()
  string: "unavailable"
  macro to use: return i18n::tr("unavailable");

line: 1189
  function: indicator_to_display()
  string: "unavailable"
  macro to use: (i18n::tr("unavailable"), StyleHint::Dim)
```

Both are operator-facing fallback labels shown when the OS release or an indicator value
is absent. The same msgid `"unavailable"` covers both — a single entry in the `.pot`.

---

## Strings intentionally excluded

- `report_name()` → `"OS Detection"` and `report_subject()` → `"Platform Identity and Integrity"`:
  these are internal metadata strings used for JSON/report output identifiers, not
  operator-facing display labels. Exclusion confirmed.
- `"(no data)"` / `"(invalid tab index)"` at line 347: internal fallback guard, never
  seen by operators in a correctly built binary.
- `"  lockdown (header)"` at line 1012: internal label string used as a technical key,
  not a human-facing label.
- Distro name strings in `distro_label()`: proper nouns (RHEL, Fedora, Ubuntu, etc.) —
  not translated.
- `trust_level_label()` and `trust_level_description()` return values: already wrapped
  with `i18n::tr()` at their call sites.
- `source_kind_label()` return values: already wrapped at call sites.
- `family_label()` return values: already wrapped at call sites.

---

## Implementation order recommended

1. Fix the `i18n::init("umrs-ui")` bug (Item BUG above). Without this, no translation
   is ever loaded at runtime.
2. Change `const fn help_text_for_tab` to `fn`. The Tab 0 arm already calls `i18n::tr()`,
   which proves it is not `const`-compatible.
3. Wrap help text arms (Items 3, 4, 5).
4. Wrap `label_trust_display` return strings (Item 1).
5. Wrap the "Configured:" format fragment (Item 2).
6. Wrap the two "unavailable" fallback strings (Item 6).
