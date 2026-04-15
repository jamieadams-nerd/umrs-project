# Plan: Staging Bundle CM-8 / SA-12 Full Implementation

**Status:** draft — deferred, not active
**Date drafted:** 2026-04-13
**Author:** Opus (session lead), at Jamie's direction
**Activation trigger:** Deployment / packaging workstream resumes; or ATO / CMMC evidence preparation begins; or first external adopter requests signed bundles.
**ROADMAP alignment:** G2 (Platform Library), G5 (Deployment & Operations), G9 (Compliance Evidence) — to be confirmed against current ROADMAP when plan is activated.

---

## Why This Plan Exists

`xtask stage` (landed 2026-04-13) cites NIST SP 800-53 CM-8 (component inventory) and SA-12 (supply chain protection). The citations describe the direction the code is heading, not the current posture. Today's artifact is:

- An in-source Rust constant (`EXPECTED_BINARIES`) listing five expected binaries.
- A runtime presence check: all five must land in `staging/bin/`.

That satisfies *count and names*, nothing more. An auditor asking *"what exactly was in this bundle on date X, and how do I know it wasn't tampered with?"* cannot be answered from the bundle itself today. This plan closes that gap.

---

## Authoritative References

| Reference | Applies to |
|---|---|
| NIST SP 800-53 Rev 5 CM-8 (Information System Component Inventory) | Phase 1 manifest |
| NIST SP 800-53 Rev 5 CM-8(3) (Automated Unauthorized Component Detection) | Phase 1 manifest verification on target |
| NIST SP 800-53 Rev 5 SA-12 (Supply Chain Protection) | Phases 2–4 signing / attestation |
| NIST SP 800-53 Rev 5 SI-7 (Software, Firmware, and Information Integrity) | Phase 3 IMA appraisal on deployment target |
| NIST SP 800-218 SSDF PS.2 (Protect All Forms of Code from Unauthorized Access and Tampering) | Phase 2 signature custody |
| NIST SP 800-218 SSDF PS.3 (Verify Third-Party Software Complies with Security Requirements) | Phase 1 SBOM inclusion |
| SLSA Build Level 3 specification | Phase 4 provenance attestation |
| CycloneDX 1.5 specification | Phase 1 SBOM format |
| In-toto attestation framework | Phase 4 provenance |
| RHEL 10 STIG — IMA / EVM requirements | Phase 3 target-side enforcement |

---

## Phase Overview

Four phases, ordered by value-delivered-per-unit-effort. Each phase produces an artifact that is useful on its own; later phases compose on top of earlier ones.

| Phase | Deliverable | Rough effort | Satisfies |
|---|---|---|---|
| 1 — Manifest | `staging/MANIFEST.json` + SBOM | ~1 day | CM-8 core |
| 2 — Manifest signature | Detached signature over MANIFEST.json | ~3 days incl. key setup | SA-12 minimum |
| 3 — SLSA provenance | `staging/provenance.intoto.jsonl` from CI | ~2 days | SA-12 + SSDF PS.2 (CI-driven) |
| 4 — IMA per-binary signing | `security.ima` xattrs on staging/bin/ | ~5 days incl. TPM enrollment | SA-12 full + SI-7 (target-side) |

---

## Phase 1 — Component Inventory Manifest (CM-8 Core)

### Scope

Produce a machine-readable inventory artifact at stage time that captures, for every item in `staging/bin/` and `staging/config/`:

- **Identity:** file name, crate name, semver
- **Provenance:** git commit SHA, git dirty flag, build timestamp (UTC, ISO 8601), builder host identity, Rust toolchain version, target triple, build profile (debug/release)
- **Integrity:** SHA-256 over the file contents (SHA-512 optional and recommended for long-retention deployments)
- **Dependency set:** embedded reference to a CycloneDX SBOM listing every transitive crate with version, license, and package-URL
- **Classification:** `kind = binary | script | config`

### Deliverables

