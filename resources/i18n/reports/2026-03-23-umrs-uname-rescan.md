# umrs-uname i18n Rescan — Unwrapped Strings

Crate: `umrs-uname`
Domain: `umrs-uname`
Date: 2026-03-23
Scope: Full pass after first translation session. Identifies every operator-visible string
not currently wrapped with `i18n::tr()`.

Previous report: `2026-03-23-umrs-uname-unwrapped.md` (first-pass, pre-translation session).
This report is the follow-up rescan after the initial translation was confirmed working.

---

## Summary of Findings

| # | File | Line | String | Action |
|---|---|---|---|---|
| 1 | main.rs | 321 | `"OS Detection"` | Wrap with `i18n::tr()` |
| 2 | main.rs | 325 | `"Platform Identity and Integrity"` | Wrap with `i18n::tr()` |
| 3 | main.rs | 347 | `"(no data)"` and `"(invalid tab index)"` | Wrap both |
| 4 | main.rs | 375 | `"ID"` (os-release field key) | Flag — see note |
| 5 | main.rs | 379 | `"NAME"` (os-release field key) | Flag — see note |
| 6 | main.rs | 381 | `"VERSION_ID"` (os-release field key) | Flag — see note |
| 7 | main.rs | 384 | `"PRETTY_NAME"` (os-release field key) | Flag — see note |
| 8 | main.rs | 387 | `"CPE_NAME"` (os-release field key) | Flag — see note |
| 9 | main.rs | 391 | `"os-release"` (row key for absent file) | Flag — see note |
| 10 | main.rs | 438 | `"boot_id"` (row key) | Flag — see note |
| 11 | main.rs | 686 | `format!("baseline {CATALOG_KERNEL_BASELINE}")` | Refactor — see note |
| 12 | main.rs | 692–693 | Format string: `"{r} is newer than catalog baseline ({b}) — some indicators may have changed"` | Refactor — see note |
| 13 | main.rs | 705–706 | Format string: `"{r} is older than catalog baseline ({b}) — update your kernel"` | Refactor — see note |
| 14 | main.rs | 788 | `format!("{readable} readable — all hardened ✓")` | Refactor — see note |
| 15 | main.rs | 799–800 | `format!("{readable} readable — {hardened} hardened, {not_hardened} not hardened ({pct}%)")` | Refactor — see note |
| 16 | main.rs | 1804 | `"unavailable"` (initial os_name placeholder) | Wrap with `i18n::tr()` |

---

## Detailed Entries

---

### 1 — report_name()

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 321
string: "OS Detection"
macro to use: i18n::tr()
context: AuditCardApp::report_name() — displayed in the TUI header as part of
         the combined "OS Detection / Platform Identity and Integrity" assessment line.
```

Note: `report_name()` returns `&'static str`. The developer must change the return type to
`String` (or introduce a wrapper) to accommodate `i18n::tr()` which returns `String`.
Alternatively, if the trait supports a `String` return type for this method, switch to that.
Check the `AuditCardApp` trait definition in `umrs-ui/src/app.rs` — if it already returns
`String`, no change to the trait is needed; just change the implementation.

---

### 2 — report_subject()

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 325
string: "Platform Identity and Integrity"
macro to use: i18n::tr()
context: AuditCardApp::report_subject() — displayed in the TUI header alongside
         report_name(). Same return-type note as above applies.
```

---

### 3 — data_rows() fallback

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 347
string: "(no data)" (first argument)
string: "(invalid tab index)" (second argument)
macro to use: i18n::tr()
context: Fallback row shown when an out-of-range tab index is requested.
         Operator-visible if a tab index bug occurs. Both strings must be wrapped.
```

Suggested change:
```rust
_ => vec![DataRow::normal(i18n::tr("(no data)"), i18n::tr("(invalid tab index)"))],
```

---

