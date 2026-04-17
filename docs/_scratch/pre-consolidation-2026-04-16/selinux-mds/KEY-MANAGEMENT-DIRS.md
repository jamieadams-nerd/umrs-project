# UMRS Key Management: Directory Structure, FHS 3.0, and NIST Alignment

**Document:** UMRS-SEC-KM-001  
**Standard References:** FHS 3.0 (FreeDesktop, 2025), NIST SP 800-57 Part 1 Rev 5, NIST SP 800-53 Rev 5, CMMC Level 2  
**Applies To:** RHEL 10 / CentOS Stream 10, SELinux MLS enforcing mode  

---

## 1. Standards Overview

### 1.1 Filesystem Hierarchy Standard (FHS 3.0)

FHS 3.0 was originally published June 3, 2015 by the Linux Foundation and republished November 6, 2025 by FreeDesktop, which now maintains the specification. It is the **current and only active version**. There is no FHS 4.x.

FHS 3.0 defines four categories of files relevant to key material placement:

| Category | Definition | Examples |
|---|---|---|
| Static + Unshareable | Does not change without admin action; host-specific | `/etc/` config, private keys |
| Static + Shareable | Does not change without admin action; sharable | `/usr/` binaries, CA certs |
| Variable + Unshareable | Changes at runtime; host-specific | `/var/lib/` runtime state |
| Variable + Shareable | Changes at runtime; shareable | `/var/mail/` |

Key material is **static + unshareable** at rest. Runtime/session key state is **variable + unshareable**.

### 1.2 Relevant FHS 3.0 Sections

| Section | Path | Relevance |
|---|---|---|
| §3.5 | `/etc/` | Host-specific configuration and static secrets |
| §3.15 | `/var/` | Variable runtime data |
| §4.5 | `/opt/` | Add-on application packages |
| §3.13 | `/etc/opt/` | Configuration for `/opt/` packages |

FHS §3.5 explicitly states `/etc/` **must not contain binaries** and is reserved for host-local configuration. Private key material stored as inert files is consistent with this constraint.

### 1.3 NIST SP 800-57 Part 1 Rev 5 — Key Lifecycle States

SP 800-57 §5.3 defines the following states relevant to directory placement:

| State | Description | Directory Implication |
|---|---|---|
| Pre-activation | Generated, not yet in use | `/etc/keys/umrs/staging/` |
| Active | In use for protect/unprotect operations | `/etc/keys/umrs/<type>/` |
| Suspended | Temporarily unusable, not destroyed | `/var/lib/umrs/keys/suspended/` |
| Deactivated | No longer used to protect, may still unprotect | `/var/lib/umrs/keys/retired/` |
| Destroyed | Cryptographically erased | N/A — material removed |

---

## 2. Authoritative Directory Structure

```
/etc/pki/                               # System PKI root — RHEL convention, FHS §3.5
│                                       # NIST SC-12, SC-17, SP 800-57 §5.3.4
├── tls/
│   ├── certs/                          # Public certificates (CA bundles, host certs)
│   │   ├── ca-bundle.crt               # mode 0644 root:root — world-readable trust anchors
│   │   └── umrs-host.crt               # mode 0644 root:root
│   └── private/                        # Asymmetric private keys
│       └── umrs-host.key               # mode 0600 root:root — NIST SC-12(1)
│
├── ca-trust/                           # RHEL system trust anchor management
│   ├── source/anchors/                 # Drop zone for local/airgap CA certs
│   └── extracted/                      # Output of update-ca-trust(8)
│
├── rpm-gpg/                            # Package signing public keys (informational)
└── nssdb/                              # NSS shared certificate database

/etc/keys/                              # Application-layer key material — NIST SC-12, SP 800-57 §5.3
└── umrs/                               # Project namespace — mode 0700 root:root
    │
    ├── sealing/                        # Symmetric keys for LUKS/dm-crypt volume sealing
    │   │                               # NIST SC-28, SP 800-57 §5.3.5, SP 800-38F
    │   └── vol-root.key                # mode 0600 root:root
    │
    ├── signing/                        # IMA/DIGSIG asymmetric signing keys
    │   │                               # NIST SI-7, SC-12, SP 800-89
    │   └── ima-signing.key             # mode 0600 root:root
    │
    ├── wrapping/                       # Key Encryption Keys (KEKs) for key wrapping
    │   │                               # NIST SC-12(2), SP 800-38F (AES-KW / AES-KWP)
    │   └── kek-primary.key             # mode 0600 root:root
    │
    └── staging/                        # Pre-activation key material — SP 800-57 §5.3 pre-active state
        └── mode 0700 root:root

/etc/opt/umrs/                          # FHS §3.13 — configuration for /opt packages
│                                       # NOT for key material; policy and metadata ONLY
└── key-policy.toml                     # Key lifecycle policy — SP 800-57 §5.2, NIST PL-1, SA-8

/var/lib/umrs/                          # FHS §5.x — variable, persistent, host-specific runtime data
└── keys/                               # Runtime key lifecycle state — SP 800-57 §5.3
    │                                   # NIST SC-12, AC-3
    ├── active/                         # Currently loaded/unwrapped session keys
    │   └── mode 0700 root:root         # Short-lived; cleared on session end
    │
    ├── suspended/                      # Temporarily deactivated keys (SP 800-57 suspended state)
    │   └── mode 0700 root:root
    │
    └── retired/                        # Post-active, pre-destruction staging
        └── mode 0700 root:root         # NIST MP-6, SC-12 — awaiting secure deletion
```

