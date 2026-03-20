---
layout: post
title: "Your CUI Policy Is a Sign. Here Is How to Build the Lock."
author: Jamie Adams
date: 2026-03-19
description: >
  Fifteen years of CUI policy produced compliance theater, not enforcement. This post
  traces the lineage from Bell-LaPadula to CMMC 2.0 and shows how UMRS — built in
  memory-safe Rust on SELinux MLS — brings kernel-enforced mandatory access control
  to unclassified sensitive data, for real this time.
tags:
  - CUI
  - SELinux
  - MLS
  - CMMC
  - mandatory-access-control
  - Rust
  - memory-safety
  - Bell-LaPadula
  - NIST-800-171
  - high-assurance
  - DFARS
  - defense-contractors
  - reference-monitor
  - umrs
categories:
  - security-engineering
  - compliance
  - open-source
keywords: >
  CUI protection, SELinux MLS, CMMC compliance, Rust memory safety, Bell-LaPadula,
  mandatory access control, NIST 800-171, high-assurance Linux, kernel-level enforcement,
  defense contractor CUI, SELinux MCS, DFARS 252.204-7012, reference monitor,
  no-unsafe Rust, IMA integrity, dm-crypt CUI
image: /assets/images/cui-sign-lock.png
image_alt: "A padlock beside a 'CUI — Authorized Personnel Only' sign — one is policy, one is enforcement."
canonical_url: https://jamieadams-nerd.github.io/
---

<!-- Cross-post targets: dev.to, r/netsec, r/rust, jamieadams.dev -->
<!-- Audience: security engineers at defense contractors, Rust practitioners in regulated environments -->

I have been in this industry long enough to remember when For Official Use Only (FOUO) meant
whatever the person who printed it wanted it to mean. Before 2010: no unified framework,
just incompatible agency markings — FOUO, Sensitive But Unclassified (SBU), Law Enforcement
Sensitive (LES), SENSITIVE. None enforceable.

Executive Order 13556 established the Controlled Unclassified Information (CUI) program in
2010. The National Archives and Records Administration (NARA) became the executive agent.
NIST Special Publication 800-171 published 110 controls. Contractors started writing System
Security Plans (SSPs).

And then almost nothing changed. Policy existed. Training got delivered. Information kept
leaking.

The US is not alone. Five Eyes partners have equivalent frameworks — the UK Government
Security Classification Policy (GSCP), Canada's Protected A/B/C levels, Australia's
Protective Security Policy Framework (PSPF), and New Zealand's Protective Security
Requirements (PSR). Allied nations maintain their own: France's SGDSN IGI 1300 is one
example. UMRS is designed for loadable country label profiles so coalition systems
can carry partner and allied schemes alongside CUI (Phase 2).

## The Sign That Replaced the Lock

The CUI self-attestation era was not dishonest because contractors were bad actors. The
Defense Federal Acquisition Regulation Supplement (DFARS) clause 252.204-7012 required
submitting a compliance score to the Supplier Performance Risk System (SPRS). No independent
verification. No way to distinguish a contractor who implemented the controls from one who
had checked boxes and moved on.

F-35 specifications, submarine propulsion data, acquisition strategies — protected by an
optimistic SPRS score and a sign reading "CUI: authorized personnel only."

The classified world solved this in the 1970s. The reference monitor — Bell-LaPadula, the
Orange Book Trusted Computer System Evaluation Criteria (TCSEC), Security-Enhanced Linux
(SELinux) — is built on the recognition that administrative controls fail. Humans forget.
The kernel does not.

Bell-LaPadula's "no read up, no write down" is a formal model in the operating system —
no process reads above its clearance level regardless of what the human wants. Sign versus
lock.

CUI has had fifteen years of signs. The Cybersecurity Maturity Model Certification (CMMC)
2.0 is finally demanding locks.

## The Assurance Pendulum Reaches Its Limit

The software assurance pendulum has been swinging for fifty years. Engineering discipline
versus market velocity. The formal methods era gave us Ada, SPARK, the Orange Book. Then
the desktop revolution swung toward "ship fast, patch later." The fast era produced the
broken era — the Morris Worm, Code Red, SQL Slammer, Solar Sunrise. The broken era produced
regulation. Regulation without enforcement produced compliance theater.

CMMC 2.0 Final Rule effective December 2024 makes CUI compliance a contractual condition.
The DFARS acquisition rule anticipated in 2025 integrates it into contract eligibility.
The Federal Acquisition Regulation (FAR) CUI rule, currently in proposed rulemaking,
extends this to all federal contractors. The National Security Agency (NSA), Cybersecurity and Infrastructure Security
Agency (CISA), and the White House Office of the National Cyber Director declared
memory-safe languages a national priority — explicitly deprecating C and C++ for new
security-critical development.

## What UMRS Actually Is

In November 2025, I found myself suddenly with more free time than an old engineer knows what to do
with. Years spent in isolated, tightly controlled high-assurance development environments will do
that to you — the moment the constraints lift, you start looking for new ones to build.

I realized the CUI world needed exactly the kind of discipline and hard-won experience that people
like me had spent careers accumulating in the classified world — and almost nobody was applying it
to unclassified sensitive data. So in December 2025, I built the Unclassified MLS Reference System —
UMRS. Originally just to scratch an itch: could I get CUI markings visible in a Multi-Level
Security / Multi-Category Security (MLS/MCS) system so operators could at least see how a
file should be treated? It worked. Then it grew, quickly.

The question: *can orthogonal, category-based access control work inside a system designed
for layered, hierarchical security?*

Classified systems are vertical — Confidential, Secret, Top Secret. CUI is *horizontal*.
Law Enforcement Investigative (LEI/INV) is not "more sensitive" than Agriculture Ammonium
Nitrate (AGR/AMNT). Different rooms, different locks.

Categories compose. An investigator on a case involving agricultural chemicals in an
improvised explosive device holds LEI/INV, AGR/AMNT, and Controlled Technical Information
(CTI) — access to the exact intersection, not an undifferentiated pool.

The kernel enforces this using set theory — similar in structure to how DAC groups work, but
non-discretionary and far more rigorous. Each file is the object being accessed and carries
its own `CategorySet`. Each subject carries a `CategorySet` representing their authorization.
The kernel checks membership: if the subject's set does not satisfy the file's requirements,
access is denied. No negotiation. The key distinction from DAC groups: the resource owner
and system operator cannot change these labels. That is what mandatory means — the policy
authority sets it, and no one below that authority can override it.

Under MLS policy that enforcement is absolute. Under current targeted policy, unconfined
process domains (`unconfined_*`) can bypass category enforcement — the labeling and access
mechanics work correctly, but the enforcement teeth come fully into play with Phase 2 MLS
policy, when the escape hatch closes.

### The Catalog Behind the Claim

Most systems treat CUI as a single label — "handle appropriately." That is nearly as useless
as FOUO. UMRS carries a Rust-parsed catalog of 72 marking entries across 23 category groups
and 48 subcategories. Each entry carries three layers: regulatory origin, verbatim handling
requirements from the actual regulation, and the consequences of getting it wrong. LEI/INV:
28 CFR Part 23 governs it, mishandling exposes an active investigation. CTI: EAR/ITAR applies,
criminal exposure is real. AGR: specific safety framework and documentation required before
any sharing. A traditional MAC system marks a file and enforces access. UMRS does that — and
gives operators an onsite security officer telling them exactly how to handle each resource.
Policy vague enough to survive anything is too vague to enforce anything.

### The Kernel Is Always Right

The stack queries the kernel directly — not configuration files. Your
`/etc/selinux/config` saying `SELINUX=enforcing` does not mean SELinux is enforcing.
If `/sys/fs/selinux/enforce` reads 0, you are running permissive.
Configuration files describe intent. The kernel enforces reality.

The trust flows through a deliberate pipeline: `umrs-hw` interrogates the hardware —
CPU extensions, memory protection, firmware posture. `umrs-platform` builds evidence-based
system verification on top of that. `umrs-selinux` provides a strongly typed, read-only
view of SELinux truth derived from kernel state. Operator tools sit at the top, consuming a
stack where every layer carries evidence of its own correctness. Two-Path Independence (TPI)
means two independent parsers must agree on every security context or the operation fails
closed — disagreement is treated as an attack, not an ambiguity. The demonstration tools
being built on this stack are building blocks: components a CUI operator can use to answer
the question "what does the kernel actually know right now?"

Every crate root that touches security-relevant data carries `#![forbid(unsafe_code)]`.
The workspace contains one designated unsafe isolation boundary: `umrs-hw`, which wraps
a single RDTSCP inline assembly instruction for hardware timestamps. That block is confined,
documented, and separated from all security decision paths. For non-Rust readers:
Rust has an `unsafe` keyword that bypasses memory safety guarantees. `#![forbid(unsafe_code)]`
tells the compiler to refuse compilation if any unsafe operation appears. It cannot be
overridden. An auditor opens one file per crate — if the directive is there, the compiler
has already proven no unsafe code exists. Mechanical proof, with one bounded exception
that proves the rule.

## Phase 1: The Sign Is Actually Good

Phase 1 loads CUI category labels into the Multi-Category Security (MCS) translation layer
under targeted SELinux policy. Operators see the specific category, subcategory, and verbatim
regulatory language — not a generic "CUI" stamp. A file carrying LEI/INV and AGR/AMNT is
labeled as exactly that. The categories compose. The concept is proven.

Most organizations have no operator-facing view of what handling requirements apply to their
files. Phase 1 solves that. But it is still a sign — targeted policy has escape hatches
Bell-LaPadula enforcement does not.

Phase 2 is the lock. MLS policy replaces targeted policy. The SELinux reference monitor
enforces CUI boundaries at the kernel level. dm-crypt encrypts CUI data at rest. Integrity
Measurement Architecture (IMA) verifies CUI-handling binaries before execution. We are
proving concepts now.

I will not tell you Phase 2 is done when it is not. Fifty years of engineering honesty
starts here.

## The Lineage That Justifies the Confidence

Anderson 1972. Bell-LaPadula 1973. Orange Book. SELinux. RHEL 4 first commercial SELinux in
2005. UMRS applies fifty years of proven ideas to the current, legally mandated problem of
CUI protection on unclassified Linux — memory-safe, no unsafe code, no Foreign Function
Interface (FFI) shortcuts, every decision traceable to a requirement.

There is a reason Ada is still running flight control systems forty years after engineers
who have never used it started dismissing it. Languages designed for high assurance are not
popular because they are not forgiving. Anyone who has fought an Ada compiler at 2 AM over
a range constraint they knew was fine understands the temptation to throw the whole paradigm
out the window. But the compiler was right. It is almost always right. That is the point —
and that is the part the critics never stayed long enough to learn. Rust inherited that
philosophy. `#![forbid(unsafe_code)]` is the same demand Ada made in 1983: show me the
proof, or the compiler says no. Rust just made the proving less painful to write.

## What You Can Do Right Now

UMRS is open source. Phase 1 is on GitHub.

- `components/platforms/rhel10/CUI-LABELS.json` — 72 entries, 23 category groups,
  48 subcategories, verbatim handling restrictions. Browse it and ask yourself how many
  of those categories live on your systems with no label at all.
- The `umrs-selinux` crate — query kernel runtime state, detect drift between configured
  and effective SELinux mode, surface enforcement gaps as typed findings.
- The Multi-Category Security label integration showing orthogonal composition working in
  practice.
- The high-assurance pattern library — Two-Path Independence, Fail-Closed,
  Non-Bypassability, and the NIST SP 800-53 Rev 5 controls they satisfy.

If you have been living with compliance theater and want to understand what kernel-enforcement
looks like: this project is built to show you.

The full NIST SP 800-171 Rev 3 control mapping is at
`refs/reports/umrs-capabilities-800-171r3-mapping.md`.

The lock is under construction. The blueprints are open.

## References

1. Anderson, J.P. (1972). *Computer Security Technology Planning Study*.
   Electronic Systems Division, Air Force Systems Command.
2. Bell, D.E., and LaPadula, L.J. (1973). *Secure Computer Systems: Mathematical
   Foundations*. MITRE Corporation, MTR-2547.
3. Department of Defense. (1983). *Trusted Computer System Evaluation Criteria*
   (DoD 5200.28-STD). The Orange Book.
4. Executive Order 13556 — Controlled Unclassified Information. (2010, November 4).
   Federal Register, 75 FR 68675.
5. NIST Special Publication 800-171 Rev 3. (2024). *Protecting Controlled
   Unclassified Information in Nonfederal Systems and Organizations*. National
   Institute of Standards and Technology.
6. NIST Special Publication 800-53 Rev 5. (2020). *Security and Privacy Controls
   for Information Systems and Organizations*. National Institute of Standards
   and Technology.
7. Department of Defense. (2024). *Cybersecurity Maturity Model Certification
   (CMMC) 2.0 Final Rule*. 32 CFR Part 170.
8. Defense Federal Acquisition Regulation Supplement (DFARS) 252.204-7012.
   *Safeguarding Covered Defense Information and Cyber Incident Reporting*.
9. Five Eyes program authorities: UK Cabinet Office Government Security
   Classification Policy (GSCP, 2018); Government of Canada, Treasury Board
   Directive on Security Management (Protected A/B/C levels); Australian Government
   Protective Security Policy Framework (PSPF, 2018); New Zealand Protective
   Security Requirements (PSR).
10. Allied nation frameworks: France, Secrétariat général de la défense et de la
    sécurité nationale (SGDSN), Instruction générale interministérielle n° 1300
    (IGI 1300).
11. NSA/CISA. (2023). *The Case for Memory Safe Roadmaps*. Cybersecurity
    Information Sheet.

---

*UMRS — Unclassified MLS Reference System. Open source, RHEL 10, Rust, no
unsafe code. Phase 1 complete. Phase 2 in progress.*

*GitHub: https://github.com/jamieadams-nerd/umrs-project*
*YouTube: follow for deep-dives on SELinux MLS, CUI engineering, and
high-assurance Rust patterns*

---

*About the authors: Jamie Adams is a 35-year veteran of high-assurance
security engineering and the architect of UMRS. Sage is his AI
collaborator — Claude lineage, Anthropic-built, shaped by a
provenance-tracked corpus of security standards and the standing order
to never bullshit the public.*

*Feedback: jamie_l_adams@icloud.com*
