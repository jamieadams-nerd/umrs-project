# IBRS / eIBRS (Indirect Branch Restricted Speculation / Enhanced IBRS)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | IBRS (Indirect Branch Restricted Speculation) / eIBRS (Enhanced IBRS) / AutoIBRS (AMD Automatic IBRS) |
| 2 | Vendor | Both (Intel & AMD) |
| 3 | CPUID detection | **IBRS/IBPB:** CPUID EAX=07H, ECX=0, EDX bit 26 (SPEC_CTRL + PRED_CMD). **eIBRS (Intel):** IA32_ARCH_CAPABILITIES MSR bit 1 (IBRS_ALL). **AutoIBRS (AMD):** CPUID 0x80000008, EBX bit 30. Controlled via IA32_SPEC_CTRL MSR bit 0 (IBRS). |
| 4 | Linux `/proc/cpuinfo` flag | `ibrs` (legacy), `ibrs_enhanced` or `arch_capabilities` (eIBRS inferred), `autoibrs` (AMD) |
| 5 | Key instructions | No dedicated instructions. Controlled via MSR writes: IA32_SPEC_CTRL[0] = IBRS enable. WRMSR to IA32_SPEC_CTRL. |
| 6 | Introduced | **IBRS:** Intel via microcode update on Skylake+ (2018), AMD Zen via microcode (2018). **eIBRS:** Intel Cascade Lake and newer (2019). **AutoIBRS:** AMD Zen 4 / EPYC 9004 (2022). |
| 7 | Security relevance | Primary kernel-side mitigation for Spectre variant 2 (Branch Target Injection, CVE-2017-5715). IBRS restricts speculative indirect branch predictions to prevent cross-privilege speculation. eIBRS provides this protection permanently after a single MSR write at boot, rather than requiring toggling on every kernel entry/exit. On Intel eIBRS systems, cross-thread protection (STIBP) is implicitly enabled. |
| 8 | Performance benefit | **eIBRS** has minimal performance overhead (single MSR write at boot). **Legacy IBRS** has significant overhead because it must be set on every kernel entry and cleared on kernel exit. eIBRS eliminates the need for retpoline on hardware that supports it. |
| 9 | Known vulnerabilities | eIBRS does not fully isolate Branch History Buffer (BHB) across privilege levels. BHI (Branch History Injection) attacks can bypass eIBRS. Intel ITS (Indirect Target Selection, CVE-2024-28956) affects eIBRS guest/host isolation on some processors. Post-barrier RSB (PBRSB) vulnerability affects eIBRS systems on VM exit. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SI-16 (Memory Protection); CMMC SC.L2-3.13.10; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** |
| 12 | Classification rationale | Absence removes the primary hardware mitigation against Spectre v2, leaving the system reliant on software-only retpoline (which is less effective on some microarchitectures and has higher overhead). Without IBRS/eIBRS, cross-privilege branch target injection is a viable attack. |
| 13 | Linux kernel support | Kernel detects IBRS via CPUID and selects the best mitigation strategy. eIBRS is preferred over retpoline when available. `CONFIG_MITIGATION_IBRS_ENTRY`, `CONFIG_MITIGATION_RETPOLINE`. Controlled by `spectre_v2=` boot parameter. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` for `ibrs_enhanced` or `arch_capabilities` flag. Read `/sys/devices/system/cpu/vulnerabilities/spectre_v2` for active mitigation status (authoritative). |
| 15 | Virtualization confidence | **MEDIUM RISK** -- Hypervisor can mask SPEC_CTRL CPUID bits. Guest may see eIBRS support that is actually emulated. The sysfs vulnerability file within the guest reflects the guest's mitigation state, not the host's hardware capability. ITS vulnerability (CVE-2024-28956) specifically breaks eIBRS guest/host isolation on affected processors. |
| 16 | ARM/AArch64 equivalent | ARM CSV2 (Cache Speculation Variant 2) provides equivalent branch prediction isolation. Firmware-level mitigation via SMCCC interface. |
| 17 | References | Intel Speculative Execution Side Channel Mitigations (336996); AMD Architecture Guidelines for Indirect Branch Control Extension; Linux kernel `spectre.rst`; Intel ITS advisory |
| 18 | Disposition when unused | **INVESTIGATE** -- If eIBRS is available but the spectre_v2 sysfs shows retpoline or no mitigation, the most effective hardware mitigation is not being utilized. Check for `spectre_v2=off` or `mitigations=off` on kernel command line. |
| 19 | Software utilization detection | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` -- look for "Enhanced IBRS" or "Enhanced / Automatic IBRS" in the output. Retpoline or LFENCE indicates eIBRS is not in use. |
| 20 | FIPS utilization requirement | N/A (security mitigation, not cryptographic primitive) |
| 21 | Active mitigation status | `/sys/devices/system/cpu/vulnerabilities/spectre_v2` |
| 22 | Feature accessible vs advertised | IBRS requires microcode update to expose CPUID bit. BIOS does not gate IBRS, but stale microcode means the MSR may exist but not function correctly. eIBRS requires CPU hardware support (not microcode-grantable on older silicon). Hypervisors can hide SPEC_CTRL from guests. |
| 23 | Guest-vs-host discrepancy risk | **HIGH** -- Guest may not see eIBRS even when host has it. ITS vulnerability means eIBRS guest/host isolation is broken on some Skylake-era and Cascade Lake processors. VM migration between eIBRS and non-eIBRS hosts silently changes mitigation strategy. |

## IBRS vs eIBRS vs AutoIBRS Comparison

