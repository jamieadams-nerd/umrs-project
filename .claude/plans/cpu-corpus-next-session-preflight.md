# CPU Corpus Plan — Next Session Preflight Checklist

**Created:** 2026-03-18
**Purpose:** Ensure subagent write permissions work before launching any background agents.

## Problem Statement

Subagent researcher agents cannot write files despite `Write(.claude/**)` in settings.json.
The permission sandbox doesn't honor relative or absolute path Write permissions for subagents.
This caused all Phase 1C/1D/1E agents to complete research but fail on file output — wasting
significant tokens on relaunches.

## Root Cause Hypothesis

Subagents resolve paths differently from the main thread. The `Write(.claude/**)` rule
uses a relative glob that doesn't match when subagents use absolute paths. Adding
`Write(/media/psf/repos/umrs-project/.claude/**)` also failed because `/media/psf/` mount
is not traversable by subprocesses (per CLAUDE.md path rules).

## Preflight Steps (do ALL before launching any agent)

### 1. Test subagent write permissions
Launch a trivial test agent that writes a single file:
```
Agent: researcher (or any type)
Prompt: "Write the text 'test' to /media/psf/repos/umrs-project/.claude/references/cpu-extensions/test.md using the Write tool. If Write fails, try Bash with tee. Report which method worked."
```
If BOTH fail, escalate to Jamie — we need a different approach (e.g., agents write to /tmp/
and main thread copies files).

### 2. Pre-create ALL output directories
```bash
mkdir -p .claude/references/cpu-extensions/vector
mkdir -p .claude/references/cpu-extensions/confidential-computing
mkdir -p .claude/references/cpu-extensions/mitigations
mkdir -p .claude/references/cpu-extensions/entropy-bigint
mkdir -p .claude/references/cpu-extensions/access-controls
mkdir -p .claude/references/cpu-extensions/virt-reliability
mkdir -p .claude/references/cpu-extensions/proc-crypto
```

### 3. If subagent writes still fail — fallback plan
Have agents write to `/tmp/cpu-corpus-<phase>/` and copy results:
```bash
cp /tmp/cpu-corpus-1c/* .claude/references/cpu-extensions/vector/
cp /tmp/cpu-corpus-1d/* .claude/references/cpu-extensions/confidential-computing/
# etc.
```

## Work Queue for Next Session

### Priority 1: Write files for completed research (main thread)
These phases have research done. Write files from the agent result summaries:
- **1C** — 4 files → `.claude/references/cpu-extensions/vector/`
- **1D** — 12 files → `.claude/references/cpu-extensions/confidential-computing/`
- **1E** — 11 files → `.claude/references/cpu-extensions/mitigations/`

The agent result summaries are in this conversation's context. Key data points:

**1C sources:** Intel ISE Programming Reference, AVX-512 Wikipedia, OpenSSL OPENSSL_ia32cap docs,
Travis Downs AVX-512 frequency analysis, Cloudflare frequency scaling blog, KVM CPU flags blog

**1D sources:** Intel SGX spec, AMD SEV developer docs, kernel snp-tdx-threat-model.rst,
CacheWarp CVE-2023-20592, SGX Foreshadow/SGAxe/CrossTalk/LVI/AEPIC, TDX Google-Intel audit

**1E sources:** Intel SDM speculative chapters, Linux hw-vuln docs, vulnerability sysfs interface,
existing SignalId::Mitigations and SignalId::Pti connections

### Priority 2: Launch new research phases
All of these are unblocked:
- **1B** — Entropy & Big Integer (800-90B acquired)
- **1F** — CPU Access Controls (CET docs acquired)
- **1G** — Virt, Reliability, Topology
- **1H** — /proc/crypto & Software Utilization

### Priority 3: If all above complete
- **1I** — Matrix Synthesis (needs 1A-1H)
