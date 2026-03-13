# Task: PQC Documentation — Post-Quantum Cryptography Coverage

**Created by:** researcher
**Date:** 2026-03-13
**Assigned to:** senior-tech-writer, tech-writer
**Status:** open
**Blocked by:** Antora doc restructure (Phase 3 — new content phase)

---

## Background

Three NIST post-quantum cryptography FIPS standards were downloaded and ingested into the
RAG on 2026-03-13:

- FIPS 203 (ML-KEM / CRYSTALS-Kyber) — `refs/nist/fips/fips203.pdf`
- FIPS 204 (ML-DSA / CRYSTALS-Dilithium) — `refs/nist/fips/fips204.pdf`
- FIPS 205 (SLH-DSA / SPHINCS+) — `refs/nist/fips/fips205.pdf`

Additionally, 10 supplementary web articles were ingested into the `nist-pqc` RAG collection
(`.claude/references/nist-pqc/web/`). The collection now has 264 chunks.

The crypto reference page at `docs/modules/reference/pages/crypto-post-quantum.adoc` already
exists as a stub. The fips-cryptography-cheat-sheet.adoc references FIPS 203/204/205.

---

## Required Documentation

Produce PQC documentation covering the following topics. Final placement is at the
senior-tech-writer's discretion (likely `docs/modules/reference/pages/` given the Antora restructure).

### 1. Emergence of Post-Quantum Cryptography

- Why classical public-key cryptography is vulnerable to quantum computers
- Shor's algorithm: breaks RSA (integer factorization) and ECDH/ECDSA (elliptic curve discrete log)
- Grover's algorithm: reduces symmetric key security by half (AES-256 remains adequate; AES-128 does not)
- The "harvest now, decrypt later" threat model
- Timeline to cryptographically-relevant quantum computers (CRQC): expert consensus 5-15 years
- NIST's 8-year standardization effort (2016-2024): 82 candidates, 4 rounds, 3 finalized

### 2. The Three FIPS Standards

#### FIPS 203 — ML-KEM (Module-Lattice-Based Key-Encapsulation Mechanism)
- Based on CRYSTALS-Kyber (CRYSTALS = Cryptographic Suite for Algebraic Lattices)
- Mathematical foundation: Module Learning With Errors (MLWE) problem
- Parameter sets: ML-KEM-512 (128-bit), ML-KEM-768 (192-bit), ML-KEM-1024 (256-bit)
- Role: replaces RSA key exchange and ECDH key agreement
- API distinction: KEM (not NIKE) — different from ECDH semantics

#### FIPS 204 — ML-DSA (Module-Lattice-Based Digital Signature Standard)
- Based on CRYSTALS-Dilithium
- Mathematical foundation: Module Learning With Errors (MLWE) + Module Short Integer Solution
- Parameter sets: ML-DSA-44 (128-bit), ML-DSA-65 (192-bit), ML-DSA-87 (256-bit)
- Role: primary replacement for RSA signatures and ECDSA
- Provides: authentication, data integrity, non-repudiation

#### FIPS 205 — SLH-DSA (Stateless Hash-Based Digital Signature Standard)
- Based on SPHINCS+
- Mathematical foundation: hash function security properties only — no lattice assumptions
- Conservative hedge: security does not depend on lattice problem hardness
- Role: backup/alternative signature standard to FIPS 204
- Twelve parameter sets (SHA-256 and SHAKE variants, simple and robust)
- Larger signatures than FIPS 204, but minimal cryptographic assumptions
- Note: CNSA 2.0 includes LMS and XMSS but excludes SLH-DSA (relevant for NSA-governed environments)

### 3. Algorithm Replacement Mapping

| Classical Algorithm | Vulnerability | Quantum Replacement |
|---|---|---|
| RSA key encapsulation | Shor (integer factorization) | ML-KEM / FIPS 203 |
| ECDH key agreement | Shor (EC discrete log) | ML-KEM / FIPS 203 |
| RSA digital signatures | Shor (integer factorization) | ML-DSA / FIPS 204 |
| ECDSA digital signatures | Shor (EC discrete log) | ML-DSA / FIPS 204 or SLH-DSA / FIPS 205 |
| DSA digital signatures | Shor (discrete log) | ML-DSA / FIPS 204 |
| AES-128 | Grover (halves key space) | AES-256 (no standard change needed) |
| AES-256, SHA-384, SHA-512 | Grover (manageable) | No change needed |

### 4. Developer Awareness in Crypto Documentation

Update the existing crypto documentation (`fips-cryptography-cheat-sheet.adoc`,
`crypto-post-quantum.adoc`) to include:

- In FIPS environments (RHEL 10), FIPS 203/204/205 must be used through FIPS-validated providers
  when those providers become available — do not roll your own
- Migration timeline: NIST IR 8547 — quantum-vulnerable algorithms deprecated by 2035;
  high-risk systems should transition earlier
- Hybrid deployment approach: run classical and post-quantum algorithms simultaneously during
  transition to maintain interoperability
- FIPS 206 (FN-DSA, based on FALCON) is in development; HQC (code-based backup KEM)
  selected March 2025 as NIST IR 8545

---

## RAG Resources Available

Query `rag-query --collection nist-pqc` for both the FIPS standard text (authoritative)
and the web article context (accessible explanations, migration guidance).

Key source files for each topic:

| Topic | Best source files |
|---|---|
| Emergence/threat model | cloudflare-pqc-standards.md, serverion-pqc-standards-en.md |
| FIPS 203 (ML-KEM) | fips203.pdf, wolfssl-fips-203-204-205.md, csa-fips-203-204-205-quantum-safe.md |
| FIPS 204 (ML-DSA) | fips204.pdf, wolfssl-fips-203-204-205.md |
| FIPS 205 (SLH-DSA) | fips205.pdf, wolfssl-fips-203-204-205.md, terraquantum-pqc-standards.md |
| Replacement mapping | hklaw-pqc-standards-2024.md, csrc-nist-pqc-standardization.md |
| Migration timeline | serverion-pqc-standards-en.md, csa-fips-203-204-205-quantum-safe.md |
| Official announcement | nist-pqc-announcement-2024.md, csrc-nist-pqc-project.md |

---

## Controls

`SC-12` (Cryptographic Key Establishment and Management), `SC-13` (Cryptographic Protection),
`SI-7` (Software, Firmware, and Information Integrity)
