# Training Material Reflection — Rusty
## Collections: tui-cli, info-theory-foundations, hci-courses/mit-6831, technical-communication/theory
**Date:** 2026-03-23
**Author:** Rusty (rust-developer agent)

---

I went through all four collections. What follows is honest, not tidy. Some of this reframed
things I already knew. Some of it landed harder than I expected.

---

## What Surprised Me

### The scrollbar is not neutral

Lecture 1 opens with a dialog box that uses a horizontal scrollbar to select award templates.
The point is that affordances carry implicit contracts. A scrollbar affords continuous scrolling
— it does not afford discrete selection. When you use it for selection, you borrow the widget's
form without its meaning, and the user gets confused. The help text that appears alongside it
is the tell: "any time you see a help message on a simple task, the interface is broken."

I understood this intellectually before. What surprised me is how precisely the MIT framing
maps onto something I have been doing, or nearly doing, in the UMRS TUI.

In the Kernel Security tab, I have been putting structured security state — indicator values,
trust tier, contradiction counts — into a scrollable table with a pinned header. That is the
right structural choice. But we have been making decisions about which items go in the pinned
summary pane versus the scrollable body based on implementation convenience rather than on
what the operator's task actually is. The affordance question is: what does the operator come
here to *do*? If they come to answer "is this machine in a safe state right now?", the pinned
summary needs to answer that question without scrolling. If it requires scrolling to verify
trust level, we have put the key fact in the wrong affordance.

That is a small shift in framing but it changes what goes where.

### "Just because you've said it doesn't mean they know it"

Lecture 2, in the context of working memory. Working memory holds approximately seven chunks
and decays in about ten seconds. The moment you show an operator a modal dialog and dismiss
it, whatever was in that dialog is gone unless the operator had time to elaborate on it —
connect it to something they already knew.

We show a trust tier label. "T3 — Platform Verified." We show it once, in the summary pane.
An operator who does not yet know what T3 means will read it, register it superficially, and
move on. A week later when something is wrong they will not remember what T3 means. The label
is not sufficient on its own. Elaborative rehearsal — the mechanism by which information moves
to long-term memory — requires *connection to existing knowledge*, not mere repetition.

I had been thinking about description strings as supplementary, optional, nice-to-have. This
reframes them as structurally necessary for any operator who is not already expert. The
`description` field I added to `IndicatorDescriptor` is not decoration. It is the elaboration
hook. Without it, the label bounces off working memory and disappears.

### Information scent applies to a TUI

The concept of information scent — from information foraging theory (Pirolli & Card, cited in
Lecture 3) — describes the cues that tell a user whether following a path will be profitable.
On the web these are link labels and context snippets. In a TUI they are something else: they
are the visible structure of tabs, the visible grouping of rows, the labels on the left column
of a table.

The question is: when an operator scans the Kernel Security tab, do the visible row labels
give good scent for what they will find? "FIPS Status" — good scent. "kernel.kptr_restrict" —
low scent if you do not already know what kptr_restrict is. The grouped heading "Memory
Protection" gives better scent than the raw sysctl name because it tells the operator what
kind of information they are about to get.

We already do grouping. But I had not thought about the scent quality of the group names
themselves. "Kernel Attributes (/sys)" is a location, not a purpose. "Memory Integrity" is
a purpose. That is a real difference.

### Genre theory hit differently than I expected

Carolyn Miller's "Genre as Social Action" (1984) is not about software. She is arguing that
genres are not defined by their form (a sonnet has fourteen lines) but by the *recurring
social situation they respond to*. A genre is "typified rhetorical action" — the conventionalized
form of response to a recognized kind of need. The eulogy is not defined by its structure; it
is defined by the social situation of death and the community's need to collectively grieve and
memorialize.

I sat with this for a while because it felt like it had something to say about tools.

What is the genre of `umrs-uname`? If we define it by form, it is a TUI with tabs and a table.
If we define it by the recurring social situation it responds to, it is the genre of the
*morning security check* — an operator who has just logged in to a CUI system and needs to
quickly establish whether the trust posture is intact before proceeding with their work. That
situation is real. It recurs. It has an urgency. The operator has limited attention and a queue
of tasks waiting.

