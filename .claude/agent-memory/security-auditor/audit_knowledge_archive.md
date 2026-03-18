# Security Auditor — Audit Knowledge Archive

Moved from MEMORY.md 2026-03-18 during housekeeping. Reference material not needed
in every session's active context.

## CPU Feature Matrix — Detailed Audit Principles (2026-03-14)

Report: `.claude/reports/cpu-matrix-review/security-auditor-review.md`

### Two-Layer Model: Hardware Capability vs. Software Utilization
A CPU feature being present is necessary but not sufficient for a security posture claim.
The audit must verify BOTH layers:
1. CPU reports the feature (CPUID, /proc/cpuinfo)
2. Software actually uses it (/proc/crypto, openssl probing, ELF binary headers)

Key example: AES-NI present in CPU + OpenSSL compiled without AES-NI support = HIGH finding
on a FIPS system.

### /proc/crypto as Primary Detection Interface
Shows kernel-registered algorithm implementations and whether hardware-backed.
`fips_allowed` field and `selftest: passed` are FIPS-relevant.
More authoritative than /proc/cpuinfo for actual hardware acceleration use.

### NIST SP 800-90B Required for RDRAND/RDSEED Classification
Cannot classify CPU entropy features without consulting SP 800-90B.
RDRAND is Critical on FIPS systems.

### Missing Category: Defensive CPU Controls
Speculative-execution mitigations (IBRS, IBPB, STIBP, SSBD, MDS/MD_CLEAR)
and CPU-enforced access controls (SMEP, SMAP, CET, UMIP, NX/XD).
Primary detection: `/sys/devices/system/cpu/vulnerabilities/`.

### CET Binary Verification
CET requires CPU + kernel + per-binary ELF opt-in (-fcf-protection=full).
Verify via `.note.gnu.property` section (`eu-readelf -n` or `objdump -p`).

## SEC Pattern — Detailed Observations (2026-03-11)

- Pattern spec requires process start time from /proc/self/stat; implementation
  uses wall-clock subsecond nanos. Code wins.
- FIPS gate: now correctly fails closed (resolved).
- decode_cached_result re-runs pipeline on every hit — tamper detection only.
- Cite SC-13 in Cargo.toml hmac dep comment.

## RMF Lifecycle Mapping for UMRS Plan Components

- Signal catalog / baseline definition → Select S-2, S-3
- Runtime collection mechanism → Monitor M-2 (ongoing assessments)
- Contradiction detection output → Monitor M-3 (ongoing remediation)
- Control implementation (FIPS gate, CET, etc.) → Implement I-2
- Open architectural decisions → Authorize R-1
- Research corpus → Select S-2 pre-requisite

## "Other Than Satisfied" Controls Across Current Plans (2026-03-15)

- CA-7: monitoring frequency ODP undefined (posture probe)
- CM-6(iii): no persistent deviation document produced (posture probe)
- SC-13: Layer 2/3 utilization assessment procedure absent (CPU corpus)
- SI-7: Ubuntu dpkg path has no Test coverage (umbrella)
- SC-28: DetectionResult serialization deferred (umbrella)
- CM-8: inventory persistence mechanism undefined for OS Detection (umbrella)

## Confirmed Resolved Items (2026-03-14)

- `evidence.rs` `records` field now `pub(crate)` private — AU-10 enforced ✓
- SEC FIPS gate now correctly fails closed ✓
- `decode_cached_result` re-runs pipeline on cache hit — acknowledged design note
