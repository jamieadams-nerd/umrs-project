#!/bin/bash
# -------------------------------------------------------------------------
# build-setrans-targeted.sh
#
# Generates setrans.conf (TARGETED policy) from taxonomy JSON.
#
# Constraints:
#   • s0 only (no MLS levels)
#   • Top-level CUI groups → base categories
#   • Subgroups → paired category markings
#   • Inline comments include marking "name" field
#
# Requires:
#   /usr/bin/jq
# -------------------------------------------------------------------------

set -euo pipefail
IFS=$'\n\t'

JQ_BIN="/usr/bin/jq"

INPUT_JSON="${1:-taxonomy.json}"
OUTPUT_FILE="${2:-setrans.conf}"

TMP_FILE="$(mktemp)"

cleanup() {
    rm -f "$TMP_FILE"
}
trap cleanup EXIT

# -------------------------------------------------------------------------
# Validation
# -------------------------------------------------------------------------

if [[ ! -f "$INPUT_JSON" ]]; then
    echo "[FAIL] JSON not found: $INPUT_JSON" >&2
    exit 1
fi

if ! "$JQ_BIN" empty "$INPUT_JSON" >/dev/null 2>&1; then
    echo "[FAIL] Invalid JSON." >&2
    exit 1
fi

# -------------------------------------------------------------------------
# Header
# -------------------------------------------------------------------------

cat > "$TMP_FILE" <<'EOF'
#-------------------------------------------------------------------------
# AUTO-GENERATED SETRANS.CONF (TARGETED)
#
# Generated from taxonomy JSON.
# Targeted policy → s0 only (informational translations)
#-------------------------------------------------------------------------

s0=SystemLow

#-------------------------------------------------------------------------
# BASE CUI UMBRELLA
s0:c0=CUI

EOF

# -------------------------------------------------------------------------
# Extract top-level CUI groups
# -------------------------------------------------------------------------

mapfile -t TOP_GROUPS < <(
"$JQ_BIN" -r '
.markings
| to_entries[]
| select(.value.parent_group == "CUI")
| .key
' "$INPUT_JSON" | sort
)

# Category allocator
cat_index=10
spacing=10

declare -A CAT_MAP

# -------------------------------------------------------------------------
# Process groups
# -------------------------------------------------------------------------

for group in "${TOP_GROUPS[@]}"; do

    abbr=$("$JQ_BIN" -r ".markings[\"$group\"].abbrv_name" "$INPUT_JSON")
    name=$("$JQ_BIN" -r ".markings[\"$group\"].name" "$INPUT_JSON")

    base_cat=$cat_index
    CAT_MAP["$group"]=$base_cat

    {
        echo "#-------------------------------------------------------------------------"
        echo "# $abbr"
        printf "c%-3s=%-10s # %s\n" "$base_cat" "$abbr" "$name"
        echo
        echo "# Subcategories"
    } >> "$TMP_FILE"

    # -----------------------------------------------------------------
    # Subcategories
    # -----------------------------------------------------------------

    sub_index=$((base_cat + 1))

    mapfile -t SUBGROUPS < <(
    "$JQ_BIN" -r --arg parent "$abbr" '
    .markings
    | to_entries[]
    | select(.value.parent_group == $parent)
    | .key
    ' "$INPUT_JSON" | sort
    )

    for sub in "${SUBGROUPS[@]}"; do

        sub_abbr=$("$JQ_BIN" -r ".markings[\"$sub\"].abbrv_name" "$INPUT_JSON")
        sub_name=$("$JQ_BIN" -r ".markings[\"$sub\"].name" "$INPUT_JSON")

        printf "c%-3s=%-10s # %s\n" \
            "$sub_index" \
            "$sub_abbr" \
            "$sub_name" >> "$TMP_FILE"

        CAT_MAP["$sub"]=$sub_index
        ((sub_index++))
    done

    echo >> "$TMP_FILE"

    # -----------------------------------------------------------------
    # Markings
    # -----------------------------------------------------------------

    echo "# Markings" >> "$TMP_FILE"

    printf "s0:c%-3s = %s\n" \
        "$base_cat" \
        "$group" >> "$TMP_FILE"

    for sub in "${SUBGROUPS[@]}"; do

        sub_name=$("$JQ_BIN" -r ".markings[\"$sub\"].name" "$INPUT_JSON")

        printf "s0:c%-3s,c%-3s = %-25s # %s\n" \
            "$base_cat" \
            "${CAT_MAP[$sub]}" \
            "$sub" \
            "$sub_name" >> "$TMP_FILE"
    done

    echo >> "$TMP_FILE"

    # Advance allocator block
    cat_index=$((cat_index + spacing))

done

# -------------------------------------------------------------------------
# Finalize
# -------------------------------------------------------------------------

mv "$TMP_FILE" "$OUTPUT_FILE"

echo "[OK] Generated $OUTPUT_FILE"

