# IBPB (Indirect Branch Predictor Barrier)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | IBPB (Indirect Branch Predictor Barrier) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | **Intel/AMD:** CPUID EAX=07H, ECX=0, EDX bit 26 (SPEC_CTRL / PRED_CMD combined enumeration). IBPB is issued via WRMSR to IA32_PRED_CMD (MSR 0x49) with bit 0 set. Note: the same CPUID bit enumerates both IBRS and IBPB support. **AMD also:** CPUID 0x80000008, EBX bit 12 (IBPB). |
| 4 | Linux `/proc/cpuinfo` flag | `ibpb` (AMD-specific enumeration), or inferred from `ibrs` (Intel -- IBRS/IBPB share CPUID bit) |
| 5 | Key instructions | No dedicated instruction. IBPB is a command issued via WRMSR to IA32_PRED_CMD MSR (0x49), bit 0. This is a serializing operation that flushes the indirect branch predictor state for the current logical processor. |
| 6 | Introduced | **Intel:** Via microcode update on Skylake+ (January 2018, in response to Spectre v2). **AMD:** Via microcode update on Zen family (January 2018). Extended IBPB (for SRSO) on Zen 3/4 via later microcode. |
| 7 | Security relevance | IBPB provides a one-shot barrier that flushes all indirect branch prediction state on the executing logical processor. This prevents branch target injection attacks across security boundaries (context switch between user processes, VM entry/exit). On SELinux MLS systems, IBPB at context switch prevents cross-domain branch predictor contamination -- a speculative information flow that violates label boundaries even though no architectural data transfer occurs. |
| 8 | Performance benefit | None -- IBPB is purely a security mechanism with a performance cost. Each IBPB execution adds overhead to context switches and VM transitions. The cost varies by microarchitecture (typically 1-5 microseconds per barrier). |
| 9 | Known vulnerabilities | IBPB itself is not vulnerable, but its effectiveness depends on completeness of the predictor flush. AMD Zen 1/2 had incomplete IBPB that did not fully flush the Return Address Predictor, leading to SRSO (CVE-2023-20569). Extended IBPB microcode for AMD addresses this. Intel ITS (CVE-2024-28956) can bypass IBPB in certain configurations. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), AC-4 (Information Flow Enforcement -- prevents speculative cross-domain information flow); CMMC SC.L2-3.13.10; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** |
| 12 | Classification rationale | Without IBPB, indirect branch predictor state persists across context switches and VM transitions. An attacker in one security domain can poison the branch predictor and influence speculative execution in a different domain after a context switch. This is a documented, exploitable attack (Spectre v2 user-to-user, guest-to-host). |
| 13 | Linux kernel support | Kernel issues IBPB on context switch when switching between processes with different ASID/credentials, on VM exit, and when requested via `prctl(PR_SET_SPECULATION_CTRL)`. Controlled by `spectre_v2_user=` boot parameter. `CONFIG_MITIGATION_IBPB_ENTRY`. IBPB status appears in the spectre_v2 sysfs output. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` for `ibpb` flag. Read `/sys/devices/system/cpu/vulnerabilities/spectre_v2` -- look for `IBPB: conditional`, `IBPB: always-on`, or `IBPB: disabled` in the composite output string. |
| 15 | Virtualization confidence | **MEDIUM** -- Hypervisor controls when IBPB is issued. Guest can request IBPB via IA32_PRED_CMD MSR (KVM emulates this). However, the host decides whether to issue IBPB on VM entry/exit. A guest cannot verify that the host is issuing IBPB on its behalf. |
| 16 | ARM/AArch64 equivalent | ARM SMCCC ARCH_WORKAROUND_1 provides equivalent branch predictor invalidation via firmware call. Some ARM implementations provide BPIALL (Branch Predictor Invalidate All) instruction. |
| 17 | References | Intel Speculative Execution Side Channel Mitigations (336996); AMD Architecture Guidelines for Indirect Branch Control Extension; Linux kernel `spectre.rst`; AMD SRSO advisory |
| 18 | Disposition when unused | **INVESTIGATE** -- If IBPB is available but the spectre_v2 sysfs shows `IBPB: disabled`, cross-process and cross-VM speculative attacks are not mitigated. Check for `spectre_v2_user=off` or `mitigations=off` on kernel command line. |
| 19 | Software utilization detection | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` -- look for the IBPB component in the composite string. Values: `IBPB: disabled`, `IBPB: conditional`, `IBPB: always-on`. |
| 20 | FIPS utilization requirement | N/A (security mitigation, not cryptographic primitive) |
| 21 | Active mitigation status | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` (IBPB status embedded in composite string) |
| 22 | Feature accessible vs advertised | IBPB requires microcode update to expose the CPUID bit and implement the MSR. The CPUID bit can be present but microcode may be stale (pre-fix version) -- in that case the MSR write may be a no-op or have incomplete effect. Extended IBPB (for SRSO mitigation) requires additional AMD microcode beyond the original Spectre v2 fix. Hypervisors can hide IBPB support from guests. |
| 23 | Guest-vs-host discrepancy risk | **MEDIUM** -- Guest may not see IBPB CPUID bit if hypervisor masks it. The host may or may not issue IBPB on behalf of the guest during VM transitions. VM migration between hosts with different microcode versions can change IBPB effectiveness silently (original IBPB vs extended IBPB). |

## IBPB Usage Modes

| Mode | Trigger | Cost | Protection |
|------|---------|------|------------|
| Disabled | Never | None | None |
| Conditional | Context switch between processes with different credentials or SECCOMP; on prctl request | Per-context-switch | Protects targeted processes |
| Always-on | Every context switch | Highest | Full user-to-user protection |
| VM exit | On every VM exit | Per-VM-exit | Guest-to-host protection |
| SRSO extended | IBPB + RAP flush | Per-barrier | Full Spectre v2 + SRSO protection (AMD) |

## IBPB vs IBRS vs STIBP Interaction

IBPB, IBRS, and STIBP address different aspects of Spectre v2:

| Mechanism | What it protects | When it acts | Scope |
|-----------|-----------------|--------------|-------|
| IBPB | Cross-context predictor contamination | At context switch / VM transition | One-shot barrier, clears predictor state |
| IBRS/eIBRS | Cross-privilege speculative execution | Continuously while IBRS bit is set | Restricts speculation to same or higher privilege |
| STIBP | Cross-SMT-thread predictor sharing | Continuously while STIBP is active | Isolates sibling thread predictions |

A complete Spectre v2 mitigation typically uses all three: eIBRS for kernel protection, IBPB for process isolation, STIBP for SMT isolation. Disabling any one leaves a specific attack vector open.

## prctl Interface

User processes can request IBPB protection via the prctl interface:

```
prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_FORCE_DISABLE, 0, 0)
```

When a process has indirect branch speculation disabled:
- IBPB is issued when context switching to/from this process
- STIBP is enabled while this process is running (on SMT systems)

The `spectre_v2_user=` kernel boot parameter controls the default policy:
- `off` -- No user-to-user mitigation
- `auto` -- Kernel selects based on CPU
- `prctl` -- Per-process opt-in via prctl
- `prctl,ibpb` -- prctl + unconditional IBPB
- `seccomp` -- SECCOMP processes protected
- `seccomp,ibpb` -- SECCOMP + unconditional IBPB
- `on` -- All processes protected (IBPB + STIBP always-on)

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact on IBPB | Fix |
|----|------|------|------|----------------|-----|
| CVE-2017-5715 | Spectre v2 / Branch Target Injection | 2018 | 5.6 | IBPB is part of the mitigation | Microcode + kernel |
| CVE-2023-20569 | SRSO (Speculative Return Stack Overflow) | 2023 | 5.6 | Original IBPB insufficient on AMD (did not flush RAP) | Extended IBPB microcode (AMD Zen 1-4) |
| CVE-2024-28956 | ITS (Indirect Target Selection) | 2024 | 4.7 | IBPB alone insufficient on affected Intel | Additional aligned branch/return thunks |
| CVE-2022-29900 | Retbleed (AMD) | 2022 | 6.5 | IBPB used as part of mitigation strategy | IBPB on VM exit + kernel safe RET |

## Kernel Command Line Parameters

| Parameter | Values | Effect on IBPB |
|-----------|--------|----------------|
| `spectre_v2_user=` | `on`, `off`, `auto`, `prctl`, `prctl,ibpb`, `seccomp`, `seccomp,ibpb` | Controls when IBPB is issued for user-to-user protection |
| `mitigations=off` | (global) | Disables IBPB along with all other mitigations |
| `nospectre_v2` | (no value) | Disables all Spectre v2 mitigations including IBPB |
| `spec_rstack_overflow=ibpb` | (AMD) | Uses IBPB as SRSO mitigation strategy |

## Microcode Requirements

- **Minimum for IBPB support:** January 2018 microcode update on Intel Skylake+ and AMD Zen
- **Extended IBPB (AMD SRSO):** Additional microcode update (2023) that extends IBPB to also flush the Return Address Predictor
- Without the extended microcode, AMD systems are "Vulnerable: No microcode" for SRSO even if basic IBPB works

## UMRS Posture Signal Connection

**IndicatorId::Mitigations (Critical):**
- IBPB availability enables process-level Spectre v2 isolation
- If `/sys/devices/system/cpu/vulnerabilities/spectre_v2` shows `IBPB: disabled` on a multi-user system, this is a MEDIUM finding -- user-to-user speculative attacks are not mitigated
- If it shows `IBPB: disabled` on a system running VMs with untrusted guests, this is a HIGH finding

**SELinux MLS relevance:**
- IBPB on context switch between differently-labeled processes prevents speculative cross-label information flow
- Without IBPB, a process at one sensitivity level could speculatively influence branch prediction for a process at a higher level after a context switch

## Sources

- [Intel Speculative Execution Side Channel Mitigations](https://www.intel.com/content/dam/develop/external/us/en/documents/336996-speculative-execution-side-channel-mitigations.pdf)
- [AMD Architecture Guidelines for Indirect Branch Control Extension](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/white-papers/111006-architecture-guidelines-update-amd64-technology-indirect-branch-control-extension.pdf)
- [Linux kernel spectre.rst](https://docs.kernel.org/admin-guide/hw-vuln/spectre.html)
- [AMD SRSO advisory](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-7005.html)
