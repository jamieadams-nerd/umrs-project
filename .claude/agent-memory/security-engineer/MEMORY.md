# Security Engineer — Persistent Memory

## SELinux Type Naming Conventions (umrs-platform)
- No dedicated SELinux types written yet for the OS detection subsystem.
- First policy artifact authoring: defer until coder closes F-01/F-02/F-03.

## Established DAC/MAC Patterns

### ProcfsText / SysfsText — Mandatory for /proc and /sys reads
Every read from `/proc/` or `/sys/` MUST use `ProcfsText` or `SysfsText` +
`SecureReader::read_generic_text`. Raw `File::open` on these paths is PROHIBITED.
This is enforced consistently in the OS detection subsystem. See `umrs-platform/src/kattrs/`.

### opened_by_fd Flag Discipline
`EvidenceRecord::opened_by_fd` must be `true` ONLY when the file was actually
opened via an fd-anchored call (ProcfsText/SysfsText engine or explicit
`openat2` + re-verified `fstat`). `File::open(path)` = `opened_by_fd: false`.
Violation found in `integrity_check.rs` — F-02 in 2026-03-11 audit.

## Recurring Gap Patterns Across Components

### Pattern: FIPS gate missing before non-validated cryptographic primitive
When any code asserts a trust/integrity claim backed by a RustCrypto primitive
(`sha2`, `sha3`, etc.), there must be a runtime FIPS gate reading
`/proc/sys/kernel/fips_enabled` via `ProcfsText` before the claim is made.
If FIPS is active, the phase must downgrade and emit `log::warn!`.
First observed: `integrity_check.rs` T4 assertion. Flag in every future review.

### Pattern: Stub that returns parse_ok=true with can_query_ownership=false
Stubs that partially implement `PackageProbe` must emit `log::warn!` when
selected for T3 assertion with capability flags false. Without the warning,
operators see T3 in logs with no indication it was presence-only evidence.
First observed: `rpm.rs` + `dpkg.rs` stubs. Flag in every future review.

### Pattern: Path::exists() used for security-relevant presence checks
`Path::exists()` is a TOCTOU check-then-use. For package DB or security-file
presence, require at minimum a `statfs` magic check after existence confirmation.
Record `fs_magic` in the EvidenceRecord. Flag everywhere this appears.

### Pattern: Second path-based file open after statx
A file selected via `statx` in one phase and re-opened by path in a later
phase is a TOCTOU window. The correct pattern is:
  open → fstat → compare (dev,ino) → read from the same fd.
First observed: `release_parse.rs` `read_candidate()` re-opening by path.

## Build/Packaging Integration Notes
- No RPM spec or semodule integration written for OS detection subsystem yet.
- OS detection produces no installed files; SELinux policy needed when binaries
  using `OsDetector::detect()` are packaged and assigned types.

## Audit Reports Produced
- `2026-03-11-os-detection-umrs-platform.md` — 8 findings (2H, 3M, 3L)
  Key issues: FIPS gate missing (F-01), false opened_by_fd (F-02),
  release_parse TOCTOU (F-03), stub T3 warning (F-04), Path::exists TOCTOU (F-05).
