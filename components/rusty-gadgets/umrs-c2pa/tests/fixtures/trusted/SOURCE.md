# C2PA Trusted Fixture Acquisition Record

**Purpose:** Production-signed images for exercising real certificate chains in umrs-c2pa
trust validation tests. These fixtures should verify as `TRUSTED` (or better-characterized
as trusted against a specific trust list) rather than `UNVERIFIED`.

**Created:** 2026-04-07
**Acquisition status:** DOWNLOADED 2026-04-07 — 4 files verified

## Downloaded Files

| Filename | Magic | Size | SHA-256 | Trust list | Source |
|---|---|---|---|---|---|
| `Firefly_tabby_cat.jpg` | ffd8ff (JPEG) | 1023K | `bdc5c19d4211d9c9f9f8dce5bf4e4dec29e9b32e308ff12a60979481e928154f` | Official 2025 (Adobe) | contentauth/example-assets (MIT) |
| `car-es-Ps-Cr.jpg` | ffd8ff (JPEG) | 567K | `17c9e5b9f40ef79bb8e8af4adf36fe2be54d9c4a7f96f29813e6c3850ab8caa8` | Official 2025 (Adobe) | contentauth/example-assets (MIT) |
| `cloudscape-ACA-Cr.jpeg` | ffd8ff (JPEG) | 663K | `eefaa04e4e2fe6d04a0473dc01ac64763281e666fadacf8e54db5cbe898c64cd` | Official 2025 (Adobe) | contentauth/example-assets (MIT) |
| `ChatGPT_Image.png` | 89504e (PNG) | 2.1M | `7cd57b4a33f05e4ce7f63de18751f59f4b3ea8c55c281204e0cd6d1f621e466f` | ITL (Microsoft/OpenAI) | contentauth/example-assets (MIT) |

**Truepic images NOT downloaded:** c2pa.org/public-testfiles uses Git LFS — `curl` returns HTML redirect or LFS pointer, not the actual JPEG. Would require `git clone` with LFS enabled. Deferred.

---

## Why this directory exists

The umrs-c2pa crate (delivered 2026-04-02) needs to exercise real root CA chains to test the
TRUSTED path. Existing fixtures (`test.jpg`, `test_umrs_signed.jpg`, and the `c2pa-rs/`
collection) are signed with test certificates that do not resolve against any production
trust list. They exercise manifest parsing and UNVERIFIED paths only.

---

## Trust list landscape (researched 2026-04-07)

Two trust lists are relevant:

### Official C2PA Trust List (2025 Conformance Program)
URL: `https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem`

Root CAs present as of 2026-04-07:
- **Adobe Inc.** — `Adobe Product Issuing CA vault-a-or2.adobe.net cai`
- **Google LLC** — `Google C2PA Root CA G3` (Pixel 10 camera, Google Photos, Pixel Camera app)
- **DigiCert, Inc.** — `DigiCert RSA4096 Root for C2PA G1` / `DigiCert ECC P384 Root for C2PA G1`
- **SSL Corporation** — `SSL.com C2PA RSA Root CA 2025` / `SSL.com C2PA ECC Root CA 2025`
- **Trufo Inc.** — `Trufo C2PA Root CA (2025, ECC P384)` (note: Trufo is different from Truepic)
- **Xiaomi Inc.** — `Xiaomi Root CA (EC-P384)`
- **Vivo Mobile Communication Co., Ltd.** — `Vivo Content Provenance and Authenticity Root CA`
- **Irdeto BV** — `Irdeto C2PA Root CA G1`
- **Tauth Labs Inc.** — `Tauth Root CA`

### Legacy Interim Trust List (ITL) — frozen 2026-01-01
URL: `https://contentcredentials.org/trust/anchors.pem`

Additional root CAs present (NOT in official trust list):
- **Microsoft** — used for Azure OpenAI / DALL-E 3 signed images
- **Leica Camera AG** — hardware camera signing (M11-P, SL3-S, etc.)
- **Truepic** — stands up its own C2PA CA; early adopter
- **Adobe** — also present here (redundant with official list)

**Important:** The ITL was frozen on 2026-01-01. No new entries. Existing certificates
signed during the ITL certificate validity period remain valid against the legacy trust model.
The official conformance program trust list is the current standard.

**Nikon revocation (September 2025):** Nikon revoked ALL C2PA certificates issued to date
after a security vulnerability was found in the Z6 III's multiple-exposure signing path.
The `nikon-20221019-building.jpeg` from public-testfiles will verify as REVOKED when
revocation checking is enabled. It should NOT be used as a TRUSTED fixture.

---

## Target fixtures and planned file list

### Tier 1: Official C2PA Trust List — Adobe root CA

These images are signed by Adobe production services (Firefly, Photoshop, Lightroom,
Adobe Content Authenticity website). Adobe's root CA is in the official 2025 trust list.

