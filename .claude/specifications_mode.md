# Specification Mode Rules

This mode is for **formal behavior descriptions** — almost like a lightweight
technical standard.

# Agent Mode: Specification Writing

## Purpose

Specification mode describes system behavior in a precise and testable way.

The goal is to define what a system must do.

Specifications must remove ambiguity.

Use this mode for:

- software specifications
- protocol definitions
- configuration schemas
- behavioral rules
- API contracts
- security requirements

---

# Writing Style

- Use formal language.
- Statements must be precise.
- Avoid conversational text.
- Avoid explanatory storytelling.
- Prefer declarative statements.

Example:

The system logs all authentication failures.

Do not write:

The system will usually log authentication failures.

---

# Requirement Keywords

Use requirement keywords consistently.

MUST  
MUST NOT  
SHALL  
SHALL NOT  
SHOULD  
MAY

Example:

The service MUST start after network initialization.

The system MUST NOT accept unsigned configuration files.

---

# Requirement Structure

Each requirement should contain one rule.

Example:

REQ-001  
The system MUST record all authentication failures.

REQ-002  
The audit log MUST include a timestamp.

REQ-003  
The system MUST reject invalid configuration files.

---

# Behavior Description

Describe observable system behavior.

Example:

When the service receives a configuration reload signal:

1. The service validates the configuration.
2. The service reloads parameters.
3. The service writes a log entry.

---

# Data Definitions

Define structured data precisely.

Example:

Configuration File

Field: service_port  
Type: integer  
Range: 1–65535

Field: log_level  
Type: string  
Allowed values: error, warning, info, debug

---

# Error Handling

Describe how the system reacts to failures.

Example:

If the configuration file is invalid:

1. The system MUST reject the configuration.
2. The system MUST log the error.
3. The running configuration MUST remain unchanged.

---

# Interface Definition

Define interfaces precisely.

Example:

API Endpoint

Method: POST  
Path: /audit/event

Required Fields:

- event_id
- timestamp
- source

---

# Agent Behavior

When Specification Mode is active the agent must:

1. Write deterministic requirements.
2. Remove vague language.
3. Use requirement keywords.
4. Define data structures precisely.
5. Assign identifiers to requirements when appropriate.
