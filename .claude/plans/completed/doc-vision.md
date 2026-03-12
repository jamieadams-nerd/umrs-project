# Notes for Senior Technical Writer: UMRS Documentation Vision

These notes capture Jamie’s current vision for how the **UMRS documentation ecosystem** should be understood, organized, and evolved. This is not yet the final prescribed structure or final ordering of books/modules. It is a faithful description of the **documentation domains, purposes, audiences, and organizational intent** that should guide future work.

A critical point: the lists of topics under each section are **illustrative, not exhaustive**. In many areas, more content already exists than is named here. The purpose of this note is to define the **groupings and intent**, not to freeze a limited table of contents.

Another critical point: the desired future workflow is not merely “write docs when asked.” The long-term goal is for the documentation system, and the AI agent assisting with it, to become capable of handling requests such as:

> “Here is a new topic. Determine where it best fits in our organizational structure.”

That means the documentation architecture must be coherent enough that new material can be classified, placed, cross-referenced, and maintained without guesswork.

---

## 1. Documentation ecosystem: multiple documentation forms

UMRS does not have only one kind of documentation. It has multiple layers, each serving a different purpose.

### Rust API documentation

This is the `rustdoc` / Rust doc-comment documentation for crates, modules, types, traits, and functions.

Purpose:

- code-level API reference
- developer-facing usage details
- library and module reference
- explanation of public interfaces

Audience:

- software developers
- integrators
- maintainers

### Man pages

These are the traditional CLI manual pages for tools.

Purpose:

- command reference
- options and flags
- examples
- operational invocation guidance

Audience:

- operators
- administrators
- engineers using command-line tooling

### Mallard / Yelp help

This is integrated desktop help for GUI or desktop-oriented tooling.

Purpose:

- contextual help
- shorter, task-oriented help
- embedded or nearby assistance for end users

Important characteristic:

- these help pages should cross-reference back to the main body of documentation when deeper explanation is needed

Audience:

- GUI users
- operators using desktop-facing tools
- users needing integrated help rather than a full manual

### Antora / AsciiDoc books

These are the main long-form books and the authoritative knowledge corpus.

Purpose:

- architecture
- concepts
- deployment
- development
- operations
- rationale
- security explanations
- historical background
- reference material

This is the primary documentation body where the organizational architecture matters most.

---

## 2. Intended audiences

The documentation is meant for people interested in the security, assurance, and architectural aspects of UMRS and related technologies. Likely audiences include:

- security engineers
- software developers
- system administrators
- operators
- security auditors
- organizational managers
- project owners
- technically curious readers
- potential collaborators
- evaluators who may want to reuse ideas, modules, or techniques

Not every document serves every audience equally, but the overall documentation ecosystem should support all of them.

---

## 3. Project introduction / orientation

One major documentation domain is a high-level introduction to the project.

Purpose:

- explain what UMRS is at a high level
- excite the reader
- encourage further reading
- encourage collaboration
- help readers discover tools, modules, ideas, or techniques they may want to use

This material should make the project approachable to someone who is newly curious but technically serious.

It should include:

- a high-level description of UMRS
- what is available to readers
- what parts of the project exist
- why the project matters
- high-level orientation to the problem space

This introduction may also include high-level architecture material, but not deep or overly detailed architecture.

---

## 4. What UMRS is, and what it is not

A related but distinct domain is a more precise introductory explanation of the project’s scope and identity.

This must explain:

- what UMRS is
- what UMRS is not
- what its assurance focus is
- how it relates to multilevel security concepts
- that UMRS may use classified-information handling frameworks or MLS concepts as scaffolding, but the entire system is **not only about MLS**

This distinction matters a great deal. The documentation must avoid misleading readers into thinking UMRS is merely an MLS product or only a classified-systems exercise. The larger emphasis is **high assurance**, disciplined trust, evidence, validation, and strong engineering patterns.

---

## 5. Historical background and technology evolution

Historical context is an important domain, but it is not meant to become detached storytelling. Its role is to support understanding and justify present architectural choices.

This material should explain how we got here:

- the historical evolution of important security ideas
- why certain mechanisms became necessary
- how assurance concerns developed over time
- why modern systems need certain controls and verification approaches

