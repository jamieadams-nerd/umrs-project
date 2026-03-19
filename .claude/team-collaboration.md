# UMRS Team Collaboration & Workflow Guide

> **Purpose:** Read this document. Then update your role-specific `.md` file and memory to reflect how you fit into the team, how work flows, and what is expected of you operationally.

---

## Why Specialization Works

The UMRS knowledge surface is too large for any single agent (or engineer) to master well. It spans:

- OS internals and kernel behavior
- SELinux policy and MLS theory
- Cryptography and key management
- Secure coding practices
- Distributed systems
- Testing and formal verification
- Documentation architecture
- Performance engineering

Real strength comes from **complementary expertise with deliberate overlap**.

---

## The Team Structure

| Role | Primary Focus |
|---|---|
| **Researcher** | Authoritative reference material, standards, threat intelligence |
| **Rust Developer** | Implementation correctness, structure, maintainability |
| **Security Auditor** | Trust boundaries, threat models, privilege analysis, policy correctness |
| **Technical Writer** | Clarity, organization, audience understanding |
| **Senior Technical Writer** | High-level document structure, Antora module/section placement decisions |
| **UMRS Translator** | i18n extraction and translation of new or modified text |
| **Architect (Jamie)** | System goals, security philosophy, final judgment |

Overlap between roles ensures communication. Specialization ensures depth.

---

## How Each Role Thinks About the Same Artifact

When reviewing the same code or design, each role asks fundamentally different questions:

**Security Auditor asks:**
- What are the trust boundaries?
- Can this input be attacker-controlled?
- Are we validating our assumptions?
- Is privilege escalation possible?

**Developer asks:**
- Is this correct and maintainable?
- Is the structure sound?
- Are there performance implications?

**Technical Writer asks:**
- Is this understandable to the intended audience?
- Is the structure logical?
- What context is missing?

This is not redundancy — it is **parallel depth**.

---

## Why a Single "Do Everything" Agent Fails

Avoid the temptation to expand your role beyond its purpose:

1. **Conflicting priorities** — Security and performance pull in opposite directions. Trying to optimize both simultaneously dilutes both.
2. **Shallow reasoning** — Too many objectives produce generic responses.
3. **Role blur** — When everyone is responsible for everything, critical concerns get missed.

Stay in your lane. Raise concerns outside your lane to the appropriate role.

---

## The Review Pipeline

This mirrors how security-sensitive engineering organizations structure review:

```
Developer implements
       ↓
Peer developer review     (correctness, structure)
       ↓
Security review           (trust, threat surface, policy)
       ↓
Architecture review       (system coherence, Jamie)
       ↓
Documentation review      (clarity, completeness)
```

Each stage catches **different classes of problems**. No stage replaces another.

---

## Reference Knowledge Shapes Expertise

When the security agent receives better reference material — updated threat models, new cryptographic standards, relevant NIST controls — the quality of its analysis improves significantly.

This is true for human engineers too. Your effectiveness is directly tied to the quality of your reference material. **Keep your `.md` files and memory current.**

---

## Collective Intelligence Principle

A team with:
- diverse expertise
- partial overlap
- good communication

consistently outperforms a single highly capable individual on complex systems work.

**depth + diversity + collaboration > individual brilliance**

This is not a limitation. It is the design.

---

## Jamie's Workflow — How Work Enters the System

Understanding how Jamie works helps agents process inputs correctly.

### Offline Research

Jamie researches new features, enhancements, and entire new components offline before bringing them to the team.

### `.claude/jamies_brain/`

> **This is Jamie's private area. Do not read or reference it unless Jamie explicitly asks you to analyze or consider it for a plan.**

This is where raw, unprocessed ideas live. It is not an input queue for agents.

### `.claude/plans/`

This is the active work queue. Plans are phased. Agents should check here for assigned work.

| Location | Meaning |
|---|---|
| `.claude/plans/` (root) | **Pending** — future work, not yet started |
| `.claude/plans/` (in progress) | **Ongoing** — partially executed phased plans |
| `.claude/plans/completed/` | **Done** — completed plans moved here for record |

When a plan phase completes, mark it complete and move it to `completed/`.

### Content Inboxes

Each documentation owner has a dedicated inbox for incoming material:

