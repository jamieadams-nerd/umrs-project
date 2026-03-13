# PQC Status Tracker

Maintained by the researcher agent. All agents may read this file for current PQC status.
Updated on each library refresh cycle.

**Last checked**: 2026-03-13

---

## NIST PQC Program Status

### Finalized Standards (August 13, 2024)

| Standard | Algorithm | Type | Status |
|---|---|---|---|
| FIPS 203 | ML-KEM (CRYSTALS-Kyber) | Key encapsulation | **Published** |
| FIPS 204 | ML-DSA (CRYSTALS-Dilithium) | Digital signature | **Published** |
| FIPS 205 | SLH-DSA (SPHINCS+) | Hash-based signature | **Published** |

### In Development

| Standard | Algorithm | Type | Status | Expected |
|---|---|---|---|---|
| FIPS 206 | FN-DSA (FALCON) | Lattice signature | Finalizing | Shortly (2025) |
| (TBD) | HQC | Code-based KEM (backup) | Selected March 2025 | Draft standard early 2026 |

### Signature On-Ramp (Round 2)

NIST is evaluating 14 additional digital signature candidates to provide algorithm diversity
beyond lattice-based schemes. Notable candidates: CROSS, FAEST, MAYO.

Purpose: hedge against potential future lattice vulnerabilities.

**Monitor URLs**:
- https://csrc.nist.gov/projects/pqc-dig-sig
- https://csrc.nist.gov/projects/pqc-dig-sig/round-2-additional-signatures

### Key NIST Documents

| Document | URL | Content |
|---|---|---|
| NIST IR 8413 | https://csrc.nist.gov/pubs/ir/8413 | Status report, Round 3 |
| NIST IR 8545 | https://csrc.nist.gov/pubs/ir/8545/final | Status report, Round 4 (HQC selection rationale) |
| NCCoE Migration | https://csrc.nist.gov/Projects/migration-to-post-quantum-cryptography-nccoe | Practical migration guidance |

### Standardization page last updated: December 11, 2025

### FIPS 140-3 Validation for PQC

NIST has NOT yet defined quantum-safe FIPS 140-3 validation requirements.
It is not yet clear when NIST will begin this process.
Until requirements are defined and modules validated, standalone PQC cannot be used in FIPS mode.

**Exception**: Hybrid key exchange algorithms that combine FIPS-approved classical curves
with ML-KEM ARE functional in FIPS mode (see RHEL section below).

**Monitor URL**: https://csrc.nist.gov/projects/cryptographic-module-validation-program

---

## RHEL 10 PQC Availability

| RHEL Version | PQC Status | How to Enable |
|---|---|---|
| RHEL 10.0 (May 2025) | Technology Preview | `crypto-policies-pq-preview` + `DEFAULT:TEST-PQ` |
| RHEL 10.1+ | **Generally Available** | Supported in DEFAULT crypto-policy |
| Any (FIPS Mode) | **Partial** — hybrid KEx only | See FIPS section below |

### RHEL 10.1 Detail

- ML-KEM and ML-DSA in TLS: GA and fully supported in OpenSSL 3.5, GnuTLS, NSS, and Go
- DEFAULT crypto-policy enables and **prefers** PQC by default — TLS and SSH connections
  automatically use post-quantum key exchange where available
- PQC algorithms enabled by default in **all** predefined policy levels (except FIPS provider)
- SSH: `mlkem768x25519-sha256` and `sntrup761x25519-sha512` key exchange supported
- OpenPGP: PQC support via Sequoia-PGP included in RHEL 10.1

### RPM Signing Milestone

Red Hat has created a hybrid signing key (ML-DSA-87 + Ed448) and started signing RPM packages
with it. **RHEL is the first major Linux distribution to sign packages with PQC.**

### FIPS Mode + PQC: The Hybrid Exception

The PQC algorithms in OpenSSL 3.5 are in the **default provider**, NOT the FIPS provider.
However, **hybrid key exchange** algorithms that use FIPS-approved curves ARE functional
in FIPS mode:

| Hybrid Algorithm | Components | FIPS Mode |
|---|---|---|
| SecP256r1MLKEM768 | ECDH P-256 + ML-KEM-768 | **Works** |
| SecP384r1MLKEM1024 | ECDH P-384 + ML-KEM-1024 | **Works** |
| Standalone ML-KEM | ML-KEM only | **Blocked** |
| ML-DSA signatures | ML-DSA only | **Blocked** |

