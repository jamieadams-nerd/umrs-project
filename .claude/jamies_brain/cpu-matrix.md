 # Post-Research Review Procedure: CPU Extension Knowledge Corpus

 ## Purpose
 - Ensure the research corpus produced for CPU extensions is:
   - understood
   - discoverable
   - actionable
   - reusable by all agents.

 - Prevent a common failure mode where agents ignore resources because they do not know they exist.

 - Pre-seed agent knowledge bases so they know:
   - what CPU extensions are relevant
   - what the research corpus contains
   - when to consult it.

 ---

 # Agents Involved

 This procedure is executed by:

 - Security Engineer Agent
 - Rust Developer Agent

 The Researcher Agent is the author of the corpus but must also receive feedback.

 ---

 # Stage 1: Corpus Awareness

 Each reviewing agent must confirm the existence of the new research corpus.

 Required awareness items:

 - location of the corpus
 - structure of the corpus
 - major topic areas
 - document naming conventions

 Each reviewing agent must record in its internal knowledge base:

 - the name of the corpus
 - where it is stored
 - what questions it can answer

 ---

 # Stage 2: Knowledge Pre-Seeding

 Each reviewing agent must extract a **minimal knowledge summary** from the corpus.

 Purpose:

 - prevent the agent from having zero knowledge of CPU extensions
 - ensure the agent knows which topics exist and can query the corpus later

 The summary must contain:

 - categories of CPU extensions
 - examples of extensions
 - why they matter to our system

 ---

 # Required Knowledge Categories

 Each agent must record at minimum the following categories:

 - cryptographic acceleration extensions
 - vector extensions used by cryptography
 - hardware entropy sources
 - trusted execution technologies
 - confidential computing extensions
 - memory encryption capabilities
 - virtualization security features

 ---

 # Example Knowledge Entry

 Each agent should create an internal entry similar to the following:

 Topic:
 CPU security-relevant extensions

 Categories include:

 - AES-NI and related crypto acceleration
 - SHA hardware acceleration
 - CLMUL / PCLMULQDQ
 - RDRAND / RDSEED entropy instructions
 - AVX / AVX2 / AVX-512 vector crypto acceleration
 - Intel SGX enclave execution
 - AMD SEV / SEV-SNP encrypted virtualization
 - Intel TDX confidential VM support
 - AMD SME memory encryption

 Knowledge location:

 CPU extension research corpus

 ---

 # Stage 3: Corpus Evaluation

 After reading the corpus, each reviewing agent must evaluate:

 - completeness
 - clarity
 - technical accuracy
 - usability for automation

 Agents should ask the following questions:

 1. Can this data support CPU audit cards?
 2. Does the corpus explain how features are detected?
 3. Are CPUID flags documented?
 4. Are Linux detection methods documented?
 5. Are virtualization masking behaviors described?
 6. Are security implications explained?

 ---

 # Stage 4: Gap Analysis

 Each reviewing agent must identify:

 - missing extensions
 - missing documentation
 - unclear explanations
 - missing detection techniques

 Possible gap categories include:

 - CPUID flag mapping
 - microarchitecture differences
 - virtualization masking behavior
 - firmware dependency
 - Linux kernel exposure

 ---

 # Stage 5: Resource Evaluation

 Each reviewing agent must determine whether the corpus includes sufficient authoritative sources.

 Required source types include:

 - Intel architecture manuals
 - AMD architecture manuals
 - Linux kernel documentation
 - confidential computing documentation

 If important sources are missing, the agent must request them.

 ---

 # Stage 6: Feedback to Researcher Agent

 Each reviewing agent must produce structured feedback containing:

 - missing CPU extensions
 - missing categories
 - missing documentation
 - missing detection methods
 - missing vendor documentation

 Feedback must be specific and actionable.

 Example:

 - Add CPUID bit mapping for AES-NI
 - Add detection method via CPUID instruction
 - Add Linux kernel feature flag mapping

 ---

 # Stage 7: Implementation Relevance

 The Rust Developer Agent must also evaluate:

 - which CPU features should be visible in a CPU audit card
 - which features should be summarized in headers
 - which features require deeper evidence tracking

 The Rust Developer must propose:

 - CPU audit card layout ideas
 - detection methods suitable for Rust
 - data structures required to represent CPU capability state

 ---

 # Stage 8: Security Review

 The Security Engineer Agent must evaluate:

 - which CPU features materially affect security posture
 - which features must appear in security audit outputs
 - which features are optional informational signals

 The security engineer should classify each feature as:

 - critical
 - important

 ---

 # Stage 9: Final Output

 Each reviewing agent must produce:

 1. Knowledge base summary entry
 2. Corpus evaluation notes
 3. Gap analysis report
 4. Structured feedback for the researcher agent

 ---

 # Final Objective

 Ensure that:

 - the research corpus becomes a widely used resource
 - agents know it exists
 - agents understand what problems it solves
 - the corpus improves through iterative feedback.



 # Knowledge Index Entry: CPU Security Extensions

 ## Topic
 CPU instruction set extensions related to cryptography and high assurance systems

 ## Purpose
 This corpus explains CPU capabilities that affect:
 - cryptographic performance
 - trusted execution
 - confidential computing
 - entropy generation
 - virtualization security

 ## Resource Location
 CPU extension research corpus

 ## Topics Covered
 - AES-NI cryptographic acceleration
 - SHA hardware hashing instructions
 - CLMUL carryless multiplication
 - RDRAND hardware random number generator
 - RDSEED entropy source
 - AVX / AVX2 / AVX512 vector cryptography acceleration
 - Intel SGX enclave execution
 - AMD SEV encrypted virtualization
 - AMD SEV-SNP confidential VMs
 - Intel TDX trusted domain execution
 - AMD SME memory encryption
 - Intel Key Locker key protection

 ## Example Questions This Corpus Can Answer

 - Which CPU extensions accelerate TLS encryption?
 - Which CPU features improve cryptographic performance?
 - Which CPU extensions enable confidential computing?
 - How can Linux detect AES-NI support?
 - Which CPU features materially affect system security posture?

 ## When To Consult This Resource

 Consult this corpus when working on:

 - CPU audit cards
 - cryptography performance analysis
 - security posture evaluation
 - confidential computing features
 - entropy generation analysis

 ## Related Audit Cards

 - CPU audit card
 - cryptographic capability audit
 - virtualization security audit
