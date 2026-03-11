#!/usr/bin/bash

set -euo pipefail
IFS=$’\n\t’
umask 077

SCRIPT_NAME=”$(basename “$0”)”
REPO_ROOT=””

die() {
  printf ‘%s: ERROR: %s\n’ “$SCRIPT_NAME” “$1” >&2
  exit 1
}

info() {
  printf ‘%s: %s\n’ “$SCRIPT_NAME” “$1” >&2
}

need_cmd() {
  command -v “$1” >/dev/null 2>&1 || die “Missing required command: $1”
}

# Resolve repo root safely.
if command -v git >/dev/null 2>&1; then
  REPO_ROOT=”$(git rev-parse –show-toplevel 2>/dev/null || true)”
fi
if [[ -z “${REPO_ROOT}” ]]; then
  REPO_ROOT=”$(cd “$(dirname “$0”)/..” && pwd -P)”
fi

HELP_SRC_DIR=”${REPO_ROOT}/help/umrs/C”
OUT_DIST_DIR=”${REPO_ROOT}/dist”
OUT_HELP_HTML_DIR=”${OUT_DIST_DIR}/help-html/umrs”
OUT_HELP_EPUB_DIR=”${OUT_DIST_DIR}/help-epub”

[[ -d “$HELP_SRC_DIR” ]] || die “Mallard help source directory not found: $HELP_SRC_DIR”
[[ -f “${HELP_SRC_DIR}/index.page” ]] || die “Missing Mallard index page: ${HELP_SRC_DIR}/index.page”

need_cmd yelp-build

# Optional but useful validation; not fatal if absent.
HAS_YELP_CHECK=0
if command -v yelp-check >/dev/null 2>&1; then
  HAS_YELP_CHECK=1
fi

rm -rf – “$OUT_HELP_HTML_DIR” “$OUT_HELP_EPUB_DIR”
mkdir -p – “$OUT_HELP_HTML_DIR” “$OUT_HELP_EPUB_DIR”

if [[ “$HAS_YELP_CHECK” -eq 1 ]]; then
  info “Running yelp-check (Mallard validation)…”
  # Validate all pages in the directory.
  yelp-check validate “$HELP_SRC_DIR” || die “yelp-check validation failed”
else
  info “yelp-check not found; skipping validation”
fi

info “Generating static HTML from Mallard into: $OUT_HELP_HTML_DIR”
# For Mallard, yelp-build can take a directory and generate HTML for all .page files.
yelp-build html -o “$OUT_HELP_HTML_DIR” “$HELP_SRC_DIR”

info “Generating EPUB from Mallard into: $OUT_HELP_EPUB_DIR”
# EPUB is optional but handy for offline reading on systems without Yelp.
yelp-build epub -o “$OUT_HELP_EPUB_DIR” “$HELP_SRC_DIR”

info “Done.”
info “Help HTML:  $OUT_HELP_HTML_DIR/index.html (name is derived from page id; see directory listing)”
info “Help EPUB:  $OUT_HELP_EPUB_DIR (umrs.epub or similar output; see directory listing)”
info “Desktop integration install path (RPM): /usr/share/help/C/umrs/”

