# NIST Releases Three Post-Quantum Cryptography Standards

**Source URL:** https://www.hklaw.com/en/insights/publications/2024/08/nist-releases-three-post-quantum-cryptography-standards
**Published:** August 20, 2024
**Authors:** Jacob W. S. Schneider, Paul Stimers
**Publisher:** Holland & Knight IP/Decode Blog
**Retrieved:** 2026-03-13

---

## Overview

The National Institute of Standards and Technology (NIST) announced approval of three post-quantum cryptographic algorithms in mid-August 2024. This announcement follows the 2022 Quantum Computing Cybersecurity Preparedness Act, which directed federal agencies to prepare for quantum computing threats to current encryption methods.

## The Three FIPS Standards

On August 13, 2024, NIST finalized three Federal Information Processing Standards (FIPS):

### FIPS 203: Module-Lattice-Based Key-Encapsulation Mechanism Standard
- General encryption standard
- Uses lattice cryptography based on finding lowest common multiples (Module Learning With Errors problem)
- Replaces RSA key exchange and ECDH key agreement mechanisms

### FIPS 204: Module-Lattice-Based Digital Signature Standard
- Digital signature standard for user authentication
- Also uses lattice cryptography (Module Learning With Errors)
- Replaces RSA and ECDSA digital signatures

### FIPS 205: Stateless Hash-Based Digital Signature Standard
- Digital signature standard
- Uses hash functions as its mathematical foundation
- Serves as an alternative/backup to lattice-based FIPS 204

## Background

NIST began a public competition in 2016 to develop "quantum-safe algorithms." The competition addressed the threat that "large-scale quantum computers will be able to break many of the public-key cryptosystems currently in use." RSA encryption, which relies on prime number factorization, faces particular vulnerability.

The competition narrowed 82 initial algorithms to finalists through four elimination rounds.

## Algorithms These Standards Replace

The following classical algorithms are vulnerable to quantum attacks via Shor's algorithm and are targeted for replacement:

- **RSA**: Key exchange and digital signatures — replaced by ML-KEM (FIPS 203) and ML-DSA (FIPS 204)
- **ECDH (Elliptic Curve Diffie-Hellman)**: Key agreement — replaced by ML-KEM (FIPS 203)
- **ECDSA (Elliptic Curve Digital Signature Algorithm)**: Digital signatures — replaced by ML-DSA (FIPS 204) and SLH-DSA (FIPS 205)

## Federal Government Requirements

The Quantum Computing Cybersecurity Preparedness Act mandates that:

- The Office of Management and Budget (OMB) develop migration guidance for federal agencies
- Agencies establish inventories of quantum-vulnerable IT systems
- Agencies report results to OMB
- OMB issue further guidance within one year of NIST's standard approval
- Progress reports go to Congress

Federal IT contractors have already begun preparing to incorporate these standards.

## Private Sector Implications

Organizations face "harvest now, decrypt later" risks, where adversaries collect encrypted data today for decryption when quantum computers become available. Companies must assess deployment timelines based on potential liability and reputational damage, which will likely increase now that standards are finalized.
