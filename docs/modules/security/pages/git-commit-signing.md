# Signing Commits for GIT

Digitally signing Git commits aligns with multiple NIST Special Publications, especially those concerned with software integrity, supply-chain security, configuration management, and provenance.


# Security Control Mappings
The following provides a mapping to security controls so you understand what this sinmple step can satisify. Specifically, the NIST Special Publications that GIT Signing directly supports.


### NIST SP 800-53
* SI-7: Software, Firmware, and Information Integrity
  - Git commit signing directly supports this control because it ensures:
	- The source code has not been altered without detection
	- The origin of commits can be authenticated
	- Unauthorized modifications can be detected early

* SI-7(1): Integrity Checks — Cryptographic
  - This is one of the strongest mappings. This enhancement explicitly calls for cryptographic integrity mechanisms, which includes:
	- digital signatures
	- signed metadata
	- tamper-evident records

* CM-5(3): Access Restrictions for Change — Signed Components
  - Signing Git commits is exactly this. This enhancement requires cryptographically verifying changes before they are accepted as legitimate.
  - A software development team that requires commit signing and rejects unsigned commits is implementing CM-5(3).

* CM-3 & CM-6 (Configuration Change Control / Baseline Integrity)
  - Git commit signing helps ensure:
	- only authorized individuals submit code
	- changes are traceable and attributable
	- malicious or mistaken alterations cannot slip in unnoticed


### NIST SP 800-171 (Protecting CUI in Non-Federal Systems)
* 3.4.8 — Prevent unauthorized modification of software
  - Signing commits ensures only authenticated developers can push changes.

* 3.13.8 — Verify integrity of software before installation or execution
  - Software must have known-good provenance.
  - Signed commits provide verifiable provenance at the development stage.


### NIST SP 800-172 (Enhanced Requirements for High-Value Assets)
This use case fits 800-172 better than typical commercial workflows. This is where things get even stronger.

* 800-172 emphasizes:
  - cryptographically authenticated development workflows
  - strong provenance assurance
  - tamper-evident build pipelines

* Signed Git commits directly satisfy:
  - “Integrity protection of code throughout the development lifecycle.”
  - “Verification of the identity of code contributors.”

### NIST SP 800-160 Vol. 1 (Systems Security Engineering)
This is foundational to high-assurance engineering.

Commit signing supports the engineering principles:

* evidence-based trustworthiness
* provenance of components
* secure development practices
* traceability and accountability


### NIST SP 800-161 (Supply Chain Risk Management)
This SP focuses on trusted supply chains. This is also a very clean match for signing audit and log files, too.

Digitally signing Git commits satisfies portions of:

* ensuring authenticity of source code
* providing traceability of contributions
* detecting unauthorized insertion of malicious code
* assurance of development artifacts used downstream


###  NIST SP 800-193 (Firmware/Platform Integrity)
Commit signing is a natural extension of that integrity chain upstream in the software lifecycle.

While focused on firmware, it establishes the broader pattern:

* software artifacts must be signed
* only authenticated updates are trusted
* cryptographic provenance is required

## SUMMARY: WHAT NIST EXPECTS & HOW GIT SIGNING MEETS IT

NIST does not require “*sign Git commits specifically*,” but NIST consistently requires:

* cryptographic integrity mechanisms
* authenticity of developers/contributors
* protection against unauthorized modification
* traceability of changes
* trusted supply chain
* verification of provenance

Digitally signing Git commits provides all of these in a strong, easily auditable way.


## A PERFECT SENTENCE FOR DOCUMENTATION

> “Digitally signing Git commits provides cryptographic assurance of the authenticity, integrity, and provenance of source code, directly supporting NIST 800-53 SI-7, SI-7(1), CM-5(3), and related requirements in NIST 800-171, 800-172, 800-161, and 800-160 for trusted software development and supply-chain integrity.”