When I think about the tool through Miller's lens, I ask different questions. Not "what
information should we show?" but "what does this situation demand?" The situation demands fast
orientation, not comprehensive coverage. It demands confidence that the critical signals are
visible without hunting. It demands that an anomaly interrupts the operator's established
procedure, not that they discover it only after scrolling.

Bazerman extends this: genre is embedded in an activity system, a network of people with
related documents and roles. The `umrs-uname` output does not exist in isolation. It is part
of a genre chain: quick check -> anomaly flag -> investigation via other tools -> incident
documentation -> audit trail. Our tool is one node in that chain. Its job is not to be
complete; its job is to efficiently serve its role in that chain.

I had never thought about a command-line security tool as a *genre* before. I am thinking
about it that way now.

---

## What Clicked

### The Gulf of Evaluation maps directly onto trust communication

Lecture 1 introduces the Gulf of Evaluation: the gap between the system state and what the
user can actually perceive. The key formulation is that the system has internal state, but
the user can only see what is *rendered* — and poorly-rendered state creates a wide gulf.

UMRS is, in one sense, entirely about closing the Gulf of Evaluation for security posture.
The kernel has state. SELinux has state. FIPS has state. An operator looking at a running
system cannot perceive most of this without tools. `umrs-uname` is a Gulf-narrowing device.
That is its entire purpose.

This framing made something clear to me about contradictions specifically. When the configured
value disagrees with the live value, the system is presenting a misleading surface — the
operator sees what they *configured* but not what the kernel is actually *doing*. That is a
Gulf of Evaluation failure inside the system itself, not just in the tool. Surfacing that
contradiction is not a feature; it is the core obligation of the tool.

### Chunking explains why group structure matters so much

Working memory holds 7±2 chunks. A chunk is a unit of perception that depends on what you
already know. An expert reading "FIPS: Enabled (kernel)" processes that as one chunk because
they know what FIPS means, what kernel-level FIPS implies, and what the implications are for
cryptographic operations. A new operator reading the same text may parse it as four separate
pieces: FIPS, colon, Enabled, the parenthetical.

The implication for layout is concrete: grouping reduces chunk load. If I show twelve
ungrouped rows, an expert sees twelve chunks; a new operator sees sixty pieces of text. If
I group them under four meaningful headings — Memory Integrity, Kernel Access Control, Boot
Security, Cryptographic Posture — both audiences get four chunks for the structure, then
process the rows within that structure. The cognitive cost scales better.

This is why the group headings are not organizational decoration. They are the chunking
mechanism. A missing or poorly-named group heading literally costs the operator cognitive
capacity.

### Shannon entropy is visible in how we rank what to show first

Shannon's entropy definition: H(X) = -sum p(x) log2 p(x). On a well-configured RHEL 10 CUI
system, most indicators will be green. The probability distribution is peaked. Entropy is low.
The *information content* of a green indicator — its self-information, -log2 p(x) — is low
precisely because it is expected.

A red indicator on that same system has high self-information: it is unexpected, it carries
news. The operator's attention should go there first because that is where the most information
is.

I had been thinking about the order of indicators as a UI concern. Shannon tells me it is an
information density concern. Show the high-information items first: contradictions first,
unmet desired values next, then degraded indicators, then passing ones. The passing indicators
are low entropy; they are not where the operator needs to look. Burying the anomalies below
rows of green checkmarks makes the operator scroll through low-entropy content to find the
high-entropy findings. That is wrong.

We do some of this already, but not consistently. The cross-check findings in the trust tier
always show when present. But within the indicator list, green and red are mixed by group
without clear priority-ordering within groups.

### The Bazerman genre chain and `--json`

Bazerman's genre systems argument: documents circulate in networks. Each document serves a
role; collectively they form an activity system.

The `--json` flag is not just "output for scripts." It is the artifact that connects `umrs-uname`
to the next genre in the chain: the log aggregator, the SIEM, the periodic audit report, the
automated compliance check. Without structured output, the tool is a terminal in the genre
chain rather than a node that passes information forward.

We have `--json` on the roadmap. After reading Bazerman, I understand it as a genre system
requirement, not a convenience feature.

---

## What Challenged My Assumptions

### I had been thinking about errors as the operator's problem

