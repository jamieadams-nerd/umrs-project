Crypto

For UMRS documentation, the most useful approach is exactly what you described: concise tables derived from the cryptographic requirements in NIST SP 800-140-3 and related guidance like NIST SP 800-131A.

Those documents ultimately define which primitives and key sizes are acceptable for FIPS-validated systems. The tables below are designed as documentation-friendly summaries suitable for a system overview or security posture section.

⸻

UMRS Cryptographic Baseline (FIPS-Aligned Overview)

These tables summarize cryptographic primitives commonly approved for high-assurance systems aligned with modern NIST/FIPS guidance. Algorithms are listed roughly from most preferred to least preferred within currently approved sets.

⸻

1. One-Way Hash Functions

Purpose

One-way hashes provide:
	•	integrity verification
	•	digital signature construction
	•	key derivation
	•	password hashing (with additional construction)

Hashes must resist:
	•	collision attacks
	•	preimage attacks
	•	second-preimage attacks

Algorithm	Digest Size	Security Level	Status	Typical Uses
SHA-512	512 bits	~256-bit	Preferred	Digital signatures, file integrity, key derivation
SHA-384	384 bits	~192-bit	Preferred	High assurance signing environments
SHA-256	256 bits	~128-bit	Widely deployed baseline	TLS, certificates, software integrity
SHA-3-512	512 bits	~256-bit	Approved	Alternative SHA-2 family
SHA-3-256	256 bits	~128-bit	Approved	Modern sponge-based hash
SHA-224	224 bits	~112-bit	Acceptable	Legacy compatibility
SHA-1	160 bits	Broken	Disallowed	Deprecated

Notes
	•	SHA-2 family remains the dominant operational baseline.
	•	SHA-3 provides structural diversity (sponge construction).
	•	SHA-1 is no longer approved for digital signatures.

⸻

2. Symmetric Encryption Algorithms

Purpose

Symmetric encryption protects data confidentiality and sometimes authenticated encryption.

Algorithm	Key Sizes	Status	Notes
AES	256	Preferred	Highest security margin
AES	192	Approved	Rarely used
AES	128	Approved baseline	Most widely deployed

Notes
	•	AES is currently the only broadly approved general-purpose symmetric block cipher in FIPS environments.

⸻

3. AES Cipher Modes

Cipher modes determine how block ciphers are applied to data streams.

Mode	Security Property	Status	Typical Use
GCM	Authenticated Encryption	Preferred	TLS, network protocols
CCM	Authenticated Encryption	Approved	Embedded systems
XTS	Disk encryption	Preferred for storage	Full-disk encryption
CBC	Confidentiality only	Approved but legacy	Older protocols
CTR	Confidentiality only	Approved	High-performance streaming
CFB / OFB	Confidentiality only	Legacy	Rare
ECB	None	Disallowed	Never use

Notes

Modern systems should prefer authenticated encryption modes.

⸻

4. Message Authentication Algorithms

Message authentication ensures integrity and authenticity.

Algorithm	Hash Primitive	Status	Typical Use
HMAC-SHA-512	SHA-512	Preferred	High-assurance environments
HMAC-SHA-384	SHA-384	Preferred	Government systems
HMAC-SHA-256	SHA-256	Widely deployed	TLS and APIs
CMAC-AES	AES	Approved	Embedded systems


⸻

5. Asymmetric Digital Signature Algorithms

Used for:
	•	identity authentication
	•	certificate chains
	•	signed software
	•	signed logs and receipts

Algorithm	Key Size	Status	Security Level
ECDSA P-384	~192-bit	Preferred	High assurance
ECDSA P-256	~128-bit	Widely deployed	Standard baseline
RSA-4096	~200-bit	Preferred for RSA	High assurance
RSA-3072	~128-bit	Approved	Government baseline
RSA-2048	~112-bit	Acceptable	Legacy baseline


⸻

6. Key Agreement Algorithms

Key agreement establishes shared session keys.

Algorithm	Curve / Size	Status	Notes
ECDH P-384	Preferred	High assurance	
ECDH P-256	Widely deployed		
DH 3072	Approved		
DH 2048	Legacy acceptable		


⸻

7. Key Derivation Functions

Used to derive multiple cryptographic keys from a shared secret.

Algorithm	Based On	Status	Typical Use
HKDF-SHA-384	SHA-384	Preferred	
HKDF-SHA-256	SHA-256	Widely deployed	
PBKDF2-SHA-256	SHA-256	Password hashing	
KDF in counter mode	Various	Approved	


