# Intel CET Specification Summary

**Source:** Intel CET Specification Document 334525-003

## Architecture Overview

Control-flow Enforcement Technology (CET) provides two complementary mechanisms to defend
against control-flow hijacking attacks (ROP, JOP, COP):

### 1. Shadow Stack (CET-SS)
- Hardware-maintained second stack that stores only return addresses
- On `CALL`: CPU pushes return address to both normal stack and shadow stack
- On `RET`: CPU compares return address from both stacks
- Mismatch triggers `#CP` (Control Protection) fault — immediate process termination
- Shadow stack pages have special page-table attribute (dirty bit = 0, writable = 1)
- Cannot be written by normal memory instructions — only by `CALL`/`RET` and `WRSS`/`WRSSD`/`WRSSQ`

### 2. Indirect Branch Tracking (CET-IBT)
- Requires `ENDBR32`/`ENDBR64` instruction at every valid indirect branch target
- CPU enters "tracker" state on indirect `JMP`/`CALL`
- Next instruction MUST be `ENDBR{32,64}` or `#CP` fault is raised
- `ENDBR64` opcode: `F3 0F 1E FA` (4 bytes)
- Legacy NOP on non-CET processors — backward compatible

## CPUID Detection

| Feature | CPUID Leaf | Register | Bit |
|---------|-----------|----------|-----|
| Shadow Stack (SHSTK) | EAX=07H, ECX=0H | ECX | bit 7 |
| Indirect Branch Tracking (IBT) | EAX=07H, ECX=0H | EDX | bit 20 |

## MSR Registers

| MSR | Address | Purpose |
|-----|---------|---------|
| `IA32_U_CET` | 0x6A0 | User-mode CET configuration |
| `IA32_S_CET` | 0x6A2 | Supervisor-mode CET configuration |
| `IA32_PL3_SSP` | 0x6A7 | Ring 3 Shadow Stack Pointer |
| `IA32_PL2_SSP` | 0x6A6 | Ring 2 Shadow Stack Pointer |
| `IA32_PL1_SSP` | 0x6A5 | Ring 1 Shadow Stack Pointer |
| `IA32_PL0_SSP` | 0x6A4 | Ring 0 Shadow Stack Pointer |

## `#CP` Error Codes

| Code | Meaning |
|------|---------|
| 1 | NEAR_RET — shadow stack return address mismatch |
| 2 | FAR_RET/IRET — far return/iret mismatch |
| 3 | ENDBRANCH — missing ENDBR at indirect branch target |
| 4 | RSTORSSP — shadow stack restore token mismatch |

## UMRS Relevance

- **NIST SP 800-53 SI-16** (Memory Protection) — CET provides hardware-enforced CFI
- **NSA RTB RAIN** (Non-Bypassability) — hardware enforcement, not software
- CET binary verification is a Layer 2 software utilization check: hardware present (Layer 1) but binaries not compiled with `-fcf-protection=full` = capability unused

## Sources

- [Intel CET Specification 334525-003](https://kib.kiev.ua/x86docs/Intel/CET/334525-003.pdf)
- [A Technical Look at Intel CET](https://www.intel.com/content/www/us/en/developer/articles/technical/technical-look-control-flow-enforcement-technology.html)
- [Intel CPUID Enumeration and Architectural MSRs](https://www.intel.com/content/www/us/en/developer/articles/technical/software-security-guidance/technical-documentation/cpuid-enumeration-and-architectural-msrs.html)
