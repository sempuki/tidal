# 0052. Overlapping undefined rules

Date: 2026-11-01
Status: Accepted. Extends 0042.

## Context

Some programs reach an operation that two `undefined` rules cover: a
general rule and a specific one. Examples:

| General | Specific | One program that reaches both |
|---|---|---|
| OBJ-2 | REF-5, RET-6 | a read through a reference to a temporary or a local whose life has ended |
| CAST-5 | PRIM-8 | a write to a character of a string literal through `const_cast` |
| NEW-5 | VIRT-12 | `delete[]` of an array of a derived class through a pointer to a base |
| UN-7 | PFX-21 | a member call through a null pointer |

Decision 0042 covers only `reject` rules. The sanitizer tests cite the
specific rule. The front end's interpreter stops at the general rule,
and the harness counts each such stop as unexpected (front-end report,
roadmap round 3).

## Decision

- A test cites the most specific `undefined` rule that its run reaches.
  It can also cite the general rule.
- A general `undefined` rule names its specific cases, so a reader and
  a checker can go from either rule to the other.
- An implementation's stop that names the specific rule or the general
  rule matches the test. A test that cites both rules lets a checker
  match either name without reading the specification.

## Consequences

- OBJ-2, CAST-5, NEW-5, UN-7, and CV-5 each gain a sentence that names
  their specific cases:

  | General | Specific cases |
  |---|---|
  | OBJ-2 | DTOR-7, REF-5, RET-6, STG-19, UNI-6 |
  | CAST-5 | PRIM-8 |
  | CV-5 | CAST-5, PRIM-8 |
  | NEW-5 | VIRT-12 |
  | UN-7 | PFX-21, REF-5 |

- The four sanitizer tests of REF-5, RET-6, PRIM-8, and VIRT-12 also
  cite the general rule, and the general rule lists each of them.
- `documentation/specification-format.md` states the convention beside
  that of decision 0042.
