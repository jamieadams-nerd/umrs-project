SHELL := /bin/sh

ANTORA_DIR := build-tools/antora
PLAYBOOK   := antora-playbook.yml
OUT_DIR    := build/site

.PHONY: help tools docs docs-clean docs-serve clean

help:
	@printf "%s\n" \
	  "Targets:" \
	  "  make tools       Install Antora tooling (npm ci)" \
	  "  make docs        Build the Antora site into $(OUT_DIR)" \
	  "  make docs-serve  Serve $(OUT_DIR) locally on http://127.0.0.1:8000" \
	  "  make docs-clean  Remove generated site output" \
	  "  make clean       docs-clean + remove node_modules" \
	  "  make i18n-setup  Setup i18n directory structure" \
	  "  make i18n-build  Build i18n message objects "

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
I18N_TEXT_DOMAINS  := umrs-logspace umrs-ps umrs-df
I18N_LOCALES       := en_US fr_FR en_GB en_AU en_NZ

I18N_POT_EXT := .pot
I18N_PO_EXT  := .po
I18N_MO_EXT  := .mo

# ------------------------------------------------------------
# i18n setup (idempotent)
# ------------------------------------------------------------

.PHONY: i18n-setup
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

.PHONY: i18n-build
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

