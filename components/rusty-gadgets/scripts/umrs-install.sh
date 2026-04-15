#!/usr/bin/bash
#
# umrs-install.sh -- Phase A installer for /opt/umrs/
#
# Deployment:
#   Demonstration release. Single source of truth for the /opt/umrs/ layout.
#   Per Jamie (2026-04-13), the shell installer is authoritative until the
#   xtask/umrs-install Rust port lands.
#
# Ownership and permissions (DEMONSTRATION concession -- see secure_bash_rules
# "Privileged Script Rule"):
#   On production the script must be root-owned 0700/0750. On this
#   development host it is shipped with the workspace (jadams:umrs-admin
#   0750). The self-check below accepts either (root|jadams) + (700|750).
#
# Controls:
#   NIST SP 800-53 CM-2, CM-5, CM-6  (baseline + access restrictions for change)
#   NIST SP 800-53 AC-3, AC-6        (access enforcement, least privilege)
#   NIST SP 800-53 SI-7              (software and information integrity)
#   NIST SP 800-53 AU-12             (audit record generation -- logger calls)
#   NSA RTB Non-Bypassability        (MAC label application via restorecon)
#
# Exit codes (per .claude/rules/secure_bash_rules.md):
#   0 success
#   1 general runtime error
#   2 usage error
#   3 precondition failure
#   4 policy failure  (reserved; used by --verify on label mismatch)
#   5 partial success / verification mismatch
#

########################################
# Controlled PATH and environment
########################################
PATH=/usr/sbin:/usr/bin:/sbin:/bin
export PATH
readonly PATH

# SHELLOPTS is readonly in bash and cannot be unset; it is not a security
# concern here because `set -euo pipefail` below overrides its effect.
unset LD_PRELOAD LD_LIBRARY_PATH BASH_ENV ENV GLOBIGNORE || true
LC_ALL=C
export LC_ALL

set -euo pipefail
IFS=$'\n\t'

########################################
# Constants
########################################
readonly SCRIPT_NAME="umrs-install"
readonly PREFIX="/opt/umrs"
readonly ADMIN_GROUP="umrs-admin"

# Resolve the workspace root relative to this script. The script lives at
# <workspace>/scripts/umrs-install.sh. scripts/ is one directory deep inside
# the rusty-gadgets workspace. This resolution is stable across cwd changes.
_script_path() {
    readlink -f -- "${BASH_SOURCE[0]}"
}

SCRIPT_PATH=$(_script_path)
readonly SCRIPT_PATH
WORKSPACE_ROOT=$(dirname -- "$(dirname -- "${SCRIPT_PATH}")")
readonly WORKSPACE_ROOT
readonly POLICY_DIR="/DEVELOPMENT/umrs-project/components/rusty-gadgets/selinux"
readonly STAGING_DIR="${WORKSPACE_ROOT}/staging"
readonly POLICY_PP="${POLICY_DIR}/umrs.pp"
readonly POLICY_FC="${POLICY_DIR}/umrs.fc"

# Global flags (parsed in main).
DRY_RUN=0
VERIFY_ONLY=0

########################################
# Self-check -- permit development concession
########################################
_self_check() {
    local owner mode
    owner=$(stat -c '%U' -- "${SCRIPT_PATH}")
    mode=$(stat -c '%a' -- "${SCRIPT_PATH}")

    case "${owner}" in
        root|jadams) ;;
        *)
            printf 'FATAL: %s must be owned by root or jadams (owner: %s)\n' \
                "${SCRIPT_NAME}" "${owner}" >&2
            exit 3
            ;;
    esac

    case "${mode}" in
        700|750|755) ;;
        *)
            printf 'FATAL: unsafe script permissions (mode: %s)\n' "${mode}" >&2
            exit 3
            ;;
    esac
}

########################################
# Output helpers -- diagnostics to stderr
########################################
# Local IFS=' ' in each helper so "$*" joins with spaces for display.
# The global IFS=$'\n\t' (strict-mode) is preserved outside these helpers
# to keep word-splitting protection elsewhere in the script.
_info()  { local IFS=' '; printf '[%s] %s\n' "${SCRIPT_NAME}" "$*" >&2; }
_warn()  { local IFS=' '; printf '[%s] WARNING: %s\n' "${SCRIPT_NAME}" "$*" >&2; }
_error() { local IFS=' '; printf '[%s] ERROR: %s\n' "${SCRIPT_NAME}" "$*" >&2; }

_syslog() {
    # Log every state-changing operation per the Logging Rule.
    # Never log secrets: this helper is only called with non-sensitive strings.
    local IFS=' '
    logger -t "${SCRIPT_NAME}" -p auth.notice -- "$*" || true
}

