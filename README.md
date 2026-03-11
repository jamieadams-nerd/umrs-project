<img src="docs/images/umrs-logo-1024px.png" align="right" width=200>
<p>A high-assurance system is a computer system that must not merely work correctly, but must be proven, verified, and demonstrably trustworthy under the most demanding security or safety conditions.</p>

<p>High-assurance engineering has its roots in High Assurance Computing and Management Systems (HACAMS). In the 1990s and early 2000s, HACAMS was often used in academic papers, DARPA programs, and DoD system descriptions. Over time, the terminology shifted.</p>

Today, you rarely hear *HACAMS* used explicitly. Instead, the same ideas live on under different names, such as:
* High-assurance systems
* Trusted systems
* MLS (multi-level security) systems
* Cross-domain systems (CDS)
* Safety-critical and mission-critical systems
* Raise-the-Bar or similar assurance initiatives
* ...For more background, check my [HACAMS page](docs/HACAMS.md).


<br><ins>Real World Examples of High-Assurance Systems:</ins>

* Military / Intelligence
  - Cross-Domain Solutions (CDS) / Guards: Must meet NSA “Raise the Bar,” NCDSMO standards, formal MLS policies, and cannot fail open under any circumstance.
  - SIGINT collection platforms: Systems must behave deterministically and securely on submarines, aircraft, or ground stations.
  - Weapons-control computers: Missile-defense and targeting systems require mathematically proven correctness and safety.

* National Infrastructure
  - Nuclear command and control: Sub-1% chance of failure is unacceptable; must be nearly mathematically impossible.
  - Air-traffic control radars and coordination systems: Require absolute integrity and predictable real-time behavior.

* Formal Safety-Critical Industries
  - Aviation flight computers: Governed by DO-178C Level A: every line of code must be traceable, testable, verified.
  - Medical devices (pacemakers, infusion pumps): Must be provably safe; any crash can kill.
  - Industrial safety systems (oil refineries, power plants): Loss of control could cause large-scale disasters.

* High-Integrity Cryptography & Security
  - FIPS 140-3 validated crypto modules: Algorithms and implementations must be proven secure under strict constraints.
  - Formally verified microkernels (e.g., seL4): Entire correctness is mathematically proven.

---

# HIGH-ASSURANCE SYSTEM (CORE CONCEPT)

A high-assurance system is one where: Correctness, Security, and Policy enforcement are not assumed. Instead must be formally shown, rigorously tested, and verified through structured evidence, such as:
* Formal proofs
* Model checking
* Code auditing
* Controlled development processes
* Trusted build environments
* Independent evaluation
* Continuous verification during the lifecycle

>[!Note]
> A high-assurance system provides mathematical or process-based evidence that it behaves correctly, securely, and predictably, even under attack.

## HIGH-ASSURANCE SYSTEMS
A high-assurance system is a system that:
* is engineered under strict, auditable processes,
* has a minimized and verified trusted computing base,
* has a mathematically defined security policy,
* enforces MAC/MLS or similar non-bypassable controls,
  - Enforces MLS separation on RHEL with SELinux MLS
* Uses formally verified microkernels (e.g., seL4)
* Implements deterministic control in an aircraft flight computer
* Uses FIPS 140-3 validated cryptographic modules
* undergoes independent evaluation or formal analysis,
* asses Common Criteria EAL5+ or higher
* and provides strong assurance — not just hope — that it will behave securely.

## THE KEY FEATURES OF HIGH-ASSURANCE SYSTEMS
1.	Mandatory Access Control (MAC) and MLS enforcement
    - The policy is not discretionary; users and applications cannot override security decisions.

2.	Verifiable and minimal trusted computing base (TCB)
    - Only a very small part of the system is trusted; everything else is untrusted by design.

3.	Strict development process
    - Coding standards, code reviews, static analysis, secure coding rules, and documentation are mandatory, not optional.

4.	Independent evaluation or certification
    - Examples:
	  - Common Criteria EAL5 – EAL7
      - NSA Raise-the-Bar guidance
      - NCDSMO evaluations
      - FIPS 140-3 validation
      - DO-178C (aviation)
      - IEC 61508 / ISO 26262 (industrial/safety)

5.	Resistance to insider threat, misuse, or misconfiguration
    - The system should remain secure even when users or software try to bypass controls.

6.	Formal security model
    - Typically Bell–LaPadula (confidentiality) or Biba (integrity), or modern variants.

## THE KEY DIFFERENCES
The key differences between High-Assurance systems and traditional systems are summarized in the table below:

|                            | **High-Assurance Systems**                   | **Traditional Systems**
|----------------------------|:---------------------------------------------|:---------------------------------------------|
| Level of required evidence | High-assurance systems require formal proofs, mathematical models, auditable processes, and verification evidence.|Traditional systems rely on: unit tests, integration tests, spot checks, and general best practices
| Impact of failure|Failure may mean a warfighter dies, intelligence is compromised, attackers cross domains, critical national data is exposed, or a weapon system misfires.|Failure means downtime, inconvenience, bugs.
|Trust boundary rigor|Every trusted component must be: minimal, auditable, inspected, verified, and controlled through change management|TCB (trusted computing base) isn’t carefully minimized; components grow organically.
|Development discipline|High-assurance:<ul><li>strict coding standards</li><li>formal peer reviews</li><li>static analysis (Coverity, etc.)</li><li>configuration control</li><li>long-cycle testing</li><li>threat modeling</li><li>documentation requirements</li><li>security models (MLS, RBAC, etc.)</li><li>reproducible builds</li><li>mandatory hardening (FIPS, MAC, etc.)</li></ul>| Agile, quick iteration, “move fast,” flexible.

---

# HIGH-ASSURANCE ENGINEERING
High-assurance engineering is the practice of building systems that must be proven trustworthy, not simply “designed well.”

It applies to systems where failure is unacceptable because it could cause:
- loss of life
- national-security compromise
- mission failure
- catastrophic financial loss
- classified data leakage
- critical infrastructure disruption

>[!Important]
> In high-assurance work, the goal is not *works correctly most of the time,* but demonstrable correctness, verifiable security, and predictable behavior under all conditions—even adversarial ones.
>
> In simple terms:
> - Traditional engineering = “We think it works.”
> - High-assurance engineering = “We can prove it works, and prove it fails safely.”

High-assurance is *not* just:
* “Secure coding”
* Memory-safe language choice (e.g., Rust)
* SELinux enforcing mode
* Firewalls
* Encryption
* Unit tests

Those are security measures, not assurance.
> Assurance = evidence + process + verification + proven behavior.


---
_This document is licensed under the Creative Commons Attribution 4.0 International License (CC BY 4.0).
You may copy, redistribute, and adapt this material, provided that appropriate credit is given to the original author._

Author: Jamie L. Adams<br>
License text: https://creativecommons.org/licenses/by/4.0/

