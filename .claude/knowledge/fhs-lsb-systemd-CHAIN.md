# FHS / systemd file-hierarchy / LSB / systemd UIDS-GIDS -- Unified Compliance Chain

**Author:** Knox (security-auditor)
**Date:** 2026-04-18 (updated from original 2026-04-18 trio; FHS 3.0 and systemd file-hierarchy added)
**Purpose:** Single reference explaining how FHS 3.0, systemd file-hierarchy(7), LSB 5.0,
and systemd UIDS-GIDS interlock as a compliance chain for UMRS filesystem and account decisions.

---

## The Four Documents and Their Roles

| Document | Role | Authority Type |
|---|---|---|
| FHS 3.0 (2015) | Filesystem hierarchy layout -- /opt package canonical standard | Formal standard (Linux Foundation) |
| systemd file-hierarchy(7) (RHEL 10, systemd 257) | RHEL 10 runtime layout, state/cache semantics, systemd unit directives | De facto standard (systemd project), RHEL 10 ground truth |
| LSB 5.0 (2015) | UID/GID allocation ranges + required accounts | Formal standard (Linux Foundation) |
| systemd UIDS-GIDS (2025) | Modern UID range refinement + NSS requirements | De facto standard (systemd project) |

Plus the implementation layer:

| Document | Role | Authority Type |
|---|---|---|
| RHEL 10 /etc/login.defs | Machine-executable UID/GID configuration | Ground truth for useradd -r behavior |

No single document is sufficient. Each fills gaps the others leave:

- **FHS 3.0** defines where files go for /opt packages. Says nothing about who owns them
  numerically, or about systemd unit directives.
- **systemd file-hierarchy(7)** extends FHS 3.0 for systemd-managed systems. Adds state/cache
  separation, RuntimeDirectory=, node-type discipline. Does NOT cover /opt packages
  (defers to FHS for those). Is the RHEL 10 ground truth for runtime behavior.
- **LSB 5.0 section 23** defines who can be (numerically). Says nothing about login shells,
  home directories, or password locking.
- **systemd UIDS-GIDS** bridges LSB's 499 gap to 999, defines NSS resolvability requirements,
  and specifies modern container UID ranges (including SUB_UID_MIN = 524288).
- **RHEL 10 /etc/login.defs** converts all four into machine-executable configuration that
  useradd -r actually reads.

---

## The Compliance Chain: UMRS Path Decisions

```
Question: Where does UMRS install its files?

FHS 3.0 section 3.13 (/opt)
  /opt is reserved for add-on application software packages
  Static files go in /opt/package (package form, no LANANA registration needed)
    |
/opt/umrs/ -- static binaries, data, assets

FHS 3.0 section 3.7.4 (/etc/opt)
  Host-specific configuration must be in /etc/opt/subdir
  where subdir matches the /opt subtree name
    |
/etc/opt/umrs/ -- configuration files

FHS 3.0 section 5.12 (/var/opt)
  Variable data of /opt packages must be in /var/opt/subdir
    |
/var/opt/umrs/ -- logs, state, vault data
  Internal subdivision (systemd file-hierarchy convention):
    /var/opt/umrs/state/ -- persistent state
    /var/opt/umrs/cache/ -- regenerable cache
    /var/opt/umrs/log/ -- logs
    /var/opt/umrs/vaults/ -- UMRS-specific vault data

FHS 3.0 section 3.13.2 exception clause + NIST CM-6/SA-8
  Key material must reside in specific locations to function properly
  (IMA/EVM tooling uses well-known paths under /etc/keys/, /etc/ima/)
    |
/etc/keys/umrs/ -- IMA/EVM cryptographic trust anchors
  (deliberately outside /opt hierarchy, independently hardened with SELinux labels)

FHS 3.0 section 3.15 (/run) + systemd file-hierarchy(7) RUNTIME DATA
  /run is the canonical runtime directory on RHEL 10
  systemd RuntimeDirectory=umrs creates /run/umrs/ automatically at service start
    |
/run/umrs/ -- runtime PID files and UNIX domain sockets (if needed)
```

---

## The Compliance Chain: UMRS Account Decisions

```
Question: What UID should the umrs system account receive?

LSB 5.0 section 23.3
  "UIDs 100-499 should be reserved for dynamic allocation
   by system administrators and post install scripts using useradd"
    | (extends and refines)
systemd UIDS-GIDS
  "1-999 are system users; 999/1000 boundary strongly recommended"
  "System users must be resolvable without network"
    | (implements)
RHEL 10 /etc/login.defs
  SYS_UID_MIN=201, SYS_UID_MAX=999
  useradd -r allocates from this range
    | (execution)
useradd -r -s /sbin/nologin umrs
  Allocates UID from 201-999
  Places entry in local /etc/passwd
  No home directory (server/daemon convention)
  Locked password
    | (compliance result)
UMRS umrs account is compliant with all four sources
```

