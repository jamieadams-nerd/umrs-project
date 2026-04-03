# Ensuring Secure, clean Rust Code

## Periodically, we must do the following:
- Check for crate updates?
- How do often do we perform a "cargo audit"?
  - Maintain RUSTSEC-AUDIT.md and update audit.toml in workspace.
- Execute: cargo deny check
  - update the deny.toml file. 
  - Update the RUSTSEC-AUDIT.md file if necessary
- Run cargo geiger on each package (not workspace)
  - Tell me about geiger
  - HOw does this help?
  - Does it help?
  - It reported 438 warnings in c2pa alone? Does that matter

- Let's try Cargo install cargo-fuzz
  - Tell me about cargo-fuzz. 
  - cargo fuzz init
  - cargo fuzz run <target>
  - What does do for us? Does it  help?




##-----------------------------------------------------------------------
## 1) Static Analysis *with different semantics* (not style/lints)

### ✔️ `cargo-audit`
- Checks dependencies against RustSec advisories
- This is *supply-chain assurance*, not code correctness

👉 Run in CI, fail builds on vuln:
```bash
cargo audit
```

---

### ✔️ `cargo-geiger`
- Flags **unsafe usage** (including transitive deps)
- Critical for your “no FFI / high assurance” model

```bash
cargo geiger
```

What you’re looking for:
- Any unexpected `unsafe`
- Especially in dependencies you implicitly trust

---

### ✔️ `cargo-deny`
- Enforces:
  - License policy
  - Dependency bans
  - Multiple-version resolution
  - Advisory checks (overlaps with audit, but stronger policy)

```bash
cargo deny check
cargo deny list 
```

This becomes your **policy gate**, not just a scanner.

---

## 2) Dynamic / behavioral analysis (this is where you’ll find “buried” issues)

### ✔️ Fuzzing (`cargo-fuzz`)
This is the **next level**.

You’re doing parsing, structured input, kernel ingestion → this is exactly where fuzzing shines.

```bash
cargo fuzz init
cargo fuzz run <target>
```

Focus fuzz targets on:
- parsers (JSON, SELinux contexts, kernel files)
- boundary readers (/proc, /sys ingestion)
- any dual-path parsing logic

**This will find things humans won’t.**

---

### ✔️ Sanitizers (ASan / UBSan / MSan)
Rust is memory safe—but:
- unsafe blocks
- FFI (even if minimized)
- dependencies

Use nightly:
```bash
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run
```

---

## 3) Property Testing (very underused, very powerful)

### ✔️ `proptest` or `quickcheck`

Instead of:
> “test known inputs”

You assert:
> “this invariant must *always* hold”

Example mindset:
- parsing → serialize → parse == identity
- category sets → ordering doesn’t change meaning
- normalization → idempotent

This is *perfect* for your typed SELinux model.

---

## 4) Formal-ish invariant checking (your domain is ideal for this)

You don’t need full formal verification, but:

### Add **explicit invariants** in code:
- CategorySet must never exceed 1024 bits
- MLS dominance rules must always hold
- SecurityContext parsing must be total or fail closed

Then:
- enforce with `debug_assert!` + test harnesses
- fuzz + property tests will hammer these

---

## 5) Miri (undefined behavior detector)

This is subtle and powerful:

```bash
cargo +nightly miri test
```

Catches:
- UB in unsafe code
- invalid aliasing
- subtle lifetime violations

---

## 10) One thing most teams miss

### 👉 **Negative testing discipline**

You should have tests that *prove failure paths*:

- malformed SELinux contexts → reject
- truncated kernel reads → reject
- invalid category ranges → reject
- conflicting evidence → flag

Most bugs live here—not in happy paths.

