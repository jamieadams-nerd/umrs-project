#!/usr/bin/env bash
#
# Copyright (c) 2025 Jamie L. Adams
# SPDX-License-Identifier: MIT
#
# TODO: Add rsyslogging when creating keys and such.
#
###############################################################################
# High-Assurance Signing Key Manager
#
# Features:
#   - Manages dedicated signing key pairs for:
#       * Audit log signing (e.g., /var/log/audit/)
#       * System log signing (e.g., /var/log/messages, /var/log/secure, etc.)
#       * Journal export signing (journal archives)
#       * Log-rotation bundle signing (tarballs in /var/log/archive)
#       * Archive-set signing (offline / tape / cold storage)
#       * Configuration snapshot signing (/etc backups)
#       * RPM database snapshot signing
#       * Software release / build signing
#   - Subcommands:
#       * init          : create base directories and any missing keys
#       * rotate        : rotate one or all keys, archiving old material
#       * list          : brief inventory (purpose, paths, fingerprints)
#       * report        : detailed, “formal” report of all keys
#       * bundle-public : export all current public certs for transport
#
#   - Configurable:
#       * Base directory       (default: /etc/pki/ha-signing)
#       * Algorithm / key size (default: RSA 4096, SHA-384 – FIPS-approved)
#       * Expiration in days   (default: 730)
#
#   - Design constraints:
#       * No passphrases on signing keys (non-interactive use)
#       * Private keys 0600, directories 0700, owned by root
#       * Verbose, colorized output where possible
#
# Usage (after stripping leading § characters):
#   ha-signing-manager.sh [global options]  [args]
#
# Global options:
#   -b|–base-dir DIR    : base directory (default /etc/pki/ha-signing)
#   -A|–alg ALG         : key algorithm (rsa|ecdsa) [default: rsa]
#   -L|–length LEN      : key length or curve (rsa bits, or ec curve)
#                          default for rsa: 4096, for ecdsa: secp384r1
#   -E|–expiry DAYS     : certificate validity in days [default: 730]
#   -n|–dry-run         : show what would be done, do not modify disk
#   -h|–help            : show help
#
# Examples:
#   ha-signing-manager.sh init
#   ha-signing-manager.sh -b /etc/pki/my-signing init
#   ha-signing-manager.sh -A ecdsa -L secp384r1 init
#   ha-signing-manager.sh rotate all
#   ha-signing-manager.sh report
#   ha-signing-manager.sh bundle-public /root/ha-public-keys.tar.gz
###############################################################################

set -euo pipefail

###############################################################################
# Color handling
###############################################################################

