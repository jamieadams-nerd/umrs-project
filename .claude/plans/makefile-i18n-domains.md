# Plan: Makefile i18n Multi-Domain Variable Refactoring

**Date**: 2026-03-11
**Agent**: umrs-translator
**Status**: READY FOR EXECUTION

---

## Goal

Make adding a new i18n text domain a single-variable operation:
add the domain name to `I18N_ACTIVE_DOMAINS`, define its source list and
active locales, and all extract/merge/compile targets work automatically.

---

## Problem with Current Makefile

- `I18N_TEXT_DOMAINS` exists but the extract/merge/compile pipeline is
  hardcoded to `umrs-ls` only (three named phony targets).
- Adding a new domain requires copy-pasting three targets and editing the
  `i18n` roll-up target manually.
- `i18n-check` hardcodes `umrs-ls/fr_CA.po` — does not loop over domains.
- `.PHONY` declaration at the top must also be manually updated.

---

## Design Decision: Two Domain Lists

| Variable | Purpose |
|---|---|
| `I18N_TEXT_DOMAINS` | All planned domains — used by `i18n-setup` and `i18n-build` to scaffold dirs and compile any `.po` found |
| `I18N_ACTIVE_DOMAINS` | Domains with committed source files — drives extract/merge/compile pipeline |

`I18N_TEXT_DOMAINS` keeps its current four placeholder entries plus the new ones.
`I18N_ACTIVE_DOMAINS` is a new, smaller list: only domains with real source files.

---

## Changes Required

### 1. Configuration block

```makefile
I18N_BASE_DIR     := resources/i18n
I18N_TEXT_DOMAINS := umrs-logspace umrs-ps umrs-df umrs-ls umrs-core umrs-platform
I18N_ACTIVE_DOMAINS := umrs-ls umrs-core umrs-platform
I18N_LOCALES      := en_US fr_FR en_GB en_AU en_NZ
```

Add source file lists (hyphens → underscores in variable names):

```makefile
I18N_SOURCES_umrs_ls := \
    components/rusty-gadgets/umrs-ls/src/main.rs

I18N_SOURCES_umrs_core := \
    components/rusty-gadgets/umrs-core/src/lib.rs \
    components/rusty-gadgets/umrs-core/src/i18n.rs \
    components/rusty-gadgets/umrs-core/src/validate.rs \
    components/rusty-gadgets/umrs-core/src/timed_result.rs \
    components/rusty-gadgets/umrs-core/src/prelude.rs \
    components/rusty-gadgets/umrs-core/src/audit/mod.rs \
    components/rusty-gadgets/umrs-core/src/audit/events.rs \
    components/rusty-gadgets/umrs-core/src/audit/schema.rs \
    components/rusty-gadgets/umrs-core/src/audit/emit.rs \
    components/rusty-gadgets/umrs-core/src/console/mod.rs \
    components/rusty-gadgets/umrs-core/src/console/ansi.rs \
    components/rusty-gadgets/umrs-core/src/console/boxmsg.rs \
    components/rusty-gadgets/umrs-core/src/console/macros.rs \
    components/rusty-gadgets/umrs-core/src/console/spinner.rs \
    components/rusty-gadgets/umrs-core/src/console/symbols.rs \
    components/rusty-gadgets/umrs-core/src/console/typography.rs \
    components/rusty-gadgets/umrs-core/src/cui/mod.rs \
    components/rusty-gadgets/umrs-core/src/cui/catalog.rs \
    components/rusty-gadgets/umrs-core/src/cui/palette.rs \
    components/rusty-gadgets/umrs-core/src/fs/mod.rs \
    components/rusty-gadgets/umrs-core/src/human/mod.rs \
    components/rusty-gadgets/umrs-core/src/human/metricfmt.rs \
    components/rusty-gadgets/umrs-core/src/human/sizefmt.rs \
    components/rusty-gadgets/umrs-core/src/human/textwrap.rs \
    components/rusty-gadgets/umrs-core/src/robots/mod.rs \
    components/rusty-gadgets/umrs-core/src/robots/builtins.rs \
    components/rusty-gadgets/umrs-core/src/robots/data.rs \
    components/rusty-gadgets/umrs-core/src/selinux/mod.rs \
    components/rusty-gadgets/umrs-core/src/selinux/mcs.rs

I18N_SOURCES_umrs_platform := \
    components/rusty-gadgets/umrs-platform/src/lib.rs \
    components/rusty-gadgets/umrs-platform/src/confidence.rs \
    components/rusty-gadgets/umrs-platform/src/evidence.rs \
    components/rusty-gadgets/umrs-platform/src/os_identity.rs \
    components/rusty-gadgets/umrs-platform/src/os_release.rs \
    components/rusty-gadgets/umrs-platform/src/detect/mod.rs \
    components/rusty-gadgets/umrs-platform/src/detect/file_ownership.rs \
    components/rusty-gadgets/umrs-platform/src/detect/integrity_check.rs \
    components/rusty-gadgets/umrs-platform/src/detect/kernel_anchor.rs \
    components/rusty-gadgets/umrs-platform/src/detect/label_trust.rs \
    components/rusty-gadgets/umrs-platform/src/detect/mount_topology.rs \
    components/rusty-gadgets/umrs-platform/src/detect/pkg_substrate.rs \
    components/rusty-gadgets/umrs-platform/src/detect/release_candidate.rs \
    components/rusty-gadgets/umrs-platform/src/detect/release_parse.rs \
    components/rusty-gadgets/umrs-platform/src/detect/substrate/mod.rs \
    components/rusty-gadgets/umrs-platform/src/detect/substrate/dpkg.rs \
    components/rusty-gadgets/umrs-platform/src/detect/substrate/rpm.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/mod.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/procfs.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/security.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/selinux.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/sysfs.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/tpi.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/traits.rs \
    components/rusty-gadgets/umrs-platform/src/kattrs/types.rs
```

