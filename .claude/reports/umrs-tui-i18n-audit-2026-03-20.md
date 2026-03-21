# umrs-tui i18n Audit Report

**Crate:** umrs-tui
**Domain:** umrs-tui (confirmed — `i18n::init("umrs-tui")` at line 1906 of `main.rs`)
**Date:** 2026-03-20
**Auditor:** umrs-translator agent
**Status:** Audit complete — no source modifications made

---

## 1. Domain Verification

The binary correctly initialises its text domain at startup:

```rust
// main.rs line 1906
i18n::init("umrs-tui");
```

This call must occur before any `i18n::tr()` invocation. Verified: it is the
first statement in `main()`, before logging setup and header context
construction. Domain assignment is correct.

---

## 2. Currently Wrapped Strings (i18n::tr() already present)

The following strings are already wrapped and will be extracted by `xtr`.
This section serves as the baseline for `.pot` generation.

### main.rs — Tab labels (lines 215–217, 301–303)

```
"OS Information"
"Kernel Security"
"Trust / Evidence"
```

### main.rs — Error path (from_error builder, lines 254–294)

```
"Hard gate: procfs is not real procfs"
"Hard gate: PID coherence broken"
"Hard gate: I/O error during kernel anchor"
"Status"
"Detection pipeline failed"
"Reason"
"Trust Level"
"T0 — Untrusted"
"Hard gate failure aborted pipeline"
"Detection pipeline failed"   [also used as StatusMessage text]
```

### main.rs — OS info rows (lines 391–431)

```
"not available"              [os-release absent]
"Platform Family"
"Platform Distro"
"Platform Version"
"Platform Facts"
"Probe Used"
"Platform Identity"
"not available"              [substrate absent]
"not available"              [boot_id absent]
```

Note: `"not available"` appears in three separate call sites. All three should
translate identically. The `xtr` tool will produce one msgid entry.

### main.rs — AuditCardApp impl (line 328)

```
"OS Detection Audit"         [card_title()]
```

### main.rs — Trust summary rows (lines 466–564)

```
"Label Trust"
"Trust Tier"
i18n::tr(trust_level_label(level))    [resolves to one of 5 trust_level_label strings]
"Description"
i18n::tr(trust_level_description(level))  [resolves to one of 5 description strings]
"Downgrade Reasons"
"No downgrade — full trust retained"
"Each reason below prevented a trust tier upgrade."
"Contradictions"
"None detected"
"All evidence sources agreed \u{2014} no conflicting identity assertions detected."
"Two independent sources reported conflicting values. Review each pair — this may indicate tampering."
"Evidence Records"
```

### main.rs — trust_level_label() helper (lines 66–72, const fn)

These are `const fn` returning `&'static str`. They are called via
`i18n::tr(trust_level_label(level))` — the string is materialised at runtime
and then passed to `tr()`. This is valid: `xtr` will see the string literals
inside `trust_level_label()` as translatable candidates if the function is
scanned. See Section 4 for the wrapping note.

```
"T0 — Untrusted"
"T1 — Kernel Anchored"
"T2 — Environment Anchored"
"T3 — Platform Verified"
"T4 — Integrity Anchored"
```

### main.rs — trust_level_description() helper (lines 75–91, const fn)

Same pattern as above.

```
"No kernel anchor established."
"procfs verified via PROC_SUPER_MAGIC + PID coherence."
"Mount topology cross-checked (mountinfo vs statfs)."
"Platform identity verified; >= 2 independent package facts confirmed."
"os-release ownership + installed digest verified."
```

### main.rs — build_status() (lines 1762–1787)

```
"Integrity Anchored"
"Platform Verified"
i18n::tr(trust_level_label(TrustLevel::EnvAnchored))
i18n::tr(trust_level_description(TrustLevel::EnvAnchored))
i18n::tr(trust_level_label(TrustLevel::KernelAnchored))
i18n::tr(trust_level_description(TrustLevel::KernelAnchored))
"Untrusted — no kernel anchor"
```

### main.rs — Kernel Security summary (lines 678–768)

```
"Kernel Version"
"All values below are read live from the running kernel via /proc and /sys."
"Indicators"
"No Assessment"
"Contradictions"
"None"
"No disagreements between running kernel and persisted configuration."
"The running kernel and persisted configuration disagree on one or more settings. Drift means intended hardening is not active; hotfixes mean current hardening will be lost on reboot."
"Curated indicators selected to give you the clearest view of your system's security posture. Items marked in red can be hardened — see each indicator's recommended setting below."
```

