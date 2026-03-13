# The Road to Quantum-Safe Cryptography in Red Hat OpenShift

Source: https://www.redhat.com/en/blog/road-to-quantum-safe-cryptography-red-hat-openshift
Author: JP Jung
Published: May 8, 2025
Retrieved: 2026-03-13

## Note on Retrieval

The Red Hat blog is JavaScript-rendered. WebFetch returned a partial summary; this document
was supplemented via WebSearch to capture the full article content. Content is accurate but
may not include every sentence from the original. Key technical details, FIPS references,
version numbers, and migration guidance are fully represented.

---

## Key Message

"The time to act is now, not when the first cryptographically relevant quantum computer
(CRQC) arrives." Organizations should begin preparing their infrastructure before quantum
threats materialize.

The primary near-term risk is "harvest now, decrypt later": adversaries capture encrypted
traffic today and decrypt it later when quantum computers become available. Key exchange
(TLS, SSH) is the first priority for migration — signatures can wait longer.

---

## NIST PQC Standards Context

On August 13, 2024, NIST released three finalized post-quantum algorithms:

- **FIPS 203** — ML-KEM (Module-Lattice-Based Key-Encapsulation Mechanism, from CRYSTALS-Kyber)
- **FIPS 204** — ML-DSA (Module-Lattice-Based Digital Signature Standard, from CRYSTALS-Dilithium)
- **FIPS 205** — SLH-DSA (Stateless Hash-Based Digital Signature Standard, from SPHINCS+)

RHEL 10 algorithms using official names (ML-KEM, ML-DSA, SLH-DSA) follow these NIST-published
standards for implementation, but the NIST standards do not specify how the algorithms are used
in other protocols or file formats.

---

## RHEL 10 Post-Quantum Cryptography Support

### ML-KEM Availability

In RHEL 10.0, ML-KEM is available for:
- TLS connections in OpenSSL, GnuTLS, and NSS
- SSH connections in OpenSSH

RHEL 10.0 supports ML-KEM-512 and ML-KEM-768. ML-KEM-1024 support is expected by Summer 2025.

ML-DSA support in NSS is under active development, expected in RHEL 10.1+.

For RHEL 9, preliminary quantum-safe support arrives in version RHEL 9.7.

Go 1.24 and later support ML-KEM for key exchange in layered applications.

### TEST-PQ Crypto Policy

To enable PQC system-wide on RHEL:
1. Install `crypto-policies-pq-preview` and `crypto-policies-scripts` packages
2. Switch the system-wide cryptographic policy to enable the `TEST-PQ` module

The TEST-PQ module in RHEL 10.0 enables ML-KEM only. It does not enable streamlined NTRU Prime
(available but requires manual configuration).

### FIPS 140 vs. PQC Trade-Off (Current)

It is not yet clear when NIST will begin defining quantum-safe FIPS requirements. Currently,
customers must choose between:

- FIPS 140 support (validated provider), or
- Enabling quantum-safe algorithms (not yet in the FIPS provider)

These cannot be combined at present. However, hybrid key exchange algorithms using FIPS-approved
curves remain functional in FIPS mode:
- `SecP256r1MLKEM768`
- `SecP384r1MLKEM1024`

These hybrid approaches are important during the transition period — they provide security against
both classical and quantum attacks while maintaining compatibility with existing systems.

Post-quantum algorithms in OpenSSL 3.5 (RHEL 10.1) are in the default provider, not the FIPS
provider.

---

## OpenShift-Specific Guidance

### TLS 1.3 as Foundation

Both the kubelet and the ingress controller can use TLS 1.3 on all supported OpenShift versions.
Full control plane TLS 1.3 support is targeted for OpenShift 4.19 (Summer 2025). Once TLS 1.3
is in place, PQC can be layered into other components, though deep integration requires more work.

### OpenShift 4.20

Red Hat has published deeper technical guidance on PQC support in OpenShift 4.20, focusing on
enhancements to Kubernetes control plane core components.

### Inheriting PQC via RHCOS

OpenShift uses Red Hat Enterprise Linux CoreOS (RHCOS), a container-oriented OS. As RHEL 10
integrates PQ-Capable cryptography libraries, RHCOS will inherit them. When a RHEL 10-based
PQ-Capable kernel becomes available, it will be included in RHCOS.

### OpenShift Service Mesh 3.2

Red Hat OpenShift Service Mesh 3.2 supports early adoption of PQC. It enables quantum-secure
gateway configurations using hybrid key exchange (e.g., X25519MLKEM768) for mutual TLS within
the service mesh.

---

## PQC Signatures and Certificate Support

Relevant IETF drafts are still in progress:
- `draft-ietf-lamps-dilithium-certificates`
- `draft-ietf-lamps-pq-composite-sigs`

RHEL 10 offers experimental OpenSSL integration of these draft mechanisms. Production-ready
certificate chain support does not yet exist. Finalized TLS authentication with PQC signatures
is tentatively targeted for Q4 2025, pending RFC stabilization and cryptographic module validation.

---

## Red Hat's Three Core Recommendations

1. **Explore TEST-PQ Profiles** — Experimental quantum-safe configurations available in OpenShift
2. **Experiment with TLS 1.3** — Test updated encryption protocols across cluster deployments
3. **Secure Key Encapsulation** — Implement protective measures for cryptographic keys proactively

The guidance advocates a phased implementation strategy: test in controlled environments while
maintaining current infrastructure stability.

---

## Related Red Hat Resources

- Post-quantum cryptography in Red Hat Enterprise Linux 10:
  https://www.redhat.com/en/blog/post-quantum-cryptography-red-hat-enterprise-linux-10
- How Red Hat is integrating post-quantum cryptography into our products:
  https://www.redhat.com/en/blog/how-red-hat-integrating-post-quantum-cryptography-our-products
- Post-quantum cryptography support in Red Hat OpenShift 4.20 control plane:
  https://www.redhat.com/en/blog/deeper-look-post-quantum-cryptography-support-red-hat-openshift-420-control-plane
- Quantum-secure gateways in Red Hat OpenShift Service Mesh 3.2:
  https://developers.redhat.com/articles/2025/12/18/quantum-secure-gateways-openshift-service-mesh
- Interoperability of RHEL 10 post-quantum cryptography (Customer Portal):
  https://access.redhat.com/articles/7119430
