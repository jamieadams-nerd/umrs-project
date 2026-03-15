#!/usr/bin/env bash
# .claude/hooks/session-cleanup.sh
# Runs at SessionStart to:
#   1. Remove nested .claude directories
#   2. Merge settings.local.json into settings.json
#   3. Report status of upstream issues filed by this project
# See CLAUDE.md: "Settings and Data Location Rule", "Single .claude Directory Rule"

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"
CLAUDE_DIR="$PROJECT_DIR/.claude"
SETTINGS="$CLAUDE_DIR/settings.json"
LOCAL="$CLAUDE_DIR/settings.local.json"

# ── 1. Remove nested .claude directories ──────────────────────────────
if [ -d "$CLAUDE_DIR/.claude" ]; then
    rm -rf "$CLAUDE_DIR/.claude"
    echo "Removed nested .claude/.claude/ directory."
fi

# ── 2. Merge settings.local.json into settings.json, then delete it ──
if [ -f "$LOCAL" ]; then
    if command -v python3 >/dev/null 2>&1; then
        python3 - "$SETTINGS" "$LOCAL" <<'PYEOF'
import json, sys

settings_path, local_path = sys.argv[1], sys.argv[2]

with open(settings_path) as f:
    settings = json.load(f)
with open(local_path) as f:
    local = json.load(f)

merged = 0
# Merge permissions.allow lists (deduplicate)
if "permissions" in local and "allow" in local["permissions"]:
    existing = set(settings.get("permissions", {}).get("allow", []))
    for perm in local["permissions"]["allow"]:
        if perm not in existing:
            settings.setdefault("permissions", {}).setdefault("allow", []).append(perm)
            existing.add(perm)
            merged += 1

# Merge permissions.deny lists (deduplicate)
if "permissions" in local and "deny" in local["permissions"]:
    existing = set(settings.get("permissions", {}).get("deny", []))
    for perm in local["permissions"]["deny"]:
        if perm not in existing:
            settings.setdefault("permissions", {}).setdefault("deny", []).append(perm)
            existing.add(perm)
            merged += 1

with open(settings_path, "w") as f:
    json.dump(settings, f, indent=2)
    f.write("\n")

if merged > 0:
    print(f"Merged {merged} permission(s) from settings.local.json into settings.json.")
else:
    print("settings.local.json had no new permissions to merge.")
PYEOF
        rm "$LOCAL"
        echo "Deleted settings.local.json."
    else
        echo "WARNING: python3 not found — cannot merge settings.local.json. Deleting it."
        rm "$LOCAL"
    fi
fi

# ── 3. Report status of tracked upstream issues ───────────────────────
ISSUES_FILE="$CLAUDE_DIR/tracked-issues.txt"
if [ -f "$ISSUES_FILE" ] && command -v gh >/dev/null 2>&1; then
    echo ""
    echo "── Tracked upstream issues ──"
    while IFS='|' read -r repo number description || [ -n "$repo" ]; do
        # Skip comments and blank lines
        [[ "$repo" =~ ^#.*$ || -z "$repo" ]] && continue
        # Query issue state (open/closed) with timeout
        state=$(gh issue view "$number" --repo "$repo" --json state --jq '.state' 2>/dev/null) || state="unknown"
        echo "  $repo#$number ($state) — $description"
    done < "$ISSUES_FILE"
fi