### 4–8 — os-release field name keys (ID, NAME, VERSION_ID, PRETTY_NAME, CPE_NAME)

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
lines: 375, 379, 381, 384, 387
strings: "ID", "NAME", "VERSION_ID", "PRETTY_NAME", "CPE_NAME"
macro to use: i18n::tr()  — conditional, see note
```

**Translator recommendation**: These are standardized field names from the
`/etc/os-release` specification (freedesktop.org). French-speaking operators reading the
TUI will cross-reference these labels with the actual file on the system. Translating them
(e.g., "ID" → "ID", "NAME" → "NOM") would introduce a mismatch between the TUI label and
the file key.

**Decision for developer**: Leave these as technical identifiers (no `i18n::tr()` wrap)
unless the design intent is to display a localized label alongside the raw field name.
If the intent is full localization, wrap them and the translator will provide:
- `"ID"` → `"ID"` (unchanged — it is an identifier, not a word)
- `"NAME"` → `"NOM"` (corpus:coreutils/tar: "NAME" → "NOM")
- `"VERSION_ID"` → `"ID_VERSION"` (UMRS decision — os-release key, not prose)
- `"PRETTY_NAME"` → `"NOM_COMPLET"` (UMRS decision)
- `"CPE_NAME"` → `"CPE_NAME"` (unchanged — CPE is a NIST standard identifier)

**Translator note**: If the developer confirms these should be translated, msgids and msgstrs
will be added to the .po file. Currently omitted pending that decision.

---

### 9 — "os-release" row key (absent file case)

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 391
string: "os-release"
macro to use: i18n::tr()
context: Row label when /etc/os-release could not be read. Used as the left-column key
         in a DataRow::new(). Value is the translated "not available".
```

**Translator note**: "os-release" is a filename — it is not translated. The wrap is needed
so the string is extractable, but the French msgstr will be identical: `"os-release"`.
If the developer prefers, they could replace this with a more descriptive key such as
`i18n::tr("OS release file")` — translator will provide `"Fichier os-release"`.

---

### 10 — "boot_id" row key

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 438
string: "boot_id"
macro to use: i18n::tr()
context: Row label for the kernel boot identifier. Used as the left-column key
         in a DataRow::normal(). The boot_id value is a UUID — not translated.
```

**Translator note**: "boot_id" is a kernel sysfs/procfs field name. French msgstr will be
`"boot_id"` (retained as a technical identifier) unless the developer prefers a localized
label such as `i18n::tr("Boot ID")` → `"Identifiant de démarrage"`.

---

### 11 — Catalog baseline (parse failure case)

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 686
string: format!("baseline {CATALOG_KERNEL_BASELINE}")
macro to use: refactor required
context: Value string shown when the kernel version string cannot be parsed as
         MAJOR.MINOR.PATCH. The static word "baseline" is English.
```

**Recommended refactor:**
```rust
format!("{} {}", i18n::tr("baseline"), CATALOG_KERNEL_BASELINE)
```

Or, if "baseline" is awkward as a standalone word, use a full template msgid:
```rust
format!("{} {CATALOG_KERNEL_BASELINE}", i18n::tr("catalog:"))
```

**Translator note for `"baseline"` standalone**: No corpus match. UMRS vocabulary
uses `"base de référence"` for the full phrase "Catalog Baseline". For the bare word
used as a prefix before a version string, the appropriate translation is `"référence"`.
Display result: `"référence 6.12.0"`. This will be added to the .po file once the
developer confirms the wrapping approach.

---

### 12 — "newer than catalog baseline" message

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
lines: 692–693
string: "{r} is newer than catalog baseline ({b}) — some indicators may have changed"
macro to use: refactor required
context: Value string when the running kernel is newer than the catalog target version.
         The static English text is the description; {r} and {b} are version values.
```

**Recommended refactor** — extract static template as a msgid:

```rust
format!(
    "{} {} {} ({}) {} {}",
    r,
    i18n::tr("is newer than catalog baseline"),
    "",  // absorbed by template
    b,
    i18n::tr("—"),
    i18n::tr("some indicators may have changed")
)
```

Better approach — use a single msgid template with `{r}` and `{b}` as named placeholders,
and perform substitution after translation:

```rust
let template = i18n::tr(
    "{r} is newer than catalog baseline ({b}) \u{2014} some indicators may have changed"
);
template
    .replace("{r}", &r.to_string())
    .replace("{b}", &b.to_string())
```

**Translator will provide** the French msgstr with `{r}` and `{b}` placeholders
preserved in the correct grammatical position.

---

### 13 — "older than catalog baseline" message

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
lines: 705–706
string: "{r} is older than catalog baseline ({b}) — update your kernel"
macro to use: refactor required
context: Value string when the running kernel is older than the catalog target version.
         Rare case. Same placeholder pattern as #12.
```

**Recommended refactor** — same template substitution approach as #12:

```rust
let template = i18n::tr(
    "{r} is older than catalog baseline ({b}) \u{2014} update your kernel"
);
template
    .replace("{r}", &r.to_string())
    .replace("{b}", &b.to_string())
```

---

### 14 — Indicators summary: all-hardened case

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 788
string: format!("{readable} readable — all hardened ✓")
macro to use: refactor required
context: Shown in the Kernel Security pinned summary when all readable indicators
         meet the hardened baseline. The numeric count {readable} is dynamic.
