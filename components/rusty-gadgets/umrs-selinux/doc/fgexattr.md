In High-Assurance (HA) engineering and NSA Raise the Bar (RTB) standards, fgetxattr provides provenance because it anchors the security decision to a Verified File Descriptor rather than a mutable, spoofable string path.
Here is why this specific syscall is the "Gold Standard" for your UMRS project:
1. Elimination of TOCTOU (Time-of-Check to Time-of-Use)
If you use a path-based call like getxattr("/etc/passwd", ...):
Check: Your code verifies the file exists.
Window of Vulnerability: In the microsecond between the check and the read, an attacker replaces /etc/passwd with a symbolic link to a malicious file.
Use: The kernel follows the link and returns the wrong security label.
The fgetxattr Solution: By using a File object (a file descriptor), you are talking to a specific inode already opened by the kernel. Even if an attacker renames the file or swaps the path on disk, the descriptor remains pinned to the original data. The provenance is locked at the moment of File::open.
2. Bypass of the "Userland Proxy" (TCB Minimization)
Standard libraries often wrap syscalls in "helpers" that might:
Cache results (giving you stale, unverified data).
Use internal buffers that could be subject to heap-spraying or memory corruption.
Intercept calls via LD_PRELOAD shims.
The rustix (linux_raw) Benefit: By using raw syscalls, we are talking directly to the Kernel LSM (Linux Security Module). We are reading "byte-for-byte" exactly what the filesystem driver (e.g., XFS or EXT4) extracted from the disk's physical blocks. There is no "middleman" to lie to us.
3. Atomic Metadata Retrieval
When we read a security context via fgetxattr, the kernel treats the extended attribute as an atomic unit.
Corruption Check: If a disk error occurred, the syscall fails.
Partial Read Protection: Unlike reading a flat file where you might get a partial buffer if the file is being written to, fgetxattr returns the entire label or nothing.
4. Why "Byte-for-Byte" is Critical for RTB
When we say we are reading "byte-for-byte," it means we aren't letting a C library interpret the data as a "string" first.
The "Null Byte" Attack: In C, a \0 terminates a string. An attacker could put unconfined_u:object_r:admin_t\0:s0:c0 in an xattr. A standard C-library parser might stop at the \0 and tell you the type is admin_t, ignoring the rest of the malicious payload.
Our UMRS Strategy: We get the raw Vec<u8>. We see the \0. Our Redundant (TPI) Parsers will look at those raw bytes and, because they are independent, they will likely disagree on how to handle that hidden null byte. Result: Fail-Closed.
Summary for your 30-Year Career Perspective:
In your career, you've seen systems trust the "filename." In UMRS, we trust the Handle.
fgetxattr ensures that the Security Context we are about to parse actually belongs to the Object we are currently holding. It is the "Chain of Custody" for a digital asset.
