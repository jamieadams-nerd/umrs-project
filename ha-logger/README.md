
# High-Assurance Logger Project Concept

This project provides a high-assurance event logging library (the “logger”) for security-critical systems. A primary goal is to make it very hard for developers to log the wrong thing, or forget mandatory security fields. The library forces a consistent, structured event format that is suitable for audits, forensics, and formal security evaluations.

The logger is written in Rust, but the design is language-neutral. The same ideas could be applied to C, C++, or other languages.

### What is a high-assurance system?

A high-assurance system is a system where:
* Security is a primary mission requirement, not just a convenience.
* We need strong evidence that the system enforces its security policy correctly.
* We assume skilled, motivated attackers.
* We expect independent review or accreditation.

Examples include:
* Multi-level security systems that process Top Secret and Secret data on the same platform under strict policy.
* Critical command-and-control systems in defense or intelligence.
* Some safety-critical industrial systems where security failures could cause physical harm.

In this world, it is not enough that “things usually work.” We need evidence. We need traceability. We need to prove, or at least strongly argue, that:
* actions are authorized,
* policy is enforced,
* and security failures are detectable and diagnosable.


### Why a special logger is needed

Typical application logging libraries focus on:
* simple debug messages,
* performance logs,
* or business-level events.

They often allow completely free-form messages, and they do not enforce any security structure. In a high-assurance system, this is a problem:
* Developers might forget to log key security fields.
* Different components may log the same type of event in different formats.
* Important context (who, what, where, when, result, why) may be missing or incomplete.
* Forensic investigators must reverse-engineer the meaning of logs.
* It is hard to map logs to security requirements or to an audit policy.

In high-assurance systems, we want every security-relevant event to carry a minimum set of mandatory fields. These fields should be consistent across all components. We also want to separate:
* the event definition (what must be recorded),
* from the transport (syslog, database, file, remote collector).

This High-assurance logger addresses those needs and makes the event model easier to understand and easier to test.


### Summary of the project concept
High-assurance systems need more than simple logs; they need structured, mandatory, and consistent security events.
* The high-assurance logger provides:
  - a structured Event model with rich context: automatic capture of actor, process, and host information,
  - a simple EventLogger API with clear result semantics: a pluggable sink model, starting with SyslogSink.
* The design aligns with the spirit of RTB, NCDSMO, NIST 800-53 AU controls, and similar high-assurance guidance, without claiming to be an official reference implementation.
* Documentation and naming follow simplified technical English principles to make the system easier to understand, adopt, and evaluate.


# Core design principles
* Completeness enforced at compile time as much as possible.
  - Events must not be allowed to go out missing required fields like actor, operation, target, and result.

* Automatic collection of environment context where possible.
  - The library should gather things like loginuid, pid, executable path, hostname, SELinux context, etc., so each caller doesn’t have to.

* Simple high-level API.
  - Once you instantiate a logger with its static context, you can call methods like:
    - `logger.success(“updated configuration”, target_path)`
    - `logger.failure(“failed to update configuration”, target_path, error)`
    - `logger.error(“unexpected IO error while writing config”, details)`

* Pluggable sinks.
  - The actual serialization and transport of events should be pluggable:
    - syslog (authpriv, for example)
    - journald
    - flat file
    - SQLite
    - or a custom trait implementation

* Deterministic, minimal, and auditable.
  - Code should be small, with minimal dependencies, clear error handling, and no hidden network behavior.


## Event schema (what every event should contain)

Event identity and timing:
* `event_id           (string or 128-bit value)`
* `schema_version     (e.g., “1.0”)`
* `timestamp_utc      (ISO-8601 or similar)`

Actor (subject):
* `actor_login_uid    (u32 from /proc/self/loginuid)`
* `actor_user_name    (resolved from loginuid or SUDO_USER or uid)`
* `actor_uid          (real uid of process)`
* `actor_selinux_ctx  (optional: from /proc/self/attr/current)`
* `actor_role         (optional logical role, e.g. “admin”, “operator”)`

Process / environment:
* `process_pid        (pid)`
* `process_exe        (executable path)`
* `component_name     (e.g. “cds_admin_gui”, “guard_monitor”)`
* `host_name          (system hostname)`
* `system_domain      (logical domain, e.g. HIGH, LOW, ADMIN)`

Action:
* `operation          (string enum-like: “modify_config”, “start_guard”, “stop_guard”, “approve_transfer”, “login”, etc.)`
* `target_type        (e.g. “file”, “service”, “connection”, “user”, “policy”)`
* `target_identifier  (e.g. file path, service name, flow id)`
* `target_selinux_ctx (optional: SELinux label of object if relevant)`

Outcome:
* `result             (enum: SUCCESS, FAILURE, DENIED, ERROR, PARTIAL)`
* `reason_code        (optional numeric or string code)`
* `reason_text        (human-readable explanation, safe to display)`

Integrity / traceability:
* `sequence_number    (monotonic counter per process or per logger instance)`
* `originating_node   (optional; used if logs are aggregated)`
* `optional signature later (outside the library, if events are batched and signed)`

The library should manage as much of this as possible automatically.



## Library structure conceptually