if [[ -t 1 ]]; then
    COLOR_RED=$’\033[31m’
    COLOR_GREEN=$’\033[32m’
    COLOR_YELLOW=$’\033[33m’
    COLOR_BLUE=$’\033[34m’
    COLOR_MAGENTA=$’\033[35m’
    COLOR_CYAN=$’\033[36m’
    COLOR_BOLD=$’\033[1m’
    COLOR_RESET=$’\033[0m’
else
    COLOR_RED=””
    COLOR_GREEN=””
    COLOR_YELLOW=””
    COLOR_BLUE=””
    COLOR_MAGENTA=””
    COLOR_CYAN=””
    COLOR_BOLD=””
    COLOR_RESET=””
fi

info()  { printf ‘%s[INFO]%s  %s\n’  “$COLOR_BLUE”  “$COLOR_RESET” “$”; }
warn()  { printf ‘%s[WARN]%s  %s\n’  “$COLOR_YELLOW” “$COLOR_RESET” “$”; }
error() { printf ‘%s[ERROR]%s %s\n’ “$COLOR_RED”   “$COLOR_RESET” “$” >&2; }
ok()    { printf ‘%s[OK]%s    %s\n’  “$COLOR_GREEN” “$COLOR_RESET” “$”; }

###############################################################################
# Defaults (overridable via CLI)
###############################################################################

BASE_DIR=”/etc/pki/ha-signing”
KEY_ALG=“rsa”          # rsa | ecdsa
KEY_LEN=””             # rsa: bits; ecdsa: curve name (e.g., secp384r1)
EXPIRY_DAYS=730        # 2 years
DRY_RUN=0

###############################################################################
# Key “profiles” – what each key is for
#   id|subdir|basename|description|signed-artifacts
###############################################################################

KEY_PROFILES=(
  “audit:audit:auditd-signing:Audit log signing:Files under /var/log/audit (e.g., /var/log/audit/audit.log)”
  “syslog:syslog:syslog-signing:System log signing:/var/log/messages, /var/log/secure, /var/log/maillog, etc.”
  “journal:journal:journal-signing:Journal export signing:journalctl –output=export archives and journal backups”
  “logrotate:logrotate:logrotate-signing:Log-rotation bundle signing:Tarballs and bundles in /var/log/archive”
  “archive:archive:archive-signing:Archive-set signing:Offline / tape / cold storage bundles”
  “config:config:config-signing:Configuration snapshot signing:/etc snapshot archives and configuration bundles”
  “rpmdb:rpmdb:rpmdb-signing:RPM database snapshot signing:Dumped RPM DB snapshots (e.g., rpm –qf listings)”
  “swrel:sw-release:swrel-signing:Software release/build signing:Internal build artifacts, tarballs, packages”
)

###############################################################################
# Helper: print usage
###############################################################################

usage() {
    cat <<EOF
Usage: $0 [global-options]  [args]

Global options:
  -b, –base-dir DIR    Base directory (default: ${BASE_DIR})
  -A, –alg ALG         Key algorithm: rsa | ecdsa  (default: ${KEY_ALG})
  -L, –length LEN      RSA bits (e.g., 4096) or EC curve (e.g., secp384r1)
  -E, –expiry DAYS     Certificate validity in days (default: ${EXPIRY_DAYS})
  -n, –dry-run         Show actions but do not modify disk
  -h, –help            Show this help

Commands:
  init                  Initialize base directories and create any missing keys
  rotate [all|ID…]    Rotate all keys or specified key IDs (e.g., audit, syslog)
  list                  Brief inventory of keys
  report                Detailed key report
  bundle-public OUT     Create tar.gz of current public certs at OUT

Examples:
  $0 init
  $0 -b /etc/pki/my-signing init
  $0 -A ecdsa -L secp384r1 init
  $0 rotate all
  $0 report
  $0 bundle-public /root/ha-public-keys.tar.gz
EOF
}

###############################################################################
# CLI argument parsing
###############################################################################

parse_global_args() {
    while [[ $# -gt 0 ]]; do
        case “$1” in
            -b|–base-dir)
                BASE_DIR=”$2”; shift 2 ;;
            -A|–alg)
                KEY_ALG=”$2”; shift 2 ;;
            -L|–length)
                KEY_LEN=”$2”; shift 2 ;;
            -E|–expiry)
                EXPIRY_DAYS=”$2”; shift 2 ;;
            -n|–dry-run)
                DRY_RUN=1; shift 1 ;;
            -h|–help)
                usage; exit 0 ;;
            )
                # First non-option is the command
                break ;;
        esac
    done

    if [[ -z “${KEY_LEN}” ]]; then
        if [[ “${KEY_ALG}” == “rsa” ]]; then
            KEY_LEN=“4096”
        elif [[ “${KEY_ALG}” == “ecdsa” ]]; then
            KEY_LEN=“secp384r1”
        else
            error “Unsupported algorithm: ${KEY_ALG}”
            exit 1
        fi
    fi

    if [[ $# -lt 1 ]]; then
        usage
        exit 1
    fi

    COMMAND=”$1”
    shift
    COMMAND_ARGS=(”$@”)
}

###############################################################################
# Safety checks
###############################################################################

require_root() {
    if [[ “$(id -u)” -ne 0 ]]; then
        error “This script must be run as root.”
        exit 1
    fi
}

require_openssl() {
    if ! command -v openssl >/dev/null 2>&1; then
        error “openssl not found. Install openssl and retry.”
        exit 1
    fi
}

maybe_run() {
    if [[ “${DRY_RUN}” -eq 1 ]]; then
        warn “[DRY RUN] $”
    else
        eval “$@”
    fi
}

###############################################################################
# Directory helpers
###############################################################################

ensure_directories() {
    local dirs=(
        “${BASE_DIR}”
        “${BASE_DIR}/private”
        “${BASE_DIR}/public”
        “${BASE_DIR}/meta”
        “${BASE_DIR}/archive”
    )
    for d in “${dirs[@]}”; do
        if [[ ! -d “$d” ]]; then
            info “Creating directory: $d”
            maybe_run “mkdir -p ‘$d’”
            maybe_run “chmod 700 ‘$d’”
            maybe_run “chown root:root ‘$d’”
        fi
    done
}

profile_get_field() {
    local profile=”$1”
    local index=”$2”
    IFS=’:’ read -r id subdir basename desc artifacts <<< “${profile}”
    case “$index” in
        id)        echo “$id” ;;
        subdir)    echo “$subdir” ;;
        basename)  echo “$basename” ;;
        desc)      echo “$desc” ;;
        artifacts) echo “$artifacts” ;;
        )         return 1 ;;
    esac
}

