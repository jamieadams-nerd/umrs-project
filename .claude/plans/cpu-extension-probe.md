---
name: CPU Extension Probe
path: components/rusty-gadgets/umrs-platform
agent: rust-developer
split-from: umrs-platform-posture-and-cross-platform.md (2026-03-18)
---

# Plan: CPU Extension Probe

**Status:** Approved — grouped with `umrs-tool-init.md` as ready-to-execute implementation work. Prerequisites (kernel-security-posture-probe + cpu-security-corpus-plan) must complete before phase work begins.

## Purpose

Design and implement the CPU extension detection subsystem for `umrs-platform`. This is
Pillar 3 of the platform expansion: a three-layer hardware/OS/software activation model
that determines whether security-relevant CPU extensions are available AND actually used.

**Prerequisites (both must be met before implementation begins):**
1. Kernel security posture probe project complete (all phases — see `kernel-security-posture-probe.md`)
2. CPU security corpus research validated (see `cpu-security-corpus-plan.md`)

**ROADMAP:** G3 (CPU security posture), G6 (crypto assurance)

---

## Display Grouping Reference

When posture signals are presented to operators (TUI, reports, `--json`), organize them
under Jamie's 7-domain Capability Matrix. See `.claude/references/capability-matrix-domains.md`
for the full mapping. Source: `.claude/jamies_brain/kernel-probe-grouping.txt`.

---

## Dual-Audience API

The public interface must serve two audiences:

- **Novice and intermediate programmers** need a single object they can create and then
  query for answers. They just want to know: is SELinux capable? What OS version is this?
  Is this package installed? The `OsDetector::detect()` pattern is exactly right — simple to
  create, simple to query, hides the evidence chain.

- **Experienced programmers and auditors** need the full evidence chain and trust tier
  classification when they need it. The evidence and confidence model must remain accessible
  without complicating the simple path.

The goal is: easy things are easy, hard things are possible. Do not collapse these two
audiences into a single complicated API. The detailed trust checks and evidence chain should
be available but should not impose on callers who only need a basic answer.

Jamie's note: "Things like the OsDetector:: is **GREAT**! Love public facing detectors like
this. Simple for them and keep the detailed, advanced stuff we have for experienced
programmers."

---

## The Three-Layer Activation Model

Having a CPU extension does not mean it is being used. Utilization depends on several
layers of the software stack. In most cases the extension must be explicitly enabled by
the compiler, runtime library, or application code. Only a small subset are transparently
used by the system.

To assess whether a platform actually benefits from an extension, think in terms of three
layers:

**Layer 1 — Hardware Availability (CPU Capability)**

At the lowest layer the processor advertises support through CPUID flags. These appear in
`/proc/cpuinfo`, `cpuid`, and `lscpu`. This only means the silicon supports the
instruction. Nothing is using it yet.

Security-relevant examples: AES-NI, AVX, AVX2, AVX-512, SHA, RDRAND, RDSEED, SGX, BMI1,
BMI2, ADX, VAES, VPCLMULQDQ, SMEP, SMAP, CET-SS, CET-IBT, NX/XD.

Detection path: `/proc/cpuinfo` flags line (safe Rust path, no unsafe required).

**Layer 2 — OS Enablement (Kernel Support)**

Some extensions require the operating system to enable state management. The kernel enables
these through mechanisms like XSAVE, the XCR0 register, and CR4 flags. If the OS does not
enable the feature, software cannot use it even if the CPU supports it.

| Extension  | OS Involvement | Reason                                    |
|------------|----------------|-------------------------------------------|
| AVX / AVX2 | YES            | Kernel must save/restore vector registers |
| AVX-512    | YES            | Large register context                    |
| SGX        | YES            | Enclave management                        |
| PKU        | YES            | Protection key management                 |
| AMX        | YES            | Tile state management                     |

Detection path: `/proc/self/status` (xsave flags), kernel-managed sysfs nodes.

**Layer 3 — Software Utilization (Compiler / Library / Application)**

This is the most important layer. The majority of extensions are only used if software is
compiled to target them. Three common patterns:

- **Compile-time targeting**: compiler generates instructions directly
  (`-C target-cpu=native`, `-mavx2`, `-maes`)
