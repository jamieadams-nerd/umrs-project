#!/bin/bash

# umrs-shred: UMRS best-effort file sanitization wrapper
#
# SECURITY MODEL
# - Any user may shred files they are permitted to modify (usually files they own)
# - Root may shred any file
# - No forced root requirement
# - Per-user locking to avoid privilege collisions
#
# HONESTY NOTE
# - Best-effort overwrite using shred
# - Not guaranteed on SSD/NVMe or on CoW/journaling filesystems
# - UMRS treats this as defense-in-depth; primary assurance is full-disk encryption + key destruction
#
# USAGE
# umrs-shred FILE [FILE ...]
#
# EXIT CODES
# 0 success
# 2 usage error
# 3 precondition failure / refused target
# 4 authorization failure
# 5 shred failure
# 6 lock failure

set -euo pipefail
IFS=$'\n\t'
umask 077
set -o noclobber

export PATH='/usr/sbin:/usr/bin:/sbin:/bin'
readonly PATH

unset -v LD_LIBRARY_PATH LD_PRELOAD LD_AUDIT LD_DEBUG LD_ASSUME_KERNEL LD_ORIGIN_PATH LD_PROFILE LD_USE_LOAD_BIAS || true
unset -v DYLD_LIBRARY_PATH DYLD_INSERT_LIBRARIES || true
unset -v PYTHONPATH PYTHONHOME PYTHONSTARTUP PERL5LIB PERL5OPT RUBYLIB GEM_PATH GEM_HOME || true
unset -v BASH_ENV ENV || true

readonly UMRS_TOOL_NAME='umrs-shred'
readonly UMRS_SYSLOG_TAG='umrs-shred'

readonly PASSES=3
readonly DO_ZERO=1
readonly DO_UNLINK=1

LOCK_FD=''
LOCK_DIR=''
LOCK_FILE=''

die() {
 local -r code="$1"
 local -r msg="$2"
 printf '%s: ERROR: %s\n' "${UMRS_TOOL_NAME}" "${msg}" >&2
 exit "${code}"
}

usage() {
 printf '%s\n' "Usage: ${UMRS_TOOL_NAME} FILE [FILE ...]" >&2
}

have_cmd() {
 command -v -- "$1" >/dev/null 2>&1
}

now_iso_utc() {
 date -u '+%Y-%m-%dT%H:%M:%SZ'
}

get_actor() {
 local a=''
 a="$(id -un 2>/dev/null || true)"
 if [[ -z "${a}" ]]; then
 a="uid:$(id -u 2>/dev/null || printf 'unknown')"
 fi
 printf '%s' "${a}"
}

get_loginuid() {
 if [[ -r /proc/self/loginuid ]]; then
 cat /proc/self/loginuid 2>/dev/null || true
 else
 printf ''
 fi
}

get_selinux_context() {
 if id -Z >/dev/null 2>&1; then
 id -Z 2>/dev/null || true
 else
 printf ''
 fi
}

log_msg() {
 local -r level="$1"
 local -r msg="$2"

 local ts_utc actor uid loginuid selinux
 ts_utc="$(now_iso_utc)"
 actor="$(get_actor)"
 uid="$(id -u 2>/dev/null || printf 'unknown')"
 loginuid="$(get_loginuid)"
 selinux="$(get_selinux_context)"

 if [[ -z "${loginuid}" ]]; then
 loginuid='unknown'
 fi
 if [[ -z "${selinux}" ]]; then
 selinux='none'
 fi

 if have_cmd logger; then
 logger -t "${UMRS_SYSLOG_TAG}" -- "ts_utc=${ts_utc} level=${level} tool=${UMRS_TOOL_NAME} actor=${actor} uid=${uid} loginuid=${loginuid} selinux=${selinux} ${msg}" || true
 else
 printf '%s\n' "ts_utc=${ts_utc} level=${level} tool=${UMRS_TOOL_NAME} actor=${actor} uid=${uid} loginuid=${loginuid} selinux=${selinux} ${msg}" >&2
 fi
}

