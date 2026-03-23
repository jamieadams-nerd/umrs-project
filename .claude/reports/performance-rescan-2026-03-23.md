# Performance Rescan — 2026-03-23

**Author:** Rusty (rust-developer agent)
**Scope:** All production crates under `components/rusty-gadgets/`
**Related plan:** `.claude/plans/performance-baseline.md`
**Status:** Analysis only — no code changes. Awaiting Jamie's prioritization.

## Methodology

Full source scan using `rg` across all `.rs` files in the workspace (excluding `tests/`, `examples/`,
`benches/`). Targeted allocation patterns, I/O patterns, clone/copy patterns, and render-path
inefficiencies. Every finding includes a security impact statement.

---

## Summary Table

| ID | Crate | Location | Category | Impact | Severity |
|----|-------|----------|----------|--------|----------|
| P-01 | umrs-ui | `data_panel.rs:329–332` | Hot path allocation | High | High |
| P-02 | umrs-ui | `data_panel.rs:252, 309` | Duplicate scan | Medium | Medium |
| P-03 | umrs-ui | `data_panel.rs:1039–1080` | String allocation per render | High | Medium |
| P-04 | umrs-ui | `data_panel.rs:721–722` | O(n) char counting in word_wrap | High | Medium |
| P-05 | umrs-uname | `main.rs:341–358` | Full row clone every frame | High | High |
| P-06 | umrs-ui | `app.rs:922` | Trait contract pushes allocation | High | Medium |
| P-07 | umrs-platform | `posture/display.rs:56–57` | `to_owned()` on literal constants | Cold | Low |
| P-08 | umrs-selinux | `observations.rs:219–249` | `to_owned()` on literal constants | Cold | Low |
| P-09 | umrs-core | `validate.rs:95–107` | Regex clone on cache hit | Cold | Low |
| P-10 | umrs-platform | `confidence.rs:191` | Clone-before-move | Cold | Low |
| P-11 | umrs-platform | `detect/kernel_anchor.rs:136,243,315` | PathBuf clone for ProcfsText | Cold | Low |
| P-12 | umrs-ls | `main.rs:144–157` | String allocation for arg matching | Cold | Low |
| P-13 | umrs-selinux | `mcs/translator.rs:174, 419` | Clone in add_rule / from_level | Cold | Low |
| P-14 | umrs-ui | `data_panel.rs:617` | `value.to_owned()` in indicator render | High | Medium |
| P-15 | umrs-platform | `posture/fips_cross.rs:217` | `Vec<String>` for small join | Cold | Low |
| P-16 | umrs-stat | `main.rs:652–653` | `Box::leak` per binary invocation | One-time | Info |

---

## Findings

---

### P-01 — Full expand-then-flatten on every render frame
**File:** `umrs-ui/src/data_panel.rs:329–346`
**Category:** Hot path allocation
**Impact:** HIGH — executes on every `terminal.draw()` call (~4 Hz poll)

**Current code:**
```rust
let expanded: Vec<Vec<Line<'_>>> = scrollable_rows
    .iter()
    .map(|r| expand_row_lines(r, inner_width_base, theme, widths))
    .collect();

let total_lines: usize = expanded.iter().map(Vec::len).sum();
// ...
let scrollable_visible: Vec<Line<'_>> = expanded
    .into_iter()
    .flatten()
    .skip(offset)
    .take(inner_height)
    .collect();
```

The code expands ALL rows into styled `Line` objects, sums them to get `total_lines`, then skips
past rows the operator cannot see. With a 37-indicator kernel security tab (each IndicatorRow
expands to 4–6 lines), a full expand runs ~200 line constructions, each allocating a `Vec<Span>`,
per frame — even when the operator is looking at a different tab or has not scrolled.

**Proposed improvement (optional — needs approval):**
Two-pass approach: first compute `total_lines` using the cheap `expanded_row_line_count` counting
function (already exists), then expand only the visible window. The counting pass is branchless
for most row types and does not allocate. `expanded_row_line_count` already mirrors `expand_row_lines`
logic exactly (per the existing comment at line 477).