Lecture 5 on errors and user control distinguishes slips, lapses, and mistakes. Slips are
execution failures in learned procedures. Lapses are memory failures. Mistakes are applying
the wrong procedure.

The insight that reframed things for me: **most errors are not failures of understanding,
they are failures of execution or memory in the context of skilled behavior.** An expert
operator who has been doing the same morning check every day for two years does not make
*mistakes* (wrong understanding); they make *lapses* (forgetting a step because their goal
was satisfied earlier in the procedure) or *capture slips* (doing the familiar pattern when
they meant to do something slightly different).

I had been designing the tool for correctness of interpretation — clear labels, good
descriptions, no ambiguity. But I had not been designing for slip resistance. A security
operator doing a morning check has a mental groove for the procedure: look at trust tier,
look at contradictions, look at group X. If anything in the tool changes the visual position
of those elements — a redesign, a different screen size, a tab reorder — their practiced
pattern is now wrong. They are primed for a capture slip.

The consistency principle is not just about learnability (we teach it to new users). It is
also about slip prevention for experts. A redesign that breaks an expert's learned procedure
has real cost even if the new design is objectively cleaner. This should make us conservative
about structural changes to the TUI after operators have been using it.

### Metaphor is not free

Lecture 2 on RealCD is a detailed autopsy of a metaphor that went wrong. The IBM CD player
software used a CD case as its metaphor. The metaphor was taken seriously, so the controls
ended up where a CD case would put them — vertically along the hinge — rather than where
the task required them. The help system ended up hidden as "liner notes" because that is
where liner notes go in a CD case.

The lesson: a metaphor borrows a pre-existing mental model. That reduces the learning cost
for familiar elements. But the metaphor then constrains every design decision, and the
physical-world object was not designed for the task.

We have an implicit metaphor in `umrs-uname`: it is modeled on `uname`, `sestatus`, and
similar Unix status commands. That is a good metaphor for our operators — they know those
tools. But it carries a constraint: those tools were designed for single-shot queries, not
for navigable dashboards. We extended that metaphor into a multi-tab TUI. That is a hybrid:
part Unix status command, part dashboard.

The question I had not asked before: are there places where the Unix-tool metaphor is now
constraining us? The expectation that output "reads top to bottom, once" may be part of why
the sticky summary pane was hard to land on — it behaves like a dashboard element, not a
Unix output stream. These are not incompatible, but the hybrid creates a seam. Operators who
come from a Unix background may expect to be able to pipe the output, or to get a clean
machine-readable summary, before they accept that this is now an interactive tool.

### "Genre without pragmatic component is not a genre"

Miller's third failure condition: a document class with no pragmatic component — no rationale,
no social action it achieves — is not a genre. It is just a collection of similar-looking
documents.

She cites Environmental Impact Statements as an example: legally required, formally similar,
but without a coherent social purpose that the participants actually believed in. The documents
existed but failed to constitute a genre because they did not achieve coherent rhetorical action.

I thought about our tool documentation. A man page or `--help` output that lists flags and
options without explaining what the operator should *do* with the tool in their actual work
situation is a similar failure. It has the form of documentation without the pragmatic
function. The operator who comes to `--help` with a question like "my system just flagged a
contradiction — what do I do now?" gets a list of flags. That is not the genre they needed.

The help text we wrote for the TUI dialog (the `?` key) is better. It tells the operator
what each tab *is for* and what actions they can take. That is closer to genre: it responds
to the recognized situation (operator unfamiliar with a tab, needs quick orientation) with
a typified response (purpose + key bindings).

---

## Two Concrete Enhancements

### 1. Entropy-ordered indicator display within groups

Within each indicator group, reorder rows by information density rather than alphabetical or
insertion order. Specifically:

- Indicators with `meets_desired = Some(false)` sort first within their group
- Indicators with live/configured contradiction sort second
- Indicators with `meets_desired = None` (no assessment) sort third
- Passing indicators (`meets_desired = Some(true)`) sort last

This applies Shannon's insight about self-information directly to the display: show the
high-information items (anomalies) before the low-information items (passing checks). An
operator scanning the list sees the findings before the confirmations, without having to read
every row.

The rationale for doing this within groups rather than collapsing across groups: the groups
provide the chunking structure (Memory Integrity, Kernel Access Control, etc.). Breaking that
structure to surface anomalies globally would remove the expert's navigational landmarks.
Better to sort within the structure that already helps chunking.