---

## 3. Control Mapping by Path

| Path | NIST SP 800-53 Rev 5 | CMMC L2 | SP 800-57 Reference |
|---|---|---|---|
| `/etc/pki/tls/private/` | SC-12, SC-17, AC-3 | SC.L2-3.13.10 | §5.3.4 — asymmetric private key storage |
| `/etc/keys/umrs/sealing/` | SC-12(1), SC-28 | SC.L2-3.13.10 | §5.3.5 — symmetric key storage |
| `/etc/keys/umrs/signing/` | SI-7, SC-12 | SI.L2-3.14.6 | SP 800-89 — signing key assurance |
| `/etc/keys/umrs/wrapping/` | SC-12(2) | SC.L2-3.13.10 | SP 800-38F — key wrapping |
| `/etc/keys/umrs/staging/` | SC-12, CM-3 | SC.L2-3.13.10 | §5.3 — pre-activation state |
| `/etc/opt/umrs/key-policy.toml` | PL-1, SA-8 | — | §5.2 — key policy documentation |
| `/var/lib/umrs/keys/active/` | SC-12, AC-3, SC-4 | SC.L2-3.13.4 | §5.3.3 — key in active use |
| `/var/lib/umrs/keys/suspended/` | SC-12, AC-2 | — | §5.3 — suspended state |
| `/var/lib/umrs/keys/retired/` | SC-12, MP-6, MP-7 | MP.L2-3.8.3 | §5.3.6 — post-active/archival |

---

## 4. Filesystem Permissions Reference

All key-bearing directories and files must be owned `root:root`. No `setuid`, `setgid`, or world bits permitted.

| Path | Mode | Rationale |
|---|---|---|
| `/etc/keys/umrs/` | `0700` | Namespace root — no listing by non-root |
| `/etc/keys/umrs/*/` | `0700` | Subdirectory — no listing by non-root |
| `/etc/keys/umrs/**/*.key` | `0600` | Key files — root read/write only |
| `/etc/pki/tls/private/` | `0700` | RHEL convention; consistent with above |
| `/etc/pki/tls/private/*.key` | `0600` | Asymmetric private keys |
| `/etc/pki/tls/certs/` | `0755` | Public certs — world-readable is correct |
| `/var/lib/umrs/keys/` | `0700` | Runtime state root |
| `/var/lib/umrs/keys/*/` | `0700` | All lifecycle subdirs |

---

## 5. SELinux MLS Labeling Requirements

FHS defines *where* files live. SELinux MLS defines *who* can access them regardless of DAC permissions. Both layers are required. DAC alone (chmod/chown) is insufficient for MLS compliance.

### 5.1 Recommended SELinux Type Assignments

| Path | SELinux Type | MLS Sensitivity |
|---|---|---|
| `/etc/pki/tls/private/` | `cert_t` (extend to `umrs_privkey_t`) | `s3` or per-volume label |
| `/etc/keys/umrs/sealing/` | `umrs_seal_key_t` | Match protected volume sensitivity |
| `/etc/keys/umrs/signing/` | `umrs_sign_key_t` | `s3` |
| `/etc/keys/umrs/wrapping/` | `umrs_kek_t` | Highest sensitivity in use |
| `/var/lib/umrs/keys/active/` | `umrs_session_key_t` | Transient; matches active session level |
| `/var/lib/umrs/keys/retired/` | `umrs_retired_key_t` | Preserve original sensitivity label |

