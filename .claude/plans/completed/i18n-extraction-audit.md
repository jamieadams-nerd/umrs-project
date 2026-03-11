# Plan: i18n Extraction Audit & Fix

**Date**: 2026-03-10
**Status**: READY FOR EXECUTION (step-by-step with approval gates)

---

## Root Cause

`xtr` only extracts strings from function calls whose last path segment matches a built-in
keyword list (`tr`, `gettext`, `ngettext`, `dgettext`, `dcngettext`).

The `tr_core()` function in `umrs-core/src/i18n.rs` is **not** in xtr's default keyword list.
The fix is to pass `--keywords tr_core` to every `xtr` invocation that scans umrs-core source.

Affected strings (all in umrs-core — **not** extracted into the pot):

| File | String |
|---|---|
| `umrs-core/src/validate.rs:106` | `"Invalid validation regex"` |
| `umrs-core/src/console/macros.rs:100` | `"[INFO]"` |
| `umrs-core/src/console/macros.rs:124` | `"[WARN]"` |
| `umrs-core/src/console/macros.rs:149` | `"[ERROR]"` |

---

## Crate-by-Crate Steps

### Step 1 — umrs-core (BROKEN: strings wrapped but not extracted)

**Finding**: 4 strings wrapped with `tr_core()`. The `.pot` file has only a header — all 4 are
missing. Extraction runs but produces no msgid entries.

**Fix (Makefile change)**:
In `I18N_DOMAIN_RULES`, the `i18n-extract-$(1)` recipe passes no `--keywords` flag.
The recipe needs a per-domain keywords variable, populated for umrs-core.

Option A — Global `--keywords` on all domains (simpler, safe):
```makefile
I18N_KEYWORDS_umrs_core := --keywords tr_core
```
Then in the extract recipe:
```makefile
xtr --package-name $(1) \
    --output $(I18N_BASE_DIR)/$(1)/$(1)$(I18N_POT_EXT) \
    $(I18N_KEYWORDS_$(call i18n_var,$(1))) \
    `find $(I18N_SRC_DIR_$(call i18n_var,$(1))) -name '*.rs' | sort`
```
Domains with no `I18N_KEYWORDS_*` variable expand to empty string (no-op).

**Verification after fix**:
```bash
make i18n-extract-umrs-core
grep -c ^msgid resources/i18n/umrs-core/umrs-core.pot
# Expected: 4 (plus the empty header msgid)
```

Confirm these 4 msgids appear in the pot:
- `"Invalid validation regex"`
- `"[INFO]"`
- `"[WARN]"`
- `"[ERROR]"`

Then merge and compile:
```bash
make i18n-merge-umrs-core
make i18n-compile-umrs-core
```

**→ STOP. Report result and ask Jamie to approve Step 2.**

---

### Step 2 — umrs-selinux (NOT IN PIPELINE: no strings wrapped)

**Finding**: No `tr_core()`, `tr!()`, or gettext calls anywhere in `umrs-selinux/src/`.
The crate is not in `I18N_ACTIVE_DOMAINS` and has no `.pot` file.

**Action**: Verify intentional. Two outcomes:
- **Confirmed out-of-scope**: No action. Document in `resources/i18n/domains.md`.
- **Needs i18n**: Identify which user-facing strings should be wrapped, wrap them,
  add to `I18N_ACTIVE_DOMAINS` with `I18N_SRC_DIR_umrs_selinux`.

No Makefile changes needed unless strings are identified.

**→ STOP. Report finding and ask Jamie to approve Step 3.**

---

### Step 3 — umrs-state (NOT IN PIPELINE: no strings wrapped)

**Finding**: No i18n calls in `umrs-state/src/main.rs`. Not in `I18N_ACTIVE_DOMAINS`.

**Action**: Same decision gate as umrs-selinux — confirm intentional or identify strings
to wrap.

**→ STOP. Report finding and ask Jamie to approve Step 4.**

---

### Step 4 — umrs-platform (IN PIPELINE: extraction runs, pot is empty — expected)

**Finding**: `umrs-platform` is in `I18N_ACTIVE_DOMAINS` with `I18N_SRC_DIR_umrs_platform`
pointing to `components/rusty-gadgets/umrs-platform/src`. The `.pot` file exists but contains
only a header — no msgid entries.

**Grep confirms**: No `tr_core()`, `tr!()`, or gettext calls in umrs-platform source.

**Action**: Verify that `make i18n-extract-umrs-platform` runs cleanly (no error).
Empty pot is correct — the crate has no wrapped strings yet.

If umrs-platform has user-facing strings that should be wrapped (e.g., OS detection messages),
those would need to be identified and wrapped in a separate task.

**→ STOP. Report finding and ask Jamie to approve Step 5.**

---

### Step 5 — umrs-logspace (NOT IN ACTIVE DOMAINS: no strings wrapped)

**Finding**: `umrs-logspace` is in `I18N_TEXT_DOMAINS` (scaffolded) but NOT in
`I18N_ACTIVE_DOMAINS`. No `I18N_SRC_DIR_umrs_logspace` is defined. The `.pot` file exists
but is empty.

**Grep confirms**: No i18n calls in umrs-logspace source (`config_loader.rs` only has
`toml::from_str` — not an i18n call).

**Action**: Confirm out-of-scope. If umrs-logspace needs i18n, add:
```makefile
I18N_ACTIVE_DOMAINS += umrs-logspace
I18N_SRC_DIR_umrs_logspace := components/rusty-gadgets/umrs-logspace/src
I18N_ACTIVE_LOCALES_umrs_logspace := fr_CA
```

**→ DONE. Report final status.**

---

## Summary Table

| Crate | Status | Issue | Fix |
|---|---|---|---|
| umrs-core | BROKEN | `tr_core()` not recognized by xtr | Add `--keywords tr_core` to Makefile |
| umrs-selinux | OUT OF SCOPE | No strings wrapped, not in pipeline | Confirm with Jamie |
| umrs-state | OUT OF SCOPE | No strings wrapped, not in pipeline | Confirm with Jamie |
| umrs-platform | OK (empty) | In pipeline, extraction runs, no strings | Verify clean run |
| umrs-logspace | OUT OF SCOPE | Scaffolded but no strings, not in pipeline | Confirm with Jamie |

---

## Files to Modify

| File | Change |
|---|---|
| `Makefile` | Add `I18N_KEYWORDS_umrs_core := --keywords tr_core` config var; thread it into extract recipe |
| `resources/i18n/domains.md` | Update status row for umrs-core once fixed |
