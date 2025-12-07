A high-assurance system is a computer system that must not merely work correctly, but must be proven, verified, and demonstrably trustworthy under the most demanding security or safety conditions.


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
