# Audit Logging Research Collection

Acquired: 2026-04-02
Acquired by: The Librarian (researcher agent)
Purpose: Reference corpus for UMRS audit logging architecture — structured event formats,
ontologies, and telecommunications alarm standards relevant to high-assurance audit trails,
IDMEF integration, and forensic data interchange.

---

## Files

### cc-part2-rev1-2022.pdf

| Field | Value |
|---|---|
| Title | Common Criteria for Information Technology Security Evaluation — Part 2: Security Functional Components, Revision 1 (2022) |
| Source URL | https://www.commoncriteriaportal.org/files/ccfiles/CC2022PART2R1.pdf |
| Download date | 2026-04-02 |
| SHA-256 | `16c853ad1ccf6eb10120726b15fe9ecfa95ac381f85d8edb84c12b0d0ecf1b01` |
| File size | 3.1 MB |
| Status | Downloaded — verified %PDF magic bytes |

Purpose: CC Part 2 defines Security Functional Components (SFCs) including FAU (Security Audit)
class requirements. Directly applicable to UMRS audit event generation, audit trail protection,
audit review, and audit analysis requirements. The FAU_GEN, FAU_SAR, FAU_STG, and FAU_ANA
families define the formal functional requirements that UMRS audit logging must satisfy for
Common Criteria evaluations.

---

### 3gpp-ts28111-v18.1.0.pdf

| Field | Value |
|---|---|
| Title | 3GPP TS 28.111 v18.1.0 — Telecommunication Management; Fault Management; Part 1: 3G and LTE |
| Source URL | https://www.etsi.org/deliver/etsi_ts/128100_128199/128111/18.01.00_60/ts_128111v180100p.pdf |
| Download date | 2026-04-02 |
| SHA-256 | `cfeb041a87a5cb1dbdffcff83141e5835be6170255b186511349f70f6b88d4ef` |
| File size | 266 KB |
| Status | Downloaded — verified %PDF magic bytes |

Purpose: Defines the 3GPP Fault Management model including alarm information models,
alarm correlation, and structured alarm notification formats. Relevant to UMRS posture
signal taxonomy design — the alarm severity levels (critical, major, minor, warning,
indeterminate, cleared) and alarm attribute structures inform how UMRS should categorize
and correlate security posture findings.

---

### rfc4765-idmef.txt

| Field | Value |
|---|---|
| Title | RFC 4765 — The Intrusion Detection Message Exchange Format (IDMEF) |
| Source URL | https://www.rfc-editor.org/rfc/rfc4765.txt |
| Download date | 2026-04-02 |
| SHA-256 | `756b544dda4ab5ab29f5cf0001a880477bbdd207305c8339be94f7d21bc292f2` |
| File size | 301 KB (8795 lines — includes embedded XML schema definitions) |
| Status | Downloaded — verified RFC header (Network Working Group, H. Debar, France Telecom) |

Purpose: IDMEF defines a standard XML data model for representing intrusion detection
events and alerts for exchange between IDS components. The IDMEF Alert, Heartbeat,
and Analyzer models inform UMRS structured security finding representation. Directly
relevant to the Security Findings as Data rule — IDMEF's machine-processable alert
taxonomy provides a reference for designing UMRS finding enum variants that are
queryable, filterable, and countable rather than free-form log strings.

---

### cef-implementation-standard.pdf

| Field | Value |
|---|---|
| Title | Common Event Format (CEF) Implementation Standard — ArcSight SmartConnectors 8.4 |
| Source URL | https://www.microfocus.com/documentation/arcsight/arcsight-smartconnectors-8.4/pdfdoc/cef-implementation-standard/cef-implementation-standard.pdf |
| Download date | 2026-04-02 |
| SHA-256 | `974fd57dcdf5d3723b4d53e5cd3d7d92f98b5f680070eaac3efbae2d7cda1e12` |
| File size | 277 KB |
| Status | Downloaded — verified %PDF magic bytes |

Purpose: CEF is the de facto SIEM event interchange format widely deployed in government
and enterprise security operations. The CEF header fields (deviceVendor, deviceProduct,
deviceVersion, signatureId, name, severity) and extension vocabulary define the canonical
wire format UMRS audit events should be compatible with when forwarding to SIEM systems.
Relevant to UMRS AU-3 (Content of Audit Records) compliance and SIEM integration design.

