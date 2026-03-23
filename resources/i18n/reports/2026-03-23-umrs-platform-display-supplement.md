# Supplement Report — umrs-platform (posture/display.rs)

Crate: umrs-platform
Domain: umrs-platform
Date: 2026-03-23

---

## Scope

This report supplements the earlier wrapping report
(`resources/i18n/reports/2026-03-11-umrs-platform-unwrapped.md`) with the display
annotation strings from `posture/display.rs`. These strings are the agreed i18n scope
for `umrs-platform` — they are the canonical, operator-facing value labels that every
consumer (TUI, CLI, JSON output) must call rather than duplicating. Translating them
at the library level ensures consistent terminology across all binaries.

The earlier report covered `log::warn!` / `log::error!` strings and `thiserror`
`#[error]` strings. That scope decision (wrap at binary display boundary) is unchanged.

---

## String wrapping instructions

Apply `tr_platform("msgid")` at each site below. The `tr_platform()` function must be
added to `umrs-platform/src/i18n.rs` as described in the 2026-03-11 report.

### annotate_live_value (src/posture/display.rs lines 56–68)

```
file: components/rusty-gadgets/umrs-platform/src/posture/display.rs
line: 56
  string: "enabled"
  current: "enabled".to_owned()
  macro to use: tr_platform("enabled")

line: 57
  string: "disabled"
  current: "disabled".to_owned()
  macro to use: tr_platform("disabled")

line: 65
  string: "Not Present"
  current: "Not Present".to_owned()
  macro to use: tr_platform("Not Present")
```

---

### annotate_integer — IndicatorId::RandomizeVaSpace (lines 93–98)

```
line: 94
  string: "ASLR disabled"
  macro to use: tr_platform("ASLR disabled")

line: 95
  string: "partial randomization"
  macro to use: tr_platform("partial randomization")

line: 96
  string: "full ASLR"
  macro to use: tr_platform("full ASLR")
```

---

### annotate_integer — IndicatorId::KptrRestrict (lines 100–104)

```
line: 101
  string: "pointers visible"
  macro to use: tr_platform("pointers visible")

line: 102
  string: "hidden from unprivileged"
  macro to use: tr_platform("hidden from unprivileged")

line: 103
  string: "hidden from all users"
  macro to use: tr_platform("hidden from all users")
```

---

### annotate_integer — IndicatorId::UnprivBpfDisabled (lines 106–109)

```
line: 107
  string: "unprivileged BPF allowed"
  macro to use: tr_platform("unprivileged BPF allowed")

line: 108
  string: "restricted to CAP_BPF"
  macro to use: tr_platform("restricted to CAP_BPF")
```

Note: `CAP_BPF` is a Linux capability name — it must not be translated. It appears
inside the msgid for translator context only; the translator will retain it verbatim
in the French msgstr.

---

### annotate_integer — IndicatorId::YamaPtraceScope (lines 111–116)

```
line: 112
  string: "unrestricted"
  macro to use: tr_platform("unrestricted")

line: 113
  string: "children only"
  macro to use: tr_platform("children only")

line: 114
  string: "admin only"
  macro to use: tr_platform("admin only")

line: 115
  string: "no attach"
  macro to use: tr_platform("no attach")
```

---

### annotate_integer — IndicatorId::DmesgRestrict (lines 118–121)

```
line: 119
  string: "world-readable"
  macro to use: tr_platform("world-readable")

line: 120
  string: "restricted"
  macro to use: tr_platform("restricted")
```

---

### annotate_integer — IndicatorId::ModulesDisabled (lines 123–126)

```
line: 124
  string: "loading allowed"
  macro to use: tr_platform("loading allowed")

line: 125
  string: "loading locked"
  macro to use: tr_platform("loading locked")
```

---

### annotate_integer — IndicatorId::UnprivUsernsClone (lines 128–131)

```
line: 129
  string: "restricted"
  NOTE: same msgid as DmesgRestrict arm above — single .pot entry covers both.

line: 130
  string: "allowed"
  macro to use: tr_platform("allowed")
```

---

### annotate_integer — IndicatorId::Sysrq (lines 133–136)

```
line: 134
  string: "fully disabled"
  macro to use: tr_platform("fully disabled")

line: 135
  string: "all functions enabled"
  macro to use: tr_platform("all functions enabled")
```

---

### annotate_integer — IndicatorId::SuidDumpable (lines 138–142)

```
line: 139
  string: "no core dumps"
  macro to use: tr_platform("no core dumps")

line: 140
  string: "core dumps enabled"
  macro to use: tr_platform("core dumps enabled")

line: 141
  string: "readable by root only"
  macro to use: tr_platform("readable by root only")
```

---

### annotate_integer — IndicatorId::ProtectedSymlinks/ProtectedHardlinks (lines 144–149)

```
line: 145
  string: "not protected"
  macro to use: tr_platform("not protected")

line: 146
  string: "protected"
  macro to use: tr_platform("protected")
```

---

### annotate_integer — IndicatorId::ProtectedFifos/ProtectedRegular (lines 151–156)

```
line: 152
  string: "not protected"
  NOTE: same msgid as above — single .pot entry covers all four arms.

line: 153
  string: "partial protection"
  macro to use: tr_platform("partial protection")

line: 154
  string: "fully protected"
  macro to use: tr_platform("fully protected")
```

---

### annotate_integer — IndicatorId::FipsEnabled (lines 159–162)

```
line: 160
  string: "Disabled"
  macro to use: tr_platform("Disabled")

line: 161
  string: "Enabled"
  macro to use: tr_platform("Enabled")
```

Note: capitalised forms ("Enabled", "Disabled") are distinct msgids from the lowercase
forms used in `annotate_live_value`. Both must appear in the `.pot`.

---

### annotate_integer — IndicatorId::NfConntrackAcct (lines 164–167)

```
line: 165
  string: "accounting off"
  macro to use: tr_platform("accounting off")

line: 166
  string: "accounting on"
  macro to use: tr_platform("accounting on")
```

---

### annotate_signed_integer — IndicatorId::PerfEventParanoid (lines 196–201)

```
line: 197
  string: "fully open"
  macro to use: tr_platform("fully open")

line: 198
  string: "kernel profiling allowed"
  macro to use: tr_platform("kernel profiling allowed")

line: 199
  string: "user profiling allowed"
  macro to use: tr_platform("user profiling allowed")

line: 200
  string: "restricted"
  NOTE: same msgid already covered above — single .pot entry.
```

---

## Wrapping pattern

The `format!("{v} ({note})")` call at line 174 and `format!("{v} ({note})")` at line 206
must be restructured so the parentheses and spacing are translatable:

```rust
// Before:
format!("{v} ({note})")

// After:
format!("{v} ({})", tr_platform(note))
```

The static annotation string is the msgid. The numeric value `{v}` is dynamic and must
not be part of the msgid.

---

## Strings intentionally excluded

- The numeric fallback `v.to_string()` / `v.to_string()` at lines 177 and 208: pure
  integer-to-string conversion, not a user-facing label.
- `IndicatorId` variant names as match arms: internal Rust identifiers, not surfaced
  to operators.
- `//` doc comments and `//!` module comments: documentation only.
