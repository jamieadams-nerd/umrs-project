# Source: Filesystem Hierarchy Standard 3.0

## Document Information

| Field | Value |
|---|---|
| Full title | Filesystem Hierarchy Standard |
| Version | 3.0 |
| Issuing authority | LSB Workgroup, The Linux Foundation |
| Published | 2015 |
| Retrieval date | 2026-04-18 |
| Retrieval method | curl -L (HTTP 200, direct download) |

## Files

### fhs-3.0.txt

| Field | Value |
|---|---|
| Source URL | https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.txt |
| SHA-256 | ec52379984c85fdeddea6fabd5a84c8c358016e4d7c616995c2b147451d127b3 |
| Size | 112046 bytes |
| Content description | Plain-text form of FHS 3.0; defines the standard Linux filesystem hierarchy including /run (§3.15), /opt (§3.13), /etc/opt (§3.8), /var/opt (§5.12), and /usr/include (§4.5) |

### fhs-3.0.pdf

| Field | Value |
|---|---|
| Source URL | https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.pdf |
| SHA-256 | 53d239e569a2d7b31a74fa09d585368c0f5a164e4624723fa2894660dd10fd23 |
| Size | 248943 bytes |
| Content description | PDF form of FHS 3.0; identical content to the plain-text version, suitable for section-numbered citation |

## Integrity Notes

- Plain text: confirmed non-empty; opens with "Filesystem Hierarchy Standard" and "Version 3.0"
- PDF: `head -c 4` returns `%PDF` — valid PDF magic bytes

## Relevance

Sections cited by the `fhs-lsb-uid-gid` skill:
- §3.8 `/home` (maps to `/etc/opt` path convention)
- §3.13 `/opt` — UMRS installs to `/opt/umrs/`
- §3.15 `/run` — run-time variable data; formalized here, not present in FHS 2.3
- §4.5 `/usr/include` — governs why `/etc/keys/umrs/` is placed outside `/opt`
- §5.12 `/var/opt` — UMRS variable data at `/var/opt/umrs/`

## Note on FHS 2.3

The prior version (FHS 2.3) is retained at `.claude/references/linux-fhs-2.3/fhs-2.3.txt`
for historical reference. Do not delete it. FHS 3.0 is the authoritative version for
RHEL 10 deployment targets.