```rust
// Pass 1: count without allocating (existing function already does this)
let total_lines: usize = scrollable_rows
    .iter()
    .map(|r| expanded_row_line_count(r, inner_width_base))
    .sum();
let max_offset = total_lines.saturating_sub(inner_height);
let offset = scroll_offset.min(max_offset);

// Pass 2: expand only the visible window
let mut visible_lines: Vec<Line<'_>> = Vec::with_capacity(inner_height);
let mut skipped = 0usize;
for row in scrollable_rows {
    let row_lines = expand_row_lines(row, inner_width_base, theme, widths);
    let row_count = row_lines.len();
    if skipped + row_count <= offset {
        skipped += row_count;
        continue; // entirely before the window — skip without collecting
    }
    for line in row_lines {
        if skipped < offset {
            skipped += 1;
            continue;
        }
        if visible_lines.len() >= inner_height {
            break;
        }
        visible_lines.push(line);
    }
    if visible_lines.len() >= inner_height {
        break;
    }
}
```

Estimated improvement: reduces allocations from O(all_rows) to O(visible_rows) per frame.
With 37 indicators at 4 lines each = ~148 `Vec<Span>` freed per frame becomes ~20 (the visible
window). At 60-column terminals with short indicator lists the difference is small; at full-screen
terminals with many indicators it is substantial.

**Security impact statement:** Zero. This is pure rendering logic — it does not touch I/O,
security label data, or kernel attribute reads. The `expand_row_lines` and `expanded_row_line_count`
functions are display-layer only. The existing `expanded_row_line_count` function is already
in production and is correct by construction. No security invariants are affected.

---

### P-02 — `TableWidths::from_rows` called twice when pinned pane is present
**File:** `umrs-ui/src/data_panel.rs:252, 309`
**Category:** Duplicate scan
**Impact:** MEDIUM — `from_rows` iterates the full row slice with `chars().count()` on every field

**Current code:**
```rust
// render_pinned_pane:
let widths = TableWidths::from_rows(pinned);   // line 252

// render_scrollable_pane:
let widths = TableWidths::from_rows(rows);     // line 309
```

When the Kernel Security and Trust/Evidence tabs render (both have pinned rows), `from_rows` is
called twice per frame — once for the pinned pane and once for the scrollable pane. Each call
iterates the relevant row slice and calls `chars().count()` on string fields.

**Proposed improvement (optional — needs approval):**
Compute `TableWidths` once in `render_data_panel` before dispatching to the two sub-renderers,
then pass the pre-computed widths to both. For tabs without pinned content, nothing changes.

**Security impact statement:** Zero. `TableWidths` is a pure display computation — column widths
for terminal alignment. It does not read from kernel, procfs, or security attributes. No security
invariants are affected.

---

### P-03 — String allocation per render in `pad_key`, `pad_value`, `clip_pad`
**File:** `umrs-ui/src/data_panel.rs:1039–1080`
**Category:** String allocation per render
**Impact:** HIGH — called for every visible row on every frame

**Current code:**
```rust
fn pad_key(key: &str, width: usize) -> String {
    // ...
    format!("{key}: {}", " ".repeat(pad))  // allocates per call
}
fn pad_value(value: &str, width: usize) -> String {
    format!("{value}{}", " ".repeat(pad))  // allocates per call
}
fn clip_pad(value: &str, width: usize) -> String {
    format!("{value}{}", " ".repeat(pad))  // allocates per call
}
```

Every call to `pad_key`, `pad_value`, and `clip_pad` allocates a new `String`. These are called
for every visible row on every draw frame. The `" ".repeat(pad)` itself also allocates a temporary
`String` that is immediately moved into the `format!` output.

**Proposed improvement (optional — needs approval):**
Use `write!` into a pre-allocated `String::with_capacity`, avoiding the intermediate `repeat`
allocation:

```rust
fn pad_key(key: &str, width: usize) -> String {
    let char_count = key.chars().count();
    if char_count >= width {
        let mut s = String::with_capacity(width.saturating_add(2));
        s.extend(key.chars().take(width.saturating_sub(2)));
        s.push_str(": ");
        s
    } else {
        let pad = width.saturating_sub(char_count).saturating_sub(2);
        let mut s = String::with_capacity(width);
        s.push_str(key);
        s.push_str(": ");
        for _ in 0..pad {
            s.push(' ');
        }
        s
    }
}
```

