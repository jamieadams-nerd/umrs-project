# Henri Onboarding Case Study — AI Transparency Artifact

**Date:** 2026-03-23
**Context:** First addition of a new specialized agent to a mature project team. Henri (Canadian Government Information Management & Bilingual Policy Specialist) was onboarded using the same orientation process a human team member would receive: infrastructure setup, corpus acquisition (via the Librarian), and a self-directed project familiarization pass.

**What was prepared before Henri arrived:**
- Agent definition (`.claude/agents/henri.md`) with role boundaries, rules, finding format
- Corpus acquisition brief (`jamies_brain/librarian-henri-needs.md`) — 5-priority-tier plan for Canadian federal policy documents
- Existing project architecture docs (ROADMAP, ARCHITECTURE, team-collaboration)
- Existing terminology corpus (58,091 entries across Termium Plus and OQLF GDT)
- Existing CUI labeling work and Five Eyes research

**What Henri was asked to do:**
Read everything relevant. Write orientation notes. Identify gaps. Ask questions.

---

## Henri's Orientation Response (Verbatim)

### What He Found

The project has a strong foundation. The terminology corpus (58,091 entries across Termium Plus and OQLF GDT) is genuinely impressive infrastructure. The Phase 1/Phase 2 language discipline is exactly right for regulatory credibility. Simone's translation pipeline is operational with the first domain (umrs-uname) translated to fr_CA.

### The Eight Gaps He Identified

1. **No Canadian Protected A/B/C mapping exists.** The CUI labeling work is entirely US-centric. M3 explicitly calls for "Canadian CUI equivalent labels" but no structured mapping has been produced.

2. **The CUI-to-Protected mapping will break down.** US CUI is category-based taxonomy; Canadian Protected A/B/C is impact-based tiering. These are structurally different systems. A simple equivalency table will be inaccurate.

3. **No Canadian policy validation of labels or tool output.** No one has reviewed what a Canadian operator sees when UMRS displays CUI markings, or whether dual-marking (CUI + PROTEGE B) is required.

4. **Terminology corpus is not policy-validated.** Termium Plus confirms correct federal French; it does not confirm correct regulatory weight in a security context. The TBS Directive's own terminology governs.

5. **My corpus is incomplete.** Only the TBS Directive on Security Management (English) has been acquired. I need the French version, the Standard on Security Categorization, the Policy on Government Security, and the legislation.

6. **No ATIP review has been done.** If UMRS deploys on Canadian government systems, ATIP obligations will apply to tool output and reports.

7. **French translations are linguistically correct but policy-unvalidated.** Simone's coinages ("verrou dur", "palier de confiance") need cross-referencing against TBS policy vocabulary.

8. **Phase 1 Language Directive needs a Canadian equivalent** with TBS-sourced vocabulary.

### His Correction of Our Terminology

> The phrasing "Canadian CUI equivalent labels" in the ROADMAP is misleading. Canada does not have CUI equivalents. Canada has Protected A/B/C, which is a different system with a different structure. What I will produce are Canadian Protected designations with documented approximate correspondence to US CUI — with caveats at every level where the mapping breaks down.

### Where He Said His Expertise Will Matter Most

**Immediate (M3 preparation):**
1. Produce the Protected A/B/C to CUI mapping document with proper caveats
2. Policy-validate Simone's French translations against TBS policy language
3. Review CUI labeling documentation for Canadian regulatory accuracy

**Medium-term (M3 execution):**
4. Define Canadian Protected labels for the MCS label architecture
5. Validate setrans.conf entries for Canadian deployments
6. Review Five Eyes interoperability documentation from the Canadian side

**Longer-term (M3.5+):**
7. ATIP assessment for Canadian government deployment scenarios
8. Official Languages Act compliance review
9. CCCS alignment validation

### His Six Questions for Jamie

1. Is UMRS expected to be deployed on Canadian government systems? (advisory vs prescriptive)
2. What is the relationship with Canadian government procurement? (TBS as reference vs hard requirement)
3. Should the Librarian prioritize the TBS Standard on Security Categorization?
4. Has Jamie discussed UMRS with any Canadian government contacts?
5. Which CUI categories will have French labels in the Canadian program?
6. The Termium Plus Military/Security subject download is pending — can Jamie prioritize?

### What He Said Is Working Well

- The terminology corpus infrastructure (58K+ entries before he started)
- The Simone/Henri separation (linguistic vs policy accuracy = different functions)
- The Phase 1/Phase 2 language discipline
- Jamie's corpus acquisition brief (bilingual pairing = the TBS's own French renderings ARE the authoritative term source)

### What He Said Needs Attention

- Canada deserves deeper Five Eyes treatment — only partner with a structurally different language requirement
- The CUI-to-Protected mapping is being discussed as if it were a simple equivalency table — it is not
- ROADMAP terminology "Canadian CUI equivalent labels" is misleading and needs correction

---

## Case Study Observations (Jamie / AI Transparency)

This is a case study in onboarding a new specialized AI agent to a mature project:

1. **Documentation readiness enabled quality orientation.** Because the project had architecture docs, a ROADMAP, team-collaboration rules, and existing domain research, Henri could orient himself without hand-holding. The quality of his output directly reflects the quality of what was available to read.

2. **The agent immediately identified a structural error the team had been carrying.** "Canadian CUI equivalent labels" was accepted shorthand — Henri correctly identified it as misleading because the systems are structurally different. This is the value of specialist perspective.

3. **Eight gaps in one pass.** A human consultant doing a first-day walkthrough of a complex security project would be expected to produce exactly this kind of deliverable — and most would not produce eight specific, actionable findings on day one.

4. **The orientation process itself is replicable.** Infrastructure setup → corpus acquisition → self-directed familiarization → written notes with gaps and questions. This is a transferable pattern for onboarding any new specialized agent.

5. **The agent's first instinct was to correct the team's language.** Henri didn't accept the existing terminology — he pushed back with a precise correction and a rationale grounded in the structural difference between the two systems. This is exactly what a Canadian government consultant would do.
