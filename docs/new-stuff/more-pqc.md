# RAG Query
- During last work, I saw an error:
```
usage: ingest.py [-h] [--collection COLLECTION] [--force] [--summary]
                      [--drop-collection NAME]
     ingest.py: error: unrecognized arguments: --source
     /media/psf/repos/umrs-project/.claude/references/nist-pqc/
```
- Does the query.py need a "source" option? If so, update the query.py as you see fit.


# Post Quantum Crptography

- researcher, your task is to ingest more post-quantum-cryptography information.
  - You've already downloaded the NIST documents 203, 204, and 205 into he NIST collection.
  - Make it available via the RAG, Does it need it's own PQC collection? 
  - If searching a PQC related items, it should get this new data as well as the NIST documents.
- Senior-tech-writer and tech-writer, we need post-quantum-cryptography information under our cryptography
  - It should dsicuss the emergence of the PQC
  - The three new ones are implemented to possible replace existing ciphers/algorithms
  - Identify that mapping and make sure develoeprs are aware of this in the documentation
- Once ingested, notify the team of the resource available. 


## Download URLs
researcher, download and ingest this data into the new PQC collection.

- https://blog.cloudflare.com/nists-first-post-quantum-standards/
- https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards
- https://www.hklaw.com/en/insights/publications/2024/08/nist-releases-three-post-quantum-cryptography-standards
- https://www.serverion.com/uncategorized/nist-standards-for-post-quantum-cryptography/
- https://csrc.nist.gov/projects/post-quantum-cryptography
- https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards
- https://www.sectigo.com/blog/who-are-nists-post-quantum-algorithm-winners
- https://www.wolfssl.com/what-are-fips-203-204-and-205/
- https://www.serverion.com/nn/uncategorized/nist-standards-for-post-quantum-cryptography/
- https://terraquantum.swiss/news/diving-into-nists-new-post-quantum-standards/
- https://cloudsecurityalliance.org/blog/2024/08/15/nist-fips-203-204-and-205-finalized-an-important-step-towards-a-quantum-safe-future
- https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization
