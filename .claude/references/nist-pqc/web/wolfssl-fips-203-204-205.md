# What are FIPS 203, 204, and 205?

**Source URL:** https://www.wolfssl.com/what-are-fips-203-204-and-205/
**Publisher:** wolfSSL (embedded SSL/TLS library vendor)
**Retrieved:** 2026-03-13

---

## Overview

NIST released three official standards documents in 2024:
- **FIPS 203** — ML-KEM (Module-Lattice-based Key-Encapsulation Mechanism)
- **FIPS 204** — ML-DSA (Module-Lattice-based Digital Signature Algorithm)
- **FIPS 205** — SLH-DSA (StateLess Hash-based Digital Signature Algorithm)

## FIPS 203: ML-KEM (Module-Lattice-based Key-Encapsulation Mechanism)

"Kyber became ML-KEM (Module-Lattice-based Key-Encapsulation Mechanism) which is specified by NIST's FIPS 203 document."

ML-KEM serves general-purpose communication protocol needs. The standard functions similarly to ECDH but with important distinctions:

- ECDH operates as a NIKE (Non-Interactive Key Exchange)
- ML-KEM functions as a KEM (Key Encapsulation Mechanism)
- This creates API and semantic differences

ML-KEM is the primary replacement for RSA key exchange and ECDH key agreement.

## FIPS 204: ML-DSA (Module-Lattice-based Digital Signature Algorithm)

"Dilithium became ML-DSA (Module-Lattice-based Digital Signature Algorithm) which is specified by NIST's FIPS 204 document."

ML-DSA provides a direct replacement for RSA and ECDSA in cryptographic applications.

## FIPS 205: SLH-DSA (StateLess Hash-based Digital Signature Algorithm)

"SPHINCS+ became SLH-DSA (StateLess Hash-based Digital Signature Algorithm) which is specified by NIST's FIPS 205 document."

SLH-DSA addresses specialized use cases similar to LMS (Leighton-Micali Signature) and XMSS (eXtended Merkle Signature Scheme), maintaining a stateless design.

**Important note on CNSA 2.0**: The Commercial National Security Algorithm Suite 2.0 specifically includes LMS and XMSS but excludes SLH-DSA. This distinction matters for NSA-governed environments.

## Algorithm Lineage Summary

| FIPS Standard | New Name | Original Algorithm | Replaces |
|---|---|---|---|
| FIPS 203 | ML-KEM | CRYSTALS-Kyber | RSA key exchange, ECDH |
| FIPS 204 | ML-DSA | CRYSTALS-Dilithium | RSA signatures, ECDSA |
| FIPS 205 | SLH-DSA | SPHINCS+ | RSA signatures, ECDSA (backup) |

## Implementation (wolfSSL)

wolfSSL offers optimized implementations of ML-KEM and ML-DSA. Configuration flags:
- `--enable-kyber` — enables ML-KEM (FIPS 203)
- `--enable-dilithium` — enables ML-DSA (FIPS 204)
