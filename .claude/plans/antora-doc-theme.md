# Plan: UMRS Antora Documentation Theme

**Status:** Approved — ready for implementation

**ROADMAP alignment:** G8 (Human-Centered Design), G10 (AI Transparency — project identity)

**Source:** `.claude/jamies_brain/doc-theme/doc-theme.md` (Jamie Adams via Claude Chat)

---

## Mission

Replace the default Antora UI theme with the UMRS wizard theme. Dark-first, serious craft aesthetic — rigorous, precise, built with genuine love for the craft. Not corporate. Not a toy.

**Scope:** CSS, partials, and Antora supplemental UI only. No content file modifications.

---

## Design Identity

The UMRS brand is a **hooded wizard** — the Unix magic tradition. Profound complexity happening invisibly, producing trustworthy results. The geometric wizard icon uses stacked horizontal lines below a diamond to represent MLS sensitivity layers — the form does conceptual work, not just decoration.

---

## Color Palette

### Dark theme (default)

| Token | Hex | Usage |
|---|---|---|
| `--umrs-bg-deep` | `#020d05` | Page background |
| `--umrs-bg-surface` | `#040f07` | Navigation panel |
| `--umrs-bg-panel` | `#071a0a` | Cards, active states |
| `--umrs-border-dim` | `#1a3d1f` | Subtle borders |
| `--umrs-border-mid` | `#2d8a40` | Emphasis borders |
| `--umrs-green-dim` | `#1a5c28` | Muted text, meta |
| `--umrs-green-mid` | `#2d8a40` | Secondary text |
| `--umrs-green-bright` | `#4ade6a` | Primary accent, headings |
| `--umrs-green-glow` | `#7fff8a` | Page titles, active nav |
| `--umrs-text-primary` | `#a8c9aa` | Body text |
| `--umrs-text-muted` | `#6aab74` | Secondary text |
| `--umrs-amber` | `#EF9F27` | WARNING admonition only |
| `--umrs-amber-bg` | `#130e03` | WARNING background |
| `--umrs-amber-dim` | `#854F0B` | WARNING meta text |
| `--umrs-amber-muted` | `#8a6300` | CAUTION admonition border |
| `--umrs-caution-bg` | `#0f0c04` | CAUTION background |

### Light theme

| Token | Hex | Usage |
|---|---|---|
| `--umrs-bg-deep` | `#f4f7f0` | Page background |
| `--umrs-bg-surface` | `#e8f0e3` | Navigation panel |
| `--umrs-bg-panel` | `#edf2e8` | Cards, active states |
| `--umrs-border-dim` | `#b8d4bc` | Subtle borders |
| `--umrs-border-mid` | `#7ab87e` | Emphasis borders |
| `--umrs-green-dim` | `#4a7a54` | Muted text |
| `--umrs-green-mid` | `#2d6b3a` | Secondary text |
| `--umrs-green-bright` | `#0f5c38` | Primary accent |
| `--umrs-green-glow` | `#073d24` | Page titles |
| `--umrs-text-primary` | `#2d4a30` | Body text |
| `--umrs-text-muted` | `#4a7a54` | Secondary text |
| `--umrs-amber` | `#854F0B` | WARNING admonition |
| `--umrs-amber-bg` | `#faeeda` | WARNING background |
| `--umrs-amber-dim` | `#633806` | WARNING meta text |

Theme toggle via `data-theme` attribute on `<html>`, persisted in `localStorage`.

---

## Typography

| Element | Size | Weight | Extras |
|---|---|---|---|
| Page title (h1) | 22px | 600 | `--umrs-green-glow` |
| Section head (h2) | 16px | 600 | `--umrs-green-bright` |
| Sub-head (h3) | 14px | 600 | Uppercase, letter-spacing 0.08em |
| Body text | 13px | 400 | Line-height 1.7, `--umrs-text-primary` |
| Nav items | 12px | 400 | `--umrs-text-muted` |
| Nav labels | 10px | 600 | Uppercase, letter-spacing 0.12em |
| Code | 12px | 400 | Line-height 1.6 |
| Meta / tags | 10px | 400 | Letter-spacing 0.06em |

