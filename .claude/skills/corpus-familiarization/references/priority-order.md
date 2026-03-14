# Processing Priority Order — UMRS Collections

When performing a familiarization pass, process documents in the order below
within each collection. Higher priority documents establish the foundational
vocabulary and regulatory frame that lower-priority documents are read through.

---

## Collection: `style-guides`

| Priority | Document | Rationale |
|----------|----------|-----------|
| 1 | MIL-STD-38784A | Governs warning/caution/note hierarchy — foundational for all procedures |
| 2 | Plain Language Guidelines (Federal) | Regulatory baseline; other guides are read as refinements of this |
| 3 | NIST Author Instructions | Establishes register for NIST-adjacent output |
| 4 | Google Developer Style Guide | Primary style authority for technical/API documentation |
| 5 | Microsoft Writing Style Guide | Secondary style authority; strong on examples |
| 6 | Apple Style Guide | Internationalization and translation guidance |
| 7 | DigitalOcean Guidelines | Tutorial and procedure structure patterns |
| 8 | Mailchimp Style Guide | Voice and tone; accessibility |

---

## Collection: `domain-references`

| Priority | Document | Rationale |
|----------|----------|-----------|
| 1 | NIST SP 800-53 Rev 5 | Primary control vocabulary; all annotations derive from this |
| 2 | NIST SP 800-171 Rev 3 | CUI requirement identifiers; close relationship to 800-53 |
| 3 | CMMC Assessment Guide Level 2 | Maps onto 800-171; defines practice vocabulary for Five Eyes procurement |
| 4 | Common Criteria Parts 1 & 2 | SFR structured English register; applies when writing CC artifacts |
| 5 | RHEL 10 Security Guide | Product-specific terminology for SELinux, IMA, dm-crypt, audit |
| 6 | SELinux Project Notebook | Conceptual and policy language for MLS/MCS |

---

## Collection: `supplemental`

| Priority | Document | Rationale |
|----------|----------|-----------|
| 1 | NASA Technical Writing Guidance | Safety-critical procedure writing patterns |

---

## Cross-collection processing order

When multiple collections are present and processed together:

1. `domain-references` — establishes what terms mean
2. `style-guides` — establishes how to express those terms
3. `supplemental` — adds patterns and edge case handling

Never process style guides before domain references. An agent that learns
style conventions before learning domain vocabulary will apply generic
language patterns to technical terms that have precise normative meanings.

