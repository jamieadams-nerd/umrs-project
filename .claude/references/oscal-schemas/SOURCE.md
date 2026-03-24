# OSCAL Schemas RAG Collection

## Purpose

This collection provides machine-readable OSCAL schema and FedRAMP baseline profile files
for RAG augmentation. It supports the assessment-engine work and any agent reasoning about
OSCAL document structure, FedRAMP control selections, and RMF artifact formats.

## Contents

### OSCAL v1.1.2 JSON Schemas (8 files)

**Source:** https://github.com/usnistgov/OSCAL/releases/tag/v1.1.2
**Authority:** NIST (usnistgov)
**Retrieved:** 2026-03-23

| File | Description |
|---|---|
| `oscal_assessment-plan_schema.json` | Assessment Plan (AP) schema |
| `oscal_assessment-results_schema.json` | Assessment Results (AR) schema |
| `oscal_catalog_schema.json` | Catalog schema (control definitions) |
| `oscal_complete_schema.json` | Combined/complete schema (all models) |
| `oscal_component_schema.json` | Component Definition schema |
| `oscal_poam_schema.json` | Plan of Action and Milestones (POA&M) schema |
| `oscal_profile_schema.json` | Profile schema (control selections) |
| `oscal_ssp_schema.json` | System Security Plan (SSP) schema |

### FedRAMP Rev5 OSCAL Baseline Profiles (6 files)

**Source:** https://github.com/GSA/fedramp-automation/tree/master/dist/content/rev5/baselines/json
**Authority:** GSA (fedramp-automation) — official FedRAMP managing agency
**Retrieved:** 2026-03-23

| File | Description |
|---|---|
| `fedramp_rev5_LOW-baseline_profile.json` | FedRAMP LOW impact profile (control selections + params) |
| `fedramp_rev5_LOW-baseline-resolved-profile_catalog.json` | LOW resolved catalog (full control text, 3.3 MB) |
| `fedramp_rev5_MODERATE-baseline_profile.json` | FedRAMP MODERATE impact profile |
| `fedramp_rev5_MODERATE-baseline-resolved-profile_catalog.json` | MODERATE resolved catalog (5.0 MB) |
| `fedramp_rev5_HIGH-baseline_profile.json` | FedRAMP HIGH impact profile |
| `fedramp_rev5_HIGH-baseline-resolved-profile_catalog.json` | HIGH resolved catalog (5.9 MB) |

## Update Procedure

To check for newer OSCAL versions:
1. Query: https://api.github.com/repos/usnistgov/OSCAL/releases/latest
2. Compare tag_name against v1.1.2
3. If newer, download all 8 *_schema.json assets from the new release

To check for FedRAMP profile updates:
1. Check commit history: https://api.github.com/repos/GSA/fedramp-automation/commits?path=dist/content/rev5/baselines/json
2. If newer commits exist, re-download the 6 baseline files

## Relevance

- UMRS assessment-engine (planned) — OSCAL output format
- CMMC CA-7, RA-3, CA-5 (POA&M)
- FedRAMP Rev5 authorization artifacts
- RMF documentation artifacts (SSP, SAP, SAR)
- Control family coverage analysis (via resolved-profile catalogs)