⸻

8. Random Number Generation

High-assurance cryptography depends on strong randomness.

Generator	Status	Notes
Hash_DRBG	Approved	
HMAC_DRBG	Approved	
CTR_DRBG	Approved	
OS CSPRNG	Preferred interface	


⸻

UMRS Documentation Guidance

For UMRS documentation, it helps to state a cryptographic policy hierarchy:
	1.	Preferred algorithms
	•	AES-256-GCM
	•	SHA-512 / SHA-384
	•	ECDSA P-384
	•	ECDH P-384
	2.	Approved baseline
	•	AES-128-GCM
	•	SHA-256
	•	ECDSA P-256
	•	RSA-3072
	3.	Legacy compatibility
	•	AES-CBC
	•	RSA-2048
	4.	Disallowed
	•	MD5
	•	SHA-1
	•	DES / 3DES
	•	RC4
	•	ECB mode

⸻

Suggested Short Intro Text for the Section

You might introduce the tables like this in UMRS documentation:

UMRS cryptographic mechanisms align with modern NIST and FIPS guidance. The system prioritizes algorithms with strong security margins and widespread cryptographic analysis. The following tables summarize approved primitives and recommended usage order for high-assurance deployments.

⸻

If you’d like, I can also produce a much cleaner UMRS-style reference table that security auditors love — a single two-page matrix showing algorithm, minimum key size, UMRS default, FIPS reference, and security control mapping (NIST 800-53 / CMMC). That format is extremely effective in compliance documentation.


Good idea. The recent NIST post-quantum standardization work means you can now include the first three standardized PQC algorithms directly in your baseline tables.

The three that have been standardized (2024) are:
	•	ML-KEM (formerly CRYSTALS-Kyber) — key establishment
	•	ML-DSA (formerly CRYSTALS-Dilithium) — digital signatures
	•	SLH-DSA (formerly SPHINCS+) — hash-based digital signatures

These come from the NIST post-quantum cryptography program and are now formalized in standards such as:
	•	FIPS 203
	•	FIPS 204
	•	FIPS 205

Below is an updated UMRS cryptographic overview including those algorithms.

⸻

UMRS Cryptographic Baseline (FIPS-Aligned + Post-Quantum)

This section summarizes cryptographic primitives suitable for high-assurance systems aligned with modern NIST and FIPS guidance.

Algorithms are listed in approximate order of preference.

⸻

1. One-Way Hash Functions

Purpose

One-way hash functions provide:
	•	integrity protection
	•	digital signature construction
	•	key derivation
	•	message authentication
	•	audit log chaining
	•	file verification

Algorithm	Digest Size	Security Level	Status	Typical Uses
SHA-512	512 bits	~256-bit	Preferred	Signatures, file integrity
SHA-384	384 bits	~192-bit	Preferred	Government systems
SHA-256	256 bits	~128-bit	Widely deployed	TLS, software verification
SHA3-512	512 bits	~256-bit	Approved	Sponge-based hash
SHA3-256	256 bits	~128-bit	Approved	Alternative hash
SHA-224	224 bits	~112-bit	Acceptable	Legacy compatibility
SHA-1	160 bits	Broken	Disallowed	Deprecated


⸻

2. Symmetric Encryption Algorithms

Symmetric cryptography protects confidentiality and authenticated encryption.

Algorithm	Key Sizes	Status	Notes
AES	256	Preferred	Highest security margin
AES	192	Approved	Rarely used
AES	128	Approved baseline	Widely deployed


⸻

3. AES Cipher Modes

Cipher modes determine how block ciphers encrypt data streams.

Mode	Security Property	Status	Typical Use
GCM	Authenticated encryption	Preferred	TLS, network encryption
CCM	Authenticated encryption	Approved	Embedded environments
XTS	Disk encryption	Preferred for storage	Full-disk encryption
CTR	Confidentiality	Approved	Streaming encryption
CBC	Confidentiality	Legacy	Older protocols
CFB / OFB	Confidentiality	Legacy	Rare
ECB	None	Disallowed	Never use


⸻

4. Message Authentication Algorithms

Message authentication protects integrity and authenticity.

