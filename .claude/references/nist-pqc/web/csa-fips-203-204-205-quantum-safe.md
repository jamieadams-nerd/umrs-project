# NIST FIPS 203, 204, and 205 Finalized: An Important Step Towards a Quantum-Safe Future

**Source URL:** https://cloudsecurityalliance.org/blog/2024/08/15/nist-fips-203-204-and-205-finalized-an-important-step-towards-a-quantum-safe-future
**Published:** August 15, 2024
**Author:** Mehak Kalsi, Co-Chair, CSA Quantum-Safe Security Working Group
**Reviewer:** Bruno Huttner, Co-Chair, CSA Quantum-Safe Security Working Group
**Publisher:** Cloud Security Alliance (CSA)
**Retrieved:** 2026-03-13

---

## Overview

The National Institute of Standards and Technology (NIST) has released three post-quantum cryptography (PQC) algorithms designed to protect against cryptographically relevant quantum computers (CRQP). These Federal Information Processing Standards address the vulnerability of current asymmetric cryptography infrastructure. Experts anticipate a functional CRQP within 5 to 10 years, making immediate organizational preparation essential.

## The Three FIPS Standards Released

### FIPS 203: Module-Lattice-Based Key-Encapsulation Mechanism
- **Full Title:** Module-Lattice-Based Key-Encapsulation Mechanism Standard
- **Primary Use:** Encryption alongside symmetric-key cryptographic algorithms
- **Technical Foundation:** Leverages the Module Learning with Errors (MLWE) problem
- **Parameter Options:** ML-KEM-512, ML-KEM-768, and ML-KEM-1024 (higher numbers provide greater security with performance trade-offs)
- **Replaces:** RSA key encapsulation, ECDH key agreement

### FIPS 204: Module-Lattice-Based Digital Signature
- **Full Title:** Module-Lattice-Based Digital Signature Standard
- **Primary Use:** Generate and verify digital signatures
- **Security Benefits:** Provides authentication, data integrity verification, and non-repudiation
- **Replaces:** RSA digital signatures, ECDSA

### FIPS 205: Stateless Hash-Based Digital Signature
- **Full Title:** Stateless Hash-Based Digital Signature Standard
- **Technical Basis:** Built on SPHINCS+ algorithm
- **Primary Use:** Digital signature operations
- **Security Benefits:** Confirms signer authentication, ensures data integrity, enables non-repudiation
- **Mathematical basis:** Security relies solely on hash function properties — conservative hedge

## Timeline and Historical Context

- **December 20, 2016:** NIST solicited proposals for quantum-resistant algorithms
- **2016-2022:** Four rounds of evaluation reducing 82 candidates to finalists
- **July 2022:** Three finalist algorithms announced
- **August 24, 2023:** Public comment period opened on draft FIPS documents
- **November 22, 2023:** Comment submission deadline
- **August 13, 2024:** Final standards released (FIPS 203, 204, 205)

## Implementation Recommendations

Organizations should prioritize quantum-safe strategy development immediately. U.S. Government vendors will face requirements sooner rather than later, and major global governments are similarly investing in post-quantum readiness.

**Starting Points:**
- Reference "Quantum-Readiness: Migration to Post Quantum Cryptography" (joint CISA, NSA, NIST guidance)
- Monitor NIST Computer Security Resource Center (CSRC) for upcoming PQC seminars beginning fall 2024
- Consult CSA Quantum-Safe Security Working Group publications

## Key Takeaway

Organizations must begin preparing their quantum-safe strategies now. If government agencies and international bodies are investing resources in post-quantum planning, civilian organizations should follow suit to remain competitive and secure in the coming quantum era.

NIST IR 8547 establishes that quantum-vulnerable algorithms will be deprecated by 2035, with earlier deadlines for high-risk systems.