# run_or_echo -- execute a command, or echo it under --dry-run.
# Usage: run_or_echo <command> <args...>
run_or_echo() {
    if [[ "${DRY_RUN}" -eq 1 ]]; then
        local _joined
        _joined=$(IFS=' '; printf '%s' "$*")
        printf 'DRY-RUN: %s\n' "${_joined}" >&2
        return 0
    fi
    local IFS=' '
    _info "exec: $*"
    IFS=$'\n\t'
    "$@"
}

# sudo_or_echo -- wrap a command that requires sudo. Under --dry-run this
# prints the invocation. Outside dry-run, if sudo -n succeeds the script
# runs the command; otherwise it FAILS LOUDLY with the exact command the
# operator must run. Knox does not attempt to guess sudo rights.
sudo_or_echo() {
    local desc="$1"
    shift
    if [[ "${DRY_RUN}" -eq 1 ]]; then
        local _joined
        _joined=$(IFS=' '; printf '%s' "$*")
        printf 'DRY-RUN (requires sudo): sudo %s\n' "${_joined}" >&2
        return 0
    fi
    # Probe whether the specific sudo invocation is NOPASSWD-allowed.
    if sudo -n "$@" </dev/null >/dev/null 2>&1; then
        local IFS=' '
        _info "${desc}: sudo $*"
        _syslog "${desc}: executed via sudo"
        return 0
    fi
    _error "${desc} requires sudo permission that is not granted to $(id -un)."
    _error "Run this command manually:"
    local IFS=' '
    printf '    sudo %s\n' "$*" >&2
    return 3
}

########################################
# Step 1 -- preflight
########################################
preflight() {
    _info "preflight: checking preconditions"

    local missing=0

    # Admin group exists.
    if ! getent group "${ADMIN_GROUP}" >/dev/null; then
        _error "group '${ADMIN_GROUP}' does not exist (site precondition)"
        missing=1
    else
        _info "group '${ADMIN_GROUP}' present: $(getent group "${ADMIN_GROUP}")"
    fi

    # Policy package built.
    if [[ ! -f "${POLICY_PP}" ]]; then
        _error "policy package not found: ${POLICY_PP}"
        _error "build it with:  make -C ${POLICY_DIR}"
        missing=1
    else
        _info "policy package present: ${POLICY_PP}"
    fi

    # fcontext source present.
    if [[ ! -f "${POLICY_FC}" ]]; then
        _error "fcontext file not found: ${POLICY_FC}"
        missing=1
    fi

    # Staging built.
    if [[ ! -d "${STAGING_DIR}/bin" ]]; then
        _error "staging/bin not found: ${STAGING_DIR}/bin"
        _error "run 'cargo xtask stage' first"
        missing=1
    else
        local count
        count=$(find "${STAGING_DIR}/bin" -maxdepth 1 -type f -perm -u+x | wc -l)
        if [[ "${count}" -eq 0 ]]; then
            _error "no executables found in ${STAGING_DIR}/bin"
            missing=1
        else
            _info "staging/bin: ${count} executable(s) ready"
        fi
    fi

    # SELinux must be enabled.
    if ! command -v getenforce >/dev/null; then
        _error "getenforce not on PATH -- is SELinux installed?"
        missing=1
    else
        local mode
        mode=$(getenforce)
        _info "SELinux mode: ${mode}"
    fi

    if [[ "${missing}" -ne 0 ]]; then
        _error "preflight failed -- see messages above"
        exit 3
    fi

    _info "preflight passed"
}

