# Trust List Maintenance Guide

**SPDX-License-Identifier: MIT**
**Copyright (c) 2025 Jamie Adams**

---

## Overview

UMRS C2PA validates media signatures against a set of trusted root certificates
(trust anchors). These certificates are stored as PEM files on disk. The operator
is responsible for obtaining, placing, and periodically updating these files.

There is no hardcoded default path. All trust file locations are set in
`umrs-c2pa.toml` under the `[trust]` section.

---

## Two Trust Lists — Signing CAs and TSA CAs

The C2PA ecosystem uses two distinct trust lists:

| File | Purpose | What it validates |
|---|---|---|
| `C2PA-TRUST-LIST.pem` | **Signing CA roots** | The certificate chain of whoever signed the manifest |
| `C2PA-TSA-TRUST-LIST.pem` | **Timestamp Authority CA roots** | The certificate chain of the TSA that timestamped the signature |

Both are PEM bundles of X.509 root CA certificates. Both are needed for full
trust validation.

### Why the TSA trust list matters

A TSA timestamp proves *when* a signature was created. Without the TSA trust
list, the SDK cannot verify that the timestamp was issued by a recognized
authority. The timestamp may still be present in the manifest, but its
authenticity cannot be confirmed.

### TSA validation is local — no internet required

Verifying an existing TSA timestamp embedded in a manifest is a purely local
cryptographic operation. The SDK checks the timestamp's certificate chain
against the TSA root CAs in the trust list. **The `internet` feature is not
required for TSA timestamp validation** — it only controls whether umrs-c2pa
can *contact* a TSA server to *obtain new* timestamps during signing.

| Operation | Requires `internet` feature? | Requires TSA trust list? |
|---|---|---|
| Verify existing TSA timestamp in a manifest | No | Yes |
| Obtain a new TSA timestamp when signing | Yes | No (signing, not verifying) |
| Sign without a TSA timestamp (air-gapped) | No | No |

### How to combine both trust lists

The c2pa SDK accepts a single PEM bundle for trust anchors. To include both
signing CAs and TSA CAs, concatenate them into one file:

```bash
cat C2PA-TRUST-LIST.pem C2PA-TSA-TRUST-LIST.pem > combined-trust-anchors.pem
```

Then point `trust_anchors` at the combined file. Alternatively, place the TSA
list in the `user_anchors` path — UMRS automatically combines `trust_anchors`
and `user_anchors` into a single bundle before passing them to the SDK.

**Recommended layout:**

```toml
[trust]
# C2PA signing CA roots + TSA CA roots (combined)
trust_anchors = "config/C2PA-TRUST-LIST.pem"

# TSA CA roots as a separate file (automatically merged with trust_anchors)
user_anchors = "config/C2PA-TSA-TRUST-LIST.pem"
```

This keeps the files independently updateable while ensuring both are loaded.

---

## What Goes in the Trust Anchor Files

Each trust anchor file is a PEM bundle containing one or more X.509 root CA
certificates. When UMRS validates a C2PA manifest signature, it walks the
certificate chain from the signer's leaf cert up to a root and checks whether
that root appears in the loaded trust bundle.

**If the root is present:** the manifest is marked `TRUSTED`.
**If the root is absent:** the manifest is marked `UNVERIFIED` (signature may
still be cryptographically valid, but the issuer is not recognized).

---

## Where to Get Trust Anchors

### C2PA Official Trust Lists

The C2PA consortium publishes official trust lists for both signing CAs and
timestamp authorities. These are the primary trust lists for validating content
credentials from cameras, software (Adobe, Microsoft, OpenAI, etc.), and other
C2PA-compliant tools.

**Download URLs:**

```
https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem
https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TSA-TRUST-LIST.pem
```

**Download procedure:**

```bash
curl -o config/C2PA-TRUST-LIST.pem \
  https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem

curl -o config/C2PA-TSA-TRUST-LIST.pem \
  https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TSA-TRUST-LIST.pem
```