### main.rs — Kernel Security group titles and descriptions (lines 800–888, 950–996)

All group titles and descriptions are passed through `i18n::tr()` via
`append_indicator_group()` and `append_boot_integrity_group()`:

```
"BOOT INTEGRITY"
"Verifies the kernel loaded in a tamper-resistant state and cannot be silently replaced at runtime."
"CRYPTOGRAPHIC POSTURE"
"Verifies government-validated cryptography and correct entropy sourcing. Failures here mean sensitive operations may use unvalidated algorithms."
"KERNEL SELF-PROTECTION"
"Controls that hide kernel internals from unprivileged processes. Weak settings let attackers locate exploitable code and bypass ASLR."
"PROCESS ISOLATION"
"Controls how much one process can see or interfere with another. Weak settings allow credential theft across sibling processes."
"FILESYSTEM HARDENING"
"Closes privilege escalation paths through the filesystem. Absent controls allow symlink and hardlink attacks in world-writable directories."
"MODULE RESTRICTIONS"
"Verifies high-risk kernel modules are blocked from loading. USB, FireWire, and Thunderbolt are primary data exfiltration and DMA attack vectors."
"NETWORK AUDITING"
"Controls that enable traffic accounting for anomaly detection and forensic reconstruction. Without these, network audit logs lack the volume data needed to identify exfiltration."
```

### main.rs — Evidence table headers (lines 1597–1599)

```
"Evidence Type"
"Source"
"Verification"
```

### main.rs — Evidence group labels / source_kind_label (lines 1618, 1624)

These are passed through `i18n::tr(source_kind_label(...))`:

```
"Kernel runtime (/proc)"
"Configuration file"
"Package database"
"Symlink target"
"Kernel attributes (/sys)"
"Filesystem identity"
```

### header.rs — Header field labels (lines 193–199, 274–277)

```
"Assessment"
"Host"
"Tool"
"OS"
"Assessed"
"SELinux"
"FIPS"
```

---

## 3. Unwrapped Strings — Developer Action Required

The following strings are operator-facing but are NOT currently wrapped in
`i18n::tr()`. They must be reviewed for translatability.

### 3.1 Strings in const fn contexts — CANNOT use i18n::tr()

These strings are in `const fn` functions. Rust's `const fn` evaluator cannot
call non-`const` functions like `i18n::tr()` at compile time, and using them
at runtime from inside a `const fn` is not permitted.

These require a different i18n strategy (see Section 5 for recommendation).

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1812–1818
string: (full help_text_for_tab tab 0 block)
  "OS Information\n\
   Shows identity fields from /etc/os-release, platform identity,\n\
   and boot ID. These fields identify the system under assessment.\n\
   \n\
   Navigation: Tab / Shift-Tab = switch tabs  j/k = scroll\n\
   \n\
   Press Enter, Esc, or q to close this help."
context: const fn help_text_for_tab(tab_index: usize) — cannot call i18n::tr()
macro to use: deferred translation (see Section 5)
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1821–1846
string: (full help_text_for_tab tab 1 block)
  "Kernel Security\n\
   Shows live kernel security posture from /proc and /sys.\n\
   [... full text ...]"
context: const fn help_text_for_tab(tab_index: usize) — cannot call i18n::tr()
macro to use: deferred translation (see Section 5)
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1848–1885
string: (full help_text_for_tab tab 2 block)
  "Trust / Evidence\n\
   [... full text ...]"
context: const fn help_text_for_tab(tab_index: usize) — cannot call i18n::tr()
macro to use: deferred translation (see Section 5)
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1888–1893
string: (help_text_for_tab fallback / unknown tab)
  "Press Tab / Shift-Tab to switch tabs.\n\
   Press j/k or Up/Down to scroll.\n\
   Press q or Esc to quit.\n\
   \n\
   Press Enter, Esc, or q to close this help."
context: const fn help_text_for_tab(tab_index: usize) — cannot call i18n::tr()
macro to use: deferred translation (see Section 5)
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`indicator_description()` — `const fn`, 37-entry match (lines 1018–1157)

All 37+ description strings in `indicator_description()` are operator-facing
and currently unwrapped. They cannot use `i18n::tr()` because the function is
`const fn`.

