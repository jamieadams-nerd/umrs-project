# Plan: Extended Attribute Sanitization Gap — Proof, Tool, and Documentation

**Status:** Draft — awaiting activation.

**ROADMAP alignment:** G5 (Security Posture), G4 (Tool Ecosystem), G9 (Documentation Quality)

**Source:** `jamies_brain/xattr-stripper.txt` (Jamie Adams, archived 2026-03-19)

---

## Problem

`shred(1)` is widely believed to securely erase files. It does not.

`shred` overwrites the file's **data blocks** as referenced by the inode. It does NOT
touch extended attribute storage. On ext4, xattrs may be stored in:

- The inode itself (small xattrs, inline)
- Separate xattr blocks referenced by inode fields

Neither location is part of the file's data region. When an xattr is removed via
`setfattr -x`, the filesystem marks the storage as free — it does not overwrite it.
The data persists until the blocks are reused.

This means:

1. `shred` + `rm` leaves xattr data recoverable
2. `setfattr -x` leaves xattr data recoverable
3. `user.*` xattrs can contain arbitrary data — they are a covert storage channel
4. MITRE ATT&CK tracks xattr abuse as a hiding technique

On modern storage this gets worse:
- **Journaling** (ext4, xfs): metadata updates including xattrs may be journaled
- **COW** (btrfs, ZFS): old xattr blocks persist in previous tree versions
- **SSDs**: wear leveling prevents deterministic overwrite of any block

NIST SP 800-88 Rev. 2 does not recommend file-level shredding as a sanitization
control for modern storage. The recommended approaches are full-disk encryption +
crypto erase, block device purge, or physical destruction.

**The gap:** Most operators don't know this. Most sanitization guides don't mention
xattrs. `shred --help` doesn't warn about it.

---

## Goals

1. **Prove it** — demonstrate with reproducible evidence that xattr data survives `shred`
2. **Build a tool** — `umrs-xattr-strip` to properly enumerate and scrub xattrs
3. **Document it** — publish the gap for operators, with NIST/CMMC control mappings

---

## Phase 1: Proof of Concept — Capture Evidence

**Executor:** rust-developer (test harness) + security-engineer (analysis)

**Scope:** Create a reproducible test that demonstrates xattr data surviving `shred`.

### Test Script

```
1. Create a test file on ext4
2. Set user.secret = "CLASSIFIED_DATA_12345" via setfattr
3. Record the inode number and xattr block location (debugfs)
4. Run shred -vzn 3 on the file
5. Remove the file (rm)
6. Examine the raw disk blocks where the xattr was stored (debugfs/dd)
7. Search for "CLASSIFIED_DATA_12345" in the raw blocks
8. Record: found / not found, block addresses, filesystem state
```

### Expected Result

The xattr value string is recoverable from the raw disk after `shred` + `rm`.

### Deliverable

- Test script in `components/rusty-gadgets/umrs-xattr-strip/tests/` or a standalone script
- Evidence report in `docs/sage/reviews/` (this is publishable security research)
- Screenshots / hex dumps showing the surviving data

---

## Phase 2: `umrs-xattr-strip` Tool

**Executor:** rust-developer

**Scope:** New binary crate — enumerate, report, and strip extended attributes.

### Modes

| Mode | Behavior |
|---|---|
| `--audit` | Enumerate all xattrs, report by namespace, compute size footprint |
| `--strip user` | Remove all `user.*` xattrs |
| `--strip all` | Remove all xattrs (WARNING: breaks SELinux labels, ACLs, IMA) |
| `--whitelist <file>` | Strip everything except whitelisted xattr names |
| `--manifest` | Generate a before/after manifest (path + namespace + name + size + sha256) |
| `--json` | JSON output for all modes |
| `--recursive` | Operate on directory trees |
| `--dry-run` | Show what would be stripped without doing it |

### Architecture

```
umrs-xattr-strip/
├── Cargo.toml
├── src/
│   ├── main.rs          ← CLI (clap)
│   ├── enumerate.rs     ← xattr listing + classification
│   ├── strip.rs         ← removal logic per namespace
│   ├── manifest.rs      ← before/after manifest generation
│   └── policy.rs        ← whitelist/blacklist evaluation
└── tests/
    └── strip_tests.rs   ← integration tests
```