Examples include:

- the emergence of the reference monitor concept
- Anderson / RAND-era thinking and similar foundational work
- the evolution of mandatory access control
- why enforcement and trust boundaries became necessary
- why evidence and verification matter

The purpose of this history is:

- context
- rationale
- justification
- conceptual grounding

It should be sprinkled where appropriate and used as a support mechanism, not necessarily isolated as a giant standalone history book.

---

## 6. Security concepts and assurance principles

Another major documentation domain is a concepts section that grounds readers in the principles the rest of the documentation depends upon.

This section is meant to help readers understand the vocabulary and reasoning that later sections will assume.

Examples of content include:

- assurance concepts
- trust models
- evidence-based verification
- source of truth
- ground truth
- terminology around trust and validation
- provenance
- supply-chain or chain-of-custody style trust thinking
- ideas from Raise-the-Bar / RAIN-type concepts and similar recent assurance discussions
- verification versus assumption

These concepts are intertwined. The key point is that the documentation should establish a conceptual vocabulary for assurance and trust so later sections on development, operations, implementation, and architecture can refer back to these concepts instead of re-explaining them each time.

This section should also connect backward to the historical material where useful, showing how the concepts evolved.

---

## 7. Deployment / getting started

There needs to be a deployment-oriented domain for people who want to evaluate, use, or begin building with UMRS.

Purpose:

- help someone get a suitable environment ready
- help someone begin evaluation
- help someone start development or experimentation

This includes:

- preparing a baseline operating system
- instructions for configuring that operating system
- recommendations about hardware
- recommendations about filesystem or storage layout
- security-control-driven setup considerations

The structure should include:

- generic common steps
- OS-specific subsections

For example, some material may be shared by all platforms, while some guidance differs between Ubuntu and another OS family.

This is about providing a clean path from interest to usable environment.

---

## 8. Baseline OS hardening

Within or near deployment, there is a baseline hardening concern that should be explicitly recognized.

This includes:

- a generic OpenSCAP hardening script
- basic lockdown and security baseline measures
- baseline preparation before moving into higher-assurance enhancements

This is the “secure baseline” stage before more advanced integrity, verification, or high-assurance features are layered on top.

---

## 9. High assurance enhancements

This is a major domain. These are features or controls that are often already available in the operating system or its ecosystem, but that require deliberate configuration and understanding.

Important framing:

- these are not necessarily UMRS inventions
- they are operating system or platform capabilities that UMRS values, documents, and may recommend

Examples include:

- per-user `/tmp` isolation
- IMA/EVM
- AIDE
- kernel lockdown
- kernel security flags and related settings

For each enhancement, documentation should generally explain:

- what it is
- why it matters
- how to configure it
- how to verify it
- what security controls or assurance objectives it helps satisfy
- pros and cons
- tradeoffs
- operational considerations
- when an organization may or may not want to adopt it

The goal is not merely “turn on every feature.” The goal is to help a reader understand the feature well enough to make an informed, assurance-oriented choice.

---

## 10. Cryptography

There is a cryptography domain. It may not be large, but it is dense and important.

This should include high-level but structured cryptographic guidance such as:

- FIPS-approved algorithms
- grouped by type
- symmetric algorithms
- asymmetric algorithms
- digital signatures
- hash functions
- HMAC and related categories

For each entry, the documentation may include structured details such as:

- algorithm or scheme
- cipher where applicable
- mode
- key length or bit size
- ordering from most preferred to less preferred but still acceptable

This section should also include the currently relevant post-quantum cryptographic algorithms, specifically the three currently recognized ones Jamie referenced.

The cryptography material is intended to be precise, referenceable, and useful for engineering and policy alignment.

---

## 11. Development

Development is one of the largest and most important domains in the entire documentation vision.

Its purpose is to help engineers understand how to build software in and around UMRS, using a high-assurance mindset and the libraries and patterns the project has already established.

### Why Rust

The development material should explain why Rust is recommended.

This should not merely be language advocacy. It should tie the recommendation to security, robustness, maintainability, and assurance goals.

### Secure coding guidance

