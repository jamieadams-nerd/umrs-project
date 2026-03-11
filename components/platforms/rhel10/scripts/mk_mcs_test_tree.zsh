#!/usr/bin/zsh
set -euo pipefail

# -----------------------------------------------------------------------------
# mk_mcs_test_tree.zsh
#
# Build a deterministic MCS test directory tree from setrans.conf mappings.
#
# - Reads keys of the form: s0:cNNN[,cMMM...] = ...
# - Creates: /MLS_TEST_DIRS/<base_cat>/
# - Labels base directory to s0:<base_cat> (only if that exact key exists)
# - Creates files:
#     base.txt              -> s0:<base_cat>
#     cMMM.txt              -> s0:<base_cat>,cMMM
#     cMMM+cNNN.txt (etc)   -> s0:<base_cat>,cMMM,cNNN  (if ever present)
#
# Security posture:
# - Absolute interpreter path
# - Quotes all expansions
# - No PATH-dependent tool invocation assumptions beyond standard system tools
# -----------------------------------------------------------------------------

TRANSFILE="/etc/selinux/targeted/setrans.conf"
BASE="/MLS_TEST_DIRS"

# ---- Utilities (absolute paths where reasonable) -----------------------------
GREP="/usr/bin/grep"
CUT="/usr/bin/cut"
TR="/usr/bin/tr"
MKDIR="/usr/bin/mkdir"
TOUCH="/usr/bin/touch"
CHCON="/usr/bin/chcon"
LS="/usr/bin/ls"

# ---- Preconditions -----------------------------------------------------------
if [[ ! -r "$TRANSFILE" ]]; then
  print -u2 "ERROR: Can't read $TRANSFILE"
  exit 1
fi

if [[ ! -d "$BASE" ]]; then
  print -u2 "ERROR: Base directory does not exist: $BASE"
  print -u2 "       Create it first (e.g., sudo mkdir -p $BASE) and set perms as desired."
  exit 1
fi

# ---- Build an in-memory list of levels (keys) -------------------------------
typeset -a LEVELS
LEVELS=()

while IFS= read -r lvl; do
    LEVELS+=("$lvl")
done < <(
    /usr/bin/awk '
        # Skip blank lines and full-line comments
        /^[[:space:]]*($|#)/ { next }

        # Match lines whose key starts with s0:c (allow leading whitespace)
        /^[[:space:]]*s0:c/ {
            line = $0

            # Split at first "=" and keep the left side
            split(line, a, "=")
            key = a[1]

            # Trim all whitespace from key (spaces/tabs)
            gsub(/[[:space:]]+/, "", key)

            print key
        }
    ' "$TRANSFILE"
)


# ---- Index which base categories have an explicit base label -----------------
# Example: if "s0:c110" exists in LEVELS, we will label directory and base.txt.
typeset -A HAS_BASE
for lvl in "${LEVELS[@]}"; do
  # lvl is like "s0:c110,c111" or "s0:c110"
  cats="${lvl#s0:}"          # "c110,c111" or "c110"
  base="${cats%%,*}"         # "c110"
  if [[ "$cats" == "$base" ]]; then
    HAS_BASE["$base"]=1
  fi
done

# ---- Main loop ---------------------------------------------------------------
for lvl in "${LEVELS[@]}"; do
  cats="${lvl#s0:}"          # e.g. "c110,c111"
  base="${cats%%,*}"         # e.g. "c110"

  base_dir="${BASE}/${base}"

  # Ensure directory exists (non-root create; labeling requires privileges)
  if [[ ! -d "$base_dir" ]]; then
    $MKDIR -p "$base_dir"
  fi

  # Label the base directory only once, and only if we have an explicit base key.
  # (We avoid labeling dirs to a level that isn't in setrans keys, per your intent.)
  if [[ -n "${HAS_BASE[$base]:-}" ]]; then
    # Only attempt if not already labeled correctly; avoid needless churn.
    # (We intentionally do not parse contexts here; just set idempotently.)
    $CHCON -l "s0:${base}" "$base_dir" 2>/dev/null || true
  fi

  # Decide file naming:
  # - If lvl is exactly base: create base.txt
  # - Else: create a file named by the "extra" categories (excluding base)
  if [[ "$cats" == "$base" ]]; then
    file="${base_dir}/base.txt"
    target_lvl="s0:${base}"
  else
    extras="${cats#${base},}"            # e.g. "c111" or "c111,c112"
    safe_extras="${extras//,/+}"         # e.g. "c111+c112"
    file="${base_dir}/${safe_extras}.txt"
    target_lvl="s0:${cats}"
  fi

  # Create file if missing
  if [[ ! -e "$file" ]]; then
    $TOUCH "$file"
  fi

  # Apply label to file
  $CHCON -l "$target_lvl" "$file"

done

print "[OK] Created and labeled test tree under: $BASE"
print
$LS -lZ "$BASE" 2>/dev/null || true

