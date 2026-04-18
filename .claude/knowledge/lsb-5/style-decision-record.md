# LSB 5.0 — Style Decision Record

**Familiarized:** 2026-04-18

---

## SDR-LSB-001: Citation Strength — Never Cite LSB §23 Alone

**Tension:** LSB §23.3 uses "should" (not "shall") for UID range recommendations.
Citing LSB alone in documentation or audit artifacts leaves the compliance claim weak.

**Ruling:** All UMRS compliance claims about UID/GID allocation MUST cite all three
sources in sequence:
1. LSB 5.0 §23.3 (formal standard body, provides normative authority)
2. systemd UIDS-GIDS (de facto standard for systemd distros, extends LSB to 999)
3. RHEL 10 `/etc/login.defs` values (implementation evidence)

This is already the practice in `2026-04-17-uid-gid-compliance-reference.md`.
All future code comments, doc pages, and audit responses must follow this pattern.

---

## SDR-LSB-002: Legacy Accounts — Prohibit Use of `bin` and `daemon`

**Tension:** LSB §23.2 Table 23-1 marks `bin` and `daemon` as "Legacy User ID/Group ID"
with explicit notes that new applications should not use them.

**Ruling:** UMRS MUST NOT use `bin` or `daemon` as the running identity for any component.
UMRS correctly uses the dedicated `umrs` account. This is the right pattern.
Any future component (e.g., a new daemon) should receive its own dedicated service account.

---

## SDR-LSB-003: Prohibited Username Namespace

**Tension:** LSB §23.2 Tables 23-1 and 23-2 enumerate names that are either required or
conventionally reserved. UMRS must not use these names.

**Ruling (and confirmed safe list):** The `umrs` account name does not appear in either
table. The following names are off-limits for UMRS accounts:
`root`, `bin`, `daemon`, `adm`, `lp`, `sync`, `shutdown`, `halt`, `mail`, `news`,
`uucp`, `operator`, `man`, `nobody`

---

## SDR-LSB-004: POSIX API Requirement

**Tension:** LSB §23 requires that programs use the provided API to read user/group
databases, not parse `/etc/passwd` or `/etc/group` directly.

**Ruling:** Any UMRS Rust code that needs to resolve user/group identity must call
POSIX API (via `nix` crate or `libc` bindings: `getpwnam()`, `getpwuid()`, `getgrnam()`).
Parsing `/etc/passwd` directly is prohibited.

**Remediation owner:** coder — enforce at code review.

---

## SDR-LSB-005: UID Allocation for Future Service Accounts

**Ruling:** Any additional UMRS service accounts must be allocated with `useradd -r`,
which on RHEL 10 allocates from SYS_UID_MIN–SYS_UID_MAX (201–999). This satisfies
LSB §23.3 (within 100–499 subset) and the full three-source compliance chain.

If a specific static UID is ever needed (e.g., for NFS consistency), document the choice
explicitly, avoid the 0–200 range (distro-reserved on RHEL 10), and verify the name does
not conflict with Tables 23-1 or 23-2.
