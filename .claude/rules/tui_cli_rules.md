## TUI/CLI Design Principles

UMRS targets security operators in high-stakes environments. The interface must communicate trust.

- Honor `NO_COLOR` environment variable unconditionally.
- State changes must be explicitly communicated to the user.
- Provide `--json` output mode for all commands that return structured data.
- Default output must be operator-readable without log-level labels or debug noise.
- Verbose mode (`--verbose` / `-v`) is for developer-facing output.
- Error messages must describe what happened and what the operator should do next.
