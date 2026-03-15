# TUI Audit Card Template – Developer Feedback and Enhancement Instructions

## Purpose
- Improve the reusable TUI template used for **UMRS Audit Cards**.
- Ensure the interface communicates **system security posture quickly and clearly**.
- Prepare the template for multiple future audit cards (OS detection, file stat, logspace, process analysis, etc.).
- Support the **security auditor agent** as a primary consumer of this interface.

---

# Audit Card Concept

## Goal
- Audit cards are **posture summaries**, not troubleshooting tools.
- They communicate the **aggregate security condition** of a system.

## Operator expectation
- The operator should scan the screen and quickly determine:
  - overall security posture
  - whether the system appears healthy or suspicious
  - where deeper investigation is required.

## Example Audit Cards
- OS Detection Card
- Directory Security Card
- Running Process (and even Systemd)
- Logspace / Auditing Card
- File Stat Card

## File Stat Card Example Data
- File metadata
- owner / group
- permissions
- effective permissions of the viewing user
- MAC security context
- originating package
- provenance metadata
- Belongs to package
  - install date
  - package version
  - Package checksum/hash valid


## Security Auditor Role
- The **security auditor agent** is a prudent, detailed auditor will review audit cards.
- The interface must prioritize **clear security posture signals**.

---

# Template Layout

## Existing layout
- Header Bar
- Dynamic Data Area
- Status Bar

---

# Header Module (`header.rs`)

## Current issue
- The header is too minimal.
- Keep the text-art logo in the right corner
- The other header area to the left, It only displays:
  - report title
  - hostname
- possible new data in the header:
  - SELinux status
  - FIPS mode
  - active LSM (SELinux / AppArmor)
  - kernel lockdown mode
  - secure boot state
  - other critical posture flags

## Design rules
- These indicators should be **pre-populated by the template**.
- The header must remain **extensible**.

---

# Header Extensibility

## Requirement
`main.rs` must be able to modify or extend header information.
- Provide a **supplemental header area**.

---

# Evidence Tracking Tab

## Purpose
The evidence tab shows the **chain of trust and provenance** used to establish conclusions.

Example:
- OS detection trust validation
- kernel flag verification
- package ownership validation
- filesystem evidence

## Behavior
Evidence must be **grouped by subsystem**.

Example structure:

OS DETECTION
evidence items…

KERNEL SECURITY FLAGS
evidence items…

## Requirements
- Evidence must be **scrollable**.
- Evidence should use **table formatting if appropriate**.
- Use available horizontal space efficiently.

---

# Kernel Security Tab

## Requirement
Add placeholder support for a **second tab**.

### Tab structure
Tab 1:
- OS detection data

Tab 2:
- kernel security configuration

## Kernel tab will include (future work)
Examples:

- lockdown state
- module loading restrictions
- FIPS state
- secure boot
- kernel configuration security flags

## Evidence linkage
Each kernel item must have a **corresponding evidence chain entry**.

---

# Dynamic Data Layout

## Current problem
- The template uses a **single vertical stream** of data.
- This wastes terminal width.

## Required layout
The dynamic area must support **two columns**.

LEFT COLUMN        RIGHT COLUMN

⸻

OS identity        Kernel flags
SELinux state      Boot security
Package sources    Evidence signals

---

# Column Support

## Requirement
Rows must declare which column they belong to.

Example concept:

add_row(Column::Left, key, value)
add_row(Column::Right, key, value)

## Behavior
- Rendering system places rows in the specified column.
- Existing key/value styling functions should remain.

---

# Group Titles

## Purpose
Group titles organize related posture data.

## Visual design
Group titles must be **minimal and subtle**.

## Styling rules
- No boxes
- No heavy ASCII decoration
- Text-only styling

Possible attributes (theme controlled):

- bold
- subtle color
- dim
- underline

---

# Group Title Layout Rules

Group titles are **flush left**.

Group items are **indented one space**.

Example:

SELINUX
 mode: enforcing
 policy: targeted
 mls: enabled

KERNEL SECURITY
 lockdown: integrity
 module_loading: restricted

---

# Theme Integration

All styling must be controlled in:
- theme.rs

Examples:

- group title style
- key style
- value style

The theme must control:

- color
- boldness
- text attributes

Layout code must **not hardcode styling**.

---

# Evidence Display

Evidence chains should be:

- grouped
- readable
- scrollable

Use table format if appropriate.

Example conceptual format:

Evidence Type        Source            Verification

os-release           /etc/os-release   digest verified
package ownership    rpm database      validated
filesystem metadata  statx             confirmed

---

# Dialog
From main.rs, I'd like to be able to:
- Present a dialoge box in the center (horizontlly/vertically) in the screen. comprises of one line
  of text but stretched to an aesthetically please lenght. No stubby boxes only 10 chars?
  - This popup has the focus
  - I should be able to pass a message to get ehtir attention. Esc or something common to close it.
  - Smame message but itneractive. I can do two choice options: Y/N  or Cancel/Okay. 
  - give me some basic themes for style. Simple information, versus error, or soemthing security
    serious.

# Design Philosophy

The interface should feel like:

- a structured **security audit report**
- not a dashboard
- not decorative

## Priority
- readability
- information density
- posture comprehension

---

# Implementation Order

1. Enhance `header.rs` with security indicators.
2. Allow `main.rs` to extend header fields.
3. Implement two-column layout in dynamic area.
4. Add group title support.
5. Add theme-based styling for group titles.
6. Implement evidence tab grouping.
7. Add placeholder kernel security tab.

---

# Final Objective

The operator should be able to glance at an audit card and immediately understand:

- system identity
- security enforcement status
- kernel posture
- trust evidence used to determine these conclusions
