# Makefile — components/rusty-gadgets/
#
# Convenience targets for building, staging, and cleaning the workspace.
# Callable from the top-level project Makefile.
#
# NIST SP 800-53 CM-2: Baseline Configuration — build and stage targets
# produce a reproducible, auditable artifact set for pre-installation review.
# NIST SP 800-53 SA-12: Supply Chain Protection — the release target drives
# the staging checkpoint that precedes IMA signing.
# NIST SP 800-53 SA-22: Unsupported System Components — the man-pages target
# produces the operator-facing documentation artifact for each installed binary.

.PHONY: build release clean-staging man-pages install install-dry-run install-verify

# man-pages — compile all AsciiDoc man page sources to groff/troff format.
#
# Source discovery:
#   English:         <crate>/man/*.1.adoc
#   Canadian French: <crate>/man/fr_CA/*.1.adoc
#
# Output layout:
#   staging/share/man/man1/          ← English .1 pages
#   staging/share/man/fr_CA/man1/    ← Canadian French .1 pages
#
# Requirements: asciidoctor must be on PATH.
#   RHEL 10:  sudo dnf install rubygem-asciidoctor
#   Ubuntu:   sudo apt install asciidoctor
#
# Verification after build:
#   man -l staging/share/man/man1/umrs-ls.1
#   man -l staging/share/man/fr_CA/man1/umrs-ls.1
#
# See docs/man-page-conventions.md for authoring conventions and section layout.
man-pages:
	@command -v asciidoctor >/dev/null 2>&1 || \
		{ echo "[man] ERROR: asciidoctor not found — install rubygem-asciidoctor (RHEL) or asciidoctor (Ubuntu)" >&2; exit 3; }
	@mkdir -p staging/share/man/man1
	@mkdir -p staging/share/man/fr_CA/man1
	@found=0; \
	for src in */man/*.1.adoc; do \
		[ -f "$$src" ] || continue; \
		tool=$$(basename "$$src" .adoc); \
		asciidoctor -b manpage "$$src" -o "staging/share/man/man1/$$tool" 2>&1 \
			&& echo "[man] compiled: staging/share/man/man1/$$tool" \
			|| { echo "[man] ERROR: failed to compile $$src" >&2; exit 1; }; \
		found=$$((found + 1)); \
	done; \
	for src in */man/fr_CA/*.1.adoc; do \
		[ -f "$$src" ] || continue; \
		tool=$$(basename "$$src" .adoc); \
		asciidoctor -b manpage "$$src" -o "staging/share/man/fr_CA/man1/$$tool" 2>&1 \
			&& echo "[man] compiled: staging/share/man/fr_CA/man1/$$tool" \
			|| { echo "[man] ERROR: failed to compile $$src" >&2; exit 1; }; \
		found=$$((found + 1)); \
	done; \
	if [ "$$found" -eq 0 ]; then \
		echo "[man] no *.1.adoc sources found — nothing to compile"; \
	else \
		echo "[man] man page compilation complete ($$found page(s))"; \
	fi

build:
	cargo build --workspace
	cargo xtask stage

release:
	cargo build --release --workspace
	cargo xtask stage --release

clean-staging:
	cargo xtask clean

# install — Phase A demonstration installer for /opt/umrs/.
#
# The shell installer at scripts/umrs-install.sh is the single source of
# truth for the /opt/umrs/ layout (per the 2026-04-13 deployment proposal,
# §9). These targets are thin wrappers; no install logic is duplicated in
# Make or xtask yet.
#
#   make install          run the installer (requires sudo grants)
#   make install-dry-run  print every action without executing
#   make install-verify   re-run only the post-install verification
#
# NIST SP 800-53 CM-3 (Configuration Change Control), SA-10.
install:
	@scripts/umrs-install.sh

install-dry-run:
	@scripts/umrs-install.sh --dry-run

install-verify:
	@scripts/umrs-install.sh --verify