Representative sample (each is a distinct msgid):

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1021
string: "Kernel lockdown LSM restricts operations that let root modify the running kernel. Without it, boot-time integrity checks can be bypassed after the system is up."
context: const fn indicator_description() — cannot call i18n::tr()
```

```
line: 1026
string: "Prevents loading a new kernel image at runtime. Without this, an attacker with root can replace the running kernel without a reboot, bypassing Secure Boot."
```

```
line: 1031
string: "Requires all kernel modules to be cryptographically signed. Without this, any code can be loaded as a kernel module, defeating lockdown and enabling rootkits."
```

*(Full list of 27 populated indicator descriptions omitted for brevity — all
follow the same pattern and require the same deferred-translation strategy.)*

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`indicator_recommended()` — `const fn`, 20+ entries (lines 1180–1234)

All recommendation strings are operator-facing and currently unwrapped.
Same constraint: `const fn` cannot call `i18n::tr()`.

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1182
string: "2 (hidden from all users)"
context: const fn indicator_recommended() — cannot call i18n::tr()
```

*(Remaining entries: "2 (full ASLR)", "1 (restricted to CAP_BPF)", etc.)*

---

### 3.2 Strings that CAN be wrapped — runtime context

These strings are in regular (non-const) functions and could receive
`i18n::tr()` wrapping. They are currently unwrapped.

---

**File:** `components/rusty-gadgets/umrs-tui/src/dialog.rs`

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 440
string: " Information "
context: dialog title, match arm in render_dialog()
macro to use: i18n::tr()
note: Includes padding spaces — strip before wrapping; add padding after
      tr() returns. Developer should write: format!(" {} ", i18n::tr("Information"))
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 441
string: " Error "
context: dialog title, match arm in render_dialog()
macro to use: i18n::tr()
note: Same padding pattern as above.
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 442
string: " Security Warning "
context: dialog title, match arm in render_dialog()
macro to use: i18n::tr()
note: Same padding pattern.
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 443
string: " Confirm "
context: dialog title, match arm in render_dialog()
macro to use: i18n::tr()
note: Same padding pattern.
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 501
string: " [OK] "
context: single-button render_buttons(), Info/Error modes
macro to use: i18n::tr()
note: Strip padding before wrapping; format after tr().
      Bracket syntax "[OK]" must be preserved — it is a UI convention.
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 517
string: " [Cancel] "
context: render_buttons(), SecurityWarning mode, secondary button
macro to use: i18n::tr()
note: Strip padding before wrapping. "[Cancel]" convention preserved.
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 519
string: " [OK] "
context: render_buttons(), SecurityWarning mode, primary button
macro to use: i18n::tr()
note: Same msgid as line 501 — single msgid, two call sites.
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 535
string: " [No] "
context: render_buttons(), Confirm mode, secondary button
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/dialog.rs
line: 537
string: " [Yes] "
context: render_buttons(), Confirm mode, primary button
macro to use: i18n::tr()
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/status_bar.rs`

```
file: components/rusty-gadgets/umrs-tui/src/status_bar.rs
line: 39
string: "  Tab: tabs | ↑↓/jk: scroll | ?: help | q: quit"
context: KEY_LEGEND const — operator-visible key hint in status bar
macro to use: Cannot use i18n::tr() — declared as const
note: This is a const &str. If translation is desired, the approach is the
      same as for const fn strings (see Section 5). The arrow characters
      (↑↓) are Unicode and locale-neutral; only the text labels need translation.
      Low priority for fr_CA — key labels are typically not localized.
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 134–135
string: "unavailable"   [os_name_from_release fallback]
context: fn os_name_from_release() — not wrapped
macro to use: i18n::tr()
note: This is a data value, not a UI label. "unavailable" is used in three
      other places where it IS already wrapped. Wrapping here would be
      consistent. Decision for developer.
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 346
string: "(no data)"
context: fn data_rows() — invalid tab index fallback
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 346
string: "(invalid tab index)"
context: fn data_rows() — invalid tab index fallback
macro to use: i18n::tr()
note: This is an internal error state unlikely to reach an operator in normal
      operation, but it is operator-visible if it appears.
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`translate_live_value()` and helpers (lines 1354–1490)

