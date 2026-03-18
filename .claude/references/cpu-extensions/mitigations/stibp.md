# STIBP (Single Thread Indirect Branch Predictors)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | STIBP (Single Thread Indirect Branch Predictors) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | **Intel/AMD:** CPUID EAX=07H, ECX=0, EDX bit 27 (STIBP). Controlled via IA32_SPEC_CTRL MSR (0x48) bit 1. **Note:** On Intel eIBRS systems, STIBP is implicitly enabled and does not need explicit activation. On AMD AutoIBRS and legacy IBRS systems, STIBP must be explicitly enabled. |
| 4 | Linux `/proc/cpuinfo` flag | `stibp` |
| 5 | Key instructions | No dedicated instruction. STIBP is controlled via MSR write: IA32_SPEC_CTRL[1] = STIBP enable. WRMSR to IA32_SPEC_CTRL. |
| 6 | Introduced | **Intel:** Via microcode update on Skylake+ (2018). **AMD:** Via microcode update on Zen family (2018). Hardware native on later steppings. |
| 7 | Security relevance | STIBP prevents a logical processor's indirect branch predictions from being influenced by its SMT sibling thread. Without STIBP on an SMT system, a malicious thread on one sibling can poison the shared branch target buffer and influence speculative execution on the other sibling. This is a cross-thread Spectre v2 attack that bypasses IBPB (which only protects across context switches, not concurrent sibling threads). |
| 8 | Performance benefit | None -- STIBP is a security mechanism with performance cost. Enabling STIBP restricts the branch predictor's ability to use shared prediction state, which can reduce prediction accuracy and degrade performance. Impact varies: typically 5-30% on branch-heavy workloads. |
| 9 | Known vulnerabilities | STIBP addresses a specific attack vector (SMT sibling thread branch predictor sharing). It does not protect against: BHI (Branch History Injection) which can bypass eIBRS including its implicit STIBP, cross-thread MDS attacks (which leak data through shared buffers, not branch prediction), or L1TF cross-thread attacks. STIBP is one layer; full SMT protection requires MDS mitigations + L1D flush + STIBP together. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SI-16 (Memory Protection); CMMC SC.L2-3.13.10; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** (on SMT systems) |
| 12 | Classification rationale | On SMT systems, without STIBP, a co-resident thread can influence speculative execution of its sibling through the shared branch predictor. This is a documented Spectre v2 cross-thread attack vector. On non-SMT systems or with SMT disabled, STIBP is not applicable. |
| 13 | Linux kernel support | Kernel enables STIBP based on CPU capabilities and boot parameters. `spectre_v2_user=` controls STIBP policy for userspace. On Intel eIBRS systems, STIBP is implicitly active. On AMD/legacy systems, STIBP is set via IA32_SPEC_CTRL[1] when a protected process is running. `CONFIG_MITIGATION_STIBP`. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` for `stibp` flag. Read `/sys/devices/system/cpu/vulnerabilities/spectre_v2` -- look for `STIBP: forced`, `STIBP: conditional`, or `STIBP: disabled` in the composite output string. Check `/sys/devices/system/cpu/smt/active` to determine if STIBP matters. |
| 15 | Virtualization confidence | **MEDIUM** -- Hypervisor can mask STIBP CPUID bit from guests. Guest can set STIBP in its own IA32_SPEC_CTRL (KVM emulates this), but cannot verify host STIBP configuration for host threads co-scheduled on sibling. Intel eIBRS implicit STIBP may not be visible to guest as an explicit STIBP capability. |
| 16 | ARM/AArch64 equivalent | ARM CSV2 (Cache Speculation Variant 2) provides branch predictor isolation. ARM does not have an exact STIBP equivalent because ARM SMT implementations are less common. Where ARM SMT exists, vendor-specific firmware mitigations apply. |
| 17 | References | Intel Deep Dive: Single Thread Indirect Branch Predictors; AMD Architecture Guidelines for Indirect Branch Control Extension; Linux kernel `spectre.rst` |
| 18 | Disposition when unused | **INVESTIGATE** -- If STIBP is available, SMT is active, and the spectre_v2 sysfs shows `STIBP: disabled`, cross-thread Spectre v2 attacks are possible. Acceptable if all sibling threads are trusted (same security domain). Unacceptable on multi-tenant or MLS systems. |
| 19 | Software utilization detection | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` -- look for the STIBP component. Values: `STIBP: disabled`, `STIBP: conditional`, `STIBP: forced`. |
| 20 | FIPS utilization requirement | N/A (security mitigation, not cryptographic primitive) |
| 21 | Active mitigation status | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` (STIBP status embedded in composite string) |
| 22 | Feature accessible vs advertised | STIBP requires microcode that exposes CPUID.7.0.EDX[27]. On Intel eIBRS systems, STIBP is implicit even if the STIBP CPUID bit is not separately enumerated. Hypervisors can hide STIBP from guests. BIOS does not gate STIBP. |
| 23 | Guest-vs-host discrepancy risk | **HIGH on SMT systems** -- Guest STIBP setting only affects the guest's own IA32_SPEC_CTRL. If the host does not enforce STIBP or eIBRS for host threads running on the sibling, cross-thread attacks from host to guest remain possible. Guest cannot verify host SMT policy. VM migration between eIBRS and non-eIBRS hosts changes implicit STIBP behavior. |

## SMT Dependency Matrix

STIBP's relevance depends entirely on SMT state:

| SMT State | STIBP Relevance | Notes |
|-----------|-----------------|-------|
| SMT active, untrusted workloads | **CRITICAL** | Cross-thread Spectre v2 attacks possible without STIBP |
| SMT active, trusted workloads | Important | Defense in depth; may be acceptable to leave conditional |
| SMT disabled (`nosmt`) | N/A | No sibling thread exists; STIBP has no effect |
| SMT not supported | N/A | CPU does not have SMT capability |

## STIBP Operating Modes

| Mode | When Active | Performance Impact | Protection Level |
|------|-------------|-------------------|-----------------|
| Disabled | Never | None | None (vulnerable on SMT) |
| Conditional | When a protected process is running (prctl or SECCOMP) | Only during protected process execution | Per-process opt-in |
| Forced | Always on for all processes | Continuous (5-30% on branch-heavy workloads) | Full SMT isolation |
| Implicit (eIBRS) | Always, as part of eIBRS | Minimal (included in eIBRS overhead) | Full SMT isolation (Intel only) |

## Interaction with eIBRS

On Intel systems with eIBRS:
- STIBP is **implicitly enabled** as part of eIBRS
- No separate MSR write needed for STIBP
- Cross-thread branch prediction isolation is included
- The `STIBP: disabled` sysfs value will NOT appear -- eIBRS subsumes STIBP

On AMD AutoIBRS systems:
- STIBP is **NOT implicit** -- must be explicitly enabled
- AMD documents that AutoIBRS does not protect userspace from sibling threads
- STIBP must be set in IA32_SPEC_CTRL for user-to-user cross-thread protection

On legacy IBRS systems:
- IBRS bit is cleared on return to userspace
- STIBP must be explicitly enabled for user-mode protection
- Both IBRS (for kernel) and STIBP (for userspace) are needed

## prctl Interface

User processes can influence STIBP via the same prctl interface as IBPB:

```
prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_FORCE_DISABLE, 0, 0)
```

When conditional STIBP is active (`spectre_v2_user=prctl` or `spectre_v2_user=seccomp`):
- STIBP is enabled in IA32_SPEC_CTRL when a protected process is scheduled
- STIBP is disabled when an unprotected process is scheduled (for performance)
- The MSR toggle on every context switch adds overhead

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Relation to STIBP | Notes |
|----|------|------|------|-------------------|-------|
| CVE-2017-5715 | Spectre v2 | 2018 | 5.6 | STIBP is part of the cross-thread mitigation | Microcode + kernel |
| CVE-2022-0001 | BHI (Branch History Injection) | 2022 | 4.7 | BHI can bypass eIBRS (and its implicit STIBP) | Additional BHI_DIS_S or clearing sequence needed |
| CVE-2022-27672 | Cross-Thread Return Address Predictions | 2022 | -- | Affects AMD Zen1 RAP sharing on SMT state transitions | RSB filling + KVM HLT intercept |

## Kernel Command Line Parameters

| Parameter | Values | Effect on STIBP |
|-----------|--------|-----------------|
| `spectre_v2_user=` | `on`, `off`, `auto`, `prctl`, `seccomp` | `on` forces STIBP always; `off` disables; `prctl`/`seccomp` enable conditional STIBP |
| `nosmt` | (no value) | Disables SMT, making STIBP unnecessary |
| `mitigations=off` | (global) | Disables STIBP along with all other mitigations |

## UMRS Posture Signal Connection

**IndicatorId::Mitigations (Critical):**
- On SMT-active systems, STIBP availability and enablement is critical for cross-thread isolation
- If sysfs shows `STIBP: disabled` AND `/sys/devices/system/cpu/smt/active` is `1`, this is a finding
- On Intel eIBRS systems, explicit STIBP check is unnecessary (eIBRS implies STIBP)

**Cross-signal dependency:**
- STIBP finding is conditional on SMT state (IndicatorId for SMT)
- If SMT is disabled, STIBP finding should be suppressed
- If eIBRS is active (from spectre_v2 sysfs), STIBP is implicitly covered

**SELinux MLS relevance:**
- On SMT systems with MLS policy, co-scheduled threads at different sensitivity levels without STIBP creates a speculative cross-label channel via branch prediction sharing

## Sources

- [Intel Deep Dive: Single Thread Indirect Branch Predictors](https://software.intel.com/security-software-guidance/insights/deep-dive-single-thread-indirect-branch-predictors)
- [AMD Architecture Guidelines for Indirect Branch Control Extension](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/white-papers/111006-architecture-guidelines-update-amd64-technology-indirect-branch-control-extension.pdf)
- [Linux kernel spectre.rst](https://docs.kernel.org/admin-guide/hw-vuln/spectre.html)
