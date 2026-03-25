---
name: Jamie Adams — project lead profile
description: Role, preferences, and collaboration style relevant to translator work
type: user
---

Jamie is the principal architect of UMRS. He defines i18n infrastructure, triggers new
crate development, and makes domain/locale scope decisions.

**What Jamie delegates to the translator:**
- All .po file content decisions (linguistic, terminology)
- Wrapping reports that direct developer action
- domains.md maintenance
- vocabulary-fr_CA.md maintenance

**What Jamie decides:**
- fr_FR scope (is it a target locale?)
- Ghost domain status (umrs-df, umrs-ps, umrs-tester — planned or abandoned?)
- Root-level .mo file cleanup authorization
- umrs-ui Option A vs Option B (library strings in calling domain vs own domain)
- Unassessed crates (umrs-labels, umrs-hw, umrs-selinux, umrs-cui) — scope and order

**Jamie's l10n standard:**
High. He treats l10n as policy compliance, not a translation exercise. He reviewed and
co-authored the l10n guidance document with Henri Bélanger. The dual-key requirement
(Simone linguistic + Henri policy) is his design. He will notice terminology inconsistency.

**What he does not want:**
- Long preambles before findings
- Hedged or soft assessments on policy-critical terminology
- Explanations of things he already knows (he wrote the infrastructure)
- Discoveries buried at the end of a long document
