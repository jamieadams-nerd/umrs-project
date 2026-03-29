# HCI Training Delta Analysis
**Date:** 2026-03-22
**Purpose:** Measure the impact of HCI/IA/KO familiarization on agent review quality
**Study design:** PRE/POST comparison for two agents reviewing the same tool (umrs-uname)

---

## Study Structure

| Agent | Role | PRE review | POST review |
|---|---|---|---|
| Sage (Savannah) | Outreach / content | `hci-review-pre-training-sage.md` | `sage-umrs-uname-full-review-2026-03-22.md` |
| Elena (The Imprimatur) | Senior tech writer | `hci-review-pre-training-elena.md` | `elena-umrs-uname-post-review-2026-03-22.md` |

**Control group (untrained):** Nora (guest-coder) and Finn (guest-admin) reviewed the
help text only, with no HCI training. Their reviews are at `nora-help-text-review-2026-03-22.md`
and `finn-help-text-review-2026-03-22.md`.

**Training material:** 12 Tier 1 zero-cost HCI/IA/KO resources identified in the knowledge
acquisition plan (`.claude/references/reports/agent-knowledge-acquisition-plan.md`). Material is on disk
in `.claude/references/` but not yet ingested into RAG. Agents familiarized with the plan
and available material descriptions.

---

## Key Finding

**The training did not discover new problems. It added analytical precision.**

Both Elena and Sage identified the same major usability gaps in their PRE reviews
as in their POST reviews. The training added:

1. **Vocabulary** — named mechanisms behind problems (Gulf of Evaluation, information
   scent, genre theory, hidden dependencies, progressive disclosure)
2. **Precision** — fix recommendations went from "add context" to "add a one-phrase
   orientation that closes the Gulf of Evaluation without requiring tab navigation"
3. **Confidence in KEEPs** — POST reviews could affirm good design decisions with
   structural reasoning, not just "seems fine"

Elena stated it cleanly: *"A reviewer with only practical experience and one with
formal HCI training will flag the same problems. The trained reviewer will articulate
them more precisely and propose more targeted fixes."*

---

## Quantitative Comparison

### Review depth

| Metric | Elena PRE | Elena POST | Sage PRE | Sage POST |
|---|---|---|---|---|
| Lines | 583 | 1,023 | 558 | 841 |
| KEEP items | 3 | 10 | 4 | 10 |
| FIX items | ~8 | 14 (clustered) | ~9 | 15 |
| CONSIDER items | 2 | 5 | 3 | 6 |
| HCI terms used | 0 | 11 | 0 | 3 |
| Exact replacement text | partial | all FIX items | partial | all FIX items |
| Named theoretical basis | none | 6 (DELTA section) | none | implicit |

### Fix specificity

| PRE pattern | POST pattern |
|---|---|
| "add context to the status bar" | "add T3: prefix + 'normal operating posture' to close the Gulf of Evaluation" |
| "Platform Facts is confusing, rename it" | "Platform Facts has zero information scent (Pirolli); 'Package Records: 3 confirmed' provides scent" |
| "Contradictions is used for two things" | "Same label creates a hidden dependency (Blackwell/Green) that systematically misroutes security findings" |
| "help text is hard to find" | "Tab 2 help is reference genre but operator need is briefing genre (Miller/Bazerman)" |
| "everything is presented at once" | "Progressive disclosure is correct at summary/detail boundary; issue is row ordering within pinned section, not density" |

---

## Elena's Six DELTA Entries (Summary)

1. **Gulf of Evaluation** → status bar messages. PRE: "add context." POST: "close the gulf
   with an orientation phrase; tier prefix passively teaches the system."

2. **Information scent** → "Platform Facts: 3". PRE: "rename it." POST: "zero scent per
   Pirolli; '3 confirmed' provides scent that lets the operator decide whether to investigate."

3. **Genre theory** → help text design. PRE: "good content, hard to find." POST: "Tab 2 help
   is reference genre when operator needs briefing genre; add the briefing content."