The annotation strings used to augment integer indicator values are currently
unwrapped. These are operator-visible (rendered in the Kernel Security tab).

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 1364
string: "Not Present"
context: translate_live_value(), "absent" sentinel display
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
lines: 1384–1456  (translate_integer match arms)
context: fn translate_integer() — annotation strings for integer indicator values
macro to use: i18n::tr() on the annotation &'static str before format!()
note: These are inside a non-const fn, so i18n::tr() is callable.
      However, the annotations are passed to format!() as: format!("{v} ({note})").
      Developer must restructure to: format!("{v} ({})", i18n::tr(note))
      where note is the annotation string.
```

Sample annotation strings (all in translate_integer / translate_signed_integer):

```
"ASLR disabled"
"partial randomization"
"full ASLR"
"pointers visible"
"hidden from unprivileged"
"hidden from all users"
"unprivileged BPF allowed"
"restricted to CAP_BPF"
"unrestricted"
"children only"
"admin only"
"no attach"
"world-readable"
"restricted"
"loading allowed"
"loading locked"
"allowed"                        [UnprivUsernsClone value 1]
"fully disabled"
"all functions enabled"
"no core dumps"
"core dumps enabled"
"readable by root only"
"not protected"
"protected"
"partial protection"
"fully protected"
"Disabled"                       [FipsEnabled 0]
"Enabled"                        [FipsEnabled 1]
"accounting off"
"accounting on"
"fully open"                     [PerfEventParanoid]
"kernel profiling allowed"
"user profiling allowed"
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`evidence_verification_str()` (lines 1701–1717)

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
lines: 1701–1717
context: fn evidence_verification_str() — verification outcome strings
note: The method/detail components ("fd, PROC_MAGIC", "fd, SYS_MAGIC", etc.)
      are technical identifiers, not prose. Translating them would reduce
      precision. These should NOT be wrapped — they are technical constants.
      However, the surrounding "ok" and "FAIL" tokens are translatable.
      Recommend: wrap only the "ok" and "FAIL" tokens; keep the method tokens
      as literal English (they refer to POSIX/Linux API names).
macro to use: i18n::tr() on "ok" and "FAIL" only
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`label_trust_display()` (lines 1733–1757)

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
lines: 1733–1757
context: fn label_trust_display() — LabelTrust display strings
strings (currently unwrapped):
  "UntrustedLabelCandidate — do not use for policy"
  "LabelClaim — structurally valid; integrity unconfirmed"
  "TrustedLabel — T4: ownership + digest verified"
  "IntegrityVerifiedButContradictory: ..."  [format! with variable]
macro to use: i18n::tr() on the fixed portions; variable portion stays outside tr()
note: The "IntegrityVerifiedButContradictory" variant builds a string from a
      dynamic contradiction description. The prefix is wrappable; the dynamic
      suffix is not. Recommend splitting:
        format!("{}: {desc}", i18n::tr("IntegrityVerifiedButContradictory"))
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`distro_label()` (lines 142–154)

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
lines: 142–154
context: fn distro_label() — OS distribution name strings
strings: "RHEL", "Fedora", "CentOS", "AlmaLinux", "Rocky Linux",
         "Debian", "Ubuntu", "Kali Linux"