This means CUI/CMMC deployments CAN get hybrid PQC key exchange in FIPS mode today,
even though standalone PQC is blocked. This is a significant nuance.

### Protocol Standards Still In Progress

- TLS authentication with PQC signatures: draft-ietf-lamps-dilithium-certificates and
  draft-ietf-lamps-pq-composite-sigs (finalized support tentatively targeted Q4 2025)
- OpenPGP PQC: specification in final stages; Sequoia-PGP implementation in RHEL 10.1

### OpenShift Timeline

- OpenShift 4.20: emerging PQC support in control plane (Go 1.24, X25519MLKEM768 hybrid)
- OpenShift based on EUS releases only — must wait for RHEL 9.8 and 10.2 (Spring 2026)
  for core quantum-safe support
- UBI 9.7, UBI 10, UBI 10.1: PQC-capable OpenSSL available for layered applications now
- Service Mesh 3.2: quantum-secure gateway configuration available

### UMRS Impact

- `umrs-crypto` (planned) must gate PQC on FIPS mode — but the gate is more nuanced:
  - If `fips_enabled=1`: hybrid KEx (SecP384r1MLKEM1024) is available
  - If `fips_enabled=1`: standalone ML-DSA/ML-KEM signatures are blocked
  - If `fips_enabled=0`: full PQC available on RHEL 10.1+
- Hybrid deployment (classical + PQC) is possible even in FIPS mode via hybrid KEx
- Track CMVP status — once PQC modules are FIPS-validated, standalone PQC becomes available
- RPM signing with hybrid keys: consider verifying PQC signatures on UMRS packages

---

## Monitoring Sources

Check these on each refresh cycle:

### NIST PQC Program
| Source | URL | What to check |
|---|---|---|
| NIST PQC project | https://csrc.nist.gov/projects/post-quantum-cryptography | New selections, standard publications |
| NIST PQC standardization | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization | Round status, candidate changes |
| NIST PQC signature on-ramp | https://csrc.nist.gov/projects/pqc-dig-sig | Signature candidate evaluation progress |
| NIST CMVP | https://csrc.nist.gov/projects/cryptographic-module-validation-program | FIPS 140-3 PQC validation status |
| NIST news | https://www.nist.gov/news-events/news | FIPS 206 publication, HQC draft |

### RHEL PQC Support
| Source | URL | What to check |
|---|---|---|
| Red Hat PQC in RHEL 10.1 | https://www.redhat.com/en/blog/whats-new-post-quantum-cryptography-rhel-101 | RHEL 10.x PQC updates |
| Red Hat PQC in RHEL 10 | https://www.redhat.com/en/blog/post-quantum-cryptography-red-hat-enterprise-linux-10 | Baseline PQC support changes |
| Red Hat quantum-safe roadmap | https://www.redhat.com/en/blog/road-to-quantum-safe-cryptography-red-hat-openshift | OpenShift + FIPS timeline |
| Red Hat PQC integration | https://www.redhat.com/en/blog/how-red-hat-integrating-post-quantum-cryptography-our-products | Cross-product PQC strategy |
| Red Hat year of PQC | https://www.redhat.com/en/blog/if-how-year-post-quantum-reality | Strategic PQC direction |
| Red Hat PQC overview | https://www.redhat.com/en/technologies/linux-platforms/enterprise-linux-10/post-quantum-cryptography | Product page |
| RHEL 10 interop article | https://access.redhat.com/articles/7119430 | PQC interoperability details |
| RHEL 10 crypto policies doc | https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/html/security_hardening/using-system-wide-cryptographic-policies | Official crypto-policy docs |
| RHEL 10 release notes | https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10 | Crypto-policy changes per release |
| OpenShift 4.20 PQC | https://www.redhat.com/en/blog/deeper-look-post-quantum-cryptography-support-red-hat-openshift-420-control-plane | OpenShift PQC control plane |
| Service Mesh 3.2 PQC | https://developers.redhat.com/articles/2025/12/18/quantum-secure-gateways-openshift-service-mesh | Quantum-secure gateways |

---

## Change Log

| Date | Change |
|---|---|
| 2026-03-13 | Initial tracker created. FIPS 203/204/205 published. RHEL 10.1 GA for PQC. |
| 2026-03-13 | Major update: FIPS hybrid exception (SecP256r1MLKEM768, SecP384r1MLKEM1024 work in FIPS mode). RPM hybrid signing milestone. OpenShift 4.20 PQC. Protocol standards status. Expanded monitoring sources to 16 URLs. |