This reduces 2 allocations to 1 per call (no intermediate `repeat` string). On a 40-row tab with
3–4 padded fields per row, this saves ~120 allocations per frame.

**Security impact statement:** Zero. These are terminal display padding functions. They operate
only on key and value display strings — not on security attribute data, kernel reads, or
classification labels. The output is visually equivalent; only the allocation strategy changes.

---

### P-04 — `chars().count()` (O(n) for UTF-8) on every word in `word_wrap` hot path
**File:** `umrs-ui/src/data_panel.rs:721–722`
**Category:** O(n) char counting
**Impact:** HIGH — `word_wrap` is called for every IndicatorRow description on every frame

**Current code:**
```rust
for word in text.split_whitespace() {
    let word_len = word.chars().count();       // O(n) per word
    let current_len = current.chars().count(); // O(n) per word iteration
    // ...
}
```

`chars().count()` on a `&str` is O(n) because UTF-8 requires scanning each byte to count
codepoints. For indicator descriptions (20–60 chars of plain ASCII), this is called twice per
word — once for `word_len` and once for `current_len`. Since all content is plain ASCII,
`str::len()` (O(1)) would give the same result. The same pattern appears in `boxmsg.rs:31,51`,
`textwrap.rs:22`, and `robots/data.rs:77,80`.

**Proposed improvement (optional — needs approval):**
All strings in this codebase are plain ASCII (no multibyte characters in indicator keys,
descriptions, or security labels). Replace `chars().count()` with `.len()` in word_wrap
and the parallel counting path. If future Unicode support is needed, this can be reverted.

Additionally, `current_len` is recomputed inside the loop. It can be tracked as a running
counter instead:

```rust
let mut current_len = 0usize;
for word in text.split_whitespace() {
    let word_len = word.len(); // O(1) for ASCII
    if current_len > 0 {
        if current_len.saturating_add(1).saturating_add(word_len) <= width {
            current.push(' ');
            current_len += 1;
        } else {
            lines.push(current.clone());
            current.clear();
            current_len = 0;
        }
    }
    current.push_str(word);
    current_len += word_len;
}
```

**Security impact statement:** Zero. `word_wrap` is a terminal display function used exclusively
for formatting indicator descriptions and key-value row text. It does not process security labels,
kernel attribute values, or classification data. The change from `chars().count()` to `.len()` is
safe for ASCII-only content; if a non-ASCII character somehow appeared, the worst outcome is
visual misalignment, not a security failure. No classification labels or security decisions pass
through this path.

---

### P-05 — Full `Vec<DataRow>` clone on every frame in `umrs-uname` and `umrs-stat`
**File:** `umrs-uname/src/main.rs:341–358`, `umrs-stat/src/main.rs:690–697`
**Category:** Hot path allocation
**Impact:** HIGH — data_rows() / pinned_rows() are called by the render loop on every frame

**Current code (umrs-uname):**
```rust
fn data_rows(&self, tab_index: usize) -> Vec<DataRow> {
    match tab_index {
        0 => self.os_info_rows.clone(),        // full clone
        1 => self.kernel_security_rows.clone(), // full clone
        2 => self.trust_evidence_rows.clone(),  // full clone
        _ => vec![DataRow::normal("(no data)", "(invalid tab index)")],
    }
}
fn pinned_rows(&self, tab_index: usize) -> Vec<DataRow> {
    match tab_index {
        1 => self.kernel_security_summary_rows.clone(), // full clone
        2 => self.trust_summary_rows.clone(),           // full clone
        _ => Vec::new(),
    }
}
```

The `data_rows()` and `pinned_rows()` trait methods return `Vec<DataRow>`, so every call (per
frame) clones the entire row vector. For the Kernel Security tab (~80 rows), this is ~80 DataRow
heap allocations per frame plus all the String fields inside each DataRow. The rows themselves
never change after initialization.

**Proposed improvement (optional — needs approval):**
The `AuditCardApp` trait defines `data_rows(&self) -> Vec<DataRow>` (line 922 in `app.rs`).
The natural fix is to change the trait signature to `fn data_rows(&self) -> &[DataRow]`.
This is a breaking API change to the trait and all implementors (umrs-uname, umrs-stat,
show_logo example, all test mocks), so it requires careful coordination.