The documentation should identify three recommended secure coding guides at a high level and point developers toward them.

### UMRS libraries and packages

The development section should introduce the crates, packages, and modules already written using high-assurance patterns and strongly encourage new developers on the platform to use them.

Examples already mentioned include:

- `umrs-platform`
- `umrs-selinux`
- `umrs-core`

There are more, and more will come later.

These libraries are not just utilities; they are a selling point of the platform because they embody the project’s assurance patterns and engineering philosophy.

### What the modules provide

The documentation should explain these modules in detail:

- what they provide
- how they provide it
- why they are trustworthy building blocks
- what problems they solve
- how they should be used

This explanation should begin from foundational platform capability and move upward. For example:

- platform or OS detection
- trusted decision-making inputs
- secure file access
- system interrogation
- SELinux-related handling
- primitives other software can build upon

An essential point Jamie made is that these foundational modules provide trusted decision inputs for other software. For example, if a module determines what OS the system is running, that becomes a critical building block for other logic. Therefore, the documentation must explain these modules not only as APIs, but as trusted foundations in a larger assurance architecture.

### Practical use cases

The development section should also show common developer needs and how UMRS modules help:

- “I want to read a file securely”
- “I want to know what OS I’m on”
- “I want to query trusted system state”
- “I want to obtain reliable security-context information”

This makes the value concrete.

---

## 12. High assurance software patterns

There is also a broader domain of reusable software patterns.

These include:

- patterns designed internally by the UMRS project
- patterns recommended by external assurance or security sources

This material should explain:

- the pattern
- why it exists
- when to use it
- how it improves assurance
- what tradeoffs it may involve

A very important requirement is that these pattern descriptions should cross-reference to **real working implementations** in UMRS wherever possible. The documentation should not be merely theoretical. It should show readers where the pattern lives in actual code, such as in `umrs-selinux` or other crates.

This lets readers move from principle to real implementation.

---

## 13. UMRS tools

There is a separate domain for the tools built using the software and patterns above.

This section should explain:

- what tools exist
- what they do
- how they are used
- what assurance or operational value they provide
- how they demonstrate the underlying development patterns

This section helps bridge development and operations by showing how the libraries become real tools.

---

## 14. Operations

Operations is another major domain.

This covers day-to-day use and care of the system and its tooling. It should include:

- using UMRS tools in operational settings
- care and feeding of the environment
- operational procedures
- security-conscious operations
- routine handling and maintenance tasks

Audience:

- operators
- administrators
- security teams
- maintainers

This is where the project’s work becomes lived practice.

---

## 15. Logging and auditing

Logging and auditing form an important cross-cutting domain that may touch enhancements, development, and operations.

This domain should cover:

- logging mechanisms
- improvements to logging infrastructure
- how higher-assurance operational logging is configured
- how software should audit and log correctly
- how logs are rotated, protected, retained, and handled operationally
- chain of custody for logs and audit data

This should span multiple perspectives:

- system configuration and enhancements
- development practices
- operations and maintenance

The goal is evidence-grade, trustworthy, usable logging and auditing.

---

## 16. Reference material

There is a large reference domain.

This is where detailed, sometimes lower-level technical content lives, including material that may not fit naturally into narrative sections.

Examples may include:

- SELinux concepts and internals
- security context details
- sensitivity levels
- categories
- roles
- architecture reference material
- detailed data-type explanations

Jamie also noted that some material currently lingering in broad reference areas should later be moved into more purposeful sections where it better serves readers. So the reference section is important, but it should not become a dumping ground forever.

---

## 17. Glossary

A glossary is required.

This should provide clear definitions of terminology used throughout the documentation, especially terms that recur in assurance, access control, SELinux, cryptography, logging, trust, and evidence discussions.

Examples may include:

- assurance
- source of truth
- ground truth
- reference monitor
- provenance
- custody
- categories
- sensitivity
- integrity-related terms
- cryptographic terms

The glossary should reduce ambiguity and support consistency across books.

---

## 18. Artificial intelligence in the project

There must also be a section that explains the role of artificial intelligence in UMRS.

This is important for transparency. Jamie does not want mystery around the role AI plays.

This section should explain:

