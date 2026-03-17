# UMRS Documentation Theme Implementation
## For: The Imprimatur (Senior Tech Writer)
## Type: Implementation — CSS, Partials, and Antora Supplemental UI
## Prerequisite: Documentation restructure plan approved and complete

---

## Mission

Replace the default Antora UI theme with the UMRS wizard theme. The theme
has been designed and approved. Your job is to implement it precisely in
Antora's supplemental UI system without breaking the build or the content.

**Do not modify any content files.** This task touches only:
- `docs/supplemental-ui/css/custom.css`
- `docs/supplemental-ui/partials/`
- `docs/supplemental-ui/img/`
- `antora-playbook.yml` (UI bundle reference only)

---

## Design Brief

The UMRS theme is derived from the project banner image and wizard motif.
The brand identity is: **serious craft project** — rigorous, precise, and
built with genuine love for the craft. Not corporate. Not a toy.

### The wizard motif
The project uses a hooded wizard figure as its primary mark. The wizard
represents the UNIX magic tradition — profound complexity happening
invisibly, producing trustworthy results. The motif appears in:
- The site header (geometric wizard icon derived from the stacked-layers logo)
- Section decorations (subtle, not cartoonish)
- The collection home page tiles

### Color palette — dark theme (default)

```css
--umrs-bg-deep:      #020d05;   /* page background */
--umrs-bg-surface:   #040f07;   /* navigation panel */
--umrs-bg-panel:     #071a0a;   /* cards, active states */
--umrs-border-dim:   #1a3d1f;   /* subtle borders */
--umrs-border-mid:   #2d8a40;   /* emphasis borders */
--umrs-green-dim:    #1a5c28;   /* muted text, meta */
--umrs-green-mid:    #2d8a40;   /* secondary text */
--umrs-green-bright: #4ade6a;   /* primary accent, headings */
--umrs-green-glow:   #7fff8a;   /* page titles, active nav */
--umrs-text-primary: #a8c9aa;   /* body text */
--umrs-text-muted:   #6aab74;   /* secondary text */
--umrs-amber:        #EF9F27;   /* WARNING admonition only */
--umrs-amber-bg:     #130e03;   /* WARNING background */
--umrs-amber-dim:    #854F0B;   /* WARNING meta text */
```

### Color palette — light theme

```css
--umrs-bg-deep:      #f4f7f0;
--umrs-bg-surface:   #e8f0e3;
--umrs-bg-panel:     #edf2e8;
--umrs-border-dim:   #b8d4bc;
--umrs-border-mid:   #7ab87e;
--umrs-green-dim:    #4a7a54;
--umrs-green-mid:    #2d6b3a;
--umrs-green-bright: #0f5c38;
--umrs-green-glow:   #073d24;
--umrs-text-primary: #2d4a30;
--umrs-text-muted:   #4a7a54;
--umrs-amber:        #854F0B;
--umrs-amber-bg:     #faeeda;
--umrs-amber-dim:    #633806;
```

---

## Typography

```css
Body font:    Inter, system-ui, sans-serif
Code font:    JetBrains Mono, Courier New, monospace
Load from:   Google Fonts (Inter + JetBrains Mono weights 400, 500, 600)
```

### Type scale
```
Page title (h1):    22px / weight 600 / color --umrs-green-glow
Section head (h2):  16px / weight 600 / color --umrs-green-bright
Sub-head (h3):      13px / weight 600 / uppercase / letter-spacing 0.08em
Body text:          13px / weight 400 / line-height 1.7 / color --umrs-text-primary
Nav items:          12px / weight 400 / color --umrs-text-muted
Nav labels:         10px / weight 600 / uppercase / letter-spacing 0.12em
Code:               12px / weight 400 / line-height 1.6
Meta / tags:        10px / weight 400 / letter-spacing 0.06em
```

---

## Layout Fixes (beyond color)

These are the specific pain points in the default theme that must be fixed:

### Navigation
```css
/* Compact nav — kill the default excessive padding */
.nav-list .nav-item > .nav-link {
  padding: 4px 16px 4px 24px;
  font-size: 12px;
  line-height: 1.5;
}

/* Remove massive bullets — replace with thin left-border accent */
.nav-list .nav-item > .nav-link::before {
  display: none;  /* kill default bullet */
}

.nav-list .nav-item.is-current-page > .nav-link {
  border-left: 2px solid var(--umrs-green-bright);
  color: var(--umrs-green-glow);
  background: var(--umrs-bg-panel);
  padding-left: 22px;
}

/* Nav section titles */
.nav-title {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--umrs-green-mid);
  padding: 6px 16px 4px;
}
```

