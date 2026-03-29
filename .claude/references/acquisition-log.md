# RAG Collection Acquisition Log

**Session:** 2026-03-22
**Agent:** The Librarian (researcher)
**Authorization:** Jamie Adams — green light given for Tier 1 downloads
**Plan source:** `.claude/references/reports/agent-knowledge-acquisition-plan.md`

---

## Results Summary

| # | Resource | Status | Files | Notes |
|---|---|---|---|---|
| 1 | ISKO IEKO encyclopedia | DOWNLOADED | 119 HTML articles | All priority articles; full catalog coverage |
| 2 | Stanford CS147 AU2023 lectures | REQUIRES MANUAL DOWNLOAD | 0 | RHEL10 FIPS TLS incompatibility with hci.stanford.edu |
| 3 | MIT OCW 6.831 lectures | DOWNLOADED | 25 PDFs | All 25 lectures; CC BY-NC-SA 4.0 |
| 4 | Miller "Genre as Social Action" | DOWNLOADED | 1 PDF | Valid; author-distributed |
| 5 | Bazerman genre chapter | DOWNLOADED | 1 PDF | Valid; author-distributed from UCSB |
| 6 | Pirolli information foraging ch. 1 | DOWNLOADED | 1 PDF | Valid; author-distributed uncorrected proof |
| 7 | Svenonius Ch. 5 | DOWNLOADED | 1 PDF | Valid; hosted by Evergreen State College |
| 8 | Hjørland KO theories paper | DOWNLOADED | 1 PDF | Valid; DOI 10.5771/0943-7444-2013-3-169 |
| 9 | Blackwell/Green Cognitive Dimensions | DOWNLOADED | 1 PDF | Valid; from Cambridge author page |
| 10 | NISO Z39.19 standard | DOWNLOADED | 1 PDF | Valid; official NISO free download |
| 11 | Precision Content IA white paper | DOWNLOADED | 1 PDF | Valid; free white paper |
| 12 | Vannevar Bush "As We May Think" | DOWNLOADED | 1 PDF | Valid; Life Magazine version from worrydream.com |

**Total downloaded:** 152 files (119 HTML + 33 PDF)
**Requires manual action:** 1 item (Stanford CS147)

---

## Checksums

### Technical Communication / Theory

| File | SHA-256 |
|---|---|
| miller-genre-as-social-action.pdf | `592d0bf19c914c6bf2fdcee81ac6832555c3f80fbe19fde454e535a87fb902dc` |
| bazerman-genre-as-social-action.pdf | `517c0ba46ce1eeb48784b204b17c7843205c072e162bb119f3630d199022891a` |

### Information Architecture / Theory

| File | SHA-256 |
|---|---|
| pirolli-information-foraging-ch1.pdf | `ef281f9d724d001e749a76ab72317f36ac97d55808e3041d46d3f18c2272515a` |
| precision-content-understanding-ia.pdf | `5a5df77a9cc074da320cc2566571275649daff856affb31f0a942cfc677ae5e6` |

### Information Architecture / Standards

| File | SHA-256 |
|---|---|
| niso-z39-19-2005r2010.pdf | `703f1df79771720b95069f2afd49a0e80d6cc950195050175d4cbf2edb2bd323` |

### Knowledge Organization / Texts

| File | SHA-256 |
|---|---|
| svenonius-intellectual-foundation-ch5.pdf | `533a8a19ab20653c799e0b8634855623cbd3c995a7c49a7fedf67839f8f203ca` |
| hjorland-theories-of-ko.pdf | `193b67867d672a8d00e88c1778ae54efc22d7bd6aceb0a484644571a7ec5ed01` |

### HCI Courses / Theory Papers

| File | SHA-256 |
|---|---|
| blackwell-green-cognitive-dimensions.pdf | `b69a251d1aae306aa9305be4dfbf534107444ec3dabbec009fa3d8d9ae9b9d29` |
| vannevar-bush-as-we-may-think-1945.pdf | `a766eb1d16b0d19990a54495a56d5a69db72af83cd1036f7cb1d1029fff6fff3` |

### HCI Courses / MIT 6.831 (spot-check)

| File | SHA-256 |
|---|---|
| MIT6_831S11_lec01.pdf | `e528d4d3ca78663f3787a0852bf03d04cd66d4fe45345e0ec58bc29003c3ebdf` |
| MIT6_831S11_lec13.pdf | `fcad37c7571316eed45baa4f9838cc2dc9e81d69c7c4c0228b4467a0bfd5985c` |
| MIT6_831S11_lec25.pdf | `9b1023c7564bca70a7226c11a02580e2b9ec2ac67e849bdf76d1fbe1e5fd2fd6` |

IEKO HTML articles: checksums not computed individually (119 files); verify by re-fetching from source if needed.

---

## Manual Download Required: Stanford CS147

**Reason:** hci.stanford.edu uses a TLS configuration incompatible with RHEL10 FIPS crypto policy.
Error: `TLS connect error: error:1C8000E9:Provider routines::ems not enabled`

This blocks both `curl` and `wget`. Python urllib also fails with `[SSL] unsupported`.

**Action needed:** Manual browser download of 20 PDFs.
**Destination:** `.claude/references/hci-courses/stanford-cs147/`
**Instructions:** See `.claude/references/hci-courses/stanford-cs147/SOURCE.md`
**URL:** https://hci.stanford.edu/courses/cs147/2023/au/lectures/

---

## Provenance Notes

All downloaded materials are:
- CC-licensed (MIT OCW: BY-NC-SA 4.0)
- Author-distributed open access (Miller, Bazerman, Pirolli, Blackwell/Green)
- Open-access by publishing organization (ISKO IEKO, NISO Z39.19)
- Public domain (Vannevar Bush 1945)
- Free white paper (Precision Content)

No paywalled content was bypassed. No unauthorized copies were downloaded.

---

## Next Steps

1. **Stanford CS147:** Manual download needed (20 PDFs)
2. **Ingest all collections** via `ingest.py` — run after Stanford download or in parallel
3. **Post-ingestion familiarization** — agent must run corpus-familiarization skill per `agent_behavior_rules.md`
4. **Tier 2 purchase decisions** — Jamie to decide on O'Reilly subscription (unlocks Rosenfeld/Morville IA book)
5. **Tier 3 YouTube pipeline** — MIT OCW 9.00SC cognitive psychology transcripts (establish yt-dlp pipeline first)