- **Runtime CPU dispatch**: libraries detect CPU features dynamically and select the
  fastest implementation path (OpenSSL, libcrypto, zlib-ng, Rust std crypto backends)
- **Intrinsics / assembly**: code directly calls CPU instruction wrappers
  (`_mm_aesenc_si128()`)

Detection path: `/proc/crypto` (kernel crypto driver registration); ELF binary inspection
(`objdump -d binary | grep aesenc`); `OPENSSL_ia32cap` environment variable.

**The High-Assurance Insight**

For a security evaluation platform like UMRS, the real questions are:
1. Does the CPU support the instruction?
2. Did the kernel enable it?
3. Do the cryptographic libraries actually use it?
4. Was the software compiled to take advantage of it?

Only when all four are true does the platform get the full benefit. A CPU with AES-NI that
runs a binary compiled with `-march=x86-64` gets no AES-NI acceleration.

**Practical mental model:**

```
CPU capability
    ↓
OS enablement
    ↓
compiler support
    ↓
library implementation
    ↓
application usage
```

Failure at any level means the extension is effectively unused.

### Extensions That Are Automatically Used

A small subset of extensions are automatically picked up by standard crypto libraries:
- **AES-NI** — OpenSSL, BoringSSL, libsodium, Rust `ring` all detect and use it
- **SHA extensions** — used automatically if compiled into crypto libraries
- **RDRAND / RDSEED** — used by kernel entropy pools and crypto libraries
- **PCLMULQDQ** — used automatically by AES-GCM implementations

### Extensions That Require Explicit Enablement

These require intentional optimization at the compiler or library level:
AVX-512, AMX, BMI1/BMI2, ADX, SHA512 extensions, VAES. If software was compiled
generically (`-march=x86-64`), none of these will be used.

---

## Empirical Reference Data

Jamie's Ubuntu experiments on octopussy (Ubuntu 24.04.3, ARM64, kernel 6.14.0) empirically
validate the three-layer model. Working scripts are in
`.claude/agent-memory/rust-developer/reference/cpu-crypto/`:

- **`cpu_info.sh`** — Layer 1 + Layer 2 detection: CPU flags, kernel crypto drivers
- **`umrs-openssl-audit.sh`** — Layer 3 detection: OpenSSL version, providers, algorithm
  surface, ARM crypto benchmarks, kernel crypto cross-reference
- **`create_ima_keys.sh`** — IMA/EVM key generation (crypto infrastructure reference)
- **`ima-reresh.sh`** — IMA/EVM re-signing (crypto infrastructure reference)

Key findings:
- ARM CE flags confirmed: aes, pmull, sha1, sha2, sha3, sha512, crc32, atomics, paca, pacg
- Kernel crypto parity: aes-ce, gcm-aes-ce, sha256-ce, sha3-*-ce
- OpenSSL 3.0.13 actively using ARM CE: AES-128-GCM ~8.9 GB/s, SHA-256 ~3.3 GB/s
- Binary hardening (readelf/elfdump): BTI, PAC, RELRO, PIE, NX — first-class probe requirement

**OpenSSL as system-wide trust anchor:** binary analysis must assess whether linked binaries
inherit OpenSSL's hardening properties (ARM CE, branch protection, provider-only mode).
Cross-ref: `docs/modules/cryptography/pages/openssl-no-vendoring.adoc`

---

## Proposed `CpuIndicatorId` Design

**Decided (2026-03-16):** Separate `CpuIndicatorId` enum (not extending `IndicatorId`). Keeps
posture catalog and CPU extension catalog from growing into a single unwieldy type.

### Phase 1E/1F Variants (from corpus research)

| Variant | `/proc/cpuinfo` flag | Layer | Platform | Classification |
|---|---|---|---|---|
| `CpuIndicatorId::Pcid` | `pcid` | 1 | x86_64 | Important |
| `CpuIndicatorId::CetShadowStack` | `shstk` | 1 | x86_64 | Critical/Defensive |
| `CpuIndicatorId::CetIbt` | `ibt` | 1 | x86_64 | Critical/Defensive |
| `CpuIndicatorId::Umip` | `umip` | 1 | x86_64 | Important |
| `CpuIndicatorId::Pku` | `pku` + `ospke` | 1 | x86_64 | Important |
| `CpuIndicatorId::ArmPac` | `paca` | 1 | aarch64 | Important |
| `CpuIndicatorId::ArmBti` | `bti` | 1 | aarch64 | Important |
| `CpuIndicatorId::ArmMte` | `mte`/`mte2`/`mte3` | 1 | aarch64 | Important |