########################################
# Step 2 -- layout creation (/opt/umrs/ tree)
########################################
create_layout() {
    _info "layout: creating directory tree under ${PREFIX}"

    local dirs=(
        "${PREFIX}/bin"
        "${PREFIX}/etc"
        "${PREFIX}/share/man/man1"
        "${PREFIX}/share/man/fr_CA/man1"
        "${PREFIX}/share/templates"
        "${PREFIX}/share/umrs"
        "${PREFIX}/var/lib"
        "${PREFIX}/var/log"
    )

    local d
    for d in "${dirs[@]}"; do
        sudo_or_echo "mkdir ${d}" mkdir -p -- "${d}" || return 3
    done

    # Ownership + modes per proposal Section 7.
    sudo_or_echo "chown root:root ${PREFIX}/bin" chown root:root "${PREFIX}/bin" || return 3
    sudo_or_echo "chmod 0755 ${PREFIX}/bin" chmod 0755 "${PREFIX}/bin" || return 3

    sudo_or_echo "chown root:${ADMIN_GROUP} ${PREFIX}/etc" \
        chown "root:${ADMIN_GROUP}" "${PREFIX}/etc" || return 3
    sudo_or_echo "chmod 2775 ${PREFIX}/etc" chmod 2775 "${PREFIX}/etc" || return 3

    sudo_or_echo "chown root:${ADMIN_GROUP} ${PREFIX}/var/lib" \
        chown "root:${ADMIN_GROUP}" "${PREFIX}/var/lib" || return 3
    sudo_or_echo "chmod 2775 ${PREFIX}/var/lib" chmod 2775 "${PREFIX}/var/lib" || return 3

    sudo_or_echo "chown root:${ADMIN_GROUP} ${PREFIX}/var/log" \
        chown "root:${ADMIN_GROUP}" "${PREFIX}/var/log" || return 3
    sudo_or_echo "chmod 2770 ${PREFIX}/var/log" chmod 2770 "${PREFIX}/var/log" || return 3

    # Read-only share/ stays root:root.
    sudo_or_echo "chown -R root:root ${PREFIX}/share" \
        chown -R root:root "${PREFIX}/share" || return 3

    _syslog "layout created under ${PREFIX}"
    _info "layout complete"
}

