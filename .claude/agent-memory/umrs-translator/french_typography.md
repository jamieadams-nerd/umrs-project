---
name: French Typography Rules (Guillemets and Punctuation)
description: Jamie's canonical rules for French quotation marks and typography in UMRS translations — apply to all fr_CA .po output
type: feedback
---

French typography rules for UMRS translation output. Source: `.claude/jamies_brain/french_quotes.txt`.

**Why:** Jamie provided these rules explicitly so that translated strings produce native-quality
French technical text rather than generic machine translation. These are deterministic style
rules, not preferences.

**How to apply:** Apply to all `msgstr` values written into `fr_CA.po` files. Check every
string that contains quoted text, introducing terms, or literal program output before finalizing.

---

## Guillemets vs. English Quotes

Use « » (French guillemets) instead of English straight quotes " " in translated strings.

Correct:   « texte »
Incorrect: "texte"

## Mandatory Spacing Inside Guillemets

Spaces are required inside the guillemets.

Correct:   « texte »
Incorrect: «texte»

Preferred: non-breaking spaces (U+00A0) — « texte » — but normal spaces are acceptable
when non-breaking spaces cannot be produced.

## When to Use Guillemets

1. Direct quotations — quoting spoken or written text.
   Example: Il a déclaré : « Le système est sécurisé. »

2. Literal program output — exact error messages, UI messages, screen output.
   Example: Le système affiche « Permission denied ».

3. Introducing a new or foreign technical term — first use only.
   Example: Cette technique est appelée « fuzzing ».
   After first introduction, do not quote the term again.
   Correct later usage: Le fuzzing permet de détecter des erreurs.

4. Referring to a word as a word (meta-linguistic use).
   Example: Le terme « sandbox » est utilisé dans ce document.

## When NOT to Use Guillemets

- Do not quote a word simply because it is English.
  Incorrect: Le système utilise « Linux ».
  Correct:   Le système utilise Linux.

- Do not quote acronyms or proper names.
  Correct: OTAN, ONU, Linux, SELinux, Rust — no guillemets, ever.

- Do not use guillemets for commands, code, or file paths.
  Use monospace formatting (backticks in source; msgstr literal in .po context).
  Command:   Exécutez la commande `ls -l`.
  File path: Le fichier `/etc/passwd` contient les comptes utilisateurs.
  Code:      La fonction `open()` retourne un descripteur de fichier.

## Nested Quotations

Quotation inside a quotation: use double quotes " " inside the guillemets.
Example: Il a déclaré : « Le système affiche "Permission denied". »

## Summary Checklist (apply in order)

1. Replace all English " " with « » in translated text.
2. Insert spaces (preferably non-breaking) inside every « ».
3. Use guillemets for: quotations, literal program messages, first-use terms, word-as-word.
4. Do NOT quote: acronyms, proper names, common technical terms used normally.
5. Use monospace (backtick) for: commands, code identifiers, file paths.

## Future Scope (not yet specified by Jamie)

Jamie noted these additional French typography topics may be specified later:
- Non-breaking spaces before : ; ? !
- Capitalization rules for headings
- Treatment of English technical vocabulary beyond the guillemets rules