A non-breaking alternative: store rows in `Arc<Vec<DataRow>>` and clone the `Arc` instead of
the full vector (cheap reference count increment). This is zero-copy for the row data itself.

The render path in `data_panel.rs` receives the rows as `&[DataRow]` already — it is the
trait method that forces the clone. The trait API is the bottleneck.

**Security impact statement:** Zero. Row data is display-layer content: key-value strings,
style hints, and group titles. None of it is security-critical state. The rows are built once
from `DetectionResult` after the security evaluation pipeline completes, and they are
read-only after that point. Changing the clone strategy does not affect the security pipeline,
kernel attribute reads, or trust level computation.

---

### P-06 — `AuditCardApp::data_rows` trait contract forces allocation
**File:** `umrs-ui/src/app.rs:922`
**Category:** Trait contract pushes allocation to all implementors
**Impact:** HIGH — root cause of P-05; fixing here fixes all implementations simultaneously

**Current trait method:**
```rust
fn data_rows(&self, tab_index: usize) -> Vec<DataRow>;
```

The `Vec<DataRow>` return forces callers to either (a) clone a stored Vec, or (b) construct
one fresh each call. All current implementations do (a). The render loop in `data_panel.rs`
immediately borrows the result as `&[DataRow]`, so the `Vec` exists only to be immediately
borrowed and then dropped.

**Proposed improvement (optional — awaits approval):**
Change the trait to return `Cow<'_, [DataRow]>` or — preferably — a separate
`fn data_rows_ref` with `&[DataRow]` lifetime tied to `&self`. The simpler approach
is `Cow<'_, [DataRow]>` which allows implementors that construct rows dynamically to
return `Cow::Owned` without breaking implementors that store rows and can return `Cow::Borrowed`.

This would make P-05 trivial: all store-backed implementors return `Cow::Borrowed(&self.rows)`.

**Security impact statement:** Zero. This is a Rust API ergonomics change to the display layer.
No security data, kernel reads, or classification logic is involved.

---

### P-07 — `to_owned()` on `&'static str` literals in `posture/display.rs`
**File:** `umrs-platform/src/posture/display.rs:56–57, 64–65`
**Category:** Unnecessary allocation
**Impact:** LOW — called per posture indicator annotation, not on the render hot path

**Current code:**
```rust
LiveValue::Bool(true) => "enabled".to_owned(),
LiveValue::Bool(false) => "disabled".to_owned(),
// ...
"Not Present".to_owned()
```

These are literal constants converted to heap-allocated `String`. The function returns `String`
because some arms (`LiveValue::Text(s)` where `s.clone()`) do need ownership. For the literal
arms, there is no semantic reason to allocate.

**Proposed improvement (optional):**
If `annotate_live_value` were changed to return `Cow<'static, str>` (or even `&'static str`
where possible), the constant arms could return the literal directly. However, the function is
not in the render hot path — it is called once per indicator during snapshot collection, not
per frame. The improvement is real but the impact is low.

**Security impact statement:** Zero. `annotate_live_value` produces display strings derived
from boolean posture indicator values. It does not affect security decisions or audit state.

---

### P-08 — `to_owned()` on `&'static str` literals in `SecurityObservation::Display`
**File:** `umrs-selinux/src/observations.rs:219–249`
**Category:** Unnecessary allocation
**Impact:** LOW — called when formatting observations for display, not in a tight loop

**Current code:**
```rust
Self::WorldWritable => "world-writable".to_owned(),
Self::NoSelinuxContext => "no SELinux context (unlabeled)".to_owned(),
// ... (all literal arms)
```

Every fixed-message arm allocates a `String` from a `&'static str`. These could all be
`&'static str` returns if the `Display` impl used `write!(f, "world-writable")` directly
rather than constructing intermediate `String` values.

**Proposed improvement (optional):**
Refactor the match to write directly into the formatter without the intermediate `String`:
```rust
let msg: &str = match self {
    Self::WorldWritable => "world-writable",
    // ...
    Self::HardLinked { nlink } => {
        // Only this arm and similar ones need format!
        return write!(f, "{symbol} hard-linked (nlink={nlink})");
    }
    // ...
};
write!(f, "{symbol} {msg}")
```

