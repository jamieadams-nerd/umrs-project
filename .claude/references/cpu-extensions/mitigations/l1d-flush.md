# L1D Flush (L1 Data Cache Flush)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | L1D Flush (L1 Data Cache Flush for L1TF / Foreshadow mitigation) |
| 2 | Vendor | Intel (AMD, Centaur, and non-Intel vendors not affected by L1TF) |
| 3 | CPUID detection | **L1D_FLUSH capability:** IA32_ARCH_CAPABILITIES presence indicates CPU awareness. Specifically, the kernel checks IA32_FLUSH_CMD MSR (0x10B) availability, which is enumerated via CPUID EAX=07H, ECX=0, EDX bit 28 (L1D_FLUSH). **L1TF immunity:** IA32_ARCH_CAPABILITIES MSR bit 0 (RDCL_NO) -- when set, processor is not affected by L1TF or Meltdown. |
| 4 | Linux `/proc/cpuinfo` flag | `flush_l1d` (L1D flush capability). Bug flag: `l1tf` (indicates vulnerability). |
| 5 | Key instructions | **WRMSR to IA32_FLUSH_CMD (0x10B)** with bit 0 set triggers hardware L1D flush. Alternatively, the kernel can perform a software L1D flush by walking a dedicated data buffer to evict all L1D lines. The hardware flush (MSR-based) is faster and more reliable. |
| 6 | Introduced | **L1D flush MSR:** Intel, via microcode update on Skylake+ (August 2018, in response to L1TF/Foreshadow). Newer CPUs (from ~2019) with RDCL_NO are immune to L1TF and do not require L1D flush. |
| 7 | Security relevance | L1TF (L1 Terminal Fault / Foreshadow) allows speculative access to L1D-resident data by exploiting how the CPU handles non-present page table entries. The attack bypasses all protection domains: userspace can read kernel data, guests can read host data, and guests can read other guests' data (via SMT shared L1D). L1D flush before VM entry ensures no host secrets remain in L1D when guest code executes. Kernel-to-user protection uses PTE inversion (permanent, zero-cost). |
| 8 | Performance benefit | None -- L1D flush is a security mechanism with performance cost. Flushing L1D means the guest starts with a cold cache after every VM entry. Impact depends on VMENTER frequency: 1-50% depending on workload. Conditional flush mode reduces overhead by skipping flush after audited code paths. |
| 9 | Known vulnerabilities | L1TF itself: CVE-2018-3615 (SGX), CVE-2018-3620 (OS/SMM), CVE-2018-3646 (Virtualization). Also: Multihit (CVE-2018-12207) -- page walk cache L1TF variant. L1D flush does NOT protect against cross-SMT-thread attacks -- a sibling thread's activity repopulates L1D continuously. Full protection requires L1D flush + SMT disabled. |
| 10 | Compliance mapping | NIST SP 800-53 SC-39 (Process Isolation), SI-16 (Memory Protection), SC-2 (Separation of System and User Functionality); CMMC SC.L2-3.13.10; NSA RTB (Defense in Depth) |
| 11 | Classification | **Critical/Defensive** |
| 12 | Classification rationale | Without L1D flush, a malicious VM guest can read any host physical memory that was recently in L1D, including secrets, crypto keys, and data from other VMs. This is a documented, practical attack (Foreshadow demonstrated against SGX enclaves and VMs). On bare-metal systems without VMs, the kernel's PTE inversion provides permanent zero-cost protection -- L1D flush is primarily critical for virtualization scenarios. |
| 13 | Linux kernel support | L1D flush is triggered by KVM before VMENTER. Controlled by `kvm-intel.vmentry_l1d_flush=` module parameter. Host kernel uses PTE inversion for user-space protection (always enabled, no parameter). Boot parameter `l1tf=` controls overall L1TF mitigation policy. `CONFIG_MITIGATION_L1TF`. |
| 14 | Detection method (safe Rust) | Parse `/proc/cpuinfo` for `flush_l1d` flag. Read `/sys/devices/system/cpu/vulnerabilities/l1tf` for current status. |
| 15 | Virtualization confidence | **HIGH RISK for guests** -- L1D flush is a HOST-side mitigation performed before entering the guest. The guest cannot verify whether the host is performing L1D flush on its behalf. Guest sysfs shows guest kernel's L1TF posture (PTE inversion), not host's VM-entry flush status. On CPUs with RDCL_NO, this is moot (not vulnerable). |
| 16 | ARM/AArch64 equivalent | ARM processors are not affected by L1TF. No equivalent mitigation needed. ARM's cache architecture and speculative execution model differ from Intel's in ways that prevent L1TF-class attacks. |
| 17 | References | Intel L1TF advisory (INTEL-SA-00161); Linux kernel `l1tf.rst`; Foreshadow academic paper |
| 18 | Disposition when unused | **CRITICAL for VM hosts** -- If running untrusted VMs on an L1TF-affected CPU and L1D flush is disabled, guest-to-host memory disclosure is possible. Check `kvm-intel.vmentry_l1d_flush` and `l1tf` boot parameters. For bare-metal systems, PTE inversion (always active) is sufficient. |
| 19 | Software utilization detection | `/sys/devices/system/cpu/vulnerabilities/l1tf` -- see values below. Also `/sys/module/kvm_intel/parameters/vmentry_l1d_flush` for KVM flush mode. |
| 20 | FIPS utilization requirement | N/A (security mitigation, not cryptographic primitive). However, L1TF can leak crypto key material from L1D, making it indirectly relevant to FIPS system integrity. |
| 21 | Active mitigation status | `/sys/devices/system/cpu/vulnerabilities/l1tf` |
| 22 | Feature accessible vs advertised | L1D flush MSR requires microcode update. On older CPUs without the microcode update, the kernel uses a software flush (walking a data buffer). CPUs with RDCL_NO bit in IA32_ARCH_CAPABILITIES are immune and do not need L1D flush. |
| 23 | Guest-vs-host discrepancy risk | **HIGH** -- Guest cannot determine host's L1D flush policy. Guest sysfs shows PTE inversion status (always mitigated from guest kernel perspective). Host may have L1D flush disabled (`kvm-intel.vmentry_l1d_flush=never`) without guest awareness. |

