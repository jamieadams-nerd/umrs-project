SHA-256 and SHA-384 do not have distinct, individual code names like a secret military operation; however, they are part of a family developed under a larger umbrella project. [1, 2] 
## 1. The Capstone Project (The Umbrella)
The entire SHA family—including the original SHA (SHA-0), SHA-1, and the SHA-2 family (which includes 256 and 384)—originated from a massive U.S. government initiative known as the Capstone Project. [2, 3] 

* Purpose: Capstone was a multi-agency effort (led by the NSA and NIST) in the 1990s to develop a standardized suite of cryptographic tools.
* The Suite: This included the Skipjack encryption algorithm, the Digital Signature Algorithm (DSA), and the Secure Hash Algorithm (SHA). [2, 3, 4] 

## 2. SHA-2 (The Successor)
While the name "SHA-2" is now standard, it was essentially a "supersized" upgrade to SHA-1 that was developed when older 80-bit security standards became insufficient. [2] 

* SHA-256 and SHA-512: These were the primary new designs.
* SHA-384: This is technically a truncated version of SHA-512. It performs the exact same internal math as SHA-512 but discards the final bits to prevent specific types of cryptanalysis. [5, 6, 7] 

## 3. SHA-3 (The Competition)
If you're thinking of a "name" that sounds like a project title, you might be remembering Keccak.

* Keccak: This was the original name of the algorithm that won the NIST competition to become SHA-3.
* Unlike the SHA-2 family (designed by the NSA), SHA-3 was a public submission from an international team. [8, 9, 10] 

Since you are documenting your UMRS baseline, you might find it useful to refer to these algorithms by their FIPS 180-4 designations, as that is the official U.S. government standard that governs their use in high-assurance systems. [4, 11] 
Are you planning to mention the NSA origins of these algorithms in your software's "Security Considerations" section?
Proactive Follow-up: I can help you find the official FIPS 180-4 documentation if you need it for your LSB/FHS compliance references.