**Security impact statement:** Zero. `SecurityObservation::Display` produces human-readable
strings for terminal output. These strings are observation labels, not security decisions.
Changing allocation strategy in `Display` cannot affect SELinux access decisions or audit
state.

---

### P-09 — Regex clone on cache hit in `validate.rs`
**File:** `umrs-core/src/validate.rs:99–100`
**Category:** Unnecessary clone
**Impact:** LOW — `validate` is called on user input, not on a tight loop

**Current code:**
```rust
fn get_regex(kind: UmrsPattern) -> Regex {
    let cache = REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();  // write lock acquired even for reads

    if let Some(re) = map.get(&kind) {
        return re.clone();  // clone Regex (deep clone of compiled DFA)
    }
    // ...
    compiled
}
```

Two issues:
1. A `Mutex<HashMap>` is acquired with `lock()` (exclusive) for every lookup, even on cache hits.
   `RwLock<HashMap>` would allow concurrent reads.
2. `Regex::clone()` is a deep clone of the compiled automaton state. It is not cheap.

A better approach: use `OnceLock<Regex>` per pattern (since `UmrsPattern` is a small fixed
enum), eliminating both the HashMap and the lock entirely. Each variant gets its own static.

**Security impact statement:** Zero. The regex cache is used for input validation (email,
hostname, etc.). Changing the locking strategy does not affect what patterns are applied or
what inputs they accept. A read-only `RwLock` gives the same correctness guarantees as a
write-lock with fewer contention delays.

---

### P-10 — Clone before move in `confidence.rs::record_contradiction`
**File:** `umrs-platform/src/confidence.rs:191`
**Category:** Unnecessary clone
**Impact:** LOW — called at most once per contradiction per detection run

**Current code:**
```rust
pub fn record_contradiction(&mut self, contradiction: Contradiction, downgrade_to: TrustLevel) {
    let reason = contradiction.description.clone(); // clone String
    self.contradictions.push(contradiction);         // move into Vec
    self.downgrade(downgrade_to, reason);             // consume clone
}
```

`contradiction.description` is cloned so it can be passed to `downgrade()` after `contradiction`
is moved into `self.contradictions`. If `downgrade()` accepted `&str` instead of `String`, the
clone could be avoided by borrowing before the move.

**Proposed improvement (optional):**
Check `downgrade()`'s signature. If it accepts `impl Into<String>`, consider cloning only a
`&str` reference: borrow description before the move. Alternatively, reorder: push a partial
struct, then call downgrade with a reference into the just-pushed item.

**Security impact statement:** Zero. `Contradiction::description` is a human-readable string
describing a detected disagreement between detection phases. It is not a security label or
classification token — it is audit trail text. The audit trail is not affected by whether the
string is cloned before or after the push.

---

### P-11 — PathBuf cloned before `ProcfsText::new` consumption in `kernel_anchor.rs`
**File:** `umrs-platform/src/detect/kernel_anchor.rs:136, 243, 315`
**Category:** Unnecessary clone
**Impact:** LOW — cold path, called once per detection run

**Current code (representative):**
```rust
let path = PathBuf::from("/proc/sys/kernel/random/boot_id");
let node = ProcfsText::new(path.clone()).ok()?;  // clone before consuming
// path is never used after this
```

`ProcfsText::new` takes ownership of the `PathBuf`. The `clone()` is unnecessary because `path`
is not used again after the `ProcfsText::new` call. The fix is to pass `path` directly.
However, `path` is also used in the error branch for `EvidenceRecord::path_requested` (as
`path.display().to_string()`). In that case the pattern is legitimate — but it can be refactored
to borrow `path` for the display call before moving it.

**Security impact statement:** Zero. `PathBuf` carries a filesystem path string, not security
label data. The clone is a Rust ownership bookkeeping issue. `ProcfsText::new` validates the
path prefix (`/proc/`) in both the original and the clone — the validation result is the same
either way.

---

### P-12 — String allocation for CLI arg matching in `umrs-ls`
**File:** `umrs-ls/src/main.rs:144–157`
**Category:** String allocation for comparison
**Impact:** LOW — cold path, called once at binary startup

