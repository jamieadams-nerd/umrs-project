# Agent Knowledge Acquisition Plan: HCI, Information Architecture, Knowledge Organization, and Technical Communication

**Date:** 2026-03-21
**Prepared by:** The Librarian (researcher agent)
**Purpose:** Prioritized acquisition plan for educational materials that make Elena, Sage, and Simone's academic backgrounds operationally real as RAG knowledge.

---

## Executive Summary

This plan covers four academic disciplines corresponding to the UMRS team's defined educational backgrounds: Human-Computer Interaction (HCI), Information Architecture (IA), Knowledge Organization (KO), and Technical Communication Theory. For each domain, this report identifies:

- What open-access materials actually exist (verified by search)
- What the canonical foundational texts are
- Whether YouTube lecture ingestion is practical (answer: yes, with caveats)
- Non-obvious high-value sources
- A prioritized acquisition sequence

---

## 1. What Is Actually Available — Domain by Domain

### 1.1 Human-Computer Interaction (HCI)

**MIT OCW 6.831 — User Interface Design and Implementation (Spring 2011)**
The most directly relevant MIT course available. All 23+ lecture PDFs are publicly downloadable. Topics include: usability, learnability, visibility, efficiency, error handling, user-centered design, task analysis, UI software architecture, layout, input/output models, user testing, and controlled experiments. All materials are CC-licensed.
- URL: https://ocw.mit.edu/courses/6-831-user-interface-design-and-implementation-spring-2011/
- Format: PDFs, downloadable as a zip package
- Status: READY FOR INGESTION — direct curl download of individual PDFs or the course zip

**Stanford CS147 — Introduction to HCI Design (Autumn 2023)**
Full lecture deck publicly available at a directory index. 22 lectures covering: design discovery, define/ideate, prototyping, human abilities, visual design, conceptual models, heuristic evaluation, usability testing, design patterns, and human-AI interaction.
- URL: https://hci.stanford.edu/courses/cs147/2023/au/lectures/
- Format: PDF slides (all publicly accessible, no login required)
- Status: READY FOR INGESTION — curl each PDF from the directory

**Stanford CS347 — HCI Foundations & Frontiers (Winter 2025)**
Graduate-level course. The syllabus is fully public and lists all papers per week. Approximately 2 papers per session across 10 weeks. Topics include ubiquitous computing, social computing, design cognition, cognitive models, visualization, accessibility, and methodology.
- URL: https://stanfordhci.github.io/cs347-winter-2025/syllabus
- Papers include: Weiser "Computer for the 21st Century" (1991), Norman DOET Ch. 2 (2013), Schön "Reflective Practitioner" Ch. 3 (1983), Olson & Olson "Distance Matters" (2000), Vannevar Bush "As We May Think" (1945), Nissenbaum "Privacy as Contextual Integrity" (2004), and ~18 CHI/UIST/TOCHI papers (mixed access)
- Status: ACQUISITION REQUIRED — papers are not bundled; many are open access on ACM DL (2026 full OA) or author pages; some require institutional access

**MIT OCW Cognitive Science courses (supporting material)**
These feed into HCI's "user cognition" dimension:
- 9.65 Cognitive Processes (Spring 2004): https://ocw.mit.edu/courses/9-65-cognitive-processes-spring-2004/
- 9.66J Computational Cognitive Science (Fall 2004): https://ocw.mit.edu/courses/9-66j-computational-cognitive-science-fall-2004/
- 9.00SC Introduction to Psychology (Fall 2011): full video lectures with transcripts