- what AI agents are used for in the project
- what they help with
- what roles they play

Examples include:

- research assistance
- documentation drafting
- organization
- writing support
- quality review
- test-case generation
- code review assistance
- structure and planning assistance

This section should make clear that AI is being used as an accelerator and force multiplier, particularly because the project has thus far been driven by a single person. It should explain the reality plainly: AI helps scale research, organization, review, and writing effort.

It should also make clear that AI assistance does not remove human direction, architecture, judgment, or accountability.

---

## 19. Organizational principle: examples are not exhaustive

For every major section above, any lists of subtopics should be treated as examples, not exhaustive inventories.

There is already more content than was named in the conversation, and additional content will continue to appear. The documentation architecture must therefore be flexible, principled, and capable of absorbing new material.

This is a central design requirement.

---

## 20. Organizational principle: final order is not fixed yet

The domains described here are not necessarily in the final correct order.

These are conceptual groupings Jamie wants recognized. The final documentation system may decide:

- which books they belong in
- which components/modules they belong in
- how they cross-reference
- which content is introductory versus reference
- where content should be split versus merged

So this note should be treated as a **content model and intent statement**, not a frozen table of contents.

---

## 21. Desired future capability of the documentation agent

A major goal is that the documentation agent eventually become capable of intelligent organizational decisions.

The target interaction looks like this:

> “Here is a new topic. Determine where it best fits into our organizational structure.”

That means the agent must be able to:

- understand the documentation taxonomy
- understand audience and purpose
- place material where it belongs
- avoid duplication
- preserve consistency
- maintain cross-references across documentation forms
- distinguish narrative material from reference material
- distinguish introductory material from implementation detail

This is one of the most important takeaways from this note.

---

## 22. Overall editorial understanding

The documentation system is not meant to be a random collection of manuals. It is meant to become a coherent body of knowledge that:

- introduces UMRS clearly
- explains its purpose and philosophy
- provides historical and conceptual grounding
- supports evaluation and deployment
- teaches development on the platform
- documents reusable libraries and patterns
- explains tools and operations
- preserves reference material and terminology
- documents transparency around AI participation
- supports future growth without collapsing into disorder

The Antora/AsciiDoc books are the primary home for this knowledge architecture, while Rustdoc, man pages, and Mallard/Yelp help each serve supporting but important roles.

---

## 23. Documentation as the project’s public voice

An important contextual point the technical writer must understand:

Jamie’s entire career prior to this project was spent in **isolated, air-gapped environments**. Public open-source development and public technical communication are relatively new aspects of this work.

Because of that, the documentation has an additional role beyond traditional manuals.

It functions as the **public voice of the project**.

The documentation must therefore:

- communicate the **depth and seriousness of the work**
- explain the **assurance philosophy behind the project**
- make the project **approachable to outside engineers**
- present the work in a way that encourages **adoption and collaboration**

The Senior Technical Writer should think of the documentation not only as internal reference material, but also as the **primary communication channel to the outside technical community**.

---

## 24. Documentation as a foundation for outreach

The documentation should be written and organized in a way that allows it to serve as source material for other communication channels.

Examples include:

- technical blog posts
- conference talks
- written articles
- tutorial series
- YouTube or presentation content
- community discussions
- educational material explaining assurance concepts

Ideally, well-written sections of the documentation should be easy to:

- extract
- summarize
- adapt
- convert into presentation form

This means documentation should favor:

- clear explanations
- strong conceptual framing
- logical modular sections
- narrative clarity where appropriate

The goal is that a section of documentation can often become the **backbone of an article, presentation, or educational video**.

---

## 25. Adoption as a project goal

A major personal goal of the project is **adoption and usefulness to the broader community**.

The documentation should therefore help:

- engineers understand the project quickly
- organizations evaluate whether ideas or modules are useful
- developers reuse patterns or libraries
- security professionals learn from the assurance techniques used

The Senior Technical Writer should actively assist in making the project understandable and approachable to the wider engineering and security communities.

The goal is not marketing in a superficial sense. The goal is **clear communication of valuable engineering work** so that people who could benefit from it are able to discover and understand it.

---

These points should guide the tone and structure of the documentation moving forward.
