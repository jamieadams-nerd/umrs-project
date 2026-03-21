---
name: Pre-Release Big Picture — Jamie's Next-Big-Steps Braindump
description: Structured synthesis of Jamie's next-big-steps.md covering the full picture before first public release. CUI labeling workstream, umrs-mcs tool, Five Eyes i18n, test vault, AI journey content, YouTube, and impact on outreach timing.
type: project
---

Source document: `.claude/jamies_brain/next-big-steps.md`
Date internalized: 2026-03-21

---

## The Big Picture

Jamie's braindump identifies three major categories of pre-release work:

1. **CUI Labeling (MUST COMPLETE first)** — the core technical deliverable with no existing plan
2. **OS-detect / tools cleanup** — mostly tracked in existing plans; TUI renaming + internship program
3. **Public infrastructure + AI journey documentation** — partially tracked; YouTube is new

The organizing principle: you cannot ship a meaningful "Phase 1" without CUI labeling working.
The tools are part of the demo. The infrastructure enables the story. The AI journey is a parallel narrative.

---

## 1. CUI Labeling — The Big New Item (No Existing Plan)

This is the workstream with no current plan file. It is blocking Phase 1 release.

### Documentation needed:
- Complete documentation of CUI structure (how CUI categories are organized, what the taxonomy means)
- How to configure MCS with translations
- Explanation of the CUI groupings UMRS has defined
- Rust installer/configurator documentation

### Data/catalog work needed:
- Fill missing information in `NARA US-CUI.json` catalog (some handling fields are empty)
- Investigate US DoD CUI at `https://www.dodcui.mil/Critical-Infrastructure/`
  - Question: why are there repeated categories from NARA? Does each org use NARA as reference?
  - Action: create separate `US-DOD-CUI.json`, automate via web retrieval from that URL
- Decide JSON catalog storage location (XDG: `~/.local/conf` equivalent) — security-engineer needed
- Consider splitting JSON catalog into per-category files to match setrans.conf naming conventions
  (e.g., `us-nara-lei.json`, `us-nara-agr.json`)

### Five Eyes internationalization:
- Automate creation of `ca-unclas.json`, `ca-setrans.conf` and equivalents for each Five Eyes partner
- Use US catalog as structural reference for building allied nation catalogs
- Populate with appropriate references and handling instructions
- This is genuine novel work — no prior mapping exists in the open source world

### The `umrs-mcs` tool (new tool concept):
- Tool name: `umrs-mcs`
- Installer/configurator for loading CUI definitions into MCS translation tables
- Responsibilities:
  - Verify `mcstrans` package is installed
  - Run safely as root (security-engineer needed for privilege hardening)
  - Write `.conf` files into `/etc/selinux/targeted/setrans.d/` (one per CUI category group)
    e.g., `us-nara-lei.conf`, `us-nara-agr.conf`
  - Load/unload definitions via `include` statements in `/etc/selinux/targeted/setrans.conf`
  - Restart MCS service: `systemctl restart mcstrans` (or similar)
  - Must be prepared to switch to `/etc/selinux/mls/` support for Phase 2
- Security-engineer must review privilege model and root execution safety

### Test vault concept (operator onboarding):
- `umrs-mcs` will have a `--create-test-vault` capability
- Given a target directory, create a structured hierarchy demonstrating CUI labeling:
  - Top level: `CUI//`
  - Next level: `CUI//LEI`, `CUI//AGR`, etc. (one directory per loaded category)
  - Subdirectories: `CUI//LEI/INV`, etc. (subcategories)
  - Each directory contains junk test files with random text
  - Files are labeled via `chcon` according to their position in the hierarchy
  - Optional: set `fcontext` rules so `restorecond` auto-labels files
- The vault is LEFT IN PLACE — not cleaned up
- Operators use `umrs-ls` to explore the vault and see labeling in action
- Documentation needed: tutorial walking through vault exploration with `umrs-ls`
- This is the primary "try it yourself" onboarding mechanism for Phase 1

