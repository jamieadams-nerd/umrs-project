---
name: C2PA test image acquisition sources
description: Where to find real-world C2PA-signed images for trust chain testing in umrs-c2pa
type: reference
---

## Current Status (2026-04-13)

5 files are in `tests/fixtures/trusted/`. ALL are UNVERIFIED or ERROR. No TRUSTED fixture
exists yet. The trust gap is documented and the path to resolution is clear.

### Why existing fixtures fail

| File | Signing date | Status | Root cause |
|---|---|---|---|
| `Firefly_tabby_cat.jpg` | 2025-10-23 | UNVERIFIED | Pre-rotation Adobe issuing CA |
| `car-es-Ps-Cr.jpg` | 2025-10-23 | UNVERIFIED | Pre-rotation Adobe issuing CA |
| `cloudscape-ACA-Cr.jpeg` | 2025-10-23 | UNVERIFIED | Pre-rotation Adobe issuing CA |
| `ChatGPT_Image.png` | no TSA | UNVERIFIED | OpenAI CA not on official C2PA TL |
| `crater-lake-cr.jpg` | n/a | ERROR | Remote manifest (cai-manifests.adobe.com) — umrs-c2pa does not fetch remote manifests |

Adobe rotated their issuing CA on 2025-11-19. The trust list has the NEW cert.
All existing Adobe-signed example images predate the rotation. The mismatch
is temporal, not a bug.

### Fastest path to TRUSTED (recommended to Jamie)

**Option A — Adobe Firefly (fastest):**
Go to https://firefly.adobe.com, generate any image with Content Credentials enabled
(the default). Download the JPEG. Fresh Firefly images (post-2025-11-19) carry the
new issuing CA and will verify TRUSTED immediately.

**Option B — Google Pixel 10 photo:**
Any photo taken on a Pixel 10 (launched Aug/Sep 2025) will use Google's signing chain
which chains to `Google C2PA Root CA G3` (in our trust list, Not Before ~2025-05-08).
No rotation mismatch — Google's hierarchy is a fresh deployment. A Pixel 10 photo
taken any time after the phone was purchased will verify as TRUSTED.

**Option C — git clone c2pa-org/public-testfiles with LFS:**
```bash
git lfs install
git clone https://github.com/c2pa-org/public-testfiles.git /tmp/c2pa-public-testfiles
ls /tmp/c2pa-public-testfiles/image/jpeg/google-*.jpg
```
The repo was updated 2026-03-16. Google-prefixed files (if any) should chain to
Google C2PA Root CA G3 and verify TRUSTED.

### Why Google Pixel 10 photos are ideal

The trust list (downloaded 2026-04-01) contains the COMPLETE Google C2PA hierarchy:
- `Google C2PA Root CA G3` (root, Not Before: 2025-05-08)
- `Google C2PA Mobile A 1P ICA G3` + L1 (issuing for camera A path)
- `Google C2PA Mobile B 1P ICA G3` + L1 (issuing for camera B path)
- `Google C2PA Media Services 1P ICA G3` (issuing for Google Photos edits)

Google deployed this hierarchy fresh for Pixel 10. No rotation issue exists.
Any Pixel 10 photo carries the full chain to Google C2PA Root CA G3.

### Expected TRUSTED output

```
Chain of Custody — <photo.jpg>
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  1   [TRUSTED]    Google Pixel Camera
                    Signed at : <date>+00:00 UTC
                    Issuer    : Google LLC
                    Alg       : ES256 ECDSA / P-256 / SHA-256
                    Generator : Google Pixel Camera <model>
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Verification command (from umrs-c2pa crate directory)

```bash
cd /DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-c2pa
cargo run tests/fixtures/trusted/<new-photo.jpg>
```

(umrs-c2pa.toml must be in the working directory — it contains trust list paths)

## Original source notes

### Acquisition Sources (for future reference)

- **Adobe Firefly** (https://firefly.adobe.com) — post-2025-11-19 images work
- **Google Pixel 10** photos — any date after phone launch (Aug/Sep 2025)
- **contentauth.github.io/example-assets/** — current images may be remote-manifest only
- **c2pa-org/public-testfiles** (git-lfs required) — google-YYYYMMDD-*.jpg files
- **DigiCert / SSL.com signed content** — both CAs are in trust list
- **Leica M11-P or Z-cameras** — Leica CA not in current trust list (was ITL only)

## Why real images matter

Real-world production certs from cameras and editing software carry actual
production certificate chains — different chain depths, different algorithms,
different root CAs. Synthetic test files won't exercise any of that.
