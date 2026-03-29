# MIL-STD-38784B — Manual Download Required

**Full title:** Department of Defense Standard Practice: General Style and Format Requirements
for Technical Manuals
**Document number:** MIL-STD-38784B
**Version:** Revision B (supersedes MIL-STD-38784A)
**Date:** 16 November 2020
**Issuing authority:** Department of Defense (NPFC — Naval Publications and Forms Center)

## Status: Requires Manual Download

**Reason:** The official download source (ASSIST / quicksearch.dla.mil) requires DoD/CAC
credentials. The plan specified everyspec.com, which is a third-party mirror and is NOT on
the researcher approved sources list. Researcher policy prohibits adding documents from
unapproved sources.

## How to Download

**Option 1 — CAC-authenticated access (preferred):**
1. Go to https://quicksearch.dla.mil
2. Search for "MIL-STD-38784B"
3. Download the PDF
4. Save to `.claude/references/tech-writer-corpus/gov-standards/mil-std-38784b.pdf`
5. Compute SHA-256 and record in manifest

**Option 2 — via AF eTIMS (requires .mil email):**
- https://www.my.af.mil/etims/ETIMS/index.jsp

**Option 3 — User approval for everyspec.com:**
- If Jamie approves everyspec.com as a source, the direct download URL is:
  https://everyspec.com/MIL-STD/MIL-STD-10000-and-Up/download.php?spec=MIL-STD-38784A.036449.pdf
  (Note: this is Revision A; Revision B URL would need to be confirmed)
- Note: everyspec.com is a widely-used community mirror of unclassified DoD specs, but is
  not an official .mil source.

## Key Content Summary (from search results)

MIL-STD-38784B governs:
- General style and format requirements for DoD Technical Manuals (TMs)
- Applicable to installation, operation, maintenance, repair, and logistics support publications
- Warnings, cautions, and notes in procedures
- Service-specific requirements (Army, Air Force, Marine Corps, Navy)
- SGML/DTD requirements for electronic data delivery
- CUI (Controlled Unclassified Information) requirements — added in Revision B
- Table of contents structure
- List of illustrations and tables
- Alphabetical index requirements

## Revision B Changes (from MIL-STD-38784A)

- Added CUI requirements throughout
- Clarification: foreword/preface/introduction and TOC start on right-hand page
- Clarification: list of illustrations starts on a new page; list of tables starts on new page
- Column headers in alphabetical index should reflect contents
- Removal of figure title and number from margin data requirements

## NAVSEA DTD/Schema Resources

The Naval Sea Systems Command hosts related MIL-STD-38784 DTDs/Schemas at:
https://www.navsea.navy.mil/Home/Warfare-Centers/NSWC-Carderock/Resources/Technical-Information-Systems/Navy-XML-SGML-Repository/DTDs-Schemas/MIL-STD-38784/

## Relevance to UMRS

MIL-STD-38784B governs how warnings, cautions, and notes appear in DoD technical manuals —
directly relevant to UMRS procedure documentation structure. Key patterns applicable to UMRS:

- Warning/Caution/Note hierarchy (maps to AsciiDoc WARNING/CAUTION/NOTE admonitions)
- Numbered procedure step format
- Service-specific content tagging (relevant for multi-audience documentation)
- CUI requirements for marking controlled content