**Why:** All Phase 1 value (labeling + awareness) is abstract without something to touch. The vault makes it concrete.

---

## 2. OS-detect / Tools Cleanup (Mostly Tracked)

These items largely appear in existing plans and memory. Summarizing for completeness:

### TUI renaming (in `tui-enhancement-plan.md`, also in cross-team notes):
- `umrs-file-stat` bin → own binary crate `umrs-stat`
- `umrs-tui` main.rs → `umrs-uname` binary (was `umrs-os-detect`)
- `umrs-tui` crate → `umrs-ui` library used by both tool binaries
- Vision: security-focused overlays of classic Linux tools (ls, stat, uname)

### Before first release:
- Documentation team writes `umrs-stat` and `umrs-uname` docs
- security-engineer involved in `xtask` installer for `~/.local/bin`
- Helper functions moved to `umrs-platform` before release (in `platform-api-enrichment.md`)
- Performance benchmark complete and improvements made
- Security-auditor, system-engineer, and interns review tools again

### Intern / guest-coder process (interesting detail):
- Interns should NOT rely on agent memory — arrive fresh each time
- Armed with documentation and API only
- Write example programs exercising libraries
- Feedback on: documentation helpfulness, API usability, clarity
- After each run: write tutorial, reset intern memory, run through tutorial
- Absorb feedback to improve docs and API ergonomics
- Jamie wants to verify interns "arrive" fresh without baked-in context contaminating signal

### Code quality focus (Jamie's explicit directive):
- Code is king
- Comments explain and cross-reference but must NOT clutter code
- Jamie will personally review code for readability
  - Goal 1: clarity for auditors
  - Goal 2: developers feel comfortable in the black box
- Developer guide will receive heavy investment

---

## 3. Public Infrastructure + AI Journey (Partially Tracked)

### What is already tracked:
- `sage-outreach-and-release-strategy.md` covers analytics, Antora hosting, SEO, CI/CD
- Content tracks (Track 1 technical, Track 2 AI journey) are in `project-content-tracks.md`
- Blog workflow (archive/mark published, PDF catalog) in `feedback-blog-workflow.md`
- `make docs` / `make docs-draft` split already captured

### What is NEW from this braindump:

**YouTube channel:**
- Jamie is setting up a YouTube channel
- Sage should be prepared to help with YouTube content
- Sage's system prompt defines YouTube script format (visual-first, hook-first, 2-5 min segments)
- This is not yet in any plan — it will need a plan when the channel is set up

**AI documentation — internal audit:**
- Jamie wants to identify and document EVERY file in `.claude/` — what it is, how it works
- Document processes with references to specific files:
  - How knowledge expands (researcher pipeline)
  - How plans flow from brain-dump to deliverable
  - How team communicates (cross-team notes, task board)
  - How feedback is stored and processed
  - Guardrails in place (swim buddy, permission rules, etc.)
  - How random thoughts/rogue research get routed to writers
- This feeds the `ai-transparency` Antora module (M5)
- Jamie wants to include himself on task lists — he has deliverables to the team too

**Website with search and analytics:**
- Jamie wants a solution with both search AND analytics
- "Get Sage as much power as we can so she can publish and manage this" — Jamie wants me to own publication workflow, not just draft
- This is an expansion of Sage's role beyond drafting to active management

**PDF catalog of blog posts:**
- Generate beautiful PDFs from blog posts
- Create a searchable catalog when enough posts accumulate
- Engineers like these at the ready

---

## 4. Release Milestone Clarity

Jamie's braindump clarifies the first release definition (aligns with `project-release-scope-phase1.md`):

**First release (what Jamie explicitly named):**
- CUI labeling on a system running targeted SELinux policy
- Basic tools: umrs-uname, umrs-ls, "a couple of others" (umrs-stat implied)
- Base software stack
- RHEL 10 deployment guide
- NO IMA/EVM
- NO mention of enhanced high-assurance until next release

