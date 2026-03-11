To validate the CategorySet and enforce Information Flow Control (MLS), the Reference Monitor utilizes the Lattice Theory of Power Sets. In this model, security categories are treated as a finite set of distinct elements, and the validation relies on the Sub-lattice Dominance Algorithm.
I. Formal Set Definition
Let 
 be the universal set of all possible security categories (e.g., 
).
The total number of available categories is 
.
A specific file or process label contains a CategorySet 
, which is a subset of the power set of 
:

II. The Validation Invariant (TPI Gate)
When the Two-Path Integrity (TPI) gate ingests a label string (e.g., "c1,c2"), it must ensure the mapping to the bitmask is bijective and consistent.
Let 
 be the Declarative Path (nom parser) and 
 be the Imperative Path (split/join).
The algorithm validates the input string 
 if and only if:

Where 
 is the resulting set of indices 
 such that each 
 and 
.
III. The 
 Dominance Algorithm
To enforce the Bell-LaPadula "No Read Up" policy, the monitor must determine if a subject (user) set 
 dominates an object (file) set 
.
In set notation, dominance is defined as the Superset Relation:

1. The Mathematical Identity
Using bitwise logic (where each bit represents an element of the set), the subset relation is validated using the Intersection Invariant:

2. The Machine Implementation
Because we use a 1024-bit bitmask (a [u64; 16] array), the comparison is performed as a constant-time bitwise AND operation across the array blocks:

If this condition holds for all 
, then 
 dominates 
, and access is granted.
IV. Visualizing the Lattice 
The CategorySets form a Completely Distributive Lattice:
The Join (
): The least upper bound of two sets (the union of their categories).
The Meet (
): The greatest lower bound (the categories they have in common).
The Top (
): The set containing all 1024 categories (SystemHigh).
The Bottom (
): The empty set (SystemLow).
Why this is High-Assurance:
Deterministic: By using the 
 relation on bitmasks, we avoid string parsing during access decisions.
Order-Agnostic: Since 
 is a mathematical set, 
.
Non-Bypassable: The bitmask is the Ground Truth used by the kernel; our 
 check is a mirrored hardware-level verification.
UI Implementation Note
In your Zsh cleanup, you can use the Tricolon (⁝) or the Nabla (
) to represent the dominance check in your audit logs:
jadams {c1,c2} ⁝ Dominates ⁝ TESTFILE {c1} ? 
 True
Shall we implement the dominates() function in your CategorySet struct next to lock in this math?
Status: Mathematical Lattice Formalized.
Logic: 
 (Superset Dominance).
Next: 
 Comparison in Rust.