**h3 rationale:** h3 must be visually distinct from body text (13px). Uppercase + 1px size differential + weight 600 creates scannable hierarchy for auditors reviewing control mappings.

**Fonts:** Inter + JetBrains Mono (weights 400, 500, 600).

> **Network isolation concern:** The target deployment environment has no outbound network access. Google Fonts imports will silently fail.
>
> **Alternatives:**
> 1. **(Recommended) Vendor the fonts** — download Inter and JetBrains Mono WOFF2 files, place in `supplemental-ui/fonts/`, reference via `@font-face` in CSS. Zero network dependency. Auditors will not question it.
> 2. **System-ui fallback only** — design the type scale to look correct with `system-ui, -apple-system, sans-serif` and `ui-monospace, monospace`. Lower maintenance but less visual control.
> 3. **Progressive enhancement** — vendor fonts AND specify system-ui as fallback. Best of both worlds.
>
> **Recommendation:** Option 3 (progressive enhancement with vendored fonts). Vendor the WOFF2 files for reliable rendering, keep system-ui in the fallback stack for resilience.

---

## Key Layout Fixes

### Navigation
- Compact padding (4px vertical, kill default excessive spacing)
- Remove massive bullets — replace with thin 2px left-border accent on active item
- Section titles: 10px uppercase with letter-spacing

### Lists
- Thin dash bullets (6px × 1px line) instead of massive discs
- Tight spacing (3px vertical padding per item)

### Spacing
- Paragraph margin: 12px (down from default excess)
- Section spacing: sect1 28px, sect2 20px
- Heading margins tightened

### Code blocks
- Terminal-like: dark background (`#020d05`), green text, 3px left border accent
- Language label: 10px uppercase
- Inline code: pill-style with border

---

## Admonition Hierarchy (MIL-STD-38784B adapted)

Visual weight encodes severity — border weight, border color, and background warmth all reinforce the level simultaneously. Unambiguous at a glance.

**Two-register color system for admonitions:**
- **Warm tones** (amber family) = something may go wrong: WARNING (full amber `#EF9F27`), CAUTION (muted amber `#8a6300`)
- **Cool tones** (green family) = information/action: IMPORTANT (green-mid `#2d8a40`), NOTE (green-dim `#1a5c28`), TIP (green-dim `#1a5c28`, dashed border)

This ensures CAUTION reads as "degraded WARNING" not "elevated NOTE" — unambiguous at a glance.

| Level | Border | Background | Text Color | MIL-STD Meaning |
|---|---|---|---|---|
| **WARNING** | 4px solid `#EF9F27` (amber) | Warm dark `#130e03` | `#d4a96a` | Security breach, data loss, system damage |
| **CAUTION** | 3px solid `#8a6300` (muted amber) | `#0f0c04` (warm surface) | `--umrs-text-primary` | Recoverable degradation, compliance gap |
| **IMPORTANT** | 3px solid `#2d8a40` (green-mid) | `#040f07` (surface) | `--umrs-text-primary` | Required action, prerequisite |
| **NOTE** | 2px solid `#1a5c28` (green-dim) | `#020d05` (deep) | `--umrs-text-muted` | Supplementary context |
| **TIP** | 2px dashed `#1a5c28` (green-dim) | `#020d05` (deep) | `--umrs-text-muted` | Optional shortcut |

### Complete Admonition CSS Specifications

