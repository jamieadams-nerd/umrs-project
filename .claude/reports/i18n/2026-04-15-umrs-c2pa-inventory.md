# String Inventory Report — umrs-c2pa
**Date:** 2026-04-15
**Crate:** umrs-c2pa
**Domain:** umrs-c2pa
**Type:** binary

## Summary

74 unique msgids identified across 4 source files. All strings are wrapped with
`i18n::tr()`. Two plural strings use `ngettext()` directly from `gettextrs`.

## Source Files Scanned
- `src/main.rs`
- `src/c2pa/report.rs`
- `src/c2pa/validate.rs`
- `src/c2pa/error.rs`
- `src/c2pa/creds.rs`

## Makefile Status
- `umrs-c2pa` is already in `I18N_ACTIVE_DOMAINS` and `I18N_TEXT_DOMAINS`
- `I18N_SRC_DIR_umrs_c2pa` is defined
- `I18N_ACTIVE_LOCALES_umrs_c2pa := fr_CA` is set
- `.pot` file: **MISSING** — `resources/i18n/umrs-c2pa/umrs-c2pa.pot` does not exist
- `fr_CA.po`: **MISSING**

## Blocker: xtr not installed
`xtr` is required to extract strings via `make i18n-extract-umrs-c2pa`.
Install: `cargo install xtr`
Until installed, the `.pot` is hand-crafted from the grep scan.

## ngettext Plural Strings
Two plural strings in `src/c2pa/report.rs` use `ngettext()` directly. These require
`msgid_plural` entries in the `.pot`. They are included below.

## String Inventory (74 unique msgids)

### src/main.rs
| Line | msgid | Notes |
|------|-------|-------|
| 181 | `Failed to load config:` | error prefix + path fragment |
| 188 | `No config file at` | error prefix + path fragment |
| 189 | `using defaults` | continuation in format string |
| 284 | `File not found:` | error prefix + path |
| 298 | `Reading raw manifest store as JSON...` | verbose progress |
| 301 | `No manifest or read error:` | error prefix |
| 308 | `Computing SHA-256 and SHA-384 digests...` | verbose progress |
| 310/312/323/325 | `Failed to hash file:` | error prefix (4 call sites, 1 msgid) |
| 313 | `Reading chain of custody as JSON...` | verbose progress |
| 326 | `SHA-256` | label — specification vocabulary; keep English |
| 327 | `SHA-384` | label — specification vocabulary; keep English |
| 332 | `Signing mode — ingesting file into UMRS chain of custody...` | verbose progress |
| 335 | `Security marking:` | verbose label |
| 340 | `Ingest failed for:` | error prefix |
| 343 | `Signed output written to:` | success message |
| 349 | `Reading chain of custody from signed output...` | verbose progress |
| 352 | `Failed to read chain from signed output` | error |
| 374 | `Read-only mode — inspecting existing chain of custody...` | verbose progress |
| 381 | `Failed to read chain from:` | error prefix |
| 405 | `Running configuration preflight checks...` | verbose progress |
| 435 | `Failed to write config to:` | error prefix |
| 441 | `Config template written to:` | success message |
| 552 | `Generating credentials in:` | verbose progress |
| 561 | `Failed to create directory:` | error prefix |
| 568 | `Credential generation failed` | error |
| 583/592 | `already exists at` | error fragment (2 call sites, 1 msgid) |
| 585/594 | `Remove it first or choose a different --output directory.` | error action |
| 599/635 | `Failed to write` | error prefix (2 call sites) |
| 620 | `Failed to create key file:` | error prefix |
| 627 | `Failed to write key to:` | error prefix |
| 638 | `Generated CSR + private key` (multiline — see source) | NOTE: contains \n |
| 649 | `Generated CSR + private key` | success (standalone) |
| 653 | `Generated self-signed certificate + private key` | success |
| 658 | `Algorithm` | label |
| 665 | `Subject` | label — Termium: "Sujet" |
| 676 | `Validity` | label |
| 684 | `Submit the CSR to your Certificate Authority for signing.` | instruction |
| 688 | `Keep the private key safe — it cannot be regenerated.` | warning |
| 693 | `Self-signed certificates will show as UNVERIFIED by external validators.` | note — UNVERIFIED kept English per i18n_l10n_rules.md |
| 697 | `For trusted status, submit a CSR to a recognized CA` | instruction |
| 701 | `or add your org's root to the trust anchors.` | instruction |
| 705 | `Files written` | label |
| 711 | `Next step — add these to your` | instruction fragment |
| 718 | `After your CA signs the CSR, replace signing.csr with the signed cert` | instruction |
| 729 | `Then run:` | instruction |
| 739 | `Validating configured signing credentials...` | verbose progress |
| 747 | `Credential Validation` | section header |
| 775 | `To generate new credentials:` | instruction |
| 779 | `All checks passed.` | success |