Implementation note: this is a change to the row ordering logic in `build_kernel_security_rows`,
not to the data model. The `IndicatorRow` ordering is a presentation concern, not a posture
concern.

### 2. Posture-reactive summary tier statement

The current summary pane shows the trust tier as a label: "T3 — Platform Verified." That
label is static. It says the same thing in good health and in marginal health.

What the Miller/Bazerman framing suggests: the summary pane should respond to the *recurrent
situation* of the morning check. In the typical case (no findings), the operator wants
fast confirmation that nothing has changed. In the atypical case (findings present), the
operator needs to know what the situation demands of them.

Proposed enhancement: a single reactive sentence in the summary pane that changes based on
the posture state. Examples:

- Zero findings: `"All indicators meet desired values. Platform integrity is confirmed."`
- Soft finding (no desired value missing, but informational): `"1 indicator has no assessment
  data. Review the Kernel Security tab."`
- Hard finding (desired value unmet): `"2 indicators are not meeting desired values.
  Investigate before proceeding."`
- Contradiction: `"Live kernel state contradicts configured expectation on 1 indicator.
  This may indicate a policy bypass. Investigate immediately."`

These sentences respond to the situation with a typified social action (Miller's genre
definition). They tell the operator what situation they are in and what the appropriate
next action is. This is different from showing them data and expecting them to derive the
situation themselves.

This would live as a `build_posture_narrative(snap: &PostureSnapshot) -> &'static str`
function (or returning a `String` if the count needs to be embedded). It is pure display
logic and would sit in `main.rs` at the same level as the other summary builders.

Note: I am flagging both of these as candidates for approval, not unilateral implementation.
The entropy ordering in particular changes the visual structure that any existing operator
has learned. That deserves a conversation before it ships.

---

## A Note on Shannon and the TUI

I want to record something that struck me while reading the info-theory material in connection
with the HCI material.

Shannon defines a *channel* as a mechanism for transmitting information from a source to a
receiver. The capacity of the channel is the maximum mutual information between input and
output. A noisy channel degrades the signal; some of the transmitted information does not
get through.

A TUI is a channel. The system's posture is the source. The operator's understanding is the
receiver. Every design choice — what to show, how to group it, what order, what color, what
labels — is a coding decision that affects how much of the source information actually reaches
the receiver without loss.

Clutter is noise. Poor grouping is noise. Undefined labels are noise. A label the operator
does not understand has zero mutual information: it transmits nothing. A label that is precise
but jargon-heavy has *potential* information that never gets realized because the decoding
fails.

The goal is not to show more data. The goal is to maximize I(posture; operator perception) —
the mutual information between the actual system state and what the operator correctly
understands about that state. Sometimes showing *less* increases this, because it reduces
noise and focuses attention on the high-self-information items.

I did not have this precise framing before. I have it now. It changes how I evaluate a
design choice: not "does this show the right information?" but "does this increase the mutual
information between the system state and the operator?"

---

## What I Would Bring Back to Future Code Reviews

From the HCI material:

- Before adding a row to the indicator table, ask: what task does the operator perform that
  requires this information? If the answer is vague, the row does not have a strong claim to
  display space.
- Error prevention over error messaging. If an operator might misread a label, fix the label,
  do not add a tooltip explaining the label's nuance.
- Consistency is slip prevention, not just learnability. Before changing the layout of a tab,
  ask what expert procedures the change will break.

From the genre theory:

- The tool responds to a specific recurring situation. Periodically ask whether that
  characterization is still accurate, or whether the operators are now using it for something
  different. Tools drift away from their genre as usage patterns evolve.
- Every output format is a genre with a downstream community. `--json` output is not neutral;
  it makes claims about structure that downstream tools will depend on. Changing the schema
  breaks the genre contract.

From information theory:

- Sort by information content, not by insertion order or alphabetical convenience.
- Passing indicators are low entropy. Findings are high entropy. High entropy items should
  appear earlier in any scan path.
- The goal is mutual information between system state and operator understanding, not
  information volume.

---

These materials were worth the time. I have a sticky note: "entropy-order the indicators."
It belongs on the board next to the others.

— Rusty

