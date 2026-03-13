# NIST Standards for Post-Quantum Cryptography (Norwegian Translation)

**Source URL:** https://www.serverion.com/nn/uncategorized/nist-standards-for-post-quantum-cryptography/
**Language:** Norwegian Bokmål (translation of the English article at serverion.com/uncategorized/)
**Retrieved:** 2026-03-13

---

## Note on Content

This is the Norwegian-language version of the Serverion PQC article. The content is a translation of the English article saved as `serverion-pqc-standards-en.md`. Both versions cover the same technical material:

- FIPS 203 (ML-KEM / Kyber) — lattice-based key encapsulation, replaces RSA/ECDH
- FIPS 204 (ML-DSA / Dilithium) — lattice-based digital signatures, replaces RSA/ECDSA
- FIPS 205 (SLH-DSA / SPHINCS+) — hash-based digital signatures, backup standard
- HQC — code-based backup KEM (Round 4 selection, March 2025)

## Key Content Themes (from Norwegian version)

The Norwegian article covers the same core subjects:

- The quantum threat: "Quantum computers could crack RSA and ECC encryption in mere hours"
- "Harvest now, decrypt later" scenario — an estimated $3.5 trillion in vulnerable assets
- Three NIST FIPS standards: FIPS 203 (Kyber), FIPS 204 (Dilithium), FIPS 205 (SPHINCS+)
- Implementation timeline: 2028 discovery, 2031 high-priority migrations, 2035 full transition
- Performance data: Kyber achieves 5.98x speedup with AVX2 optimization
- Business recommendations: cryptographic asset inventory, hybrid deployment, crypto-agility

## Algorithm Replacement Summary

| Standard | New Name | Replaces |
|---|---|---|
| FIPS 203 | ML-KEM | RSA key exchange, ECDH |
| FIPS 204 | ML-DSA | RSA signatures, ECDSA |
| FIPS 205 | SLH-DSA | RSA signatures, ECDSA (backup) |

Refer to `serverion-pqc-standards-en.md` for the full English-language content with detailed technical sections.
