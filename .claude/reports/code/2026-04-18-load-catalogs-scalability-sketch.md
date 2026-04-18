---
date: 2026-04-18
author: rust-developer (Rusty)
status: sketch — awaiting Jamie's approval before implementation
---

# `load_catalogs` / `lookup_marking_detail` — Five Eyes Scalability Sketch

## Problem

`load_catalogs()` and `lookup_marking_detail()` in `umrs-ls/src/main.rs` are
copy-paste structures: each Five Eyes nation (US, CA today; UK, AUS, NZ pending) adds
one `Vec<String>` candidate block and one if-branch to both functions.  By the time
all five nations are present, there will be five parallel blocks in each function
and ten total.

## Proposed API

Replace both functions with a registry driven from a const table.

```rust
/// ## Compliance
///
/// - **NIST SP 800-53 AC-16**: Security Attributes — the catalog table is
///   the single authoritative source for all Five Eyes CUI label definitions.
/// - **FHS 3.0 §3.13**: Static data for `/opt/umrs/` packages lives under
///   `/opt/umrs/share/umrs/`.
struct CatalogSource {
    /// ISO nation code, used for logging and flag lookup.
    nation: &'static str,
    /// Filename in the canonical share directory, config override, and dev path.
    filename: &'static str,
}

static CATALOG_SOURCES: &[CatalogSource] = &[
    CatalogSource { nation: "US", filename: "US-CUI-LABELS.json" },
    CatalogSource { nation: "CA", filename: "CANADIAN-PROTECTED.json" },
    // Future additions — one line each:
    // CatalogSource { nation: "UK", filename: "UK-OFFICIAL.json" },
    // CatalogSource { nation: "AU", filename: "AUS-OFFICIAL.json" },
    // CatalogSource { nation: "NZ", filename: "NZ-OFFICIAL.json" },
];

fn load_catalogs() -> Vec<Option<Catalog>> {
    let config_dir = std::env::var("UMRS_CONFIG_DIR").ok();
    CATALOG_SOURCES
        .iter()
        .map(|src| {
            let candidates = build_candidates(src.filename, config_dir.as_deref());
            try_load_catalog(&candidates.iter().map(String::as_str).collect::<Vec<_>>())
        })
        .collect()
}

fn build_candidates(filename: &str, config_dir: Option<&str>) -> Vec<String> {
    let mut v = Vec::with_capacity(4);
    if let Some(dir) = config_dir {
        v.push(format!("{dir}/{filename}"));
    }
    v.push(format!("/opt/umrs/share/umrs/{filename}"));
    v.push(format!("config/{filename}"));
    v.push(format!("../umrs-label/config/{filename}"));
    v
}
```

`lookup_marking_detail` becomes a linear scan over the returned `Vec<Option<Catalog>>`:

```rust
fn lookup_marking_detail(
    marking: &str,
    catalogs: &[Option<Catalog>],
) -> Option<MarkingDetailData> {
    for cat_opt in catalogs {
        let Some(cat) = cat_opt else { continue };
        if let Some(m) = cat.marking(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(marking, m, &flag));
        }
        if let Some((key, m)) = cat.marking_by_mcs_level(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(key, m, &flag));
        }
        if let Some((key, m)) = cat.marking_by_banner(marking) {
            let flag = cat.country_flag().unwrap_or_default();
            return Some(marking_to_detail(key, m, &flag));
        }
    }
    None
}
```

## Migration Path

1. Add `CatalogSource` struct and `CATALOG_SOURCES` const table to `main.rs`.
2. Replace `load_catalogs()` return type from `(Option<Catalog>, Option<Catalog>)` to
   `Vec<Option<Catalog>>`.
3. Update all `load_catalogs()` call sites (currently 1 in `main.rs`).
4. Update `lookup_marking_detail` signature and all call sites (currently 2 in
   `handle_enter` + 1 in CLI path).
5. Thread `&[Option<Catalog>]` through `render_listing` and `build_group_header_item`
   in `tui_render.rs` (currently 2 params `us_catalog`, `ca_catalog`; becomes 1).
6. Update `handle_enter` to take `catalogs: &[Option<Catalog>]` (removes one arg,
   partially offsetting the `level_registry` addition).

## Diff Estimate

- `main.rs`: ~40 lines deleted, ~30 lines added (net -10).
- `tui_render.rs`: ~8 lines changed (signature updates).
- `tests/marking_lookup_tests.rs`: ~15 lines changed (fixture updates).

Adding a new nation after migration: 1 line in `CATALOG_SOURCES`, 0 changes elsewhere.

## Call Sites Affected

| File | Symbol | Action |
|------|--------|--------|
| `main.rs` | `load_catalogs()` | replace return type |
| `main.rs` | `handle_enter()` | update param |
| `main.rs` | `lookup_marking_detail()` | update signature |
| `tui_render.rs` | `render_listing()` | collapse 2 params → 1 |
| `tui_render.rs` | `build_group_header_item()` | collapse 2 params → 1 |
| `tests/marking_lookup_tests.rs` | test fixtures | update catalog arg |

## Status

Sketch only.  Implementation blocked pending Jamie's approval.
Items 1/2/3/5/6 from the punch list are complete without this change.