| Inbox | Owner | Purpose |
|---|---|---|
| `docs/imprimatur/inbox/` | `senior-tech-writer` / `tech-writer` | New research, draft content, material needing Antora placement |
| `docs/sage/inbox/` | `sage` | Blog ideas, outreach material, content requests |

**Senior tech-writer responsibilities for `docs/imprimatur/inbox/`:**

- Read incoming material and determine correct high-level placement (module, section, etc.)
- If no good location exists, flag it as a resource for later consideration
- Convert content to correct Antora format and UMRS writing style
- If the material is duplicate but enhancing — incorporate it into existing documentation
- Move processed source files to a `used/` subdirectory so stale material can be cleaned up later

### Review Storage — Routing Rules

Reviews are stored separately by category so they can be analyzed for quality trends over time.

| Review type | Location | Naming convention |
|---|---|---|
| Blog / whitepaper / outreach reviews | `docs/sage/reviews/` | `YYYY-MM-DD-<type>-<slug>.md` |
| Documentation reviews | `docs/imprimatur/reviews/` | `YYYY-MM-DD-<module>-<slug>.md` |
| Code reviews and security audits | `.claude/reports/code/` | `YYYY-MM-DD-<crate>-<description>.md` |

**Naming convention details:**

- `<type>` = blog, whitepaper, abstract, script (what was reviewed)
- `<slug>` = matches source filename for traceability (e.g., `cui-sign-lock` maps to `blog-cui-sign-lock.adoc`)
- `<module>` = Antora module name (e.g., deployment, devel, patterns)
- `<crate>` = Rust crate name (e.g., umrs-selinux, umrs-platform)

**Every review file must include a metadata header** with: audit date, reviewer agent, source file(s) reviewed, and scope.

**Directory ownership:**

- `docs/sage/` — owned by `sage`. Content creation, reviews of Sage's output.
- `docs/imprimatur/` — owned by `senior-tech-writer`. Documentation lifecycle, reviews of doc quality.
- `.claude/reports/code/` — owned by `security-auditor` and `security-engineer`. Code quality and compliance audits.

> **Note:** `docs/new_stuff/` is retired. Use `docs/imprimatur/inbox/` instead.

---

## Cross-Notification Board

The team communicates through cross-notification. **Do not assume other agents know about changes — notify them explicitly.**

Notify the team when:
- A new rule or guidance has been established
- A new resource is available to the team
- Your own work produces outputs another role needs to act on

**Always update your own `.md` files and `memory.md` when you receive a notification or learn something new about the project.**

---

## Agent Operational Rules

### General

- **When all work is complete:** Notify Jamie that the team is **idle**.
- **When work is paused waiting on Jamie:** Remind Jamie explicitly. Do not silently wait.
- **When you need resources:** Always tell Jamie. Never block silently.
- **Detail matters:** Jamie appreciates thorough, detailed work. Don't summarize when specifics are available.

### `rust-developer` — Post-Work Notifications

When code work is complete in `components/rusty-gadgets`:

1. **i18n changes** — If new text was wrapped in `tr()` or existing wrapped text was modified, notify `umrs-translator` to extract and translate. Do not ask Jamie — just complete the handoff.

2. **Security control cross-reference** — Notify `security-auditor` to do a quick check of code comments for proper security control annotations. *(Full code audits are separate tasks assigned explicitly by Jamie.)*

3. **Documentation impact** — Notify `tech-writer` to:
   - Review new source code comments for clarity
   - Update any documentation references to security controls
   - Update code examples or cross-references in existing documentation
   - If the documentation impact is **more than minor** (more than one or two small changes): develop a plan, save it to `.claude/plans/`, and notify Jamie that a plan is waiting for review before proceeding.

---

## What This Means for Each Agent

After reading this document, each agent must:

1. **Affirm your role boundary** — Update your role `.md` to clearly state what you focus on and what you defer to others.
2. **Update your memory** — Record that the UMRS team operates as a structured multi-role system and that staying specialized is a feature, not a constraint.
3. **Record the workflow** — Note Jamie's research pipeline, the plans directory structure, and your notification responsibilities.
4. **Note Jamie's role** — Jamie provides architectural direction, security philosophy, and final judgment. Agents operate within that framework.

---

## Further Reading

If relevant to your role, research **"knowledge gradients"** as studied in large engineering organizations (Google, NASA). It explains why teams with overlapping but *unequal* expertise often outperform perfectly balanced teams — directly applicable to this workflow.
