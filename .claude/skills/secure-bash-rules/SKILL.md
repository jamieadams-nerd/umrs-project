---
name: secure-bash-rules
description: >
  Security rules for writing bash scripts in the UMRS project: shebang directives, PATH
  locking, environment sanitization, strict mode, quoting, input validation, exit codes,
  temporary files, privileged script requirements, and secret handling. Use this skill when
  working with bash scripts, .sh files, shebang, PATH lock, env sanitization in bash,
  strict mode, privileged scripts, or writing bash. Trigger when creating or editing any
  .sh file or bash script content.
---

## Scope

These rules apply when **writing bash script files** (`.sh` files with a shebang,
or scripts intended to be saved and executed). They do not apply to one-liner
commands agents run interactively via the Bash tool during development.

## Interpreter Directive Rule

[RULE]
All bash scripts MUST use the absolute interpreter path in the shebang line.

- Use `#!/usr/bin/bash`, never `#!/usr/bin/env bash`.
- `env`-based shebangs allow PATH manipulation to substitute a different interpreter at runtime.
- The absolute path binds the script to the system interpreter with a known provenance.

## Controlled PATH Rule

[RULE]
Scripts MUST define, export, and lock PATH explicitly at the top of the script body.

- Set PATH to the minimum required directories. Example: `PATH=/usr/bin:/usr/sbin:/bin`
- Export and mark readonly immediately: `export PATH; readonly PATH`
- Never rely on an inherited PATH from the calling environment.

Rationale: an attacker-controlled PATH allows command substitution attacks against unqualified command names.

## Environment Sanitization Rule

[RULE]
Scripts MUST unset dangerous inherited environment variables before performing any security-relevant work.

Variables to unset unconditionally:
- `LD_PRELOAD` — allows shared library injection
- `LD_LIBRARY_PATH` — redirects dynamic linker resolution
- `BASH_ENV` — sourced by bash in non-interactive mode
- `ENV` — sourced by POSIX shells
- `SHELLOPTS` — can re-enable dangerous shell options
- `GLOBIGNORE` — affects glob expansion behavior

Additionally:
- Set `LC_ALL=C` to prevent locale-dependent parsing of tool output.
- Set `IFS` to a safe value after unsetting inherited IFS (see Strict Mode Rule).

## Strict Mode Rule

[RULE]
Every script MUST enable strict execution mode immediately after environment sanitization.

Required options:

```bash
set -euo pipefail
IFS=$'\n\t'
```

- `-e`: exit immediately on any command returning non-zero.
- `-u`: treat unset variables as errors.
- `-o pipefail`: the exit status of a pipeline is the rightmost non-zero exit, not the last command.
- `IFS=$'\n\t'`: removes the space character from the field separator, preventing word-splitting on filenames with spaces.

[ANTI-PATTERN]
Do not use `IFS=' \n\t'` (with a leading space) in security contexts — space-splitting enables argument injection through filenames.

## Quoting Rule

[RULE]
All variable expansions MUST be double-quoted unless word-splitting is explicitly required.

- Use `"${variable}"` not `$variable`.
- Use `[[ ]]` for conditionals, not `[ ]`. `[[ ]]` is a bash keyword and is not subject to word-splitting or pathname expansion.
- Disable globbing for any code path that handles untrusted input: `set -f` before expansion, `set +f` after.

[ANTI-PATTERN]
Unquoted variable expansions in command arguments are a word-splitting and glob injection vector. Never leave them bare.

## Variable Scoping Rule

[RULE]
- Declare constants with `readonly` at script scope.
- Declare all variables inside functions with `local`.
- Do not allow function-local state to leak into the global script environment.

Example:

```bash
readonly CONFIG_DIR=/etc/umrs
some_function() {
    local result
    result=$(compute_something)
    echo "${result}"
}
```

## Input Validation Rule

[RULE]
All external input (arguments, environment variables, file content) MUST be validated before use.

- Validate at the earliest possible point — on ingestion, not at the use site.
- Use allow-lists over deny-lists. Define what is permitted; reject everything else.
- Validate numeric values before any arithmetic or numeric comparison.
- Reject inputs that contain unexpected characters (path separators, shell metacharacters, null bytes) unless they are explicitly required and handled.

[PATTERN]
For argument validation, define an explicit allow-list pattern and match against it:

```bash
[[ "${mode}" =~ ^(enforce|permissive|disabled)$ ]] || { echo "Invalid mode" >&2; exit 2; }
```

## Type-Correct Comparison Rule

[RULE]
Use the correct comparison operator for the operand type. Never mix string and integer operators.

- Integer comparison: `-eq`, `-ne`, `-lt`, `-le`, `-gt`, `-ge`
- String comparison: `==`, `!=`, `<`, `>`

[ANTI-PATTERN]
- Never use `==` to compare integers — string comparison gives wrong results for values with leading zeros or differing whitespace.
- Never use `-eq` to compare strings — bash coerces non-numeric strings to zero silently, masking bugs.

