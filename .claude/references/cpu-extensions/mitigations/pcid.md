# PCID (Process Context Identifiers)

## 23-Column Matrix Profile

| # | Column | Value |
|---|--------|-------|
| 1 | Feature name | PCID (Process Context Identifiers) / INVPCID (Invalidate PCID) |
| 2 | Vendor | Intel (primary), AMD (Zen 3+) |
| 3 | Category | 10 — Speculative Execution Mitigations (PTI performance enabler) |
| 4 | Purpose | PCID allows the CPU to tag TLB entries with a 12-bit process context identifier. When the OS switches address spaces, PCID-tagged TLB entries from the previous context can be preserved rather than flushed. This is critical for PTI/KPTI (Kernel Page Table Isolation) performance: without PCID, every kernel entry/exit requires a full TLB flush because PTI switches between two separate page tables. With PCID, the kernel and user page tables each get a different PCID tag, and context switches preserve TLB entries from both — reducing the PTI performance penalty from 30-50% to under 5%. |
| 5 | Key instructions | INVPCID (Invalidate Process-Context Identifier — leaf 7, EBX bit 10): targeted TLB invalidation by PCID. Four types: individual-address, single-context, all-including-global, all-except-global. MOV to CR3 with bit 63 set preserves TLB entries for the new PCID rather than flushing. |
| 6 | CPUID detection | PCID: CPUID EAX=01H, ECX bit 17. INVPCID: CPUID EAX=07H, ECX=0H, EBX bit 10. PCID is enabled by setting CR4.PCIDE (bit 17). |
| 7 | Linux `/proc/cpuinfo` flag | `pcid`, `invpcid` |
| 8 | Linux detection — authoritative path | `/proc/cpuinfo` flags: `pcid` and `invpcid`. Kernel activates PCID automatically when PTI is enabled and CPU supports it. `dmesg | grep pcid` shows "PCID enabled" or "PCID disabled" at boot. |
| 9 | Minimum CPU generations | Intel Westmere (2010) for PCID, Intel Haswell (2013) for INVPCID. AMD Zen 3 (2020) for PCID support. Earlier AMD CPUs advertise PCID but may not fully implement it — kernel checks carefully. |
| 10 | Security benefit | PCID itself is not a security feature — it is a performance optimization that makes the Meltdown mitigation (PTI/KPTI) practical in production. Without PCID, PTI's TLB flush overhead makes it prohibitively expensive for latency-sensitive workloads, creating pressure to disable it. PCID removes that pressure, making PTI deployment sustainable. |
| 11 | Performance benefit | Critical. PTI without PCID: 30-50% performance regression on syscall-heavy workloads. PTI with PCID: under 5% regression. Database, web server, and container workloads see the largest benefit. The difference is that PCID allows the TLB to retain entries across the user↔kernel page table switch. |
| 12 | Assurance caveats | PCID does not protect against any attack directly. Its value is entirely in enabling PTI to remain active without crippling performance. On systems without PCID, administrators may be tempted to disable PTI (`nopti` kernel parameter) to recover performance — this re-exposes the system to Meltdown. PCID must be present for PTI to be considered sustainable in production. |
| 13 | Virtualization behavior | KVM: PCID is passed through to guests when available on host. VMware: PCID supported in guest VMs. Hyper-V: supports PCID. Guest PTI performance depends on guest seeing PCID — if hypervisor masks it, guest PTI runs at degraded performance. |
| 14 | Firmware / BIOS / microcode dependency | `{ bios_enable_required: false, microcode_required: false }` — PCID is a native CPU feature controlled via CR4. No BIOS gate. |
| 15 | Audit-card relevance | **Important** (PTI performance enabler) |
| 16 | Recommended disposition when unused | If PCID is present but PTI is disabled (`nopti` in cmdline): **CRITICAL** finding — Meltdown mitigation disabled. If PCID is absent on a system running PTI: **CAUTION** — PTI is active but at significant performance cost, creating operational pressure to disable it. If PCID is present and PTI is active: optimal configuration. |
| 17 | Software utilization detection method | Layer 1: `grep pcid /proc/cpuinfo`. Layer 2: check PTI status in `/sys/devices/system/cpu/vulnerabilities/meltdown` — should show "Mitigation: PTI". Verify `nopti` is NOT in `/proc/cmdline`. Cross-check: `dmesg | grep PCID` for explicit activation message. |
| 18 | FIPS utilization requirement | N/A — performance optimization, not cryptographic primitive. |
| 19 | Active mitigation status path | `/sys/devices/system/cpu/vulnerabilities/meltdown` — look for "Mitigation: PTI" (PTI active, PCID enabling it). |
| 20 | Feature accessible vs advertised | PCID cannot be BIOS-disabled. If CPUID reports it, it is available. However, CR4.PCIDE must be set by the OS (kernel does this automatically when available). Early AMD Zen CPUs had incomplete PCID implementations; Linux kernel has explicit quirk checks. |
| 21 | Guest-vs-host discrepancy risk | Medium. Older hypervisors may not expose PCID to guests. Guest running PTI without PCID suffers severe performance penalty. VM migration from PCID-capable to PCID-incapable host silently degrades PTI performance. |
| 22 | Notes | PCID is the reason PTI is viable in production. Intel's Meltdown response strategy was: (1) PTI in software, (2) PCID to make it fast. Systems without PCID face a choice between security (PTI on, slow) and performance (PTI off, vulnerable). Connection to existing posture signal: `SignalId::Pti` (High priority). PCID should be a cross-referenced signal — if Pti is active and pcid is absent, flag the performance risk. |
| 23 | Sources | Intel SDM Vol 3A Section 4.10.1 (Process-Context Identifiers); AMD APM Vol 2; LWN: "PCID is now a critical performance feature" (2018); Linux kernel TLB management code (arch/x86/mm/tlb.c); KAISER/KPTI patches |