Add active locale lists:

```makefile
I18N_ACTIVE_LOCALES_umrs_ls       := fr_CA
I18N_ACTIVE_LOCALES_umrs_core     := fr_CA
I18N_ACTIVE_LOCALES_umrs_platform := fr_CA
```

---

### 2. Replace hardcoded per-domain targets with a `define`/`eval`/`call` template

Remove the six hardcoded targets:
- `i18n-extract-umrs-ls`, `i18n-merge-umrs-ls`, `i18n-compile-umrs-ls`
- Their explicit `.PHONY` entries

Replace with a template that auto-generates targets for every entry in
`I18N_ACTIVE_DOMAINS`:

```makefile
# Internal helper: hyphen-to-underscore for variable name lookup.
# Usage: $(call i18n_var,umrs-ls) -> umrs_ls
i18n_var = $(subst -,_,$(1))

define I18N_DOMAIN_RULES

.PHONY: i18n-extract-$(1) i18n-merge-$(1) i18n-compile-$(1)

i18n-extract-$(1):
	@echo "==> Extracting strings: $(1)"
	@command -v xtr >/dev/null 2>&1 || \
		(echo "  ! xtr not found — install with: cargo install xtr" >&2; exit 1)
	xtr --package-name $(1) \
		--output $(I18N_BASE_DIR)/$(1)/$(1)$(I18N_POT_EXT) \
		$(I18N_SOURCES_$(call i18n_var,$(1)))

i18n-merge-$(1):
	@echo "==> Merging .po files: $(1)"
	@set -e; \
	pot_file="$(I18N_BASE_DIR)/$(1)/$(1)$(I18N_POT_EXT)"; \
	for locale in $(I18N_ACTIVE_LOCALES_$(call i18n_var,$(1))); do \
		po_file="$(I18N_BASE_DIR)/$(1)/$$$$locale$(I18N_PO_EXT)"; \
		if [ -f "$$$$po_file" ]; then \
			echo "  - Merging $$$$pot_file into $$$$po_file"; \
			msgmerge --update --backup=none "$$$$po_file" "$$$$pot_file"; \
		else \
			echo "  ! $$$$po_file not found (run msginit to initialize)"; \
		fi; \
	done

i18n-compile-$(1):
	@echo "==> Compiling .mo files: $(1)"
	@set -e; \
	for locale in $(I18N_ACTIVE_LOCALES_$(call i18n_var,$(1))); do \
		po_file="$(I18N_BASE_DIR)/$(1)/$$$$locale$(I18N_PO_EXT)"; \
		mo_file="$(I18N_BASE_DIR)/$(1)/$$$$locale$(I18N_MO_EXT)"; \
		if [ -f "$$$$po_file" ]; then \
			echo "  - Compiling $$$$po_file -> $$$$mo_file"; \
			msgfmt -o "$$$$mo_file" "$$$$po_file"; \
		else \
			echo "  ! Missing $$$$po_file (skipping)"; \
		fi; \
	done

endef

$(foreach _d,$(I18N_ACTIVE_DOMAINS),$(eval $(call I18N_DOMAIN_RULES,$(_d))))
```

