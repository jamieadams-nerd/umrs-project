# RHEL 10 Security Reference Guides

**Collection:** tech-writer-corpus / domain-refs / rhel-security-guide
**Created:** 2026-03-17
**Purpose:** RHEL 10 official documentation covering security hardening, SELinux (including MLS/MCS),
audit system, IMA/EVM, LUKS, FIPS, and crypto policy. Primary domain reference for writing
accurate UMRS documentation about RHEL 10 security features.

**Source:** docs.redhat.com (Red Hat official documentation — approved source)
**License:** Creative Commons Attribution-Share Alike 3.0 Unported (per Red Hat documentation notice)

---

## Files

### rhel10-security-hardening.pdf
**Title:** Red Hat Enterprise Linux 10 — Security Hardening
**URL:** https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/pdf/security_hardening/Red_Hat_Enterprise_Linux-10-Security_hardening-en-US.pdf
**Date retrieved:** 2026-03-17
**Last updated (per document):** 2026-03-16
**SHA-256:** f00f29908467bdb4d49f02183c5b385cb95731f62a4687d73730e4ee00510aa7
**Size:** 1.2 MB

Chapters:
- Ch 1: Switching RHEL to FIPS Mode
- Ch 2: Using System-Wide Cryptographic Policies
- Ch 3: Configuring Applications to Use Cryptographic Hardware (PKCS #11)
- Ch 4: Controlling Access to Smart Cards (polkit)
- Ch 5: Scanning the System for Configuration Compliance (OpenSCAP)
- Ch 6: Ensuring System Integrity with Keylime
- Ch 7: Checking Integrity with AIDE
- Ch 8: Managing Sudo Access
- Ch 9: Configuring Automated Unlocking of Encrypted Volumes (NBDE/Clevis/Tang)
- Ch 10: Blocking and Allowing Applications (fapolicyd)
- Ch 11: Protecting Systems Against Intrusive USB Devices

UMRS relevance: FIPS mode, crypto policies, LUKS/Clevis, integrity checking, compliance scanning

---

### rhel10-using-selinux.pdf
**Title:** Red Hat Enterprise Linux 10 — Using SELinux
**URL:** https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/pdf/using_selinux/Red_Hat_Enterprise_Linux-10-Using_SELinux-en-US.pdf
**Date retrieved:** 2026-03-17
**SHA-256:** 83cb891eec8ed9e89af34dc0396364ae3cd0ea18baf695ca0e4d88a6707baa91
**Size:** 625 KB

Chapters:
- Ch 1: Getting Started with SELinux
- Ch 2: Changing SELinux States and Modes
- Ch 3: Managing Confined and Unconfined Users
- Ch 4: Configuring SELinux for Applications and Services with Non-Standard Configurations
- Ch 5: Troubleshooting Problems Related to SELinux
- Ch 6: Using Multi-Level Security (MLS)
- Ch 7: Using Multi-Category Security (MCS) for Data Confidentiality
- Ch 8: Writing a Custom SELinux Policy
- Ch 9: Creating SELinux Policies for Containers
- Ch 10: Deploying the Same SELinux Configuration on Multiple Systems
- Ch 11: Configuring Polyinstantiated Directories

UMRS relevance: Core reference for SELinux MLS and MCS — directly maps to umrs-selinux crate

---

### rhel10-risk-reduction-recovery.pdf
**Title:** Red Hat Enterprise Linux 10 — Risk Reduction and Recovery Operations
**URL:** https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/pdf/risk_reduction_and_recovery_operations/Red_Hat_Enterprise_Linux-10-Risk_reduction_and_recovery_operations-en-US.pdf
**Date retrieved:** 2026-03-17
**SHA-256:** 5e72bc8ab214edf0fac9be74123aa3447ffc09a190dc447a271f3280badcb7da
**Size:** 645 KB

Chapters:
- Ch 1: Recovering and Restoring a System
- Ch 2: Troubleshooting Problems by Using Log Files
- Ch 3: Reviewing and Filtering Logs in the Web Console
- Ch 4: Configuring the systemd Journal
- Ch 5: Configuring a Remote Logging Solution
- Ch 6: Configuring Logging by Using RHEL System Roles
- Ch 7: Auditing the System  (primary relevance — auditd, audit rules, compliance)
- Ch 8: Managing and Monitoring Security

UMRS relevance: Audit subsystem (auditd) configuration and rules — maps to umrs-logspace

---

### rhel10-kernel-management.pdf
**Title:** Red Hat Enterprise Linux 10 — Managing, Monitoring, and Updating the Kernel
**URL:** https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/10/pdf/managing_monitoring_and_updating_the_kernel/Red_Hat_Enterprise_Linux-10-Managing_monitoring_and_updating_the_kernel-en-US.pdf
**Date retrieved:** 2026-03-17
**SHA-256:** 6b1501fb1e94a020acfe4d58541963e98e559392a45538bb929491c468bd7f3e
**Size:** 1.3 MB

Key chapters for UMRS:
- Ch 22: Signing a Kernel and Modules for Secure Boot
- Ch 23: Updating the Secure Boot Revocation List
- Ch 24: Enhancing Security with the Kernel Integrity Subsystem (IMA/EVM)

UMRS relevance: IMA-Measurement, IMA-Appraisal, IMA-Audit, EVM — maps to umrs-platform
integrity monitoring, kernel module signing

---

## Notes

- All four documents are from docs.redhat.com, an approved source per researcher role instructions
- "Last updated: 2026-03-16" appears on rhel10-security-hardening.pdf — collection is current
- IMA/EVM moved from Security Hardening guide (RHEL 8/9) to Kernel Management guide in RHEL 10
- LUKS block device encryption is in the "Managing Storage Devices" guide; the NBDE/Clevis
  automated unlocking topic remains in Security Hardening (Ch 9)
- SELinux MLS documentation (Ch 6 of Using SELinux) is the primary UMRS-relevant chapter
