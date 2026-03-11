# Secure Rust Guidelines — Introduction (ANSSI)

Source: https://anssi-fr.github.io/rust-guide/
GitHub: https://github.com/ANSSI-FR/rust-guide
Retrieved: 2026-03-10

---

This document serves as a comprehensive guide for developing secure applications in Rust. The content emphasizes that while Rust provides strong memory safety guarantees through its ownership system, developers must understand how to properly utilize language features to maintain security.

## Overview

Rust is described as a "multi-paradigm language with a focus on memory safety." The guide acknowledges that despite Rust's built-in protections against data races and memory errors, certain constructs can introduce vulnerabilities if misused.

## Target Audience

The guidelines are designed for application developers with "strong security level requirements," though anyone seeking to preserve Rust's safety guarantees may benefit from the recommendations.

## Document Structure

The guide is organized around different development phases:
- Rust ecosystem tools and secure practices
- External library selection and usage
- Language construct recommendations
- Established library guidance

**Note:** Async Rust is not currently addressed.

## Notation System

Recommendations use specific terminology:
- **"Must"** indicates prescriptive requirements for higher security
- Multiple solution options are presented with varying security levels
- Recommendations are periodically reassessable as part of ongoing risk management

## GitHub Repository

https://github.com/ANSSI-FR/rust-guide — open to contributions

---

NOTE: Individual chapter content is fetched separately. See anssi-chapter-*.md files.
