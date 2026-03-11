# Cryptographic Right Answers

Source: https://latacora.com/blog/cryptographic-right-answers/
Author: Latacora (originally Colin Percival 2009, Thomas Ptacek 2015, Latacora 2018)
Retrieved: 2026-03-10

---

This page provides a pragmatic guide for developers (not cryptography engineers) on which cryptographic primitives to use. "Boring" is the goal.

> "You should keep things simple and conventional and easy to analyze; 'boring', as the Google TLS people would say."

---

## Encrypting Data

| Year | Recommendation |
|---|---|
| Percival 2009 | AES-CTR with HMAC |
| Ptacek 2015 | NaCl/libsodium default, ChaCha20-Poly1305, or AES-GCM |
| **Latacora 2018** | **KMS or XSalsa20+Poly1305** |

- Use KMS (Amazon or Google HSM) if available. No need to understand how it works.
- Otherwise use AEAD: XSalsa20-Poly1305 (from libsodium/NaCl).
- XSalsa20 supports extended nonces — safe to generate randomly without nonce collision concerns.
- **Avoid:** AES-CBC, AES-CTR alone, 64-bit block ciphers (especially Blowfish), OFB mode, RC4.

## Symmetric Key Length

| Year | Recommendation |
|---|---|
| **All** | **256-bit keys** |

- Your AES key is far less likely to break than your public key pair.
- Avoid: constructions with huge keys, cipher cascades, keys under 128 bits.

## Symmetric "Signatures" (MACs)

| Year | Recommendation |
|---|---|
| **All** | **HMAC** |

- For API authentication: use a secure compare function.
- Watch for "crypto canonicalization bugs" in how data is fed to the MAC.
- HMAC works with SHA-2; KMAC with SHA-3 is theoretically better but unnecessary.
- **Avoid:** custom keyed hash constructions, HMAC-MD5, HMAC-SHA1, CRC.

## Hashing Algorithm

| Year | Recommendation |
|---|---|
| **All** | **SHA-2** |

- Prefer SHA-512/256 (truncated output, sidesteps length extension attacks).
- SHA-2 still looks great; no rush to move to SHA-3.
- **Avoid:** SHA-1, MD5, MD6.

## Random IDs

| Year | Recommendation |
|---|---|
| **All** | **256-bit random numbers from /dev/urandom** |

- **Avoid:** userspace RNGs, OpenSSL RNG, havaged, prngd, egd, /dev/random.

## Password Handling

| Year | Recommendation |
|---|---|
| Percival 2009 | scrypt or PBKDF2 |
| Ptacek 2015 | scrypt > bcrypt > PBKDF2 |
| **Latacora 2018** | **scrypt > argon2 > bcrypt > PBKDF2** |

- In practice: use any real secure password hash. Which one matters less than using one.
- Don't build elaborate password-hash-agility schemes.
- **Avoid:** SHA-3, naked SHA-2, SHA-1, MD5.

## Asymmetric Encryption

| Year | Recommendation |
|---|---|
| Percival 2009 | RSAES-OAEP with SHA-256 |
| **Ptacek 2015 / Latacora 2018** | **NaCl/libsodium (box / crypto_box)** |

- Narrow use case: encrypting to strangers, store-and-forward, offline decryption.
- Use Curve25519 (NaCl default). Don't freelance public key encryption.
- **Reasons to avoid RSA:**
  - Drags towards backwards-compatibility (downgrade attacks)
  - Too many knobs
  - Encourage direct encryption with public key primitive
- **Avoid:** RSA-PKCS1v15, RSA, ElGamal, any new system using RSA.

## Asymmetric Signatures

| Year | Recommendation |
|---|---|
| Ptacek 2015 | NaCl, Ed25519, or RFC6979 |
| **Latacora 2018** | **NaCl or Ed25519** |

- Use deterministic signatures (Ed25519, RFC6979) — misuse-resistant.
- Conventional DSA/ECDSA: reuse of random number leaks secret keys (PS3 attack).
- Ed25519 (NaCl default): most popular public key signature outside Bitcoin.
- **Avoid:** RSA-PKCS1v15, RSA, ECDSA, DSA, conventional DSA/ECDSA.

## Diffie-Hellman

| Year | Recommendation |
|---|---|
| Percival 2009 | DH-2048, Group #14, generator 2 |
| Ptacek 2015 | DH-2048 or NaCl |
| **Latacora 2018** | **Probably nothing. Or use Curve25519.** |

- Don't freelance encrypted transports.
- Use Curve25519 if you need DH. Libraries available in virtually every language.
- Don't use ECDH with NIST curves — must verify curve points to avoid secret leakage.
- **Avoid:** conventional DH, SRP, J-PAKE, elaborate key negotiation schemes.

## Website Security

| Year | Recommendation |
|---|---|
| **Latacora 2018** | **AWS ALB/ELB or OpenSSL with Let's Encrypt** |

- If you can pay AWS to handle this: do it.
- Otherwise: OpenSSL. It has improved significantly post-2016.
- Let's Encrypt: free and automated. Set up cron for certificate renewal.
- **Avoid:** PolarSSL, GnuTLS, MatrixSSL.

## Client-Server Application Security

| Year | Recommendation |
|---|---|
| **Latacora 2018** | **AWS ALB/ELB or OpenSSL with Let's Encrypt** |

- Use TLS 1.2+ with Curve25519 and ChaPoly (ChaCha20-Poly1305).
- In custom protocols: don't need CA; can use whitelist of self-signed certs.
- Many TLS attacks require browser context (victim executes attacker JS) — irrelevant for custom protocols.
- **Avoid:** designing your own encrypted transport; default TLS configurations.

## Online Backups

| Year | Recommendation |
|---|---|
| **All** | **Tarsnap** |

---

## Key Summary for UMRS Project (FIPS Context)

| Purpose | Recommendation | FIPS 140-2 Note |
|---|---|---|
| Symmetric encryption | AES-GCM-256 | Approved |
| Hashing | SHA-256 / SHA-384 / SHA-512 | Approved |
| MAC | HMAC-SHA-256 | Approved |
| Asymmetric signatures | ECDSA (P-256/P-384) or Ed25519 | P-256/P-384 approved; Ed25519 pending |
| Key derivation | HKDF, PBKDF2 | Both approved |
| Password hashing | PBKDF2 | Only FIPS-approved option |
| Random IDs | /dev/urandom (kernel DRBG) | Approved on RHEL10 |
| Asymmetric encryption | RSA-OAEP or ECDH (P-256/P-384) | Approved |

Note: XSalsa20/Poly1305/ChaCha20 are NOT FIPS 140-2 approved. On FIPS-mode RHEL10, use AES-GCM.
