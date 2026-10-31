# 0022. Rename Legality Rules to Static Semantics

Date: 2026-10-30
Status: Accepted. Amends 0016.

## Context

Decision 0016 took the segment names of the Ferrocene Language
Specification. Its `Legality Rules` segment holds the rules that an
implementation checks before the program runs. The name reads poorly in
Tidal, and the segment for run-time rules is already `Dynamic Semantics`.

## Decision

The segment is `Static Semantics`. It holds the same rules: `required`
rules checked before run time, and `reject` rules. All pages, the style
guide, and `tools/check_spec.py` use the new name.

## Consequences

- A page reads Syntax, Static Semantics, Dynamic Semantics, then the
  other segments, so the two kinds of rules have parallel names.
- Rule IDs and rule classes do not change.