### 5.2 Applicable NIST Controls

- **SC-3** (Security Function Isolation): Key material paths inaccessible to processes below required clearance
- **AC-3(3)** (Mandatory Access Control): SELinux MLS enforces sensitivity-based access independent of DAC
- **SC-28(1)** (Cryptographic Protection of Data at Rest): Applies to retired key material pending destruction

### 5.3 Critical Rules

- No `unconfined_t` domain may have read access to any path under `/etc/keys/` or `/etc/pki/tls/private/`
- MLS constraints must prevent read-down: a process at `s1` cannot read material labeled `s3`
- `restorecon -RFv` must be run after any manual directory creation to apply file contexts
- Custom types (`umrs_*_t`) require a policy module; do not rely on default `file_t` fallback

---

## 6. Conflicts and Tensions Between FHS 3.0 and NIST

This is the most operationally significant section. FHS and NIST do not directly contradict each other, but they operate at different layers and create real tensions that require explicit design decisions.

### 6.1 Conflict: `/etc/` Immutability vs. Key Rotation

**FHS Position:** `/etc/` is for *static* configuration. Files here should not change without administrator intervention.

**NIST Position:** SP 800-57 §5.3 requires regular key rotation. SC-12 requires automated key management support. Keys in active directories will change on rotation schedules.

**Resolution for UMRS:**
- `/etc/keys/umrs/` treats key rotation as an administrative action (consistent with FHS) — rotation is a deliberate, logged, privileged operation, not an application runtime write
- Application processes **must not** write directly to `/etc/keys/umrs/` at runtime
- Rotation is performed by a privileged `umrs-keymgr` process/xtask, logging to journald (NIST AU-2, AU-3)
- This satisfies both: FHS static semantics preserved; NIST rotation requirement met via controlled admin path

### 6.2 Conflict: FHS `/var/lib/` Persistence vs. NIST Key Destruction

**FHS Position:** `/var/lib/` holds persistent application state that survives reboots.

**NIST Position:** SP 800-57 §5.3.6 and MP-6 require that destroyed keys are cryptographically unrecoverable. Retired keys in `/var/lib/umrs/keys/retired/` must eventually be securely deleted.

**Resolution for UMRS:**
- `/var/lib/umrs/keys/retired/` is a **staging area**, not permanent archival
- A destruction service must periodically execute secure deletion (`shred -u` or dm-crypt key slot wipe) against retired material
- Destruction events must produce an audit record (NIST AU-2, AU-9, MP-6)
- Do not rely on filesystem delete (`unlink`) — on journaling filesystems this does not guarantee data erasure

### 6.3 Conflict: FHS `/opt/` Autonomy vs. NIST Configuration Control

**FHS Position:** FHS §4.5 grants `/opt/<package>/` full directory autonomy. A package may place anything under its own subtree, including subdirectories named `etc/` or `keys/`.

**NIST Position:** SA-8 (Security Engineering Principles) and CM-6 require that configuration and secrets follow an auditable, policy-controlled layout. An ad-hoc `/opt/umrs/keys/` would be outside the system's standard audit and access control baseline.

**Resolution for UMRS:**
- UMRS **does not** use `/opt/umrs/keys/` or any key storage under `/opt/`
- `/etc/opt/umrs/` is used only for policy metadata (`key-policy.toml`), never raw key material
- This is a deliberate deviation from FHS §4.5 autonomy in favor of NIST CM-6 and SA-8 compliance
- Document this decision in the system security plan (SSP)

### 6.4 Conflict: FHS Shareable/Unshareable vs. NIST Need-to-Know

**FHS Position:** FHS encourages placing shareable files (e.g., public CA certs) in `/usr/share/` to allow network mounting.

**NIST Position:** SC-12 and AC-3 require that even public key material distribution be controlled. Freely network-mounting a CA cert directory may expose the trust anchor update mechanism to tampering.

**Resolution for UMRS:**
- Public CA certs remain in `/etc/pki/ca-trust/` (RHEL convention, non-shareable)
- UMRS does not place any PKI material under `/usr/share/`
- In airgap deployments, CA trust anchors are distributed via controlled media, not NFS/autofs