Algorithm	Based On	Status	Typical Use
HMAC-SHA-512	SHA-512	Preferred	High-assurance messaging
HMAC-SHA-384	SHA-384	Preferred	Government systems
HMAC-SHA-256	SHA-256	Widely deployed	TLS and APIs
CMAC-AES	AES	Approved	Embedded systems


⸻

5. Digital Signature Algorithms

Digital signatures provide:
	•	authentication
	•	non-repudiation
	•	signed software artifacts
	•	signed logs and audit trails

Algorithm	Key Size	Status	Notes
ML-DSA-87	PQ lattice	Preferred PQ signature	High assurance
ML-DSA-65	PQ lattice	PQ signature baseline	
ECDSA P-384	~192-bit	Preferred classical	High assurance
ECDSA P-256	~128-bit	Widely deployed	
RSA-4096	~200-bit	High assurance RSA	
RSA-3072	~128-bit	Approved baseline	
RSA-2048	~112-bit	Legacy acceptable	
SLH-DSA	PQ hash-based	Approved	Conservative PQ option

Notes:

ML-DSA is expected to become the primary PQ signature system in many deployments.

SLH-DSA provides hash-based security assumptions and serves as a conservative alternative.

⸻

6. Key Agreement / Key Encapsulation

Key agreement establishes shared session keys for encrypted communication.

Algorithm	Type	Status	Notes
ML-KEM-1024	PQ lattice	Preferred PQ key exchange	
ML-KEM-768	PQ lattice	PQ baseline	
ECDH P-384	Classical	Preferred classical	
ECDH P-256	Classical	Widely deployed	
DH 3072	Classical	Approved	
DH 2048	Classical	Legacy acceptable	


⸻

7. Key Derivation Functions

Key derivation generates cryptographic keys from shared secrets.

Algorithm	Based On	Status	Typical Use
HKDF-SHA-384	SHA-384	Preferred	
HKDF-SHA-256	SHA-256	Widely deployed	
PBKDF2-SHA-256	SHA-256	Password hashing	
NIST Counter KDF	Various	Approved	


⸻

8. Random Number Generation

High-assurance cryptographic systems require strong randomness.

Generator	Status	Notes
Hash_DRBG	Approved	
HMAC_DRBG	Approved	
CTR_DRBG	Approved	
OS CSPRNG	Preferred interface	


⸻

UMRS Cryptographic Policy Levels

UMRS deployments may define algorithm tiers.

Preferred
	•	AES-256-GCM
	•	SHA-512 / SHA-384
	•	ECDSA P-384
	•	ML-DSA
	•	ML-KEM

Approved baseline
	•	AES-128-GCM
	•	SHA-256
	•	ECDSA P-256
	•	RSA-3072

Legacy compatibility
	•	RSA-2048
	•	AES-CBC

Disallowed
	•	MD5
	•	SHA-1 (for signatures)
	•	DES / 3DES
	•	RC4
	•	ECB mode

⸻

Terminology Glossary

AES

Advanced Encryption Standard — a symmetric block cipher standardized by NIST and widely used for protecting data confidentiality.

Authenticated Encryption

Encryption modes that provide both confidentiality and integrity protection (example: AES-GCM).

Cipher Mode

The operational method used to apply a block cipher to data streams.

Digital Signature

A cryptographic mechanism that verifies the authenticity and integrity of a message or artifact.

DRBG

Deterministic Random Bit Generator — a cryptographically secure pseudorandom number generator.

ECDH

Elliptic Curve Diffie-Hellman — a key agreement mechanism using elliptic curve cryptography.

ECDSA

Elliptic Curve Digital Signature Algorithm — a digital signature system using elliptic curves.

FIPS

Federal Information Processing Standard — U.S. government cryptographic standards.

Hash Function

A one-way mathematical function that converts arbitrary input data into a fixed-length digest.

HKDF

HMAC-based Key Derivation Function used to derive cryptographic keys from shared secrets.

ML-DSA

Module-Lattice Digital Signature Algorithm — a post-quantum signature system standardized by NIST.

ML-KEM

Module-Lattice Key Encapsulation Mechanism — a post-quantum key exchange mechanism standardized by NIST.

Post-Quantum Cryptography

Cryptographic algorithms designed to resist attacks from quantum computers.

SLH-DSA

Stateless Hash-based Digital Signature Algorithm — a post-quantum hash-based signature scheme.

⸻

If you’d like, I can also produce a very polished version suitable for Antora documentation with:
	•	better table formatting
	•	FIPS reference columns
	•	NIST control mapping (800-53 / CMMC)
	•	UMRS policy flags