All are `/proc/cpuinfo` reads — `ProcfsText` + `SecureReader::read_generic_text`. No new
routing infrastructure needed.

### Rust CET/PAC Limitation

Stable Rust does not support `-Z cf-protection=full` (rust-lang/rust#93754). UMRS binaries
will NOT have CET or PAC/BTI ELF notes. Audit classification:
- **C/C++ binary** without CET on CET-capable system → **HIGH**
- **Rust binary** without CET → **INFORMATIONAL** (Rust memory safety compensates)

### PCID/Pti Contradiction Logic

- `Pcid` absent + `Pti` active → **CAUTION** (30-50% perf hit, pressure to disable PTI)
- `Pcid` present + `Pti` disabled → **CRITICAL** (Meltdown mitigation off despite hardware fix)

### Layer 4 — Runtime Mitigation State (Vulnerability Sysfs)

`/sys/devices/system/cpu/vulnerabilities/` is a fourth detection layer orthogonal to the
three-layer activation model. It answers "are mitigations actually running?" The existing
`Mitigations` indicator only checks the cmdline umbrella flag.

`VulnerabilityReport` lives in `posture::vulnerability` (sibling to `posture::bootcmdline`),
not inside the CPU extension probe. See Rusty's review:
`.claude/reports/cpu-probe-openssl-plan-review.md`

### Shared `ElfInspector`

Binary hardening analysis (CET notes, NEEDED entries, RELRO/PIE/NX) uses a shared
`umrs-platform::elf_inspect` module backed by `goblin`. Shared between this plan and the
OpenSSL posture module. See Rusty's review for the full type design.

### Reference

The full feature inventory (60 features across 15 categories, 9 detection interfaces,
23-column matrix) is in `.claude/plans/cpu-security-corpus-plan.md`.
Rusty's implementation review: `.claude/reports/cpu-probe-openssl-plan-review.md`

---

## Compliance Citations

| Section | Controls |
|---------|----------|
| CPU Extension Detection | NIST SP 800-53 SC-13 (cryptographic protection), SI-7 (software integrity) |

---

## Pre-Implementation Tasks

- [ ] Generate consolidated detection reference sheet from corpus (all sysfs/procfs paths,
  value formats, flag names — single lookup table for Rusty during implementation)
- [ ] Close plan gaps identified in Rusty's review (`.claude/reports/cpu-probe-openssl-plan-review.md`)

## Definition of Done

- [x] CPU corpus research complete and validated (Phases 0–1H, 60 files, 645 RAG chunks)
- [x] Rusty familiarization and plan review complete
- [ ] `CpuIndicatorId` enum implemented with three-layer detection
- [ ] Layer 1 (hardware), Layer 2 (OS), Layer 3 (software) probes implemented
- [ ] Layer 4 (vulnerability sysfs) — `VulnerabilityReport` in `posture::vulnerability`
- [ ] Shared `ElfInspector` module (`umrs-platform::elf_inspect`) with `goblin`
- [ ] Binary hardening evidence: CET/PAC/BTI notes, RELRO, PIE, NX
- [ ] PCID/Pti contradiction logic in contradiction detector
- [ ] OpenSSL linkage analysis integrated into binary inspection path
- [ ] `cargo xtask clippy && cargo xtask test` clean on all supported platforms

---

## Model Assignments

| Work Item | Agent | Model | Rationale |
|---|---|---|---|
| CPU Extension Detection (design) | rust-developer | **opus** | New signal catalog, three-layer model, CpuIndicatorId design |

---

## DO NOT CHANGE ANY CODE Right Now

This is a planning document. No implementation work begins without an explicit decision from
Jamie. Keep this plan in the queue. Ask questions, record decisions, and update this
document as the work evolves.