### 6.5 Summary: Conflicts and Resolutions

| Conflict | FHS Requirement | NIST Requirement | Resolution |
|---|---|---|---|
| Key rotation in `/etc/` | Static files only | Rotation required (SP 800-57 §5.3) | Rotation is a privileged admin operation, not runtime write |
| Secure deletion in `/var/lib/` | Persistent state | Cryptographic destruction (MP-6) | Retirement staging + active destruction service |
| `/opt/` autonomy for keys | Package owns subtree | Config control required (CM-6, SA-8) | No key material under `/opt/`; explicit SSP deviation note |
| Shareable public certs | Place in `/usr/share/` | Access control on trust anchors (SC-12) | Stay in `/etc/pki/`; no NFS mount of PKI paths |

---

## 7. Paths to Avoid

| Path | Reason |
|---|---|
| `/opt/umrs/keys/` | FHS autonomy conflicts with NIST CM-6; outside standard audit baseline |
| `/tmp/` | World-writable; violates SP 800-57 §5.3 unconditionally |
| `/home/<user>/` | User-accessible; violates AC-3, SC-12 |
| `/usr/share/` | Shareable mount point; violates SC-12 need-to-know for trust anchors |
| `/run/` | tmpfs; volatile across reboots; acceptable only for ephemeral session keys with explicit policy |
| `/proc/`, `/sys/` | Pseudo-filesystems; not for persistent material |
| `/etc/crypto-policies/` | RHEL system-owned; do not collide with this namespace |

---

## 8. Implementation Checklist

- [ ] Create `/etc/keys/umrs/` with `install -d -m 0700 -o root -g root`
- [ ] Create all subdirectories with same ownership/mode
- [ ] Write SELinux policy module defining `umrs_*_t` types
- [ ] Run `restorecon -RFv /etc/keys/umrs/` after policy load
- [ ] Verify no `unconfined_t` access via `sesearch --allow -t umrs_seal_key_t`
- [ ] Create `/etc/opt/umrs/key-policy.toml` with rotation schedules and algorithm selections
- [ ] Create `/var/lib/umrs/keys/{active,suspended,retired}/` with `0700 root:root`
- [ ] Implement destruction service for `/var/lib/umrs/keys/retired/` with journald audit logging
- [ ] Document FHS §4.5 deviation (no key material under `/opt/`) in SSP
- [ ] Verify `fips=1` kernel parameter active before any key generation (CMMC SC.L2-3.13.10)

---

## 9. Normative References

| Document | Sections | Topic |
|---|---|---|
| FHS 3.0 (FreeDesktop, 2025) | §3.5, §3.13, §3.15, §4.5 | Directory structure and semantics |
| NIST SP 800-57 Part 1 Rev 5 | §5.2, §5.3 | Key lifecycle and storage requirements |
| NIST SP 800-53 Rev 5 | AC-3, MP-6, PL-1, SA-8, SC-3, SC-4, SC-12, SC-17, SC-28, SI-7 | Security controls |
| NIST SP 800-38F | All | Key wrapping (AES-KW, AES-KWP) |
| NIST SP 800-89 | All | Signing key assurance |
| CMMC Level 2 | SC.L2-3.13.10, SI.L2-3.14.6, MP.L2-3.8.3 | DFARS compliance |
| RHEL 10 Security Guide | crypto-policies, SELinux MLS | Platform-specific implementation |








Can you write all of that up in .md instructions and explanations. Are there any conflicts bettween nist and FHS?

9:27 AM

Viewed a file, created a file, read a file
Viewed a file, created a file, read a file
Nine sections covering the full picture. The conflicts section (§6) is the most operationally important — the four tensions to be aware of:

Key rotation vs. /etc/ static semantics — resolved by making rotation a privileged admin operation, not a runtime write
Secure destruction vs. /var/lib/ persistence — resolved by treating retired/ as a staging area with an active destruction service
/opt/ autonomy vs. NIST CM-6 — UMRS deliberately avoids key material under /opt/; this deviation needs an SSP note
Shareable public certs vs. SC-12 need-to-know — stay in /etc/pki/, no NFS mounts of PKI paths
The FHS and NIST don't conflict at the letter level since FHS doesn't govern security. The tensions are architectural — FHS optimizes for interoperability and clean separation, NIST optimizes for auditability and least privilege. Where they pull in different directions, NIST wins and you document the deviation.


