# French-CA Terminology Corpus — Source Provenance

This directory contains authoritative Canadian French terminology databases
for Simone's `french-lookup` skill. All sources are official Government of Canada
or Quebec provincial government data.

## Files

### termium-plus-fr_CA.tsv

| Field | Value |
|---|---|
| Source | TERMIUM Plus® — Government of Canada Translation Bureau |
| Publisher | Public Services and Procurement Canada (TPSGC/PSPC) |
| License | Open Government Licence — Canada |
| Date retrieved | 2026-03-23 |
| SHA-256 | `5e23382e3948cda4d58a5ead518f8380ae580afea595bc5be06fcd719f46c074` |
| Total entries | 32,210 |

**Content sources (merged):**

| Subject | Source URL | Entries |
|---|---|---|
| Electronics & Informatics | https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-ElectroniqueInformatique-subject-ElectronicsInformatics.zip | ~12,911 raw |
| Administration | https://donnees-data.tpsgc-pwgsc.gc.ca/bt1/tp-tp/domaine-subject-administration.zip | ~18,472 raw |
| Information Security Glossary | https://www.btb.termiumplus.gc.ca/publications/securite-information-security-eng.html | 287 |
| CCCS Glossary | https://www.cyber.gc.ca/en/glossary + https://www.cyber.gc.ca/fr/glossaire | 76 |

**Note on Military/Security subject:** The Open Government Portal lists a
"Military and Security Subject" resource (resource ID `99a220a8-fa42-4231-9aa9-c626135e0912`)
at https://open.canada.ca/data/en/dataset/94fc74d6-9b9a-4c2e-9c6c-45a5092453aa/resource/99a220a8-fa42-4231-9aa9-c626135e0912
but the direct download URL requires JavaScript rendering to discover.
The CKAN API is blocked. This subject remains **pending manual download**.
Manual steps: visit the above URL in a browser, click Download, save as
`/media/psf/repos/umrs-project/.claude/references/corpus/termium-military-security-raw.zip`
then run `python3 /media/psf/repos/umrs-project/.claude/references/scripts/extract_termium.py` to process it.

### oqlf-gdt-fr_CA.tsv

| Field | Value |
|---|---|
| Source | Office québécois de la langue française (OQLF) — Grand dictionnaire terminologique |
| Publisher | Government of Quebec |
| License | Creative Commons Attribution 4.0 (CC BY 4.0) |
| Date retrieved | 2026-03-23 |
| SHA-256 | `624895ae2a5d4e3aa271b546a40cd1392a6f45cf8a819180b0b3273cb74ddf35` |
| Total entries | 25,881 |

**Content sources (merged):**

| Resource | Source URL | Entries |
|---|---|---|
| Fiches terminologiques (GDT) | https://www.donneesquebec.ca/recherche/dataset/1c6567bf-8995-40b9-84a4-50faabae12f4/resource/c3ce0af4-7c0f-4dd2-b53a-6dc7fb3ea5ef/download/fiches_recentes_signees_oqlf_2026-01-19.csv | 53,899 raw → 25,750 relevant |
| Termes officialisés | https://www.donneesquebec.ca/recherche/dataset/1c6567bf-8995-40b9-84a4-50faabae12f4/resource/882453c2-93c3-4204-b5ff-6d6297082ad9/download/termes_officialises_2026-01-14.csv | 1,471 raw → 131 relevant |

## TSV Format

```
english_term\tfr_ca_term\tdomain\tsource\tnotes
```

- **english_term**: Primary English term (may be empty for FR-primary entries)
- **fr_ca_term**: Canadian French term (may include semicolon-separated synonyms for GDT)
- **domain**: Subject domain tag (`information technology`, `information security`,
  `cybersecurity`, `public administration`, `national defence`)
- **source**: Database and sub-source name
- **notes**: Grammatical gender (masculine/feminine noun), abbreviations,
  synonym relationships, domain sub-path, GDT officialisation type

## Coverage Notes

TERMIUM Plus covers 100% of 41 sampled UMRS key terms.
OQLF GDT covers 82% (34/41) — missing: configuration management, mandatory access control,
patch management, penetration testing, security label, separation of duties,
vulnerability assessment. These are better covered in TERMIUM Plus.

## Filtering Applied

Both databases contain millions of terms across all subject areas. Only terms
from IT, security, cybersecurity, public administration, and national defence
domains were extracted, filtered by keyword matching against a UMRS-specific
term list. Road safety, nuclear safety, occupational safety, and other
non-information-security uses of "sécurité" were excluded.

## Update Procedure

To refresh from source:
1. Download fresh ZIPs from donnees-data.tpsgc-pwgsc.gc.ca (GoC open data)
2. Re-run `python3 /media/psf/repos/umrs-project/.claude/references/scripts/extract_termium.py`
3. Re-run `python3 /media/psf/repos/umrs-project/.claude/references/scripts/extract_gdt.py`
4. Re-run `python3 /media/psf/repos/umrs-project/.claude/references/scripts/parse_termium_glossary3.py`
5. Re-run `python3 /media/psf/repos/umrs-project/.claude/references/scripts/parse_cccs_bilingual.py`
6. Update checksums in this file

GDT source CSVs are dated in their filename (e.g., `fiches_recentes_signees_oqlf_2026-01-19.csv`).
TERMIUM Plus ZIPs do not include dates — check for updates quarterly via the Open Government Portal.
