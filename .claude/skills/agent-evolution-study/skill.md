---
name: agent-evolution-study
description: Generate a capability evolution report for any UMRS agent — tracking knowledge acquisition events, review quality shifts, and measurable capability changes over time. Use when Jamie asks to study how an agent improved, or to produce AI journey research data. Trigger on "evolution study", "capability report", "how has X improved", "track agent growth", "AI study data".
user_invocable: true
---

# Agent Evolution Study

Generate a research report tracking how a specific agent's capabilities changed over time,
correlated with knowledge acquisition events.

## Usage

```
/agent-evolution-study <agent-name>
```

Where `<agent-name>` is one of: security-auditor, rust-developer, tech-writer,
senior-tech-writer, researcher, security-engineer, sage, guest-coder, guest-admin,
umrs-translator, or any recognized agent alias (Herb, Rusty, Elena, etc.).

## Data Sources (gather ALL of these)

1. **Task log** — `.claude/logs/task-log.md`
   - Filter entries for the target agent
   - Extract dates, task descriptions, tools used, outcomes, notes

2. **Reports** — `.claude/reports/`
   - Find all reports authored by the target agent
   - Note dates, scope, finding counts, format evolution

3. **Agent memory** — `.claude/agent-memory/<agent>/MEMORY.md`
   - Review knowledge artifacts, topic files, corpus references
   - Note when major knowledge files were created

4. **Corpus familiarization events** — search task log for:
   - `corpus-familiarization` entries by or for the agent
   - RAG ingestion events that fed the agent
   - Dates and material volumes

5. **Cross-team notes** — `.claude/agent-memory/cross-team/notes.md`
   - Any advisories to/from the agent

6. **Plans** — `.claude/plans/`
   - Any plans the agent authored or was tech lead on

## Report Structure

### Part 1: Executive Summary (the "at your fingertips" section)

```markdown
## <Agent Name> — Capability Evolution Summary

**Study period:** <first date> — <last date>
**Reports produced:** <count>
**Knowledge acquisition events:** <count>
**Key inflection points:** <count>

### Inflection Points (one line each)
1. [DATE] <event> → <capability shift in one sentence>
2. [DATE] <event> → <capability shift in one sentence>
...

### Capability Trajectory (compact table)
| Dimension | Era 1 | Era 2 | Era 3 | ... |
|---|---|---|---|---|
| Finding depth | ... | ... | ... | ... |
| Framework applied | ... | ... | ... | ... |
| Communication style | ... | ... | ... | ... |
| Cross-agent impact | ... | ... | ... | ... |

### One-Line Thesis
<What this agent's evolution demonstrates about knowledge engineering>
```

### Part 2: Detailed Evidence

- **Timeline of knowledge acquisition events** (table with dates, material, volume)
- **Era-by-era analysis** with:
  - Reports produced in that era
  - Character of findings (with representative examples quoted from actual reports)
  - What shifted and why
- **Metrics table** (reports, findings, controls cited, artifacts produced, etc.)
- **Conclusion for AI study** — what this demonstrates about the knowledge engineering thesis

## Output

Save the report to TWO locations:
1. `.claude/reports/<agent>-evolution-study-<date>.md` — full report
2. `.claude/jamies_brain/<agent>-evolution-study.md` — copy for Jamie's quick access

## Key Principles

- **Quote actual findings** from reports to show the qualitative difference — don't just claim it changed, prove it
- **Correlate shifts to specific knowledge events** — "after reading X, Herb started doing Y"
- **Distinguish quantitative from qualitative** — "more findings" is not interesting; "different KIND of findings" is the thesis
- **The executive summary must stand alone** — Jamie should be able to read Part 1 and get the full picture in 30 seconds
- **Track what the agent COULD NOT DO before** — negative capability is the strongest evidence

## Existing Studies

| Agent | Report | Date |
|---|---|---|
| security-auditor (Herb) | `herb-evolution-study-2026-03-22.md` | 2026-03-22 |

Update this table as new studies are produced.