Verify the downloads contain valid PEM:

```bash
grep -c 'BEGIN CERTIFICATE' config/C2PA-TRUST-LIST.pem
grep -c 'BEGIN CERTIFICATE' config/C2PA-TSA-TRUST-LIST.pem
```

**Note:** The c2pa-rs SDK also includes test root certificates in its source
tree (`sdk/tests/fixtures/certs/trust/test_cert_root_bundle.pem`). These are
for development and testing only — production deployments should use the
officially published trust anchors above.

### Organization Root CAs

If your organization issues its own signing certificates (e.g., for internal media
workflows), add your org's root CA to the `user_anchors` file. This is separate
from the C2PA official list so you can update them independently.

---

## Configuration

All paths are set in `umrs-c2pa.toml`:

```toml
[trust]
# C2PA signing CA roots
trust_anchors = "config/C2PA-TRUST-LIST.pem"

# TSA CA roots (automatically combined with trust_anchors at load time)
user_anchors = "config/C2PA-TSA-TRUST-LIST.pem"

# End-entity certificate allowlist (optional)
# Directly trust specific signer certs without chain validation.
# Can contain PEM certificates or base64-encoded SHA-256 hashes.
#allowed_list = "/opt/umrs/trust/allowed-signers.pem"

# EKU OID filter (optional)
# Only trust certificates with these Extended Key Usage OIDs.
# One OID per line, comments start with //
#trust_config = "/opt/umrs/trust/ekus.cfg"

# Master switch (default: true when any trust file is configured)
verify_trust = true
```

### How trust files are loaded

At startup, UMRS reads the PEM **content** from each configured path and passes
the certificate data to the c2pa SDK as in-memory strings. Paths are relative
to the current working directory (use absolute paths in production).

When both `trust_anchors` and `user_anchors` are configured, their PEM content
is concatenated into a single bundle before passing to the SDK. This is why
placing the TSA trust list in `user_anchors` works — both sets of root CAs end
up in the same trust bundle.

Verbose mode (`--verbose`) shows each file being loaded and the certificate count.

### Directory layout patterns

Choose whatever directory layout suits your deployment:

| Environment | Example Path |
|---|---|
| Development / local | `config/C2PA-TRUST-LIST.pem` |
| Server (FHS) | `/etc/umrs/trust/c2pa-anchors.pem` |
| Container | `/opt/umrs/trust/c2pa-anchors.pem` |
| Air-gapped RHEL | `/etc/pki/umrs/c2pa-anchors.pem` |

---

## Updating Trust Anchors

### Manual Update Procedure

1. **Download the updated PEM bundles** from the C2PA consortium:
   ```bash
   curl -o /tmp/C2PA-TRUST-LIST.pem \
     https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem

   curl -o /tmp/C2PA-TSA-TRUST-LIST.pem \
     https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TSA-TRUST-LIST.pem
   ```

2. **Verify the files are valid PEM:**
   ```bash
   openssl x509 -in /tmp/C2PA-TRUST-LIST.pem -noout -text | head -20
   grep -c 'BEGIN CERTIFICATE' /tmp/C2PA-TRUST-LIST.pem
   grep -c 'BEGIN CERTIFICATE' /tmp/C2PA-TSA-TRUST-LIST.pem
   ```

3. **Replace the files** at the configured paths:
   ```bash
   cp /tmp/C2PA-TRUST-LIST.pem config/C2PA-TRUST-LIST.pem
   cp /tmp/C2PA-TSA-TRUST-LIST.pem config/C2PA-TSA-TRUST-LIST.pem
   chmod 644 config/C2PA-TRUST-LIST.pem config/C2PA-TSA-TRUST-LIST.pem
   ```