choose_lock_location() {
 local -r uid="$(id -u)"
 if [[ "${uid}" -eq 0 ]]; then
 LOCK_DIR='/run/umrs'
 else
 if [[ -d "/run/user/${uid}" && -w "/run/user/${uid}" && -x "/run/user/${uid}" ]]; then
 LOCK_DIR="/run/user/${uid}/umrs"
 else
 LOCK_DIR="/tmp/umrs-${uid}"
 fi
 fi
 LOCK_FILE="${LOCK_DIR}/${UMRS_TOOL_NAME}.lock"
}

ensure_lock_dir() {
 install -d -m 0700 -- "${LOCK_DIR}" || die 6 "Cannot create lock directory: ${LOCK_DIR}"
}

acquire_lock() {
 have_cmd flock || return 0
 choose_lock_location
 ensure_lock_dir
 exec {LOCK_FD}>"${LOCK_FILE}" || die 6 "Cannot open lock file: ${LOCK_FILE}"
 flock -n "${LOCK_FD}" || die 6 "Another ${UMRS_TOOL_NAME} instance is running"
}

# ShellCheck cannot always see trap-driven control flow. These are invoked by cleanup() via trap.
# shellcheck disable=SC2317
release_lock() {
 if [[ -n "${LOCK_FD}" ]]; then
 exec {LOCK_FD}>&- >/dev/null 2>&1 || true
 LOCK_FD=''
 fi
 if [[ -n "${LOCK_FILE}" && -e "${LOCK_FILE}" ]]; then
 rm -f -- "${LOCK_FILE}" >/dev/null 2>&1 || true
 fi
}

# shellcheck disable=SC2317
cleanup() {
 release_lock
}
trap cleanup EXIT HUP INT TERM

validate_target() {
 local -r p="$1"

 [[ -n "${p}" ]] || die 2 "Empty path not allowed"

 if [[ "${p}" != /* ]]; then
 die 3 "Refusing non-absolute path: ${p}"
 fi

 if [[ "${p}" == $'\n' || "${p}" == $'\r' || "${p}" == $'\t' ]]; then
 die 3 "Path contains control characters"
 fi

 [[ -e "${p}" ]] || die 3 "Target does not exist: ${p}"
 [[ ! -L "${p}" ]] || die 3 "Refusing symlink: ${p}"
 [[ -f "${p}" ]] || die 3 "Not a regular file: ${p}"

 if [[ "$(id -u)" -ne 0 && ! -w "${p}" ]]; then
 die 4 "Not writable; rerun as root if authorized: ${p}"
 fi
}

run_shred() {
 local -r p="$1"
 local -a args=()

 args+=("--iterations=${PASSES}")
 args+=("--verbose")
 [[ "${DO_ZERO}" -eq 1 ]] && args+=("--zero")
 [[ "${DO_UNLINK}" -eq 1 ]] && args+=("--remove")

 shred "${args[@]}" -- "${p}" >/dev/null 2>&1
}

main() {
 have_cmd shred || die 3 "Required command not found: shred"
 have_cmd date || die 3 "Required command not found: date"
 have_cmd id || die 3 "Required command not found: id"

 [[ "$#" -ge 1 ]] || { usage; die 2 "No files specified"; }

 acquire_lock

 local fail=0
 local f=''

 for f in "$@"; do
 validate_target "${f}"

 log_msg "INFO" "action=sanitize result=BEGIN file=$(printf '%q' "${f}") detail=requested"

 if run_shred "${f}"; then
 log_msg "INFO" "action=sanitize result=SUCCESS file=$(printf '%q' "${f}") detail=shred_completed"
 else
 log_msg "ERROR" "action=sanitize result=FAIL file=$(printf '%q' "${f}") detail=shred_failed"
 printf '%s: ERROR: Failed to sanitize: %s\n' "${UMRS_TOOL_NAME}" "${f}" >&2
 fail=1
 fi
 done

 if [[ "${fail}" -ne 0 ]]; then
 die 5 "One or more files failed sanitization"
 fi

 exit 0
}

main "$@"
