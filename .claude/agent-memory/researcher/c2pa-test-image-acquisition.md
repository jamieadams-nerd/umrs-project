---
name: C2PA test image acquisition sources
description: Where to find real-world C2PA-signed images for trust chain testing in umrs-c2pa
type: reference
---

## Purpose

umrs-c2pa needs real vendor-signed images to test trust chain validation.
Self-signed and test certs only exercise the UNVERIFIED path. We need images
that resolve as TRUSTED against the official C2PA trust list.

## Acquisition Sources (by likelihood of C2PA manifests)

### Flickr
Best bet. Photojournalists and serious photographers upload from C2PA-enabled
cameras. Search for images tagged "content credentials" or "leica m11-p".

### 500px
Professional photographers, higher likelihood of Leica/Sony Alpha gear with
C2PA enabled.

### Adobe Stock / Behance
Adobe actively pushes Content Credentials on their platforms. Exports from
Photoshop/Firefly with credentials attached are common.

### Associated Press / Reuters photo feeds
AP and Reuters are active C2PA members piloting credentialed photojournalism.
Public-facing sites occasionally have credentialed images.

## Verification Method

Use the Content Authenticity Initiative online verifier at
`contentcredentials.org/verify` — drag and drop to confirm a valid manifest
exists before downloading. No toolchain needed.

## Target Vendors (different cert chains and algorithms)

| Vendor | Root CA | Why it matters |
|---|---|---|
| Adobe | Adobe root | Definitely in C2PA-TRUST-LIST.pem. Should be first TRUSTED result. |
| Leica | D-Trust (Bundesdruckerei) | Hardware camera trust chain. |
| Sony | Sony root (α9 III, α1 II) | Different camera manufacturer chain. |
| OpenAI/Truepic | Truepic root | Already have images (jamie_desk.png) — need to determine if root is in trust list. |

## Output

Place acquired images in `umrs-c2pa/tests/fixtures/trusted/` with a `SOURCE.md`
documenting provenance, download date, and verification status for each file.

## Why real images matter

Real-world production certs from cameras and editing software carry actual
production certificate chains — different chain depths, different algorithms,
different root CAs. Synthetic test files won't exercise any of that.
