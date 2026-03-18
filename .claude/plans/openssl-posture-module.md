# Plan: OpenSSL Posture Module

**Status:** draft
**ROADMAP:** G3 (CPU security posture), G6 (crypto assurance)
**Crate:** `umrs-platform`
**Agent:** rust-developer
**Depends on:** nothing — ships independently ahead of CPU extension probe
**Feeds into:** CPU extension probe (Layer 3), binary analysis tooling, TUI crypto tab, `--json` reports

---

## Why This Matters

OpenSSL is the system-wide cryptographic trust anchor. Every binary that links against it
inherits — or fails to inherit — its hardening properties: hardware acceleration, provider
model, build-time protections, algorithm surface.

Today `umrs-platform` can say "OpenSSL is installed" (package presence). It cannot say:

- Is it provider-only or polluted with legacy/engine paths?
- Is FIPS provider loaded?
- Are hardware crypto extensions actually being used?
- Was it compiled with branch protection?
- Do my binaries link against system libssl, or did something vendor its own?

That's the difference between **inventory** and **assurance**. This module closes the gap.

Jamie's audit script (`umrs-openssl-audit.sh`) proves all of this is detectable from
userspace. This plan translates that shell script into typed, evidence-backed Rust.

---

## Detection Layers

### Layer A — OpenSSL Identity & Build Posture

Source: `openssl version -a` (or equivalent library query)

| Field | Example | Why it matters |
|-------|---------|----------------|
| Version | 3.0.13 | Known CVE surface, provider support |
| Build date | 2024-01-30 | Staleness indicator |
| Platform | debian-arm64 | Architecture match |
| Compiler flags | `-mbranch-protection=standard` | Binary hardening proof |
| OPENSSL_armcap / OPENSSL_ia32cap | `0xfd` | Hardware crypto enablement bitmask |
| OPENSSLDIR | `/usr/lib/ssl` | Config root for provider/policy paths |

Detection path: parse `openssl version -a` output, or read the same data from
`libcrypto.so` symbol queries if we want to avoid subprocess.

### Layer B — Provider Model Status

Source: `openssl list -providers` (or provider directory scan)

| State | Meaning | Assurance impact |
|-------|---------|------------------|
| default only | Modern, clean baseline | Good — no legacy contamination |
| default + fips | FIPS boundary active | Required for FIPS compliance |
| default + legacy | Legacy algorithms available | Flag — potential compliance gap |
| engine active | Old engine model in use | Warning — opaque crypto path |

This is the single most important crypto posture signal. Provider-only mode means all crypto
paths are visible and auditable. Engine mode means there's a black box.

### Layer C — Algorithm Surface Inventory

Source: `openssl list -cipher-algorithms`, `-digest-algorithms`, `-mac-algorithms`,
`-public-key-algorithms`, `-signature-algorithms`

Not a full enumeration in the type — but a **policy check**: are deprecated algorithms
present (DES, RC4, MD5, SHA-1 for signing)? Are required algorithms present (AES-256-GCM,
SHA-256, SHA-3, ED25519)?

### Layer D — Hardware Acceleration Evidence

Source: `/proc/crypto` cross-referenced with CPU flags from `/proc/cpuinfo`

| Check | What it proves |
|-------|----------------|
| Kernel crypto drivers with `-ce` suffix | OS has enabled hardware crypto path |
| OpenSSL armcap/ia32cap bitmask | OpenSSL knows about the hardware |
| Performance plausibility | GB/s throughput proves hardware, not software fallback |

Note: we may defer the benchmark/plausibility check to a separate "deep audit" mode.
The flag + driver presence check is sufficient for posture.

### Layer E — Binary Linkage Analysis

Source: `readelf -d <binary>` or ELF parsing

| Check | What it proves |
|-------|----------------|
| NEEDED libssl.so.3 / libcrypto.so.3 | Binary uses system OpenSSL |
| No vendored openssl-sys build artifacts | Build did not vendor |
| RELRO, PIE, NX flags | Binary hardening |
| BTI, PAC (ARM) | Branch protection at binary level |

This is the "trust chain" check: CPU has the extension → kernel enabled it → OpenSSL uses
it → **this binary links against that OpenSSL**. Without Layer E, the chain is incomplete.

---

## Proposed Types

```
OpenSslPosture
├── version: OpenSslVersion        (semver + build date + platform)
├── build_flags: BuildFlags         (branch protection, fortify, etc.)
├── providers: Vec<ProviderStatus>  (name, active, version)
├── provider_model: ProviderModel   (ProviderOnly | LegacyLoaded | EngineActive)
├── hw_acceleration: HwAccelStatus  (detected flags, kernel crypto parity)
├── algorithm_policy: AlgorithmPolicyCheck  (required present, deprecated absent)
└── evidence: EvidenceBundle        (provenance trail for all of the above)

BinaryLinkageReport
├── path: PathBuf
├── openssl_linkage: LinkageStatus  (SystemDynamic | Vendored | None | Unknown)
├── hardening: BinaryHardening      (relro, pie, nx, bti, pac)
└── evidence: EvidenceBundle
```

### Design Principles