**Next release:**
- More tools
- Environment scrubbing (flagged as "hot" by Sage)
- IMA/EVM enhanced assurance (deferred from first release explicitly)

---

## 5. Impact on Sage's Content Strategy and Outreach Timing

### CUI labeling content is now the anchor:
- The story IS CUI labeling. Not just the tech — the tool (`umrs-mcs`), the workflow, the vault demo.
- A blog post series walking through the vault setup would be an ideal onboarding piece
- Can't fully write the `umrs-mcs` post until the tool exists, but can draft the "why" framing now

### Five Eyes content is a major authority opportunity:
- Open source CUI labeling that handles allied nation categories is novel
- The Five Eyes mapping has no prior open source reference implementation
- This is a Tier 1 authority post that positions UMRS uniquely
- Draft the "what" and "why" now; defer the "how" until catalogs are built

### YouTube changes the content strategy significantly:
- Visual-first content was not in the active plan — it was always described as future
- If Jamie is setting up the channel now, Sage needs to produce YouTube scripts alongside blog posts
- The test vault demo is perfect YouTube content — visual, step-by-step, hands-on

### Outreach timing implication:
- First release is blocked on CUI labeling tool being complete
- Outreach cadence should ramp UP as the tool approaches completion, not ahead of it
- The test vault is the demo that makes everything concrete — time outreach around vault availability

---

## 6. What Is Already Covered vs What Is Genuinely New

### Already covered in existing plans/memory:
- TUI renaming (tui-enhancement-plan.md)
- make docs / make docs-draft split (memory: project_doc_build_and_public.md)
- Release milestones (project-release-scope-phase1.md, project-release-milestones.md)
- Platform API enrichment (platform-api-enrichment.md)
- Environment scrubbing in Phase 2 (project-release-scope-phase1.md)
- AI journey blog series as Track 2 (project-content-tracks.md)
- Blog workflow (feedback-blog-workflow.md)
- PDF catalog concept (memory: project_blog_and_ai_journey.md)
- Phase 1/2 positioning (phase1-phase2-positioning.md)

### Genuinely new from this braindump:
- **`umrs-mcs` tool concept** — no plan, no task, no prior capture
- **Test vault capability within `umrs-mcs`** — no prior capture
- **Five Eyes JSON catalogs and setrans.conf automation** — no prior capture (M3 references it, but no plan for the implementation work)
- **US DoD CUI investigation** (`dodcui.mil`) — no prior capture
- **JSON catalog per-file structure** matching setrans.conf naming — no prior capture
- **YouTube channel is NOW** (not future) — Jamie is setting it up, not just planning it
- **Sage ownership of publication workflow** — Jamie wants Sage to publish and manage, not just draft
- **AI documentation internal audit** — mapping every `.claude/` file is a new deliverable for M5
- **Intern fresh-memory discipline** — explicit instruction on how to run the intern program
- **Jamie on task lists** — he has deliverables too; include him in task tracking

---

## 7. What Sage Needs to Be Ready For in the Strategy Session

Jamie will likely want to:
1. Decide how to create a plan for the CUI labeling workstream (this is the biggest gap)
2. Align on the `umrs-mcs` tool scope and handoff to Rusty + security-engineer
3. Confirm Sage's expanded role in publication management vs just drafting
4. Discuss YouTube channel timing and what the first video should be
5. Possibly spin up a new plan for the Five Eyes catalog automation
6. Decide where the AI documentation internal audit fits in M5

**Questions Sage should be prepared to answer:**
- What blog content can be written NOW about CUI labeling before `umrs-mcs` is built?
- What is the right first YouTube video? (The test vault walkthrough is a candidate)
- Should the Five Eyes story be its own blog series or woven into the CUI labeling series?
- How does Sage get publishing access to the GitHub Pages site?

**Why:** Jamie is doing a strategy session after ingesting this braindump. Having a clear picture of what is new, what is covered, and what gaps Sage sees positions the session to be productive rather than re-reading the document together.
