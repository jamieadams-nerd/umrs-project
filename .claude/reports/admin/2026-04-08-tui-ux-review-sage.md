# TUI UX Review — umrs-ls, umrs-stat, umrs-label
**Reviewer:** Sage  
**Date:** 2026-04-08  
**Scope:** Pre-integration UX review ahead of embedding umrs-stat and umrs-label as views inside umrs-ls

---

## First Impression

This does not look like a student project. The header with hostname, OS, SELinux mode,
FIPS state, and the process domain — all on every frame — immediately signals that this
tool is aware of where it's running. That's the correct vibe for a security tool.

A CISO walking by would not need to ask what it does. The column headers (`IOV / MODE /
OWNER:GROUP / MODIFIED / NAME`), the palette-colored group headers in umrs-ls, and the
structured detail panel in umrs-label together say: "this tool has opinions about security
posture and it's going to show you."

The thing that could still read as "early" is density management. The header takes 9 rows.
On a 24-line terminal that's a big tax. On a 40-line terminal it's fine. Worth deciding
whether the header scales down on narrow terminals or stays fixed — inconsistent behavior
between machines is the kind of thing that erodes trust.

---

## Operator Flow

The analyst flow works. umrs-ls gives you a grouped view organized by SELinux type + marking
— which is exactly the mental model a security analyst brings. The cuddled sibling display
reduces noise without hiding data. Search is on `/`. Help is on `?`.

What's slightly rough: the key legend in the status bar is doing too much for its space.
`↑↓:nav  Enter:show  Tab:panel  /:search  ?:help  q:quit` is fine on 120 columns. On 80
it's going to truncate. The `?:help` overlay already carries the full reference — the status
bar legend should shrink to just the two or three things you'd forget mid-task. Something
like: `/ search  ? help  q quit`. Let the overlay earn its existence.

The Tab key context-switching between tree and detail panels in umrs-label is clean. The
dim/active border state change is unambiguous. No complaints there.

---

## Visual Hierarchy

The eye lands on the marking key first (`CUI//SP-CTI`, bold white, top of the detail panel).
That's correct. Everything else is subordinate. Designation color-coding (yellow for
specified, green for basic) in the detail panel is exactly the kind of no-explanation-needed
visual affordance that operators come to rely on.

One thing that catches slightly more attention than it should: the group headers in umrs-ls
use the `ICON_BANNER` triangle glyph as a styled segment separator, plus palette colors for
the marking portion. These are striking, which is appropriate — group changes in a security
context *are* important. But verify that on a busy directory with many distinct `(type,
marking)` pairs, the screen isn't mostly group headers and mostly noise. If a directory
has 20 groups of 2 files each, the signal-to-noise inverts. Consider whether a count
threshold should collapse sparse groups differently.

The wizard logo in the upper-right corner: it's character, not noise. It's 15 columns wide
and it's green in a field that's otherwise cyan/white/yellow. It reads like a "this is a
UMRS tool" brand mark, which is useful when you have multiple terminal panes open. Keep it.
Making it runtime-swappable is a nice idea — do it — but keep the default.

---

## Country Flag in Detail Panel

Good call removing the flag from the tree. In the tree, it would have competed with the
marking key text for scan attention and broken the scan rhythm. In the detail panel,
flush-right on the key header line, it's a small attribution signal that tells you "this
is a US CUI entry" without taking up a labeled row. It earns its two characters. Correct
placement.

---

## Color Palette Coherence

The palette is cohesive. Cyan for structure (borders, active tabs, key labels), white for
data values, green for trust-positive signals and the wizard, yellow for warnings and
"specified" designation, dim gray for inactive/unavailable. These do not fight each other.

The `list_selection` style — `rgb(160,145,95)` warm parchment with black foreground — is
the one element that could read as unexpected in a security tool context, but I actually
think it's right. It's distinct from cyan without being a competing color. The comment in
the code calling it "aged-parchment background" is accurate. It works.

What's currently absent but planned: the light palette stub. Flag this for the Canadian VM
acceptance pass — some government-issued terminals default to light-background
configurations. The `Theme::light()` returning `default_dark()` with a TODO comment is
honest about the gap. Just make sure it surfaces visibly in pre-deployment testing.

---

## Integration Vision

When umrs-stat and umrs-label views pop up from umrs-ls, they should feel like **panel
swaps with a breadcrumb trail**, not modal overlays and not full-screen replacements.

Here's why: an operator in umrs-ls is in a browsing headspace. They've built spatial
memory of the directory. A full-screen launch of umrs-stat destroys that context — when
they return to umrs-ls, they've lost their position. A centered modal overlay is better
but still dims the underlying context.

The pattern that would feel most natural — and is closest to what ranger and lazygit do —
is: Enter on a file opens umrs-stat as the **right-panel content**, not a new full screen.
The file listing shrinks to a narrow left column; the stat card fills the right 60-70%.
Backspace or Esc restores the full listing. This preserves location context and maps to
what operators expect from tools where you "drill in and come back."

For umrs-label from umrs-ls: Enter on a marking chip in the listing opens the label detail
in the right panel. The user is already in a listing context; showing the label detail
alongside is a natural extension, not an interruption.

If a full-screen child launch is necessary (e.g., the stat card needs the full width for
long SELinux context display), at minimum restore cursor position on return and show a
"returned from umrs-stat: /path/to/file" breadcrumb in the path bar for one tick.

---

## What Would Make Me Go "Oh, That's Nice"

One small touch: when a file has a required warning statement (the yellow field in the
label detail panel), surface a compact indicator in the umrs-ls group header or file row —
not the full text, just a glyph. `ICON_FLAG` is already defined and semantically appropriate.
An operator scanning a directory should be able to spot "there's a file here that carries
a required statutory warning" without opening the detail view. Right now that information
is only visible after navigation.

A second one: the search bar uses `PROMPT_ARROW` (➜) to signal input mode. Good. When
the search returns zero results, the bar could flash the border to yellow briefly — not
persistent, not alarming, just a one-frame "nothing matched" acknowledgment before the
display stabilizes. That's the difference between a tool that feels responsive and one
that feels like it's silently ignoring you.

---

## Summary

The foundation is solid. The visual language is coherent and correct for the audience.
The main pre-integration work item is the interaction model for the stat/label panel
launch — get that right before shipping and the whole thing will feel intentional rather
than bolted together. The panel-swap-with-breadcrumb approach is the recommendation.

The header density question and the status bar legend trimming are quality-of-life items,
not blockers. Flag them for the first round of operator feedback after initial integration.
