CPU exten layers

Short answer: having a CPU extension does not mean it is being used. Utilization depends on several layers of the software stack. In most cases the extension must be explicitly enabled by the compiler, runtime library, or application code. Only a small subset are transparently used by the system.

To assess whether a platform actually benefits from extensions, you need to think in terms of a three-layer activation model.

⸻

1. Hardware Availability (CPU Capability)

At the lowest layer the processor simply advertises support through CPUID flags.

Examples on x86:

AES-NI
AVX
AVX2
AVX-512
SHA
RDRAND
SGX
BMI1/BMI2
ADX
VAES
VPCLMULQDQ

These show up in:

/proc/cpuinfo
cpuid
lscpu

But this only means the silicon supports the instruction. Nothing is using it yet.

⸻

2. OS Enablement (Kernel Support)

Some extensions require the operating system to enable state management.

Examples:

Extension	OS involvement	Reason
AVX / AVX2	YES	kernel must save/restore vector registers
AVX-512	YES	large register context
SGX	YES	enclave management
PKU	YES	protection keys
AMX	YES	tile state management

The kernel enables these through mechanisms like:

XSAVE
XCR0 register
CR4 flags

If the OS does not enable the feature, software cannot use it even if the CPU supports it.

Example:

cat /proc/cpuinfo | grep avx

But also:

cat /proc/self/status | grep -i xsave


⸻

3. Software Utilization (Compiler / Library / Application)

This is the most important layer.

The majority of extensions are only used if software is compiled to use them.

There are three common patterns.

⸻

A. Compile-time targeting

The compiler generates instructions directly.

Example:

gcc -march=native

or

-RUSTFLAGS="-C target-cpu=native"

or explicitly:

-mavx2
-maes
-msha

If compiled without these flags, the binary will not emit those instructions.

⸻

B. Runtime CPU dispatch

Many high-performance libraries detect CPU features dynamically.

Typical pattern:

if (cpu_supports_avx2())
    use_avx2_implementation();
else
    use_scalar();

Libraries that do this:

Library	Extensions Used
OpenSSL	AES-NI, AVX, AVX2, SHA
libcrypto	VAES, PCLMUL
BLAS libraries	AVX/AVX2/AVX512
zlib-ng	AVX2
Rust std crypto backends	AES-NI

This is the most common modern approach.

⸻

C. Intrinsics / Assembly

Some code directly calls CPU instructions.

Example:

_mm_aesenc_si128()

These compile to AES instructions.

Used in:
	•	cryptography
	•	compression
	•	math kernels
	•	SIMD libraries

⸻

4. Extensions That Are Automatically Used

Only a few categories are often used automatically through libraries.

AES-NI

Most crypto libraries automatically detect and use it.

Example:

OpenSSL
BoringSSL
libsodium
Rust ring

SHA extensions

Used automatically if compiled into crypto libraries.

RDRAND / RDSEED

Used by kernel entropy pools or crypto libraries.

PCLMULQDQ

Used automatically by AES-GCM implementations.

⸻

5. Extensions Rarely Used Without Explicit Enablement

These require intentional optimization.

Extension	Typical Usage
AVX512	HPC / AI / compression
AMX	AI matrix workloads
BMI1/BMI2	bit manipulation optimizations
ADX	big integer math
SHA512 extensions	crypto libraries
VAES	vector AES

If software was compiled generically, none of these will be used.

⸻

6. Why Vendors Ship Many Extensions That Are Often Idle

Because software distribution must remain portable.

Example:

A Linux binary compiled with:

-march=x86-64

must run on CPUs without AVX or AES.

So vendors build binaries targeting baseline CPUs.

Only specialized builds use advanced instructions.

⸻

7. Modern Solution: Multi-version Binaries

Many libraries now compile multiple optimized versions.

Example inside OpenSSL:

AES scalar
AES-NI
AES-NI + AVX
VAES + AVX512

At runtime:

CPUID -> select fastest implementation

This gives maximum performance while staying portable.

⸻

8. For Your CPU Audit Project

When evaluating a platform you should record three distinct states:

Layer 1 — CPU Capability

cpuid flags

Example:

aes avx avx2 sha bmi2 rdrand


⸻

Layer 2 — OS Enablement

Kernel state:

xsave
avx enabled
avx512 enabled


⸻

Layer 3 — Software Usage

Detect if libraries actually use them.

Examples:

openssl speed -evp aes-256-gcm

Check:

OPENSSL_ia32cap

Or inspect binaries:

objdump -d binary | grep aesenc


⸻

9. High-Assurance Insight (Important)

For a security evaluation platform like what you’re building for UMRS, the real questions are:
	1.	Does the CPU support the instruction?
	2.	Did the kernel enable it?
	3.	Do the cryptographic libraries actually use it?
	4.	Was the software compiled to take advantage of it?

Only when all four are true do you get the benefit.

⸻

10. A Practical Mental Model

Think of extensions as:

CPU capability
    ↓
OS enablement
    ↓
compiler support
    ↓
library implementation
    ↓
application usage

Failure at any level means the extension is effectively unused.

⸻

If you’d like, I can also show you something extremely useful for your research agent:

the 20–25 CPU extensions that actually matter for high-assurance systems (cryptography, isolation, memory safety, performance).

Most CPUs advertise hundreds, but only a small subset truly matter for security platforms like the one you’re building.
