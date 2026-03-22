---
name: Jamie's editorial direction on help text — onsite security officer framing
description: The tool surfaces findings and steers toward resolution; it does not dictate site procedure
type: project
---

Jamie's canonical statement (2026-03-22):

"We surfaced the finding, we steer them in the right direction. We don't dictate site
procedure. The tool is an onsite security officer pointing something out — not writing
the incident response plan."

**Operational implication for help text:**
- Write: "Check appropriate system logs to identify the cause."
- Do NOT write: "Run `ausearch -m MAC_CONFIG_CHANGE -ts today`."

Different sites have different SOPs. Named commands in help text imply a universal procedure
that does not exist. The tool directs attention; the operator's site procedures handle response.

**Why:** This is a deliberate design decision, not an oversight. Finn (guest-admin) flagged
DRIFT's lack of named commands as a HIGH finding. Jamie's response was to establish this
principle explicitly. The intern's finding surfaced the design principle into explicit policy.

**How to apply:** All outreach and blog content describing UMRS tools must honor this framing.
The "onsite security officer" metaphor is Jamie's canonical image for what UMRS tools do.
This is the same metaphor in the CUI enrichment vision notes. Use it. It is accurate, vivid,
and accessible to a wide audience including program managers and government buyers.

**For Sage specifically:** When writing about UMRS tool capabilities for any audience, lead
with the "onsite security officer" frame. Not a scanner. Not a hardening tool. Not a
compliance checker. An officer who notices something, points it out clearly, and trusts
the site to know its own procedures.