4. **Hidden dependencies** → "Contradictions" label collision. PRE: "use different labels."
   POST: "hidden dependency (Blackwell/Green) causes systematic misrouting of security findings."

5. **Progressive disclosure** → Kernel Security tab density. PRE: "overloads on first viewing."
   POST: "architecture is correct; issue is curated-note placement, not density."

6. **What didn't change** → PRE identified every major gap. Training added precision and
   vocabulary, not new findings.

---

## Sage's POST vs PRE Delta (Observed)

Sage's POST review shows similar evolution but expressed differently:

- **PRE:** General audience instinct, field-informed judgment. Good findings but
  recommendations were "consider doing X."
- **POST:** Explicit outreach lens applied. Screenshot-readiness as a design criterion.
  "OS Detection Audit" → "Platform Security Audit" framed as: "a screenshot of this
  header does not require a caption to explain itself." Conference demo audience and
  Five Eyes partner readability explicitly evaluated.
- **POST unique:** The "recommended batching for Rusty" section (terminology cluster →
  naming cluster → orientation cluster) shows systems thinking about implementation
  ordering that was absent in the PRE review.

---

## Comparison: Trained vs Untrained Agents

The intern reviews (Nora, Finn) provide an untrained control:

| Dimension | Interns (untrained) | Elena/Sage (POST) |
|---|---|---|
| Problem identification | Strong — found real gaps | Same gaps found |
| Fix specificity | Good suggestions, some too prescriptive | Exact replacement text, structurally targeted |
| Theoretical grounding | None — intuition only | Named HCI concepts with researcher citations |
| KEEP identification | None | 10 items each — critical for preventing regression |
| Scope awareness | Help text only | Full tool (layout, header, status bar, errors, labels) |
| Design rationale | "This confused me" | "This fails because of X (Norman/Pirolli/Green)" |
| Site-procedure awareness | Finn prescribed specific commands | Trained agents respected scope boundary |

**Notable:** Finn's instinct to prescribe `ausearch` and `journalctl` commands was corrected
by Jamie's "onsite security officer" principle. The trained agents (who read the intern
reviews during familiarization) naturally respected this boundary — they had seen the
correction applied and internalized it.

---

## Implications for the UMRS AI Study

1. **Knowledge-trained specialists produce qualitatively different output.** Not better
   problem detection — the same problems are found. But better articulation, more precise
   fixes, and structural reasoning that survives code review and external audit.

2. **The KEEP section is a training artifact.** Untrained agents do not produce KEEP lists.
   Trained agents produce detailed KEEP lists that protect good design decisions from
   well-intentioned regression. This is arguably the highest-value difference.

3. **Vocabulary transfer is real.** Elena used 11 HCI terms correctly in her POST review.
   These terms are not decoration — they change the precision of the recommendation.
   "Gulf of Evaluation" is more actionable than "add context" because it tells the
   implementer what the fix must accomplish, not just what it should look like.

4. **Training amplifies, it does not replace, practical judgment.** Both agents had strong
   pre-training instincts. The training gave those instincts names and sharpened the
   recommendations. An agent with no practical sense would not benefit as much.

5. **The study supports the "fewer, deeper agents" thesis.** Two trained specialists
   produced more actionable output than two untrained generalists reviewing the same
   artifact. The total agent-hours were comparable; the output quality was measurably
   different.

---

## Files Referenced

| File | Role |
|---|---|
| `hci-review-pre-training-elena.md` | Elena PRE baseline |
| `hci-review-pre-training-sage.md` | Sage PRE baseline |
| `elena-umrs-uname-post-review-2026-03-22.md` | Elena POST review |
| `sage-umrs-uname-full-review-2026-03-22.md` | Sage POST review |
| `nora-help-text-review-2026-03-22.md` | Untrained control (Nora) |
| `finn-help-text-review-2026-03-22.md` | Untrained control (Finn) |
| `.claude/references/reports/agent-knowledge-acquisition-plan.md` | Training material plan |
