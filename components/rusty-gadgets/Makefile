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

# man-pages — DEPRECATED. Man pages are now authored as pre-built troff under
#   <crate>/docs/*.1       (English)
#   <crate>/docs/fr/*.1    (Canadian French)
# and copied directly into the staging tree by `cargo xtask stage`.
#
# Run `cargo xtask stage` to assemble the staging bundle including man pages
# and compiled gettext catalogs. This target is preserved as a no-op to avoid
# breaking any external caller that still invokes it; it will be removed once
# all callers are updated.
man-pages:
	@echo "[man] make man-pages is DEPRECATED — use 'cargo xtask stage' instead"
	@echo "[man] man page sources are now pre-built troff under <crate>/docs/*.1"

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
