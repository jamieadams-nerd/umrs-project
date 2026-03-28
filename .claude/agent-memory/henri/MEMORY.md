# Henri -- Agent Memory Index

## Operating Posture (established 2026-03-23)
- UMRS is a **reference system**, not production. All Canadian findings are **advisory/informational**.
- TBS is a reference framework, not a compliance gate. Divergences are informational, not blockers.
- No Canadian government contacts or deployment pipeline. Forward-looking positioning only.
- Henri owns Canadian category design. Jamie reviews.
- [Jamie's directive answers](jamies-directive-answers.md) -- full text of all 6 answers + recalibrated posture

## Onboarding
- [First day orientation](orientation-notes.md) -- project overview, architecture, team, role boundaries
- [First task assignments](first-tasks.md) -- French QA on umrs-uname + Canadian Protected A/B/C catalog

## Corpus
- Corpus files stored in `corpus/`
- Corpus index: `corpus-index.md` (pending Librarian acquisition)
- TBS Standard on Security Categorization: acquired (EN + FR) -- key reference for Protected A/B/C
- TBS Directive on Security Management: acquired (EN + FR)
- TBS Policy on Government Security: acquired (EN + FR)
- Access to Information Act, Privacy Act, Official Languages Act: acquired (EN + FR)
- TBS Directive on Privacy: acquired (EN + FR)
- CCCS ITSG-33 PBMM profile: acquired (PDF)
- RAG ingestion: NOT recommended at this time (corpus files suffice for current scope)
- Termium Plus Military/Security subject: PENDING -- routed to Librarian (2026-03-23)

## Findings
- [Canadian program naming analysis](finding-canadian-program-name.md) -- PROTECTED/PROTEGE canonical term; PA/PB/PC for compact; 3 formal findings filed (2026-03-23)

## Resolved Decisions (2026-03-23, second round)
- Category range: c200-c299 per tier (Jamie wants room to breathe)
- Protected C: include in Phase 1 with caveat about Phase 2 enforcement
- UTF-8 encoding: confirmed for all JSON files

## Deliverables Completed
- Canadian category requirements report: `.claude/reports/2026-03-23-canadian-protected-category-requirements.md` (updated with Jamie's decisions)
- CANADIAN-PROTECTED.json catalog: `components/rusty-gadgets/umrs-cui/data/ca/CANADIAN-PROTECTED.json`

## TBS Style Guide Analysis (2026-03-28)
- [Full analysis](tbs-style-guide-analysis.md) -- Canada.ca Content Style Guide EN + FR
- Key finding: French headings must use infinitive mood per TBS s5.1.1 (not imperative)
- Key finding: Federal French uses "redaction epicene" (Termium/BdT), not OQLF "ecriture inclusive"
- EN must/need-to distinction collapses to "devez" in FR -- flag if distinction needed
- Plain language rules compatible with STE mode (20 words max, active voice, one idea per sentence)
- Number formatting differences documented (100 $ vs $100, 20 % vs 20%, 16 h 30 vs 4:30 pm)
- Guide is advisory for UMRS (reference system), but positions FR content for federal deployment

## Key Policy References
- TBS Standard on Security Categorization, Appendix J, section J.2.4.2 -- defines Protected A/B/C
- EN: "Protected A" / "Protected B" / "Protected C"
- FR: "Protege A" / "Protege B" / "Protege C" (TBS authoritative French terms)
- Injury thresholds: PA=limited/moderate, PB=serious, PC=extremely grave (outside national interest)
- National interest = classified (Confidential/Secret/Top Secret). Non-national interest = Protected.
