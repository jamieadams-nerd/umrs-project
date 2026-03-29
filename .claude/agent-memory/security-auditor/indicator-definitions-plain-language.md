---
name: indicator-definitions-plain-language
description: Authoritative plain-language reference for every kernel security posture indicator, TUI group, Trust Evidence tab semantics, and trust tier definitions. Source of truth for Rusty and Von Neumann in subsequent TUI enhancement phases.
type: reference
---

# Kernel Security Posture — Plain Language Reference

**Produced by:** The IRS (security-auditor)
**Date:** 2026-03-17
**Source files read:**
- `components/rusty-gadgets/libs/umrs-platform/src/posture/catalog.rs`
- `components/rusty-gadgets/libs/umrs-platform/src/posture/indicator.rs`
- `components/rusty-gadgets/libs/umrs-platform/src/confidence.rs`
- `components/rusty-gadgets/umrs-tui/src/main.rs`
- `components/rusty-gadgets/umrs-tui/src/indicators.rs`
- `components/rusty-gadgets/umrs-tui/src/tabs.rs`

**Purpose:** Rusty uses this for help text, tooltips, and documentation.
Von Neumann uses this for TUI display strings, descriptions, and status bar content.
The IRS uses this as the audit baseline for annotation review.

---

## Part 1 — TUI Tab Definitions

The TUI renders three tabs for every assessed system:

| Tab index | Label | What it shows |
|---|---|---|
| 0 | OS Information | Identity fields from `/etc/os-release`, package substrate facts, boot ID |
| 1 | Trust / Evidence | Label trust classification, confidence tier, downgrade reasons, contradictions, evidence records |
| 2 | Kernel Security | Live kernel security posture from `PostureSnapshot`; boot integrity, cryptographic posture, hardening groups |

---

## Part 2 — Indicator Groups (Kernel Security Tab)

Groups are rendered in this fixed order. A group is omitted entirely if no indicator in it has a readable live value — empty groups never appear.

### BOOT INTEGRITY

These indicators verify that the kernel loaded in a controlled, tamper-resistant state and cannot be silently replaced at runtime. A failure here means an attacker with root can replace or modify the running kernel without detection, undermining every other security control on the system.

Indicators in this group: Lockdown, KexecLoadDisabled, ModuleSigEnforce, Mitigations, Pti.

### CRYPTOGRAPHIC POSTURE

These indicators verify that the system uses government-validated cryptographic algorithms and handles entropy sources correctly. A failure here means cryptographic operations may use unvalidated algorithms, or that random number generation may be seeded from untrusted sources.

Indicators in this group: FipsEnabled, ModulesDisabled, RandomTrustCpu, RandomTrustBootloader.

Note: ModulesDisabled appears here because locking the module load gate is the final step that prevents replacing cryptographic kernel modules at runtime. Its presence enforces the integrity of the cryptographic subsystem.

### KERNEL SELF-PROTECTION

These indicators measure how well the kernel conceals its own internals from unprivileged processes. When these controls are weak, an attacker can use kernel memory addresses, performance counters, or debug interfaces to locate exploitable code and bypass ASLR.

Indicators in this group: RandomizeVaSpace, KptrRestrict, UnprivBpfDisabled, PerfEventParanoid, YamaPtraceScope, DmesgRestrict.

### PROCESS ISOLATION

These indicators control how much one process can see or interfere with another. When weak, a compromised process can steal credentials from a sibling, dump memory from a privileged process, or exploit the SysRq key to bypass normal access controls.

Indicators in this group: UnprivUsernsClone, Sysrq, SuidDumpable.

### FILESYSTEM HARDENING

These indicators close common privilege escalation paths through the filesystem. When missing, an attacker who can write to `/tmp` or another world-writable directory can craft symlinks or hardlinks that trick privileged programs into reading or writing files they should not.

Indicators in this group: ProtectedSymlinks, ProtectedHardlinks, ProtectedFifos, ProtectedRegular.

### MODULE RESTRICTIONS

These indicators verify that high-risk kernel modules are blocked from loading and that audit-relevant modules are properly configured. USB storage and wireless interfaces are primary data exfiltration vectors; FireWire and Thunderbolt can be used for direct memory attacks that bypass the OS entirely.

Indicators in this group: BluetoothBlacklisted, UsbStorageBlacklisted, FirewireCoreBlacklisted, ThunderboltBlacklisted, NfConntrackAcct.

---

## Part 3 — Per-Indicator Definitions

All 37 indicators in the catalog, ordered by group. Impact tiers: Critical > High > Medium.