## Sysfs Values

### /sys/devices/system/cpu/vulnerabilities/l1tf

Base value:

| Value | Meaning |
|-------|---------|
| `Not affected` | Processor immune (RDCL_NO=1 or non-Intel) |
| `Mitigation: PTE Inversion` | Host kernel protected via PTE inversion |

If KVM/VMX is enabled and processor is vulnerable, appended to `Mitigation: PTE Inversion`:

| Component | Values |
|-----------|--------|
| SMT status | `VMX: SMT vulnerable`, `VMX: SMT disabled` |
| L1D flush mode | `L1D vulnerable`, `L1D conditional cache flushes`, `L1D cache flushes` |

Example output:
```
Mitigation: PTE Inversion; VMX: conditional cache flushes, SMT vulnerable
```

### /sys/module/kvm_intel/parameters/vmentry_l1d_flush

| Value | Meaning |
|-------|---------|
| `always` | L1D flush on every VMENTER |
| `cond` | Flush only after non-audited code paths (default) |
| `never` | No L1D flush (VULNERABLE) |

## L1D Flush Modes

| Mode | Protection | Performance Impact | When Appropriate |
|------|-----------|-------------------|------------------|
| `always` | Maximum -- flush on every VMENTER | High (cold L1D every VM entry) | Untrusted guests, high-security environments |
| `cond` (default) | High -- flush after non-audited paths | Moderate (skip flush after safe paths) | Standard deployments with mixed trust |
| `never` | None -- no L1D flush | None | Trusted guests only, or RDCL_NO CPUs |

## SMT Interaction

L1D flush does NOT protect against cross-SMT-thread attacks:

1. Host flushes L1D before VMENTER
2. Guest begins executing
3. Sibling thread (host or other guest) brings data into shared L1D
4. Guest can speculatively access sibling's L1D-resident data via L1TF

Full protection requires EITHER:
- Disable SMT entirely (`nosmt` or `l1tf=full,force`)
- Pin guest VCPUs to dedicated physical cores with no other workload on sibling
- Use EPT disabling (significant performance impact)

## Connection to MDS

L1D flush and MDS mitigation interact:

- On CPUs affected by BOTH L1TF and MDS: L1D flush on VMENTER also clears CPU buffers that MDS targets, so L1D flush subsumes MDS VMENTER mitigation
- On CPUs affected by MDS but NOT L1TF: VERW-based buffer clearing is used at VMENTER instead of L1D flush
- On CPUs affected by L1TF with L1D flush DISABLED: MDS VERW mitigation is invoked explicitly at VMENTER if MDS mitigation is enabled

## CVE / Vulnerability Table

| ID | Name | Year | CVSS | Scope | Fix |
|----|------|------|------|-------|-----|
| CVE-2018-3615 | L1TF / Foreshadow (SGX) | 2018 | 7.9 | SGX enclave data | L1D flush + microcode |
| CVE-2018-3620 | L1TF (OS/SMM) | 2018 | 5.6 | Kernel memory | PTE inversion (kernel) |
| CVE-2018-3646 | L1TF (Virtualization) | 2018 | 5.6 | VM host memory | L1D flush on VMENTER |
| CVE-2018-12207 | Machine Check on Page Size Change (Multihit) | 2019 | 6.5 | Page walk cache L1TF variant | Exec-only EPT mappings |

## Kernel Command Line Parameters

| Parameter | Values | Effect |
|-----------|--------|--------|
| `l1tf=` | `full`, `full,force`, `flush`, `flush,nosmt`, `flush,nowarn`, `off` | Overall L1TF policy |
| `kvm-intel.vmentry_l1d_flush=` | `always`, `cond`, `never` | KVM VM-entry flush mode |
| `nosmt` | (no value) | Disables SMT (provides full L1TF cross-thread protection) |
| `mitigations=off` | (global) | Disables L1D flush along with all other mitigations |

## UMRS Posture Signal Connection

**IndicatorId::Mitigations (Critical):**
- On L1TF-affected CPUs running VMs: check that L1D flush is not disabled
- If sysfs shows `L1D vulnerable`, this is a CRITICAL finding for VM hosts
- `L1D conditional cache flushes` is the expected default -- acceptable for most deployments
- On non-VM systems, PTE inversion (always active) provides full protection

**Cross-signal dependency:**
- L1D flush findings are relevant only on Intel CPUs that are L1TF-affected (no RDCL_NO)
- SMT state (IndicatorId for SMT) affects the severity of L1D flush findings
- If RDCL_NO is set (newer CPUs), L1TF/L1D flush findings should be suppressed

## Sources

- [Intel L1TF Advisory (INTEL-SA-00161)](https://www.intel.com/content/www/us/en/security-center/advisory/intel-sa-00161.html)
- [Linux kernel l1tf.rst](https://docs.kernel.org/admin-guide/hw-vuln/l1tf.html)
- [Foreshadow: Extracting the Keys to the Intel SGX Kingdom](https://foreshadowattack.eu/)
