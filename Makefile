SHELL := /bin/sh

ANTORA_DIR := build-tools/antora
PLAYBOOK   := antora-playbook.yml
OUT_DIR    := build/site

.PHONY: help tools docs docs-clean docs-serve clean \
        i18n i18n-check i18n-setup i18n-build \
        i18n-extract-umrs-ls i18n-merge-umrs-ls i18n-compile-umrs-ls

help:
	@printf "%s\n" \
	  "Targets:" \
	  "  make tools        Install Antora tooling (npm ci)" \
	  "  make docs         Build the Antora site into $(OUT_DIR)" \
	  "  make docs-serve   Serve $(OUT_DIR) locally on http://127.0.0.1:8000" \
	  "  make docs-clean   Remove generated site output" \
	  "  make clean        docs-clean + remove node_modules" \
	  "  make i18n-setup   Setup i18n directory structure" \
	  "  make i18n-build   Build i18n message objects (.po -> .mo)" \
	  "  make i18n         Extract strings, merge .po files, compile .mo files" \
	  "  make i18n-check   Validate all .po files with msgfmt --check"

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
	@rm -rf "$(OUT_DIR)"

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

I18N_BASE_DIR      := resources/i18n
I18N_TEXT_DOMAINS  := umrs-logspace umrs-ps umrs-df umrs-ls
I18N_LOCALES       := en_US fr_FR en_GB en_AU en_NZ

# Per-domain source file lists for xtr extraction.
# Add new entries here when a new crate is onboarded to the i18n pipeline.
# Format: I18N_SOURCES_<domain with hyphens replaced by underscores> := <source files...>
I18N_SOURCES_umrs_ls := \
	components/rusty-gadgets/umrs-ls/src/main.rs

# Active locales per domain (locales with committed .po files ready for compilation).
# The generic I18N_LOCALES list above is for i18n-setup scaffolding only.
# Add a locale here once its .po file has been initialized and translated.
I18N_ACTIVE_LOCALES_umrs_ls := fr_CA

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

i18n-build:
	@echo "==> Building i18n .mo files"
	@set -e; \
	for domain in $(I18N_TEXT_DOMAINS); do \
		domain_dir="$(I18N_BASE_DIR)/$$domain"; \
		for locale in $(I18N_LOCALES); do \
			po_file="$$domain_dir/$$locale$(I18N_PO_EXT)"; \
			mo_file="$$domain_dir/$$locale$(I18N_MO_EXT)"; \
			if [ -f "$$po_file" ]; then \
				echo "  - Compiling $$po_file -> $$mo_file"; \
				msgfmt -o "$$mo_file" "$$po_file"; \
			else \
				echo "  ! Missing $$po_file (skipping)"; \
			fi; \
		done; \
	done
	@echo "==> i18n build complete"

# ------------------------------------------------------------
# i18n extract, merge, and compile (full pipeline)
# Requires: xtr (cargo install xtr), msgmerge, msgfmt
# ------------------------------------------------------------

.PHONY: i18n
i18n: i18n-extract-umrs-ls i18n-merge-umrs-ls i18n-compile-umrs-ls
	@echo "==> i18n pipeline complete"

.PHONY: i18n-extract-umrs-ls
i18n-extract-umrs-ls:
	@echo "==> Extracting strings: umrs-ls"
	@command -v xtr >/dev/null 2>&1 || \
		(echo "  ! xtr not found — install with: cargo install xtr" >&2; exit 1)
	xtr --package-name umrs-ls \
		--output $(I18N_BASE_DIR)/umrs-ls/umrs-ls$(I18N_POT_EXT) \
		$(I18N_SOURCES_umrs_ls)

.PHONY: i18n-merge-umrs-ls
i18n-merge-umrs-ls:
	@echo "==> Merging .po files: umrs-ls"
	@set -e; \
	pot_file="$(I18N_BASE_DIR)/umrs-ls/umrs-ls$(I18N_POT_EXT)"; \
	for locale in $(I18N_ACTIVE_LOCALES_umrs_ls); do \
		po_file="$(I18N_BASE_DIR)/umrs-ls/$$locale$(I18N_PO_EXT)"; \
		if [ -f "$$po_file" ]; then \
			echo "  - Merging $$pot_file into $$po_file"; \
			msgmerge --update --backup=none "$$po_file" "$$pot_file"; \
		else \
			echo "  ! $$po_file not found (run msginit to initialize)"; \
		fi; \
	done

.PHONY: i18n-compile-umrs-ls
i18n-compile-umrs-ls:
	@echo "==> Compiling .mo files: umrs-ls"
	@set -e; \
	for locale in $(I18N_ACTIVE_LOCALES_umrs_ls); do \
		po_file="$(I18N_BASE_DIR)/umrs-ls/$$locale$(I18N_PO_EXT)"; \
		mo_file="$(I18N_BASE_DIR)/umrs-ls/$$locale$(I18N_MO_EXT)"; \
		if [ -f "$$po_file" ]; then \
			echo "  - Compiling $$po_file -> $$mo_file"; \
			msgfmt -o "$$mo_file" "$$po_file"; \
		else \
			echo "  ! Missing $$po_file (skipping)"; \
		fi; \
	done

# ------------------------------------------------------------
# i18n validation (msgfmt --check on all active .po files)
# ------------------------------------------------------------

.PHONY: i18n-check
i18n-check:
	@echo "==> Validating .po files"
	@set -e; \
	failed=0; \
	po_file="$(I18N_BASE_DIR)/umrs-ls/fr_CA$(I18N_PO_EXT)"; \
	if [ -f "$$po_file" ]; then \
		echo "  - Checking $$po_file"; \
		msgfmt --check "$$po_file" || failed=1; \
	else \
		echo "  ! Missing $$po_file (skipping)"; \
	fi; \
	if [ $$failed -eq 1 ]; then \
		echo "==> i18n-check FAILED"; \
		exit 1; \
	fi
	@echo "==> i18n-check passed"

