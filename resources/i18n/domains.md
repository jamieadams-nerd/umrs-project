# UMRS i18n Text Domain Assignments

Maintained by the `umrs-translator` agent. One row per crate. Updated whenever a new crate
is onboarded to the i18n pipeline.

---

## Domain Assignment Table

**Status key:** Active = .pot + fr_CA.po + compiled .mo present. Pending = domain directory
exists but pipeline incomplete. Reserved = placeholder for a planned crate, not yet in workspace.
Incorrect = entry is wrong and should be removed or corrected.

| Crate | Crate Type | Text Domain | Domain Directory | Pipeline Status | Notes |
|---|---|---|---|---|---|
| `umrs-ls` | binary | `umrs-ls` | `resources/i18n/umrs-ls/` | Active | First binary crate in pipeline; full fr_CA catalog compiled |
| `umrs-core` | library | *(no domain)* | *(none)* | Incorrect entry — remove | umrs-core provides i18n::tr() but does not translate strings; no domain needed |
| `umrs-platform` | library | `umrs-platform` | `resources/i18n/umrs-platform/` | Active | Kernel posture display.rs strings; fr_CA compiled |
| `umrs-uname` | binary | `umrs-uname` | `resources/i18n/umrs-uname/` | Active | OS detection TUI; fr_CA compiled; help text overlays not yet translated |
| `umrs-state` | binary | `umrs-state` | `resources/i18n/umrs-state/` | Active | State management tool; fr_CA compiled |
| `umrs-logspace` | binary | `umrs-logspace` | `resources/i18n/umrs-logspace/` | Active | Log storage tool; fr_CA compiled |
| `umrs-stat` | binary | `umrs-stat` | *(missing)* | Blocking gap | Calls i18n::init("umrs-stat") but no domain directory, .pot, or fr_CA.po exist; ~20 strings also unwrapped |
| `umrs-ui` | library | *(no domain — Option A)* | *(none)* | Decision pending | Strings in data_panel.rs and header.rs resolved in calling binary's domain; pending Jamie decision on Option A vs Option B |
| `umrs-df` | binary | `umrs-df` | `resources/i18n/umrs-df/` | Reserved | Placeholder; crate not yet in workspace; .pot and all .po files are 0 bytes |
| `umrs-ps` | binary | `umrs-ps` | `resources/i18n/umrs-ps/` | Reserved | Placeholder; crate not yet in workspace; .pot and all .po files are 0 bytes |
| `umrs-tester` | binary? | `umrs-tester` | `resources/i18n/umrs-tester/` | Reserved | Placeholder; crate not yet in workspace; .pot and all .po files are 0 bytes |
| `umrs-labels` | library | *(not assessed)* | *(none)* | Not onboarded | Pending Jamie direction on scope |
| `umrs-selinux` | library | *(not assessed)* | *(none)* | Not onboarded | Pending Jamie direction on scope |
| `umrs-hw` | library? | *(not assessed)* | *(none)* | Not onboarded | Pending Jamie direction on scope |
| `umrs-cui` | binary? | *(not assessed)* | *(none)* | Not onboarded | Pending Jamie direction on scope |

---

## Supported Locales

| Locale | Description | Vocabulary Reference |
|--------|-------------|----------------------|
| `en_US` | American English (base/fallback) | — (msgid is en_US) |
| `fr_CA` | Canadian French | `vocabulary-fr_CA.md` |
| `en_GB` | British English | `vocabulary-en_GB.md` |
| `en_AU` | Australian English | `vocabulary-en_AU.md` |
| `en_NZ` | New Zealand English | `vocabulary-en_NZ.md` |

All Five Eyes nations are represented. `en_US` is the msgid baseline; the three
Commonwealth English variants (en_GB, en_AU, en_NZ) differ primarily in spelling
conventions per their respective government style guides.

---

## Domain Strategy

**Library crates** (`umrs-selinux`, `umrs-core`, etc.) — each gets its own text domain named
after the crate. Library strings are extracted separately so they remain reusable across
multiple binaries.

**Binary crates** (CLI tools) — each binary gets its own text domain. Binaries bind their
own domain plus the domains of the libraries they consume.

---

## Toolchain

- Extraction: `xtr` (Rust-aware; install with `cargo install xtr`)
- Init: `msginit`
- Merge: `msgmerge`
- Validate: `msgfmt --check`
- Compile: `msgfmt`
- Target locales: `fr_CA` (Canadian French), `en_GB`, `en_AU`, `en_NZ` (Commonwealth English variants)
