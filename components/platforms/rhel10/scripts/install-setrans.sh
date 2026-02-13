#!/bin/bash
#
# -------------------------------------------------------------------------
# install-setrans.sh
#
# Installs UMRS setrans.conf for SELinux TARGETED policy on RHEL 10.
#
# Operations:
#   • Detect OS + version (must be RHEL 10)
#   • Detect active SELinux policy
#   • Validate mcstrans installation
#   • Backup existing setrans.conf
#   • Install new targeted translation file
#   • Restart mcstrans daemon
#
# Usage:
#   sudo ./install-setrans.sh targeted/setrans.conf--TARGETED
# -------------------------------------------------------------------------

set -euo pipefail
IFS=$'\n\t'

# -------------------------------------------------------------------------
# Constants — absolute paths
# -------------------------------------------------------------------------

CP_BIN="/usr/bin/cp"
RPM_BIN="/usr/bin/rpm"
DATE_BIN="/usr/bin/date"
CAT_BIN="/usr/bin/cat"
GREP_BIN="/usr/bin/grep"
AWK_BIN="/usr/bin/awk"
SESTATUS_BIN="/usr/sbin/sestatus"
SYSTEMCTL_BIN="/usr/bin/systemctl"

INPUT_FILE="${1:-}"

# -------------------------------------------------------------------------
# Validation — input file
# -------------------------------------------------------------------------

if [[ -z "$INPUT_FILE" ]]; then
    echo "[FAIL] No input setrans file specified."
    echo "Usage: sudo $0 <setrans.conf--TARGETED>"
    exit 1
fi

if [[ ! -f "$INPUT_FILE" ]]; then
    echo "[FAIL] Input file not found: $INPUT_FILE"
    exit 1
fi

echo "[INFO] Input file: $INPUT_FILE"

# -------------------------------------------------------------------------
# Detect OS + Version
# -------------------------------------------------------------------------

echo 
echo "[INFO] Detecting operating system…"

if [[ ! -f /etc/redhat-release ]]; then
    echo "[FAIL] Not a Red Hat–based system."
    exit 1
fi

release=$($CAT_BIN /etc/redhat-release)

echo "[INFO] Release string: $release"

if ! $GREP_BIN -q "release 10" /etc/redhat-release; then
    echo "[FAIL] This installer supports RHEL 10 only."
    exit 1
fi

echo "  [OK] RHEL 10 detected."
echo

# -------------------------------------------------------------------------
# Detect SELinux Policy
# -------------------------------------------------------------------------

echo
echo "[INFO] Detecting SELinux policy…"

policy=$($SESTATUS_BIN | \
    $AWK_BIN -F': ' '/Loaded policy name/ {gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')

if [[ -z "$policy" ]]; then
    echo "  [FAIL] Unable to determine SELinux policy."
    exit 1
fi

echo "   [OK] Active policy: $policy"

# -------------------------------------------------------------------------
# Determine setrans path
# -------------------------------------------------------------------------

SETRANS_DIR="/etc/selinux/${policy}"
SETRANS_FILE="${SETRANS_DIR}/setrans.conf"

if [[ ! -d "$SETRANS_DIR" ]]; then
    echo "[FAIL] SELinux directory missing: $SETRANS_DIR"
    exit 1
fi

echo "[INFO] setrans directory: $SETRANS_DIR"

# -------------------------------------------------------------------------
# Validate mcstrans package
# -------------------------------------------------------------------------

echo "[INFO] Checking mcstrans installation…"

if ! $RPM_BIN -q mcstrans >/dev/null 2>&1; then
    echo "[FAIL] mcstrans package not installed."
    echo "Install with: sudo dnf install mcstrans"
    exit 1
fi

echo "[OK] mcstrans package present."

# -------------------------------------------------------------------------
# Backup existing configuration
# -------------------------------------------------------------------------

timestamp=$($DATE_BIN +%Y%m%d-%H%M%S)

BACKUP_FILE="${SETRANS_FILE}.bak-${timestamp}"

if [[ -f "$SETRANS_FILE" ]]; then
    echo "[INFO] Backing up existing setrans.conf…"
    echo "       → $BACKUP_FILE"

    /usr/bin/sudo $CP_BIN -p "$SETRANS_FILE" "$BACKUP_FILE"

    echo "[OK] Backup complete."
else
    echo "[WARN] No existing setrans.conf found — skipping backup."
fi

# -------------------------------------------------------------------------
# Install new configuration
# -------------------------------------------------------------------------

echo "[INFO] Installing new setrans.conf…"

/usr/bin/sudo $CP_BIN -p "$INPUT_FILE" "$SETRANS_FILE"

echo "[OK] Installed → $SETRANS_FILE"

# -------------------------------------------------------------------------
# Restart mcstrans daemon
# -------------------------------------------------------------------------

echo "[INFO] Restarting mcstrans service…"

$SYSTEMCTL_BIN restart mcstrans

echo "[OK] mcstrans restarted."

# -------------------------------------------------------------------------
# Completion
# -------------------------------------------------------------------------

echo
echo "[SUCCESS] setrans installation complete."
echo "          Policy : $policy"
echo "          Backup : $BACKUP_FILE"
echo

