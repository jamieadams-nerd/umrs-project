---
name: Jamie BC2A Operational History
description: Jamie's firsthand account of the 1996 BC2A (Bosnian Command and Control Augmentation) program at RAF Molesworth — SATCOM, ATM networking, ISR, early Predator video, NITF imagery. Primary source material for AI Journey blog and conference context.
type: user
---

## What This Is

Jamie's written account of operational work in 1996 at RAF Molesworth on the BC2A program.
Source file: `docs/sage/inbox/jamie-bc2a.txt`

This is primary source material — Jamie was there. Treat every technical detail as firsthand,
citable, and authoritative. Do not restate it as hearsay.

## Technical Content (exact details from Jamie's account)

- **Program**: BC2A (Bosnian Command and Control Augmentation), stood up post-Bosnian War
- **Location**: RAF Molesworth, 1996
- **Problem solved**: Near-zero communications infrastructure in theater; coalition C2 had no backbone
- **Solution**: Rapidly deployed SATCOM (VSAT, Ku-band) augmentation layer + ATM networking
- **ATM significance**: Chosen for QoS/CBR (Constant Bit Rate) — deterministic delivery for real-time data; IP networks of the era could not reliably support low-jitter, low-latency streams
- **Jamie's role**: Sun Solaris-based systems ingesting and disseminating intelligence products, specifically NITF-formatted imagery from SATCOM links
- **ISR integration**: Live RQ-1 Predator video transmitted over Ku-band SATCOM, ingested into ground network, distributed across ATM backbone
- **Video codecs**: H.261 and early H.263, hardware-assisted; streams packetized into ATM cells
- **Collaboration capability**: Live ISR feeds embedded directly into video teleconferences — geographically dispersed teams watching real-time sensor feeds while coordinating; described as "striking" for mid-1990s
- **Partners**: DARPA involvement, Charles Stark Draper Laboratory contributors
- **Character of program**: Small, highly collaborative, experimental/advanced-technology

## Why This Matters for Outreach

This is the origin story layer of Jamie's career. It predates SELinux, predates the CUI
framework, predates everything UMRS is building on — and it establishes the stakes. The
problems UMRS solves (CUI handling, structured data dissemination, coalition information
sharing with integrity) have direct operational ancestors here.

**Content angles:**

1. **AI Journey blog**: Section on Jamie's background. The BC2A story shows why information
   assurance at the system level is not theoretical to Jamie — it was operational necessity
   in a war zone. Predator video over ATM is the ancestor of every CUI data pipeline UMRS
   is designed to protect.

2. **Conference abstract (NSA IA Symposium, Five Eyes)**: Contextual credibility. Jamie
   was doing BLOS ISR data dissemination with structured intelligence formats (NITF) in
   1996. That lineage is directly relevant when claiming deep domain knowledge.

3. **"How We Got Here" blog post**: The thread from Bosnian SATCOM to UMRS is a compelling
   narrative arc. NITF imagery dissemination in 1996 → CUI labeling and enforcement in 2026.
   The problem of "getting the right data to the right people with integrity" is the same
   problem, thirty years later, now running on commodity Linux in Rust.

4. **Five Eyes partner introductions**: Coalition interoperability is in UMRS's DNA.
   BC2A was a coalition operation. That framing is directly relevant when approaching
   UK/CA/AU/NZ partners.

## How to Use

- Do NOT publish the BC2A text verbatim without Jamie's approval — this is background material
- Use technical details freely when they appear naturally in Jamie's voice (AI Journey blog)
- The summary line Jamie wrote is quotable for context: "BC2A established a SATCOM-enabled,
  ATM-based (QoS/CBR) communications backbone in a communications-denied environment...
  well ahead of what conventional IP networks could support at the time."
- When Jamie is ready to write the AI Journey background section, pull from this file
- This material also supports E-E-A-T (Experience, Expertise, Authoritativeness, Trustworthiness)
  claims — it is exactly the kind of lived operational experience that Google's quality
  raters and human readers both respond to

## Status

Filed 2026-03-22. Awaiting Jamie's direction on when and how to use.
