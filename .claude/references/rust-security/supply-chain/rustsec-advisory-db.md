# RustSec Advisory Database

Source: https://github.com/RustSec/advisory-db (README)
Retrieved: 2026-03-10

---

The RustSec Advisory Database is a centralized repository documenting security vulnerabilities in Rust crates published on crates.io. It serves as a critical resource for the Rust ecosystem's security infrastructure.

## Key Features

**Data Accessibility**: The database exports advisories in the OSV (Open Source Vulnerabilities) format, making data available through osv.dev and its API. Additionally, the GitHub Advisory Database imports these advisories for broader visibility.

**Integration Tools**: Several tools leverage this database for vulnerability auditing:
- cargo-audit and cargo-deny for auditing Cargo.lock files
- trivy for comprehensive vulnerability scanning
- dependabot for automated security updates via pull requests

## Advisory Submission Process

Community members can report vulnerabilities by submitting pull requests. The advisory format uses Markdown with TOML front matter, following a standardized schema that includes:

- Unique identifier (RUSTSEC-YYYY-NNNN format)
- Affected crate name
- Disclosure date
- Vulnerability categories and CVSS scores
- Patched version information
- Affected functions and platforms

## Licensing

Content is predominantly in the public domain under CC0-1.0, with the exception of advisories imported from GitHub's database, which use CC-BY-4.0 licensing and include proper attribution links to original sources.