Source repository: `https://github.com/contentauth/example-assets`
Hosted at: `https://contentauth.github.io/example-assets/images/`
License: MIT (see repository LICENSE file)

| Filename | Creator | Source URL |
|---|---|---|
| `Firefly_tabby_cat.jpg` | Adobe Firefly (AI generated) | https://contentauth.github.io/example-assets/images/Firefly_tabby_cat.jpg |
| `car-es-Ps-Cr.jpg` | Adobe Photoshop (edited) | https://contentauth.github.io/example-assets/images/car-es-Ps-Cr.jpg |
| `cloudscape-ACA-Cr.jpeg` | Adobe Content Authenticity website | https://contentauth.github.io/example-assets/images/cloudscape-ACA-Cr.jpeg |
| `crater-lake-cr.jpg` | Adobe Lightroom (edited) | https://contentauth.github.io/example-assets/images/crater-lake-cr.jpg |

Adobe verify URL pattern:
`https://contentauthenticity.adobe.com/inspect?source=https://contentauth.github.io/example-assets/images/<filename>`

Cert chain: leaf cert → Adobe intermediate → `Adobe Product Issuing CA vault-a-or2.adobe.net cai` → Adobe root

### Tier 2: Legacy ITL — Truepic root CA

These three images are from the c2pa-org/public-testfiles conformance repository.
They were submitted by Truepic as a conforming generator product. Truepic's root CA
is on the legacy ITL (frozen 2026-01-01), not on the official 2025 trust list.

Source: `c2pa-org/public-testfiles` (CC BY-SA 4.0)
Hosted at: `https://c2pa.org/public-testfiles/image/jpeg/`

| Filename | Content | Source URL |
|---|---|---|
| `truepic-20230212-camera.jpg` | Camera/device capture | https://c2pa.org/public-testfiles/image/jpeg/truepic-20230212-camera.jpg |
| `truepic-20230212-landscape.jpg` | Landscape photograph | https://c2pa.org/public-testfiles/image/jpeg/truepic-20230212-landscape.jpg |
| `truepic-20230212-library.jpg` | Library interior | https://c2pa.org/public-testfiles/image/jpeg/truepic-20230212-library.jpg |

**Trust status note:** These files will verify as TRUSTED only when using the legacy ITL
(`contentcredentials.org/trust/anchors.pem`), not the official 2025 trust list. They
correctly exercise the ITL path — useful for testing that umrs-c2pa respects which
trust list is configured.

### Tier 3: Official C2PA Trust List — ChatGPT/OpenAI (pending trust list confirmation)

One OpenAI ChatGPT-generated image is in the example-assets repository.
OpenAI's signing CA is Microsoft-backed; Microsoft is on the ITL but NOT confirmed
on the official 2025 trust list as of research date.

| Filename | Creator | Source URL | Trust list |
|---|---|---|---|
| `ChatGPT_Image.png` | OpenAI ChatGPT / DALL-E | https://contentauth.github.io/example-assets/images/ChatGPT_Image.png | ITL only (unconfirmed) |

**Action required:** Before committing this fixture as a TRUSTED test case, verify
the cert chain by downloading and running:
```
c2patool ChatGPT_Image.png trust \
  --trust_anchors 'https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem'
```
If it shows TRUSTED against the official trust list, move it to Tier 1 and update this file.
If UNTRUSTED against the official list but TRUSTED against ITL, document it under Tier 2.

---

## Files intentionally excluded

| File | Reason |
|---|---|
| `nikon-20221019-building.jpeg` | Nikon revoked ALL C2PA certs (September 2025, Z6 III vulnerability). Will verify as REVOKED when revocation checking is enabled. Not suitable as a TRUSTED fixture. |
| `adobe-20220124-*.jpg` (public-testfiles) | README in c2pa-org/public-testfiles explicitly states these are signed with "C2PA Tool's test certificate, which is not on the Verify tool's temporary known certificate list." They will show as UNKNOWN SOURCE, not TRUSTED. |
| `truepic-20230212-*.jpg` (if ITL frozen causes issues) | Re-evaluate if umrs-c2pa requires official trust list compliance only |

---

## Download commands

Run from the repo root with Bash available. SHA-256 values must be filled in after download.

### Adobe Firefly / Photoshop / Lightroom (Tier 1)