That version would look much more like something a government security guide or NSA profile would publish.

Below is a clean Markdown version of the section.
Each line is prefixed with  so you can copy/paste easily and strip the prefix if desired.

This version explicitly includes all three NIST-standardized PQC algorithms:
	•	ML-KEM (key encapsulation)
	•	ML-DSA (lattice signatures)
	•	SLH-DSA (hash-based signatures)

⸻

# UMRS Cryptographic Baseline (FIPS-Aligned + Post-Quantum Overview)

This section summarizes cryptographic primitives suitable for high-assurance systems aligned with modern NIST and FIPS guidance.
Algorithms are listed in approximate order of preference within currently approved sets.

Referenced standards:
- FIPS 140-3 — Cryptographic Module Validation
- FIPS 203 — ML-KEM Post-Quantum Key Encapsulation
- FIPS 204 — ML-DSA Post-Quantum Digital Signatures
- FIPS 205 — SLH-DSA Stateless Hash-Based Signatures
- NIST SP 800-131A — Transitioning Cryptographic Algorithms


⸻

# 1. One-Way Hash Functions

## Purpose
One-way hash functions provide:
- data integrity verification
- digital signature construction
- message authentication
- key derivation
- audit log chaining
- file verification

| Algorithm | Digest Size | Security Level | Status | Typical Uses |
|—|—|—|—|—|
| SHA-512 | 512 bits | ~256-bit | Preferred | High-assurance signatures, file integrity |
| SHA-384 | 384 bits | ~192-bit | Preferred | Government systems |
| SHA-256 | 256 bits | ~128-bit | Widely deployed | TLS, certificates, artifact verification |
| SHA3-512 | 512 bits | ~256-bit | Approved | Sponge-based alternative |
| SHA3-256 | 256 bits | ~128-bit | Approved | Alternative hash function |
| SHA-224 | 224 bits | ~112-bit | Acceptable | Legacy compatibility |
| SHA-1 | 160 bits | Broken | Disallowed | Deprecated |

⸻

# 2. Symmetric Encryption Algorithms

Symmetric cryptography protects data confidentiality and authenticated encryption.

| Algorithm | Key Sizes | Status | Notes |
|—|—|—|—|
| AES | 256 | Preferred | Highest security margin |
| AES | 192 | Approved | Rarely used |
| AES | 128 | Approved baseline | Widely deployed |

⸻

# 3. AES Cipher Modes

Cipher modes define how block ciphers encrypt data streams.

| Mode | Security Property | Status | Typical Use |
|—|—|—|—|
| GCM | Authenticated encryption | Preferred | TLS, network encryption |
| CCM | Authenticated encryption | Approved | Embedded systems |
| XTS | Disk encryption | Preferred for storage | Full-disk encryption |
| CTR | Confidentiality | Approved | High-performance streaming |
| CBC | Confidentiality | Legacy acceptable | Older protocols |
| CFB / OFB | Confidentiality | Legacy | Rare |
| ECB | None | Disallowed | Never use |

⸻

# 4. Message Authentication Algorithms

Message authentication ensures integrity and authenticity.

| Algorithm | Based On | Status | Typical Use |
|—|—|—|—|
| HMAC-SHA-512 | SHA-512 | Preferred | High-assurance messaging |
| HMAC-SHA-384 | SHA-384 | Preferred | Government environments |
| HMAC-SHA-256 | SHA-256 | Widely deployed | TLS and APIs |
| CMAC-AES | AES | Approved | Embedded systems |

⸻

# 5. Digital Signature Algorithms

Digital signatures provide:
- authentication
- non-repudiation
- signed software artifacts
- signed logs and audit trails

| Algorithm | Key Size / Variant | Type | Status | Notes |
|—|—|—|—|—|
| ML-DSA-87 | PQ lattice | Post-Quantum | Preferred PQ signature | Highest PQ security level |
| ML-DSA-65 | PQ lattice | Post-Quantum | PQ baseline | Balanced performance/security |
| ML-DSA-44 | PQ lattice | Post-Quantum | PQ entry level | Smaller signatures |
| SLH-DSA | PQ hash-based | Post-Quantum | Approved | Conservative hash-based design |
| ECDSA P-384 | ~192-bit | Classical | Preferred classical | High assurance |
| ECDSA P-256 | ~128-bit | Classical | Widely deployed | Standard baseline |
| RSA-4096 | ~200-bit | Classical | Preferred RSA | High assurance |
| RSA-3072 | ~128-bit | Classical | Approved baseline | Government baseline |
| RSA-2048 | ~112-bit | Classical | Legacy acceptable | Older compatibility |