**Dollar-sign escaping note**: Inside `define`/`eval`/`call`, `$(1)` and
`$(call ...)` are expanded by `call` before `eval`. Shell variables inside
recipe lines need `$$$$` (four dollars) — `call` strips one layer (`$$$$` →
`$$`), then Make strips another at recipe execution time (`$$` → `$`).

---

### 3. Update the `i18n` roll-up target

```makefile
.PHONY: i18n
i18n: $(foreach _d,$(I18N_ACTIVE_DOMAINS),i18n-extract-$(_d) i18n-merge-$(_d) i18n-compile-$(_d))
	@echo "==> i18n pipeline complete"
```

---

### 4. Fix `i18n-check` to loop over all active domains

Replace the hardcoded single-file check with a loop:

```makefile
.PHONY: i18n-check
i18n-check:
	@echo "==> Validating .po files"
	@set -e; \
	failed=0; \
	$(foreach _d,$(I18N_ACTIVE_DOMAINS), \
		for locale in $(I18N_ACTIVE_LOCALES_$(call i18n_var,$(_d))); do \
			po_file="$(I18N_BASE_DIR)/$(_d)/$$locale$(I18N_PO_EXT)"; \
			if [ -f "$$po_file" ]; then \
				echo "  - Checking $$po_file"; \
				msgfmt --check "$$po_file" || failed=1; \
			else \
				echo "  ! Missing $$po_file (skipping)"; \
			fi; \
		done;) \
	if [ $$failed -eq 1 ]; then \
		echo "==> i18n-check FAILED"; \
		exit 1; \
	fi
	@echo "==> i18n-check passed"
```

---

### 5. Update `.PHONY` declaration and `help` target

Remove the hardcoded per-domain phony names from the top-level `.PHONY` line
(they are now declared inside the `define` template). Update `help` text to
reflect `i18n-extract-<domain>`, `i18n-merge-<domain>`, `i18n-compile-<domain>`
as the per-domain invocation pattern.

Updated `.PHONY` (top of file):

```makefile
.PHONY: help tools docs docs-clean docs-serve clean \
        i18n i18n-check i18n-setup i18n-build
```

Updated `help` target — add a line:

```
  "  make i18n-extract-<domain>  Extract strings for one domain (e.g. umrs-ls)"
  "  make i18n-merge-<domain>    Merge .po files for one domain"
  "  make i18n-compile-<domain>  Compile .mo files for one domain"
  "  Active domains: $(I18N_ACTIVE_DOMAINS)"
```

---

### 6. Create `resources/i18n/umrs-core/` directory scaffold

`umrs-platform` already has a `.pot` file at `resources/i18n/umrs-platform/`.
`umrs-core` does not yet have its directory. The `i18n-setup` target will
create it, but the `.pot` file will be empty until `i18n-extract-umrs-core`
runs. No manual action required beyond running `make i18n-setup`.

---

## Adding a New Domain Later (the workflow)

1. Add the domain name to `I18N_ACTIVE_DOMAINS` (and `I18N_TEXT_DOMAINS`).
2. Add `I18N_SOURCES_<domain_with_underscores> := <source files...>`.
3. Add `I18N_ACTIVE_LOCALES_<domain_with_underscores> := <locales...>`.
4. Run `make i18n-setup` to scaffold the directory.
5. Run `make i18n-extract-<domain>` to generate the `.pot`.
6. Run `msginit` per locale to create initial `.po` files.
7. Run `make i18n` to run the full pipeline.
8. Update `resources/i18n/domains.md` with the new row.

---

## Files Modified

| File | Change |
|---|---|
| `Makefile` | All changes above |
| `resources/i18n/domains.md` | No change needed — already lists all three domains |

---

## Verification After Execution

```bash
# Confirm generated targets exist
make -n i18n-extract-umrs-core
make -n i18n-extract-umrs-platform

# Run full pipeline dry-run
make -n i18n

# Validate check target
make -n i18n-check

# Run setup to scaffold umrs-core directory
make i18n-setup

# Full extract (requires xtr installed)
make i18n
```