```bash
DEST="/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-c2pa/tests/fixtures/trusted"

curl -L -s --max-time 30 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://contentauth.github.io/example-assets/images/Firefly_tabby_cat.jpg" \
  -o "${DEST}/Firefly_tabby_cat.jpg"

curl -L -s --max-time 30 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://contentauth.github.io/example-assets/images/car-es-Ps-Cr.jpg" \
  -o "${DEST}/car-es-Ps-Cr.jpg"

curl -L -s --max-time 30 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://contentauth.github.io/example-assets/images/cloudscape-ACA-Cr.jpeg" \
  -o "${DEST}/cloudscape-ACA-Cr.jpeg"

curl -L -s --max-time 30 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://contentauth.github.io/example-assets/images/crater-lake-cr.jpg" \
  -o "${DEST}/crater-lake-cr.jpg"

# Verify files are real JPEGs (not redirect stubs)
for f in "${DEST}"/Firefly_tabby_cat.jpg "${DEST}"/car-es-Ps-Cr.jpg \
          "${DEST}"/cloudscape-ACA-Cr.jpeg "${DEST}"/crater-lake-cr.jpg; do
  echo -n "${f}: "; head -c 3 "${f}" | xxd -p; done
```

### Optional ChatGPT image (Tier 3 — verify trust list first)

```bash
DEST="/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-c2pa/tests/fixtures/trusted"

curl -L -s --max-time 30 \
  -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
  "https://contentauth.github.io/example-assets/images/ChatGPT_Image.png" \
  -o "${DEST}/ChatGPT_Image.png"
```

### Truepic conformance files (Tier 2 — ITL only)

```bash
DEST="/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-c2pa/tests/fixtures/trusted"

for name in truepic-20230212-camera truepic-20230212-landscape truepic-20230212-library; do
  curl -L -s --max-time 30 \
    -A "Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0" \
    "https://c2pa.org/public-testfiles/image/jpeg/${name}.jpg" \
    -o "${DEST}/${name}.jpg"
done

# Verify
for f in "${DEST}"/truepic-20230212-*.jpg; do
  echo -n "${f}: "; head -c 3 "${f}" | xxd -p; done
```

### Checksum all downloaded files

```bash
DEST="/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-c2pa/tests/fixtures/trusted"
sha256sum "${DEST}"/*.jpg "${DEST}"/*.jpeg "${DEST}"/*.png 2>/dev/null
```

---

## Verification commands

### Verify against official C2PA Trust List (2025)

```bash
# Set up trust list
export C2PATOOL_TRUST_ANCHORS='https://raw.githubusercontent.com/c2pa-org/conformance-public/refs/heads/main/trust-list/C2PA-TRUST-LIST.pem'

# Verify each fixture
DEST="/DEVELOPMENT/umrs-project/components/rusty-gadgets/umrs-c2pa/tests/fixtures/trusted"
for f in "${DEST}"/*.jpg "${DEST}"/*.jpeg "${DEST}"/*.png; do
  echo "=== ${f} ===" && c2patool "${f}" trust --trust_anchors "${C2PATOOL_TRUST_ANCHORS}"
done
```

### Verify against legacy ITL (for Truepic files)

```bash
export C2PATOOL_TRUST_ANCHORS='https://contentcredentials.org/trust/anchors.pem'
c2patool truepic-20230212-camera.jpg trust --trust_anchors "${C2PATOOL_TRUST_ANCHORS}"
```

---

## Files downloaded (fill in after download)

| Filename | Download date | SHA-256 | Trust list | Verified TRUSTED? |
|---|---|---|---|---|
| `Firefly_tabby_cat.jpg` | — | — | Official C2PA TL | PENDING |
| `car-es-Ps-Cr.jpg` | — | — | Official C2PA TL | PENDING |
| `cloudscape-ACA-Cr.jpeg` | — | — | Official C2PA TL | PENDING |
| `crater-lake-cr.jpg` | — | — | Official C2PA TL | PENDING |
| `ChatGPT_Image.png` | — | — | Verify first | PENDING |
| `truepic-20230212-camera.jpg` | — | — | Legacy ITL | PENDING |
| `truepic-20230212-landscape.jpg` | — | — | Legacy ITL | PENDING |
| `truepic-20230212-library.jpg` | — | — | Legacy ITL | PENDING |

---

## License notes

- `example-assets` images (Adobe Firefly, Photoshop, Lightroom, ChatGPT): **MIT License**
  Source: https://github.com/contentauth/example-assets/blob/main/LICENSE
  No attribution requirement for test fixture use.

- `c2pa-org/public-testfiles` images (Truepic): **CC BY-SA 4.0**
  Source: https://github.com/c2pa-org/public-testfiles/blob/main/LICENSE
  Attribution required if redistributed. For internal test fixture use, no distribution occurs.
  Canonical attribution: "Truepic, from the C2PA public-testfiles collection (CC BY-SA 4.0)"

---

## Acquisition notes

**Session 2026-04-07:** Bash curl was blocked by sandbox. The `trusted/` directory was
created. This SOURCE.md was written based on web research. Files have NOT yet been
downloaded. Jamie: run the download commands above when Bash is available in a
tool-unrestricted session.

**Researcher:** The Librarian (T. Librarian)
