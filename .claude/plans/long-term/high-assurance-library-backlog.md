# Plan: High-Assurance Library Backlog

**Status:** Backburner — pick items from this list when ready. Not scheduled.

**ROADMAP alignment:** G2 (Platform Library), G4 (Tool Ecosystem), G5 (Security Posture)

**Source:** `jamies_brain/other-possible-features.txt` (Jamie Adams, archived 2026-03-19)

---

## Purpose

This is the menu, not the meal plan. Each item below is a high-assurance library that
addresses a domain where existing Rust crates are written for convenience, not assurance.
These are the gaps that security tooling falls into — symlink races, TOCTOU, implicit
trust, string-shaped data, silent fallbacks.

Every library in this backlog shares the same design DNA:

1. Explicit trust boundaries
2. Provenance tracking
3. Typed data instead of strings
4. No implicit fallbacks
5. Deterministic behavior
6. Auditable APIs

Pick items when they become prerequisites for active work, or when a gap surfaces during
implementation. Do not start these speculatively.

---

## Domain A: Filesystem Trust

These libraries address the fact that most filesystem operations trust the filesystem
implicitly — a catastrophic assumption when the filesystem is attacker-influenced.

### A1. Secure Filesystem Enumeration

**Problem:** Directory walkers (`walkdir`, `glob`) are not written for adversarial filesystems.
Symlink traversal, TOCTOU races, and mount boundary violations are all possible.

