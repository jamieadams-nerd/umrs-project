# Plan: Absorb Jamie's CPU/Crypto Experiment Artifacts

**Status:** COMPLETE (2026-03-18)
**ROADMAP:** G3 (CPU security posture), G6 (crypto assurance)
**Target plan:** `.claude/plans/cpu-extension-probe.md` — §Empirical Reference Data

---

## Context

Jamie ran hands-on CPU/crypto/IMA experiments on Ubuntu (octopussy VM). The artifacts in `.claude/jamies_brain/cpu-work/` plus `create_ima_keys.sh` are **active reference material** feeding into the CPU extension probe work in `.claude/plans/cpu-extension-probe.md`. This plan absorbs the knowledge, places scripts where Rusty can use them, and updates the CPU extension probe plan with the empirical findings.

## What These Artifacts Prove

Jamie's experiments on octopussy empirically validate the three-layer activation model:
- **Layer 1** (hardware): `cpu_info.sh` output confirms ARM CE flags (aes, pmull, sha1, sha2, sha3, sha512, crc32, atomics, paca, pacg)
- **Layer 2** (OS): kernel crypto drivers confirmed (aes-ce, gcm-aes-ce, sha256-ce, sha3-*-ce)
- **Layer 3** (software): `umrs-openssl-audit.sh` proves OpenSSL 3.0.13 actively uses ARM CE (8.9 GB/s AES-GCM, 3.3 GB/s SHA-256 — hardware-accelerated speeds)

The OpenSSL audit is especially significant: it's a **system-wide trust anchor**. Every binary that links against it inherits (or fails to inherit) those hardening properties. Binary analysis tooling (`umrs-file-stat`, future `umrs-cpuaudit`) must trace this linkage.

Cross-ref: `docs/modules/cryptography/pages/openssl-no-vendoring.adoc`

---

## Step 1 — Save Memory: Ubuntu Crypto Posture Baseline

Create `project_ubuntu_crypto_baseline.md`:
- Platform: Ubuntu 24.04.3, ARM64 (octopussy, Parallels), kernel 6.14.0
- OpenSSL 3.0.13, provider-only (no FIPS, no legacy, no engine), `-mbranch-protection=standard`
- ARM CE: AES, PMULL, SHA1, SHA2, SHA3, CRC32, SHA512, NEON, Atomics (`OPENSSL_armcap=0xfd`)
- Kernel crypto parity: aes-ce, gcm-aes-ce, sha256-ce, sha3-*-ce
- Performance: AES-128-GCM ~8.9 GB/s, SHA-256 ~3.3 GB/s, SHA3-256 ~1.1 GB/s
- Validated UMRS crypto choices: AES-256-GCM, ED25519, X25519, SHA-256/SHA-3, ChaCha20-Poly1305
- Ubuntu FIPS: no real FIPS without Ubuntu Pro; real FIPS on RHEL 10
- Binary hardening: readelf/elfdump → first-class in cpu extension probe
- System-wide: OpenSSL is a trust anchor; binary analysis must trace linkage
- Cross-ref: `docs/modules/cryptography/pages/openssl-no-vendoring.adoc`

## Step 2 — Save Memory: Ubuntu Ops Baseline

Create `project_ubuntu_ops_baseline.md`:
- Ubuntu ops baseline distinct from dev workstation
- Key services: auditd, rsyslog, chrony (not timesyncd), fail2ban (no Postfix)
- AIDE requires manual `_aide` user (Ubuntu packaging bug)
- Ubuntu = modeling+dev+demo; RHEL 10 = compliance+enforcement+cert

## Step 3 — Move Active Scripts to Rusty's Reference Area

Move to `.claude/agent-memory/rust-developer/reference/cpu-crypto/`:
- `cpu_info.sh` — CPU audit collection (Layer 1 + Layer 2 detection reference)
- `umrs-openssl-audit.sh` — OpenSSL posture audit (Layer 3 detection reference)
- `create_ima_keys.sh` — IMA/EVM key generation reference
- `ubuntu/ima-reresh.sh` — IMA/EVM re-signing reference

## Step 4 — Archive Analysis Documents

Move to `.claude/jamies_brain/archive/cpu-experiments/`:
- `cpuaudit_report.txt`, `umrs-cpuaudit-linux.txt`
- `ubuntu/devel-packages.sh`, `umrs-ops-setup.sh`, `umrs-ops-verify.sh`, `umrs-ubuntu-system.txt`, `test_violation.sh`

## Step 5 — Leave `fantastic_keyboard.rs` in `jamies_brain/`

No action needed — it stays.

## Step 6 — Remove Empty `cpu-work/` Directory

After steps 3 and 4 move all contents out, remove the empty directory tree.

## Step 7 — Update `umrs-platform-expansion.md` §CPU Extension Detection

Add to the CPU Extension Detection section:
- **Empirical reference data:** Jamie's Ubuntu experiments in `.claude/agent-memory/rust-developer/reference/cpu-crypto/` provide working examples of all three detection layers
- **OpenSSL as system-wide trust anchor:** binary analysis must assess whether linked binaries inherit OpenSSL's hardening properties (ARM CE, branch protection, provider-only mode)
- **`umrs-openssl-audit.sh`** covers: version/providers, algorithm surface (cipher/digest/mac/pubkey/sig), engine status, ARM crypto acceleration benchmarks, kernel crypto API cross-reference
- **readelf/elfdump** binary hardening evidence (BTI, PAC, RELRO, PIE, NX) is a first-class requirement for the probe

## Step 8 — Update Rusty's MEMORY.md + Project MEMORY.md

- Rusty: pointer to `reference/cpu-crypto/` with note about OpenSSL and IMA references
- Project: pointers to the two new memory files

---

## Verification

- `ls .claude/jamies_brain/cpu-work/` → gone
- `ls .claude/jamies_brain/create_ima_keys.sh` → gone
- `ls .claude/jamies_brain/fantastic_keyboard.rs` → still there
- `ls .claude/agent-memory/rust-developer/reference/cpu-crypto/` → 4 active scripts
- `ls .claude/jamies_brain/archive/cpu-experiments/` → 7 archived files
- Memory files created and indexed
- `umrs-platform-expansion.md` updated with empirical reference data