**Current code:**
```rust
if args.contains(&"--no-iov".to_owned()) {
if args.contains(&"--no-mtime".to_owned()) {
if args.contains(&"--with-size".to_owned()) {
// ...
```

`Vec<String>::contains` requires the argument to be `&String`, so `to_owned()` is called to
construct a temporary `String` for comparison. The standard fix is to use `args.iter().any(|a| a == "--no-iov")` which compares `&String` with `&str` via `PartialEq`.

**Security impact statement:** Zero. CLI argument parsing is startup-only. The flags control
display columns, not security access decisions. Changing `to_owned()` to a `str` comparison does
not affect what data is read or how it is labeled.

---

### P-13 — `clone()` in `mcs/translator.rs::add_rule` and `from_level`
**File:** `umrs-selinux/src/mcs/translator.rs:174, 415, 419`
**Category:** Unnecessary clone
**Impact:** LOW — called during setrans.conf parse (startup, not hot path)

**Current code:**
```rust
// add_rule:
if !detail.is_empty() {
    self.details.insert(range.clone(), detail);  // clone range for details map
}
self.rules.insert(range, label);  // move range into rules map

// from_level:
let t_level = SecurityLevel { categories: level.categories.clone(), .. };
Self { low: t_level.clone(), high: t_level }  // clone t_level to put in both slots
```

In `add_rule`: `range` is cloned so it can be inserted into both `self.details` and `self.rules`.
This is correct but unavoidable given the current data model. The only alternative is a different
ownership model (e.g., using indexes instead of keys in the details map).

In `from_level`: `t_level` is cloned to fill both `low` and `high`. Since `SecurityRange` needs
two owned `SecurityLevel` values for a degenerate (single-level) range, this clone is structurally
necessary unless `SecurityLevel` is made `Copy`.

**Proposed improvement (optional — low priority):**
For `from_level`: evaluate whether `SecurityLevel` could implement `Copy` (its fields are
`u32` sensitivity and `CategorySet` which is `[u64; 16]` — `Copy` is feasible). Making it
`Copy` would eliminate the clone.

**Security impact statement:** Zero. `SecurityLevel` and `SecurityRange` are mathematical
structures used for MCS dominance checks. Making `SecurityLevel` `Copy` if all fields are `Copy`
is a mechanical Rust change — the values and their semantics are unchanged. No security
invariants in the MCS lattice depend on ownership semantics.

---

### P-14 — `value.to_owned()` in `expand_indicator_row` on every frame
**File:** `umrs-ui/src/data_panel.rs:609`
**Category:** String allocation per render
**Impact:** HIGH — called for every visible IndicatorRow on every frame

**Current code:**
```rust
let kv_line = Line::from(vec![
    Span::raw(" "),
    Span::styled(key_padded, theme.data_key),
    Span::styled(value.to_owned(), theme.data_value.fg(value_color)),
]);
```

