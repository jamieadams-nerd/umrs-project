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
# Key-material at-rest root per KEY-MANAGEMENT-DIRS.md (UMRS-SEC-KM-001).
# Deliberately OUTSIDE /opt/umrs per FHS §4.5 vs. NIST CM-6 / SA-8 resolution
# (KEY-MANAGEMENT-DIRS §6.3) -- keys follow the system-wide security baseline
# rather than add-on-package autonomy.
readonly KEYS_ROOT="/etc/keys/umrs"
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

    # Key-material container per KEY-MANAGEMENT-DIRS.md §2 -- provisioned
    # out of band as root:root 0700 (the installer cannot create the
    # container because sudoers grants are scoped to ${KEYS_ROOT}/* only;
    # the container itself is a site-policy boundary).
    if [[ ! -d "${KEYS_ROOT}" ]]; then
        _error "key-material root missing: ${KEYS_ROOT} (must be provisioned out of band as root:root 0700)"
        missing=1
    else
        local keys_owner keys_mode
        keys_owner=$(stat -c '%U:%G' -- "${KEYS_ROOT}")
        keys_mode=$(stat -c '%a' -- "${KEYS_ROOT}")
        if [[ "${keys_owner}" != "root:root" ]]; then
            _error "${KEYS_ROOT} is ${keys_owner}, expected root:root"
            missing=1
        elif [[ "${keys_mode}" != "700" ]]; then
            _error "${KEYS_ROOT} mode is ${keys_mode}, expected 700"
            missing=1
        else
            _info "key-material root ${KEYS_ROOT} present and owned root:root 0700"
        fi
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
    # Every leaf AND intermediate directory is listed so chown/chmod lands
    # on each node -- mkdir -p creates intermediates (share/, share/man/)
    # as root:root by default and they would otherwise not be retouched.
    local static_dirs=(
        "${PREFIX}/bin"
        "${PREFIX}/share"
        "${PREFIX}/share/umrs"
        "${PREFIX}/share/umrs/templates"
        "${PREFIX}/share/man"
        "${PREFIX}/share/man/man1"
        "${PREFIX}/share/man/fr"
        "${PREFIX}/share/man/fr/man1"
        "${PREFIX}/share/locale"
    )

    # /var/opt/umrs/{lib,log} are NOT created by this installer.
    # No UMRS tool currently writes to those paths; the fcontext rules
    # in umrs.fc.in are in place so first-use lazy creation is labeled
    # correctly by restorecon. Treat them the same way we treat
    # /var/lib/umrs/keys/ (KEY-MANAGEMENT-DIRS.md §2, §6.2).

    local d
    for d in "${static_dirs[@]}"; do
        sudo_or_echo "mkdir ${d}" mkdir -p -- "${d}" || return 3
        sudo_or_echo "chown ${PKG_USER}:${PKG_GROUP} ${d}" \
            chown "${PKG_USER}:${PKG_GROUP}" "${d}" || return 3
        sudo_or_echo "chmod 0755 ${d}" chmod 0755 "${d}" || return 3
    done

    # ------------------------------------------------------------------
    # Key material trees per KEY-MANAGEMENT-DIRS.md (UMRS-SEC-KM-001).
    #
    # /etc/keys/umrs/ and /var/lib/umrs/keys/ live OUTSIDE /opt/umrs
    # deliberately -- key material follows the system-wide security
    # baseline rather than FHS add-on-package autonomy (FHS §4.5 vs
    # NIST CM-6 / SA-8; NIST wins, resolution documented in
    # KEY-MANAGEMENT-DIRS.md §6.3).
    #
    # Ownership: root:root, mode 0700 on directories, 0600 on files.
    # These permissions are NON-NEGOTIABLE -- no setuid/setgid/world
    # bits. See KEY-MANAGEMENT-DIRS §4 permissions reference.
    #
    # Subtree created this pass (/etc/keys/umrs):
    #   sealing/    symmetric sealing keys (LUKS/dm-crypt)
    #   signing/    IMA/DIGSIG asymmetric signing keys
    #   wrapping/   Key-Encryption-Keys (KEKs)
    #   staging/    pre-activation signing material
    #
    # Controls: NIST SP 800-53 SC-3, SC-12, SC-12(1), SC-12(2),
    #                          SC-17, SC-28, AC-3, MP-6;
    #           NIST SP 800-57 Pt 1 Rev 5 §5.3;
    #           NIST SP 800-38F (wrapping);
    #           NIST SP 800-89 (signing-key assurance);
    #           CMMC L2 SC.L2-3.13.10.
    #
    # Future work (NOT implemented this pass, per Jamie 2026-04-16):
    #   - /var/lib/umrs/keys/{active,suspended,retired}/ -- runtime
    #     lifecycle state. SELinux types and fcontext ARE defined in
    #     umrs.fc.in; directories are created lazily by the runtime
    #     (or by a future installer pass) and restorecon applies
    #     labels on first use. See KEY-MANAGEMENT-DIRS §2 and §6.2.
    #   - umrs-secadm group and user: separate Linux account dedicated
    #     to signing operations. For now all key-material operations
    #     run as root; Phase 2 will introduce umrs-secadm alongside
    #     umrs-admin and route privileged key operations via sudo
    #     rules + a dedicated umrs_sign_t SELinux domain.
    #   - /etc/opt/umrs/key-policy.toml -- policy metadata (algorithm
    #     selection, rotation schedule, wrapping requirements). File
    #     itself is shipped by the package; directory already exists
    #     under ${ETC_DIR} so no mkdir is needed here.
    # ------------------------------------------------------------------

    _info "layout: provisioning key-material trees under ${KEYS_ROOT}"

    # The container ${KEYS_ROOT} itself is provisioned OUT OF BAND as
    # root:root 0700 (preflight verifies). This installer only manages
    # the four sub-directories below. Rationale: sudoers grants are
    # scoped to ${KEYS_ROOT}/* -- the container is intentionally NOT
    # writable via this script.
    local keys_subs=(
        "${KEYS_ROOT}/sealing"
        "${KEYS_ROOT}/signing"
        "${KEYS_ROOT}/wrapping"
        "${KEYS_ROOT}/staging"
    )
    for d in "${keys_subs[@]}"; do
        sudo_or_echo "mkdir ${d}" mkdir -p -- "${d}" || return 3
        sudo_or_echo "chown root:root ${d}" \
            chown root:root "${d}" || return 3
        sudo_or_echo "chmod 0700 ${d}" chmod 0700 "${d}" || return 3
    done

    _syslog "layout subdirectories created under ${PREFIX}, ${ETC_DIR}, ${VAR_DIR}, ${KEYS_ROOT}"
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
            install -m 0755 -o root -g "${PKG_GROUP}" -- \
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
                install -m 0644 -o root -g "${PKG_GROUP}" -- \
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
                    install -m 0644 -o root -g "${PKG_GROUP}" -- \
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
                install -m 0644 -o root -g "${PKG_GROUP}" -- \
                    "${m}" "${PREFIX}/share/man/man1/$(basename -- "${m}")" || return 3
        done
    fi
    if [[ -d "${STAGING_DIR}/share/man/fr/man1" ]]; then
        local m
        for m in "${STAGING_DIR}"/share/man/fr/man1/*; do
            [[ -f "${m}" ]] || continue
            sudo_or_echo "install fr man $(basename -- "${m}")" \
                install -m 0644 -o root -g "${PKG_GROUP}" -- \
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
                install -m 0644 -o root -g "${PKG_GROUP}" -- \
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

    # /etc/opt/umrs/ carries fcontext entries outside the @PREFIX@ tree
    # (see umrs.fc.in) -- key-policy.toml in particular must land here.
    sudo_or_echo "restorecon -RF ${ETC_DIR}" restorecon -RF "${ETC_DIR}" || return 3

    # /var/opt/umrs/ -- fcontext declared (see umrs.fc.in) but directories
    # not created by this installer (no UMRS tool writes there yet). When
    # the tree is populated lazily at runtime, operators should run
    # `sudo restorecon -RF /var/opt/umrs` manually.

    # /etc/keys/umrs/ key-material at-rest labels per KEY-MANAGEMENT-DIRS.md
    # §2 and §5.1. Each sub-directory gets its own dedicated type
    # (umrs_seal_key_t, umrs_sign_key_t, umrs_kek_t) via umrs.fc.in.
    sudo_or_echo "restorecon -RF ${KEYS_ROOT}" restorecon -RF "${KEYS_ROOT}" || return 3

    _syslog "restorecon applied to ${PREFIX}, ${ETC_DIR}, ${KEYS_ROOT}"
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

    # /var/opt/umrs/{log,lib} are NOT created by this installer -- fcontext
    # only. Skip DAC checks on those paths; they do not exist yet.
    _info "verify: ${VAR_DIR}/{log,lib} = (not created by installer; fcontext only)"

    # Spot-check a representative reference-data file.
    if [[ -f "${PREFIX}/share/umrs/US-CUI-LABELS.json" ]]; then
        _info "verify: ${PREFIX}/share/umrs/US-CUI-LABELS.json present"
    else
        _warn "verify: ${PREFIX}/share/umrs/US-CUI-LABELS.json missing"
        rc=5
    fi

    # Key-material subtree per KEY-MANAGEMENT-DIRS.md (UMRS-SEC-KM-001):
    # verify ownership, mode, and SELinux type per sub-directory. All
    # sub-directories must be root:root 0700 per §4 permissions reference.
    # Types land via restorecon from umrs.fc.in -- section 5.1 mapping:
    #   sealing/  -> umrs_seal_key_t
    #   signing/  -> umrs_sign_key_t
    #   wrapping/ -> umrs_kek_t
    #   staging/  -> umrs_sign_key_t  (reuses signing type in Phase 1;
    #                                  see umrs.te rationale)
    local sub expected_type
    for sub in sealing signing wrapping staging; do
        case "${sub}" in
            sealing)  expected_type="umrs_seal_key_t" ;;
            signing)  expected_type="umrs_sign_key_t" ;;
            wrapping) expected_type="umrs_kek_t" ;;
            staging)  expected_type="umrs_sign_key_t" ;;
        esac

        if [[ -d "${KEYS_ROOT}/${sub}" ]]; then
            m=$(stat -c '%U:%G %a' -- "${KEYS_ROOT}/${sub}")
            _info "verify: ${KEYS_ROOT}/${sub} = ${m}"
            if [[ "${m}" != "root:root 700" ]]; then
                _warn "verify: ${KEYS_ROOT}/${sub} DAC mismatch (expected root:root 700)"
                rc=5
            fi
            # SELinux type check. Skipped on systems where SELinux is
            # disabled; reported as warn, not fail.
            ctx=$(ls -Zd -- "${KEYS_ROOT}/${sub}" 2>/dev/null | awk '{print $1}')
            if [[ -n "${ctx}" && "${ctx}" != *":${expected_type}:"* ]]; then
                _warn "verify: ${KEYS_ROOT}/${sub} type mismatch (got ${ctx}, expected ${expected_type})"
                rc=5
            fi
        else
            _warn "verify: ${KEYS_ROOT}/${sub} missing"
            rc=5
        fi
    done

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