## Exit Code Rule

[RULE]
Scripts MUST use the following standardized exit code assignments. Deviations require a comment explaining the reason.

| Code | Meaning |
|------|---------|
| 0 | Success — no action required |
| 1 | General runtime error |
| 2 | Usage error — invalid arguments or missing required input |
| 3 | Precondition failure — required file, service, or dependency absent |
| 4 | Policy failure — a security policy check failed (hard enforcement result) |
| 5 | Policy warning — a policy check found a non-conformance (soft enforcement) |
| 6 | Partial success — some operations succeeded, some failed |

Rationale: Consistent exit codes allow callers, systemd units, and monitoring systems to interpret results without parsing output.

## Temporary File Rule

[RULE]
Temporary files MUST be created with `mktemp` and cleaned up unconditionally via `trap`.

- Never construct temporary paths manually (e.g., `/tmp/myfile.$$`).
- Register cleanup before creating the file so cleanup runs even if creation fails.
- Write to a staging path, then rename atomically to the final destination.

[PATTERN]
Canonical temporary file pattern:

```bash
readonly tmpfile
tmpfile=$(mktemp)
trap 'rm -f "${tmpfile}"' EXIT

# ... populate tmpfile ...

mv -- "${tmpfile}" /path/to/final/destination
```

[ANTI-PATTERN]
Manually constructed temp paths (`/tmp/script.$$`) are predictable and subject to symlink attacks. Never use them.

## Output Discipline Rule

[RULE]
- Machine-readable output goes to stdout only.
- Diagnostic messages, progress, and errors go to stderr only.
- Never mix machine output and diagnostic text on the same stream.

[CONSTRAINT]
A script that emits diagnostic text to stdout cannot be safely consumed by a downstream process or pipeline without parsing errors. This constraint cannot be relaxed.

Example:

```bash
echo "Processing complete" >&2          # diagnostic — stderr
echo "${json_result}"                   # machine output — stdout
```

## Logging Rule

[RULE]
Scripts that perform state-changing operations (file modification, service restart, label change, permission change) MUST emit a structured log entry to syslog using `logger`.

- Use `logger -t umrs-<scriptname> -p <facility>.<severity>` for all state-change events.
- Use `auth.notice` or `auth.warning` for security-relevant events.

[CONSTRAINT]
Scripts MUST NOT log secrets, tokens, passwords, or key material to syslog, stderr, or any file. This constraint cannot be relaxed under any circumstance.

## Language Escalation Rule

[RULE]
Bash scripts are the orchestration layer only. When a script grows beyond simple command sequencing, escalate to Python or Rust.

Escalate when the script requires any of the following:
- Complex data structure manipulation (associative arrays beyond simple key-value)
- Multi-step error handling with recovery logic
- Cryptographic operations of any kind
- Network communication
- Parsing structured formats (JSON, XML, YAML) beyond a single `jq` call
- More than approximately 100 lines of logic

[ANTI-PATTERN]
Do not implement security logic (parsing, validation, cryptographic verification) in bash. Bash has no type system, no bounds checking, and error handling is fragile. Delegate to Rust.

## Secret Handling Rule

[CONSTRAINT]
Scripts MUST NOT embed secrets, tokens, passwords, API keys, or key material in any form — not plaintext, not base64, not as comments.

- Secrets must be read from a file with restricted permissions at runtime, or passed via a file descriptor (not an argument).
- Command-line arguments are visible in `ps` output and process accounting logs. Never pass secrets as arguments.

[ANTI-PATTERN]
`PASSWORD="s3cr3t"` in a script is a hard violation regardless of who can read the file. Process table exposure is universal.

## Privileged Script Rule

[RULE]
Scripts that run as root or with elevated capabilities MUST satisfy all of the following conditions:

- Owned by `root:root`
- Mode `0700` (root-only) or `0750` (root + trusted group) — no world-readable or world-executable bit
- Located in a directory that is itself root-owned and not world-writable
- Must not follow symbolic links for any file it reads, writes, or executes

[RULE]
Privileged scripts MUST include a self-check block that verifies their own ownership and permissions at startup. If the check fails, the script MUST exit immediately with code 3.

[PATTERN]
Self-check pattern for privileged scripts:

```bash
_self_check() {
    local script_path
    script_path=$(readlink -f -- "${BASH_SOURCE[0]}")
    local owner mode
    owner=$(stat -c '%U' -- "${script_path}")
    mode=$(stat -c '%a' -- "${script_path}")

    [[ "${owner}" == "root" ]] || {
        echo "FATAL: script not owned by root (owner: ${owner})" >&2
        exit 3
    }
    [[ "${mode}" == "700" || "${mode}" == "750" ]] || {
        echo "FATAL: unsafe script permissions (mode: ${mode})" >&2
        exit 3
    }
}
_self_check
```

[ANTI-PATTERN]
A privileged script in a world-writable directory is a local privilege escalation regardless of the script's own permissions. The directory permissions are the attack surface.