For CmdlineAbsent indicators: the hardened state is that the weakening flag is ABSENT from `/proc/cmdline`. Green = flag absent (good). Red = flag present (hardening failure).

---

### GROUP: Kernel Self-Protection

#### KptrRestrict — `kernel.kptr_restrict`

- **What is it?** Controls whether the kernel prints its own internal memory addresses in `/proc` files and system logs.
- **Good value:** `2` — kernel pointer addresses are hidden from all users, including root.
- **Bad value:** `0` — pointer addresses are visible to anyone, including unprivileged users. `1` — hidden from unprivileged users but still visible to root.
- **Why it matters:** Knowing where kernel code and data live in memory is the first step in most kernel exploits; hiding these addresses forces attackers to guess or brute-force layouts.
- **Impact:** Critical
- **NIST SP 800-53:** CM-6(a), SC-30, SC-30(2), SC-30(5)
- **CCE:** CCE-88686-1

---

#### RandomizeVaSpace — `kernel.randomize_va_space`

- **What is it?** Address Space Layout Randomization (ASLR) — randomly positions the stack, heap, and shared libraries in memory every time a program starts.
- **Good value:** `2` — full ASLR: stack, heap, mmap, and VDSO are all randomized.
- **Bad value:** `0` — ASLR disabled entirely. `1` — partial randomization (mmap not randomized).
- **Why it matters:** Without ASLR, memory-corruption exploits can reliably jump to known addresses; full randomization forces attackers to either leak addresses first or guess repeatedly.
- **Impact:** Critical
- **NIST SP 800-53:** CM-6(a), SC-30, SC-30(2)
- **CCE:** CCE-87876-9

---

#### UnprivBpfDisabled — `kernel.unprivileged_bpf_disabled`

- **What is it?** Controls whether unprivileged (non-root) users can load BPF programs into the kernel.
- **Good value:** `1` — only users with `CAP_BPF` or `CAP_SYS_ADMIN` can load BPF programs.
- **Bad value:** `0` — any user can load BPF programs.
- **Why it matters:** Unprivileged BPF has been the source of many kernel privilege escalation CVEs; the BPF JIT compiler and verifier are complex attack surfaces. Restricting access removes an entire class of potential exploits.
- **Impact:** High
- **NIST SP 800-53:** AC-6, SC-7(10)
- **CCE:** CCE-89405-5

---

#### PerfEventParanoid — `kernel.perf_event_paranoid`

- **What is it?** Controls which users can use the kernel's performance event interface (`perf_event_open`), which exposes CPU performance counters and hardware events.
- **Good value:** `2` or higher — only privileged users can access performance events.
- **Bad value:** `-1` — all users including unprivileged have full access. `0` or `1` — progressively more access for unprivileged users.
- **Why it matters:** Performance counters can be used as side-channels to leak information about other processes or to bypass ASLR by inferring memory layout.
- **Impact:** High
- **NIST SP 800-53:** AC-6
- **CCE:** CCE-90142-1

---

#### YamaPtraceScope — `kernel.yama.ptrace_scope`