macro to use: NOT recommended for wrapping
note: These are proper nouns — vendor-controlled trademarked names. They must
      not be translated. Do not wrap these strings. They pass through as-is.
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
`family_label()` (lines 156–163)

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
lines: 156–163
context: const fn family_label() — OS family strings
strings: "RPM-based", "dpkg-based", "pacman-based", "unknown"
macro to use: i18n::tr() — but see const fn constraint
note: These are returned as &'static str from a const fn, but are already
      wrapped at the call site: i18n::tr(family_label(&sub.family))
      (line 402 of main.rs). The wrapping is correct. The individual strings
      inside family_label() do not need to be wrapped independently.
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/header.rs`

```
file: components/rusty-gadgets/umrs-tui/src/header.rs
line: 411
string: "unavailable"
context: fn indicator_text() — IndicatorValue::Unavailable display
macro to use: i18n::tr()
note: This is the same "unavailable" string used elsewhere. Should be wrapped
      for consistency with other call sites.
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/indicators.rs`

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
lines: 213–226
strings: "Enforcing ({pol})", "Enforcing", "Permissive ({pol})", "Permissive"
context: fn read_selinux_status() — SELinux enforcement mode display
macro to use: i18n::tr() on "Enforcing" and "Permissive" (without pol suffix);
              pol is a dynamic string (policy name like "Targeted") — wrap separately
note: Developer must restructure:
        format!("{} ({})", i18n::tr("Enforcing"), pol)
      or keep the full string unwrapped if "Enforcing (Targeted)" is considered
      a technical value rather than operator prose.
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
line: 241
string: "Enabled"
context: fn read_fips_mode() — FIPS mode display when active
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
line: 242
string: "Disabled"
context: fn read_fips_mode() — FIPS mode display when inactive
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
line: 256
string: "none"
context: fn read_lockdown_mode() — lockdown disabled display
macro to use: i18n::tr()
note: This is a kernel value — "none" is also the literal content of the
      kernel attribute. Wrapping it changes the display string but not the
      kernel value. Recommend wrapping for operator display consistency.
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
lines: 278, 282
string: "unavailable"
context: fn format_assessed_at() — fallback when clock read fails
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
lines: 324–326
strings: "(unknown)"  [three occurrences in read_uname_fields()]
context: fallback for non-UTF-8 uname fields
macro to use: i18n::tr()
note: Single msgid "(unknown)" — three call sites.
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
line: 343, 349, 351
string: "unavailable"
context: fn read_boot_id() — fallback on read failure
macro to use: i18n::tr()
```

```
file: components/rusty-gadgets/umrs-tui/src/indicators.rs
line: 372, 383, 385
string: "unavailable"
context: fn read_system_uuid() — fallback on read failure
macro to use: i18n::tr()
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
Kernel Security contradiction format string (line 616)

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 615–618
string: "{count} \u{2014} configuration/kernel disagreements detected"
context: append_kernel_contradiction_rows() — non-zero contradiction count display
macro to use: Partially wrappable — the template text is translatable;
              the count is dynamic.
note: Developer should restructure as:
        format!("{count} {} {}", "\u{2014}", i18n::tr("configuration/kernel disagreements detected"))
      or introduce a full format-string approach with a translator comment.
      The em dash (U+2014) is universal — need not be part of the msgid.
```

---

**File:** `components/rusty-gadgets/umrs-tui/src/main.rs`
Kernel Security indicator summary format strings (lines 718, 729–731)

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
line: 718
string: "{readable} readable \u{2014} all hardened \u{2713}"
context: build_kernel_security_summary_rows() — all-hardened case
macro to use: Partially wrappable
note: The fixed text around the dynamic count should be wrapped. Developer
      should restructure:
        format!("{readable} {} {} {}",
            i18n::tr("readable"),
            "\u{2014}",
            i18n::tr("all hardened"),
            "\u{2713}")
      The checkmark is a Unicode symbol, locale-neutral.
```

```
file: components/rusty-gadgets/umrs-tui/src/main.rs
lines: 729–731
string: "{readable} readable \u{2014} {hardened} hardened, {not_hardened} not hardened ({pct}%)"
context: build_kernel_security_summary_rows() — mixed hardening case
macro to use: Partially wrappable — complex format string with 4 dynamic values
note: This format string is difficult to wrap with the current approach because
      gettext requires a single translatable string with positional or named
      arguments. Recommend introducing a translator comment and wrapping the
      entire template, with the developer restructuring the format call.
      Flag for developer: this is a format string translation — a distinct
      gettext pattern. May require format!() restructuring.