- `staging/MANIFEST.json` — UMRS-schema inventory document
- `staging/sbom.cdx.json` — CycloneDX SBOM of the workspace
- New `stage_manifest()` phase in `xtask/src/stage.rs`, executed after `stage_binaries()` / `stage_scripts()` / `stage_configs()`
- `EXPECTED_BINARIES` check becomes "every declared binary appears as a manifest entry" rather than a filesystem presence check — the manifest is now authoritative
- Unit tests for manifest schema; integration test that stages a toy workspace and verifies manifest contents

### Tooling

- CycloneDX generation: `cargo-cyclonedx` (pin version; vendor if CI-constrained)
- Hashing: `sha2` crate
- Git metadata: `git2-rs` or shell out to `git rev-parse` with `run_cmd`
- JSON: `serde_json` (already in workspace)

### Compliance citations in source

```rust
//! ## Compliance
//!
//! - `NIST SP 800-53 CM-8` — Manifest is the authoritative component inventory
//!   for the staged bundle. Every binary and config file staged must appear
//!   as a manifest entry with identity, provenance, and integrity fields.
//! - `NIST SP 800-53 CM-8(3)` — The manifest enables automated detection of
//!   unauthorized components on the deployment target.
//! - `NIST SP 800-218 SSDF PS.3` — CycloneDX SBOM declares all third-party
//!   dependencies with version and license provenance.
```

### Acceptance criteria

1. `MANIFEST.json` validates against a committed JSON schema
2. Every binary in `EXPECTED_BINARIES` appears in the manifest
3. Every file listed in the manifest actually exists in `staging/`
4. SHA-256 in the manifest matches the file's actual hash
5. Manifest is deterministic across re-runs when inputs are identical (audit reproducibility)
6. Dirty git tree produces a manifest entry with `git_dirty: true` and a warning to stderr

### Out of scope for Phase 1

- Any signing of the manifest (Phase 2)
- Any per-binary signing (Phase 4)
- Any deployment-target verification tooling (separate plan)

---

## Phase 2 — Manifest Signature (SA-12 Minimum)

### Scope

Integrity-protect the Phase 1 manifest with a detached cryptographic signature so any holder of the builder's public key can verify the manifest is authentic and unmodified.

### Deliverables

- `staging/MANIFEST.json.sig` — detached signature over `MANIFEST.json`
- New `stage_sign_manifest()` phase in `stage.rs`, gated behind a `UMRS_SIGNING_KEY` environment variable (absent → phase skips with a WARNING)
- Key management runbook in `docs/modules/deployment/pages/signing-key-management.adoc`
- Public-key distribution mechanism (committed to repo as `keys/umrs-release.pub` with a documented rotation procedure)

### Tooling options (pick one during activation)

| Tool | Pros | Cons |
|---|---|---|
| `cosign sign-blob` | Standard in container ecosystem, OIDC keyless mode, Rekor transparency log | Dependency on external service for keyless |
| `minisign` | Tiny, offline, well-audited | Less ecosystem integration |
| `sequoia-sgp` (OpenPGP) | Widely recognized trust model | Heavy key management surface |
| `age` + `signify`-style | Simple, modern | Not OpenPGP-compatible |

**Leaning:** `cosign` with offline keys (not keyless) — widely understood, future-compatible with container supply-chain tooling, and Rekor transparency is optional.

### Compliance citations

```rust
//! - `NIST SP 800-53 SA-12` — Manifest signature is the minimum integrity
//!   protection for the pre-deployment bundle. Signature verification is the
//!   deployment target's first gate.
//! - `NIST SP 800-218 SSDF PS.2` — Signing key is protected via <mechanism>;
//!   see docs/modules/deployment/pages/signing-key-management.adoc for custody
//!   and rotation procedures.
```

### Acceptance criteria

1. `MANIFEST.json.sig` is produced whenever `UMRS_SIGNING_KEY` is present
2. Signature verifies against `keys/umrs-release.pub`
3. Mutating one byte of `MANIFEST.json` makes verification fail
4. Stage run without `UMRS_SIGNING_KEY` emits a clear WARNING (not an error) and skips the phase
5. Runbook documents rotation, revocation, and compromised-key response

