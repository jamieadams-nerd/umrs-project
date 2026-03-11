# UMRS British English (en_GB) Vocabulary Reference

Canonical spelling and vocabulary decisions for UMRS British English localization.
Maintained by the `umrs-translator` agent.

**Scope**: Formal technical and government/security documentation register.
Follows UK Government Digital Service (GDS) style and NCSC published guidance.

**Update policy**: Add or revise an entry whenever a new term decision is made.
When a term has an established British equivalent, use it. Retain technical
acronyms and proper nouns unchanged (SELinux, MLS, CUI, NIST, etc.).

---

## Spelling Conventions

### -ize → -ise (consistently in en_GB formal register)

| en_US          | en_GB          | Notes |
|----------------|----------------|-------|
| authorize      | authorise      | UK Government mandate |
| initialize     | initialise     | |
| recognize      | recognise      | |
| specialize     | specialise     | |
| minimize       | minimise       | |
| maximize       | maximise       | |
| prioritize     | prioritise     | |
| organize       | organise       | |
| analyze        | analyse        | |
| utilize        | utilise        | prefer "use" in either variant |
| categorize     | categorise     | |
| synchronize    | synchronise    | |
| customize      | customise      | |

### -or → -our

| en_US     | en_GB     | Notes |
|-----------|-----------|-------|
| color     | colour    | e.g., "colour-coded" |
| honor     | honour    | |
| behavior  | behaviour | |
| neighbor  | neighbour | |
| labor     | labour    | |
| favor     | favour    | |

### -er → -re

| en_US   | en_GB   | Notes |
|---------|---------|-------|
| center  | centre  | |
| fiber   | fibre   | e.g., "optical fibre" |
| theater | theatre | rare in security contexts |
| meter   | metre   | unit of measurement only; "meter" for measuring device |

### Double l before vowel suffix

| en_US       | en_GB       | Notes |
|-------------|-------------|-------|
| labeled     | labelled    | very relevant — security labels |
| traveled    | travelled   | |
| canceled    | cancelled   | |
| signaled    | signalled   | |
| modeled     | modelled    | |
| enrolled    | enrolled    | (same in en_GB; already doubled) |

### -ense → -ence (noun forms)

| en_US   | en_GB    | Notes |
|---------|----------|-------|
| license | licence  | noun: "software licence"; verb "to license" unchanged |
| defense | defence  | |
| offense | offence  | |

### Other common differences

| en_US     | en_GB      | Notes |
|-----------|------------|-------|
| program   | program    | **program** is correct in tech contexts; "programme" is for TV/events |
| judgment  | judgement  | both acceptable; GDS uses "judgement" |
| gray      | grey       | |
| check     | check      | same (not "cheque" except for banking) |

---

## Security and Access Control

| English (en_US)       | en_GB equivalent      | Notes |
|-----------------------|-----------------------|-------|
| access denied         | access denied         | Identical |
| access control        | access control        | Identical |
| mandatory access control | mandatory access control | Identical |
| security label        | security label        | Identical |
| security context      | security context      | Identical |
| audit event           | audit event           | Identical |
| policy enforcement    | policy enforcement    | Identical |
| color-coded           | colour-coded          | Spelling change |
| labeled               | labelled              | Spelling change — critical for "security labelled data" |
| authorized            | authorised            | Spelling change |
| recognized            | recognised            | Spelling change |
| behavior analysis     | behaviour analysis    | Spelling change |

---

## Locale Settings

| Field          | Value        |
|----------------|--------------|
| Locale code    | en_GB        |
| Date format    | DD/MM/YYYY   |
| Decimal sep.   | . (period)   |
| Thousands sep. | , (comma)    |
| Currency       | £ (GBP) — not relevant for UMRS tools |
| Quotation      | 'single' or "double" (GDS prefers single for technical writing) |

---

## Sources

- UK Government Digital Service (GDS) Content Design Guide
- NCSC (National Cyber Security Centre) published advisories — use -ise consistently
- Oxford English Dictionary British English conventions
- ISO/IEC standard terminology (unchanged across variants)
