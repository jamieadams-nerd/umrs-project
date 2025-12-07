A high-assurance system is a computer system that must not merely work correctly, but must be proven, verified, and demonstrably trustworthy under the most demanding security or safety conditions.

High-assurance engineering has its roots in High Assurance Computing and Management Systems ([HACAMS](HACAMS.md)). In the 1990s and early 2000s, HACAMS was often used in academic papers, DARPA programs, and DoD system descriptions. Over time, the terminology shifted.

Today, you rarely hear “HACAMS” used explicitly. Instead, the same ideas live on under different names, such as:
* High-assurance systems
* Trusted systems
* MLS (multi-level security) systems
* Cross-domain systems (CDS)
* Safety-critical and mission-critical systems
* Raise-the-Bar or similar assurance initiatives

For more background, check my [HACAMS page](HACAMS.md).

## HIGH-ASSURANCE ENGINEERING
High-assurance engineering is the practice of building systems that must be proven trustworthy, not simply “designed well.”

It applies to systems where failure is unacceptable because it could cause:
- loss of life
- national-security compromise
- mission failure
- catastrophic financial loss
- classified data leakage
- critical infrastructure disruption

In high-assurance work, the goal is not “works correctly most of the time,” but demonstrable correctness, verifiable security, and predictable behavior under all conditions—even adversarial ones.

In simple terms:
- Traditional engineering = “We think it works.”
- High-assurance engineering = “We can prove it works, and prove it fails safely.”

### THE KEY DIFFERENCE

#### 1. Level of required evidence

Traditional systems rely on:
• unit tests
• integration tests
• spot checks
• general best practices

High-assurance systems require formal proofs, mathematical models, auditable processes, and verification evidence.

#### 2. Impact of failure

Traditional:
Failure means downtime, inconvenience, bugs.

High-assurance:
Failure may mean a warfighter dies, intelligence is compromised, attackers cross domains, critical national data is exposed, or a weapon system misfires.

3. Trust boundary rigor

Traditional:
TCB (trusted computing base) isn’t carefully minimized; components grow organically.

High-assurance:
Every trusted component must be:
• minimal
• auditable
• inspected
• verified
• controlled through change management

4. Development discipline

Traditional:
Agile, quick iteration, “move fast,” flexible.

High-assurance:
• strict coding standards
• formal peer reviews
• static analysis (Coverity, etc.)
• configuration control
• long-cycle testing
• threat modeling
• documentation requirements
• security models (MLS, RBAC, etc.)
• reproducible builds
• mandatory hardening (FIPS, MAC, etc.)


## HIGH-ASSURANCE SYSTEM (CORE CONCEPT)

A high-assurance system is one where:
* Correctness,
* Security, and
* Policy enforcement

These are not assumed, but instead must be formally shown, rigorously tested, and verified through structured evidence, such as:
* Formal proofs
* Model checking
* Code auditing
* Controlled development processes
* Trusted build environments
* Independent evaluation
* Continuous verification during the lifecycle

In other words: A high-assurance system provides mathematical or process-based evidence that it behaves correctly, securely, and predictably, even under attack.


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



## REAL-WORLD EXAMPLES
A system might be considered high assurance if it:
* Enforces MLS separation on RHEL with SELinux MLS
* Uses formally verified microkernels (e.g., seL4)
* Implements deterministic control in an aircraft flight computer
* Uses FIPS 140-3 validated cryptographic modules
* asses Common Criteria EAL5+ or higher


## WHAT HIGH-ASSURANCE IS NOT

It is **not** just:
* “Secure coding”
* Memory-safe language choice (e.g., Rust)
* SELinux enforcing mode
* Firewalls
* Encryption
* Unit tests

Those are security measures, not assurance.

**Assurance = evidence + process + verification + proven behavior.**


## THE FORMAL SUMMARY
A high-assurance system is a system that:
* has a mathematically defined security policy,
* is engineered under strict, auditable processes,
* has a minimized and verified trusted computing base,
* enforces MAC/MLS or similar non-bypassable controls,
* undergoes independent evaluation or formal analysis,
* and provides strong assurance — not just hope — that it will behave securely.
