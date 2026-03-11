Mallard help kit

§Yes. Here is a complete, minimal Mallard/Yelp “starter kit” you can drop into your UMRS repo, check in, build locally, and then extend.
§It includes:
§- 4 concrete Mallard task titles/pages (UMRS-specific)
§- a guide (index) page that links them properly
§- build tooling to validate and generate static HTML (for web hosting)
§- clear install-path guidance for desktop integration (Yelp local help)
§
§Key UMRS Mallard task titles included here:
§1) Check a file’s sensitivity and categories
§2) Change a file’s sensitivity and categories
§3) Handle a document safely after relabeling
§4) Troubleshoot “permission denied” or “operation not permitted” when labeling
§
§Directory tree additions (create these files exactly):
§./tools/build-help.sh
§./help/umrs/C/index.page
§./help/umrs/C/check-sensitivity.page
§./help/umrs/C/change-sensitivity.page
§./help/umrs/C/handle-document.page
§./help/umrs/C/troubleshoot-labeling.page
§
§Optional (recommended) update: replace your tools/build-docs.sh with the updated one below so it builds help too.
§
§––––––––––––––––––––––––––––––––––
§FILE: tools/build-help.sh
§––––––––––––––––––––––––––––––––––
§#!/usr/bin/env bash
§
§set -euo pipefail
§IFS=$’\n\t’
§umask 077
§
§SCRIPT_NAME=”$(basename “$0”)”
§REPO_ROOT=””
§
§die() {
§  printf ‘%s: ERROR: %s\n’ “$SCRIPT_NAME” “$1” >&2
§  exit 1
§}
§
§info() {
§  printf ‘%s: %s\n’ “$SCRIPT_NAME” “$1” >&2
§}
§
§need_cmd() {
§  command -v “$1” >/dev/null 2>&1 || die “Missing required command: $1”
§}
§
§# Resolve repo root safely.
§if command -v git >/dev/null 2>&1; then
§  REPO_ROOT=”$(git rev-parse –show-toplevel 2>/dev/null || true)”
§fi
§if [[ -z “${REPO_ROOT}” ]]; then
§  REPO_ROOT=”$(cd “$(dirname “$0”)/..” && pwd -P)”
§fi
§
§HELP_SRC_DIR=”${REPO_ROOT}/help/umrs/C”
§OUT_DIST_DIR=”${REPO_ROOT}/dist”
§OUT_HELP_HTML_DIR=”${OUT_DIST_DIR}/help-html/umrs”
§OUT_HELP_EPUB_DIR=”${OUT_DIST_DIR}/help-epub”
§
§[[ -d “$HELP_SRC_DIR” ]] || die “Mallard help source directory not found: $HELP_SRC_DIR”
§[[ -f “${HELP_SRC_DIR}/index.page” ]] || die “Missing Mallard index page: ${HELP_SRC_DIR}/index.page”
§
§need_cmd yelp-build
§
§# Optional but useful validation; not fatal if absent.
§HAS_YELP_CHECK=0
§if command -v yelp-check >/dev/null 2>&1; then
§  HAS_YELP_CHECK=1
§fi
§
§rm -rf – “$OUT_HELP_HTML_DIR” “$OUT_HELP_EPUB_DIR”
§mkdir -p – “$OUT_HELP_HTML_DIR” “$OUT_HELP_EPUB_DIR”
§
§if [[ “$HAS_YELP_CHECK” -eq 1 ]]; then
§  info “Running yelp-check (Mallard validation)…”
§  # Validate all pages in the directory.
§  yelp-check validate “$HELP_SRC_DIR” || die “yelp-check validation failed”
§else
§  info “yelp-check not found; skipping validation”
§fi
§
§info “Generating static HTML from Mallard into: $OUT_HELP_HTML_DIR”
§# For Mallard, yelp-build can take a directory and generate HTML for all .page files.
§yelp-build html -o “$OUT_HELP_HTML_DIR” “$HELP_SRC_DIR”
§
§info “Generating EPUB from Mallard into: $OUT_HELP_EPUB_DIR”
§# EPUB is optional but handy for offline reading on systems without Yelp.
§yelp-build epub -o “$OUT_HELP_EPUB_DIR” “$HELP_SRC_DIR”
§
§info “Done.”
§info “Help HTML:  $OUT_HELP_HTML_DIR/index.html (name is derived from page id; see directory listing)”
§info “Help EPUB:  $OUT_HELP_EPUB_DIR (umrs.epub or similar output; see directory listing)”
§info “Desktop integration install path (RPM): /usr/share/help/C/umrs/”
§
§––––––––––––––––––––––––––––––––––
§FILE: help/umrs/C/index.page
§––––––––––––––––––––––––––––––––––
§
§<page xmlns=“http://projectmallard.org/1.0/”
§      type=“guide”
§      id=“index”>
§  
§    UMRS Help
§  
§
§  UMRS Help
§
§  Use this help when you need to check a file’s sensitivity and categories, change them, or understand safe handling expectations in UMRS.
§
§  
§    UMRS is an unclassified reference system. It demonstrates enforcement patterns and operational discipline for sensitive handling workflows.
§  
§
§  
§    Common tasks
§    
§      
§        Check a file’s sensitivity and categories
§      
§      
§        Change a file’s sensitivity and categories
§      
§      
§        Handle a document safely after relabeling
§      
§      
§        Troubleshoot labeling failures
§      
§    
§  
§
§
§––––––––––––––––––––––––––––––––––
§FILE: help/umrs/C/check-sensitivity.page
§––––––––––––––––––––––––––––––––––
§
§<page xmlns=“http://projectmallard.org/1.0/”
§      type=“task”
§      id=“check-sensitivity”>
§  
§    
§  
§
§  Check a file’s sensitivity and categories
§
§  Use this task to confirm a file’s current sensitivity and categories before you open it, move it, or share it.
§
§  
§    
§      In Files, locate the document.
§    
§    
§      Open the file properties (for example, right-click the file and select Properties).
§    
§    
§      Open the UMRS security or labeling section (the exact name depends on the UMRS UI you installed).
§    
§    
§      Record the sensitivity label and all categories shown. If UMRS shows a handling summary, read it completely.
§    
§  
§
§  
§    If the UI does not show label information, use the UMRS CLI tools or administrator instructions to display the file label.
§  
§
§  Related tasks: Change a file’s sensitivity and categories.
§
§
§––––––––––––––––––––––––––––––––––
§FILE: help/umrs/C/change-sensitivity.page
§––––––––––––––––––––––––––––––––––
§
§<page xmlns=“http://projectmallard.org/1.0/”
§      type=“task”
§      id=“change-sensitivity”>
§  
§    
§  
§
§  Change a file’s sensitivity and categories
§
§  Use this task to change a file label when UMRS policy allows it and you have appropriate authorization.
§
§  
§    
§      Confirm the file’s current label using Check a file’s sensitivity and categories.
§    
§    
§      Open the UMRS labeling control for the file (for example, from Properties or a UMRS context action).
§    
§    
§      Select the new sensitivity label.
§    
§    
§      Select the required categories. Do not remove categories unless UMRS policy and your authorization allow it.
§    
§    
§      Apply the change. If UMRS prompts for justification, enter a clear reason.
§    
§    
§      Re-check the file label to confirm the final sensitivity and categories.
§    
§  
§
§  
§    If the operation fails, see Troubleshoot labeling failures.
§  
§
§  Related tasks: Handle a document safely after relabeling.
§
§
§––––––––––––––––––––––––––––––––––
§FILE: help/umrs/C/handle-document.page
§––––––––––––––––––––––––––––––––––
§
§<page xmlns=“http://projectmallard.org/1.0/”
§      type=“task”
§      id=“handle-document”>
§  
§    
§  
§
§  Handle a document safely after relabeling
§
§  Use this task after you change a label. Your goal is to prevent accidental disclosure and keep the document in the correct labeled location.
§
§  
§    
§      Confirm the new label using Check a file’s sensitivity and categories.
§    
§    
§      Store the file only in locations approved for that sensitivity and categories.
§    
§    
§      Do not copy the file to lower-sensitivity locations. Do not attach the file to messages unless policy explicitly allows it.
§    
§    
§      If you must transfer the file, use approved UMRS transfer mechanisms and follow documented handling rules.
§    
§  
§
§  
§    UMRS enforcement may block actions that violate policy. Treat blocks as expected behavior, not a malfunction.
§  
§
§
§––––––––––––––––––––––––––––––––––
§FILE: help/umrs/C/troubleshoot-labeling.page
§––––––––––––––––––––––––––––––––––
§
§<page xmlns=“http://projectmallard.org/1.0/”
§      type=“troubleshooting”
§      id=“troubleshoot-labeling”>
§  
§    
§  
§
§  Troubleshoot labeling failures
§
§  Use this page when UMRS will not apply a label change, or when you see “permission denied” or “operation not permitted”.
§
§  
§    
§      Confirm you selected an allowed sensitivity and categories combination. Some combinations are not valid.
§    
§    
§      Confirm you have the required privilege. Some label operations require administrator approval or a privileged helper.
§    
§    
§      Confirm the target directory is approved for the new label. UMRS may require you to move the file to a labeled location first.
§    
§    
§      If UMRS provides an event ID or failure reason, record it and provide it to the administrator.
§    
§    
§      If the failure repeats, consult the UMRS Administrator Guide and relevant logs for the recorded event.
§    
§  
§
§  
§    Do not attempt to bypass labeling enforcement. If you believe a policy is incorrect, request a documented policy change.
§  
§
§
§––––––––––––––––––––––––––––––––––
§OPTIONAL UPDATE FILE: tools/build-docs.sh (updated to build help too)
§If you want a single command that builds HTML site + PDFs + man pages + Mallard help.
§Replace your existing tools/build-docs.sh with the following full contents.
§––––––––––––––––––––––––––––––––––
§FILE: tools/build-docs.sh
§––––––––––––––––––––––––––––––––––
§#!/usr/bin/env bash
§
§set -euo pipefail
§IFS=$’\n\t’
§umask 077
§
§SCRIPT_NAME=”$(basename “$0”)”
§REPO_ROOT=””
§
§die() {
§  printf ‘%s: ERROR: %s\n’ “$SCRIPT_NAME” “$1” >&2
§  exit 1
§}
§
§info() {
§  printf ‘%s: %s\n’ “$SCRIPT_NAME” “$1” >&2
§}
§
§need_cmd() {
§  command -v “$1” >/dev/null 2>&1 || die “Missing required command: $1”
§}
§
§# Resolve repo root safely.
§if command -v git >/dev/null 2>&1; then
§  REPO_ROOT=”$(git rev-parse –show-toplevel 2>/dev/null || true)”
§fi
§if [[ -z “${REPO_ROOT}” ]]; then
§  # Fallback: assume script is in ./tools under repo root
§  REPO_ROOT=”$(cd “$(dirname “$0”)/..” && pwd -P)”
§fi
§
§DOCS_DIR=”${REPO_ROOT}/docs”
§MAN_SRC_DIR=”${REPO_ROOT}/man”
§OUT_SITE_DIR=”${REPO_ROOT}/site”
§OUT_DIST_DIR=”${REPO_ROOT}/dist”
§OUT_PDF_DIR=”${OUT_DIST_DIR}/pdf”
§OUT_MAN_DIR=”${OUT_DIST_DIR}/man”
§PLAYBOOK=”${REPO_ROOT}/antora-playbook.yml”
§HELP_BUILDER=”${REPO_ROOT}/tools/build-help.sh”
§
§[[ -d “$DOCS_DIR” ]] || die “docs directory not found: $DOCS_DIR”
§[[ -f “$PLAYBOOK” ]] || die “Antora playbook not found: $PLAYBOOK”
§[[ -d “$MAN_SRC_DIR” ]] || die “man source directory not found: $MAN_SRC_DIR”
§[[ -f “$HELP_BUILDER” ]] || die “help builder script not found: $HELP_BUILDER”
§
§# Tooling checks.
§need_cmd asciidoctor
§need_cmd asciidoctor-pdf
§
§HAS_ANTORA=0
§if command -v antora >/dev/null 2>&1; then
§  HAS_ANTORA=1
§elif command -v npx >/dev/null 2>&1; then
§  HAS_ANTORA=2
§else
§  HAS_ANTORA=0
§fi
§
§if [[ “$HAS_ANTORA” -eq 0 ]]; then
§  die “Antora not found. Install ‘antora’ (preferred) or provide ‘npx’ with antora available.”
§fi
§
§# Clean outputs (safe delete only if they are within repo root).
§rm -rf – “$OUT_SITE_DIR” “$OUT_DIST_DIR”
§mkdir -p – “$OUT_SITE_DIR” “$OUT_PDF_DIR” “$OUT_MAN_DIR”
§
§info “Repo root: $REPO_ROOT”
§info “Building HTML site into: $OUT_SITE_DIR”
§
§if [[ “$HAS_ANTORA” -eq 1 ]]; then
§  antora “$PLAYBOOK”
§else
§  npx antora “$PLAYBOOK”
§fi
§
§info “Building PDFs into: $OUT_PDF_DIR”
§PDF_INPUTS=(
§  “${DOCS_DIR}/pdf-admin-guide.adoc”
§  “${DOCS_DIR}/pdf-security-model.adoc”
§)
§for f in “${PDF_INPUTS[@]}”; do
§  [[ -f “$f” ]] || die “Missing PDF entrypoint: $f”
§done
§asciidoctor-pdf -q -D “$OUT_PDF_DIR” “${PDF_INPUTS[@]}”
§
§info “Building man pages into: $OUT_MAN_DIR”
§MAN_INPUTS=(
§  “${MAN_SRC_DIR}/umrs-state.1.adoc”
§  “${MAN_SRC_DIR}/umrs-verify.1.adoc”
§)
§for f in “${MAN_INPUTS[@]}”; do
§  [[ -f “$f” ]] || die “Missing manpage source: $f”
§done
§asciidoctor -q -b manpage -D “$OUT_MAN_DIR” “${MAN_INPUTS[@]}”
§
§info “Building Mallard/Yelp help (static HTML + EPUB) into dist/”
§bash “$HELP_BUILDER”
§
§info “Done.”
§info “HTML site:  $OUT_SITE_DIR”
§info “PDFs:       $OUT_PDF_DIR”
§info “Man pages:  $OUT_MAN_DIR”
§info “Help HTML:  ${OUT_DIST_DIR}/help-html/umrs”
§info “Help EPUB:  ${OUT_DIST_DIR}/help-epub”
§
§––––––––––––––––––––––––––––––––––
§How to store and ship this for desktop integration (RPM guidance)
§––––––––––––––––––––––––––––––––––
§1) Source lives in git under: help/umrs/C/*.page
§2) In your RPM %install, copy pages to:
§   /usr/share/help/C/umrs/
§3) In your GUI app, wire Help to launch:
§   yelp help:umrs
§
§––––––––––––––––––––––––––––––––––
§How to build it locally (fast test loop)
§––––––––––––––––––––––––––––––––––
§1) Install tooling (package names vary by distro; core tools are yelp-build from yelp-tools):
§   - yelp-tools (provides yelp-build; often also yelp-check)
§2) Run:
§   tools/build-help.sh
§3) Inspect output:
§   dist/help-html/umrs/
§4) View locally in a browser:
§   open the generated HTML file whose name matches the page id (index)
§5) View locally in Yelp (after installing to /usr/share/help/C/umrs/):
§   yelp help:umrs
§
§––––––––––––––––––––––––––––––––––
§Source notes (why these commands work)
§––––––––––––––––––––––––––––––––––
§- yelp-build can generate HTML from a directory of Mallard .page files.  
§- yelp-tools provides utilities for building and publishing Yelp/Mallard documentation.  
§- Yelp is the GNOME help viewer for Mallard (and can also display other formats).  