### Open questions for activation

- TPM-backed key, HSM, or file-based with OS protection?
- Transparency log (Rekor) yes/no for Canadian govt context?
- Who holds the signing key — CI only, or also a maintainer offline backup?

---

## Phase 3 — SLSA Provenance (SA-12 + CI Attestation)

### Scope

Emit a signed SLSA provenance statement proving the bundle came from a specific source commit, built by a specific CI pipeline, using a specific builder image. A second independent build can verify byte-for-byte equivalence (reproducibility).

### Deliverables

- `staging/provenance.intoto.jsonl` — in-toto attestation, SLSA v1.0 predicate
- CI workflow integration (GitHub Actions `slsa-framework/slsa-github-generator` or equivalent for the chosen CI)
- Reproducibility pass: two independent builds from the same commit produce byte-identical bundles (modulo timestamps, which must be pinned via `SOURCE_DATE_EPOCH`)

### Prerequisites

- Phase 1 manifest must exist (provenance references it by hash)
- Reproducible-build audit pass first — we cannot claim SLSA L3 without deterministic output
- CI pipeline must run in an ephemeral environment (no persistent builder state)

### Tooling

- `slsa-framework/slsa-github-generator` if we end up on GitHub Actions
- `witness` or `in-toto-attestation` Rust crate for self-hosted CI
- `SOURCE_DATE_EPOCH` pinned from `git log -1 --format=%ct`

### Compliance citations

```rust
//! - `SLSA v1.0 Build Level 3` — Provenance is produced by a hardened CI
//!   pipeline in an ephemeral environment; predicate is signed and tamper-
//!   evident.
//! - `NIST SP 800-218 SSDF PS.2` — Build environment integrity is attested
//!   by the provenance document.
```

### Acceptance criteria

1. Provenance statement validates against SLSA v1.0 predicate schema
2. Provenance references the manifest by SHA-256
3. Two CI runs of the same commit produce byte-identical manifests (reproducibility proof)
4. Provenance is signed by the CI's OIDC identity (or equivalent ephemeral credential)

---

## Phase 4 — IMA Per-Binary Signing (SA-12 Full + Target-Side SI-7)

### Scope

Attach IMA (Integrity Measurement Architecture) signatures to every binary in `staging/bin/` as `security.ima` extended attributes. On a deployment target with IMA appraisal enabled, the kernel refuses to execute an unsigned or tampered binary.

This is the RHEL 10 / STIG-grade mechanism — the strongest integrity gate available on Linux without moving to signed initramfs / signed kernel modules.

### Deliverables

- `staging/bin/*` with `security.ima` xattrs populated
- New `stage_sign_ima()` phase in `stage.rs`, gated behind `UMRS_IMA_KEY`
- IMA key enrolled in the deployment target's kernel keyring (separate operational runbook)
- `docs/modules/deployment/pages/ima-enrollment.adoc` — target-side enrollment procedure
- Integration test using a loopback filesystem with IMA enabled

### Prerequisites

- Phases 1–2 complete (we need a signed manifest to bind IMA signatures back to)
- TPM or HSM-backed IMA signing key (file-based keys are acceptable for dev but not for production deployment)
- Deployment target with `ima_appraise=enforce` in kernel cmdline (or equivalent policy)

### Tooling

- `evmctl ima_sign` (from `ima-evm-utils`) — canonical signing tool
- A pure-Rust alternative exists but is less mature; recommend shelling out via `run_cmd` with tight argument construction
- Key enrollment: `keyctl padd` or `mokutil` depending on the target's trust root

### Compliance citations

```rust
//! - `NIST SP 800-53 SA-12` — Per-binary cryptographic signatures; tamper
//!   detection at execution time on the deployment target.
//! - `NIST SP 800-53 SI-7` — IMA appraisal on the target provides the
//!   runtime integrity gate required by SI-7(1) Integrity Checks.
//! - RHEL 10 STIG — IMA / EVM enforcement requirements.
```