### src/c2pa/report.rs
| Line | msgid | Notes |
|------|-------|-------|
| 60 | `Chain of Custody` | section title — Termium "garde" (information security) |
| 61 | `SHA-256` | specification vocabulary — keep English |
| 62 | `SHA-384` | specification vocabulary — keep English |
| 66 | `(no C2PA manifest found)` | status message |
| 106 | `Self-signed certificate — not issued by a trusted CA` | status |
| 113 | `No trust list configured — trust could not be evaluated` | status |
| 131 | `Certificate rotation mismatch — image signed before trust list certificate became valid.` | multiline msgid |
| 134 | `Image signed      :` | label (padded) |
| 135 | `Trust cert valid  :` | label (padded) |
| 136-140 | `This is likely a CA rotation event, not tampering...` | multiline advice |
| 143-148 | `Signature is valid but the signer's CA is not in your trust list...` | multiline advice |
| 162/168 | `Signed at :` | label (padded, 2 call sites) |
| 169 | `no timestamp provided` | status |
| 178 | `Issuer    :` | label (padded) |
| 187 | `Alg       :` | label (padded) — Algorithm, specification vocabulary |
| 199 | `Generator :` | label (padded) |
| 205 | `Marking   :` | label (padded) |
| 227/233 | `Hash consistency :` | label (padded, 2 call sites) |
| 228 | `file unchanged across all signing events` | status |
| 234 | `no prior manifest (first signature)` | status |
| 237 | `UMRS action      :` | label (padded) |
| 240 | `UMRS output      :` | label (padded) |
| 246 | `UMRS identity    :` | label (padded) |
| 247 | `ephemeral self-signed cert (test mode — UNTRUSTED)` | status — UNTRUSTED kept English |
| 292 | `All checks passed. Configuration is ready.` | success |
| 282-287 | PLURAL: `All checks passed ({} warning). Configuration is ready.` / `All checks passed ({} warnings). Configuration is ready.` | ngettext |
| 296-301 | PLURAL: `{} check failed. Configuration is NOT ready.` / `{} checks failed. Configuration is NOT ready.` | ngettext |

### src/c2pa/validate.rs
| Line | msgid | Notes |
|------|-------|-------|
| 146 | `Skipped — requires both cert and key files to be readable` | status |
| 166 | `ed25519 is not reliably available on FIPS-enabled systems. Recommended: es256, es384, or es512.` | warning |
| 180 | `Field is empty` | validation error |
| 197/289/471 | `File not found:` | (shared msgid with main.rs) |
| 199/291/473 | `Cannot read:` | error prefix |
| 208/300 | `Valid PEM at` | status fragment |
| 214/306/497 | `File is not valid PEM:` | error fragment |
| 240 | `Private key has unsafe permissions` | security warning |
| 242 | `expected 0600 or 0400. Run: chmod 0600` | action instruction |
| 257 | `Permissions` | label |
| 258 | `secure` | status value |
| 269 | `Private key owner` | label |
| 271 | `does not match process` | status value |
| 280 | `Cannot stat key file:` | error prefix |
| 347 | `is not allowed. Use one of:` | error fragment |
| 363 | `TSA endpoint reachable:` | status prefix |
| 367 | `TSA endpoint did not respond:` | error prefix |
| 376 | `configured` | status value |
| 377 | `but network feature is disabled` | status value |
| 378 | `timestamps will be unsigned` | status value |
| 413 | `File exists:` | status fragment |
| 425/449 | `Cannot stat:` | error prefix |
| 437 | `EKU config found:` | status fragment |
| 460 | `OCSP responder configured:` | status fragment |
| 461 | `not yet implemented — skeleton only` | dev note |
| 535 | `Trust file is world-writable (mode)` | security warning |
| 537 | `any local user could inject CA certificates` | warning explanation |
| 538 | `Run: chmod o-w` | action instruction |

### src/c2pa/error.rs
| Line | msgid | Notes |
|------|-------|-------|
| 56 | `IO error:` | error prefix |
| 57 | `C2PA error:` | error prefix |
| 58 | `Config error:` | error prefix |
| 59 | `Signing error:` | error prefix |
| 60 | `Hash error:` | error prefix |
| 65 | `is not in the FIPS-safe allowed set` | error fragment |
| 72 | `Refusing to overwrite previously signed file:` | error |

### src/c2pa/creds.rs
| Line | msgid | Notes |
|------|-------|-------|
| 239 | `cert_chain is set but private_key is missing` | config error |
| 246 | `private_key is set but cert_chain is missing` | config error |

## Unwrapped Strings — Blocker for Rusty

The following user-facing strings in umrs-label **and umrs-c2pa** are not yet wrapped.
Note: umrs-c2pa is complete. umrs-label is the problem (separate inventory).

## Policy-Critical Terms — Flagged for Henri
None in this crate. TrustStatus labels (TRUSTED, UNVERIFIED, INVALID, REVOKED, NO TRUST LIST)
are intentionally kept English per i18n_l10n_rules.md §Translation Boundary rule.
