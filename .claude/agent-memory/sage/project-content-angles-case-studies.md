---
name: Content angles unlocked by case studies and impact profiles
description: Concrete blog, YouTube, and conference angles that the case study corpus and SP 800-30 impact profile work make possible. Read before planning any new content.
type: project
---

## Why these angles are different from what exists

Most CUI content is procedural: mark the file, follow the regulation. UMRS now has:
- Real breach cases mapped to specific CUI categories
- SP 800-30 adverse impact types as an authoritative consequence taxonomy
- A design claim that the filesystem itself can communicate these consequences

That combination shifts tone from compliance manual to consequence-grounded engineering story.

## Blog Series: "The Failures That Built UMRS"

One post per case study, or grouped by failure class. Each post:
- Opens with the human cost (dollar amount, conviction, death toll where applicable)
- Names the CUI category
- Explains the specific failure in plain engineering terms
- Shows the UMRS control that would have changed the outcome
- Ends with the current UMRS implementation status (honest about Phase 1 vs Phase 2)

**High-priority candidates (narrative strength + audience relevance):**

1. **Flint water crisis → CUI//WATER** — integrity failure, kernel measurement, public health consequence. Accessible to general audience. Strong IMA/EVM story (NOTE: IMA/EVM is Phase 2; frame as "this is what we're building toward").

2. **OPM breach → CUI//PRIVACY** — canonical over-access story. MLS category compartments are the technical answer. Directly relevant to federal audience.

3. **Deepwater Horizon alarm bypass → CUI//CRIT** — safety config modification story. IMA/EVM would have detected or blocked the bypass. Visceral stakes (11 deaths). NOTE: Phase 2 control.

4. **Equifax → CUI//PRIVACY** — most recognizable breach. Clear data segmentation story. $700M settlement is a program manager's number.

5. **Boeing 737 MAX → CUI//CRIT** — regulatory disclosure provenance story. Append-only audit log + separation of authorities. Resonates with engineering and safety audiences.

6. **Lorraine v. Markel + Vayner → digital evidence** — pure provenance story. No secrecy required. Shows that integrity and custody matter in court, not just in ops.

**Pitch line for the series:**
"We mapped twelve federal cases and regulatory failures to CUI categories and built a
consequence taxonomy from SP 800-30. Here's what the engineering would have looked like
if the systems were designed for assurance instead of convenience."

## The Signature Post: Impact Profile as UMRS Innovation

Standalone post (or conference abstract core):
- Opens: no framework has a construct that links CUI category + adverse impact type + real case law + machine-verifiable handling
- Explains SP 800-30 Appendix H Table H-2 in plain English (five impact types)
- Shows the JSON structure: impact_profile vs. handling_requirements split
- Explains why that split matters (compliance defect trap)
- Ends: this is in development; here's where to follow along

**Audience:** security engineers, government program managers, compliance architects.
This is the content that gets UMRS cited in academic and government papers.

## YouTube: "Failures That Built UMRS" Companion Series

Each blog post above has a natural video counterpart:
- Visual: timeline of the failure, then "what the filesystem would have looked like"
- Jamie intro + brief narrative, then live vault demo showing the relevant label
- End card: "What would umrs-stat have flagged here?" (even if hypothetical for Phase 2 features — label it clearly as future state)

**First video candidate:** Flint water crisis. Accessible, emotional, clear technical answer.
The "Water Quality Memo becomes a signed, measured artifact" framing is immediately visual.

## Conference Abstract Core

**Title candidate:** "Real Cases, Real Categories: Mapping Twelve Federal Failures to CUI
and Building Impact Profiles from SP 800-30"

**Problem statement:** CUI guidance tells you how to mark and handle information. It does
not tell you what happens when you get it wrong. We mapped twelve high-profile federal
cases — convictions, regulatory findings, and e-discovery rulings — to CUI categories
and built an impact profile taxonomy anchored to NIST SP 800-30 Rev 1 Appendix H.

**Contribution:** An open-source catalog that links every CUI category to an authoritative
adverse impact type, case-grounded rationale, and machine-verifiable handling requirements.
The catalog ships with UMRS as an advisory system in the filesystem itself.

**Venue fit:**
- RSA: business risk angle, $700M Equifax settlement opens the abstract
- NSA IA Symposium: SP 800-30 anchoring + formal CUI registry connection, full sage mode
- USENIX Security: novel construct, peer-reviewable taxonomy, honest about what's formal vs. designed

## The Pitch Line (Canonical)

"Your filesystem now has a security officer who tells you what happens if you screw up."

Use this in: conference abstracts, YouTube thumbnails, LinkedIn posts, README headline.
Never use it to imply the feature is complete. Always follow with current implementation status.

## Phase Guardrails for All Content in This Series

Before publishing any case study content:
- Check phase1-phase2-positioning.md
- IMA/EVM is Phase 2 — never describe it as currently available
- Impact profiles are in design — describe the concept, note the status
- Current Phase 1 capability: CUI labeling, MCS categories, umrs-ls/stat/uname, audit trails
- The case studies show WHERE the design is going, not WHERE IT IS NOW

The failure to be honest about implementation status is the one thing that could
damage UMRS credibility with the audiences that matter most.
