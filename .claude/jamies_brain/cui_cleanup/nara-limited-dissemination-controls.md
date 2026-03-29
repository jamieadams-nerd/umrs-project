# NARA CUI Limited Dissemination Controls

Source: https://www.archives.gov/cui/registry/limited-dissemination

## Controls

| Limited Dissemination Control | Banner Marking | Portion Marking | Description |
|---|---|---|---|
| No foreign dissemination | NOFORN | NF | Information may not be disseminated in any form to foreign governments, foreign nationals, foreign or international organizations, or non-US citizens. |
| Federal employees only | FED ONLY | FED ONLY | Dissemination authorized only to (1) employees of United States Government executive branch departments and agencies (as the agency is defined in 5 U.S.C. 105), or (2) armed forces personnel of the United States or Active Guard and Reserve (as defined in 10 USC 101). |
| Federal employees and contractors only | FEDCON | FEDCON | Dissemination authorized only to (1) employees of United States Government executive branch departments and agencies (as the agency is defined in 5 U.S.C. 105), (2) armed forces personnel of the United States or Active Guard and Reserve (as defined in 10 USC 101), or (3) individuals or employers who enter into a contract with the United States (any department or agency) to perform a specific job, supply labor and materials, or for the sale of products and services, so long as dissemination is in furtherance of that contractual purpose. |
| No dissemination to contractors | NOCON | NOCON | No dissemination authorized to individuals or employers who enter into a contract with the United States (any department or agency) to perform a specific job, supply labor and materials, or for the sale of products and services. Note: This dissemination control is intended for use when dissemination is not permitted to Federal contractors, but permits dissemination to state, local, or tribal employees. |
| Dissemination list controlled | DL ONLY | DL ONLY | Dissemination authorized only to those individuals, organizations, or entities included on an accompanying dissemination list. Note: Use of this limited dissemination control supersedes other limited dissemination controls, but cannot supersede dissemination stipulated in law, Federal regulation, or Government-wide policy. |
| Releasable by information disclosure official | RELIDO | RELIDO | A permissive foreign disclosure and release marking used on information to indicate that the originator has authorized a Senior Foreign Disclosure and Release Authority (SFDRA) to make further sharing decisions for uncaveated intelligence material in accordance with existing procedures, guidelines, and implementation guidance. Note: Only agencies eligible to use RELIDO in the IC classified information context may use this LDCM on CUI. |
| Authorized for release to certain nationals only | REL TO [USA, LIST] | REL TO [USA, LIST] | Information has been predetermined by the designating agency to be releasable or has been released only to the specific foreign country(ies)/international organization(s) indicated, through established foreign disclosure procedures and channels. Note: USA must always appear first, followed by trigraph country codes in alphabetical order, followed by tetragraph codes for coalition or international organizations in alphabetical order. |
| Display only | DISPLAY ONLY [USA, LIST] | DISPLAY ONLY [USA, LIST] | Information is authorized for disclosure to a foreign recipient but without providing the foreign recipient with a physical copy for retention (regardless of medium), to the foreign country(ies)/international organization(s) indicated. Note: USA must always appear first, followed by trigraph country codes in alphabetical order, followed by tetragraph codes for coalition or international organizations in alphabetical order. |
| Attorney-client | Attorney-Client | AC | Dissemination of information protected by the attorney-client privilege beyond the attorney, the attorney's agents, or the client can result in the loss of the privilege and is prohibited by this marking unless the agency's executive decision-makers decide to disclose the information outside the bounds of its protection. Note: For use only with the "Legal Privilege" category. |
| Attorney work product | Attorney-WP | AWP | Dissemination of information protected by the attorney work product privilege beyond the attorney, the attorney's agents, or the client can result in the loss of the privilege and is prohibited by this marking unless specifically permitted by the overseeing attorney who originated the work product or their successor. Note: For use only with the "Legal Privilege" category. |

## Notes

- LDCs are suffixed to the CUI banner marking (e.g., `CUI//EXPT//NOFORN`)
- Each LDC will have its own MCS category number (`c` number) in setrans.conf
- Attorney-Client and Attorney-WP are restricted to the "Legal Privilege" category only
- REL TO and DISPLAY ONLY require parameterized country/org lists
- In the JSON structure, LDCs will be a peer section to `markings` under the US section
- Distribution Statements (A–F) are a separate concern — details TBD