```

**Recommended refactor:**

```rust
format!(
    "{} {} {}",
    readable,
    i18n::tr("readable — all hardened"),
    "\u{2713}"
)
```

Or using template substitution:

```rust
i18n::tr("{readable} readable \u{2014} all hardened \u{2713}")
    .replace("{readable}", &readable.to_string())
```

---

### 15 — Indicators summary: mixed case

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
lines: 799–800
string: format!("{readable} readable — {hardened} hardened, {not_hardened} not hardened ({pct}%)")
macro to use: refactor required
context: Shown when some indicators do not meet the hardened baseline.
         Four dynamic numeric values: readable, hardened, not_hardened, pct.
```

**Recommended refactor** — template substitution:

```rust
i18n::tr(
    "{readable} readable \u{2014} {hardened} hardened, {not_hardened} not hardened ({pct}%)"
)
.replace("{readable}", &readable.to_string())
.replace("{hardened}", &hardened.to_string())
.replace("{not_hardened}", &not_hardened.to_string())
.replace("{pct}", &pct.to_string())
```

---

### 16 — Initial os_name placeholder

```
file: components/rusty-gadgets/umrs-uname/src/main.rs
line: 1804
string: "unavailable"
macro to use: i18n::tr()
context: Passed as the initial os_name to build_header_context(). This value is
         displayed in the TUI header as the OS name while detection runs, and
         remains visible if detection fails before os_name is updated (line 1829).
         msgid "unavailable" is already in the .po file with msgstr "indisponible".
```

Suggested change (line 1804):
```rust
build_header_context(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION"),
    i18n::tr("unavailable"),
)
```

Note: `build_header_context` accepts `impl Into<String>` for `os_name`, so passing
`i18n::tr("unavailable")` (which returns `String`) requires no signature change.

---

## Strings Reviewed and Confirmed Correct (No Wrap Needed)

| String | Reason |
|---|---|
| `"RHEL"`, `"Fedora"`, `"Ubuntu"`, etc. in `distro_label()` | Proper nouns / brand names — not translated |
| All `log::debug!`, `log::warn!`, `log::error!` strings | Developer-facing logs — stay English |
| `"fd"`, `"PROC_MAGIC"`, `"SYS_MAGIC"`, `"statfs"` in `evidence_verification_str()` | Technical verification codes — displayed verbatim in TUI, retained as identifiers (also appear in help text) |
| `"sha256:"`, `"pkg digest (...)"` in evidence rows | Technical digest labels — format string data, not localized prose |
| `"lockdown"`, `"kexec_load_disabled"`, etc. in `build_kernel_security_rows()` | sysctl/kernel parameter names — technical identifiers |
| `"bluetooth (blacklisted)"`, `"usb_storage (blacklisted)"`, etc. | Module names with status annotation — see note below |
| `format!("  {label}")` in `indicator_group_rows()` | The `{label}` values are sysctl names — technical identifiers |

**Note on `"(blacklisted)"` suffix**: The word `"blacklisted"` appears in the indicator
label strings passed to `append_indicator_group()` (lines 934, 937, 941, 945). These are
sysctl/module parameter labels used as row identifiers. They appear in the indicator column
alongside the module name. If the developer wants these localized, the appropriate
translation would be `"(bloqué)"` (blocked/blacklisted). This is a developer judgment call —
raise as a separate follow-up if desired.

---

## Developer Action Required

The following items require source changes BEFORE the translator can produce final translations:

1. **Items 1–2** (`report_name`, `report_subject`): Confirm whether the `AuditCardApp` trait
   returns `&'static str` or `String` for these methods. If `&'static str`, the return type
   must change to `String` to allow runtime translation.

2. **Items 12–15** (format strings with embedded English): Adopt the template substitution
   pattern documented above. Translator will then add msgids and msgstrs for the full
   template strings.

3. **Items 4–10** (technical field names): Developer confirms whether these should be
   localized or retained as technical identifiers. Translator awaits decision before adding
   .po entries.

---

## New .po Entries Added This Session

The following msgids have been added to `resources/i18n/umrs-uname/fr_CA.po`:

- `"OS Detection"` → `"Détection OS"`
- `"Platform Identity and Integrity"` → `"Identité et intégrité de plateforme"`
- `"(no data)"` → `"(aucune donnée)"`
- `"(invalid tab index)"` → `"(index d'onglet invalide)"`
- `"unavailable"` is already present — line 1804 wrap just re-uses the existing msgid

Template msgids for items 12–15 are added to the .po file as `#, fuzzy` pending developer
confirmation of the refactoring approach.
