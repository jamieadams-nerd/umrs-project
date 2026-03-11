# RustCrypto Overview

Source: https://github.com/RustCrypto (organization profile README)
Retrieved: 2026-03-10

---

RustCrypto maintains a comprehensive collection of pure Rust cryptographic implementations across multiple categories.

## Core Algorithms

- **Asymmetric encryption**: elliptic curves, ML-KEM, RSA
- **Symmetric encryption**: AES-GCM variants, ChaCha20Poly1305
- **Hash functions**: BLAKE2, SHA2, SHA3
- **Digital signatures**: DSA, ECDSA, Ed25519, RSA

## Specialized Components

- **Encoding formats**: DER, PEM, PKCS#8, X.509 certificates
- **Key derivation functions**: HKDF, PBKDF2
- **Password hashing**: Argon2, Scrypt
- **Message authentication codes**: HMAC
- **Sponge functions**: Ascon, Keccak

## Infrastructure

The project provides trait abstractions for:
- AEAD ciphers
- General ciphers
- Digest algorithms
- Password hashing
- Digital signatures

These enable interoperability across implementations.

## Leadership

Maintained by Artyom Pavlov and Tony Arcieri. Community engagement via Zulip chat.

## Relevance to UMRS Project

For FIPS 140-2/3 compliance:
- AES-GCM, ChaCha20Poly1305: symmetric encryption
- SHA2, SHA3: hashing (SHA-256, SHA-384, SHA-512 are FIPS-approved)
- HMAC: message authentication
- HKDF, PBKDF2: key derivation
- ECDSA, Ed25519, RSA: signatures

**Note:** RustCrypto crates are pure Rust but are NOT FIPS 140-2/3 validated modules. For FIPS-validated cryptography on RHEL10, use the system OpenSSL (FIPS provider) via FFI or subprocess.