4. **Verify UMRS reads the new lists** by inspecting a known-signed file:
   ```bash
   umrs-c2pa --verbose some-signed-image.jpg
   ```
   Verbose output shows each trust file being loaded with certificate counts.
   Look for `TRUSTED` in the chain-of-custody output where you previously saw
   `UNVERIFIED` or `NO TRUST LIST`.

### Update Frequency

- **C2PA official list:** Check quarterly, or when the consortium announces new
  CAs or revocations.
- **Organization roots:** Update when your PKI team rotates or reissues root
  certificates.
- **Allowed list:** Update when specific signer certificates are added or removed.

### Air-Gapped Environments

For systems without internet access:

1. Download the updated PEM on a connected workstation.
2. Transfer via approved media (USB, optical) per your security policy.
3. Place at the configured path.
4. Verify with `openssl x509` as above.

No UMRS code attempts to fetch trust material from the network.

---

## Which Trust Files Do I Need?

Not every deployment needs all four trust files. Start with just the trust
anchors and add the others only when you hit a specific need.

| Trust file | What it answers | You need it when... | You can skip it when... |
|---|---|---|---|
| **`trust_anchors`** | "Is this signer's CA recognized?" | Always — this is the foundation of trust validation | Never skip this one |
| **`user_anchors`** | "Are there additional CAs I trust?" | You have TSA roots, org-internal CAs, or partner CAs not in the C2PA list | You only validate against the official C2PA trust list |
| **`allowed_list`** | "Do I trust this specific signer?" | You need to trust a particular leaf certificate directly, without chain validation | All your signers chain to a root CA in your trust anchors |
| **`trust_config`** | "Is this cert allowed to sign C2PA?" | You want to restrict which certificate *purposes* (EKUs) are accepted | The SDK's built-in EKU defaults work for your environment |

### Typical deployment profiles

**Minimal (most deployments):** Just `trust_anchors` with the C2PA official list.
Validates signatures from Adobe, OpenAI, cameras, and other C2PA-compliant tools.

```toml
[trust]
trust_anchors = "config/C2PA-TRUST-LIST.pem"
verify_trust = true
```

**Standard (recommended):** Add the TSA trust list for timestamp validation.

```toml
[trust]
trust_anchors = "config/C2PA-TRUST-LIST.pem"
user_anchors  = "config/C2PA-TSA-TRUST-LIST.pem"
verify_trust = true
```

**Internal workflow:** Your org signs media with its own certificates. Add the
org root CA so those signatures validate as `TRUSTED` instead of `UNVERIFIED`.

```toml
[trust]
trust_anchors = "config/C2PA-TRUST-LIST.pem"
user_anchors  = "config/org-root-ca.pem"
verify_trust = true
```

**Mixed environment:** You receive media from a partner who uses self-signed
certificates. You trust the C2PA ecosystem broadly (trust anchors) and this
one specific partner cert directly (allowed list).

```toml
[trust]
trust_anchors = "config/C2PA-TRUST-LIST.pem"
user_anchors  = "config/C2PA-TSA-TRUST-LIST.pem"
allowed_list  = "config/partner-signer-cert.pem"
verify_trust = true
```

---

## End-Entity Allowlist (`allowed_list`)

### What it does

The allowlist lets you directly trust specific signer certificates without
requiring a chain to a root CA. Normal trust validation walks the certificate
chain from the signer's leaf cert up to a root CA and checks whether that root
is in your trust anchors. The allowlist bypasses that chain walk entirely — if
the signer's cert is in the allowlist, it is trusted directly.

### When to use it

**Scenario:** You inspect an image signed by an internal tool. The tool uses a
self-signed certificate (no CA chain at all). The image shows `UNVERIFIED`
because the cert's root isn't in any trust list — there is no root, it's
self-signed.

You have two options:

1. **Add your org's root CA to `user_anchors`** — validates the full chain.
   Use this when your org has a CA and issues multiple signing certs.
