# SSBD (Speculative Store Bypass Disable)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | SSBD (Speculative Store Bypass Disable) / Spec Store Bypass Mitigation |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | **Intel/AMD:** CPUID EAX=07H, ECX=0, EDX bit 31 (SSBD). Controlled via IA32_SPEC_CTRL MSR (0x48) bit 2. **AMD also:** CPUID 0x80000008, EBX bit 24 (SSBD). AMD additionally supports per-process SSBD via LS_CFG MSR on some older families, and VIRT_SSBD via CPUID 0x80000008 EBX bit 25 for virtualized environments. |
| 4 | Linux `/proc/cpuinfo` flag | `ssbd` (Intel and AMD), `virt_ssbd` (AMD virtualization variant), `amd_ssbd` (AMD native via LS_CFG) |
| 5 | Key instructions | No dedicated instruction. SSBD is controlled via MSR write: IA32_SPEC_CTRL[2] = SSBD enable. This prevents speculative loads from bypassing older stores to the same address. |
| 6 | Introduced | **Intel:** Via microcode update on Skylake+ (May 2018). **AMD:** Via microcode update on Zen family (May 2018). Some AMD processors use LS_CFG MSR instead of SPEC_CTRL for the same effect. |
| 7 | Security relevance | SSBD mitigates Spectre variant 4 (Speculative Store Bypass, CVE-2018-3639). In a speculative store bypass, a load instruction speculatively reads stale data from a memory address before an older store to the same address completes. An attacker can use this to speculatively read privileged data and infer it through cache side channels. SSBD prevents the CPU from speculatively bypassing stores, closing this channel. |
| 8 | Performance benefit | None -- SSBD is a security mechanism with performance cost. Disabling speculative store bypassing can degrade performance by 2-8% on workloads that benefit from store-to-load forwarding optimization. |
| 9 | Known vulnerabilities | Spectre v4 (CVE-2018-3639) is the attack SSBD mitigates. The vulnerability itself has limited practical impact compared to Spectre v1/v2 because the attacker needs precise control over store-load timing. Most real-world exploits focus on JIT environments (JavaScript engines, eBPF). Kernel code is generally not vulnerable due to limited gadget availability. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SI-16 (Memory Protection); CMMC SC.L2-3.13.10; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** |
| 12 | Classification rationale | Spectre v4 is a documented attack that allows speculative data leakage. While less practically exploitable than Spectre v2, it is a known attack class that SSBD definitively blocks. On systems running untrusted code (JIT, sandboxed processes, VMs), absence of SSBD leaves an exploitable side channel. |
| 13 | Linux kernel support | Kernel detects SSBD via CPUID and provides per-process control via `prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS)`. Global control via `spec_store_bypass_disable=` boot parameter. The kernel does NOT enable SSBD globally by default -- it uses a per-process model where only processes that opt in (or are forced by seccomp) get SSBD. `CONFIG_MITIGATION_SSB`. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` for `ssbd` flag. Read `/sys/devices/system/cpu/vulnerabilities/spec_store_bypass` for current mitigation status. |
| 15 | Virtualization confidence | **MEDIUM** -- Guest can set SSBD in its own IA32_SPEC_CTRL (KVM emulates this MSR). AMD VIRT_SSBD provides a lightweight virtualized SSBD path. However, the host controls global policy, and a guest cannot verify the host's SSBD stance for host-side code. |
| 16 | ARM/AArch64 equivalent | ARM SSBS (Speculative Store Bypass Safe) instruction/PSTATE bit. Available from ARMv8.5. Controlled per-thread via PSTATE.SSBS. `/proc/cpuinfo` flag: `ssbs`. |
| 17 | References | Intel Speculative Store Bypass advisory (INTEL-SA-00115); AMD Speculative Store Bypass advisory; Linux kernel `spectre.rst` |
| 18 | Disposition when unused | **MONITOR** -- SSBD is not globally enabled by default for good reason (performance cost, limited practical exploitability in kernel code). However, if the system runs untrusted JIT code or sandboxed processes without SSBD, this is worth noting. The per-process model via prctl/seccomp is the recommended approach. |
| 19 | Software utilization detection | `/sys/devices/system/cpu/vulnerabilities/spec_store_bypass` -- see values below. Also `/proc/<pid>/status` Speculation_Store_Bypass field for per-process status. |
| 20 | FIPS utilization requirement | N/A (security mitigation, not cryptographic primitive) |
| 21 | Active mitigation status | `/sys/devices/system/cpu/vulnerabilities/spec_store_bypass` |
| 22 | Feature accessible vs advertised | SSBD requires microcode update to expose the CPUID bit. BIOS does not gate SSBD. Some AMD processors expose SSBD via LS_CFG MSR rather than SPEC_CTRL, which requires different kernel code paths. Hypervisors generally expose SSBD to guests. |
| 23 | Guest-vs-host discrepancy risk | **LOW** -- SSBD is per-logical-processor and controlled by the executing context. Guest setting SSBD in its SPEC_CTRL directly affects its own execution. AMD VIRT_SSBD allows lightweight guest SSBD without expensive SPEC_CTRL MSR writes. |

## Sysfs Values

The file `/sys/devices/system/cpu/vulnerabilities/spec_store_bypass` contains:

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor not vulnerable to Spectre v4 |
| `Vulnerable` | Processor vulnerable, no mitigation enabled |
| `Mitigation: Speculative Store Bypass disabled` | SSBD globally enabled (all threads) |
| `Mitigation: Speculative Store Bypass disabled via prctl` | Per-process SSBD via prctl interface |
| `Mitigation: Speculative Store Bypass disabled via prctl and seccomp` | Per-process SSBD via prctl and seccomp |

## Per-Process Control

SSBD supports fine-grained per-process control, unlike most other mitigations:

```
prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_ENABLE, 0, 0)      // Allow SSB
prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_DISABLE, 0, 0)     // Disable SSB
prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_FORCE_DISABLE, 0, 0) // Force disable (cannot be re-enabled)
```

The per-process state is visible in `/proc/<pid>/status`:
```
Speculation_Store_Bypass:    thread vulnerable
Speculation_Store_Bypass:    thread mitigated
Speculation_Store_Bypass:    thread force mitigated
```

## Kernel Command Line Parameters

| Parameter | Values | Effect |
|-----------|--------|--------|
| `spec_store_bypass_disable=` | `off`, `on`, `auto`, `prctl`, `seccomp` | Controls SSBD policy |
| `mitigations=off` | (global) | Disables SSBD along with all other mitigations |

**Default:** `spec_store_bypass_disable=prctl,seccomp` -- processes can opt in via prctl, and seccomp-sandboxed processes get automatic SSBD.

## AMD-Specific Implementation Details

AMD provides multiple SSBD mechanisms:

| Mechanism | MSR | CPUID | Scope | Notes |
|-----------|-----|-------|-------|-------|
| SPEC_CTRL SSBD | IA32_SPEC_CTRL bit 2 | CPUID 7.0.EDX[31] | Per-thread (MSR) | Standard, but expensive MSR write |
| LS_CFG SSBD | LS_CFG MSR | Family-specific | Per-thread (MSR) | Older AMD path, same effect |
| VIRT_SSBD | VIRT_SPEC_CTRL | CPUID 0x80000008 EBX[25] | Per-vCPU | Lightweight path for VMs, avoids expensive MSR exits |
| PSFD | Predictive Store Forwarding Disable | CPUID 0x80000008 EBX[28] | Per-thread | AMD Zen 3+ replacement for SSBD with lower overhead |

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix |
|----|------|------|------|--------|-----|
| CVE-2018-3639 | Speculative Store Bypass (Spectre v4) | 2018 | 5.5 | Speculative load reads stale data before store completes | SSBD microcode + kernel per-process control |

## UMRS Posture Signal Connection

**SignalId::Mitigations (Critical):**
- SSBD availability is part of comprehensive speculative execution mitigation
- If `/sys/devices/system/cpu/vulnerabilities/spec_store_bypass` shows `Vulnerable`, the system lacks Spectre v4 mitigation entirely
- The per-process model (prctl/seccomp) is the expected default -- `Mitigation: ... via prctl and seccomp` is the normal healthy state
- Global SSBD (`Speculative Store Bypass disabled` without `via prctl`) is overly aggressive for most deployments

## Sources

- [Intel Speculative Store Bypass Advisory (INTEL-SA-00115)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00115.html)
- [AMD Speculative Store Bypass](https://www.amd.com/en/resources/product-security/bulletin/amd-sb-1001.html)
- [Linux kernel spectre.rst](https://docs.kernel.org/admin-guide/hw-vuln/spectre.html)
- [Linux kernel spec_ctrl documentation](https://docs.kernel.org/userspace-api/spec_ctrl.html)