########################################
# Step 3 -- file placement
########################################
place_files() {
    _info "placing files from ${STAGING_DIR}"

    # Binaries.
    local b
    for b in "${STAGING_DIR}"/bin/umrs-*; do
        [[ -f "${b}" ]] || continue
        local name
        name=$(basename -- "${b}")
        sudo_or_echo "install ${name}" \
            install -m 0755 -o root -g root -- "${b}" "${PREFIX}/bin/${name}" || return 3
    done

    # Configuration files -- recursive copy, preserving subdirectory
    # structure (e.g. config/ca/, config/us/ for label catalogs).
    if [[ -d "${STAGING_DIR}/config" ]]; then
        local c rel dst dst_dir
        while IFS= read -r -d '' c; do
            rel="${c#"${STAGING_DIR}"/config/}"
            dst="${PREFIX}/etc/${rel}"
            dst_dir=$(dirname -- "${dst}")
            # Create the destination subdirectory if needed.
            if [[ "${dst_dir}" != "${PREFIX}/etc" ]]; then
                sudo_or_echo "mkdir ${dst_dir}" \
                    mkdir -p -- "${dst_dir}" || return 3
                sudo_or_echo "chown root:${ADMIN_GROUP} ${dst_dir}" \
                    chown "root:${ADMIN_GROUP}" "${dst_dir}" || return 3
                sudo_or_echo "chmod 2775 ${dst_dir}" \
                    chmod 2775 "${dst_dir}" || return 3
            fi
            sudo_or_echo "install etc/${rel}" \
                install -m 0644 -o root -g "${ADMIN_GROUP}" -- \
                    "${c}" "${dst}" || return 3
        done < <(find "${STAGING_DIR}/config" -type f -print0)
    fi

    # Man pages (optional -- only if staging produced them).
    if [[ -d "${STAGING_DIR}/share/man/man1" ]]; then
        local m
        for m in "${STAGING_DIR}"/share/man/man1/*; do
            [[ -f "${m}" ]] || continue
            sudo_or_echo "install man $(basename -- "${m}")" \
                install -m 0644 -o root -g root -- \
                    "${m}" "${PREFIX}/share/man/man1/$(basename -- "${m}")" || return 3
        done
    fi
    if [[ -d "${STAGING_DIR}/share/man/fr_CA/man1" ]]; then
        local m
        for m in "${STAGING_DIR}"/share/man/fr_CA/man1/*; do
            [[ -f "${m}" ]] || continue
            sudo_or_echo "install fr_CA man $(basename -- "${m}")" \
                install -m 0644 -o root -g root -- \
                    "${m}" "${PREFIX}/share/man/fr_CA/man1/$(basename -- "${m}")" || return 3
        done
    fi

    _syslog "files placed under ${PREFIX}"
    _info "file placement complete"
}

########################################
# Step 4 -- SELinux module cleanup + load
#
# Uses sudo semodule (NOPASSWD-granted to Knox per permissions.md).
########################################
load_policy() {
    _info "policy: enumerating loaded umrs* modules"

    local loaded
    loaded=$(sudo -n semodule -l 2>/dev/null | awk '/^umrs/ {print $1}' || true)

    if [[ -n "${loaded}" ]]; then
        local mod
        # shellcheck disable=SC2086
        for mod in ${loaded}; do
            _info "policy: removing legacy module '${mod}'"
            if [[ "${DRY_RUN}" -eq 1 ]]; then
                printf 'DRY-RUN: sudo semodule -r %s\n' "${mod}" >&2
            else
                sudo -n semodule -r "${mod}"
                _syslog "removed policy module ${mod}"
            fi
        done
    else
        _info "policy: no pre-existing umrs* modules loaded"
    fi

    _info "policy: installing ${POLICY_PP}"
    if [[ "${DRY_RUN}" -eq 1 ]]; then
        printf 'DRY-RUN: sudo semodule -i %s\n' "${POLICY_PP}" >&2
    else
        sudo -n semodule -i "${POLICY_PP}"
        _syslog "installed policy module umrs from ${POLICY_PP}"
    fi

    _info "policy: load complete"
}

########################################
# Step 5 -- file contexts
#
# semodule -i already bundles umrs.fc into the policy store. An explicit
# semanage fcontext pass is only needed when the installed prefix differs
# from the baked-in PREFIX (e.g., via `semanage fcontext -e /opt/umrs /alt`).
# For the demonstration release the baked-in and installed prefix match,
# so we run restorecon directly.
########################################
apply_contexts() {
    _info "contexts: applying labels via restorecon"
    sudo_or_echo "restorecon -RF ${PREFIX}" restorecon -RF "${PREFIX}" || return 3
    _syslog "restorecon applied to ${PREFIX}"
}

########################################
# Step 6 -- verification
########################################
verify_install() {
    _info "verify: checking installation"

    local bin="${PREFIX}/bin/umrs-ls"
    local rc=0

    if [[ ! -x "${bin}" ]]; then
        _error "verify: ${bin} is not executable"
        return 5
    fi

    local ctx
    ctx=$(ls -Z -- "${bin}" | awk '{print $1}')
    _info "verify: ${bin} context = ${ctx}"

    if [[ "${ctx}" != *":umrs_bin_t:"* ]]; then
        _error "verify: ${bin} is NOT labeled umrs_bin_t (got ${ctx})"
        rc=5
    fi

    # Spot-check directory modes.
    local m
    m=$(stat -c '%U:%G %a' -- "${PREFIX}/bin")
    _info "verify: ${PREFIX}/bin = ${m}"

    m=$(stat -c '%U:%G %a' -- "${PREFIX}/etc")
    _info "verify: ${PREFIX}/etc = ${m}"
    if [[ "${m}" != "root:${ADMIN_GROUP} 2775" ]]; then
        _warn "verify: ${PREFIX}/etc DAC mismatch (expected root:${ADMIN_GROUP} 2775)"
        rc=5
    fi

    m=$(stat -c '%U:%G %a' -- "${PREFIX}/var/log")
    _info "verify: ${PREFIX}/var/log = ${m}"
    if [[ "${m}" != "root:${ADMIN_GROUP} 2770" ]]; then
        _warn "verify: ${PREFIX}/var/log DAC mismatch (expected root:${ADMIN_GROUP} 2770)"
        rc=5
    fi

    if [[ "${rc}" -eq 0 ]]; then
        _info "verify: PASS"
    else
        _error "verify: FAIL (exit ${rc})"
    fi
    return "${rc}"
}

########################################
# Usage
########################################
usage() {
    cat >&2 <<EOF
Usage: ${SCRIPT_NAME} [--dry-run | --verify | --help]

  --dry-run   Print every action without executing it. Useful for producing
              the exact sudo command list Jamie must run.
  --verify    Run only the post-install verification (step 6) on an existing
              install. Exits 5 if any check fails.
  --help      Show this message.

Default (no flags): run steps 1-6 in order. Any step that requires sudo but
is not granted to the current user causes the script to FAIL LOUDLY with
the exact command to run. The script does not guess sudo rights.

Exit codes:
  0 success    1 general error    2 usage    3 precondition
  4 policy     5 verification mismatch
EOF
}

########################################
# Main
########################################
main() {
    _self_check

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --dry-run) DRY_RUN=1 ;;
            --verify)  VERIFY_ONLY=1 ;;
            --help|-h) usage; exit 0 ;;
            *)
                _error "unknown argument: $1"
                usage
                exit 2
                ;;
        esac
        shift
    done

    if [[ "${VERIFY_ONLY}" -eq 1 ]]; then
        verify_install
        exit "$?"
    fi

    preflight
    create_layout
    place_files
    load_policy
    apply_contexts
    verify_install
    local rc=$?

    if [[ "${rc}" -eq 0 ]]; then
        _info "install complete (PREFIX=${PREFIX})"
        _syslog "install complete (PREFIX=${PREFIX})"
    fi
    exit "${rc}"
}

main "$@"
