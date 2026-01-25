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
	  "  make docs-serve   Serve $(OUT_DIR) locally on http://127.0.0.1:8000" \
	  "  make docs-clean   Remove generated site output" \
	  "  make clean        docs-clean + remove node_modules"

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

i18n-mo:
	mkdir -p resources/i18n/umrs-tester/locale/en_US/LC_MESSAGES
	mkdir -p resources/i18n/umrs-tester/locale/fr_FR/LC_MESSAGES
	msgfmt -o resources/i18n/umrs-tester/locale/en_US/LC_MESSAGES/umrs-tester.mo resources/i18n/umrs-tester/en_US.po
	msgfmt -o resources/i18n/umrs-tester/locale/fr_FR/LC_MESSAGES/umrs-tester.mo resources/i18n/umrs-tester/fr_FR.po


clean: docs-clean
	@rm -rf "$(ANTORA_DIR)/node_modules"