find_profile_by_id() {
    local want_id=”$1”
    local p
    for p in “${KEY_PROFILES[@]}”; do
        local id
        id=”$(profile_get_field “$p” id)”
        if [[ “$id” == “$want_id” ]]; then
            echo “$p”
            return 0
        fi
    done
    return 1
}

###############################################################################
# Key generation / rotation
###############################################################################

generate_keypair() {
    local id=”$1”
    local subdir=”$2”
    local basename=”$3”
    local desc=”$4”

    local priv_dir=”${BASE_DIR}/private/${subdir}”
    local pub_dir=”${BASE_DIR}/public/${subdir}”
    local meta_dir=”${BASE_DIR}/meta”

    local priv_key=”${priv_dir}/${basename}.key.pem”
    local pub_cert=”${pub_dir}/${basename}.crt.pem”
    local meta_file=”${meta_dir}/${id}.meta”

    info “Generating keypair for [${id}] – ${desc}”

    maybe_run “mkdir -p ‘$priv_dir’ ‘$pub_dir’”
    maybe_run “chmod 700 ‘$priv_dir’ ‘$pub_dir’”
    maybe_run “chown root:root ‘$priv_dir’ ‘$pub_dir’”

    # Enforce strict umask so private keys are 600
    local old_umask
    old_umask=$(umask)
    umask 0077

    if [[ “${KEY_ALG}” == “rsa” ]]; then
        info “  Algorithm: RSA ${KEY_LEN} (no passphrase, FIPS-approved) with SHA-384”
        maybe_run “openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:${KEY_LEN} -out ‘${priv_key}’”
        maybe_run “chmod 600 ‘${priv_key}’”
        maybe_run “chown root:root ‘${priv_key}’”
        maybe_run “openssl req -new -x509 -key ‘${priv_key}’ -out ‘${pub_cert}’ -days ${EXPIRY_DAYS} -sha384 -subj ‘/CN=${id}-signing-key/’”
    elif [[ “${KEY_ALG}” == “ecdsa” ]]; then
        info “  Algorithm: ECDSA ${KEY_LEN} (no passphrase, FIPS-approved curve) with SHA-384”
        maybe_run “openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:${KEY_LEN} -pkeyopt ec_param_enc:named_curve -out ‘${priv_key}’”
        maybe_run “chmod 600 ‘${priv_key}’”
        maybe_run “chown root:root ‘${priv_key}’”
        maybe_run “openssl req -new -x509 -key ‘${priv_key}’ -out ‘${pub_cert}’ -days ${EXPIRY_DAYS} -sha384 -subj ‘/CN=${id}-signing-key/’”
    else
        error “Unsupported algorithm: ${KEY_ALG}”
        umask “${old_umask}”
        return 1
    fi

    umask “${old_umask}”

    maybe_run “chown root:root ‘${pub_cert}’”
    maybe_run “chmod 644 ‘${pub_cert}’”

    # Record metadata
    local created_ts expires_raw expires_ts fp
    created_ts=”$(date -u +%Y-%m-%dT%H:%M:%SZ)”
    expires_raw=”$(openssl x509 -in “${pub_cert}” -noout -enddate 2>/dev/null | sed ‘s/^notAfter=//’)” || true
    if [[ -n “${expires_raw}” ]]; then
        expires_ts=”${expires_raw}”
    else
        expires_ts=“unknown”
    fi
    fp=”$(openssl x509 -in “${pub_cert}” -noout -fingerprint -sha256 2>/dev/null | sed ‘s/^SHA256 Fingerprint=//’)” || true

    info “  Created : ${created_ts}”
    info “  Expires : ${expires_ts}”
    info “  Cert FP : ${fp:-unknown}”

    if [[ “${DRY_RUN}” -eq 0 ]]; then
        cat > “${meta_file}” <<EOF_META
id=${id}
subdir=${subdir}
basename=${basename}
description=${desc}
created_utc=${created_ts}
expires=${expires_ts}
algorithm=${KEY_ALG}
length=${KEY_LEN}
private_key=${priv_key}
public_cert=${pub_cert}
fingerprint_sha256=${fp}
EOF_META
        chmod 600 “${meta_file}”
        chown root:root “${meta_file}”
    else
        warn “[DRY RUN] Would write metadata to ${meta_file}”
    fi

    ok “Keypair for [${id}] ready (priv: ${priv_key}, pub: ${pub_cert})”
}

rotate_key() {
    local id=”$1”
    local subdir=”$2”
    local basename=”$3”
    local desc=”$4”

    local priv_dir=”${BASE_DIR}/private/${subdir}”
    local pub_dir=”${BASE_DIR}/public/${subdir}”
    local meta_dir=”${BASE_DIR}/meta”
    local archive_dir=”${BASE_DIR}/archive/${id}”

    local priv_key=”${priv_dir}/${basename}.key.pem”
    local pub_cert=”${pub_dir}/${basename}.crt.pem”
    local meta_file=”${meta_dir}/${id}.meta”

    local ts
    ts=”$(date -u +%Y%m%dT%H%M%SZ)”

    info “Rotating key [${id}] – ${desc}”

    if [[ -f “${priv_key}” || -f “${pub_cert}” || -f “${meta_file}” ]]; then
        info “  Archiving existing material for [${id}] to ${archive_dir}”
        maybe_run “mkdir -p ‘${archive_dir}’”

        if [[ -f “${priv_key}” ]]; then
            maybe_run “mv ‘${priv_key}’ ‘${archive_dir}/${basename}.${ts}.key.pem’”
        fi
        if [[ -f “${pub_cert}” ]]; then
            maybe_run “mv ‘${pub_cert}’ ‘${archive_dir}/${basename}.${ts}.crt.pem’”
        fi
        if [[ -f “${meta_file}” ]]; then
            maybe_run “mv ‘${meta_file}’ ‘${archive_dir}/${id}.${ts}.meta’”
        fi
    else
        warn “  No existing material for [${id}] – this will behave like init for this key.”
    fi

    generate_keypair “${id}” “${subdir}” “${basename}” “${desc}”
}

###############################################################################
# Commands
###############################################################################

cmd_init() {
    require_root
    require_openssl
    ensure_directories

    info “Initializing key material under ${BASE_DIR}”

    local p
    for p in “${KEY_PROFILES[@]}”; do
        local id subdir basename desc
        id=”$(profile_get_field “$p” id)”
        subdir=”$(profile_get_field “$p” subdir)”
        basename=”$(profile_get_field “$p” basename)”
        desc=”$(profile_get_field “$p” desc)”

        local priv_key=”${BASE_DIR}/private/${subdir}/${basename}.key.pem”
        local pub_cert=”${BASE_DIR}/public/${subdir}/${basename}.crt.pem”

        if [[ -f “${priv_key}” && -f “${pub_cert}” ]]; then
            ok “Keypair for [${id}] already exists; skipping.”
            continue
        fi

        generate_keypair “${id}” “${subdir}” “${basename}” “${desc}”
    done

    ok “Initialization complete.”
}

cmd_rotate() {
    require_root
    require_openssl
    ensure_directories

    local targets=()

    if [[ “${#COMMAND_ARGS[@]}” -eq 0 ]]; then
        error “rotate requires ‘all’ or one or more key IDs.”
        exit 1
    fi

    if [[ “${COMMAND_ARGS[0]}” == “all” ]]; then
        local p
        for p in “${KEY_PROFILES[@]}”; do
            targets+=(”$(profile_get_field “$p” id)”)
        done
    else
        targets=(”${COMMAND_ARGS[@]}”)
    fi

    local id
    for id in “${targets[@]}”; do
        local p
        if ! p=”$(find_profile_by_id “${id}”)”; then
            warn “Unknown key ID: ${id} – skipping.”
            continue
        fi
        local subdir basename desc
        subdir=”$(profile_get_field “$p” subdir)”
        basename=”$(profile_get_field “$p” basename)”
        desc=”$(profile_get_field “$p” desc)”
        rotate_key “${id}” “${subdir}” “${basename}” “${desc}”
    done

    ok “Rotation operation complete.”
}

###############################################################################
# Inventory helpers (list/report/scan)
###############################################################################

print_key_summary() {
    local id=”$1”
    local meta_file=”$2”

    local alg length created expires fp priv pub desc
    alg=$(grep ‘^algorithm=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    length=$(grep ‘^length=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    created=$(grep ‘^created_utc=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    expires=$(grep ‘^expires=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    fp=$(grep ‘^fingerprint_sha256=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    priv=$(grep ‘^private_key=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    pub=$(grep ‘^public_cert=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “unknown”)
    desc=$(grep ‘^description=’ “${meta_file}” 2>/dev/null | cut -d= -f2- || echo “n/a”)

    printf ‘%s- ID:%s %s\n’ “${COLOR_BOLD}” “${COLOR_RESET}” “${id}”
    printf ’    Description      : %s\n’ “${desc}”
    printf ’    Algorithm/Length : %s / %s\n’ “${alg}” “${length}”
    printf ’    Created (UTC)    : %s\n’ “${created}”
    printf ’    Expires          : %s\n’ “${expires}”
    printf ’    SHA256 FP        : %s\n’ “${fp}”
    printf ’    Private key      : %s\n’ “${priv}”
    printf ’    Public cert      : %s\n’ “${pub}”
}

cmd_list() {
    require_openssl
    ensure_directories

    info “Listing keys defined in ${BASE_DIR}/meta”

    shopt -s nullglob
    local meta
    for meta in “${BASE_DIR}/meta/”.meta; do
        local id
        id=$(basename “${meta}” .meta)
        print_key_summary “${id}” “${meta}”
    done
    shopt -u nullglob

    ok “List complete.”
}

cmd_report() {
    require_openssl
    ensure_directories

    info “Generating detailed key report from ${BASE_DIR}”

    local now
    now=”$(date -u +%Y-%m-%dT%H:%M:%SZ)”

    echo
    echo “======================================================================”
    echo “HIGH-ASSURANCE SIGNING KEY REPORT”
    echo “Generated at (UTC): ${now}”
    echo “Base directory     : ${BASE_DIR}”
    echo “======================================================================”
    echo

    shopt -s nullglob
    local meta
    for meta in “${BASE_DIR}/meta/”*.meta; do
        local id
        id=$(basename “${meta}” .meta)
        echo “——————————————————————”
        print_key_summary “${id}” “${meta}”
        echo

        local pub
        pub=$(grep ‘^public_cert=’ “${meta}” 2>/dev/null | cut -d= -f2- || echo “”)
        if [[ -n “${pub}” && -f “${pub}” ]]; then
            echo “    [OpenSSL view of current public cert:]”
            openssl x509 -in “${pub}” -noout -subject -issuer -serial -startdate -enddate -fingerprint -sha256 || true
        else
            echo “    [No public cert found on disk for this key.]”
        fi
        echo
    done
    shopt -u nullglob

    echo “======================================================================”
    echo “Archive material (for verifying older data):”
    echo “Location: ${BASE_DIR}/archive”
    echo “======================================================================”

    find “${BASE_DIR}/archive” -mindepth 2 -type f -name '*.crt.pem' -o -name '*.key.pem' -o -name '*.meta' 2>/dev/null 
        | sed “s|^|  |” || true

    echo
    ok “Report generation complete.”
}

cmd_bundle_public() {
    require_openssl
    ensure_directories

    if [[ “${#COMMAND_ARGS[@]}” -ne 1 ]]; then
        error “bundle-public requires one argument: output tar.gz path”
        exit 1
    fi

    local out=”$1”
    local out_dir
    out_dir=$(dirname “${out}”)

    info “Bundling current public certs from ${BASE_DIR}/public into ${out}”

    if [[ “${DRY_RUN}” -eq 1 ]]; then
        warn “[DRY RUN] Would create tar.gz at: ${out}”
        return 0
    fi

    mkdir -p “${out_dir}”

    (
      cd “${BASE_DIR}/public”
      tar czf “${out}” .
    )

    ok “Public bundle created at ${out}”
}

###############################################################################
# Main
###############################################################################

main() {
    parse_global_args “$@”

    case “${COMMAND}” in
        init)
            cmd_init
            ;;
        rotate)
            cmd_rotate
            ;;
        list)
            cmd_list
            ;;
        report)
            cmd_report
            ;;
        bundle-public)
            cmd_bundle_public
            ;;
        *)
            error “Unknown command: ${COMMAND}”
            usage
            exit 1
            ;;
    esac
}

main “$@”