`value` is `&str` (a borrow from the row's stored `String`). `Span::styled` accepts `impl Into<Text<'_>>`,
and `String` satisfies that via ownership. `&str` also satisfies `Into<Text>` for some ratatui
versions. The `.to_owned()` converts the borrow to an owned `String` — an allocation per indicator
row per frame.

**Proposed improvement (optional):**
Verify whether the ratatui version in use accepts `Span::styled(value, style)` where `value: &str`
without `.to_owned()`. If `Span::styled` accepts `Cow<'_, str>` or `&str`, pass the borrow
directly. This would eliminate one allocation per visible indicator row per frame.

**Security impact statement:** Zero. `value` is a display string extracted from an `IndicatorValue`
enum (e.g., `"Enforcing"`, `"Enabled"`). It is not a security label being evaluated — it is a
string already decided upon and stored in the row. No security semantics attach to whether this
borrow is held or cloned.

---

### P-15 — `Vec<String>` used for 3-item join in `fips_cross.rs`
**File:** `umrs-platform/src/posture/fips_cross.rs:217`
**Category:** Small-collection inefficiency
**Impact:** LOW — cold path, called once per posture snapshot collection

**Current code:**
```rust
let mut parts: Vec<String> = Vec::new();
if let Some(m) = self.marker_present {
    parts.push(format!("marker={label}"));
}
if let Some(c) = self.cmdline_fips {
    parts.push("cmdline=fips=1".to_owned());
}
if let Some(ref p) = self.crypto_policy {
    parts.push(format!("policy={p}"));
}
let raw = parts.join(" ");
```

At most 3 items are ever pushed. A `Vec<String>` with up to 3 items is heap-allocated for both
the Vec header and each String, then freed immediately after the `.join()`. A stack-allocated
small buffer (or direct String construction with conditional appends) would avoid all heap use.

**Proposed improvement (optional):**
Build `raw` directly as a `String` with conditional appends:
```rust
let mut raw = String::new();
if let Some(m) = self.marker_present {
    let label = if m { "present" } else { "absent" };
    raw.push_str(&format!("marker={label}"));
}
// etc., appending " " separator only when raw is non-empty
```

**Security impact statement:** Zero. `fips_cross.rs` builds a human-readable audit summary
string from boolean FIPS indicator values. The string is stored in `ConfiguredValue::raw` for
display purposes only. It is not parsed back or used in access decisions. The content of `raw`
is identical regardless of whether it is built via Vec join or direct String construction.

---

### P-16 — `Box::leak` per binary invocation for `report_subject`
**File:** `umrs-stat/src/main.rs:631–632, 652–653`
**Category:** Intentional leak — informational note
**Impact:** NEGLIGIBLE — one-time allocation per binary run

**Current code:**
```rust
let subject: &'static str =
    Box::leak(dirent.path.to_string().into_boxed_str());
```

`Box::leak` is used to satisfy the `&'static str` requirement of the `AuditCardApp` trait's
`report_subject()` method. The comment correctly notes this is a one-time allocation.

**Note:** This is not a bug. It is a reasonable workaround for the `&'static str` trait constraint.
If the trait were updated to accept `&str` with a lifetime tied to `&self`, `Box::leak` could be
eliminated. This is linked to P-06 (trait API design). Tracking here for completeness.

**Security impact statement:** Zero. The leaked string is the command-line file path argument.
It does not contain classification labels or key material. The leak is bounded (one per invocation)
and is a known, intentional trade-off.

---

## Priority Ranking

Ranked by impact × implementability:

| Rank | Finding | Estimated gain | Complexity |
|------|---------|----------------|------------|
| 1 | P-01 — Full expand then flatten on every frame | High | Medium |
| 2 | P-05/P-06 — Full row clone / trait API | High | High (API break) |
| 3 | P-04 — `chars().count()` in word_wrap | Medium | Low |
| 4 | P-03 — pad_key/clip_pad allocation | Medium | Low |
| 5 | P-02 — Duplicate TableWidths scan | Low-medium | Low |
| 6 | P-14 — `value.to_owned()` per indicator per frame | Medium | Low (verify ratatui API) |
| 7 | P-09 — Regex cache uses write-lock on read | Low | Low |
| 8 | P-12 — CLI arg matching allocs | Negligible | Trivial |
| 9 | P-13 — SecurityLevel Copy | Low | Low |
| 10 | P-07, P-08 — Literal to_owned | Low | Low |
| 11 | P-10, P-11 — Confidence/PathBuf clones | Low | Low |
| 12 | P-15 — fips_cross Vec<String> | Negligible | Low |
| 13 | P-16 — Box::leak (informational) | — | — |

---

## Relationship to Existing Performance Plan

The existing `.claude/plans/performance-baseline.md` defines a 6-step benchmarking process
including criterion harnesses, baseline recording, and per-phase timing. That plan is still
valid and should be executed. The findings in this report are candidates for Step 4 (Optimization
Opportunity Analysis) of that plan.

Specifically:
- P-01 is the single highest-value target and aligns with the plan's goal of reducing
  per-frame allocation in the TUI render path.
- P-05/P-06 require an API design decision before implementation — this should be raised
  with Jamie before touching the trait.
- P-03 and P-04 are low-risk, measurable improvements suitable for the Step 6 sequential
  implementation process.

None of the findings above require new crate dependencies. All improvements are pure Rust
refactors within existing module boundaries.

---

*End of report.*