```

---

## 4. Strings Confirmed as NOT Operator-Facing (no wrapping needed)

The following strings appear in source but are log output, internal identifiers,
or technical constants that must not be translated:

- `log::info!`, `log::warn!`, `log::error!`, `log::debug!` strings throughout — these go to journald, not the operator UI
- `"OS Detection"` and `"Platform Identity and Integrity"` in `report_name()` / `report_subject()` — these are OSCAL identifiers, not display text; they appear in the header only as technical record identifiers
- Evidence method tokens `"fd, PROC_MAGIC"`, `"fd, SYS_MAGIC"`, `"fd, statfs"`, `"fd"` — POSIX/Linux API names, must remain in English
- `"sha256:"`, `"pkg digest (sha512):"`, `"md5 (weak)"` in digest display — technical identifiers
- `"boot_id"` key name in OS info rows — kernel attribute name, stays in English
- `"ID"`, `"NAME"`, `"VERSION_ID"`, `"PRETTY_NAME"`, `"CPE_NAME"` in OS info rows — `os-release` field names, technical standard; must remain in English
- Algorithm names `"sha256"`, `"sha512"`, `"md5 (weak)"` — technical identifiers
- `"unavailable"` as a raw Rust identifier in the initial `os_name` argument (`"unavailable"` at line 1925) — this initial value is immediately replaced; not operator-visible in practice

---

## 5. Strategy for const fn Strings

`const fn` functions (`help_text_for_tab`, `indicator_description`,
`indicator_recommended`, `trust_level_label`, `trust_level_description`,
`family_label`, `KEY_LEGEND`) cannot call `i18n::tr()` because `tr()` performs
a runtime lookup.

**Recommended approach: Convert to regular fn, wrap at call site.**

The developer should change:
```rust
const fn help_text_for_tab(tab_index: usize) -> &'static str { ... }
```
to:
```rust
fn help_text_for_tab(tab_index: usize) -> String {
    match tab_index {
        0 => i18n::tr("OS Information\n\..."),
        ...
    }
}
```

For `indicator_description` and `indicator_recommended`, the same conversion
applies: from `const fn` returning `&'static str` to `fn` returning `String`.
Note that the call sites in `indicator_group_rows()` pass the results to
`DataRow::indicator_row_full()` — those signatures accept `impl Into<String>`,
so a `String` return from the helper is compatible.

For `trust_level_label` and `trust_level_description`: these are already
consumed via `i18n::tr(trust_level_label(level))`. The `const fn` produces the
msgid string; `i18n::tr()` translates it at the call site. This is a valid
pattern — `xtr` will still discover the string literals inside the `const fn`.
No restructuring needed for these two.

**KEY_LEGEND:** Convert from `const` to a `fn` returning `String` or
`&'static str` via a `once_cell` / `OnceLock`. Alternatively, accept that
the key legend is not translated in the initial release — key bindings are
typically locale-neutral.

---

## 6. Summary Counts

| Category | Count |
|---|---|
| Already wrapped with `i18n::tr()` | ~55 distinct msgid strings |
| Unwrapped — can receive `i18n::tr()` (runtime fn) | ~45 strings |
| Unwrapped — const fn constraint, need fn conversion | ~50+ strings (indicator descriptions + help text) |
| Do not translate (technical IDs, log messages, vendor names) | ~20 strings |

---

## 7. Domain Confirmation

- **Domain name:** `umrs-tui`
- **init call location:** `main.rs` line 1906, first statement in `main()`
- **`xtr` invocation (when ready to extract):**

```
xtr --package-name umrs-tui \
    --output resources/i18n/umrs-tui/umrs-tui.pot \
    components/rusty-gadgets/umrs-tui/src/main.rs \
    components/rusty-gadgets/umrs-tui/src/header.rs \
    components/rusty-gadgets/umrs-tui/src/dialog.rs \
    components/rusty-gadgets/umrs-tui/src/status_bar.rs \
    components/rusty-gadgets/umrs-tui/src/indicators.rs \
    components/rusty-gadgets/umrs-tui/src/tabs.rs \
    components/rusty-gadgets/umrs-tui/src/data_panel.rs \
    components/rusty-gadgets/umrs-tui/src/app.rs
```

Note: `xtr` should be run after the developer completes wrapping of unwrapped
strings. Running it now would produce an incomplete `.pot`.

---

## 8. Priority Order for Developer Wrapping

Recommended sequence for the developer (least to most disruptive):

1. **dialog.rs titles and buttons** — simple runtime fn, direct `i18n::tr()` around string
2. **indicators.rs** — `"Enabled"`, `"Disabled"`, `"none"`, `"unavailable"`, `"(unknown)"` — simple wraps
3. **header.rs** — `"unavailable"` in `indicator_text()`
4. **main.rs — label_trust_display()** — four strings, runtime fn
5. **main.rs — translate_live_value() / translate_integer()** — annotation strings (restructure format!)
6. **main.rs — format strings** with dynamic counts (contradiction count, indicators summary)
7. **main.rs — const fn conversion** for `indicator_description()` and `indicator_recommended()`
8. **main.rs — const fn conversion** for `help_text_for_tab()`
9. **status_bar.rs — KEY_LEGEND** — lowest priority; consider deferring to a later release

---

*This report was produced by the umrs-translator agent. No source files were modified.*
