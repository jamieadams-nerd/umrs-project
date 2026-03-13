# NIST CSRC Post-Quantum Cryptography Project

**Source URL:** https://csrc.nist.gov/projects/post-quantum-cryptography
**Issuing authority:** NIST Computer Security Resource Center (CSRC)
**Retrieved:** 2026-03-13

---

## Overview

NIST's Post-Quantum Cryptography project addresses the future threat of quantum computers by securing electronic information through cryptographic innovation. The initiative involves "a multi-year international competition involving industry, academia, and governments" to develop quantum-resistant standards.

## Key Standards Released (August 13, 2024)

NIST published three principal Federal Information Processing Standards (FIPS):

- **FIPS 203**: Module-Lattice-Based Key-Encapsulation Mechanism (ML-KEM)
  - Derived from CRYSTALS-KYBER
  - General-purpose key encapsulation; replaces RSA/ECDH key exchange
- **FIPS 204**: Module-Lattice-Based Digital Signature Standard (ML-DSA)
  - Derived from CRYSTALS-Dilithium
  - Primary digital signature standard; replaces RSA/ECDSA
- **FIPS 205**: Stateless Hash-Based Digital Signature Standard (SLH-DSA)
  - Derived from SPHINCS+
  - Hash-based digital signature standard; conservative alternative

## FIPS 206 (in development)

"FALCON was also selected and will be published in FIPS 206" — FN-DSA (FALCON-based digital signature algorithm), currently in development.

## Migration Timeline

Organizations should begin transitioning to quantum-resistant cryptography now. Per NIST IR 8547, quantum-vulnerable algorithms will be deprecated and removed from standards by 2035, with high-risk systems transitioning earlier.

## Round 4 Development

On March 11, 2025, HQC (Hamming Quasi-Cyclic) received selection for standardization. Details in NIST IR 8545, "Status Report on the Fourth Round of the NIST Post-Quantum Cryptography Standardization Process." HQC is a code-based backup standard for key encapsulation.

## Process Structure

The standardization initiative includes:

- Call for Proposals (2016) — 82 algorithms submitted
- Minimum Acceptability Requirements and Evaluation Criteria
- Round 1-4 submission evaluations
- Round 3 seminars for technical discussion
- Additional Digital Signature Schemes (Round 2) — ongoing evaluation

## Related Projects

- Cryptographic Standards and Guidelines
- Migration to Post-Quantum Cryptography (NCCoE — National Cybersecurity Center of Excellence)
- PQC Digital Signature Schemes
- Hash-Based Signatures
- Multi-Party Threshold Cryptography

## Contact

Technical Contact: Dr. Dustin Moody
Group: Cryptographic Technology Division
Short URL: https://www.nist.gov/pqcrypto
