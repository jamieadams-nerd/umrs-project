# Stanford CS147 — Introduction to HCI Design (Autumn 2023)

**Collection:** `hci-courses/stanford-cs147`
**Date acquired:** 2026-03-22 (attempted)
**Source:** https://hci.stanford.edu/courses/cs147/2023/au/lectures/
**License:** Educational use (Stanford University public course materials)

## Status: REQUIRES MANUAL DOWNLOAD

**Reason:** hci.stanford.edu uses a TLS configuration that is incompatible with RHEL10 FIPS
crypto policy. The system TLS stack rejects the handshake with:
`TLS connect error: error:1C8000E9:Provider routines::ems not enabled`

This affects both `curl` and `wget`. The `WebFetch` internal tool can access the page but
cannot download binary files.

## Manual Download Instructions

1. Open a browser and navigate to: https://hci.stanford.edu/courses/cs147/2023/au/lectures/
2. Download each of the following PDFs:

| Filename | Topic |
|---|---|
| 01-introduction.pdf | Introduction to HCI Design |
| 02-design-discovery.pdf | Design Discovery |
| 03-define.pdf | Define Phase |
| 04-ideate.pdf | Ideation |
| 05-video.pdf | Video Prototyping |
| 06-design-exploration.pdf | Design Exploration |
| 07-early-stage-prototyping.pdf | Early-Stage Prototyping |
| 08-human-abilities.pdf | Human Abilities |
| 09-visual-design-wo-tufte.pdf | Visual Design |
| 11-conceptual-models.pdf | Conceptual Models |
| 12-heuristic-evaluation.pdf | Heuristic Evaluation |
| 13-hci-visions.pdf | HCI Visions |
| 14-usability-testing.pdf | Usability Testing |
| 15-midterm-review.pdf | Midterm Review |
| 17-design-patterns.pdf | Design Patterns |
| 18-career-panel.pdf | Career Panel |
| 19-tracy.pdf | Tracy Lecture |
| 20-smart-interfaces-for-hai.pdf | Smart Interfaces for HAI |
| 21-what-is-hai.pdf | What is HAI |
| 22-project-expo.pdf | Project Expo |

3. Save all files to `.claude/references/hci-courses/stanford-cs147/`
4. Run `sha256sum .claude/references/hci-courses/stanford-cs147/*.pdf` and add checksums to the acquisition log.