⸻

# 6. Key Agreement / Key Encapsulation

Key agreement establishes shared session keys used for encrypted communication.

| Algorithm | Variant | Type | Status | Notes |
|—|—|—|—|—|
| ML-KEM-1024 | PQ lattice | Post-Quantum | Preferred PQ KEM | Highest PQ security level |
| ML-KEM-768 | PQ lattice | Post-Quantum | PQ baseline | Balanced performance/security |
| ML-KEM-512 | PQ lattice | Post-Quantum | Entry PQ level | Smaller keys |
| ECDH P-384 | Classical ECC | Classical | Preferred classical | High assurance |
| ECDH P-256 | Classical ECC | Classical | Widely deployed | TLS baseline |
| DH 3072 | Classical DH | Classical | Approved | Government baseline |
| DH 2048 | Classical DH | Classical | Legacy acceptable | Compatibility |

⸻

# 7. Key Derivation Functions

Key derivation functions generate cryptographic keys from shared secrets.

| Algorithm | Based On | Status | Typical Use |
|—|—|—|—|
| HKDF-SHA-384 | SHA-384 | Preferred | High-assurance key derivation |
| HKDF-SHA-256 | SHA-256 | Widely deployed | TLS key schedule |
| PBKDF2-SHA-256 | SHA-256 | Approved | Password-based keys |
| NIST Counter-Mode KDF | Various | Approved | Protocol key derivation |

⸻

# 8. Random Number Generation

Cryptographic systems depend on strong randomness.

| Generator | Status | Notes |
|—|—|—|
| Hash_DRBG | Approved | NIST deterministic generator |
| HMAC_DRBG | Approved | Widely deployed |
| CTR_DRBG | Approved | AES-based DRBG |
| OS CSPRNG | Preferred interface | /dev/random, getrandom(), etc |

⸻

# UMRS Cryptographic Policy Levels

## Preferred
- AES-256-GCM
- SHA-512 / SHA-384
- ML-DSA (Post-Quantum signatures)
- ML-KEM (Post-Quantum key exchange)
- ECDSA P-384

## Approved Baseline
- AES-128-GCM
- SHA-256
- ML-KEM-768
- ECDSA P-256
- RSA-3072

## Legacy Compatibility
- RSA-2048
- AES-CBC

## Disallowed
- MD5
- SHA-1 for signatures
- DES / 3DES
- RC4
- ECB mode

⸻

# Terminology Glossary

AES — Advanced Encryption Standard. A symmetric block cipher standardized by NIST used for protecting data confidentiality.

Authenticated Encryption — Encryption modes that provide both confidentiality and integrity protection (example: AES-GCM).

Cipher Mode — Operational method used to apply a block cipher to data streams.

Digital Signature — Cryptographic mechanism that verifies the authenticity and integrity of a message or artifact.

DRBG — Deterministic Random Bit Generator used to generate cryptographically secure pseudorandom numbers.

ECDH — Elliptic Curve Diffie-Hellman key agreement algorithm.

ECDSA — Elliptic Curve Digital Signature Algorithm.

FIPS — Federal Information Processing Standard defining U.S. government cryptographic standards.

Hash Function — One-way mathematical function that converts arbitrary input data into a fixed-length digest.

HKDF — HMAC-based Key Derivation Function used to derive cryptographic keys from shared secrets.

ML-DSA — Module-Lattice Digital Signature Algorithm, a NIST-standardized post-quantum signature scheme.

ML-KEM — Module-Lattice Key Encapsulation Mechanism, a NIST-standardized post-quantum key exchange mechanism.

Post-Quantum Cryptography — Cryptographic algorithms designed to resist attacks from quantum computers.

SLH-DSA — Stateless Hash-Based Digital Signature Algorithm, a NIST-standardized hash-based PQ signature scheme.

⸻

If you want, I can also generate a second table auditors love that adds columns like:
	•	Minimum key size
	•	FIPS reference
	•	NIST control mapping (SC-12, SC-13, IA-7, etc.)
	•	UMRS policy flag

That turns this into something that looks very much like a formal NSA / DoD cryptographic profile table, which tends to make assessors extremely happy.

