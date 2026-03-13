# RAG Collections — Source URLs for Update Checks

Last updated: 2026-03-13

## doc-structure (ingested 2026-03-12)

| Subdirectory | Source URLs |
|---|---|
| divio/ | https://docs.divio.com/documentation-system/ (redirected from https://documentation.divio.com/) |
| diataxis/ | https://diataxis.fr/ |
| antora/ | https://docs.antora.org/antora/latest/ |
| redhat-modular/ | https://redhat-documentation.github.io/ and https://redhat-documentation.github.io/modular-docs/ |
| write-the-docs/ | https://www.writethedocs.org/guide/ |
| google-style/ | https://developers.google.com/style |
| gitlab-docs/ | https://docs.gitlab.com/development/documentation/ |

## access-control (not yet ingested — awaiting user review)

See refs/manifest.md "Access Control Reference Collection" section for all source URLs.

## selinux-notebook

| Subdirectory | Source |
|---|---|
| selinux-notebook/20240430/ | https://github.com/SELinuxProject/selinux-notebook/releases/tag/20240430 |

## kernel-docs

Cloned from: https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git (Documentation/ subtree)
Version: Linux 6.x kernel tree

## linux-fhs-2.3

| File | Source URL |
|---|---|
| fhs-2.3.txt | https://refspecs.linuxfoundation.org/FHS_2.3/fhs-2.3.txt |

## nist (ingested 2026-03-12, 1,447 chunks)

Files ingested: sp800-171r2.pdf, sp800-171r3.pdf, sp800-171Ar3.pdf, sp800-218-ssdf.pdf, sp800-53r5.pdf,
fips140-2.pdf, fips140-3.pdf, plus previously ingested NIST.SP.800-160v1r1.pdf, NIST.SP.800-175Br1.pdf,
NIST.SP.800-185.pdf, NIST.SP.800-192.pdf, nist-sp-1800-44a-ipd.pdf, nistspecialpublication800-92.pdf

| File | Source URL |
|---|---|
| sp800-171r2.pdf | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171r2.pdf |
| sp800-171r3.pdf | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171r3.pdf |
| sp800-171Ar3.pdf | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-171Ar3.pdf |
| sp800-218-ssdf.pdf | https://nvlpubs.nist.gov/nistpubs/specialpublications/nist.sp.800-218.pdf |
| sp800-53r5.pdf | https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf |
| fips140-2.pdf | https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.140-2.pdf |
| fips140-3.pdf | https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.140-3.pdf |

## nist-pqc (ingested 2026-03-13, 285 chunks — expanded 2026-03-13 +21 chunks from two new web resources)

NIST Post-Quantum Cryptography FIPS standards plus supplementary web articles. FIPS published August 13, 2024.

### FIPS PDFs (nist-pqc/ root)

| File | Source URL |
|---|---|
| fips203.pdf | https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.203.pdf |
| fips204.pdf | https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.204.pdf |
| fips205.pdf | https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.205.pdf |

### Web Articles (nist-pqc/web/)

| File | Source URL | Notes |
|---|---|---|
| cloudflare-pqc-standards.md | https://blog.cloudflare.com/nists-first-post-quantum-standards/ | Harvest-now/decrypt-later, deployment status |
| nist-pqc-announcement-2024.md | https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards | Official NIST announcement |
| hklaw-pqc-standards-2024.md | https://www.hklaw.com/en/insights/publications/2024/08/nist-releases-three-post-quantum-cryptography-standards | Legal/policy context, replacement mapping |
| serverion-pqc-standards-en.md | https://www.serverion.com/uncategorized/nist-standards-for-post-quantum-cryptography/ | Migration timeline/performance table |
| serverion-pqc-standards-no.md | https://www.serverion.com/nn/uncategorized/nist-standards-for-post-quantum-cryptography/ | Norwegian translation |
| csrc-nist-pqc-project.md | https://csrc.nist.gov/projects/post-quantum-cryptography | NIST CSRC project page |
| csrc-nist-pqc-standardization.md | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization | Standardization process, FIPS 206 status (original fetch) |
| csrc-nist-pqc-standardization-2025.md | https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization | Updated 2026-03-13; includes HQC/FIPS 207 selection (March 2025), NIST IR 8545, transition timeline |
| wolfssl-fips-203-204-205.md | https://www.wolfssl.com/what-are-fips-203-204-and-205/ | Developer-focused, CNSA 2.0 notes |
| csa-fips-203-204-205-quantum-safe.md | https://cloudsecurityalliance.org/blog/2024/08/15/nist-fips-203-204-and-205-finalized-an-important-step-towards-a-quantum-safe-future | CSA Quantum-Safe Working Group |
| sectigo-pqc-algorithm-winners.md | https://www.sectigo.com/blog/who-are-nists-post-quantum-algorithm-winners | PKI context; JS-rendered (stub) |
| terraquantum-pqc-standards.md | https://terraquantum.swiss/news/diving-into-nists-new-post-quantum-standards/ | SLH-DSA technical detail; JS-rendered (stub) |
| redhat-quantum-safe-openshift-roadmap.md | https://www.redhat.com/en/blog/road-to-quantum-safe-cryptography-red-hat-openshift | Red Hat OpenShift PQC roadmap; TEST-PQ policy, FIPS 140 vs PQC trade-off, ML-KEM RHEL/OpenShift versions, hybrid KEM, JP Jung, May 2025 |

## rustdoc-book (ingested 2026-03-12, 194 chunks)

| File | Source URL |
|---|---|
| rustdoc-book.html | https://doc.rust-lang.org/rustdoc/print.html |

## asciidoctor-ref (ingested 2026-03-12, 67 chunks)

| File | Source URL |
|---|---|
| asciidoc-ref.html | https://docs.asciidoctor.org/asciidoc/latest/syntax-quick-reference/ |
| asciidoc-writer-guide.html | https://docs.asciidoctor.org/asciidoc/latest/document-structure/ |

## dita-spec (ingested 2026-03-12, 100 chunks)

OASIS DITA 1.3 Part 2 — Technical Content Edition (concept, task, reference, bookmap topic types).

| File | Source URL | Notes |
|---|---|---|
| dita-v1.3-part2-tech-content.html | https://docs.oasis-open.org/dita/dita/v1.3/os/part2-tech-content/dita-v1.3-os-part2-tech-content.html | Ingested |
| dita-v1.3-part2-tech-content.pdf | https://docs.oasis-open.org/dita/dita/v1.3/os/part2-tech-content/dita-v1.3-os-part2-tech-content.pdf | Downloaded but ingest fails (PDF reader compat issue); HTML version covers same content |