2. **Add the specific leaf cert to `allowed_list`** — trusts that one cert
   directly, no chain needed. Use this when you trust *that exact certificate*
   without trusting everything a CA ever issued.

Option 2 is more surgical. It says "I trust this specific signer" rather than
"I trust everything this CA has ever signed."

### File format

The allowlist file can contain:

1. **PEM certificates** — full `-----BEGIN CERTIFICATE-----` blocks
2. **Base64 SHA-256 hashes** — 44-character base64 strings (one per line), each
   being the SHA-256 hash of the certificate's DER encoding

Mixed format (some PEM blocks, some hashes) is supported in the same file.

### Example

Extract a signer's certificate from a signed image and add it to the allowlist:

```bash
# Inspect the image to see who signed it
umrs-c2pa --verbose signed-image.jpg

# Extract the signer cert (future feature: umrs-c2pa --certs)
# For now, use c2patool or openssl to extract the leaf cert

# Add it to your allowlist
cat partner-signer.pem >> config/allowed-signers.pem
```

---

## EKU Configuration (`trust_config`)

### What it does

EKU stands for Extended Key Usage — a field in X.509 certificates that declares
what the certificate is allowed to do. Common EKU purposes include email
signing, web server authentication, code signing, and C2PA content signing.

The EKU config file tells UMRS: "Only accept certificates that declare one of
these specific purposes." It is a filter on top of the trust anchors — even if
a cert chains to a trusted root, it will be rejected if its EKU doesn't match.

### When to use it

**Most deployments don't need this.** The c2pa SDK has built-in defaults that
cover the standard C2PA signing OIDs. You would configure this only if:

- You want to restrict accepted certificate purposes more tightly than the
  SDK defaults (e.g., reject email protection certs that some early signers used)
- Your organization's security policy requires explicit EKU whitelisting
- You are seeing `InvalidEku` errors and need to add an OID

**Scenario:** You receive images signed by a partner whose CA issued the signing
cert with an email protection EKU (`1.3.6.1.5.5.7.3.4`) instead of the proper
C2PA OID. The SDK's default EKU list might not include it. Add the OID to your
EKU config to accept those certs.

### File format

One OID per line in dotted-decimal notation. Lines starting with `//` are
comments.

```
// C2PA content credentials signing
1.3.6.1.4.1.62558.2.1

// Document signing
1.3.6.1.5.5.7.3.36

// Email protection (used by some early C2PA signers)
1.3.6.1.5.5.7.3.4

// Timestamping
1.3.6.1.5.5.7.3.8
```

If no EKU config is provided, the c2pa SDK uses its built-in default set (which
covers the standard C2PA OIDs listed above).

---

## Troubleshooting

| Symptom | Likely Cause | Fix |
|---|---|---|
| All manifests show `NO TRUST LIST` | No `[trust]` section or no trust files configured | Add `trust_anchors` to `[trust]` in `umrs-c2pa.toml` |
| All manifests show `UNVERIFIED` | Trust file loaded but signer's root CA isn't in it | Add the missing root CA to the PEM bundle |
| `UNVERIFIED` for your org's certs only | Org root not in `user_anchors` | Add org root CA to `user_anchors` file |
| TSA timestamps show as unverified | TSA trust list not loaded | Add `C2PA-TSA-TRUST-LIST.pem` as `user_anchors` |
| Config validation warns about trust | PEM file path doesn't exist or isn't readable | Check path and permissions (`644`) |
| `InvalidEku` errors | Signer cert's EKU not in allowed list | Add the OID to your EKU config file |
| Verbose shows 0 certificates loaded | PEM file is empty or corrupted | Re-download from the C2PA consortium URLs above |

---

## Attribution

This trust system follows patterns established by the c2pa-rs project
(MIT OR Apache-2.0, Copyright 2020 Adobe). Test trust fixtures used during
development originate from that project. See `tests/fixtures/ATTRIBUTION.md`.