- **Validate at construction** — `OpenSslPosture::detect()` returns `Result`, proven valid on success
- **Evidence-backed** — every field has provenance in the `EvidenceBundle`
- **Fail-closed** — if we can't determine provider model, it's `Unknown`, not assumed good
- **Dual-audience API** — simple `is_provider_only()`, `is_fips_active()`, `has_hw_accel()`
  accessors for novice callers; full evidence chain for auditors

---

## Implementation Phases

### Phase 1 — OpenSSL Identity (Layer A + B)

- Read OpenSSL version, build date, platform, and compiler flags from system libcrypto
  (parse `/usr/lib64/libcrypto.so` ELF `.rodata` or read OPENSSLDIR config files)
- Scan provider directory for loaded providers → `ProviderModel`
- Wire as standalone `OpenSslPosture::detect()`
- Tests: version parsing, provider classification, build flag extraction
- **No subprocess calls** — all detection via file reads and ELF parsing

### Phase 2 — Algorithm Policy Check (Layer C)

- Define required/deprecated algorithm sets as associated constants
- Read OpenSSL config and provider manifests to enumerate available algorithms
- Produce `AlgorithmPolicyCheck` with pass/fail + findings

### Phase 3 — Hardware Acceleration Cross-Reference (Layer D)

- Cross-reference `/proc/crypto` kernel drivers with `/proc/cpuinfo` flags
- Parse `OPENSSL_armcap` (ARM) / `OPENSSL_ia32cap` (x86) bitmask from libcrypto
- Produce `HwAccelStatus` indicating parity between hardware, kernel, and OpenSSL

### Phase 4 — Binary Linkage Analysis (Layer E)

- Pure Rust ELF parsing via `goblin` crate (approved — see Dependency Assessment)
- ELF NEEDED entries → `LinkageStatus` (system libssl/libcrypto vs vendored vs none)
- Hardening flag extraction: RELRO, PIE, NX, BTI, PAC
- `BinaryLinkageReport` for any given binary path
- **No subprocess calls** — no `readelf`, no `ldd`, no `objdump`

### Phase 5 — Integration

- Wire `OpenSslPosture` into the posture snapshot pipeline
- Add to TUI crypto tab (when TUI reaches that phase)
- `--json` output for all fields
- Binary linkage as a query: "show me all binaries in /usr/bin that vendor OpenSSL"

---

## Dependency Assessment

**Phase 1–3:** No new crate dependencies. All detection via procfs/sysfs parsing
(existing `umrs-platform` patterns), OpenSSL config file reads, and ELF `.rodata` parsing
via `goblin`. No subprocess calls.

**Phase 4 — `goblin` crate (APPROVED by Jamie):**
- Pure Rust, zero unsafe, no FFI, no C dependencies
- Parses ELF, Mach-O, PE headers directly from bytes
- Mature and widely used: `cargo`, `miri`, `object` crate, `symbolic`
- ~15k lines, well-maintained, acceptable supply chain risk
- Aligns with project principles: pure Rust, no unsafe, no external tool dependencies

**Hard rule — no subprocess calls:** This module will NEVER shell out to `openssl`,
`readelf`, `ldd`, `objdump`, or any other external command. All detection is pure Rust
file reads and binary parsing. This is a design constraint, not a preference.

---

## Compliance Citations

| Layer | Controls |
|-------|----------|
| OpenSSL identity | NIST SP 800-53 CM-8 (component inventory), SA-12 (supply chain) |
| Provider model | NIST SP 800-53 SC-13 (cryptographic protection), SC-12 (key management) |
| Algorithm policy | NIST SP 800-53 SC-13, FIPS 140-3 |
| Hardware acceleration | NIST SP 800-53 SC-13, SI-7 (software integrity) |
| Binary linkage | NIST SP 800-53 SI-7, SA-12, CM-14 (signed components) |
| Binary hardening | NIST SP 800-53 SI-7, SA-15 (development process) |

---

## Relationship to Other Plans

- **CPU extension probe** (`.claude/plans/cpu-extension-probe.md`): Layers D and E feed
  directly into the three-layer activation model. This module can ship first and be
  absorbed into the probe later.
- **Kernel posture probe** (`.claude/plans/kernel-security-posture-probe.md`): `/proc/crypto`
  parsing may share infrastructure with existing kattrs patterns.
- **openssl-no-vendoring.adoc**: Phase 4 automates what that doc describes manually.
  Once implemented, the doc should reference the tool.
- **TUI subsystem inventory**: OpenSSL version/provider/accel status is a prime candidate
  for the subsystem inventory tab.

---

## Reference

- Jamie's audit script: `.claude/agent-memory/rust-developer/reference/cpu-crypto/umrs-openssl-audit.sh`
- Jamie's empirical data: `.claude/jamies_brain/archive/cpu-experiments/cpuaudit_report.txt`
- Documentation: `docs/modules/cryptography/pages/openssl-no-vendoring.adoc`
- Ubuntu crypto baseline: project memory `project_ubuntu_crypto_baseline.md`

---

## DO NOT CHANGE ANY CODE Right Now

This is a planning document. No implementation work begins without an explicit decision from
Jamie.