### Lists (content area)
```css
/* Fix massive default bullets */
.doc .ulist > ul {
  list-style: none;
  padding-left: 0;
  margin: 0 0 14px;
}

.doc .ulist > ul > li {
  padding: 3px 0 3px 14px;
  position: relative;
  font-size: 13px;
  line-height: 1.6;
}

/* Thin dash bullet — not a massive disc */
.doc .ulist > ul > li::before {
  content: '';
  position: absolute;
  left: 0;
  top: 11px;
  width: 6px;
  height: 1px;
  background: var(--umrs-green-mid);
}

/* Nested list indent */
.doc .ulist > ul > li > .ulist > ul > li {
  padding-left: 12px;
}

/* Ordered list */
.doc .olist > ol {
  padding-left: 20px;
  margin: 0 0 14px;
}

.doc .olist > ol > li {
  padding: 3px 0;
  font-size: 13px;
  line-height: 1.6;
  color: var(--umrs-text-primary);
}
```

### Spacing
```css
/* Reduce excessive default paragraph spacing */
.doc p {
  margin: 0 0 12px;
  font-size: 13px;
  line-height: 1.7;
  color: var(--umrs-text-primary);
}

/* Section spacing */
.doc .sect1 { margin-bottom: 28px; }
.doc .sect2 { margin-bottom: 20px; }

/* Kill default excessive margin on headings */
.doc h2 { margin: 24px 0 10px; }
.doc h3 { margin: 16px 0 8px; }
```

### Code blocks
```css
.doc pre {
  background: #020d05;
  border: 1px solid var(--umrs-border-dim);
  border-left: 3px solid var(--umrs-green-mid);
  border-radius: 4px;
  padding: 14px 16px;
  margin: 12px 0;
  font-family: 'JetBrains Mono', 'Courier New', monospace;
  font-size: 12px;
  color: var(--umrs-green-bright);
  line-height: 1.6;
  overflow-x: auto;
}

/* Language label */
.doc .listingblock .title {
  font-size: 10px;
  color: var(--umrs-green-mid);
  letter-spacing: 0.1em;
  text-transform: uppercase;
  margin-bottom: 6px;
}

/* Inline code */
.doc code {
  font-family: 'JetBrains Mono', monospace;
  font-size: 11px;
  color: var(--umrs-green-bright);
  background: var(--umrs-bg-panel);
  border: 1px solid var(--umrs-border-dim);
  padding: 1px 5px;
  border-radius: 3px;
}
```

---

## MIL-STD-38784A Admonition Blocks

Before implementing, verify the exact admonition types defined in
MIL-STD-38784A from the RAG corpus. Confirm whether IMPORTANT is defined
as a fourth type or implied. Map each type to the CSS treatment below.
If IMPORTANT exists, add a fourth block following the severity hierarchy.

**Severity hierarchy — visual weight encodes severity:**
Border weight + border color + background warmth all reinforce the level
simultaneously. The admonition type must be unambiguous at a glance.

### WARNING (highest severity)
MIL-STD definition: personal injury or death if procedure not observed.
```css
.doc .admonitionblock.warning {
  border-left: 4px solid var(--umrs-amber);
  background: var(--umrs-amber-bg);
  border-radius: 0 4px 4px 0;
  padding: 12px 16px;
  margin: 14px 0;
}

.doc .admonitionblock.warning .title,
.doc .admonitionblock.warning > table td.icon {
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--umrs-amber);
}

.doc .admonitionblock.warning > table td.content {
  font-size: 12px;
  color: #d4a96a;
  line-height: 1.65;
}
```

### CAUTION (medium severity)
MIL-STD definition: equipment or data damage if procedure not observed.
```css
.doc .admonitionblock.caution {
  border-left: 3px solid var(--umrs-green-bright);
  background: var(--umrs-bg-surface);
  border-radius: 0 4px 4px 0;
  padding: 12px 16px;
  margin: 14px 0;
}

.doc .admonitionblock.caution .title,
.doc .admonitionblock.caution > table td.icon {
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--umrs-green-bright);
}

.doc .admonitionblock.caution > table td.content {
  font-size: 12px;
  color: var(--umrs-text-primary);
  line-height: 1.65;
}
```

### NOTE (informational)
MIL-STD definition: general information the operator should know.
```css
.doc .admonitionblock.note {
  border-left: 2px solid var(--umrs-green-dim);
  border-top: 1px solid var(--umrs-border-dim);
  border-right: 1px solid var(--umrs-border-dim);
  border-bottom: 1px solid var(--umrs-border-dim);
  background: var(--umrs-bg-deep);
  border-radius: 0 4px 4px 0;
  padding: 10px 14px;
  margin: 14px 0;
}

.doc .admonitionblock.note .title,
.doc .admonitionblock.note > table td.icon {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.14em;
  text-transform: uppercase;
  color: var(--umrs-green-mid);
}

.doc .admonitionblock.note > table td.content {
  font-size: 12px;
  color: var(--umrs-text-muted);
  line-height: 1.65;
}
```

### IMPORTANT (if confirmed in MIL-STD-38784A)
If confirmed, implement between CAUTION and NOTE in severity. Use:
- Border: 3px solid between amber and green — suggest #7ab87e or teal
- Background: slightly warmer than NOTE, cooler than CAUTION
- Adjust after verifying the exact MIL-STD definition

---

## NIST Control Tags

