#!/usr/bin/bash
# French Canadian terminology lookup — searches authoritative sources in priority order.
#
# Priority: Termium Plus (federal) → OQLF GDT (Quebec) → GNU .po corpus
# TSV format: english_term\tfr_ca_term\tdomain\tsource\tnotes

PATH=/usr/bin:/usr/sbin:/bin
export PATH

TERM="$1"
CORPUS=".claude/corpus/"

if [ -z "$TERM" ]; then
  echo "Usage: french_lookup.sh <english term>" >&2
  exit 2
fi

# --- Tier 1: Canadian terminology databases (.tsv) ---
# Search order: Termium Plus (federal standard) → OQLF GDT (Quebec standard)
TSV_FILES=("termium-plus-fr_CA" "oqlf-gdt-fr_CA" "cccs-cyber-glossary-fr_CA")

for db in "${TSV_FILES[@]}"; do
  tsv_file="${CORPUS}/${db}.tsv"
  if [ -f "$tsv_file" ]; then
    RESULT=$(grep -i "$TERM" "$tsv_file" 2>/dev/null)
    if [ -n "$RESULT" ]; then
      echo "=== ${db} (authoritative fr_CA) ==="
      echo "$RESULT" | while IFS=$'\t' read -r en fr domain source notes; do
        echo "  EN: ${en}"
        echo "  FR: ${fr}"
        echo "  Domain: ${domain}"
        echo "  Source: ${source}"
        [ -n "$notes" ] && echo "  Notes: ${notes}"
        echo ""
      done
      exit 0
    fi
  fi
done

# --- Tier 2: GNU translation corpus (.po files) ---
# Priority order within GNU corpus
PO_FILES=("coreutils" "util-linux" "cryptsetup" "grep" "sed" "tar" "findutils" "bash")

for PKG in "${PO_FILES[@]}"; do
  RESULT=$(grep -h -A 1 -i "msgid \"$TERM\"" "${CORPUS}/${PKG}"* 2>/dev/null)
  if [ -n "$RESULT" ]; then
    echo "--- Found in ${PKG} (GNU corpus, may be fr-FR) ---"
    echo "$RESULT"
    exit 0
  fi
done

# --- Tier 3: Fallback — search all .po and .tsv files ---
echo "--- Broad search (all sources) ---"
grep -h -A 1 -i "msgid \".*${TERM}.*\"" "${CORPUS}"/*.po 2>/dev/null
grep -i "$TERM" "${CORPUS}"/*.tsv 2>/dev/null
