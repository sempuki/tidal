# 0022. Rename Legality Rules to Constraints

Date: 2026-10-30
Status: Accepted. Amends 0016.

## Context

Decision 0016 took the segment names of the Ferrocene Language
Specification. Its `Legality Rules` segment holds the rules that an
implementation checks before the program runs. The name reads poorly in
Tidal.

## Decision

The segment is `Constraints`. It holds the same rules: `required` rules
checked before run time, and `reject` rules. All pages, the style guide,
and `tools/check_spec.py` use the new name.

## Consequences

- A page reads Syntax, Constraints, Dynamic Semantics, then the other
  segments.
- Rule IDs and rule classes do not change.