[1] [https://mojoauth.com](https://mojoauth.com/compare-hashing-algorithms/sha-256-vs-hmac-sha384)
[2] [https://crypto.stackexchange.com](https://crypto.stackexchange.com/questions/79522/why-did-the-nsa-create-sha)
[3] [https://en.wikipedia.org](https://en.wikipedia.org/wiki/Cryptographic_hash_function)
[4] [https://datatracker.ietf.org](https://datatracker.ietf.org/doc/html/rfc6234)
[5] [https://crypto.stackexchange.com](https://crypto.stackexchange.com/questions/93369/does-sha384-make-sense-with-hmac-sha256)
[6] [https://en.wikipedia.org](https://en.wikipedia.org/wiki/Secure_Hash_Algorithms)
[7] [https://publish.obsidian.md](https://publish.obsidian.md/addielamarr/SHA-2#:~:text=Overview.%20SHA%2D2%20%28Secure%20Hash%20Algorithm%202%29%20is,384%2C%20512%2C%20512/224%2C%20and%20512/256%20bits%20long.)
[8] [https://nvlpubs.nist.gov](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.202.pdf)
[9] [https://jaimelightfoot.com](https://jaimelightfoot.com/blog/sha/)
[10] [https://www.nccoe.nist.gov](https://www.nccoe.nist.gov/news-insights/cornerstone-cybersecurity-cryptographic-standards-and-50-year-evolution)
[11] [https://csrc.nist.gov](https://csrc.nist.gov/files/pubs/fips/180-2/final/docs/fips180-2.pdf)





## Argon2 and Post-Quantum Signatures## Argon2: The Password Specialist
Argon2 is not typically used for file signing; it is a [Memory-Hard Function (MHF)](https://en.wikipedia.org/wiki/Argon2) designed specifically for password hashing and key derivation. [1, 2] 

* The "Slow" Feature: Unlike SHA-256 or SHA-384, which are designed to be extremely fast for integrity checks, Argon2 is intentionally slow and memory-intensive. This makes it economically impossible for attackers to brute-force your passwords using GPUs or ASICs.
* Code Name: While it doesn't have a flashy project name like "Capstone," Argon2's variants serve as its descriptors: Argon2d (data-dependent for GPU resistance), Argon2i (independent for side-channel resistance), and the recommended hybrid Argon2id.
* Use in your Loader: For your secure config loader, stick to SHA-2 (256/384/512) for signing the files themselves. Only use Argon2 if your loader needs to derive a decryption key from a user-provided passphrase. [1, 2, 3, 4, 5, 6, 7] 

## Post-Quantum Cryptography (PQC) for File Signing [8] 
You can indeed use new PQC algorithms for signing files, and the Linux world is already moving in this direction for high-assurance environments like yours. [9, 10, 11] 

* Linux Kernel Support: In late 2025, work began on adding [ML-DSA (Dilithium)](https://www.phoronix.com/news/Dilithium-ML-DSA-Module-Sign) support to the Linux kernel for module signing. This is part of the move toward the [FIPS 204 standard](https://csrc.nist.gov/pubs/fips/204/final), which officially standardizes Dilithium as a post-quantum digital signature algorithm.
* Available Algorithms:
* ML-DSA (Dilithium): The primary general-purpose choice. It is fast and has relatively small signatures.
   * SLH-DSA (SPHINCS+): A "stateless" hash-based signature. It is larger and slower but relies on the extreme security of hash functions rather than lattice problems.
   * FN-DSA (Falcon): Known for having very small signatures, making it ideal for systems with limited storage space (like xattrs), though it is harder to implement securely. [9, 12, 13, 14, 15, 16] 

## Implementation Warning for your Loader
If you decide to use PQC for your Rust loader, be aware that PQC signatures are significantly larger than RSA or ECDSA. [11, 17, 18, 19, 20] 

* An RSA-2048 signature is 256 bytes.
* A Dilithium (ML-DSA) signature can be over 2,400 bytes.
* Constraint: Standard filesystem extended attributes (xattrs) often have a size limit (typically 4KB or less depending on the FS). If you use SPHINCS+, your signature might exceed the space available in security.ima. [17, 21, 22] 

Pro-tip: For your UMRS baseline, SHA-384 is already considered "quantum-safe" for hashing (as it would require a massive "Grover’s algorithm" attack to break). If you want to go full PQC for the signing part, Dilithium (ML-DSA) is your best bet for RHEL 10 compatibility. [11] 
Do you want to see how to use a Rust PQC library like pqcrypto-dilithium to integrate this into your loader?

[1] [https://en.wikipedia.org](https://en.wikipedia.org/wiki/Argon2)
[2] [https://www.onlinehashcrack.com](https://www.onlinehashcrack.com/guides/cryptography-algorithms/hashing-passwords-argon2-implementation-walkthrough.php)
[3] [https://security.stackexchange.com](https://security.stackexchange.com/questions/261902/argon2-vs-sha-512-whats-better-in-my-case)
[4] [https://mojoauth.com](https://mojoauth.com/compare-hashing-algorithms/sha-1-vs-argon2)
[5] [https://github.com](https://github.com/p-h-c/phc-winner-argon2)
[6] [https://mojoauth.com](https://mojoauth.com/compare-hashing-algorithms/sha-256-vs-argon2)
[7] [https://mojoauth.com](https://mojoauth.com/compare-hashing-algorithms/sha-512-vs-argon2)
[8] [https://www.scitepress.org](https://www.scitepress.org/Papers/2024/128392/128392.pdf#:~:text=The%20second%20approach%20is%20deploying%20more%20flexible,to%20de%2D%20ploy%20on%20general%20computing%20platforms.)
[9] [https://csrc.nist.gov](https://csrc.nist.gov/projects/pqc-dig-sig)
[10] [https://www.ssh.com](https://www.ssh.com/academy/nist-pqc-standards-explained-path-to-quantum-safe-encryption)
[11] [https://www.fortanix.com](https://www.fortanix.com/blog/which-post-quantum-cryptography-pqc-algorithm-should-i-use)
[12] [https://www.phoronix.com](https://www.phoronix.com/news/Dilithium-ML-DSA-Module-Sign)
[13] [https://csrc.nist.gov](https://csrc.nist.gov/projects/post-quantum-cryptography/faqs)
[14] [https://csrc.nist.gov](https://csrc.nist.gov/projects/post-quantum-cryptography/post-quantum-cryptography-standardization)
[15] [https://eprint.iacr.org](https://eprint.iacr.org/2025/2029.pdf)
[16] [https://zknox.eth.limo](https://zknox.eth.limo/posts/2025/03/21/ETHFALCON.html)
[17] [https://www.reddit.com](https://www.reddit.com/r/ethdev/comments/1mbz90s/has_anyone_tried_implementing_postquantum/#:~:text=The%20only%20Dilithium%20I%27ve%20ever%20worked%20with,ZK%20tech%20could%20change%20the%20game%20here.)
[18] [https://safecipher.co.uk](https://safecipher.co.uk/the-myth-of-quantum-ready-hsms/#:~:text=Performance%20Trade%2DOffs:%20PQC%20keys%20are%20significantly%20larger,impacting%20storage%2C%20transmission%2C%20and%20cryptographic%20processing%20speeds.)
[19] [https://qsnp.eu](https://qsnp.eu/quantum-randomness-reinforces-post-quantum-cryptography-to-safeguard-large-enterprises-in-the-quantum-safe-era/#:~:text=While%20PQC%20algorithms%20introduce%20increased%20bandwidth%20consumption%2C,of%20integrating%20QRNGs%20to%20supply%20high%2Dquality%20entropy.)
[20] [https://ietresearch.onlinelibrary.wiley.com](https://ietresearch.onlinelibrary.wiley.com/doi/10.1049/blc2.12094#:~:text=PQC%20key%20sizes%20are%20bigger%20than%20classical%20ECDSA%20key%20sizes.)
[21] [https://falcon-sign.info](https://falcon-sign.info/#:~:text=To%20give%20a%20point%20of%20comparison%2C%20Falcon%2D512,and%20public%20keys%20use%20256%20bytes%20each.)
[22] [https://www.spiedigitallibrary.org](https://www.spiedigitallibrary.org/conference-proceedings-of-spie/13451/134510C/Integrating-post-quantum-cryptography-PQC-with-quantum-key-distribution-QKD/10.1117/12.3055954.full#:~:text=TECHNICAL%20CHALLENGES%20IN%20PQC+QKD%20INTEGRATION%20FOR%20KEY,but%20incurs%20higher%20signature%20size%20%28~8%20KB%29.)




Creating a sidecar signature file (like mylog.log.sig) is a classic and highly effective way to add tamper-proofing without modifying the original log or PDF. Since you are on RHEL 10, you can use the ML-DSA (Dilithium) standard, which is now supported in modern crypto tools. [1, 2, 3] 
## Why Sidecars?
Sidecar signatures are ideal for your high-assurance UMRS because they:

* Keep Data Pure: You don't have to worry about PDF format corruption or log parsers breaking.
* Are Easy to Batch: You can sign an entire directory of logs at once, creating a matching .sig for each.
* Scale for PQC: Since ML-DSA signatures are large (~2.5 KB to 4.5 KB), keeping them in a separate file avoids bloating the primary data. [1, 4, 5] 

## Implementation with OpenSSL 3.5+ [6] 
Modern versions of OpenSSL (3.5 and later) include support for ML-DSA. You can generate keys and sign a sidecar file with these commands: [1, 6, 7] 

   1. Generate a Private Key:
   
   openssl genpkey -algorithm ML-DSA-65 -out umrs_private.pem
   
   2. Extract the Public Key (for the verifier):
   
   openssl pkey -in umrs_private.pem -pubout -out umrs_public.pem
   
   3. Create the Sidecar Signature:
   
   openssl pkeyutl -sign -inkey umrs_private.pem -in mylog.log -out mylog.log.sig
   
   4. Verify the Sidecar:
   
   openssl pkeyutl -verify -pubin -inkey umrs_public.pem -sigfile mylog.log.sig -in mylog.log
   
   
## Integrating into Your Rust Loader
Since you already have a Rust loader, you can use the openssl crate or a dedicated PQC crate (like pqcrypto-ml-dsa) to automate this. Your loader should:

   1. Look for a file with the same name + .sig.
   2. Read the signature into memory.
   3. Perform the verification using your "anchored" public key before allowing the program to process the log/PDF.

## Considerations for Your Baseline

* ML-DSA-65: This is the "middle ground" security level (comparable to AES-192) and is typically the standard choice for general-purpose signatures.
* Stateless Advantage: Unlike older PQC algorithms, ML-DSA is stateless, meaning you can use the same private key for millions of signatures without needing to "rotate" or update the key state. [1, 5, 8, 9] 

Would you like a Rust snippet that specifically handles this "look for .sig and verify" logic using a PQC library?
Proactive Follow-up: I can help you find the [official OpenSSL 3.5 documentation](https://docs.openssl.org/3.6/man7/EVP_SIGNATURE-ML-DSA/) for ML-DSA if you need it for your compliance documentation.

[1] [https://docs.openssl.org](https://docs.openssl.org/3.6/man7/EVP_SIGNATURE-ML-DSA/)
[2] [https://www.youtube.com](https://www.youtube.com/watch?v=EcGmQjl6XEo&t=1)
[3] [https://docs.keyfactor.com](https://docs.keyfactor.com/ejbca/latest/post-quantum-cryptography-keys-and-signatures)
[4] [https://pdfa.org](https://pdfa.org/pdf-industry-embraces-post-quantum-cryptography-to-secure-digital-signatures/)
[5] [https://www.akamai.com](https://www.akamai.com/blog/security/digital-signatures-traditional-post-quantum-cryptographic)
[6] [https://aws.amazon.com](https://aws.amazon.com/blogs/security/how-to-create-post-quantum-signatures-using-aws-kms-and-ml-dsa/)
[7] [https://aws.amazon.com](https://aws.amazon.com/blogs/security/how-to-create-post-quantum-signatures-using-aws-kms-and-ml-dsa/)
[8] [https://www.youtube.com](https://www.youtube.com/watch?v=Y68Vh3WY-vU&t=18)
[9] [https://www.youtube.com](https://www.youtube.com/watch?v=coHTUXbIzU0&t=31)








The "high-end" choice for PQC (Post-Quantum Cryptography) signing depends on whether you prioritize extreme security or operational performance. In the context of a secure configuration loader for a High-Assurance baseline, two primary candidates stand out: [1] 
## 1. The "Gold Standard" for Security: SLH-DSA (SPHINCS+) [2, 3] 
If your primary goal is the highest level of long-term assurance, SLH-DSA (FIPS 205) is the high-end choice. [4, 5] 

* Security Basis: It is a stateless hash-based signature. Unlike most other PQC algorithms that rely on relatively new "lattice" math, SLH-DSA is built entirely on the security of hash functions like SHA-256.
* Trust Level: It is often considered the most conservative and reliable PQC option because its security is much better understood. The lowest security level of SLH-DSA is sometimes more trusted by cryptographers than the highest levels of newer lattice-based schemes.
* Trade-off: The signatures are quite large (~41 KB for high-end levels), and it is slower to generate and verify signatures compared to its peers. [2, 4, 6, 7, 8] 

## 2. The "Performance" Leader: ML-DSA (Dilithium)
For a loader that needs to verify signatures quickly without causing significant boot-time or runtime delay, ML-DSA (FIPS 204) is the industry standard. [2, 4, 8] 

* Highest Configuration: Use ML-DSA-87 for the high-end security tier.
* Efficiency: It offers a much better balance of speed and signature size. Verification is extremely fast, which is critical for a "gatekeeper" component like your configuration loader.
* Signature Size: At high-end levels, signatures are roughly 2.5 KB to 4.5 KB—significantly smaller than SLH-DSA. [6, 8, 9, 10, 11, 12] 

## Comparison for Your High-Assurance Baseline

| Feature [6, 8, 10, 11, 12, 13] | SLH-DSA-256 (High-End Hash) | ML-DSA-87 (High-End Lattice) |
|---|---|---|
| NIST Security Level | Level 5 (Highest) | Level 5 (Highest) |
| Security Assumption | Hash function collision resistance | Learning with Errors (Lattices) |
| Verification Speed | Slower | Very Fast |
| Signature Size | ~41-49 KB | ~4.5 KB |
| Best For | Archival logs / Extreme assurance | Real-time loading / Config validation |

Recommendation for your Rust Loader:
For a configuration loader, ML-DSA-87 is typically the high-end choice because it ensures high security without making your software feel sluggish during the integrity check. If you are signing logs for audit purposes where they might sit for 30+ years, SLH-DSA is the safer long-term bet. [6, 14] 
Would you like to see how the Rust pqcrypto crates handle the difference in signature sizes for these two high-end options?
Proactive Follow-up: I can help you find the [official NIST FIPS 204](https://csrc.nist.gov/pubs/fips/204/final) or [FIPS 205](https://csrc.nist.gov/pubs/fips/205/final) documentation if you need them for your UMRS compliance documentation.

[1] [https://www.youtube.com](https://www.youtube.com/watch?v=P9g1CMCu8DI&t=19)
[2] [https://www.nist.gov](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards)
[3] [https://csrc.nist.gov](https://csrc.nist.gov/projects/pqc-dig-sig)
[4] [https://www.nist.gov](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards)
[5] [https://www.nist.gov](https://www.nist.gov/news-events/news/2024/08/nist-releases-first-3-finalized-post-quantum-encryption-standards)
[6] [https://blog.cloudflare.com](https://blog.cloudflare.com/another-look-at-pq-signatures/)
[7] [https://mikail-eliyah.medium.com](https://mikail-eliyah.medium.com/notes-on-post-quantum-cryptography-methods-7b81e45a332f#:~:text=SLH%2DDSA%20%28Hash%2Dbased%20Signature%2C%20e.g.%2C%20SPHINCS+%29%20is%20a,lattice%20methods%2C%20such%20as%20ML%2DDSA%20and%20FN%2DDSA.)
[8] [https://blog.cloudflare.com](https://blog.cloudflare.com/another-look-at-pq-signatures/)
[9] [https://www.paloaltonetworks.com](https://www.paloaltonetworks.com/cyberpedia/pqc-standards)
[10] [https://www.bertendsp.com](https://www.bertendsp.com/products/mlds-b235/#:~:text=Derived%20from%20the%20CRYSTALS%2DDilithium%20scheme%2C%20ML%2DDSA%20is,of%20the%20NIST%20standard%20%28ML%E2%80%91DSA%E2%80%9144%2C%20ML%E2%80%91DSA%2D65%2C%20ML%2DDSA%2D87%29.)
[11] [https://www.digicert.com](https://www.digicert.com/insights/post-quantum-cryptography/dilithium)
[12] [https://www.youtube.com](https://www.youtube.com/watch?v=H6aJo3ugmys&t=30)
[13] [https://www.kaspersky.com.au](https://www.kaspersky.com.au/blog/post-quantum-cryptography-standards/34045/)
[14] [https://engineering.fb.com](https://engineering.fb.com/2026/04/16/security/post-quantum-cryptography-migration-at-meta-framework-lessons-and-takeaways/#:~:text=Research%20indicates%20that%20quantum%20computers%20will%20eventually,if%20quantum%20computers%20are%20still%20years%20away.)






For high-end PQC signing on RHEL 10 using [OpenSSL 3.5](https://openssl-library.org/post/2025-04-08-openssl-35-final-release/) or newer, you can generate sidecar signatures for your logs and PDFs using ML-DSA-87 (for speed) or SLH-DSA-SHA2-256s (for long-term security). [1, 2] 
## 1. High-End Lattice-Based: ML-DSA-87
ML-DSA-87 provides NIST Security Level 5 (the highest) and is preferred for its fast verification speed, which is ideal for a secure loader. [3, 4] 

* Generate Key Pair:

openssl genpkey -algorithm ML-DSA-87 -out umrs_mldsa_priv.pem
openssl pkey -in umrs_mldsa_priv.pem -pubout -out umrs_mldsa_pub.pem

* Create Sidecar Signature:

openssl pkeyutl -sign -inkey umrs_mldsa_priv.pem -in mylog.log -out mylog.log.sig

* Verify Sidecar Signature:

openssl pkeyutl -verify -pubin -inkey umrs_mldsa_pub.pem -sigfile mylog.log.sig -in mylog.log


## 2. High-End Hash-Based: SLH-DSA-SHA2-256s [5, 6] 
For the most conservative "high-end" assurance, SLH-DSA-SHA2-256s is used. The "s" variant indicates "small" signatures (~30 KB), though they are still much larger than ML-DSA. [7, 8, 9] 

* Generate Key Pair:

openssl genpkey -algorithm SLH-DSA-SHA2-256s -out umrs_slhdsa_priv.pem
openssl pkey -in umrs_slhdsa_priv.pem -pubout -out umrs_slhdsa_pub.pem

* Create Sidecar Signature:

openssl pkeyutl -sign -inkey umrs_slhdsa_priv.pem -in mylog.log -out mylog.log.sig

* Verify Sidecar Signature:

openssl pkeyutl -verify -pubin -inkey umrs_slhdsa_pub.pem -sigfile mylog.log.sig -in mylog.log


## Key Considerations for Your Baseline

* Algorithm Choice: Use ML-DSA-87 for real-time verification in your Rust loader to keep the system responsive. Use SLH-DSA for archival audit logs that must remain secure for 30+ years.
* OpenSSL Version: Ensure your RHEL 10 environment is fully updated; these PQC algorithms were integrated natively starting with OpenSSL 3.5.
* Signature Size: Be aware that [ML-DSA-87 signatures](https://docs.openssl.org/3.6/man7/EVP_SIGNATURE-ML-DSA/) are roughly 4.5 KB, which fits comfortably in a sidecar file but may exceed standard xattr limits if you try to use them as security.ima labels. [1, 2, 4, 10, 11] 

Would you like a bash script to automate the batch signing of an entire directory of logs into sidecar files?

[1] [https://openssl-corporation.org](https://openssl-corporation.org/post/2025-04-08-openssl-35-final-release/?utm_source=atom_feed)
[2] [https://docs.openssl.org](https://docs.openssl.org/3.6/man7/EVP_SIGNATURE-ML-DSA/)
[3] [https://docs.openssl.org](https://docs.openssl.org/3.5/man7/EVP_PKEY-ML-DSA/)
[4] [https://docs.openssl.org](https://docs.openssl.org/3.6/man7/EVP_SIGNATURE-ML-DSA/)
[5] [https://docs.openssl.org](https://docs.openssl.org/3.1/man1/openssl-dgst/#:~:text=SYNOPSIS.%20openssl%20dgst%7Cdigest%20%5B%2Ddigest%5D%20%5B%2Dlist%5D%20%5B%2Dhelp%5D%20%5B%2Dc%5D,%5B%2Dprovider%20name%5D%20%5B%2Dprovider%2Dpath%20path%5D%20%5B%2Dpropquery%20propq%5D%20%5Bfile)
[6] [https://asecuritysite.com](https://asecuritysite.com/openssl/mldsa_cert)
[7] [https://docs.openssl.org](https://docs.openssl.org/3.6/man7/EVP_SIGNATURE-SLH-DSA/)
[8] [https://docs.openssl.org](https://docs.openssl.org/3.5/man7/EVP_PKEY-SLH-DSA/)
[9] [https://github.com](https://github.com/openssl/openssl/blob/master/doc/designs/slh-dsa.md)
[10] [https://www.cryptomathic.com](https://www.cryptomathic.com/blog/quantum-ready-cryptography-with-openssl-3.5-on-rhel-9.6)
[11] [https://medium.com](https://medium.com/asecuritysite-when-bob-met-alice/a-non-lattice-pqc-signature-method-sphincs-and-openssl-3-5-a9dab260b814)





