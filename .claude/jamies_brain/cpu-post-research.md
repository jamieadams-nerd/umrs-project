 # Research Task: CPU Security Feature Matrix for Audit Cards

 ## Purpose
 - Build a structured CPU feature matrix for future CPU Audit Cards.
 - Focus on CPU features that affect:
   - high assurance posture
   - cryptography
   - trusted execution
   - confidential computing
   - virtualization isolation
   - entropy quality
   - memory protection
   - secure performance acceleration
 - The output must be usable in both:
   - RAG knowledge ingestion
   - regular static reference documents
 - The output must be organized so a future TUI audit card can summarize CPU posture quickly.

 —

 ## Primary Objective
 - Create a matrix of CPU features grouped by security function.
 - For each feature, determine:
   - what it is
   - why it matters
   - whether it improves assurance
   - whether it improves performance
   - whether it creates new attack surface
   - how Linux exposes it
   - how firmware or virtualization may mask it

 —

 ## Required Output Shape
 - Produce one row per CPU feature.
 - Each row must contain:
   - feature name
   - vendor
   - category
   - CPUID leaf/subleaf if applicable
   - Linux-visible flag or detection path
   - instruction examples if applicable
   - security relevance
   - performance relevance
   - known caveats
   - virtualization notes
   - firmware / BIOS dependency notes
   - confidence level of research
   - authoritative references

 —

 ## Output Categories
 - Organize the matrix into these categories:
   1. Symmetric cryptography acceleration
   2. Hash and authentication acceleration
   3. Big integer / public key acceleration
   4. Entropy and random generation
   5. Trusted execution and enclaves
   6. Confidential computing and encrypted virtualization
   7. Memory encryption and integrity protection
   8. Vector acceleration relevant to crypto
   9. Bit-manipulation and arithmetic helpers
   10. Platform / attestation / key protection features
   11. Reliability / availability features relevant to assurance
   12. Virtualization capability features with security impact

 —

 ## Core Feature List To Research First
 - Start with this list.
 - Expand beyond it.

 ### 1. Symmetric cryptography acceleration
 - AES-NI
 - VAES

 ### 2. Hash and authentication acceleration
 - SHA extensions
 - PCLMULQDQ / CLMUL

 ### 3. Big integer / public key helpers
 - ADX
 - BMI1
 - BMI2

 ### 4. Entropy and random generation
 - RDRAND
 - RDSEED

 ### 5. Vector acceleration relevant to crypto
 - SSE
 - SSE2
 - SSE3
 - SSSE3
 - SSE4.1
 - SSE4.2
 - AVX
 - AVX2
 - AVX-512

 ### 6. Trusted execution
 - Intel SGX

 ### 7. Confidential computing
 - AMD SEV
 - AMD SEV-ES
 - AMD SEV-SNP
 - Intel TDX

 ### 8. Memory encryption
 - AMD SME

 ### 9. Key protection
 - Intel Key Locker

 ### 10. Virtualization-security related capabilities
 - VMX
 - SVM
 - nested paging related capabilities
 - IOMMU / DMA-isolation related CPU-adjacent capabilities

 ### 11. Reliability / availability / resilience features
 - machine check architecture features
 - RAS-related features
 - ECC-related platform interactions where CPU documentation covers them
 - error containment / reporting features that affect trustworthy operation

 —

 ## Feature Interpretation Rules
 - Do not treat every CPU extension as automatically “good.”
 - For each feature ask:
   1. Does it improve performance only?
   2. Does it improve security only?
   3. Does it improve both?
   4. Does it introduce new trust assumptions?
   5. Does it depend on firmware, microcode, kernel, or hypervisor cooperation?
   6. Can it be disabled, hidden, or virtualized away?

 —

 ## Linux Detection Research Requirements
 - Research how each feature is detected from Linux.
 - Include:
   - CPUID instruction path
   - /proc/cpuinfo flags when applicable
   - kernel-specific exposure rules
   - sysfs or other kernel interfaces when applicable
   - hypervisor masking behavior
 - Do not assume /proc/cpuinfo is authoritative by itself.
 - Record when Linux kernel docs explicitly warn that /proc/cpuinfo is only marginally useful or kernel-filtered. The kernel documentation says /proc/cpuinfo flags are mainly for kernel debugging and that applications should prefer more direct CPU-query mechanisms such as kcpuid or cpuid(1).  

 —

 ## Audit Card Relevance Rules
 - For each feature, assign an audit-card relevance class:
   - Critical
   - Important
   - Informational
 - Critical means absence or disablement materially changes security posture.
 - Important means it materially changes performance, hardening, or isolation.
 - Informational means it is useful context but not decisive.

 —

 ## CPU Audit Card View Model
 - The future audit card should be able to show:
   - vendor and model
   - microarchitecture family
   - available crypto acceleration features
   - available confidential-computing features
   - available entropy features
   - virtualization isolation features
   - evidence chain used to validate each conclusion
 - Therefore research output must support both:
   - summary presentation
   - drill-down evidence view

 —

 ## Suggested Matrix Columns
 - Use these columns in the master table:
   1. Feature
   2. Vendor
   3. Category
   4. Purpose
   5. Example instructions
   6. CPUID leaf/subleaf/bit
   7. Linux flag or detection mechanism
   8. Minimum CPU generations of interest
   9. Security benefit
   10. Performance benefit
   11. Assurance caveats
   12. Virtualization behavior
   13. Firmware / BIOS dependency
   14. Microcode dependency
   15. Audit-card relevance
   16. Notes
   17. Sources

 —

 ## Mandatory Research Sections Per Feature
 - For each feature write a short structured note containing:
   - Description
   - Why security engineers care
   - Why performance engineers care
   - Trust model
   - Linux visibility
   - Hypervisor visibility
   - Known problems or controversies
   - Recommendation for audit-card display

 —

 ## Initial Prioritization
 - Prioritize these first because they have immediate CPU-audit-card value:
   1. AES-NI
   2. PCLMULQDQ
   3. SHA extensions
   4. RDRAND
   5. RDSEED
   6. SGX
   7. SEV
   8. SEV-ES
   9. SEV-SNP
   10. TDX
   11. SME
   12. Key Locker
   13. AVX2
   14. AVX-512
   15. ADX

 —

 ## Important Interpretation Notes
 - AES-NI and related instructions are important because they accelerate AES rounds in hardware and are directly relevant to secure high-throughput crypto implementations.  
 - AMD SEV uses per-VM memory-encryption keys managed by the AMD Secure Processor and is central to confidential-VM posture analysis.  
 - Intel SGX is enclave-based trusted execution with attestation significance and should be treated separately from VM-based confidential-computing features.  
 - Intel TDX protects confidential guest VMs by isolating register state and encrypting guest memory; it belongs in the VM-isolation branch of the matrix, not the enclave branch.  
 - Linux has dedicated documentation for SGX, TDX, AMD memory encryption, and a threat-model view for SNP/TDX confidential computing; ingest all of those for kernel-side semantics.  

 —

 ## Authoritative Source Pack
 - Ingest these official source families first.

 ### Intel primary sources
 - Intel SDM landing page. Primary source for architecture, instruction semantics, CPUID, and system programming.  
 - Intel combined SDM volume PDF. Use when a single searchable document is more efficient.  
 - Intel SGX official overview and linked developer material.  

 ### AMD primary sources
 - AMD64 Architecture Programmer’s Manual family. Use volume references for ISA and system-programming details. The current AMD documentation still identifies volumes 1 through 5, with volume 3 covering general-purpose and system instructions and volume 2 covering system programming.  
 - AMD SEV official developer page.  

 ### Linux primary sources
 - Linux x86 feature flags documentation. Required for understanding how kernel-exposed flags differ from raw hardware support.  
 - Linux SGX documentation.  
 - Linux TDX documentation.  
 - Linux AMD memory encryption documentation.  
 - Linux confidential-computing threat-model documentation for SNP/TDX.  

 —

 ## Secondary Source Pack
 - Use secondary sources only to enrich terminology, history, or cross-checks.
 - Primary claims must come from vendor manuals or kernel documentation.
 - Academic papers are required for:
   - side-channel concerns
   - SGX attack history
   - SEV / SNP limitations
   - TDX threat assumptions
 - Separate “vendor claim” from “research community criticism.”

 —

 ## Data Quality Rules
 - Prefer vendor manuals and kernel docs over blogs.
 - Prefer architecture manuals over marketing pages.
 - Record exact CPUID locations whenever available.
 - Record when a feature requires:
   - BIOS enablement
   - microcode update
   - kernel configuration
   - hypervisor support
 - Record when guest-visible support can differ from host support.

 —

 ## RAG Preparation Rules
 - Chunk documents by feature and by vendor manual section.
 - Preserve table structures when possible.
 - Store aliases for every feature.
 - Example aliases:
   - AES-NI / AESNI / AES instructions
   - PCLMULQDQ / CLMUL
   - SEV-SNP / SNP
   - Intel TDX / TDX
 - Keep one normalized canonical name.

 —

 ## Deliverables
 - Deliverable 1: master CPU feature matrix
 - Deliverable 2: per-feature research notes
 - Deliverable 3: Linux detection reference sheet
 - Deliverable 4: audit-card summary recommendations
 - Deliverable 5: evidence-source map showing where each conclusion is derived

 —

 ## Final Objective
 - Build a corpus that allows our system to answer:
   - Which CPU extensions materially affect security posture?
   - Which CPU extensions materially affect cryptographic performance?
   - Which CPU features indicate trusted execution or confidential-computing capability?
   - Which features are present in hardware but hidden by firmware, kernel, or hypervisor?
 - The result must support a future CPU Audit Card that conveys CPU security posture at a glance.



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