**CMU HCII**
CMU does not publish centralized open courseware. Individual faculty pages host past course sites (e.g., Brad Myers' 2007 course materials at cs.cmu.edu/~bam/uicourse/). The MHCI curriculum is described publicly but materials are behind Canvas. Not a practical bulk ingestion target without manual curation.

### 1.2 Information Architecture

**Rosenfeld, Morville, Arango — "Information Architecture: For the Web and Beyond" (4th ed., O'Reilly)**
The canonical text. Known as the "polar bear book." Still used as a textbook (Kent State MUXD program confirmed). Covers: organization systems, labeling systems, navigation systems, search systems, thesauri, controlled vocabularies, metadata, and the information ecology model.
- Access: Chapter 1 is freely available via O'Reilly preview. Full text requires purchase or O'Reilly subscription.
- A PDF of the 4th edition was found at an academic repository URL (NBU Bulgaria) — this may not be an authorized copy. Do not ingest from unauthorized sources.
- Status: MANUAL ACQUISITION — purchase or O'Reilly subscription. Priority: HIGH.

**Peter Morville — "Ambient Findability" (O'Reilly, 2005)**
Covers: information scent, wayfinding, the semantic web, tagging and folksonomies, and how humans navigate information spaces. Extends the IA framework into the mobile/web ecosystem.
- Access: O'Reilly; no verified open access.
- Status: MANUAL ACQUISITION — lower priority than polar bear book.

**Peter Morville — "Intertwingled" (Semantic Studios, 2014)**
Morville's synthesis work on the connections between systems thinking, information architecture, and cognitive ecology. Self-published; more accessible than the O'Reilly texts.
- URL: https://intertwingled.org/the-polar-bear-book/ (site exists; book sold separately)
- Status: MANUAL ACQUISITION.

**Understanding Information Architecture (Precision Content)**
A free white paper available at: https://www.precisioncontent.com/wp-content/uploads/Understanding-Information-Architecture.pdf
- Status: READY FOR INGESTION — direct PDF download.

**Information Foraging Theory — Pirolli and Card (PARC)**
Foundational theory for how users navigate information spaces. The original paper is on ResearchGate and Pirolli's own site as a PDF.
- Pirolli chapter excerpt: https://www.peterpirolli.com/ewExternalFiles/31354_C01_UNCORRECTED_PROOF.pdf
- ResearchGate PDF: https://www.researchgate.net/publication/229101074_Information_Foraging
- Status: READY FOR INGESTION — both are author-distributed copies.

### 1.3 Knowledge Organization

**ISKO Encyclopedia of Knowledge Organization (IEKO)**
Published by the International Society for Knowledge Organization since 2016. Fully open access, online. Edited by Birger Hjørland and Claudio Gnoli. Covers: classification theory, thesauri, ontologies, faceted classification, bibliographic control, knowledge representation, and epistemological foundations of KO.
- URL: https://www.isko.org/cyclo/
- Format: individual HTML articles per concept
- Status: READY FOR INGESTION — structured web content; curl + pandoc per article. This is a high-priority target.

**Elaine Svenonius — "The Intellectual Foundation of Information Organization" (MIT Press, 2000)**
The theoretical cornerstone of the KO field. Covers: bibliographic languages, work/document/subject languages, the principle of user convenience, the principle of representation, and the ontological status of documents. MIT Press published it; it is available on Internet Archive.
- Internet Archive: https://archive.org/details/intellectualfoun0000sven
- Status: VERIFY LOAN AVAILABILITY — Internet Archive lending; if borrowable, ingest the text. If not, manual acquisition.
- Evergreen College hosts Chapter 5 publicly: https://sites.evergreen.edu/wp-content/uploads/sites/226/2016/08/svenonius-chap5.pdf (READY FOR INGESTION)

**Birger Hjørland — "Theories of knowledge organization — theories of knowledge"**
Available on Semantic Scholar and Academia.edu. Hjørland is the field's leading contemporary theorist.
- Semantic Scholar: https://www.semanticscholar.org/paper/Theories-of-knowledge-organization-%E2%80%94-theories-of-Hj%C3%B8rland/9d32ec1eafc0f58281d708669902f5c5a3b99ef2
- Status: READY FOR INGESTION — open access paper.

**ISKO UK — Introductory and Advanced Publications lists**
ISKO UK maintains curated reading lists at two levels:
- Introductory: https://www.iskouk.org/knowledge-organization-introductory-publications
- Advanced: https://www.iskouk.org/knowledge-organization-advanced-publications
- Status: READ FIRST — these lists should drive the acquisition sequence for the KO domain.

**Annual Review of Information Science and Technology (ARIST)**
Published 1966–2011; relaunched 2023 in JASIST. HathiTrust holds volumes 1–45 (1966–2011) of the original run.
- HathiTrust: https://onlinebooks.library.upenn.edu/webbin/serial?id=arist
- Status: PARTIAL ACCESS — HathiTrust access depends on institutional affiliation; some volumes are in public domain or open. Investigate before bulk ingestion.

### 1.4 Technical Communication Theory

**Carolyn Miller — "Genre as Social Action" (1984)**
The most-cited article in the Quarterly Journal of Speech. Defines genre as typified rhetorical action, connecting the rhetorical tradition (Bitzer's rhetorical situation) to phenomenological sociology (Schutz's typification). This is the foundational move that made technical communication an academic discipline.
- Open access PDF: https://oportuguesdobrasil.files.wordpress.com/2015/02/miller_genre_as_social_action.pdf
- ResearchGate: https://www.researchgate.net/publication/238749675_Genre_as_Social_Action
- Status: READY FOR INGESTION.

**Charles Bazerman — "Genre as Social Action" chapter (Routledge Handbook of Discourse Analysis, 2011)**
Bazerman's treatment of the same concept, extending Miller's framework.
- Open PDF: https://bazerman.education.ucsb.edu/sites/default/files/docs/Bazerman2011chapGenreassocialaction.pdf
- Status: READY FOR INGESTION.

**Technical Communication Quarterly (TCQ)**
Hybrid open access journal (Taylor & Francis Open Select). The premier academic TC journal in North America. Some articles available without subscription; others behind paywall. Covers: genre theory, usability, plain language, technical rhetoric, information design.
- URL: https://www.tandfonline.com/journals/htcq20
- Status: SELECTIVE ACQUISITION — identify key open-access articles. Do not bulk-download paywalled content.

**STC Technical Communication Journal**
STC ceased operations January 29, 2025. Journal archived on JSTOR and Ingenta. JSTOR access to older volumes (pre-2023) may be available depending on access level.
- JSTOR: https://www.jstor.org/journal/techcomm
- Status: INVESTIGATE — check JSTOR open access eligibility for pre-2005 articles.

**IEEE Transactions on Professional Communication**
Not prominently surfaced in this search. Institutional access required for most content. Lower priority for open-access ingestion.

---

## 2. Foundational Texts — The Canonical Reading List

This is the "books every student in these fields reads" list, organized by domain.

### HCI Core
| Title | Author | Year | Status |
|---|---|---|---|
| The Design of Everyday Things (revised ed.) | Don Norman | 2013 | Purchase — excerpts in CS347 readings |
| Designing with the Mind in Mind (3rd ed.) | Jeff Johnson | 2020 | Purchase — cognitive/perceptual foundations |
| Don't Make Me Think, Revisited | Steve Krug | 2014 | Purchase — usability heuristics, practical |
| The Humane Interface | Jef Raskin | 2000 | Purchase — cited in CS347 |
| The Reflective Practitioner | Donald Schön | 1983 | Purchase — design cognition, cited in CS347 |
| Things That Make Us Smart | Don Norman | 1993 | Purchase — cited in CS347 |
| Human-Computer Interaction (3rd ed.) | Dix, Finlay, Abowd, Beale | 2004 | Purchase — standard textbook |

### Information Architecture Core
| Title | Author | Year | Status |
|---|---|---|---|
| Information Architecture: For the Web and Beyond (4th ed.) | Rosenfeld, Morville, Arango | 2015 | Purchase/O'Reilly sub — THE foundational text |
| Ambient Findability | Peter Morville | 2005 | Purchase — information scent, navigation |
| Information Foraging (book) | Peter Pirolli | 2007 | Chapter 1 PDF available free |
| A Taxonomy of Knowledge Classification | Ranganathan (secondary sources) | — | IEKO articles cover this |

### Knowledge Organization Core
| Title | Author | Year | Status |
|---|---|---|---|
| The Intellectual Foundation of Information Organization | Elaine Svenonius | 2000 | Internet Archive (borrow); Ch. 5 free |
| Introduction to Modern Information Retrieval (3rd ed.) | Salton, McGill / Chowdhury | 2010 | Purchase — IR theory foundations |
| Organizing Knowledge (4th ed.) | Rowley & Hartley | 2008 | Purchase — KO systems overview |
| ANSI/NISO Z39.19 — Guidelines for the Construction, Format, and Management of Monolingual Controlled Vocabularies | NISO | 2010 | Free from NISO (standard document) |

### Technical Communication Theory Core
| Title | Author | Year | Status |
|---|---|---|---|
| "Genre as Social Action" | Carolyn Miller | 1984 | FREE — open access PDF |
| Shaping Written Knowledge | Charles Bazerman | 1988 | Free — University of California open access |
| Technical Communication: A Reader-Centered Approach | Paul Anderson | 2018 | Purchase |
| The Handbook of Technical Communication | Faber & Johnson-Eilola (eds.) | 2010 | Purchase — academic reference |

---

## 3. YouTube Course Approach — Practical Assessment

### Is It Practical?

Yes, with specific conditions. The approach works well when:
1. The channel publishes auto-generated or manual captions (YouTube Data API confirms caption availability)
2. The content is lecture-style monologue (not discussion-only)
3. The videos are long-form (20+ minutes) — short clips produce low-density chunks

### The Logistics

**Tool:** `yt-dlp` is the correct tool. It is available on this system (check `~/.cargo/bin/` or system path).

**Transcript-only download (no video):**
```bash
yt-dlp --write-auto-subs --sub-format vtt --skip-download \
  --output "%(playlist_index)s-%(title)s.%(ext)s" \
  "<playlist_url>"
```

**VTT to plain text:** VTT files contain timestamps and duplicate lines. A small Python script strips timestamps and deduplicates consecutive identical lines to produce clean prose. This is a one-time preprocessing step.

**What to target:**

| Channel / Source | Content | Caption Quality | Viability |
|---|---|---|---|
| MIT OpenCourseWare YouTube | 9.00SC Intro to Psychology (cognitive basis); 9.13 Human Brain | Auto-generated, generally good | HIGH |
| Stanford HCI YouTube | CS147 lecture recordings (some posted) | Variable | MEDIUM — verify per video |
| Nielsen Norman Group YouTube | UX and IA talks by principals | Manual captions on many | HIGH |
| ASIS&T YouTube | Annual meeting talks on KO and IA | Variable | MEDIUM |
| CMU HCII YouTube | Seminar recordings | Variable | MEDIUM |

**What NOT to target:**
- Short panel discussion clips (too much crosstalk, poor transcript quality)
- Videos without captions (auto-generated quality degrades badly on technical content with jargon)
- Paywalled content (LinkedIn Learning, Coursera paid, etc.)

**Transcript quality warning:** Auto-generated captions on technical academic lectures frequently mishear domain terminology. "Ontology" becomes "on top of me," "faceted classification" becomes "fastest classification." Post-processing with a domain-specific word list improves quality but adds pipeline complexity. For high-priority content, manual review of a sample before bulk ingestion is recommended.

**Recommended first target:** MIT OCW 9.00SC Introduction to Psychology — full lecture set, high caption quality, directly relevant to the cognitive foundations of HCI (perception, attention, memory, decision-making).

---

## 4. Non-Obvious High-Value Sources

### ISKO Encyclopedia (IEKO) — Highest Priority
This is genuinely exceptional. It is a peer-reviewed, open-access, online encyclopedia covering the entire intellectual territory of knowledge organization — classification theory, thesauri, controlled vocabularies, ontologies, faceted classification, bibliographic description, subject analysis, and epistemological foundations. Edited by the field's leading scholar (Hjørland). Each article is 3,000–10,000 words with full citations. Approximately 100+ articles published to date.
- URL: https://www.isko.org/cyclo/
- This should be a named collection: `knowledge-organization/ieko`

### NISO Standards — Free and Authoritative
NISO (National Information Standards Organization) publishes several standards directly relevant to IA and KO that are free to download:
- **ANSI/NISO Z39.19-2005 (R2010)** — Guidelines for constructing controlled vocabularies. The professional standard for taxonomy and thesaurus design. Download from niso.org.
- **ANSI/NISO Z39.85** — Dublin Core Metadata Element Set (metadata standards)
- These are directly applicable to UMRS CUI labeling work — the metadata standards for organizing controlled information are the same intellectual machinery as CUI category taxonomy design.

### Cognitive Dimensions of Notations (Green & Blackwell)
A lightweight but powerful framework for evaluating the usability of any notation system — programming languages, UIs, data formats. Directly applicable to UMRS: evaluating the usability of security context strings, CUI labels, and posture display formats. Papers are freely available from Cambridge University (Blackwell's page) and ResearchGate.
- Cambridge PDF: https://www.cl.cam.ac.uk/~afb21/publications/BlackwellGreen-CDsChapter.pdf

### ACM Digital Library — 2026 Transition
ACM is completing a transition to 100% open access as of January 1, 2026. This means all CHI proceedings (the flagship HCI conference, running since 1983) are now or will soon be open access. The historical archive going back decades is an enormous corpus of HCI research. Worth checking current status of CHI 2024 and earlier proceedings.
- CHI 2025: https://dl.acm.org/doi/proceedings/10.1145/3706598
- CHI 2024: https://dl.acm.org/doi/proceedings/10.1145/3613904

### Bazerman's Self-Hosted Papers
Charles Bazerman (rhetorical genre theory, technical communication) hosts many of his papers as free PDFs on his UCSB faculty page (bazerman.education.ucsb.edu). This includes "Shaping Written Knowledge" chapters and numerous articles. This is a legitimate, author-distributed source.

### Vannevar Bush — "As We May Think" (1945)
Foundational HCI document — the original vision of hypertext, information retrieval, and the Memex. Free everywhere; published in The Atlantic. CS347 assigns it.
- Atlantic archives: publicly accessible.
- Worth ingesting as a historical anchor document.

### Mark Weiser — "The Computer for the 21st Century" (Scientific American, 1991)
The founding document of ubiquitous computing. Also assigned in CS347. Author copies widely available.
- Weiser's PARC page hosts the original: http://www.ubiq.com/hypertext/weiser/SciAmDraft3.html (verify availability)

---

## 5. Prioritized Acquisition Plan

### Tier 1 — Immediate, Zero-Cost, High-Impact (do these first)

These require no purchase decision and are directly fetchable:

| # | Resource | Collection Name | Method | Impact |
|---|---|---|---|---|
| 1 | ISKO IEKO encyclopedia articles | `knowledge-organization/ieko` | curl + pandoc per article | Foundational KO theory corpus |
| 2 | Stanford CS147 AU2023 lecture PDFs (22 slides) | `hci-courses/stanford-cs147` | curl from directory index | HCI design process + methods |
| 3 | MIT OCW 6.831 lecture PDFs (23 lectures) | `hci-courses/mit-6831` | curl from OCW download page | HCI fundamentals, cognitive models |
| 4 | Miller "Genre as Social Action" | `technical-communication/theory` | curl from open PDF | TC discipline foundation |
| 5 | Bazerman genre chapter (Routledge) | `technical-communication/theory` | curl from bazerman.edu.ucsb.edu | TC/genre theory |
| 6 | Pirolli information foraging chapter | `information-architecture/theory` | curl from peterpirolli.com | IA + user behavior theory |
| 7 | Svenonius Ch. 5 (Evergreen College) | `knowledge-organization/texts` | curl from evergreen.edu | KO theory excerpt |
| 8 | Hjørland KO theories paper | `knowledge-organization/texts` | Semantic Scholar PDF | KO epistemological framework |
| 9 | Blackwell/Green Cognitive Dimensions | `hci-courses/theory-papers` | curl from Cambridge | Notation/UI evaluation framework |
| 10 | Vannevar Bush "As We May Think" | `hci-courses/theory-papers` | Project Gutenberg or Atlantic | Historical foundation |
| 11 | NISO Z39.19 controlled vocabularies standard | `information-architecture/standards` | niso.org free download | Taxonomy design standard |
| 12 | Precision Content IA white paper | `information-architecture/theory` | curl from precisioncontent.com | Accessible IA overview |

### Tier 2 — Purchase Decision Required (discuss with Jamie)

| # | Resource | Approx. Cost | Priority |
|---|---|---|---|
| 1 | Rosenfeld/Morville/Arango IA 4th ed. (O'Reilly sub or purchase) | $50 or O'Reilly sub | HIGHEST — THE foundational text |
| 2 | Johnson "Designing with the Mind in Mind" 3rd ed. | $45 | HIGH — cognitive psychology for UI design |
| 3 | Norman "The Design of Everyday Things" revised ed. | $20 | HIGH — assigned in both CS147 and CS347 |
| 4 | Svenonius "Intellectual Foundation of Information Organization" (MIT Press) | $30 (or Internet Archive borrow) | HIGH — KO theoretical core |
| 5 | Krug "Don't Make Me Think, Revisited" | $25 | MEDIUM — usability heuristics, practical |
| 6 | Raskin "The Humane Interface" | $35 used | MEDIUM — assigned CS347 |
| 7 | Norman "Things That Make Us Smart" | $20 used | MEDIUM — assigned CS347 |
| 8 | Morville "Ambient Findability" | $25 used | MEDIUM — IA + semantic web |

**O'Reilly Learning subscription** ($50/month or $500/year) would unlock Rosenfeld/Morville IA, most O'Reilly technical texts, and allows text copying for RAG ingestion. Worth evaluating if the UMRS team will use it for other technical references.

### Tier 3 — YouTube Transcript Pipeline (requires pipeline work)

Recommended sequence after Tier 1 and 2 are complete:

1. **MIT OCW 9.00SC Introduction to Psychology** — full lecture set, cognitive foundations for HCI; establish VTT pipeline and preprocessing here first.
2. **Nielsen Norman Group YouTube channel** — Jakob Nielsen and Don Norman talks; manual captions on many videos; directly applicable to UX evaluation.
3. **Stanford CS147 recorded lectures** — if posted on the Stanford HCI YouTube; supplement the slide PDFs with spoken content.

### Tier 4 — Journal/Proceedings (selective, case by case)

- ACM CHI proceedings: now moving to full OA. Target key papers from CS347 syllabus that are not yet downloaded.
- TCQ open-access articles: check for open-access versions of highly-cited foundational articles (topic modeling, usability theory, plain language research).
- ARIST via HathiTrust: investigate access; if available, the survey articles from 1970s–2000s on classification theory, information retrieval, and KO are extremely dense and high-value.

---

## 6. UMRS Relevance — Why These Domains Matter for Documentation Quality

This is not education for its own sake. Each domain has direct operational leverage:

**HCI → Elena's documentation design decisions**
Understanding conceptual models, Gulf of Evaluation/Execution, and affordances lets Elena design documentation that matches users' mental models of the security system — not just describe features but scaffold understanding. The cognitive dimensions framework applies directly to evaluating whether a CLI output or TUI layout communicates security state legibly.

**Information Architecture → Taxonomy and navigation design**
The IA discipline's tools (controlled vocabularies, faceted classification, labeling systems, navigation systems) are the intellectual machinery behind CUI category taxonomy, UMRS documentation navigation, and the posture signal catalog structure. Rosenfeld/Morville is directly applicable to how the UMRS reference module is organized.

**Knowledge Organization → Catalog and classification design**
The KO field's work on classification theory (Ranganathan's laws, faceted analysis, warrant) directly informs how CUI categories are structured, how posture signals are classified, and how the UMRS catalog's taxonomy should be designed. Svenonius's principle of user convenience and principle of representation are design criteria for any classification system.

**Technical Communication Theory → Genre awareness**
Miller's genre theory explains why a deployment guide, a reference page, an audit report, and a blog post are fundamentally different genres with different social functions — not just different formats. Understanding genre lets Elena and Sage make correct decisions about what information belongs in which document type, rather than accumulating all information in one place.

---

## Collection Naming Recommendation

```
.claude/references/hci-courses/          — MIT 6.831, Stanford CS147, theory papers
.claude/references/information-architecture/  — Morville/Rosenfeld texts, IA standards, Pirolli
.claude/references/knowledge-organization/   — IEKO articles, Svenonius, Hjørland, NISO Z39.19
.claude/references/technical-communication/ — Miller, Bazerman, TCQ selected articles
```

These four collections map directly to Elena's three degrees plus Sage's editorial domain.

---

## Notes on Provenance

All Tier 1 resources listed as READY FOR INGESTION are either:
- CC-licensed (MIT OCW)
- Author-distributed open access (Miller, Bazerman, Pirolli)
- Open-access by the publishing organization (ISKO IEKO, NISO standards)
- Institutional educational content with public access (Stanford course slides)

None of the Tier 1 resources require bypassing paywalls or using unauthorized copies. The NBU Bulgaria PDF of the Rosenfeld/Morville book found during search is not included — its authorization status cannot be verified.
