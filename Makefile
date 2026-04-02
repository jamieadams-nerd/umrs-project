SHELL := /bin/sh

ANTORA_DIR := build-tools/antora
PLAYBOOK   := antora-playbook.yml
OUT_DIR    := build/site
DRAFT_PLAYBOOK := antora-playbook-draft.yml
DRAFT_OUT_DIR  := build/site-draft


.PHONY: help tools docs docs-clean docs-serve clean \
        i18n i18n-check i18n-setup i18n-build docs-draft docs-both

help:
	@printf "%s\n" \
	  "Targets:" \
	  "  make tools        Install Antora tooling (npm ci)" \
	  "  make docs         Build the Antora site into $(OUT_DIR)" \
	  "  make docs-serve   Serve $(OUT_DIR) locally on http://127.0.0.1:8000" \
	  "  make docs-clean   Remove generated site output" \
	  "  make clean        docs-clean + remove node_modules" \
	  "  make docs-draft    Build draft site (all modules) into $(DRAFT_OUT_DIR)" \
	  "  make docs-both     Build public site then draft site" \
	  "  make i18n-setup   Setup i18n directory structure" \
	  "  make i18n-build   Build i18n message objects (.po -> .mo)" \
	  "  make i18n         Extract strings, merge .po files, compile .mo files" \
	  "  make i18n-check   Validate all .po files with msgfmt --check" \
	  "  make i18n-extract-<domain>  Extract strings for one domain (e.g. umrs-ls)" \
	  "  make i18n-merge-<domain>    Merge .po files for one domain" \
	  "  make i18n-compile-<domain>  Compile .mo files for one domain" \
	  "  Active domains: $(I18N_ACTIVE_DOMAINS)"

tools:
	@test -f "$(ANTORA_DIR)/package.json" || (echo "Missing $(ANTORA_DIR)/package.json" >&2; exit 1)
	@command -v npm >/dev/null 2>&1 || (echo "npm not found (install Node.js LTS)" >&2; exit 1)
	@cd "$(ANTORA_DIR)" && npm ci

docs: tools
	@test -f "$(PLAYBOOK)" || (echo "Missing $(PLAYBOOK) at repo root" >&2; exit 1)
	@mkdir -p "$(OUT_DIR)"
	@cd "$(ANTORA_DIR)" && npx antora --clean --to-dir "../../$(OUT_DIR)" "../../$(PLAYBOOK)"

docs-serve:
	@test -d "$(OUT_DIR)" || (echo "Missing $(OUT_DIR) (run: make docs)" >&2; exit 1)
	@command -v python3 >/dev/null 2>&1 || (echo "python3 not found" >&2; exit 1)
	@cd "$(OUT_DIR)" && python3 -m http.server 8000 --bind 127.0.0.1

docs-clean:
	@rm -rf "$(OUT_DIR)" "$(DRAFT_OUT_DIR)"

docs-draft: tools
	@test -f "$(DRAFT_PLAYBOOK)" || (echo "Missing $(DRAFT_PLAYBOOK) at repo root" >&2; exit 1)
	@mkdir -p "$(DRAFT_OUT_DIR)"
	@cp docs/antora.yml docs/antora.yml.bak
	@cp docs/antora-draft.yml docs/antora.yml
	@cd "$(ANTORA_DIR)" && npx antora --clean --to-dir "../../$(DRAFT_OUT_DIR)" "../../$(DRAFT_PLAYBOOK)" || \
		(cp docs/antora.yml.bak docs/antora.yml && rm -f docs/antora.yml.bak && exit 1)
	@cp docs/antora.yml.bak docs/antora.yml
	@rm -f docs/antora.yml.bak

docs-both: docs docs-draft


#i18n-mo:
#	mkdir -p resources/i18n/umrs-tester/locale/en_US/LC_MESSAGES
#	mkdir -p resources/i18n/umrs-tester/locale/fr_FR/LC_MESSAGES
#	msgfmt -o resources/i18n/umrs-tester/locale/en_US/LC_MESSAGES/umrs-tester.mo resources/i18n/umrs-tester/en_US.po
#	msgfmt -o resources/i18n/umrs-tester/locale/fr_FR/LC_MESSAGES/umrs-tester.mo resources/i18n/umrs-tester/fr_FR.po


clean: docs-clean
	@rm -rf "$(ANTORA_DIR)/node_modules"


# ------------------------------------------------------------
# i18n configuration
# ------------------------------------------------------------

I18N_BASE_DIR       := resources/i18n
I18N_TEXT_DOMAINS   := umrs-logspace umrs-ps umrs-df umrs-ls umrs-state umrs-uname umrs-platform umrs-c2pa
I18N_ACTIVE_DOMAINS := umrs-ls umrs-state umrs-logspace umrs-uname umrs-platform umrs-c2pa
I18N_LOCALES        := en_US fr_FR en_GB en_AU en_NZ

# Internal helper: hyphen-to-underscore for variable name lookup.
# Usage: $(call i18n_var,umrs-ls) -> umrs_ls
i18n_var = $(subst -,_,$(1))

