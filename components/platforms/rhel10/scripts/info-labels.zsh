#!/usr/bin/zsh
#
# The 'jq' rpm must be installed
#
declare -r JSON="cui-mls-labels.json"
declare -r SETRANS="/etc/selinux/targeted/setrans.conf"



label_validate() {
echo
echo "Validating Labels..."
echo "-----------------------"
jq -r '
  .markings
  | to_entries[]
  | select(.value.parent_group == "CUI")
  | "\(.key) → \(.value.name)"
' $JSON
echo
}

label_lookup() {
    local label="$1"
    echo "Label lookup.... "
    echo "-----------------------"

jq -r --arg k "$label" '
  .markings[$k] |
  { key: $k, name: .name }
' $JSON

}

get_raw_level() {
    local label="$1"

    lvlnum=$(/usr/bin/grep -E "${label}" ${SETRANS} |/usr/bin/cut -f1 -d=)
    echo $lvlnum
}



####################
### Main 
####################

label_validate

label_lookup "CUI//LEI/JUV"
echo -n "Get level and category numbers for CUI//LEI/JUV :: => "
get_raw_level "CUI//LEI/JUV"
echo



