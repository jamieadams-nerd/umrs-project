# Audit Event Architecture Survey — What the Old School Got Right

**Author:** Jamie + Claude (research synthesis)
**Date:** 2026-04-02
**Purpose:** Understand how mature systems structure audit events, what "envelopes" look like across standards, and what UMRS should steal.

---

## 1. The Irreducible Audit Record (CC FAU_GEN)

Common Criteria Part 2, Section 8.3 defines the absolute minimum every audit record must contain. FAU_GEN.1.2 is the law:

> **The TSF shall record within the audit data at least the following information:**
> a) Date and time of the auditable event
> b) Type of event
> c) Subject identity (if applicable)
> d) Outcome (success or failure)

That's it. Four fields. Everything else is optional context bolted onto this skeleton. CC calls these the **irreducible audit minimum** — if your event doesn't have these four, it isn't an audit record, it's a log line.

**FAU_GEN.1.1** adds the audit level selection: the system must support at least *minimum, basic, detailed, not specified* levels of auditing. This means the event catalog must be tiered — not every event needs every field, but every event needs the four.

**FAU_GEN.2** (User Identity Association) requires that events from identified users carry the user's identity. This is a separate component because not every event has a user — kernel events, timer events, and hardware faults don't.

**Key insight for UMRS:** Our `SecurityObservation` enum already carries type and outcome. What's missing is a formal envelope that wraps it with timestamp and subject identity. We have the content; we lack the container.

---

## 2. The Telecom Alarm Model (3GPP TS 28.111 / ITU-T X.721 / X.733)

This is where your experience lives, Jamie. The telecom world built a rigorous fault management taxonomy 30+ years ago and it still runs every network on earth.

### The Fault → Error → Failure Chain

3GPP Section 3.1 defines the causal chain precisely:

| Term | Definition | Key property |
|---|---|---|
| **Fault** | The hypothesized cause (defect in design, software, external interference) | May be invisible |
| **Error** | A state different from the correct system state, caused by a fault | Has begin and end time |
| **Failure** | Inability to deliver correct service as specified | Always externally observable |
| **Alarm** | Management representation of a fault/error/failure requiring attention | Has state (active/cleared/acknowledged) |
| **Event** | An instantaneous occurrence (config change, threshold crossing, state transition) | Does NOT have state |

This is the critical distinction UMRS is missing: **alarms have state, events do not.** An alarm is raised, acknowledged, cleared. An event happened and is done. UMRS currently treats everything as events (SecurityObservation). Some of our observations — like "SELinux is permissive" — are actually ongoing conditions that should be modeled as alarms with lifecycle.

### Alarm Identifying Attributes (the "envelope")

3GPP Section 6.4 defines the four attributes that uniquely identify an alarm:

1. **objectInstance** — what managed object is affected (DN path)
2. **alarmType** — which domain (from X.733's five types, below)
3. **probableCause** — what's likely wrong (from a standardized catalog)
4. **specificProblem** — refinement of probable cause (vendor/system-specific)

These four together are the alarm's identity. Two alarms with the same four values are the same alarm. This is how deduplication works at scale — you don't compare free text, you compare structured identity tuples.

### X.733's Five Alarm Types

ITU-T X.733 (referenced by 3GPP) defines exactly five alarm domains:

| Alarm Type | Meaning | UMRS Equivalent |
|---|---|---|
| COMMUNICATIONS_ALARM | Transport/connectivity fault | Network posture (future) |
| PROCESSING_ERROR_ALARM | Software or processing fault | Tool execution errors, parsing failures |
| EQUIPMENT_ALARM | Hardware fault | CPU extension missing, hardware degradation |
| ENVIRONMENTAL_ALARM | Environmental condition (power, temperature) | Not applicable (we're software) |
| QUALITY_OF_SERVICE_ALARM | Service degradation | Posture drift, compliance gap |

Plus X.733 defines **perceived severity**: Critical, Major, Minor, Warning, Cleared, Indeterminate.

### X.721's Managed Object Classes

X.721 defines the inheritance hierarchy for log records. This is the "old school rigid way":

```
top
 └── logRecord (logRecordId, loggingTime)
      ├── eventLogRecord (managedObjectClass, managedObjectInstance, eventType)
      │    ├── alarmRecord (probableCause, perceivedSeverity, ...)
      │    ├── attributeValueChangeRecord (attributeValueChangeDefinition)
      │    ├── objectCreationRecord
      │    ├── objectDeletionRecord
      │    ├── relationshipChangeRecord
      │    ├── securityAlarmReportRecord (securityAlarmCause, securityAlarmSeverity, 
      │    │                               securityAlarmDetector, serviceUser, serviceProvider)
      │    └── stateChangeRecord (stateChangeDefinition)
      └── (other record types)
```

Every record inherits `logRecordId` and `loggingTime` from `logRecord`. Every event record adds `managedObjectClass`, `managedObjectInstance`, and `eventType`. Then each specific record type adds its domain-specific attributes.

**X.721's securityAlarmReportRecord** is particularly relevant — it has five mandatory attributes: cause, severity, detector (who/what detected it), serviceUser (who was affected), and serviceProvider (what system/service).

### 3GPP Alarm Lifecycle

3GPP Section 6.14 defines the alarm record lifecycle states that UMRS should adopt:

- **Raised** → alarm created, `ackState = unacknowledged`
- **Updated** → severity or attributes changed while active
- **Acknowledged** → operator confirms awareness (`ackUserId`, `ackTime`, `ackSystemId` recorded)
- **Cleared** → condition no longer present (`perceivedSeverity = cleared`)
- **Commented** → operator adds notes (timestamped, append-only, immutable once written)

The acknowledgment pattern is exactly what the UMRS Roadmap already describes for the Event Viewer. 3GPP got there first.

### Alarm Correlation

3GPP Section 6.10: Multiple errors from a single fault produce correlated alarms. The `correlatedNotifications` attribute links child alarms to a root cause. The `rootCauseIndicator` boolean marks which alarm is the root.

**UMRS application:** When a kernel posture probe finds SELinux permissive AND module loading enabled AND FIPS disabled, these are three symptoms of one root problem (insecure kernel configuration). Today they're three independent observations. They should be correlated.

---

## 3. The IDMEF Event Structure (RFC 4765)

IDMEF is an intrusion detection format, but its data model is the best-documented multi-actor event structure in the security domain.

### The Two Message Types

IDMEF has exactly two top-level messages:

1. **Alert** — something happened that needs attention
2. **Heartbeat** — the analyzer is alive and reporting

Heartbeats are underappreciated. They solve the "silence problem" — if you stop receiving heartbeats, the analyzer is down. UMRS tools currently have no heartbeat concept. A tool that silently fails produces no events, and the operator doesn't know posture data is stale.

### Alert Envelope Structure

Every IDMEF Alert contains:

| Component | Cardinality | Purpose |
|---|---|---|
| **Analyzer** | Exactly 1 | Who detected this (tool identity, version, host) |
| **CreateTime** | Exactly 1 | When the alert was created |
| **Classification** | Exactly 1 | What this alert is (name + reference) |
| **DetectTime** | 0..1 | When the event was detected (may differ from create time) |
| **AnalyzerTime** | 0..1 | Current time at the analyzer (clock sync verification) |
| **Source** | 0..* | Who/what caused the event (multi-actor!) |
| **Target** | 0..* | Who/what was affected (multi-target!) |
| **Assessment** | 0..1 | Impact + confidence + recommended action |
| **AdditionalData** | 0..* | Extension fields |

The **three timestamps** (CreateTime, DetectTime, AnalyzerTime) are brilliant. In distributed systems, detection, recording, and transmission happen at different times. Recording only one timestamp loses forensic information.

### Multi-Actor Model

IDMEF's Source and Target are both composite objects containing:
- **Node** — network identity (name, address)
- **User** — human identity (userId, category)
- **Process** — software identity (name, pid, path)
- **Service** — network service (name, port, protocol)

An event can have multiple sources (coordinated attack) and multiple targets (broadcast vulnerability). UMRS events are currently single-source, single-target.

### Alert Subclasses

- **ToolAlert** — alert generated by a specific tool (carries tool name + command)
- **CorrelationAlert** — alert that aggregates multiple child alerts (carries alert references)
- **OverflowAlert** — specific to buffer overflow events

### Assessment Classes

- **Impact** — severity (info, low, medium, high), completion (failed, succeeded, unknown), type (admin, dos, file, recon, user, other)
- **Action** — what was done in response (notification, block, allow, etc.)
- **Confidence** — how sure is the analyzer (numeric rating)

**Key insight for UMRS:** Our SecurityObservation has kind (Risk/Warning/Good) but no confidence level and no recommended action. IDMEF's Assessment model is richer.

---

## 4. CEF — Practical Normalized Field Naming

ArcSight CEF is the pragmatic counterweight to the theoretical models above. It's a flat key-value format designed for SIEM ingestion.

### The CEF Header (the minimal envelope)

```
CEF:Version|Device Vendor|Device Product|Device Version|Device Event Class ID|Name|Severity|[Extension]
```

Seven pipe-delimited fields. Every CEF event has all seven. The Extension field is free-form key=value pairs.

**Key fields:**
- `deviceEventClassId` — unique identifier per event type (the "signature ID"). This is what correlation engines match on. Not the name, not the description — the class ID.
- `name` — human-readable description, explicitly must NOT contain information from other fields
- `agentSeverity` — Unknown, Low, Medium, High, Very-High (or 0-10 integer)

### The Extension Dictionary

CEF defines ~100 predefined key names organized by role:
- `src`, `dst` — source/destination addresses
- `suser`, `duser` — source/destination user names  
- `act` — action taken
- `cat` — device event category (hierarchical: "/Monitor/Disk/Read")
- `msg` — human-readable message
- `cnt` — event count (for aggregated events)
- `rt` — receipt time (when the device received the event)
- `end`, `start` — event duration boundaries

**Key insight for UMRS:** CEF's category hierarchy (`cat`) is a practical taxonomy. Instead of free-text event names, events belong to a hierarchical category tree. This is what makes filtering work at scale. UMRS needs this — something like `/Posture/Kernel/SELinux`, `/Integrity/File/Xattr`, `/Custody/C2PA/Ingest`.

---

## 5. DFXML — Provenance and Forensic Object Modeling

Garfinkel's DFXML is about representing forensic artifacts with provenance. Less about events, more about the objects events act upon.

### Core Design Principle

Every DFXML file embeds a `<creator>` element that records:
- Program name and version
- Build environment (compiler, libraries)
- Execution environment (OS, hostname, architecture, user, start time, command line)

This is **tool provenance** — not just "what was found" but "what found it, under what conditions." This is directly applicable to UMRS: every posture report should carry the tool version, the system it ran on, and the exact command that produced it.

### The FileObject Model

DFXML's `<fileobject>` carries: filename, filesize, partition, allocation status, creation/modification/access times, byte runs (physical disk location), and multiple hash digests (md5, sha1, sha256). The multi-hash approach parallels what we just did with SHA-256 + SHA-384.

### Metadata Annotations

DFXML adopts Dublin Core metadata (DCMI) for annotating entire files or individual objects: publisher, abstract, acquisition date, classification. The `<classification>` tag is explicitly called out as a way to denote security classification of disk images.

**Key insight for UMRS:** DFXML's approach to provenance (embed the tool identity, build info, and runtime environment into every output) is exactly what UMRS assessment output should carry. It's the difference between "this file has these hashes" and "these hashes were computed by this tool version on this system at this time with these libraries."

---

## 6. CASE Ontology — Chain of Custody and Relationships

CASE (Cyber-investigation Analysis Standard Expression) is the newest entry. It's an OWL/RDF ontology under the Linux Foundation's Cyber Domain Ontology project.

### Core Concepts

- **Observable Object** — anything being investigated, characterized by **Facets** (attributes)
- **Facets** represent data sources (devices, media), digital objects (files, messages, multimedia), and logs
- Built on UCO (Unified Cyber Ontology) which models items, tools, people, places, and actions
- Chain of custody is first-class: who handled data, when, where
- Chain of evidence is first-class: what tools processed the data, how

### Data Markings

Data markings are inherent in CASE — controlling access to privileged, proprietary, and personal information. This maps directly to UMRS CUI labeling.

### Provenance at Every Phase

CASE maintains provenance through the entire investigation lifecycle. Every Observable Object traces back to its originating data source through an unbroken chain.

**Key insight for UMRS:** CASE's relationship-first model (every object has provenance, every action has an actor, every result traces to a tool) is the right mental model for UMRS audit events. Our events shouldn't just describe what happened — they should describe who did what with what tool on what object producing what result, all linked.

---

## 7. ITU-T X.721 — The Historical Foundation

X.721 (1992) is where it all started. It defines the GDMO (Guidelines for the Definition of Managed Objects) templates that 3GPP, X.733, and X.736 build on.

### The Type Hierarchy

X.721's notification types (Section 13) define the complete taxonomy:

| Notification | Domain |
|---|---|
| attributeValueChange | Configuration management |
| communicationsAlarm | Fault management (transport) |
| environmentalAlarm | Fault management (physical) |
| equipmentAlarm | Fault management (hardware) |
| integrityViolation | Security management |
| objectCreation / objectDeletion | Lifecycle management |
| operationalViolation | Security management |
| physicalViolation | Security management |
| processingErrorAlarm | Fault management (software) |
| qualityOfServiceAlarm | Performance management |
| relationshipChange | Configuration management |
| securityServiceOrMechanismViolation | Security management |
| stateChange | State management |
| timeDomainViolation | Security management |

Look at the security-specific ones: **integrityViolation**, **operationalViolation**, **physicalViolation**, **securityServiceOrMechanismViolation**, **timeDomainViolation**. These map almost perfectly to UMRS posture concerns:

- integrityViolation → IMA/EVM check failure, file hash mismatch
- operationalViolation → SELinux permissive when policy says enforcing
- securityServiceOrMechanismViolation → FIPS disabled, module signing bypassed
- timeDomainViolation → Clock skew, NTP failure (affects audit timestamp reliability)

### The SecurityAlarmReportRecord

X.721 Section 6.11 — five mandatory attributes that are purpose-built for security events:
- **securityAlarmCause** — what caused it
- **securityAlarmSeverity** — how bad
- **securityAlarmDetector** — what found it (the sensor identity)
- **serviceUser** — who was using the service when it happened
- **serviceProvider** — what service/system was affected

This is a tighter, more security-focused version of IDMEF's Source/Target model.

---

## 8. Event Groupings ("Envelopes")

Every standard uses a layered envelope model, but they call it different things:

| Standard | Outer envelope | Event identity | Domain-specific payload |
|---|---|---|---|
| CC FAU_GEN | timestamp + subject + outcome | event type | per-component "relevant information" |
| 3GPP/X.733 | alarm identifying attrs (4-tuple) | alarmType + probableCause | perceived severity + specific problem + conditional packages |
| IDMEF | IDMEF-Message wrapper | Alert/Heartbeat class | Analyzer + Classification + Source/Target + Assessment |
| CEF | Syslog prefix + CEF header (7 fields) | deviceEventClassId | Extension key-value pairs |
| DFXML | `<dfxml>` + `<creator>` | per-element type tag | element-specific attributes |
| CASE/UCO | Observable Object + Facets | object type + investigation context | facet-specific properties + provenance chain |
| X.721 | logRecord (id + time) → eventLogRecord (class + instance + type) | notification type | per-notification attributes + conditional packages |

The common pattern across all of them:

```
┌─────────────────────────────────────────┐
│  ENVELOPE (transport + routing)         │
│  - unique ID                            │
│  - timestamp(s)                         │
│  - source/emitter identity              │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  EVENT IDENTITY                   │  │
│  │  - event type / class ID          │  │
│  │  - severity                       │  │
│  │  - outcome (success/failure)      │  │
│  │                                   │  │
│  │  ┌─────────────────────────────┐  │  │
│  │  │  DOMAIN PAYLOAD             │  │  │
│  │  │  - subject (who/what)       │  │  │
│  │  │  - object (acted upon)      │  │  │
│  │  │  - action (what happened)   │  │  │
│  │  │  - evidence (supporting)    │  │  │
│  │  │  - assessment (so what)     │  │  │
│  │  └─────────────────────────────┘  │  │
│  └───────────────────────────────────┘  │
│                                         │
│  ┌───────────────────────────────────┐  │
│  │  PROVENANCE                       │  │
│  │  - tool identity + version        │  │
│  │  - build/runtime environment      │  │
│  │  - correlation links              │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

---

## 9. How the Old School Rigid Way Benefits UMRS

### What rigidity buys you

1. **Machine-processable events.** When every event has a typed class ID, severity, and outcome, you can filter, correlate, aggregate, and trend without parsing free text. UMRS already does this with `SecurityObservation` enum variants — but only within a single tool run. Across tools, across time, across systems: we need the envelope.

2. **Alarm lifecycle for persistent conditions.** UMRS posture drift (SELinux mode change, FIPS disabled) should not be modeled as instantaneous events. They are ongoing conditions with raise/acknowledge/clear lifecycle. 3GPP's alarm model is the right fit. An operator should be able to "acknowledge" that they know SELinux is permissive on this host, and that acknowledgment is itself an auditable event.

3. **Correlation across tools.** When `umrs-ls` finds a file with broken xattrs and `umrs-c2pa` finds a broken chain of custody on the same file, those are correlated observations. Without a correlation ID linking them, an operator sees two independent findings. With correlation, they see one problem.

4. **Deduplication.** The 3GPP alarm identity tuple (object + type + cause + specificProblem) prevents duplicate alarms from flooding the operator. UMRS posture probes that run on a schedule will produce the same finding every time until it's fixed. Without deduplication, the alarm list grows without bound.

5. **Heartbeat/liveness.** IDMEF's heartbeat concept solves the silent failure problem. If `umrs-state` runs on a cron schedule and misses a run, the absence of a heartbeat is itself a finding. No heartbeat infrastructure → no way to detect tool failure.

6. **Forensic provenance.** DFXML's `<creator>` and CASE's provenance chain ensure that every finding traces back to the specific tool version, on the specific host, at the specific time. This is what auditors need. "The system was compliant" is a claim. "umrs-uname v0.4.2, compiled with GCC 14.2 on RHEL 10, run by uid 0 at 2026-04-02T14:30:00Z, found these 37 indicators" is evidence.

7. **Event categorization hierarchy.** CEF's hierarchical `cat` field and X.721's notification type taxonomy both prove that a flat list of event names doesn't scale. UMRS needs a category tree: `/Security/Posture/Kernel`, `/Security/Integrity/File`, `/Custody/C2PA`, `/Configuration/SELinux`. This is what makes event filtering, routing, and dashboarding work.

### What rigidity costs you

1. **Schema evolution is painful.** Adding a new event type in X.721 means defining a new managed object class with GDMO templates. In Rust, we'd define a new enum variant — but consumers (event viewers, SIEM connectors, OSCAL exporters) all need to understand the new type.

2. **Not every event fits the model.** CC's FAU_GEN minimum works for security events but feels heavy for debug-level operational telemetry. The answer is tiered auditing: FAU_GEN.1.1's minimum/basic/detailed levels. Debug telemetry doesn't need the full envelope.

3. **Impedance mismatch with journald.** systemd-journald is a flat key-value store with some structure (boot_id, unit, priority). Mapping a rich event envelope to journald fields requires a convention layer. Not hard, but it's glue code that must be maintained.

### What UMRS should do

Don't adopt any single standard wholesale. Take the best ideas:

- **From CC FAU_GEN:** The irreducible minimum (timestamp, type, subject, outcome). Every UMRS event carries these four, no exceptions.
- **From 3GPP/X.733:** The alarm lifecycle (raise/acknowledge/clear) for persistent conditions. The five alarm types adapted for our domain. The severity scale.
- **From IDMEF:** Three timestamps (detect, create, receipt). Multi-actor Source/Target. The Assessment model (impact + confidence + action). Heartbeats.
- **From CEF:** Hierarchical event category tree. The deviceEventClassId concept (stable machine identifier per event type, separate from human-readable name).
- **From DFXML:** Tool provenance in every output (version, build, runtime environment).
- **From CASE:** Relationship-first modeling. Every object has provenance. Data markings as a first-class concept.
- **From X.721:** The security notification taxonomy (integrityViolation, operationalViolation, etc.). The SecurityAlarmReportRecord's five-attribute model.

Build the UMRS event catalog as a typed Rust enum with these properties. Don't store events — that's journald's job. Don't deliver events — that's journald + rsyslog's job. Define the **structure** of events so that every tool in the UMRS family produces consistent, machine-processable, forensically traceable audit records.

That's what the old school got right: the structure comes first, the plumbing comes second.