```css
/* WARNING — highest severity, warm amber */
.doc .admonitionblock.warning td.content {
  border-left: 4px solid var(--umrs-amber);
  background: var(--umrs-amber-bg);
  color: #d4a96a;
  padding: 12px 16px;
}
.doc .admonitionblock.warning td.icon {
  color: var(--umrs-amber);
  font-weight: 600;
}

/* CAUTION — degraded warning, muted amber */
.doc .admonitionblock.caution td.content {
  border-left: 3px solid var(--umrs-amber-muted);
  background: var(--umrs-caution-bg);
  color: var(--umrs-text-primary);
  padding: 12px 16px;
}
.doc .admonitionblock.caution td.icon {
  color: var(--umrs-amber-muted);
  font-weight: 600;
}

/* IMPORTANT — action required, green-mid */
.doc .admonitionblock.important td.content {
  border-left: 3px solid var(--umrs-green-mid);
  background: var(--umrs-bg-surface);
  color: var(--umrs-text-primary);
  padding: 12px 16px;
}
.doc .admonitionblock.important td.icon {
  color: var(--umrs-green-mid);
  font-weight: 600;
}

/* NOTE — supplementary context, green-dim */
.doc .admonitionblock.note td.content {
  border-left: 2px solid var(--umrs-green-dim);
  background: var(--umrs-bg-deep);
  color: var(--umrs-text-muted);
  padding: 12px 16px;
}
.doc .admonitionblock.note td.icon {
  color: var(--umrs-green-dim);
  font-weight: 400;
}

/* TIP — optional shortcut, green-dim dashed */
.doc .admonitionblock.tip td.content {
  border-left: 2px dashed var(--umrs-green-dim);
  background: var(--umrs-bg-deep);
  color: var(--umrs-text-muted);
  padding: 12px 16px;
}
.doc .admonitionblock.tip td.icon {
  color: var(--umrs-green-dim);
  font-weight: 400;
}
```

**Pre-implementation:** Verify MIL-STD-38784A/B admonition types from RAG corpus before finalizing IMPORTANT and TIP treatments.

---

## NIST Control Tags

Inline compact monospace pills — visually present but not disruptive.

```css
.nist-tag {
  display: inline-block;
  font-family: 'JetBrains Mono', monospace;
  font-size: 10px;
  color: var(--umrs-green-mid);
  background: var(--umrs-bg-surface);
  border: 1px solid var(--umrs-border-dim);
  padding: 2px 6px;
  border-radius: 3px;
}
```

AsciiDoc usage: `[.nist-tag]#AU-3# [.nist-tag]#SI-7#`

---

## Header

- Geometric wizard icon (SVG inline, 32px)
- UMRS title and subtitle
- Component navigation links across the top
- Light/dark theme toggle button

The wizard SVG uses a diamond shape with an inner eye (circle) and stacked horizontal bars representing MLS sensitivity layers.

> **Light theme compatibility:** Inline SVG must use CSS custom properties (`var(--umrs-green-mid)`, `var(--umrs-green-bright)`) for stroke and fill colors, not hardcoded hex values. This ensures the icon responds correctly to theme toggle.

> **Existing asset:** `supplemental-ui/img/wizard-logo.png` exists. Phase 8 must evaluate the PNG against the geometric SVG design intent. If the SVG is adopted, remove the unused PNG. If the PNG is kept, update the SVG specification to match. Do not ship both without a clear purpose for each.

### Light/Dark Toggle

> **JS delivery mechanism:** JavaScript cannot be placed in `custom.css`. Options:
> 1. Inline `<script>` in `footer-content.hbs` partial (lowest friction, no new files)
> 2. New `supplemental-ui/partials/body-scripts.hbs` partial
> 3. Override `site.js` from the default bundle (fragile across bundle updates)
>
> **Recommendation:** Option 1 — inline script in footer partial. Simple, reliable, no bundle coupling.

---

## Footer

Minimal: `UMRS · High-Assurance Unclassified MLS Reference System · MIT License`

Dark background, muted green text. No clutter.

---

## Collection Home Tiles

Card grid layout for the component landing page:
- `grid-template-columns: repeat(auto-fit, minmax(240px, 1fr))`
- Dark surface background with green left-border accent
- Hover: border brightens, slight background lift
- Each tile has a component name (h3) and one-line description

> **Scope clarification:** Tile layout requires AsciiDoc custom role markup (`[.tile-grid]`, `[.tile]`) in the content pages. This means Phase 11 touches content files, which is outside the CSS-only scope of this plan. **Options:**
> 1. **(Recommended) Defer Phase 11** to a follow-on task that includes content markup changes
> 2. Include the home page content markup in this plan's scope (amend the "no content files" constraint)
> 3. Use a Handlebars partial for the home page layout (template-level, not content-level)
>
> **Recommendation:** Defer to follow-on task. The tile CSS goes into `custom.css` now; the content markup is added when the collection home page is built.