Think of the library as having three main layers:
1. Context acquisition layer
   - Reads /proc/self/loginuid
   - Resolves uid to username
   - Reads SELinux context (if present)
   - Gets pid, executable path, hostname
   - Holds these in a static or long-lived struct

2. Event model + builder
   - Defines Event struct with all required fields
   - Provides a builder pattern that enforces required fields
   - Encodes result as a small enum (Success, Failure, Error, etc.)
  
3. Logger abstraction
  - An EventLogger struct instantiated once per component (or per process)
    - Holds:
      - context: environment and actor base info
      - component_name
      - system_domain
      - output sink (syslog/journal/file/…)
    - Exposes simple methods:
      - `success(operation, target_type, target_id, message)`
      - `failure(operation, target_type, target_id, reason_code, message)`
      - `error(operation, target_type, target_id, error_details)`
      - `custom(result, operation, target_type, target_id, reason, message, extra_fields)`


## Logger pattern: instantiate once, log many

Typical usage pattern:
1. At program startup:
    •    Construct an EventLogger:
    •    it automatically reads loginuid, uid, user name, SELinux context, pid, exe, hostname.
    •    you pass fixed things like component_name and system_domain.
    •    you configure the sink (syslog, journald, etc.).

2. During operations:
   - Instead of every function building all fields, they call:
```rust    
logger.success(“modify_config”, “file”, “/etc/myapp.conf”, “Updated parameter X”)
logger.failure(“modify_config”, “file”, “/etc/myapp.conf”, “EINVAL”, “Failed to parse new value”)
logger.error(“modify_config”, “file”, “/etc/myapp.conf”, format!(“IO error: {}”, e))
```

Internally, each of these methods:
* creates a new Event with:
* all static metadata from the logger context
* a new timestamp
* a new sequence number
* the operation, target_* and result passed in
* reason_code/text if applicable
* serializes the Event (e.g. JSON, key=value, or a custom concise format)
* hands it to the configured sink

This way, your CDS tools cannot forget to log “who did what to which config file”, because the library always attaches that info.



## Sinks (output backends)

Define a simple trait conceptually, for example:
```rust
trait EventSink {
    fn write(&self, event: &Event) -> Result<(), SinkError>;
}
```

You can then implement:
* SyslogSink (uses libc::openlog/syslog/closelog with LOG_AUTHPRIV or LOG_USER)
* JournalSink (writes to journald via its UNIX socket)
* FileSink (append-only, line-oriented)
* SQLiteSink (insert into a local DB for later signing and offload)
* CompositeSink (fan-out to multiple sinks)

The logger holds a boxed or generic sink and all events flow through that.



## Enforcing correctness and completeness

Things you can enforce at compile time or at least strongly encourage:

1. No “anonymous” events.
   * You do not expose a public Event struct with all fields public. Instead, all Events originate from the logger, which injects the mandatory metadata.

2. Builder for special/unusual events.
   * For cases that do not fit the success/failure/error helpers, you can have:
```rust
logger.event_builder()
    .operation(“custom_op”)
    .target(“resource”, “id123”)
    .result(Result::Partial)
    .reason(“PARTIAL_DATA”)
    .message(“Only part of the configuration was applied”)
.emit();
```

3. String enums and restricted constants.
Operation names, target types, and result codes can be strongly recommended via constants or small enums, so you avoid free-form vague text like “did stuff”.

4. Minimal dependencies and deterministic behavior.
No network, no random external calls, no heavy runtime reflection. If you need event_id randomness, use a deterministic, approved RNG (or rely on the system RNG that is already approved in your FIPS context).


# Example conceptual flow for a config-editing tool
Consider a tool that modifies a key configuration file. We must track changes to this configuration. 

1. At startup, your tool constructs:
```rust
let logger = EventLogger::new(“cds_config_tool”, “ADMIN_DOMAIN”, SyslogSink::new()?);
```

2. When a configuration change is initiated:
```rust
logger.success(
    “modify_config”,
    “file”,
    “/etc/myapp.conf”,
    “User requested update of parameter X”
);
```
3. If an error occurs:
```rust
logger.failure(
    “modify_config”,
    “file”,
    “/etc/myapp.conf”,
    “EINVAL”,
    “Validation error: invalid syntax for parameter X”
);
```

Internally, both result in Events that include:
* timestamp
* event_id
* loginuid
* username
* uid
* pid
* executable path
* component name
* host name
* operation = “modify_config”
* target_type = “file”
* target_identifier = “/etc/myapp.conf”
* result = SUCCESS or FAILURE
* reason_code and reason_text if applicable
* sequence_number

No caller can “forget” to include the actor or operation because the logger always adds them.


# Why this matches high-assurance expectations

Security guidance want:
* comprehensive audit coverage
* consistent event structure
* clear actor, action, and object
* predictable fields for forensic analysis
* tamper-evident or easily signable logs

This library:
* standardizes what an “event” looks like
* reduces per-component code and inconsistency
* makes it easy for reviewers to say “yes, every admin action uses cds_audit and therefore includes the required fields”
* improves forensic quality and reduces “para-audit” events (those half-useful logs engineers sprinkle around)



