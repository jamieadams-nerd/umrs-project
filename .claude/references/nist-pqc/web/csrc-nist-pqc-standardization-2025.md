# NIST Post-Quantum Cryptography Standardization

Source: https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization
Retrieved: 2026-03-13
Note: Updated version of csrc-nist-pqc-standardization.md. Includes FIPS 207 / HQC selection (March 2025).

---

## Overview

NIST's Computer Security Resource Center (CSRC) manages a comprehensive standardization initiative
for post-quantum cryptographic algorithms designed to resist threats from quantum computing. The
project is led by Dr. Dustin Moody and the Cryptographic Technology Group.

---

## Published Standards (August 13, 2024)

The Secretary of Commerce approved three Federal Information Processing Standards:

### FIPS 203 — Module-Lattice-Based Key-Encapsulation Mechanism Standard (ML-KEM)

- Derived from the CRYSTALS-KYBER submission
- Defines ML-KEM-512, ML-KEM-768, and ML-KEM-1024 parameter sets
- Primary quantum-resistant key encapsulation mechanism

### FIPS 204 — Module-Lattice-Based Digital Signature Standard (ML-DSA)

- Derived from the CRYSTALS-Dilithium submission
- Defines ML-DSA-44, ML-DSA-65, and ML-DSA-87 parameter sets
- General-purpose quantum-resistant replacement for RSA and ECDSA

### FIPS 205 — Stateless Hash-Based Digital Signature Standard (SLH-DSA)

- Derived from the SPHINCS+ submission
- Security relies only on hash function properties (no lattice assumptions)
- Provides a hedge against potential weaknesses in lattice-based schemes
- Twelve parameter sets across SHA-256 and SHAKE variants

---

## Standards in Development

### FIPS 206 — FN-DSA (Based on FALCON)

- Specifies FN-DSA, based on the FALCON submission
- Uses a hash-and-sign paradigm
- Offers smaller bandwidth and fast verification
- More complicated implementation than ML-DSA
- Currently under development

### FIPS 207 — HQC (Selected March 11, 2025)

- HQC (Hamming Quasi-Cyclic) was selected for standardization on March 11, 2025
- Documented in NIST IR 8545: Status Report on the Fourth Round of the NIST PQC Standardization Process
- KEM based on QC-MDPC code (code-based cryptography, not lattice-based)
- Offers strong security assurances and mature decryption failure rate analysis
- Larger public keys and ciphertext sizes than BIKE (the other fourth-round candidate)
- Selected to augment the key-establishment portfolio alongside ML-KEM
- Provides algorithmic diversity: different mathematical foundation from ML-KEM

---

## Standardization Process Structure

The initiative follows a multi-round evaluation framework:

- **Round 1** (2017): 69 submissions received
- **Round 2** (2019): 26 candidates advanced
- **Round 3** (2020): 15 finalists and alternates
- **Round 4** (ongoing): Supplementary KEM candidates under evaluation
- **Additional Digital Signature Schemes**: Round 2 submissions accepted for supplementary algorithms

---

## Selected Algorithms Summary

| Standard | Algorithm | Type | Basis |
|---|---|---|---|
| FIPS 203 | ML-KEM | Key Encapsulation | Module Learning With Errors (lattice) |
| FIPS 204 | ML-DSA | Digital Signature | Module Learning With Errors (lattice) |
| FIPS 205 | SLH-DSA | Digital Signature | Hash-based (SPHINCS+) |
| FIPS 206 (draft) | FN-DSA | Digital Signature | NTRU lattice (FALCON) |
| FIPS 207 (planned) | HQC | Key Encapsulation | Code-based (QC-MDPC) |

---

## Transition Timeline

Per NIST IR 8547, NIST will:
- Deprecate quantum-vulnerable algorithms from its standards by 2030 (some earlier for high-risk systems)
- Disallow quantum-vulnerable algorithms by 2035
- High-risk systems are expected to transition significantly earlier than 2035

---

## Migration Resources

- NCCoE Migration to Post-Quantum Cryptography project
- NIST IR 8547: Transition to Post-Quantum Cryptography Standards
- NIST IR 8545: Status Report on the Fourth Round
- 6th PQC Standardization Conference: September 24-26, 2025, Gaithersburg, Maryland

---

## Related Initiatives

- Migration to Post-Quantum Cryptography (NCCoE)
- Multi-Party Threshold Cryptography
- PQC Digital Signature Schemes (additional candidates)
- Cryptographic Standards and Guidelines

---

## Contact

PQC Crypto Technical Inquiries: Dr. Dustin Moody, Cryptographic Technology Group, NIST CSRC
