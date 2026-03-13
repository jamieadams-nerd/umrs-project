# NIST's First Post-Quantum Standards

**Source URL:** https://blog.cloudflare.com/nists-first-post-quantum-standards/
**Published:** August 20, 2024
**Authors:** Luke Valenta,Vania Goncalves, Bas Westerbaan
**Retrieved:** 2026-03-13

---

## Overview

On August 13, 2024, the National Institute of Standards and Technology released "the first three cryptographic standards designed to resist an attack from quantum computers: ML-KEM, ML-DSA, and SLH-DSA." This milestone represents significant progress in preparing digital infrastructure for the quantum computing era.

## Why Quantum Computers Pose a Threat

Modern cryptography relies on mathematical puzzles — such as factoring large numbers — that classical computers cannot efficiently solve. However, quantum computers leverage quantum mechanical properties to perform specialized calculations far more effectively than traditional systems.

Currently, no cryptographically-relevant quantum computers (CRQCs) exist. However, experts predict their emergence is inevitable. According to a 2023 survey, "more than a quarter of interviewed experts...expect that a CRQC is more likely than not to appear in the next decade."

## The "Harvest Now, Decrypt Later" Problem

Adversaries can capture encrypted data today and store it for future decryption once quantum computers become available. This creates immediate urgency for protecting current communications with post-quantum encryption.

## NIST's Standardization Effort

NIST launched a public competition in 2016, evaluating "82 algorithms" submitted by global cryptography teams. After years of rigorous analysis, NIST selected four algorithms for standardization, with three finalized in August 2024:

- **ML-KEM** (FIPS 203): Key agreement protocol, based on CRYSTALS-Kyber
- **ML-DSA** (FIPS 204): Digital signature algorithm, based on CRYSTALS-Dilithium
- **SLH-DSA** (FIPS 205): Alternative signature option, based on SPHINCS+
- **FN-DSA** (FIPS 206, FALCON-based): Planned for late 2024 release

## Key Agreement vs. Digital Signatures

**Key Agreement** enables two parties to establish shared secrets over insecure channels, creating encryption foundations. This threat model demands rapid migration.

**Digital Signatures** verify identity and authenticity in systems like WebPKI. The migration timeline is longer but more complex, involving browsers, certificate authorities, and hardware manufacturers.

## Current Deployment Status

Cloudflare has actively participated in post-quantum adoption. As of mid-August 2024, "over 16% of human-generated requests to Cloudflare's servers are already protected with post-quantum key agreement." Industry leaders including Google, Apple, Meta, and Signal have similarly deployed post-quantum protections.

## Migration Timeline

- **Key Agreement**: Rapid adoption expected industry-wide
- **Signatures**: Post-quantum certificates anticipated in 2026, though not immediately default-enabled

Organizations should begin evaluating post-quantum readiness, while remaining aware that signature migration requires broader ecosystem coordination.
