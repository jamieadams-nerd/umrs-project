# Research Plan: Access Control Corpus

## Objective

Collect foundational access control papers, government standards, and
OS security documentation into `.claude/references/access-control/`
for RAG ingestion. Do not ingest automatically — place files and await review.

---

## Directory Structure to Create

```
.claude/references/access-control/
├── foundational-papers/
├── standards/
├── os-security/
└── rule-catalog/
```

---

## Documents to Retrieve

### foundational-papers/

**Bell-LaPadula Model**
- URL: https://apps.dtic.mil/sti/pdfs/ADA023588.pdf
- Save as: bell-lapadula-1973.pdf
- Title: Secure Computer Systems: Mathematical Foundations
- Authors: Bell and LaPadula, 1973
- Topics: Simple Security Property (no read up), *-Property (no write down)

**Biba Integrity Model**
- URL: https://apps.dtic.mil/sti/pdfs/ADA039324.pdf
- Save as: biba-1977.pdf
- Title: Integrity Considerations for Secure Computer Systems
- Author: Kenneth J. Biba, 1977
- Topics: Simple Integrity Property, No Write Up rule

**Clark-Wilson Model**
- URL: https://www.cs.cornell.edu/courses/cs5430/2012sp/MCinfo/ClarkWilson.pdf
- Save as: clark-wilson-1987.pdf
- Title: A Comparison of Commercial and Military Computer Security Policies
- Authors: Clark and Wilson, 1987

**Brewer-Nash (Chinese Wall)**
- URL: https://www.cs.cornell.edu/courses/cs513/2005fa/paper.brewer-nash.pdf
- Save as: brewer-nash-1989.pdf
- Title: The Chinese Wall Security Policy
- Authors: Brewer and Nash, 1989

**Graham-Denning Model**
- URL: https://dl.acm.org/doi/pdf/10.1145/361011.361067
- Save as: graham-denning.pdf

**HRU Model**
- URL: https://dl.acm.org/doi/pdf/10.1145/360051.360056
- Save as: hru-model.pdf

**Saltzer and Schroeder**
- URL: https://www.cs.virginia.edu/~evans/cs551/saltzer/
- Save as: saltzer-schroeder.html or .pdf if available
- Title: The Protection of Information in Computer Systems
- Topics: Least privilege, complete mediation, fail-safe defaults,
  economy of mechanism, open design, separation of privilege,
  least common mechanism, psychological acceptability

---

### standards/

**NIST SP 800-53 Rev 5**
- URL: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf
- Save as: nist-sp-800-53r5.pdf
- Note: Large document (~500 pages) — confirm download completes fully

**NIST SP 800-162 (ABAC)**
- URL: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-162.pdf
- Save as: nist-sp-800-162.pdf
- Title: Guide to Attribute Based Access Control

**NIST SP 800-207 (Zero Trust)**
- URL: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-207.pdf
- Save as: nist-sp-800-207.pdf
- Title: Zero Trust Architecture

---

### os-security/

**SELinux Notebook**
- URL: https://github.com/SELinuxProject/selinux-notebook/raw/main/selinux-notebook.pdf
- Save as: selinux-notebook.pdf
- Note: Check if already present in .claude/references/selinux-notebook/
  before downloading — may be a duplicate

**Linux Capabilities Manual Page**
- URL: https://man7.org/linux/man-pages/man7/capabilities.7.html
- Save as: linux-capabilities.html

**POSIX ACL Manual Page**
- URL: https://man7.org/linux/man-pages/man5/acl.5.html
- Save as: posix-acl.html

---

### rule-catalog/

Create this directory and populate it with the following YAML rule files.
These do not require web retrieval — generate them from the content below.

**AC-001.yaml**
```yaml
id: AC-001
title: Principle of Least Privilege
rule: Subjects should only have the permissions necessary to perform their function.
category: principle
references:
  - https://csrc.nist.gov/glossary/term/least_privilege
  - Saltzer & Schroeder 1975
```

**AC-002.yaml**
```yaml
id: AC-002
title: Complete Mediation
rule: Every access request must be checked for authorization without exception.
category: principle
references:
  - Saltzer & Schroeder 1975
```

**AC-003.yaml**
```yaml
id: AC-003
title: Separation of Duties
rule: Critical operations should require multiple independent parties to complete.
category: principle
references:
  - https://csrc.nist.gov/glossary/term/separation_of_duty
```

**MLS-001.yaml**
```yaml
id: MLS-001
title: Bell-LaPadula Simple Security Property
rule: A subject may not read data at a higher classification level (no read up).
model: Bell-LaPadula
category: confidentiality
references:
  - https://csrc.nist.gov/glossary/term/simple_security_property
  - bell-lapadula-1973.pdf
```

**MLS-002.yaml**
```yaml
id: MLS-002
title: Bell-LaPadula Star Property
rule: A subject may not write information to a lower classification level (no write down).
model: Bell-LaPadula
category: confidentiality
references:
  - https://csrc.nist.gov/glossary/term/star_property
  - bell-lapadula-1973.pdf
```

**INTEGRITY-001.yaml**
```yaml
id: INTEGRITY-001
title: Biba Simple Integrity Property
rule: A subject may not read data at a lower integrity level (no read down).
model: Biba
category: integrity
references:
  - biba-1977.pdf
```

**INTEGRITY-002.yaml**
```yaml
id: INTEGRITY-002
title: Biba Star Integrity Property
rule: A subject may not write data to a higher integrity level (no write up).
model: Biba
category: integrity
references:
  - biba-1977.pdf
```

---

## After Retrieval

1. Report a manifest of all files downloaded with sizes and locations
2. Ensure @security-engineer and @rust-developer can use rag-query and new rule-catalog feature when working on rust code (@components/rusty-gadgets)
2. Report which rule-catalog YAML files were created
3. Flag any URLs that failed, redirected, or returned unexpected content
4. Note if selinux-notebook.pdf was skipped as duplicate
5. Do NOT run ingest.py — await review before ingestion
6. Notify when ready for review

---

## Notes

- PDFs from dtic.mil and nvlpubs.nist.gov are large — verify each
  download completes before moving to the next
- ACM URLs (Graham-Denning, HRU) may require institutional access —
  report if blocked rather than attempting workarounds
- The rule-catalog YAML files are generated locally, no web access needed
- Saltzer & Schroeder may only be available as HTML — that is acceptable