| Property | Legacy IBRS | Enhanced IBRS (Intel) | AutoIBRS (AMD) |
|----------|-------------|----------------------|----------------|
| MSR control | IA32_SPEC_CTRL[0] toggled on every kernel entry/exit | IA32_SPEC_CTRL[0] set once at boot | IA32_SPEC_CTRL[0] set once at boot |
| Cross-privilege protection | Only while IBRS bit is set | Always active after single write | Always active after single write |
| Implicit STIBP | No -- must be explicitly enabled | Yes -- cross-thread protection included | No -- STIBP must be explicitly enabled |
| Performance impact | HIGH (MSR toggle on every syscall/interrupt) | MINIMAL (single MSR write at boot) | MINIMAL (single MSR write at boot) |
| Retpoline needed | May still use retpoline alongside | No -- replaces retpoline | No -- replaces retpoline |
| BHB protection | No | No (BHI attacks possible) | Partial |
| Userspace protection | No -- IBRS cleared on return to user | Kernel protected, user needs STIBP | Kernel protected, user needs STIBP |

## Spectre v2 Sysfs Values (spectre_v2)

The file `/sys/devices/system/cpu/vulnerabilities/spectre_v2` contains a composite string with multiple components:

**Kernel mitigation status (one of):**
- `Not affected` -- Processor not vulnerable
- `Mitigation: None` -- Vulnerable, no mitigation
- `Mitigation: Retpolines` -- Using retpoline thunks
- `Mitigation: LFENCE` -- Using LFENCE instructions
- `Mitigation: Enhanced IBRS` -- Hardware eIBRS mitigation
- `Mitigation: Enhanced IBRS + Retpolines` -- eIBRS with retpoline fallback
- `Mitigation: Enhanced IBRS + LFENCE` -- eIBRS with LFENCE fallback
- `Mitigation: Enhanced / Automatic IBRS` -- AMD AutoIBRS

**Appended components (semicolon-separated):**
- `IBRS_FW` -- IBRS enabled for firmware calls
- `IBPB: disabled` / `IBPB: always-on` / `IBPB: conditional`
- `STIBP: disabled` / `STIBP: forced` / `STIBP: conditional`
- `RSB filling` -- RSB stuffing on context switch
- `PBRSB-eIBRS: SW sequence` / `PBRSB-eIBRS: Vulnerable` / `PBRSB-eIBRS: Not affected`
- `BHI: Not affected` / `BHI: Retpoline` / `BHI: BHI_DIS_S` / `BHI: SW loop, KVM SW loop` / `BHI: Vulnerable` / `BHI: Vulnerable, KVM: SW loop`

**Example real-world output:**
```
Mitigation: Enhanced / Automatic IBRS; IBPB: conditional; RSB filling; PBRSB-eIBRS: Not affected; BHI: Not affected
```

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Impact | Fix |
|----|------|------|------|--------|-----|
| CVE-2017-5715 | Spectre v2 / Branch Target Injection | 2018 | 5.6 | Cross-privilege speculative execution | IBRS/eIBRS + retpoline (microcode + kernel) |
| CVE-2022-0001 | Branch History Injection (BHI) | 2022 | 4.7 | BHB bypasses eIBRS | BHI_DIS_S or BHB clearing sequence (microcode + kernel) |
| CVE-2022-0002 | Intra-mode BTI | 2022 | 6.5 | Same-privilege branch target injection | eIBRS + retpoline combination |
| CVE-2024-28956 | Indirect Target Selection (ITS) | 2024 | 4.7 | eIBRS guest/host isolation bypass | Aligned branch/return thunks (microcode + kernel) |
| N/A | PBRSB (Post-barrier RSB) | 2022 | -- | RSB underflow after VM exit on eIBRS | SW RSB stuffing sequence |

## Kernel Command Line Parameters

| Parameter | Values | Effect |
|-----------|--------|--------|
| `spectre_v2=` | `on`, `off`, `auto`, `retpoline`, `retpoline,generic`, `retpoline,lfence`, `eibrs`, `eibrs,retpoline`, `eibrs,lfence`, `ibrs` | Selects Spectre v2 mitigation strategy |
| `spectre_v2_user=` | `on`, `off`, `auto`, `prctl`, `prctl,ibpb`, `seccomp`, `seccomp,ibpb` | Controls user-to-user mitigation (IBPB/STIBP) |
| `nospectre_v2` | (no value) | Disables all Spectre v2 mitigations |
| `spectre_bhi=` | `on`, `off` | Controls BHI mitigation |

## UMRS Posture Signal Connection

**IndicatorId::Mitigations (Critical):**
- IBRS/eIBRS is the primary hardware capability that enables effective Spectre v2 mitigation
- If `/sys/devices/system/cpu/vulnerabilities/spectre_v2` shows "Retpolines" on a CPU that supports eIBRS, this is a configuration finding -- the more effective hardware mitigation is available but not in use
- If it shows "None" or "Vulnerable," this is a CRITICAL finding

**Microcode dependency:**
- Legacy IBRS requires microcode that exposes CPUID.7.0.EDX[26]
- eIBRS requires hardware support (cannot be granted by microcode alone)
- BHI/ITS fixes require additional microcode updates

## Sources

- [Intel Speculative Execution Side Channel Mitigations](https://www.intel.com/content/dam/develop/external/us/en/documents/336996-speculative-execution-side-channel-mitigations.pdf)
- [AMD Architecture Guidelines for Indirect Branch Control Extension](https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/white-papers/111006-architecture-guidelines-update-amd64-technology-indirect-branch-control-extension.pdf)
- [Linux kernel spectre.rst](https://docs.kernel.org/admin-guide/hw-vuln/spectre.html)
- [Intel Indirect Target Selection Advisory](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/advisory-guidance/indirect-target-selection.html)
- [Oracle: Understanding Spectre v2 Mitigations on x86](https://blogs.oracle.com/linux/understanding-spectre-v2-mitigations-on-x86)
