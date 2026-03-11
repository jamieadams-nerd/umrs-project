#!/bin/bash
TERM=$1
CORPUS=".claude/corpus/gnu-fr"
# Define priority order
FILES=("coreutils" "util-linux" "grep" "sed" "tar" "findutils" "bash")

for PKG in "${FILES[@]}"; do
  # Search for the msgid and the following line (msgstr)
  RESULT=$(grep -h -A 1 -i "msgid \"$TERM\"" $CORPUS/$PKG* 2>/dev/null)
  if [ ! -z "$RESULT" ]; then
    echo "--- Found in $PKG ---"
    echo "$RESULT"
    exit 0 # Stop at the highest priority match
  fi
done

# Fallback: search everything else
grep -h -A 1 -i "msgid \"$TERM\"" $CORPUS/*.po