## Connection to PTI / Meltdown

### The PTI Performance Problem

Meltdown (CVE-2017-5754) allows unprivileged processes to read kernel memory. The mitigation (PTI/KPTI) separates kernel and user page tables — the kernel page table is not mapped when running in user mode.

This requires a page table switch (CR3 write) on every:
- System call entry/exit
- Interrupt entry/exit
- Exception entry/exit

Without PCID, each CR3 write flushes the entire TLB. On syscall-heavy workloads (databases, web servers, containers), this creates 30-50% performance degradation.

### How PCID Fixes It

With PCID:
1. Kernel page table gets PCID 1, user page table gets PCID 2
2. CR3 write with bit 63 set tells the CPU "keep TLB entries for the new PCID"
3. TLB entries from both contexts survive across switches
4. INVPCID instruction enables targeted invalidation when needed

Result: PTI overhead drops from 30-50% to under 5%.

### The Operational Risk

On systems without PCID:
- PTI causes severe performance regression
- Operations teams face pressure to add `nopti` to kernel command line
- Disabling PTI re-exposes the system to Meltdown
- This is a **real-world compliance risk** — performance pressure vs security

## CVE / Vulnerability Table

| ID | Name | Year | Impact | PCID Relevance |
|----|------|------|--------|---------------|
| CVE-2017-5754 | Meltdown | 2018 | Kernel memory read from user space | PCID makes the PTI mitigation performant |
| CVE-2018-3615 | L1TF / Foreshadow | 2018 | L1 cache side-channel | PTI (enabled by PCID) provides partial mitigation |
| N/A | PTI performance bypass | Operational | Teams disable PTI due to performance | PCID eliminates this pressure |

## Kernel Build Dependencies

| Config Option | Feature | Default (RHEL 10) | Since |
|---------------|---------|-------------------|-------|
| `CONFIG_PAGE_TABLE_ISOLATION` | Kernel Page Table Isolation | `=y` | Linux 4.15 (2018) |
| `CONFIG_X86_PCID` | PCID TLB optimization | Not a separate config — enabled automatically | Linux 4.14 |

## Posture Check Specification

1. Check `pcid` in `/proc/cpuinfo` flags (Layer 1 — hardware present)
2. Check `/sys/devices/system/cpu/vulnerabilities/meltdown` for "Mitigation: PTI" (PTI active)
3. Check `nopti` is NOT in `/proc/cmdline` (PTI not disabled)
4. If PTI active + PCID present → optimal
5. If PTI active + PCID absent → flag performance risk (CAUTION)
6. If PTI disabled (`nopti`) + PCID present → **CRITICAL** finding (Meltdown exposed)
7. Cross-reference with `SignalId::Pti` posture signal

## ARM Equivalent

ARM does not have a direct PCID equivalent. ARM uses ASID (Address Space ID) which serves
a similar TLB-tagging purpose. ASID is always available on ARMv8 and is used automatically
by the kernel — there is no "missing ASID" performance gap on ARM. This makes the
Meltdown/PTI performance problem Intel-specific.

## Sources

- Intel SDM Vol 3A Section 4.10.1 (Process-Context Identifiers)
- AMD APM Vol 2 Section 5.5.2
- [LWN: PCID is now a critical performance feature (2018)](https://lwn.net/Articles/771973/)
- [Linux kernel meltdown mitigation documentation](https://docs.kernel.org/admin-guide/hw-vuln/meltdown.html)
- [The Meltdown Paper (2018)](https://meltdownattack.com/)
- Linux arch/x86/mm/tlb.c — PCID management implementation
