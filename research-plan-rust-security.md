# Research Plan: Rust Security Corpus

## Objective

Collect authoritative Rust security and secure coding documents into
`.claude/references/rust-security/` for RAG ingestion. Do not ingest
automatically — place files and await review.

---

## Directory Structure to Create

```
.claude/references/rust-security/
├── secure-coding/
├── unsafe-rust/
├── supply-chain/
├── compiler-security/
├── fuzzing/
└── crypto/
```

---

## Documents to Retrieve

### secure-coding/

**ANSSI Secure Rust Guidelines**
- URL: https://anssi-fr.github.io/rust-guide/
- Action: Fetch each chapter page and save as .md or .html
- Alt: Clone https://github.com/ANSSI-FR/rust-guide and copy docs/
- Priority: HIGHEST — most authoritative structured secure coding standard for Rust

---

### compiler-security/

**Rust Compiler Exploit Mitigations**
- URL: https://doc.rust-lang.org/rustc/exploit-mitigations.html
- Action: Fetch page, save as exploit-mitigations.html
- Topics: stack canaries, ASLR, RELRO, PIE, control flow protections

**Reproducible Rust Builds**
- URL: https://reproducible-builds.org/docs/rust/
- Action: Fetch page, save as reproducible-builds.html

---

### unsafe-rust/

**The Rustonomicon**
- URL: https://doc.rust-lang.org/nomicon/
- Action: Fetch each chapter, save individually as .html files
- Topics: aliasing rules, raw pointer safety, FFI, soundness

**Unsafe Code Guidelines**
- URL: https://github.com/rust-lang/unsafe-code-guidelines
- Action: Clone repo or fetch Reference/ directory markdown files
- Topics: memory model, pointer provenance, layout guarantees

---

### supply-chain/

**RustSec Advisory Database README**
- URL: https://github.com/RustSec/advisory-db
- Action: Fetch README.md and advisories/crates/ index
- Note: Full advisory DB is large — fetch README and a sample only

**cargo-audit documentation**
- URL: https://github.com/RustSec/rustsec
- Action: Fetch README.md

**cargo-deny documentation**
- URL: https://github.com/EmbarkStudios/cargo-deny
- Action: Fetch README.md and docs/ if available

**cargo-crev documentation**
- URL: https://github.com/crev-dev/cargo-crev
- Action: Fetch README.md

---

### fuzzing/

**Rust Fuzz Book**
- URL: https://rust-fuzz.github.io/book/
- Action: Fetch each chapter, save as .html files
- Topics: libFuzzer, AFL integration, coverage guided fuzzing

---

### crypto/

**RustCrypto Overview**
- URL: https://github.com/RustCrypto
- Action: Fetch organization README and key crate READMEs

**Cryptographic Right Answers**
- URL: https://latacora.micro.blog/2018/04/03/cryptographic-right-answers.html
- Action: Fetch page, save as cryptographic-right-answers.html

---

## After Retrieval

1. Report a manifest of files downloaded with sizes and locations
2. Flag any URLs that returned errors or redirected unexpectedly
3. Do NOT run ingest.py — await review before ingestion
4. Notify when ready for review

---

## Notes

- Prefer .md source files over rendered HTML where available
- For GitHub repos: fetch raw markdown files, not rendered pages
- Do not download binary releases, compiled artifacts, or large datasets
- The ANSSI guide is the highest priority — complete it first