### Acceptance criteria

1. Every file in `staging/bin/` has a non-empty `security.ima` xattr after this phase
2. `evmctl ima_verify` succeeds against the enrolled public key
3. Mutating one byte of a staged binary causes the kernel on a test target (with IMA enforce) to refuse to execute it
4. Stage run without `UMRS_IMA_KEY` emits a WARNING and skips the phase (IMA is optional, not mandatory — some deployment targets do not have IMA enabled)

### Operational considerations

- IMA signing key must NEVER live alongside the manifest signing key — different compromise profiles, different rotation cadences
- The IMA key's public half must be pre-loaded into the target's `.ima` keyring before first deployment, or the first install will fail
- Revocation of an IMA key invalidates every binary signed with it — rotation needs a coordinated re-deployment

---

## Cross-Phase Considerations

### Key Custody Matrix

| Key | Purpose | Custody target | Rotation cadence |
|---|---|---|---|
| Manifest signing key (Phase 2) | Signs `MANIFEST.json` | TPM or HSM on release host | Annual, or on maintainer turnover |
| CI OIDC identity (Phase 3) | Signs SLSA provenance | Ephemeral, per-run | Per-build (automatic) |
| IMA signing key (Phase 4) | Signs binaries | TPM on release host, distinct from manifest key | Annual, coordinated with target re-deployment |

### Evidence Bundle Layout After Full Implementation

```
staging/
├── bin/                       ← binaries with security.ima xattrs (Phase 4)
├── config/                    ← merged config files
├── scripts/                   ← end-user scripts
├── MANIFEST.json              ← inventory (Phase 1)
├── MANIFEST.json.sig          ← manifest signature (Phase 2)
├── sbom.cdx.json              ← SBOM (Phase 1)
├── sbom.cdx.json.sig          ← SBOM signature (Phase 2, optional)
└── provenance.intoto.jsonl    ← SLSA provenance (Phase 3)
```

An auditor asking *"what is this bundle and how do I know it's authentic?"* can answer using only these artifacts:

1. `MANIFEST.json` — what is in the bundle
2. `MANIFEST.json.sig` — manifest is authentic
3. `sbom.cdx.json` — what third-party code is in it
4. `provenance.intoto.jsonl` — it was built by our CI from a specific commit
5. IMA xattrs — the kernel itself will refuse to execute tampered copies

---

## Deferred Decisions (Surface at Activation)

1. **Manifest schema governance** — do we own the schema outright, or align with an existing standard (CycloneDX, SPDX, SWID)?
2. **Signing tool selection** — cosign vs minisign vs sequoia-sgp (Phase 2). Depends on Canadian govt context and adopter expectations.
3. **Reproducible build readiness** — do we have build determinism today? A pre-Phase-3 audit is required.
4. **Release host model** — single release host with TPM, or distributed signing via CI OIDC, or both?
5. **Revocation story** — how do we pull a bad release from deployment?
6. **IMA policy on target** — enforce or log-only for initial rollout?

---

## Not in Scope

- Reproducible builds as a standalone goal — referenced as a Phase 3 prerequisite; deserves its own plan if it turns out to be heavy
- Container images — UMRS ships native binaries; containerization is a separate distribution channel if it ever happens
- Deployment-target-side verification tooling — the bundle carries evidence; the installer that consumes it is a different component

---

## Review Checklist (for Jamie, at activation time)

- [ ] ROADMAP alignment re-confirmed against current roadmap
- [ ] Phase ordering still makes sense given current deployment posture
- [ ] Key custody model decided (per Deferred Decisions)
- [ ] Signing tool selected
- [ ] Canadian govt context reviewed (Henri) — do Treasury Board signing rules affect key custody?
- [ ] Reproducible-build readiness scoped as a pre-Phase-3 task
- [ ] Plan moved from `draft` to `approved` and into active ROADMAP
