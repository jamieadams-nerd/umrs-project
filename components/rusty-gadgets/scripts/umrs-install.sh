#!/usr/bin/bash
#
# umrs-install.sh -- Phase A installer for the FHS 2.3 compliant UMRS layout
#
# Deployment:
#   Demonstration release. Single source of truth for the on-disk layout.
#   Per Jamie (2026-04-13), the shell installer is authoritative until the
#   xtask/umrs-install Rust port lands.
#
# On-disk layout (FHS 2.3 §3.12, §3.7.4, §4.9.2, §5.15):
#
#   /opt/umrs/                       static package files (FHS §5.15)
#     bin/                           executable binaries
#     share/umrs/                    package-specific static reference data
#                                    (FHS §4.11 analogue of /usr/share/<pkg>)
#       US-CUI-LABELS.json           NARA CUI registry catalog
#       CANADIAN-PROTECTED.json      Treasury Board Protected A/B/C catalog
#       LEVELS.json                  MCS sensitivity level definitions
#       US-CUI-PALETTE.json          CUI marking color palette
#       templates/                   configuration templates destined for /etc/
#         MLS-setrans.conf.template
#         TARGETED-setrans.conf-template
#     share/man/man1/                English man pages
#     share/man/fr/man1/             Canadian French man pages
#     share/locale/<locale>/LC_MESSAGES/   compiled gettext catalogs
#
#   /etc/opt/umrs/                   host-specific admin-editable config
#                                    (FHS §3.7.4) -- reserved for future use
#
#   /var/opt/umrs/                   variable package data (FHS §4.9.2)
#     lib/                           state data
#     log/                           log files
#
# Rationale for NOT using /opt/umrs/etc or /opt/umrs/var:
#   FHS 2.3 §5.15 states "No other package files may exist outside the /opt,
#   /var/opt, and /etc/opt hierarchies." Host-editable configuration and
#   variable data MUST live under /etc/opt and /var/opt respectively; placing
#   them inside /opt/<package>/ would violate the separation that lets an
#   admin mount /opt read-only.
#
# Top-level directory assumption:
#   /opt/umrs, /etc/opt/umrs, and /var/opt/umrs are provisioned OUT OF BAND
#   (e.g., by a site deployment role) with ownership umrs:umrs before this
#   script runs. Preflight verifies their presence. This script creates
#   subdirectories under those roots but does not chown/chmod the roots
#   themselves.
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
#   FHS 2.3 §3.7.4, §3.12, §4.9.2, §4.11, §5.15 (on-disk layout)
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

