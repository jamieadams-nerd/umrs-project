# NIST Post-Quantum Cryptography Standardization

**Source URL:** https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization
**Issuing authority:** NIST Computer Security Resource Center (CSRC)
**Retrieved:** 2026-03-13

---

## Overview

NIST's Computer Security Resource Center (CSRC) manages the Post-Quantum Cryptography (PQC) standardization project, addressing cryptographic vulnerabilities posed by quantum computing advances.

## Recent Standards Published

Three finalized FIPS standards were released August 13, 2024:

- **FIPS 203**: Module-Lattice-Based Key-Encapsulation Mechanism — derived from CRYSTALS-KYBER
- **FIPS 204**: Module-Lattice-Based Digital Signature Standard — derived from CRYSTALS-Dilithium
- **FIPS 205**: Stateless Hash-Based Digital Signature Standard — derived from SPHINCS+

Additionally, "FALCON was also selected and will be published in FIPS 206 (in development)."

## Round 4 Development

On March 11, 2025, HQC (Hamming Quasi-Cyclic) received selection for standardization, detailed in "NIST IR 8545, Status Report on the Fourth Round of the NIST Post-Quantum Cryptography Standardization Process."

HQC is a code-based key encapsulation mechanism selected as a backup standard — it uses error-correcting codes rather than lattice mathematics, providing diversity against potential future lattice vulnerabilities.

## Process Structure

The standardization initiative includes:

- Call for Proposals (2016) and Submission Requirements
- Minimum Acceptability Requirements and Evaluation Criteria
- Round 1-4 submission evaluations (82 initial submissions narrowed over four rounds)
- Round 3 seminars for technical discussion
- Additional Digital Signature Schemes (Round 2) — ongoing evaluation of additional signature algorithms

## Algorithm Replacement Mapping

| Classical Algorithm | Vulnerability | PQC Replacement |
|---|---|---|
| RSA key exchange | Shor's algorithm breaks integer factorization | ML-KEM (FIPS 203) |
| ECDH | Shor's algorithm breaks elliptic curve discrete log | ML-KEM (FIPS 203) |
| RSA signatures | Shor's algorithm breaks integer factorization | ML-DSA (FIPS 204) |
| ECDSA | Shor's algorithm breaks elliptic curve discrete log | ML-DSA (FIPS 204) or SLH-DSA (FIPS 205) |
| DSA | Shor's algorithm breaks discrete log | ML-DSA (FIPS 204) |

Note: Symmetric algorithms (AES-256, SHA-384/512) remain secure under quantum attacks when used with sufficiently large key sizes (Grover's algorithm reduces their effective security by half, not to zero).

## Contact & Resources

Technical Contact: Dr. Dustin Moody
Email: PQC Crypto Technical Inquiries
Group: Cryptographic Technology Division
Short URL: https://www.nist.gov/pqcrypto

## Related Projects

- Cryptographic Standards and Guidelines
- Migration to Post-Quantum Cryptography (NCCoE)
- PQC Digital Signature Schemes
- Hash-Based Signatures
- Multi-Party Threshold Cryptography