---

## The Compliance Chain: Required Account Name Validation

```
LSB 5.0 section 23.2 Table 23-1 (required, must exist):
  root, bin (legacy), daemon (legacy)

LSB 5.0 section 23.2 Table 23-2 (optional, reserved for distributions):
  adm, lp, sync, shutdown, halt, mail, news, uucp, operator, man, nobody

UMRS account name: "umrs"

Verdict: "umrs" does not appear in either table.
  No name conflict with required or optional LSB accounts.
  Compliant.
```

---

## Where Each Document Defers to Another

| Source asks about | Defer to |
|---|---|
| Where to put /opt package files | FHS 3.0 sections 3.13, 3.7.4, 5.12 |
| Runtime directory creation on RHEL 10 | file-hierarchy(7) + RuntimeDirectory= directive |
| State vs cache vs log semantics | file-hierarchy(7) (extends FHS; FHS has no subdivision guidance) |
| Who runs the files (numerically) | LSB section 23 + systemd UIDS-GIDS |
| What numeric UID to use on RHEL 10 | RHEL 10 login.defs |
| What happens if UID > 65535 | systemd UIDS-GIDS (container ranges) |
| What happens if UID >= 2^31 | systemd UIDS-GIDS (HIC SVNT LEONES -- avoid) |
| SELinux labels on paths | Not in any of these four; see selinux-rules skill |
| Cryptographic key material location | Not in these four; NIST CM-6/SA-8 governs; FHS section 3.13.2 provides exception authorization |

---

## Key Tensions Across the Chain

### T1: LSB 100-499 vs systemd 1-999 vs RHEL 201-999

The single most important tension. Resolved by the four-source chain.
- LSB: 100-499 (weak "should")
- systemd: 1-999 (de facto hard boundary)
- RHEL 10: 201-999 (implementation)
- A umrs UID in the 201-499 range satisfies all sources. A UID in 500-999 satisfies systemd
  and RHEL 10 but is technically outside LSB's stated recommendation.

**Audit guidance:** If an auditor challenges a UID > 499, cite systemd and RHEL 10 login.defs.
LSB uses "should" not "shall" -- the constraint is not absolute.

### T2: /var/run vs /run

FHS 2.3 specified /var/run. FHS 3.0 section 3.15 formalizes /run as the primary directory.
section 5.13 demotes /var/run to a compatibility symlink. file-hierarchy(7) confirms /var/run/
points to /run/. RHEL 10 implements this. Use /run/umrs/ for all new work.

### T3: /opt package variable data -- /var/opt/ vs /var/lib/

file-hierarchy(7) Table 2 recommends /var/lib/package/ as the primary state slot -- but this
applies to /usr/ system packages. UMRS is an /opt package; FHS section 5.12 is authoritative.
Use /var/opt/umrs/ as the root, subdivided following systemd conventions internally.

### T4: /etc/opt/package/ (FHS) vs /etc/package/ (file-hierarchy Table 2)

Same scope boundary as T3. file-hierarchy Table 2 /etc/package/ slot applies to /usr/ packages.
UMRS uses /etc/opt/umrs/ per FHS section 3.7.4.

---

## Skill Drift: Corrections Applied 2026-04-18

All items listed in the prior CHAIN.md "Skill Drift" table have been corrected. Summary:

| Previous Skill Claim | Reality | Status |
|---|---|---|
| "FHS 3.0 paths" without corpus | FHS 3.0 now in corpus | Resolved -- 3.0 acquired and familiarized |
| "FHS 3.8 (/etc/opt)" | /etc/opt is FHS section 3.7.4; section 3.8 is /home | Corrected in skill |
| "FHS 3.13 (/opt)" | Correct | Confirmed |
| "FHS 5.12 (/var/opt)" | Correct | Confirmed |
| "FHS 4.5 vs NIST CM-6/SA-8" for /etc/keys | section 4.5 is /usr/include; wrong citation | Corrected -- cite section 3.13.2 exception + NIST |
| SUB_UID_MIN = 100000 | RHEL 10 value is 524288 | Corrected in skill |

---

## NIST Control Mapping (from compliance report -- validated)

| Control | Relevance | Four-source basis |
|---|---|---|
| NIST SP 800-53 AC-2 | Account Management | useradd -r creates traceable account |
| NIST SP 800-53 AC-6 | Least Privilege | no login, no shell, no home, locked password |
| NIST SP 800-53 CM-6 | Configuration Settings | UID range follows four-source guidance |
| NIST SP 800-53 IA-5 | Authenticator Management | password locked, no interactive auth |
| CMMC AC.L2-3.1.1 | System Access Authorization | non-interactive service account |
