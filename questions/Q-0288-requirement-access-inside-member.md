# Q-0288: Can a requires expression inside a member function use a private member of the class?

**Status:** provisional
**Affects:** CPT-26, `tests/templates/concepts/requirement-access-member.cpp`

## Context

A static member function of `Box` returns `requires (Box b) {
b.secret; }`, where `secret` is a private member of `Box`.

| Compiler | Result |
|---|---|
| Clang 18, 22, host | `true` |
| GCC 16, host | `true` |
| GCC 14 | `false` |
| EDG 7.0 | `true` |

## Evidence

The test above. Every compiler finds a concept-id false at namespace
scope when its requirement names the private member
(`requirement-access-concept.cpp`), and true when the concept-id is
first checked inside a member of the class
(`requirement-access-concept-in-member.cpp`). The gap review of
concepts and constraints (item 21) expects the requires expression in
the member to be true. The review also expects a concept to be checked
in its own context, but no compiler does so.

## Options

1. Access is checked from the place of the check, so the member can use
   the private member (Clang, GCC 16, EDG).
2. The requirement is false (GCC 14).

## Provisional answer

Option 1. Every current implementation agrees.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
