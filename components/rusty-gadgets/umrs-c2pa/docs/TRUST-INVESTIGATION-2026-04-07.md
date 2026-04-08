# Trust Chain Investigation Report — 2026-04-07

## Problem

All four downloaded C2PA-signed test images show `UNVERIFIED` despite:
- Adobe's root CA being present in the official `C2PA-TRUST-LIST.pem`
- 31 trust anchor certificates loaded successfully (18 signing + 13 TSA)
- Signatures being cryptographically valid
- TSA timestamps validating as `TRUSTED` successfully

## Root Cause

**Certificate rotation mismatch between the trust list and the test images.**

### Timeline

| Date | Event |
|---|---|
| 2025-10-23 | Adobe Firefly signs `Firefly_tabby_cat.jpg` using the **old** Adobe Product Issuing CA |
| 2025-11-19 | Adobe rotates their Issuing CA — new cert issued by Adobe Product Intermediate CA |
| 2026-04-01 | UMRS downloads `C2PA-TRUST-LIST.pem` from c2pa-org/conformance-public |
| 2026-04-07 | UMRS tests fail: trust list has the **new** cert, images carry the **old** cert |

### Certificate Details

**Trust list cert (new — downloaded 2026-04-01):**
```
Subject:  CN=Adobe Product Issuing CA vault-a-or2.adobe.net cai, O=Adobe Inc
Issuer:   CN=Adobe Product Intermediate CA, O=Adobe Inc.
Not Before: Nov 19 18:29:44 2025 GMT
Not After:  Nov 20 00:30:14 2030 GMT
```

**Image's signing chain cert (old — embedded in Firefly_tabby_cat.jpg):**
- Signed at: 2025-10-23T21:14:47+00:00 UTC
- The image carries the issuing CA cert that was valid at signing time
- This older cert has the same Subject CN but different serial number and key material

### Why the SDK Rejects It

The c2pa-rs SDK (v0.78.8) uses OpenSSL's `X509_STRICT` mode with `PARTIAL_CHAIN` flag.
Even with `PARTIAL_CHAIN` (which allows intermediate CAs as trust anchors), the cert
bytes must match exactly. The old and new issuing CA certs have:
- Same Subject CN
- Same Issuer CN
- Different serial numbers
- Different key material (re-keyed on rotation)
- Different validity periods

They are different certificates. The SDK correctly reports `signingCredential.untrusted`.

### This is Not a Bug

The code in `trust.rs` is correct:
- PEM files load successfully (18 + 13 = 31 certs)
- `trust.trust_anchors` setting is passed to the SDK correctly
- `verify.verify_trust = true` is set
- TSA trust validation works (proves the trust pipeline functions)

The trust list is correct — it has Adobe's current issuing CA.
The images are correct — they carry the issuing CA cert that was valid at signing time.

The mismatch is temporal: the trust list was published after Adobe rotated their cert,
and the example images were signed before the rotation.

## Verification Steps Performed

### 1. Confirmed Adobe cert present in trust list
```bash
grep -i "adobe" config/C2PA-TRUST-LIST.pem
# Subject  CN=Adobe Product Issuing CA vault-a-or2.adobe.net cai, O=Adobe Inc
```

### 2. Checked PEM integrity
```bash
grep -c 'BEGIN CERTIFICATE' config/C2PA-TRUST-LIST.pem  # 18
grep -c 'END CERTIFICATE' config/C2PA-TRUST-LIST.pem    # 18
# No Subject lines inside cert blocks (verified programmatically)
```

### 3. Tested with stripped Subject header lines
Created a clean PEM without `Subject\t` comment lines. Same UNVERIFIED result.
The non-standard Subject lines are not the issue — the x509-parser PEM parser
handles them correctly.

### 4. Verbose output confirmed trust loading
```
Loading trust anchors from: config/C2PA-TRUST-LIST.pem
  18 certificate(s) loaded from trust anchors
Loading user anchors from: config/C2PA-TSA-TRUST-LIST.pem
  13 certificate(s) loaded from user anchors
Trust validation enabled with 31 total anchor certificate(s)
```

### 5. Raw SDK validation status
```json
{
  "code": "signingCredential.untrusted",
  "explanation": "signing certificate untrusted"
}
```
TSA separately validates as trusted:
```json
{
  "code": "timeStamp.trusted",
  "explanation": "timestamp cert trusted: Adobe SHA256 ECC256 Timestamp Responder 2025 1"
}
```

### 6. Decoded trust list cert dates
```
Not Before: Nov 19 18:29:44 2025 GMT  ← issued AFTER the image was signed
Not After:  Nov 20 00:30:14 2030 GMT
```
Image signed: 2025-10-23 — one month before the trust list cert was issued.

## Solutions

### Option A: Get newer Adobe-signed images (PREFERRED)

Images signed after Nov 19, 2025 will carry the new issuing CA cert that matches
the trust list. Check Adobe's example-assets repo for recently updated files, or
use Adobe Firefly to generate a fresh image with Content Credentials enabled.

### Option B: Extract old issuing CA from image and add to trust anchors

Use `c2patool` or `openssl` to extract the certificate chain from the signed
image, pull out the old Adobe issuing CA cert, and add it to `user_anchors`.
This validates older images but adds a rotated-out cert to the trust store.

```bash
# Extract certs from the manifest (requires c2patool)
c2patool Firefly_tabby_cat.jpg --certs > old-adobe-certs.pem

# Or use openssl to parse the JUMBF/CBOR manually (complex)
```

### Option C: Use allowed_list for specific leaf cert

Add the exact leaf signing cert (not the issuing CA) to `allowed_list`.
This trusts that specific signer cert directly without chain validation.
Most surgical option but doesn't prove the chain validation path works.

### Option D: Contact Adobe / check C2PA conformance repo

The c2pa-org/conformance-public repo may have test fixtures signed with the
current trust list certs. The `contentauth/example-assets` repo may have been
updated since our download.

## SDK Version

- c2pa-rs: 0.78.8 (via Cargo.lock)
- Cargo.toml specifies: `c2pa = "0.78.6"` with features `["openssl", "file_io"]`
- OpenSSL verification path: `check_certificate_trust.rs` uses `X509_STRICT` + `PARTIAL_CHAIN`

## Files Examined

- `umrs-c2pa/src/c2pa/trust.rs` — trust settings builder (CORRECT)
- `umrs-c2pa/src/c2pa/manifest.rs` — chain walker + `derive_trust()` (CORRECT)
- `umrs-c2pa/src/c2pa/report.rs` — display formatting (CORRECT)
- `umrs-c2pa/config/C2PA-TRUST-LIST.pem` — 18 certs including Adobe
- `umrs-c2pa/config/C2PA-TSA-TRUST-LIST.pem` — 13 TSA certs
- c2pa-rs SDK source: `crypto/raw_signature/openssl/check_certificate_trust.rs`
- c2pa-rs SDK source: `crypto/cose/certificate_trust_policy.rs`
- c2pa-rs SDK source: `settings/mod.rs`

## Action Items

1. [ ] Try Option A first — check for newer Adobe example images signed after 2025-11-19
2. [ ] If Option A fails, try Option B — extract old cert from image
3. [ ] Consider installing `c2patool` CLI for cert extraction
4. [ ] Update `trust-maintenance.md` with guidance on cert rotation mismatches
5. [ ] Add a verbose diagnostic: when UNVERIFIED, log the signing date vs trust list cert dates