- **What is it?** Controls which processes can attach to another process using `ptrace` (the debugging system call that lets one process read and write another's memory).
- **Good value:** `1` or higher — a process can only ptrace its own children, not arbitrary sibling processes.
- **Bad value:** `0` — any process can ptrace any other process owned by the same user.
- **Why it matters:** An attacker who compromises any process running as your user can extract passwords, private keys, and session tokens from every other process you are running — including your SSH agent, browser, and gpg-agent.
- **Impact:** High
- **NIST SP 800-53:** SC-7(10), AC-6
- **CCE:** CCE-88785-1

---

#### DmesgRestrict — `kernel.dmesg_restrict`

- **What is it?** Controls whether unprivileged users can read the kernel message ring buffer (`dmesg`), which contains boot messages, hardware events, and driver output.
- **Good value:** `1` — only users with `CAP_SYSLOG` can read dmesg.
- **Bad value:** `0` — any user can read kernel messages.
- **Why it matters:** Kernel messages often include capability-related messages and hardware addresses that give attackers reconnaissance data about the system's configuration and security controls.
- **Impact:** Medium
- **NIST SP 800-53:** SI-11(a), SI-11(b)
- **CCE:** CCE-89000-4

---

### GROUP: Kernel Integrity

#### ModulesDisabled — `kernel.modules_disabled`

- **What is it?** A one-way latch: once set to `1`, no further kernel modules can be loaded for the lifetime of the current boot.
- **Good value:** `1` — module loading locked; the kernel's attack surface is frozen.
- **Bad value:** `0` — new kernel modules can be loaded at any time.
- **Why it matters:** A root-level attacker who can load kernel modules can install rootkits, bypass SELinux, and hide their presence. Locking this gate after all needed modules are loaded prevents that class of attack.
- **Impact:** Critical
- **NIST SP 800-53:** CM-7, SI-7
- **CCE:** None (UMRS-only check; exceeds STIG coverage)

---

### GROUP: Process Isolation

#### UnprivUsernsClone — `kernel.unprivileged_userns_clone`

- **What is it?** Controls whether unprivileged users can create new user namespaces, which provide a sandboxed view of user and group IDs.
- **Good value:** `0` — only root can create user namespaces.
- **Bad value:** `1` — any user can create user namespaces.
- **Why it matters:** Unprivileged user namespaces are one of the most commonly exploited kernel features for container escapes and privilege escalation. This is an RHEL-specific sysctl (upstream kernel lacks it).
- **Impact:** High
- **NIST SP 800-53:** SC-39, CM-7
- **CCE:** None (RHEL-specific sysctl)

---

#### Sysrq — `kernel.sysrq`

- **What is it?** Controls which Magic SysRq key combinations are active. SysRq is a keyboard shortcut that can force kernel operations (sync filesystems, kill processes, reboot) bypassing normal access controls.
- **Good value:** `0` — SysRq fully disabled on production servers.
- **Bad value:** Any non-zero value enables some subset of SysRq functions. Full value `1` enables all functions.
- **Why it matters:** On a system with physical console access or a virtual console, SysRq can be used to kill security-relevant processes, dump memory, or force a reboot without authentication.
- **Impact:** Medium
- **Note:** This indicator uses `DesiredValue::Custom` — site policy may permit restricted values (e.g., `176` = sync + remount + reboot only). The TUI renders the live value; assessment requires site-specific validation.
- **NIST SP 800-53:** AC-3, CM-7

---

#### SuidDumpable — `fs.suid_dumpable`

- **What is it?** Controls whether processes running with elevated privileges (SUID binaries) produce core dump files when they crash.
- **Good value:** `0` — SUID and other privileged processes do not produce core dumps.
- **Bad value:** `1` or `2` — privileged processes produce core dumps that may contain passwords, private keys, or other sensitive memory contents.
- **Why it matters:** Core dump files from privileged processes can contain cryptographic key material, cleartext passwords pulled from memory, and session tokens — all written to the filesystem where they may be readable by other users.
- **Impact:** High
- **NIST SP 800-53:** SC-28, SI-12

---

### GROUP: Filesystem Safety

#### ProtectedSymlinks — `fs.protected_symlinks`

- **What is it?** Prevents processes from following symlinks in world-writable sticky directories (like `/tmp`) when the symlink is owned by someone other than the process or the directory owner.
- **Good value:** `1` — symlink following restrictions active.
- **Bad value:** `0` — no restrictions; symlinks in `/tmp` can be followed regardless of ownership.
- **Why it matters:** An attacker who can create files in `/tmp` can create symlinks pointing to sensitive files, then wait for a privileged process to follow them — a classic TOCTOU (time-of-check time-of-use) attack.
- **Impact:** High
- **NIST SP 800-53:** AC-6(1), CM-6(a)
- **CCE:** CCE-88796-8

---

#### ProtectedHardlinks — `fs.protected_hardlinks`

- **What is it?** Prevents processes from creating hard links to files they do not own.
- **Good value:** `1` — hardlink creation restricted to file owners.
- **Bad value:** `0` — any user can hardlink any file they can read, including SUID binaries.
- **Why it matters:** An attacker can hardlink a SUID binary into a directory they control, then exploit the binary's elevated privileges from a path they own — bypassing file permission checks that rely on path-based ownership.
- **Impact:** High
- **NIST SP 800-53:** AC-6(1), CM-6(a)
- **CCE:** CCE-86689-7

---

#### ProtectedFifos — `fs.protected_fifos`

- **What is it?** Prevents privileged processes from writing to FIFOs (named pipes) in world-writable sticky directories that the process does not own.
- **Good value:** `2` — level 2 protection active; privileged processes cannot write to FIFOs they do not own in sticky directories.
- **Bad value:** `0` — no protection. `1` — partial protection.
- **Why it matters:** An attacker can create a FIFO in `/tmp` with a predictable name, wait for a privileged process to write to it (thinking it is a file), and intercept or manipulate that data stream.
- **Impact:** Medium
- **NIST SP 800-53:** SI-10, CM-7

---

#### ProtectedRegular — `fs.protected_regular`

- **What is it?** Prevents privileged processes from writing to regular files in world-writable sticky directories that the process does not own.
- **Good value:** `2` — level 2 protection active.
- **Bad value:** `0` — no protection. `1` — partial protection.
- **Why it matters:** Similar to ProtectedFifos: an attacker can pre-create a file in `/tmp` with a predictable name and wait for a privileged process to overwrite it, potentially replacing a trusted file with attacker-controlled content.
- **Impact:** Medium
- **NIST SP 800-53:** SI-10, CM-7

---

### GROUP: Boot-time / Kernel Cmdline

#### Lockdown — `lockdown=` (SecurityFS)

- **What is it?** The kernel lockdown Linux Security Module (LSM), which restricts operations that could allow root to modify the running kernel — including reading kernel memory, writing to PCI BARs, or loading unsigned modules.
- **Good value:** `integrity` or `confidentiality` present in the lockdown file — lockdown is active.
- **Bad value:** `[none]` — lockdown inactive; root can modify the running kernel.
- **Why it matters:** Without lockdown, a root-level attacker can bypass Secure Boot guarantees by modifying the running kernel after it has been verified, making all boot-time integrity checks irrelevant.
- **Read source:** `/sys/kernel/security/lockdown` (SecurityFS, verified via `SECURITYFS_MAGIC`)
- **Impact:** Critical
- **NIST SP 800-53:** CM-7, SI-7

---

#### ModuleSigEnforce — `module.sig_enforce`

- **What is it?** Requires that all kernel modules be cryptographically signed by a trusted key before they can be loaded.
- **Good value:** `module.sig_enforce=1` present in `/proc/cmdline`.
- **Bad value:** Token absent — unsigned modules can be loaded.
- **Why it matters:** Without module signature enforcement, any code compiled as a kernel module can be loaded by root, defeating lockdown mode and allowing rootkits to be installed. Works in tandem with `ModulesDisabled`.
- **Impact:** Critical
- **NIST SP 800-53:** SI-7, CM-7

---

#### Mitigations — `mitigations=`

- **What is it?** A kernel boot parameter umbrella switch. When set to `off`, disables ALL CPU vulnerability mitigations (Spectre, Meltdown, MDS, and all others) at once.
- **Good value:** `mitigations=off` ABSENT from `/proc/cmdline`.
- **Bad value:** `mitigations=off` PRESENT — every CPU vulnerability mitigation is disabled.
- **Why it matters:** Disabling all mitigations at once is a single configuration change that exposes the system to every known speculative execution attack, allowing cross-process and cross-VM data leakage. This is sometimes done for benchmark performance and should never be present in production.
- **Impact:** Critical
- **NIST SP 800-53:** SI-16, SC-39

---

#### Pti — `pti=`

- **What is it?** Page Table Isolation — the primary mitigation for the Meltdown vulnerability (CVE-2017-5754), which separated kernel and user page tables to prevent user-space code from reading kernel memory.
- **Good value:** `pti=off` ABSENT from `/proc/cmdline` (PTI active by default).
- **Bad value:** `pti=off` PRESENT — PTI explicitly disabled.
- **Why it matters:** Without PTI, any process can read arbitrary kernel memory by exploiting the Meltdown CPU bug, exposing encryption keys, passwords, and any data the kernel has touched.
- **Impact:** High
- **NIST SP 800-53:** SI-16
- **CCE:** CCE-88971-7

---

#### RandomTrustCpu — `random.trust_cpu`

- **What is it?** Controls whether the kernel unconditionally trusts the CPU's hardware random number generator (RDRAND/RDSEED) as the primary entropy source during early boot.
- **Good value:** `random.trust_cpu=on` ABSENT from `/proc/cmdline` (RHEL 10 default: do not unconditionally trust CPU RNG).
- **Bad value:** `random.trust_cpu=on` PRESENT — kernel trusts the CPU RNG exclusively.
- **Why it matters:** NIST SP 800-90B requires entropy sources to be independently assessed. Trusting a CPU RNG unconditionally means that a compromised or backdoored CPU (e.g., with a weakened RNG) can silently weaken all cryptographic key generation. RHEL 10 mixes CPU entropy with other sources by default.
- **Impact:** Medium
- **NIST SP 800-53:** SC-12

---

#### RandomTrustBootloader — `random.trust_bootloader`

- **What is it?** Controls whether the kernel trusts entropy provided by the bootloader during early boot.
- **Good value:** `random.trust_bootloader=on` ABSENT from `/proc/cmdline`.
- **Bad value:** `random.trust_bootloader=on` PRESENT — kernel trusts bootloader-provided entropy seed.
- **Why it matters:** Bootloader-provided entropy is only as trustworthy as the boot chain. Without a verified boot chain (Secure Boot + measured boot), the bootloader could supply a predictable seed, weakening all subsequent cryptographic key generation during that boot.
- **Impact:** Medium
- **NIST SP 800-53:** SC-12, SI-7

---

### GROUP: Special

#### FipsEnabled — `crypto.fips_enabled`

- **What is it?** Indicates whether the kernel is operating in FIPS 140-2/3 mode, which restricts the cryptographic algorithms available to the system to those validated by NIST.
- **Good value:** `1` — FIPS mode active; only validated algorithms are available.
- **Bad value:** `0` — FIPS mode inactive; non-validated algorithms can be used.
- **Why it matters:** DoD and federal deployments require all cryptographic operations to use FIPS-validated algorithms. Without FIPS mode, software may silently use MD5, DES, or other deprecated algorithms for sensitive operations.
- **Impact:** Critical
- **NIST SP 800-53:** SC-13, SC-28
- **FIPS:** 140-2/140-3
- **CMMC:** SC.L2-3.13.10

---

### GROUP: modprobe.d (Phase 2a)

#### NfConntrackAcct — `modprobe:nf_conntrack/acct`

- **What is it?** Enables per-connection byte and packet counters in the netfilter connection tracking system.
- **Good value:** `1` — accounting enabled; each tracked connection records bytes and packets transferred.
- **Bad value:** `0` — accounting disabled; no per-connection traffic data.
- **Why it matters:** Connection tracking accounting feeds audit and firewall logging tools. Without it, network audit records lack the traffic volume data needed for anomaly detection and forensic reconstruction of network activity.
- **Read source:** `/sys/module/nf_conntrack/parameters/acct`
- **Impact:** Medium
- **NIST SP 800-53:** AU-12, CM-6

---

#### BluetoothBlacklisted — `modprobe:bluetooth/blacklisted`

- **What is it?** Verifies that the Bluetooth kernel stack is blacklisted in modprobe configuration so it cannot be loaded, even accidentally.
- **Good value:** Module blacklisted — `/sys/module/bluetooth` directory absent, confirming the module is not loaded.
- **Bad value:** Module present or loadable — Bluetooth stack is active or can be activated.
- **Why it matters:** The Bluetooth protocol stack is a large, complex, historically vulnerability-prone codebase that serves no purpose on server infrastructure. Blacklisting it eliminates this entire attack surface class.
- **Impact:** High
- **NIST SP 800-53:** AC-18(3), AC-18(a), CM-6(a), CM-7(a), CM-7(b), MP-7
- **CMMC:** CM.L2-3.4.6
- **CCE:** CCE-87455-2

---

#### UsbStorageBlacklisted — `modprobe:usb_storage/blacklisted`

- **What is it?** Verifies that the USB mass storage kernel module is blacklisted so external USB drives cannot be mounted.
- **Good value:** Module blacklisted — `/sys/module/usb_storage` directory absent.
- **Bad value:** Module loadable — USB drives can be connected and mounted.
- **Why it matters:** USB storage is a primary data exfiltration vector for classified and government systems. A user or attacker with physical access can copy large amounts of data in seconds. Blacklisting prevents this even if someone physically connects a drive.
- **Impact:** High
- **NIST SP 800-53:** CM-6(a), CM-7(a), CM-7(b), MP-7
- **CMMC:** MP.L2-3.8.7
- **CCE:** CCE-89301-6

---

#### FirewireCoreBlacklisted — `modprobe:firewire_core/blacklisted`

- **What is it?** Verifies that the FireWire (IEEE 1394) kernel module is blacklisted to prevent DMA-based attacks via FireWire ports.
- **Good value:** Module blacklisted — `/sys/module/firewire_core` directory absent.
- **Bad value:** Module loadable — FireWire DMA attacks are possible.
- **Why it matters:** FireWire controllers have direct memory access (DMA) to system RAM by design. An attacker with physical access and a FireWire device can read and write arbitrary memory, bypassing all software security controls including SELinux and encryption.
- **Impact:** High
- **NIST SP 800-53:** SI-7, CM-7

---

#### ThunderboltBlacklisted — `modprobe:thunderbolt/blacklisted`

- **What is it?** Verifies that the Thunderbolt kernel module is blacklisted to prevent DMA attacks via Thunderbolt ports.
- **Good value:** Module blacklisted — `/sys/module/thunderbolt` directory absent.
- **Bad value:** Module loadable — Thunderbolt DMA attacks are possible.
- **Why it matters:** Like FireWire, Thunderbolt uses DMA which can bypass IOMMU protections on some hardware configurations. An attacker with physical access can use a Thunderbolt device to read arbitrary system memory.
- **Impact:** High
- **NIST SP 800-53:** SI-7, CM-7
- **CMMC:** CM.L2-3.4.6

---

### GROUP: CPU Mitigation Sub-Indicators (Phase 2b — catalog-only, not yet in TUI groups)

These nine indicators are defined in the catalog and collected in `PostureSnapshot` but are not yet rendered in a named TUI group. Each one checks that a specific per-CVE mitigation weakening flag is ABSENT from `/proc/cmdline`. They complement the umbrella `Mitigations` indicator (which catches `mitigations=off`) by catching operators who disable individual mitigations without using the umbrella flag.

#### SpectreV2Off — `spectre_v2=off`

- **What is it?** Checks that the Spectre Variant 2 mitigation is not explicitly disabled.
- **Good value:** `spectre_v2=off` ABSENT from cmdline.
- **Bad value:** `spectre_v2=off` PRESENT — branch predictor injection mitigations (retpoline, IBRS, EIBRS) are disabled.
- **Why it matters:** Spectre v2 allows malicious code to inject speculation into the kernel's branch predictor and leak kernel data from other processes.
- **Impact:** High
- **NIST SP 800-53:** SI-16, SC-39

---

#### SpectreV2UserOff — `spectre_v2_user=off`

- **What is it?** Checks that user-space Spectre v2 mitigation is not disabled.
- **Good value:** `spectre_v2_user=off` ABSENT.
- **Bad value:** PRESENT — processes cannot opt in to IBPB/STIBP protections via `prctl`.
- **Why it matters:** Without user-space mitigations, cross-process Spectre v2 speculation attacks between processes on the same CPU are possible.
- **Impact:** Medium
- **NIST SP 800-53:** SI-16, SC-39

---

#### MdsOff — `mds=off`

- **What is it?** Checks that Microarchitectural Data Sampling (MDS) mitigations are not disabled. MDS covers RIDL, Fallout, and ZombieLoad (CVE-2018-12126 and related).
- **Good value:** `mds=off` ABSENT.
- **Bad value:** PRESENT — fill-buffer leakage attacks between processes and the kernel are possible.
- **Why it matters:** MDS attacks can leak data that recently passed through CPU microarchitectural buffers, including encryption keys and credentials processed by the kernel.
- **Impact:** High
- **NIST SP 800-53:** SI-16, SC-39

---

#### TsxAsyncAbortOff — `tsx_async_abort=off`

- **What is it?** Checks that the TSX Async Abort mitigation (CVE-2019-11135) is not disabled.
- **Good value:** `tsx_async_abort=off` ABSENT.
- **Bad value:** PRESENT — data can leak via Intel TSX asynchronous abort events.
- **Why it matters:** On Intel CPUs with TSX support, asynchronous abort events can expose data from other contexts through speculative execution side channels.
- **Impact:** Medium
- **NIST SP 800-53:** SI-16, SC-39

---

#### L1tfOff — `l1tf=off`

- **What is it?** Checks that L1 Terminal Fault (L1TF) mitigations are not disabled. L1TF covers CVE-2018-3615, CVE-2018-3620, and CVE-2018-3646.
- **Good value:** `l1tf=off` ABSENT.
- **Bad value:** PRESENT — L1 data cache contents can leak across VM and process boundaries on affected Intel CPUs.
- **Why it matters:** L1TF can be exploited by a guest VM to read data from the hypervisor or other VMs sharing the same physical CPU, and by user processes to read kernel L1 cache data.
- **Impact:** High
- **NIST SP 800-53:** SI-16, SC-39

---

#### RetbleedOff — `retbleed=off`

- **What is it?** Checks that the RETBLEED mitigation (CVE-2022-29900, CVE-2022-29901) is not disabled.
- **Good value:** `retbleed=off` ABSENT.
- **Bad value:** PRESENT — return address speculation attacks that bypass retpoline are possible on affected CPUs.
- **Why it matters:** RETBLEED exploits a flaw in retpoline (the original Spectre v2 mitigation) itself, allowing attackers to leak kernel data by manipulating CPU return address prediction.
- **Impact:** High
- **NIST SP 800-53:** SI-16, SC-39

---

#### SrbdsOff — `srbds=off`

- **What is it?** Checks that Special Register Buffer Data Sampling (SRBDS) mitigation (CVE-2020-0543) is not disabled.
- **Good value:** `srbds=off` ABSENT.
- **Bad value:** PRESENT — RNG output from special CPU registers can be sampled by other processes.
- **Why it matters:** SRBDS can leak output from the CPU's hardware random number generator (RDRAND), potentially allowing attackers to predict or reconstruct cryptographic keys generated during the attack window.
- **Impact:** Medium
- **NIST SP 800-53:** SI-16, SC-39

---

#### NoSmtOff — `nosmt=off`

- **What is it?** Checks that Simultaneous Multi-Threading (SMT/HyperThreading) has not been re-enabled via `nosmt=off` when the system was otherwise configured to disable it.
- **Good value:** `nosmt=off` ABSENT.
- **Bad value:** PRESENT — SMT has been forcibly re-enabled, weakening MDS, L1TF, and cross-HT side-channel mitigations.
- **Why it matters:** Several CPU vulnerability mitigations are less effective or ineffective when SMT is active because the two logical cores on the same physical core share microarchitectural state that can be observed by the sibling thread.
- **Impact:** Medium
- **NIST SP 800-53:** SI-16, SC-39

---

#### CorePattern — `kernel.core_pattern`

- **What is it?** Configures where and how the kernel writes core dump files (memory snapshots produced when a process crashes).
- **Good value:** Value BEGINS WITH `|` — dumps are piped to a registered handler (e.g., `systemd-coredump`) that enforces access control, compression, and audit logging.
- **Bad value:** Value does NOT begin with `|` — crash dumps are written directly to the filesystem as raw files, potentially in attacker-writable locations.
- **Why it matters:** Core dumps contain process memory, which may include encryption keys, session tokens, and plaintext credentials. Routing them to a controlled handler prevents uncontrolled disclosure and ensures audit trail coverage for every crash.
- **Impact:** High
- **NIST SP 800-53:** SC-7(10), SC-28, CM-6, AU-9
- **CMMC:** SC.L2-3.13.10
- **CCE:** CCE-86714-3

---

## Part 4 — Trust Evidence Tab Reference

### Column Definitions (Evidence Table)

The evidence table in the Trust / Evidence tab has three columns:

| Column | Header | Meaning |
|---|---|---|
| 1 | Evidence Type | The source kind — where this evidence came from (e.g., `procfs`, `regular-file`, `package-db`) |
| 2 | Source | The filesystem path that was read (truncated to 24 characters) |
| 3 | Verification | Whether the data was successfully parsed and what method was used to open it |

### Verification Column Values

| Display | Meaning |
|---|---|
| `✓ ok (fd)` | Parse succeeded; file was opened using an fd-anchored read (TOCTOU-safe) |
| `✓ ok (path)` | Parse succeeded; file was opened by path |
| `✗ FAIL (fd)` | Parse failed; file was opened fd-anchored but content was invalid |
| `✗ FAIL (path)` | Parse failed; file was opened by path but content was invalid |

The open method `fd` vs `path` matters for audit: fd-anchored reads are TOCTOU-safe (the file cannot be swapped between open and read). Path-based reads are not.

### Source Kind Labels

| Label | Meaning |
|---|---|
| `procfs` | Read from `/proc/` filesystem (PROC_SUPER_MAGIC verified) |
| `regular-file` | Read from a regular filesystem file (e.g., `/etc/os-release`) |
| `package-db` | Read from a package manager database (e.g., RPM DB, dpkg status) |
| `symlink-target` | The resolved target of a symbolic link |
| `sysfs` | Read from `/sys/` filesystem (SYSFS_MAGIC verified) |
| `statfs` | Filesystem type verification result from `statfs(2)` |

### Trust Tier Definitions (T0 → T4)

These tiers represent how many independent verification gates the detection pipeline passed. Higher tiers mean more confidence in the identity of the system.

| Tier | Code | Label | What it means |
|---|---|---|---|
| T0 | Untrusted | T0 — Untrusted | No kernel anchor established. The procfs filesystem could not be verified. Do not trust any identity claims. |
| T1 | KernelAnchored | T1 — KernelAnchored | procfs was verified via `fstatfs(PROC_SUPER_MAGIC)` and PID coherence checks passed. The kernel is real, but environment and identity are not yet confirmed. |
| T2 | EnvAnchored | T2 — EnvAnchored | Mount topology was cross-checked (mountinfo vs statfs). The execution environment is known and consistent. |
| T3 | SubstrateAnchored | T3 — SubstrateAnchored | Package substrate was parsed and identity was derived from at least two independent facts. OS identity is likely correct but not cryptographically verified. |
| T4 | IntegrityAnchored | T4 — IntegrityAnchored | `/etc/os-release` ownership was verified AND the installed package digest matched the package database. This is the highest assurance tier. |

**Color coding:**
- T0 → Red (TrustRed)
- T1, T2 → Yellow (TrustYellow) — partial trust established
- T3, T4 → Green (TrustGreen) — sufficient trust for operational use

### Downgrade Reasons — Better Framing

**Current display:** `downgrade reasons: none` (TrustGreen)

**Recommended framing for the positive case:**
- "All verification gates passed" (preferred — describes what happened, not the absence of a problem)
- "No anomalies detected" (acceptable alternative)
- "Integrity pipeline: clean" (brief variant for narrow columns)

The current `"none"` is technically correct but reads as the absence of data rather than a positive outcome. The redesigned display should make it clear that zero downgrade reasons is a good, verified result.

**For the non-zero case (current):** The count is shown in yellow with each reason listed below. This framing is adequate but could be improved by prefixing each reason with its phase label (e.g., `[T1→T2]`) to show which gate triggered the downgrade.

### Contradiction Display

**Current display:** `contradictions: none` (TrustGreen) or `contradictions: N` (TrustRed) with per-contradiction rows.

A contradiction is a case where two independent evidence sources disagree about the same fact. Examples: procfs says kernel version X but `/etc/os-release` says version Y; RPM database records a different file hash than what is on disk.

**Interpretation guidance for operators:**
- Zero contradictions + T4 = highest assurance; system identity is verified and self-consistent.
- Zero contradictions + T3 = identity is consistent but not cryptographically verified.
- Any contradictions = investigate; the system may have been modified, is running an untested configuration, or the detection pipeline has a bug. Contradictions always cause a downgrade.

### Label Trust Field

The label trust field classifies the quality of the OS identity label derived by the pipeline:

| Value | Meaning |
|---|---|
| `UntrustedLabelCandidate` | The label was produced but cannot be trusted — do not use for policy decisions |
| `LabelClaim` | Structurally valid label; integrity not yet confirmed |
| `TrustedLabel` | T4: ownership verified + digest verified — suitable for policy |
| `IntegrityVerifiedButContradictory` | Digest was verified but a contradiction was found — label integrity is uncertain |

---

## Part 5 — Header Indicator Reference (Audit Card Header)

The header shows four security indicators drawn from live kernel reads. These are the top-level security posture signals that appear in every tab.

| Field | Source | Active means | Inactive means |
|---|---|---|---|
| `selinux_status` | `/sys/fs/selinux/enforce` | SELinux is in **Enforcing** mode — MAC policy active | SELinux is in **Permissive** mode — MAC policy logging only, not enforcing |
| `fips_mode` | `/proc/sys/crypto/fips_enabled` | FIPS 140-2/3 mode **active** | FIPS mode **inactive** |
| `active_lsm` | `/sys/kernel/security/lsm` | LSM stack name — not yet implemented | Always Unavailable until implemented |
| `lockdown_mode` | `/sys/kernel/security/lockdown` | Lockdown is active (`integrity` or `confidentiality`) | Lockdown is inactive (`none`) |
| `secure_boot` | UEFI efivars | Secure Boot enabled | Always Unavailable — not yet implemented |

All reads use `SecureReader` with filesystem magic verification. On failure, the field displays `Unavailable` — never a guessed or fabricated value.

---

## Part 6 — Impact Tier Summary

Quick reference for prioritizing remediation:

| Impact | Count in catalog | Meaning |
|---|---|---|
| **Critical** | 6 | Foundational control; failure directly enables serious attacks. Fix immediately. |
| **High** | 20 | Significant hardening; failure provides a useful attack primitive. Fix promptly. |
| **Medium** | 11 | Meaningful improvement; failure has limited but real blast radius. Fix in next maintenance window. |

**Critical indicators by group:**
- KptrRestrict, RandomizeVaSpace (Kernel Self-Protection)
- ModulesDisabled (Kernel Integrity)
- KexecLoadDisabled, Lockdown, ModuleSigEnforce, Mitigations (Boot Integrity)
- FipsEnabled (Special)

---

## Part 7 — Indicators Not Yet Rendered in TUI Groups

The following 9 indicators are defined in the catalog and collected in `PostureSnapshot` but not yet assigned to a TUI display group in `build_kernel_security_rows`. They are available in the snapshot data; they just need a group added to the render function:

- SpectreV2Off, SpectreV2UserOff, MdsOff, TsxAsyncAbortOff, L1tfOff, RetbleedOff, SrbdsOff, NoSmtOff (CPU mitigation sub-indicators — Phase 2b)
- CorePattern (kernel core dump — Phase 2b)

Recommended TUI group name: `CPU VULNERABILITY MITIGATIONS` for the 8 speculative execution indicators, and `CORE DUMP POLICY` (or fold into PROCESS ISOLATION) for CorePattern.

---

## Part 8 — Indicators in TUI Groups But Not in Current Catalog Mapping

None. All TUI-rendered indicators have catalog entries.