# Per-domain source directories for xtr extraction.
# Add one entry here when a new crate is onboarded to the i18n pipeline.
# All *.rs files under the directory are discovered automatically at extract time.
# Format: I18N_SRC_DIR_<domain with hyphens replaced by underscores> := <src/ path>
I18N_SRC_DIR_umrs_ls       := components/rusty-gadgets/umrs-ls/src
I18N_SRC_DIR_umrs_state    := components/rusty-gadgets/umrs-state/src
I18N_SRC_DIR_umrs_logspace := components/rusty-gadgets/umrs-logspace/src
I18N_SRC_DIR_umrs_uname    := components/rusty-gadgets/umrs-uname/src
I18N_SRC_DIR_umrs_platform := components/rusty-gadgets/umrs-platform/src
I18N_SRC_DIR_umrs_c2pa     := components/rusty-gadgets/umrs-c2pa/src

# Active locales per domain (locales with committed .po files ready for compilation).
# The generic I18N_LOCALES list above is for i18n-setup scaffolding only.
# Add a locale here once its .po file has been initialized and translated.
I18N_ACTIVE_LOCALES_umrs_ls       := fr_CA fr_FR en_GB en_AU en_NZ
I18N_ACTIVE_LOCALES_umrs_state    := fr_CA
I18N_ACTIVE_LOCALES_umrs_logspace := fr_CA
I18N_ACTIVE_LOCALES_umrs_uname    := fr_CA
I18N_ACTIVE_LOCALES_umrs_platform := fr_CA
I18N_ACTIVE_LOCALES_umrs_c2pa     := fr_CA

I18N_POT_EXT := .pot
I18N_PO_EXT  := .po
I18N_MO_EXT  := .mo

# ------------------------------------------------------------
# i18n setup (idempotent)
# ------------------------------------------------------------

i18n-setup:
	@echo "==> Setting up i18n directory structure"
	@set -e; \
	for domain in $(I18N_TEXT_DOMAINS); do \
		domain_dir="$(I18N_BASE_DIR)/$$domain"; \
		pot_file="$$domain_dir/$$domain$(I18N_POT_EXT)"; \
		mkdir -p "$$domain_dir"; \
		if [ ! -f "$$pot_file" ]; then \
			echo "  - Creating $$pot_file"; \
			touch "$$pot_file"; \
		fi; \
		for locale in $(I18N_LOCALES); do \
			po_file="$$domain_dir/$$locale$(I18N_PO_EXT)"; \
			if [ ! -f "$$po_file" ]; then \
				echo "  - Creating $$po_file"; \
				touch "$$po_file"; \
			fi; \
		done; \
	done
	@echo "==> i18n setup complete"

# ------------------------------------------------------------
# i18n build (.po -> .mo)
# ------------------------------------------------------------

i18n-build: $(foreach _d,$(I18N_ACTIVE_DOMAINS),i18n-compile-$(_d))
	@echo "==> i18n build complete"

# ------------------------------------------------------------
# i18n extract, merge, and compile (full pipeline)
# Requires: xtr (cargo install xtr), msgmerge, msgfmt
#
# Per-domain targets are generated automatically for every entry in
# I18N_ACTIVE_DOMAINS.  To add a new domain:
#   1. Add the domain name to I18N_ACTIVE_DOMAINS (and I18N_TEXT_DOMAINS).
#   2. Add I18N_SRC_DIR_<domain_underscored> := components/rusty-gadgets/<domain>/src.
#   3. Add I18N_ACTIVE_LOCALES_<domain_underscored> := <locales...>.
#   4. Run: make i18n-setup && make i18n-extract-<domain>
# ------------------------------------------------------------

# Dollar-sign escaping inside define/eval/call:
#   $(1) and $(call ...) are expanded by call before eval.
#   Shell variables inside recipe lines need $$$$ (four dollars):
#   call strips one layer ($$$$ -> $$), Make strips another at recipe
#   execution time ($$ -> $).

define I18N_DOMAIN_RULES

.PHONY: i18n-extract-$(1) i18n-merge-$(1) i18n-compile-$(1)

i18n-extract-$(1):
	@echo "==> Extracting strings: $(1)"
	@command -v xtr >/dev/null 2>&1 || \
		(echo "  ! xtr not found — install with: cargo install xtr" >&2; exit 1)
	xtr --package-name $(1) \
		--output $(I18N_BASE_DIR)/$(1)/$(1)$(I18N_POT_EXT) \
		`find $(I18N_SRC_DIR_$(call i18n_var,$(1))) -name '*.rs' | sort`

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
		mo_dir="$(I18N_BASE_DIR)/$(1)/$$$$locale/LC_MESSAGES"; \
		mo_file="$$$$mo_dir/$(1)$(I18N_MO_EXT)"; \
		if [ -f "$$$$po_file" ]; then \
			mkdir -p "$$$$mo_dir"; \
			echo "  - Compiling $$$$po_file -> $$$$mo_file"; \
			msgfmt -o "$$$$mo_file" "$$$$po_file"; \
		else \
			echo "  ! Missing $$$$po_file (skipping)"; \
		fi; \
	done

endef

$(foreach _d,$(I18N_ACTIVE_DOMAINS),$(eval $(call I18N_DOMAIN_RULES,$(_d))))

.PHONY: i18n
i18n: $(foreach _d,$(I18N_ACTIVE_DOMAINS),i18n-extract-$(_d) i18n-merge-$(_d) i18n-compile-$(_d))
	@echo "==> i18n pipeline complete"

# ------------------------------------------------------------
# i18n validation (msgfmt --check on all active .po files)
# ------------------------------------------------------------

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