### Xattr Namespace Policy

| Namespace | Default strip behavior | Risk of stripping |
|---|---|---|
| `user.*` | Strip (safe) | None — application data |
| `security.selinux` | Preserve (unless `--strip all`) | Breaks MAC enforcement |
| `security.capability` | Preserve (unless `--strip all`) | Removes file capabilities |
| `security.ima` | Preserve (unless `--strip all`) | Breaks IMA verification |
| `security.evm` | Preserve (unless `--strip all`) | Breaks EVM verification |
| `system.posix_acl_*` | Preserve (unless `--strip all`) | Removes ACLs |
| `trusted.*` | Preserve (unless `--strip all`) | Root-only; system use |

### Safety

- `--strip all` requires `--force` confirmation (or `--yes` for scripting)
- Display a WARNING before stripping `security.*` — this breaks SELinux/IMA/capabilities
- Never strip xattrs on files you don't own (unless root)
- `#![forbid(unsafe_code)]` — use the `xattr` crate (pure safe Rust API)
- fd-based operations where possible to avoid TOCTOU

### Dependencies

- `xattr` crate (safe Rust API over listxattr/getxattr/removexattr)
- `clap` (CLI)
- `serde_json` (JSON output)
- `sha2` (manifest hashing)
- `umrs-core` (init, logging)

### Controls

- NIST SP 800-53 MP-6 (Media Sanitization)
- NIST SP 800-53 SC-28 (Protection of Information at Rest)
- NIST SP 800-88 Rev. 2 (Media Sanitization Guidelines)
- MITRE ATT&CK T1564.009 (Hide Artifacts: Extended Attributes)
- CWE-212 (Improper Removal of Sensitive Information Before Storage or Transfer)

---

## Phase 3: Documentation — The Shred Gap

**Executor:** tech-writer (with security-engineer review)

**Scope:** Publish the xattr sanitization gap as operator-facing documentation.

### Content

1. **What `shred` actually does** — data blocks only, not metadata
2. **Where xattrs live** — inline inode vs separate xattr blocks (ext4 specifics)
3. **The proof** — reference Phase 1 evidence
4. **What makes it worse** — journaling, COW, SSDs
5. **What NIST SP 800-88 actually recommends** — crypto erase, block purge, destruction
6. **What operators should do instead** — encrypted volumes + key destruction
7. **The `umrs-xattr-strip` tool** — for logical sanitization (release/export workflows)
8. **xattrs as a covert channel** — MITRE ATT&CK reference, detection guidance

### Where It Goes

- `docs/modules/security-concepts/pages/xattr-sanitization-gap.adoc` — the main page
- Blog post candidate for Sage (this is genuinely novel security research)
- Reference from `docs/modules/operations/` if sanitization procedures exist there

### Tone

This is a public service document. Write it for the operator who thinks `shred` solved
their problem. Do not assume they know what an inode is. Explain it, prove it, tell them
what to do instead.

---

## Phase 4: Integration with UMRS Ecosystem

**Deferred — after Phases 1-3 complete.**

- `umrs-xattr-strip --manifest` output feeds into chain-of-custody records
- Vault lifecycle transitions include mandatory xattr posture check
- `umrs-ls` shows xattr presence as a security finding (already partially done via `SecureDirent`)
- Posture assessment: "unexpected user.* xattrs under sensitive roots" as an indicator

---

## Hard Constraints

- No FFI — use the `xattr` crate, not raw `libc` calls
- No `--strip all` without explicit confirmation
- Manifest generation is mandatory for audit trail — stripping without evidence violates MP-6
- Phase 1 proof must be reproducible — document exact filesystem, kernel version, test steps
- Do not strip `security.*` xattrs in any default mode — only when explicitly requested

---

## Source Material

Derived from Jamie's research notes:
- `jamies_brain/xattr-stripper.txt` (archived 2026-03-19)

Key references:
- NIST SP 800-88 Rev. 2 (September 2025) — Media Sanitization Guidelines
- NIST SP 800-53 MP-6 — Media Sanitization control
- MITRE ATT&CK T1564.009 — Hide Artifacts: Extended Attributes
- ext4 documentation — xattr storage architecture