**What a high-assurance walker guarantees:**
- No symlink traversal unless explicitly allowed
- No race conditions — file descriptor–based traversal (`openat`, `O_PATH`)
- Mount boundary enforcement (don't cross into different filesystems)
- Inode/device consistency checks

**API shape:**
```
safe_walk(root_fd) → Iterator<SecureDirEntry>
safe_stat(fd) → TrustedStat
safe_open_child(dir_fd, name) → Result<OwnedFd>
```

**Why it matters:** Security tools that scan attacker-controlled directories become
vulnerable themselves. `umrs-ls` already has `SecureDirent` — this generalizes it.

**UMRS overlap:** `umrs-selinux::SecureDirent`, `SecureReader`. This would be the
framework-level generalization.

**Estimated complexity:** HIGH — TOCTOU safety requires fd-anchored everything.

### A2. Trusted Path Resolution

**Problem:** Most path handling is unsafe — symlink traversal, race conditions,
normalization attacks. `canonicalize()` is a TOCTOU gap.

**What a high-assurance resolver does:**
- Resolves path components using file descriptors at each step
- Verifies each step (ownership, permissions, device consistency)
- Rejects unsafe transitions (symlink to different owner, world-writable intermediate)
- OpenBSD `unveil`-style thinking — declare what's allowed, reject everything else

**UMRS overlap:** `validate_safe_path()` in the tool-init plan does this for env vars.
This library would generalize it for any path resolution.

**Estimated complexity:** HIGH — the symlink chain walk is O(depth) and every step
needs stat verification.

### A3. Secure Temporary File Library

**Problem:** Temp file creation is historically dangerous — predictable names, race
conditions, unsafe permissions.

**What a high-assurance temp library guarantees:**
- `O_TMPFILE` usage (nameless until linked — no race window)
- Secure fallback when `O_TMPFILE` not available
- Atomic rename into final location
- Restricted permissions (0600, owned by current UID)
- No predictable names, no reuse

**Estimated complexity:** MEDIUM — the `O_TMPFILE` path is clean; the fallback is where dragons live.

### A4. Secure Archive Extraction

**Problem:** Archive parsing is historically dangerous — path traversal (`../`), symlink
injection, device file creation, permission escalation.

**What a high-assurance extractor guarantees:**
- Safe extraction root (all paths resolve under it)
- No special files (device nodes, sockets, FIFOs rejected)
- No ownership changes (extracted files owned by current UID)
- No permission escalation (no setuid/setgid bits)
- No symlinks pointing outside extraction root

**UMRS overlap:** Future vault/custody intake pipeline.

**Estimated complexity:** HIGH — every archive format has its own quirks.

---

## Domain B: Process & Privilege Trust

These libraries address the gap between what userland *thinks* the privilege state is
and what the kernel actually enforces.

### B1. Capability / Privilege Inspection Library

**Problem:** Linux capabilities are extremely poorly surfaced in userland. Many systems
incorrectly assume `uid == 0` means "privileged" when capabilities may be present on
non-root processes or absent from root.

**What the library exposes:**
- Process capabilities (effective, permitted, inheritable, ambient, bounding)
- File capabilities (via xattrs)
- Securebits
- No-new-privs flag
- AT_SECURE detection (setuid/setgid/capability-elevated)

**Sources:** `/proc/self/status`, xattr reads, `prctl` (via `rustix`)

**API shape:**
```
ProcessCapabilities::current() → CapabilitySet
FileCapabilities::read(fd) → CapabilitySet
is_secure_execution() → bool
```

**UMRS overlap:** Posture assessment — "does this process have capabilities it shouldn't?"
Also feeds into the env scrub `detect_secure_execution()` future item.

**Estimated complexity:** MEDIUM — parsing is well-defined; the edge cases are in
inherited vs ambient vs bounding set interactions.

### B2. Secure Process Execution Library

**Problem:** Most process spawning (`system()`, `popen()`, even `Command::new()`) doesn't
enforce security constraints.

**What a high-assurance wrapper enforces:**
- Absolute path binaries only (no PATH search)
- Environment sanitization (uses `SanitizedEnv` from tool-init)
- Argument validation (no injection via shell metacharacters)
- Optional seccomp filter application
- Optional capability dropping before exec

**API shape:**
```
SecureExecBuilder::new("/usr/bin/restorecon")
    .env(sanitized_env)
    .arg("-R")
    .arg(path)
    .drop_capabilities()
    .seccomp(profile)
    .spawn()
```

**UMRS overlap:** UMRS avoids subprocess spawning by design, but when unavoidable
(e.g., `restorecon`), this is the safe path.

**Estimated complexity:** HIGH — seccomp + capability dropping + environment control
all interacting.

---

## Domain C: System Topology & Identity

These libraries answer: "What system am I running on, and can I trust what it tells me?"

### C1. Mount Topology Inspection Library

**Problem:** Mount tables are complex, layered, and can be manipulated by containers
and namespaces.

**What the library provides:**
- Mount table parsing (`/proc/self/mountinfo`)
- Overlay filesystem detection
- Container detection (cgroup, namespace indicators)
- Mount propagation flag inspection
- Read-only enforcement verification
- Filesystem type identification

**UMRS overlap:** Trust ladder — "is this a real root filesystem or an overlay?"
Container awareness for posture assessment.

**Estimated complexity:** MEDIUM — parsing is well-defined; the semantics of propagation
flags and overlay stacking are where it gets tricky.

### C2. System Identity / Host Fingerprint Library

**Problem:** Many systems want to answer "what host am I?" but the data sources are
messy, inconsistent, and spoofable.

**Evidence sources (ranked by trust):**
- TPM endorsement key (hardware-bound, highest trust)
- DMI/SMBIOS product UUID (`/sys/class/dmi/id/product_uuid`)
- `/etc/machine-id` (systemd-generated, persistent)
- Network MAC addresses (spoofable)
- Hostname (trivially spoofable)

**What the library does:**
- Gathers identity evidence from all sources
- Ranks by trust level
- Detects inconsistencies (spoofing indicators)
- Produces a composite fingerprint with provenance

**UMRS overlap:** Fits directly into the trust ladder architecture.

**Estimated complexity:** MEDIUM — collection is straightforward; trust ranking and
spoofing detection require threat modeling per source.

---

## Domain D: Configuration & Policy Trust

These libraries address the gap between "I read a config file" and "I can trust
what I read."

### D1. Secure Configuration Loader

**Problem:** Configuration parsing is one of the largest attack surfaces — unsafe file
loading, environment variable injection, partial parsing, inconsistent defaults.

**What a high-assurance config loader provides:**
- Deterministic schema validation (typed, not string-based)
- Strict type enforcement (no coercion)
- Cryptographic config signing (optional)
- Immutable runtime config (once loaded, frozen)
- No environment variable interpolation unless explicitly enabled
- Provenance: which file, what hash, when loaded

**API shape:**
```
SignedConfig<T>::load(path) → Result<VerifiedConfig<T>>
ConfigSource { path, hash, timestamp, signature }
```

**UMRS overlap:** Signed configuration concept already started. This generalizes it.

**Estimated complexity:** HIGH — schema validation + signing + provenance tracking.

### D2. Policy Evaluation Library

**Problem:** UMRS will eventually need to evaluate compound policy rules against
collected evidence.

**What the library provides:**
```
IF kernel.lockdown == enabled
AND selinux.mode == enforcing
AND fips.enabled == true
THEN posture = hardened
```

**API shape:**
```
PolicyRule { conditions: Vec<Condition>, outcome: PostureLevel }
EvidenceSet { indicators: HashMap<SignalId, Value> }
PolicyEvaluator::evaluate(rules, evidence) → PostureResult
```

**UMRS overlap:** This IS the posture engine. Currently implicit in the assessment
logic — this would make it explicit, declarative, and auditable.

**Estimated complexity:** HIGH — the evaluation engine is simple; the rule language
design and evidence binding are where the real work lives.

---

## Domain E: Security Observability

These libraries produce structured, auditable output from security-relevant operations.

### E1. Structured Security Event Library

**Problem:** Security events are typically log strings, not structured data. You can
read them but you can't query, filter, or correlate them programmatically.

**What the library provides:**
- Typed event structure: EventID, Severity, Category, Actor, Object, Outcome
- Structured journald emission (key=value fields, not just message text)
- Event correlation (request ID, session ID, boot ID)
- Machine-parseable + human-readable in one emission

**UMRS overlap:** The Security Findings as Data rule already mandates this pattern.
This library would be the implementation.

**Estimated complexity:** MEDIUM — the type system is the easy part; journald
structured field encoding and correlation are the work.

### E2. Kernel State Reader Framework

**Problem:** Reading kernel state from `/proc`, `/sys`, and securityfs requires
provenance verification, TOCTOU safety, and typed parsing — every time.

**What the framework standardizes:**
```
KernelBool::read("/proc/sys/kernel/dmesg_restrict") → Result<bool>
KernelInt::read("/proc/sys/kernel/randomize_va_space") → Result<u32>
KernelEnum::read("/sys/kernel/security/lockdown") → Result<LockdownMode>
```

Each read carries provenance metadata (source path, read timestamp, device/inode).

**UMRS overlap:** `umrs-platform` already does this for specific kernel nodes.
This would generalize it into a reusable framework with the `StaticSource` pattern.

**Estimated complexity:** MEDIUM — the pattern exists; generalizing it cleanly
without losing type safety is the challenge.

---

## Domain F: Cryptographic Operations Trust

### F1. Cryptographic Key Material Handling

**Problem:** Even good crypto libraries don't handle operational hygiene — key material
sits in pageable memory, isn't zeroized on drop, and file permissions are unchecked.

**What a high-assurance wrapper enforces:**
- Secure memory (mlock, madvise MADV_DONTDUMP)
- Zeroization on drop (`zeroize` crate)
- Restricted file permissions on key files (0600, owned by current UID)
- Safe key loading (fd-based, not path-based)
- Hardware key integration (PKCS#11, TPM)

**UMRS overlap:** Future cryptographic module work (G3 on ROADMAP).

**Estimated complexity:** HIGH — `mlock` requires privileges or rlimit adjustment;
PKCS#11 integration is a whole project.

### F2. Evidence / Provenance Library

**Problem:** High-assurance systems need to prove: who collected this data, when, from
what source, and has it been tampered with?

**What the library provides:**
```
EvidenceRecord { data, source, collector, timestamp, hash }
Receipt { evidence_hash, verifier, verification_time }
SignatureChain { records: Vec<SignedEvidence> }
```

**UMRS overlap:** Vault / chain-of-custody concept. Also applicable to posture
assessment evidence — "this posture score is based on these verified readings."

**Estimated complexity:** VERY HIGH — the signature chain and tamper detection
require careful cryptographic design.

---

## Prioritization Guidance

The original author's top 5 (unchanged):

1. **Secure filesystem walker** (A1) — covers most security tooling blind spots
2. **Trusted path resolver** (A2) — foundation for everything that touches the filesystem
3. **Mount topology inspector** (C1) — container awareness, trust ladder
4. **Capability inspection** (B1) — privilege truth, not privilege assumption
5. **Secure process execution** (B2) — when you must fork, do it right

**Items that overlap with active UMRS work:**
- E2 (Kernel State Reader) — `umrs-platform` is already building this
- D2 (Policy Evaluation) — posture assessment is heading here
- E1 (Structured Security Events) — Security Findings as Data rule demands this

**Items to pick when vault/custody work begins:**
- A4 (Secure Archive Extraction)
- F2 (Evidence / Provenance)
- D1 (Secure Configuration Loader)

**Items completed or in progress:**
- #4 (Trusted Environment Inspection) → `umrs-tool-init.md` plan — in progress

---

## The 12 High-Assurance Primitives (Noted for Future Research)

The source material mentions ~12 recurring primitives across seL4, OpenBSD, Qubes OS,
and NSA cross-domain guards. These have not been enumerated yet. When the researcher
agent is activated, this would be a valuable corpus acquisition target — identifying
these primitives and mapping them to UMRS's library architecture.

---

## Source Material

Derived from Jamie's research notes:
- `jamies_brain/other-possible-features.txt` (archived 2026-03-19)
