---
name: UMRS Case Study Corpus Status
description: Status and location of the UMRS failure case study corpus — used for outreach, architecture justification, and CUI index group coverage verification
type: project
---

The UMRS case study corpus consists of three documents:

1. `docs/modules/ROOT/pages/foundations/history/case-studies-consolidated.adoc` — ~20 US cases across CUI categories (PRVCY, HLTH, WATER, ENV, TRAN, CRIT, AGR). Includes "Trust But Didn't Verify" section and Quiet Killer productivity section.

2. `docs/modules/ROOT/pages/foundations/history/canadian-case-studies.adoc` — 18 Canadian cases mapped to Protected A/B/C tiers (CRA GCKey, BGRS/SIRVA, IRCC, LifeLabs, Ontario Health atHome, Phoenix, ArriveCAN, eHealth, Walkerton, NEB/Enbridge, Lac-Megantic, CN Rail, Nortel, VW Canada, SNC-Lavalin, muzzled scientists, SSC, Statistics Canada).

3. `.claude/references/reports/five-eyes-case-study-research.md` — 50 NEW cases researched 2026-04-03, awaiting tech-writer integration. Coverage: US (13 additional), Canada (6 additional), UK (14), Australia (10), New Zealand (7).

**Why:** Jamie asked for a massive expansion of the case study corpus across all Five Eyes nations to support UMRS outreach, compliance justification, and to ensure all 18 CUI index groups have at least one documented case.

**How to apply:** When tech-writer needs case study content for outreach, architecture documentation, or CUI category justification — point to these three files. The Five Eyes report is the research input; it needs to be integrated into publishable adoc format. Suggest creating `five-eyes-case-studies.adoc` as a new Antora page.

**Key patterns documented across all 50 new cases:**
- Trusted-without-verify (Post Office Horizon, SolarWinds, Novopay, Robodebt, Phoenix)
- Third-party custody without attestation (SSCL MOD, Mercury IT NZ, BGRS, Medibank, Latitude)
- Audit gap — detection vs attribution vs enforcement (IRS, HMRC, Te Whatu Ora, Ontario snooping)
- Population-scale single-repository amplification (Optus, Medibank, Latitude, HMRC 25M)
