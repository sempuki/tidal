# 0042. Overlapping reject rules

Date: 2026-10-31
Status: Accepted. Extends 0038.

## Context

Some programs break two `reject` rules at once: a general one and a
specific one. Examples:

| General | Specific | One program that breaks both |
|---|---|---|
| CAST-19 | CAST-2 | a `static_cast` between unrelated pointer types |
| CPY-24 | CPY-21, JMP-15 | an lvalue copy of a class whose move constructor deletes its copy |
| CE-25 | DCL-19, SW-3 | a non-constant array bound or case label |

Tests cite one of the two, and not always the same one. A checker that
names one rule then matches some tests and not others (front-end
finding G23).

## Decision

- A test cites the most specific `reject` rule that its program breaks.
  It can also cite the general rule.
- A general rule names the specific rules that it overlaps, so a reader
  and a checker can go from either one to the other.
- A checker that names the specific rule, or both, matches the test.

## Consequences

- The general rules in the table gain a sentence that names the
  specific ones, when their pages are next changed.
- `documentation/specification-format.md` states the convention with that of decision
  0038.
