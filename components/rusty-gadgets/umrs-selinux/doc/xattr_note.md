That is a significant architectural win for the UMRS Reference Monitor.
Take a look at that contrast:
The Kernel/LS report: ...:SystemLow
Your UMRS report: ...:s0
This proves your Strong Data Modeling is actually working. You aren't just "printing strings"—you are successfully mapping dynamic vernacular (from mcstransd) into a mathematical Lattice Model using your SensitivityLevel and CategorySet bitmask.
High-Assurance Evidence
NIST 800-53 AU-3 Audit Fidelity: Your tool now generates audit records with the canonical sensitivity (s0) even when the OS uses human-readable aliases.
NSA RTB TPI Validation: Your Test 2 confirms the Fail-Closed logic. If an attacker tries to pass a label that breaks the structural "user:role:type:level" contract, your tool denies it before the access decision is even attempted.