---

### dfxml-2012.pdf

| Field | Value |
|---|---|
| Title | Digital Forensics XML and the DFXML Toolset (Garfinkel, 2012) |
| Source URL | https://simson.net/clips/academic/2012.DI.dfxml.pdf |
| Download date | 2026-04-02 |
| SHA-256 | `793eb17173e73420141551bbdad0777573dc71aa1f72f74f16f2af79c648b8ef` |
| File size | 1.3 MB |
| Status | Downloaded — verified %PDF magic bytes |

Purpose: DFXML defines an XML schema for representing forensic metadata about files,
volumes, disk images, and processes — including hashes, timestamps, provenance chains,
and acquisition metadata. Directly relevant to UMRS vault pipeline design and the
C2PA chain-of-custody model. The DFXML FileObject and VolumeObject structures provide
a reference for representing file-level evidence in UMRS audit records and for
structuring digital evidence packaging in the ingest pipeline.

---

### case-ontology-intro.html

| Field | Value |
|---|---|
| Title | CASE Ontology — Introduction (caseontology.org) |
| Source URL | https://www.caseontology.org/ontology/intro.html |
| Download date | 2026-04-02 |
| SHA-256 | `aa9c6ba4e29ea3d8e3c82e6a7b575299d6bb475994b9fca8a57e1b6a60ee9da1` |
| File size | 19 KB |
| Status | Downloaded — verified HTML content (DOCTYPE html, real page content) |

Purpose: CASE (Cyber-investigation Analysis Standard Expression) is a community-developed
ontology for representing information commonly used in cyber investigations, based on the
Unified Cyber Ontology (UCO). CASE provides semantic interoperability for tool output,
evidence packaging, and chain-of-custody representation. Relevant to UMRS vault pipeline
provenance design and to the SurrealDB custody graphing research — CASE's Provenance
Records and Investigation objects map directly to UMRS chain-of-custody concepts.

---

### itu-x721.pdf

| Field | Value |
|---|---|
| Title | ITU-T Recommendation X.721 (02/92) — Information Technology: Open Systems Interconnection; Structure of Management Information: Definition of Management Information |
| Source URL | https://www.itu.int/rec/dologin_pub.asp?id=T-REC-X.721-199202-I!!PDF-E&lang=e&type=items |
| Download date | 2026-04-02 |
| SHA-256 | `89ce0ae57d0b8227f10766479d50137a2d07bd62ae01bf51698d536141f407c2` |
| File size | 258 KB (67 pages) |
| Status | Downloaded — verified %PDF magic bytes; content confirmed (strings extraction shows X.721 managed object class titles, CCITT Rec. X.721 metadata, X.721 keywords) |

Note: The ITU download URL uses a `dologin_pub.asp` redirect that normally indicates a
login wall. In this case the redirect delivered the actual document without a session.
Content was confirmed via strings extraction — the PDF contains the actual X.721 managed
object class definitions (FAU, managed object classes, name bindings, Annex B index).
If this URL stops working in future sessions, the alternative retrieval path is:
  1. Visit https://www.itu.int/rec/T-REC-X.721/en
  2. Select the 02/1992 edition
  3. Download as "PDF (free)"
The document may also be available via the ITU-T open standards portal for
pre-1995 recommendations.

Purpose: X.721 defines the Structure of Management Information (SMI) for OSI network
management — specifically the GDMO (Guidelines for the Definition of Managed Objects)
framework. Managed object classes, name bindings, and attribute definitions from X.721
underpin the ITU-T/ISO alarm model used in 3GPP TS 28.111 and the SNMP/MIB tradition.
Understanding X.721 provides the conceptual foundation for the 3GPP fault management
model and informs how UMRS should represent managed security objects in its posture catalog.

---

## Retrieval Notes

All seven documents were downloaded successfully on 2026-04-02. No manual download
intervention was required. The ITU X.721 dologin URL, which typically requires a
registered session, delivered the actual document without authentication — this behavior
may not be reproducible in all network environments or future sessions.

The RFC 4765 file is 301 KB because it includes embedded XML Schema Definition (XSD)
content describing the IDMEF data model — this is expected and correct.