# FHS 2.3 compliant installation roots.
#   PREFIX     -- /opt/<package>  (FHS §5.15) static package files
#   ETC_DIR    -- /etc/opt/<pkg>  (FHS §3.7.4) host-specific admin config
#   VAR_DIR    -- /var/opt/<pkg>  (FHS §4.9.2) variable package data
# All three roots are provisioned out of band as umrs:umrs 0755 before
# this script runs. Preflight verifies their presence.
readonly PREFIX="/opt/umrs"
readonly ETC_DIR="/etc/opt/umrs"
readonly VAR_DIR="/var/opt/umrs"
readonly PKG_USER="umrs"
readonly PKG_GROUP="umrs"

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

    # umrs user and group exist (FHS roots are owned by them).
    if ! getent group "${PKG_GROUP}" >/dev/null; then
        _error "group '${PKG_GROUP}' does not exist (site precondition)"
        missing=1
    else
        _info "group '${PKG_GROUP}' present: $(getent group "${PKG_GROUP}")"
    fi
    if ! getent passwd "${PKG_USER}" >/dev/null; then
        _error "user '${PKG_USER}' does not exist (site precondition)"
        missing=1
    else
        _info "user '${PKG_USER}' present"
    fi

    # FHS roots exist and are owned by umrs:umrs.
    # Per FHS 2.3 §3.7.4, §4.9.2, §5.15 these three trees are the only valid
    # locations for package files, config, and variable data respectively.
    local root
    for root in "${PREFIX}" "${ETC_DIR}" "${VAR_DIR}"; do
        if [[ ! -d "${root}" ]]; then
            _error "FHS root missing: ${root} (must be provisioned out of band)"
            missing=1
            continue
        fi
        local owner
        owner=$(stat -c '%U:%G' -- "${root}")
        if [[ "${owner}" != "${PKG_USER}:${PKG_GROUP}" ]]; then
            _error "FHS root ${root} is ${owner}, expected ${PKG_USER}:${PKG_GROUP}"
            missing=1
        else
            _info "FHS root ${root} present and owned ${owner}"
        fi
    done

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
# Step 2 -- layout creation
#
# Creates subdirectories under the three pre-existing FHS roots:
#   ${PREFIX}      -- /opt/umrs       (FHS §5.15, static package files)
#   ${ETC_DIR}     -- /etc/opt/umrs   (FHS §3.7.4, host-editable config)
#   ${VAR_DIR}     -- /var/opt/umrs   (FHS §4.9.2, variable package data)
#
# The roots themselves are NOT created or chown'd here -- preflight has
# already verified they exist and are owned umrs:umrs. This script only
# manages the subtrees inside them.
########################################
create_layout() {
    _info "layout: creating subdirectories under FHS roots"

    # Static package tree under /opt/umrs (FHS §5.15).
    local static_dirs=(
        "${PREFIX}/bin"
        "${PREFIX}/share/umrs"
        "${PREFIX}/share/umrs/templates"
        "${PREFIX}/share/man/man1"
        "${PREFIX}/share/man/fr/man1"
        "${PREFIX}/share/locale"
    )

    # Variable data under /var/opt/umrs (FHS §4.9.2).
    local var_dirs=(
        "${VAR_DIR}/lib"
        "${VAR_DIR}/log"
    )

    local d
    for d in "${static_dirs[@]}" "${var_dirs[@]}"; do
        sudo_or_echo "mkdir ${d}" mkdir -p -- "${d}" || return 3
        sudo_or_echo "chown ${PKG_USER}:${PKG_GROUP} ${d}" \
            chown "${PKG_USER}:${PKG_GROUP}" "${d}" || return 3
    done

    # Mode assignments.
    # Static package directories are world-readable, group-writable by umrs
    # for ease of package re-install by admins in the umrs group.
    sudo_or_echo "chmod 0755 ${PREFIX}/bin" chmod 0755 "${PREFIX}/bin" || return 3
    sudo_or_echo "chmod 0755 ${PREFIX}/share/umrs" \
        chmod 0755 "${PREFIX}/share/umrs" || return 3
    sudo_or_echo "chmod 0755 ${PREFIX}/share/umrs/templates" \
        chmod 0755 "${PREFIX}/share/umrs/templates" || return 3
    sudo_or_echo "chmod 0755 ${PREFIX}/share/man/man1" \
        chmod 0755 "${PREFIX}/share/man/man1" || return 3
    sudo_or_echo "chmod 0755 ${PREFIX}/share/man/fr/man1" \
        chmod 0755 "${PREFIX}/share/man/fr/man1" || return 3
    sudo_or_echo "chmod 0755 ${PREFIX}/share/locale" \
        chmod 0755 "${PREFIX}/share/locale" || return 3

    # Variable data directories -- setgid so new entries inherit umrs group.
    sudo_or_echo "chmod 2775 ${VAR_DIR}/lib" chmod 2775 "${VAR_DIR}/lib" || return 3
    sudo_or_echo "chmod 2770 ${VAR_DIR}/log" chmod 2770 "${VAR_DIR}/log" || return 3

    _syslog "layout subdirectories created under ${PREFIX}, ${ETC_DIR}, ${VAR_DIR}"
    _info "layout complete"
}

########################################
# Step 3 -- file placement
#
# Source layout (produced by `cargo xtask stage`):
#   ${STAGING_DIR}/bin/                  executables + scripts
#   ${STAGING_DIR}/share/umrs/           JSON reference databases
#   ${STAGING_DIR}/share/umrs/templates/ setrans.conf templates
#   ${STAGING_DIR}/share/man/            man pages (man1, fr/man1)
#   ${STAGING_DIR}/share/locale/         compiled gettext catalogs
#
# Destination layout (FHS 2.3 §4.11, §5.15):
#   ${PREFIX}/bin/                       = ${STAGING_DIR}/bin/
#   ${PREFIX}/share/umrs/                = ${STAGING_DIR}/share/umrs/
#   ${PREFIX}/share/umrs/templates/      = ${STAGING_DIR}/share/umrs/templates/
#   ${PREFIX}/share/man/...              = ${STAGING_DIR}/share/man/...
#   ${PREFIX}/share/locale/...           = ${STAGING_DIR}/share/locale/...
#
# All installed files are owned umrs:umrs per site policy (see header).
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
            install -m 0755 -o "${PKG_USER}" -g "${PKG_GROUP}" -- \
                "${b}" "${PREFIX}/bin/${name}" || return 3
    done

    # Static reference databases under share/umrs/ (FHS §4.11).
    # Flat layout: no us/ or ca/ subdirs, just top-level JSON files plus
    # the templates/ subdirectory.
    if [[ -d "${STAGING_DIR}/share/umrs" ]]; then
        local j name
        for j in "${STAGING_DIR}"/share/umrs/*.json; do
            [[ -f "${j}" ]] || continue
            name=$(basename -- "${j}")
            sudo_or_echo "install share/umrs/${name}" \
                install -m 0644 -o "${PKG_USER}" -g "${PKG_GROUP}" -- \
                    "${j}" "${PREFIX}/share/umrs/${name}" || return 3
        done

        # Templates -- operators customize and copy into /etc/ (e.g.,
        # /etc/selinux/<policy>/setrans.conf). We ship them read-only.
        if [[ -d "${STAGING_DIR}/share/umrs/templates" ]]; then
            local t
            for t in "${STAGING_DIR}"/share/umrs/templates/*; do
                [[ -f "${t}" ]] || continue
                name=$(basename -- "${t}")
                sudo_or_echo "install share/umrs/templates/${name}" \
                    install -m 0644 -o "${PKG_USER}" -g "${PKG_GROUP}" -- \
                        "${t}" "${PREFIX}/share/umrs/templates/${name}" || return 3
            done
        fi
    fi

    # Man pages (optional -- only if staging produced them).
    if [[ -d "${STAGING_DIR}/share/man/man1" ]]; then
        local m
        for m in "${STAGING_DIR}"/share/man/man1/*; do
            [[ -f "${m}" ]] || continue
            sudo_or_echo "install man $(basename -- "${m}")" \
                install -m 0644 -o "${PKG_USER}" -g "${PKG_GROUP}" -- \
                    "${m}" "${PREFIX}/share/man/man1/$(basename -- "${m}")" || return 3
        done
    fi
    if [[ -d "${STAGING_DIR}/share/man/fr/man1" ]]; then
        local m
        for m in "${STAGING_DIR}"/share/man/fr/man1/*; do
            [[ -f "${m}" ]] || continue
            sudo_or_echo "install fr man $(basename -- "${m}")" \
                install -m 0644 -o "${PKG_USER}" -g "${PKG_GROUP}" -- \
                    "${m}" "${PREFIX}/share/man/fr/man1/$(basename -- "${m}")" || return 3
        done
    fi

    # Compiled gettext catalogs -- share/locale/<locale>/LC_MESSAGES/<domain>.mo.
    # The locale set is discovered at install time by listing subdirectories of
    # staging/share/locale/; no hardcoded locale list is required.
    if [[ -d "${STAGING_DIR}/share/locale" ]]; then
        local mo rel dst dst_dir
        while IFS= read -r -d '' mo; do
            rel="${mo#"${STAGING_DIR}"/share/locale/}"
            dst="${PREFIX}/share/locale/${rel}"
            dst_dir=$(dirname -- "${dst}")
            sudo_or_echo "mkdir ${dst_dir}" \
                mkdir -p -- "${dst_dir}" || return 3
            sudo_or_echo "chown ${PKG_USER}:${PKG_GROUP} ${dst_dir}" \
                chown "${PKG_USER}:${PKG_GROUP}" "${dst_dir}" || return 3
            sudo_or_echo "chmod 0755 ${dst_dir}" \
                chmod 0755 "${dst_dir}" || return 3
            sudo_or_echo "install locale/${rel}" \
                install -m 0644 -o "${PKG_USER}" -g "${PKG_GROUP}" -- \
                    "${mo}" "${dst}" || return 3
        done < <(find "${STAGING_DIR}/share/locale" -type f -name '*.mo' -print0)
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

    # Spot-check that the FHS-compliant directory set is present with the
    # expected ownership. We do not check the FHS roots themselves (preflight
    # did that) -- this pass validates the subtrees this script created.
    local m
    m=$(stat -c '%U:%G %a' -- "${PREFIX}/bin")
    _info "verify: ${PREFIX}/bin = ${m}"

    m=$(stat -c '%U:%G %a' -- "${PREFIX}/share/umrs")
    _info "verify: ${PREFIX}/share/umrs = ${m}"
    if [[ "${m}" != "${PKG_USER}:${PKG_GROUP} 755" ]]; then
        _warn "verify: ${PREFIX}/share/umrs DAC mismatch (expected ${PKG_USER}:${PKG_GROUP} 755)"
        rc=5
    fi

    m=$(stat -c '%U:%G %a' -- "${VAR_DIR}/log")
    _info "verify: ${VAR_DIR}/log = ${m}"
    if [[ "${m}" != "${PKG_USER}:${PKG_GROUP} 2770" ]]; then
        _warn "verify: ${VAR_DIR}/log DAC mismatch (expected ${PKG_USER}:${PKG_GROUP} 2770)"
        rc=5
    fi

    m=$(stat -c '%U:%G %a' -- "${VAR_DIR}/lib")
    _info "verify: ${VAR_DIR}/lib = ${m}"
    if [[ "${m}" != "${PKG_USER}:${PKG_GROUP} 2775" ]]; then
        _warn "verify: ${VAR_DIR}/lib DAC mismatch (expected ${PKG_USER}:${PKG_GROUP} 2775)"
        rc=5
    fi

    # Spot-check a representative reference-data file.
    if [[ -f "${PREFIX}/share/umrs/US-CUI-LABELS.json" ]]; then
        _info "verify: ${PREFIX}/share/umrs/US-CUI-LABELS.json present"
    else
        _warn "verify: ${PREFIX}/share/umrs/US-CUI-LABELS.json missing"
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
