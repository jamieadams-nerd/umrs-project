This is the Master Technical Specification. I have integrated the Deterministic Bitmask Mandate, specifically highlighting the transition from expensive, non-deterministic string/collection lookups to 
 bitwise operations for CategorySet and Sensitivity lattice math.
umrs_selinux: High-Assurance SELinux Reference Monitor Substrate
Author: Jamie Adams (Imodium Operator)
Target Platform: Red Hat Enterprise Linux 10 (Kernel 6.x+)
Compliance Mandate: NIST SP 800-53 Rev. 5 (High-Impact)
Engineering Standard: NSA "Raise the Bar" (RTB) / RAIN Principles
I. Architectural Philosophy: The Reference Monitor
The umrs_selinux package is engineered as a Formal Reference Monitor. To satisfy NSA RTB mandates, a Reference Monitor must be Non-Bypassable, Tamperproof, and Verifiable.
1. Provenance Verification via rustix (NIST 800-53 SI-7 / AC-3)
Traditional security tools rely on path-based lookups (e.g., /etc/shadow), which are vulnerable to TOCTOU (Time-of-Check to Time-of-Use) symlink races.
The rustix Invariant: umrs_selinux utilizes the rustix crate’s linux_raw backend to execute direct fgetxattr syscalls, bypassing libc entirely.
Handle-Based Mediation: Access decisions are anchored to the File Descriptor (FD). By requiring an open std::fs::File handle, the monitor ensures that the security label is physically and logically pinned to the specific inode being mediated.
2. Deterministic Memory: Stack-First Logic vs. Heap Vulnerability
To eliminate heap-fragmentation risks and Use-After-Free (UAF) vulnerabilities common in C-based security stacks:
Stack-Based Bounding: Standard status flags and fixed-length attributes are processed entirely on the stack.
Bounded Heap Fallback: For variable-length security contexts, we perform a Deterministic Bounding Probe using fgetxattr_size before allocation. We only allocate a strictly-sized Vec<u8> after validating the kernel's reported size against our internal TCB limits.
II. Computational Rigor: 
 Bitmask Mandate (NIST 800-53 AC-4)
A core requirement for High-Assurance Reference Monitors is Deterministic Execution Time. Relying on string comparisons or dynamic collections for security decisions introduces non-deterministic timing and potential side-channel vulnerabilities.
Lattice Bitmasking: umrs_selinux mandates the use of bitmasks for all category-based decisions.
CategorySet (1024-bit Bitmask): MCS/MLS categories are modeled as a fixed-size [u64; 16] array.
Constant-Time Dominance: Information flow mediation (e.g., "Is Category A a subset of Category B?") is performed via bitwise AND and OR operations. This ensures that a dominance check takes the same amount of time regardless of whether 1 or 1000 categories are present, satisfying NSA RTB requirements for Deterministic Performance.
III. Implementation Rigor: NSA RTB "RAIN" Principles
1. Redundant TPI Parsing (Two-Person Integrity)
No single parsing algorithm is trusted as the sole arbiter of a security decision. umrs_selinux implements Two-Path (TPI) Logic Agreement:
Path A (Declarative/Formal): A nom-based combinator parser that decomposes the label using strictly bounded, zero-copy combinators on stack-slices.
Path B (Imperative/Manual): A manual split and match strategy providing a traditional "vernacular" check.
The Fail-Closed Gate: If Path A and Path B do not achieve bit-for-bit equality in their resulting SecurityContext objects, the system Fails-Closed with a PermissionDenied error.
2. Byte-for-Byte Integrity (TCB Minimization)
By bypassing libselinux.so, we retrieve raw bytes directly from the kernel LSM.
Null-Byte Protection: We process the full byte array, preventing "Null-Terminator Attacks" where a C-string parser might ignore malicious MLS categories hidden behind a \0 character.
Vernacular Mapping: The engine dynamically resolves RHEL 10 aliases (e.g., SystemLow) into canonical mathematical forms (s0) during the parsing phase.
IV. Technical Specification Summary
Safety: 100% Safe Rust; zero unsafe blocks in the modeling layer.
Memory Model: Stack-preferred; strictly bounded heap-fallback.
Computation: Fixed-size Bitmasks ([u64; 16]) for 
 lattice math.
Syscall Interface: Direct linux_raw syscalls via rustix.
Parsing: Redundant TPI (nom + imperative).
Security Audit Verification
To verify the integrity of a security decision:
Trace the SecureXattrReader handle-based provenance.
Validate the TPI Gate agreement between independent parsers.
Observe the transition from human-readable strings to Strongly Typed Bitmasks.
Confirm the use of constant-time bitwise logic for Lattice Dominance mediation.
The result is a Reference Monitor that is mathematically sound, computationally deterministic, and physically anchored to the filesystem integrity.