---

## Pre-Implementation: Existing Theme Audit

The current `docs/supplemental-ui/css/custom.css` contains a complete working theme (steel-blue palette, light mode). This is a REPLACEMENT, not greenfield. Before Phase 1, audit the existing CSS and determine: replace wholesale or annotate overrides. Recommendation: **replace wholesale** — the color systems are incompatible and incremental override will produce selector conflicts.

---

## Implementation Sequence

Build at each step must pass before proceeding to next.

| Phase | What | Files |
|---|---|---|
| 1 | CSS variables + base colors (dark + light) | `custom.css` |
| 2 | Typography — font imports, type scale, heading colors | `custom.css` |
| 3 | Navigation — compact padding, bullet removal, active state | `custom.css` |
| 4 | Content spacing — paragraph, heading, list fixes | `custom.css` |
| 5 | Code blocks — terminal styling, inline code pills | `custom.css` |
| 6 | Admonitions — MIL-STD hierarchy (verify corpus first) | `custom.css` |
| 7 | NIST tags — inline control annotation pills | `custom.css` |
| 8 | Header partial — wizard icon, title, component nav | `partials/header-content.hbs` |
| 9 | Footer partial — minimal footer text | `partials/footer-content.hbs` |
| 10 | Light/dark toggle — localStorage persistence | `custom.css` + JS snippet |
| 11 | Collection home tiles — card grid layout | `custom.css` |
| 12 | Final review — full site build, all components, all admonition types | All |

---

## File Changes

| File | Change |
|---|---|
| `docs/supplemental-ui/css/custom.css` | Complete theme CSS (phases 1-7, 10-11) |
| `docs/supplemental-ui/fonts/` | Vendored Inter and JetBrains Mono WOFF2 files |
| `docs/supplemental-ui/partials/header-content.hbs` | Wizard icon, title, nav, toggle (phase 8) |
| `docs/supplemental-ui/partials/footer-content.hbs` | Minimal footer + inline theme toggle JS (phase 9-10) |
| `docs/supplemental-ui/img/` | Wizard SVG asset if not inlined |
| `antora-playbook.yml` | UI bundle reference (if needed) |

**No content files are modified.**

---

## Accessibility and Print

### Accessibility Requirements
- **Color independence:** Admonition severity must be distinguishable by border weight and label alone (not just color). This supports colorblind users and high-contrast mode.
- **`@media (prefers-contrast: high)`:** Override backgrounds to system colors; ensure text remains readable.
- **Screen readers:** Admonition icon cells (`td.icon`) must retain their text labels (WARNING, CAUTION, etc.). CSS must not hide these cells.
- **`@media print`:** White backgrounds, black text, visible borders. Dark theme backgrounds will waste toner and may produce unreadable output.
- **Reduced motion:** If shimmer or animation effects are added later, respect `@media (prefers-reduced-motion: reduce)`.

---

## Verification Checklist

- [ ] Dark theme renders correctly in Chrome and Firefox
- [ ] Light theme renders correctly in Chrome and Firefox
- [ ] Theme preference persists across page navigation
- [ ] All five admonition types render with correct visual weight hierarchy
- [ ] Navigation has no excessive bullets or padding
- [ ] Code blocks feel terminal-like, not generic
- [ ] NIST tags render as compact inline pills
- [ ] Wizard icon appears in header at correct size
- [ ] Collection home tiles render in grid layout
- [ ] `make docs` builds without warnings or errors
- [ ] All Antora components render consistently
- [ ] Admonition types distinguishable without color (border weight + label)
- [ ] Print stylesheet produces readable output with white backgrounds
- [ ] High-contrast mode does not hide text
- [ ] Screen reader labels preserved on admonitions

---

## Agent Assignment

**The Imprimatur (senior-tech-writer)** owns this implementation. CSS/HTML/HBS work — no Rust involved.

Pre-implementation dependency: verify MIL-STD-38784A/B admonition types from RAG corpus before phase 6.