⸻

This feedback loop design is actually very strong architecturally for agent teams. You’ve essentially built a three-phase knowledge pipeline:
	1.	Researcher → builds corpus
	2.	Security + Developer → ingest + critique
	3.	Researcher → improves corpus

That creates a self-improving knowledge base, which is exactly what you want for something deep like CPU architecture knowledge.

If you’d like, I can also show you one additional step that dramatically improves agent awareness of large corpuses (a trick used in some internal AI dev environments) — it prevents the “agents forget the corpus exists” problem almost completely.

What you are running into is a very common problem in multi-agent systems:

Agents cannot query a resource if they do not know the vocabulary of the domain.

If an agent has zero conceptual map, it will never ask the right RAG query.

The solution used in several internal AI tooling environments is a “Knowledge Index” or “Topic Map” layer placed above the corpus.

Think of it as a table of contents for the agent brain.

Instead of asking the agent to read thousands of pages, you give it a small mental map of the territory.

⸻

The Technique: Agent Knowledge Index

You create a small document that contains:
	•	topic categories
	•	keywords
	•	resource locations
	•	example questions

The agent always reads this first.

Then when it later needs information it already knows:

“Oh, CPU extensions live in that corpus.”

⸻

Why This Works

LLMs are extremely good at:
	•	expanding keywords
	•	forming queries
	•	connecting concepts

They are not good at discovering unknown domains.

So we give them the domain map.

⸻

Example Knowledge Index Entry (For Your CPU Work)

This would live in something like:

agent-knowledge-index.md

or

.dev-intelligence/topic-index.md


⸻

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
