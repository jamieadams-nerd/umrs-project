---
name: patterns
description: High-assurance pattern conventions confirmed in this codebase
type: project
---

# High-Assurance Pattern Conventions

## Regex Cache Pattern

All three validate modules (umrs-core, umrs-labels, umrs-selinux) use:

```rust
static REGEX_CACHE: OnceLock<Mutex<HashMap<PatternEnum, Regex>>> = OnceLock::new();

fn get_regex(kind: PatternEnum) -> Regex {
    let cache = REGEX_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // ... cache lookup and insert
}
```

Note: use `std::sync::PoisonError::into_inner` not a closure `|poisoned| poisoned.into_inner()` — clippy pedantic flags the closure form.

## Pattern Enum Methods

Use `pub const fn regex(self) -> &'static str` (not just `fn`) — clippy pedantic requires `const` when the body is eligible.

Use `Self::Variant` (not `TypeName::Variant`) in match arms inside `impl` blocks — clippy `use_self` lint.

## SelinuxContext Regex

The correct pattern for the SELinux 4-field context is:
`^[^:]+:[^:]+:[^:]+:.+$`

The last field (level) can contain colons (e.g., `s0:c0,c5`, `s0-s0:c0.c1023`). Using `[^:]+` for the last field is wrong — it rejects real MLS contexts.

## MlsRange Regex

`^s\d+(:c\d+(,c\d+)*)?(-s\d+(:c\d+(,c\d+)*)?)?$`

Supports: `s0`, `s0:c0`, `s1:c0,c5`, `s0-s3:c0`, `s0:c0-s3:c0,c5`.

## Module Doc Block Required

Every `.rs` file under `src/` MUST have a `//!` block with: purpose, key exported types, `## Compliance` section.

## Clippy Line Count

`clippy::too_many_lines` fires at 100 lines per function. When a function grows over limit, extract logical sub-sections into helper functions. In `umrs-uname/src/main.rs` the pattern is: extract `append_*_rows(rows: &mut Vec<DataRow>, result: &DetectionResult)` helpers.

## must_use Convention

All public `Result`-returning functions AND all validation predicates carry:
`#[must_use = "the validation result must be checked; ignoring it defeats the purpose"]`

## Test Fixture Path

```rust
fn fixture_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("fixture-file.json")
}
```

Tests live exclusively in `tests/` — no `#[cfg(test)]` or `mod tests` in src files.

## TUI Trait Pattern (ViewerApp / ConfigApp)

The object-safe trait + separate state struct pattern:

- Trait (e.g., `ViewerApp`) — provides static data (header, tabs, initial tree). Implemented on the application data struct.
- State struct (e.g., `ViewerState`) — owns mutable UI state (selection, search, scroll). Separate so the event loop holds `&dyn Trait` while mutating state.
- `handle_action(Action) -> EventType` — all key events route through here; returns an enum telling the caller what to do next.
- `ConfigStateEvent` is `#[must_use]` at the type level — prevents the caller from silently dropping Save/Discard events.

## card_title() Lifetime — Avoid &'static str

When implementing `card_title(&self) -> &str` with a static string, clippy fires
`needless_lifetimes_tied_to_arguments` OR triggers a return-type issue with `&'static str`
where the trait requires `&str`. Fix: store the title as a `String` field in the struct and
return `&self.title`. Do not use `-> &'static str` on trait implementations.

## const fn Eligibility

Functions eligible for `const fn` in this codebase:
- Any `fn` with only match arms returning literals or integer ops (no heap allocation)
- `blocks_save()`, `display()` (returning `""` or stored string slice), `is_leaf()`, `display_count()`
- `TreeModel::new()` — `Vec::new()` is const-stable since Rust 1.39
- `move_focus_up()`, `move_focus_down()` — mutate only integer fields via saturating_sub / comparison

## Avoiding Signed Casts for Navigation

Never use `as isize` / `as usize` casts for navigation index arithmetic. Use:
- `saturating_sub(n)` for decrement (PageUp, ScrollUp)
- `saturating_add(n)` for increment, then `.min(max)` to clamp
- This avoids `cast_possible_wrap` and `cast_sign_loss` clippy errors entirely.

## Boxed Validator Closures

Field validators in `FieldDef` use `Box<dyn Fn(&FieldValue) -> ValidationResult + Send>`.
- `Send` is required because `ConfigState` is used across thread boundaries in real event loops.
- Constructors take `impl Fn(&FieldValue) -> ValidationResult + Send + 'static` and box them internally.

## let _ = for #[must_use] Intentional Discards

When an internal helper intentionally discards a `#[must_use]` return value, use:
```rust
let _ = self.validate_buffer(); // result read via self.validation on next line
```
Include an inline comment explaining WHY the value is discarded.