Content pages use inline NIST/CMMC/RTB control annotations. Style these
as compact monospace pills so they are visually present but not disruptive:

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
  margin: 2px 2px 2px 0;
  white-space: nowrap;
}
```

In AsciiDoc, these are applied via a custom role:
```asciidoc
[.nist-tag]#AU-3# [.nist-tag]#SI-7# [.nist-tag]#NSA RTB#
```

---

## Header Partial

Update `docs/supplemental-ui/partials/header-content.hbs` to include:

1. The geometric wizard icon (SVG inline — see below)
2. The UMRS title and subtitle
3. Component navigation links across the top
4. Light/dark theme toggle button

### Wizard icon SVG (geometric, scalable to any size)
```svg
<svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg"
     width="32" height="32" aria-hidden="true">
  <polygon points="16,2 22,14 26,14 16,28 6,14 10,14"
           fill="#071a0a" stroke="#2d8a40" stroke-width="1"/>
  <ellipse cx="16" cy="14" rx="4" ry="4"
           fill="#040f07" stroke="#4ade6a" stroke-width="0.75"/>
  <circle cx="16" cy="14" r="1.5" fill="#4ade6a"/>
  <rect x="13" y="17" width="6" height="1" fill="#1a3d1f"/>
  <rect x="12" y="19" width="8" height="1" fill="#1a3d1f"/>
  <rect x="11" y="21" width="10" height="1" fill="#1a3d1f"/>
</svg>
```

The stacked horizontal lines below the diamond represent MLS sensitivity
layers — the geometric form does conceptual work, not just decoration.

---

## Footer Partial

Update `docs/supplemental-ui/partials/footer-content.hbs` to include:

```
UMRS · High-Assurance Unclassified MLS Reference System · MIT License
Unix Wizardry · Secure Systems · Cryptography
```

Keep it minimal. Dark background, muted green text. No clutter.

---

## Light/Dark Toggle

Implement via a CSS class toggle on the `<html>` element. Store the
preference in `localStorage` so it persists across pages.

```javascript
const toggle = document.querySelector('.theme-toggle');
const root = document.documentElement;
const stored = localStorage.getItem('umrs-theme') || 'dark';
root.setAttribute('data-theme', stored);

toggle.addEventListener('click', () => {
  const next = root.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
  root.setAttribute('data-theme', next);
  localStorage.setItem('umrs-theme', next);
});
```

CSS variables swap via `[data-theme="light"]` selector overrides.

---

## Collection Home Page Tiles

The collection home (`docs/umrs-home/modules/ROOT/pages/index.adoc`)
uses a tile grid layout. Style the tiles as cards with:
- Dark surface background
- Green left border accent
- Wizard component icon (small, 24px, one per component)
- Hover state: border brightens, slight background lift

```css
.tile-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 16px;
  margin: 24px 0;
}

.tile {
  background: var(--umrs-bg-surface);
  border: 1px solid var(--umrs-border-dim);
  border-left: 3px solid var(--umrs-green-mid);
  border-radius: 0 6px 6px 0;
  padding: 16px 18px;
  transition: border-color 0.15s;
}

.tile:hover {
  border-left-color: var(--umrs-green-bright);
  background: var(--umrs-bg-panel);
}

.tile h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--umrs-green-bright);
  margin: 0 0 6px;
}

.tile p {
  font-size: 12px;
  color: var(--umrs-text-muted);
  margin: 0;
  line-height: 1.6;
}
```

---

## Implementation Sequence

Do not implement everything at once. Follow this sequence, verifying
the build at each step:

1. **CSS variables and base colors** — apply palette to
   `custom.css`, verify build, check dark and light modes
2. **Typography** — font imports, type scale, heading colors
3. **Navigation** — compact padding, bullet removal, active state
4. **Content spacing** — paragraph, heading, list spacing fixes
5. **Code blocks** — terminal styling, inline code pills
6. **Admonitions** — MIL-STD hierarchy (verify corpus first)
7. **NIST tags** — inline control annotation pills
8. **Header partial** — wizard icon, title, component nav
9. **Footer partial** — minimal footer text
10. **Light/dark toggle** — localStorage persistence
11. **Collection home tiles** — card grid layout
12. **Final review** — build full site, check all five components,
    verify admonition rendering on a page that uses all three types

---

## Verification Checklist

Before declaring complete:

- [ ] Dark theme renders correctly in Chrome and Firefox
- [ ] Light theme renders correctly in Chrome and Firefox
- [ ] Theme preference persists across page navigation
- [ ] All three MIL-STD admonition types render with correct
      visual weight hierarchy
- [ ] Navigation has no excessive bullets or padding
- [ ] Code blocks feel terminal-like, not generic
- [ ] NIST tags render as compact inline pills
- [ ] Wizard icon appears in header at correct size
- [ ] Collection home tiles render in grid layout
- [ ] Site builds without warnings or errors
- [ ] All five components render consistently
- [ ] `_scratch/` and `new-stuff/` content is not affected
