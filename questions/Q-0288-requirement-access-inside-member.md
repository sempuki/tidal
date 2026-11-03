# Q-0288: Can a requires expression inside a member function use a private member of the class?

**Status:** settled
**Affects:** CPT-26, CNS-10, `tests/templates/concepts/requirement-access-member.cpp`, `tests/templates/concepts/requirement-access-concept-kept.cpp`, `tests/templates/concepts/requirement-access-concept-kept-false.cpp`

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

When the same concept-id is checked inside the member and again at
namespace scope, every compiler keeps the result of the first check, in
either order, and no compiler diagnoses the change
(`requirement-access-concept-kept.cpp`,
`requirement-access-concept-kept-false.cpp`). The result would change
between the two places, so the program is not valid, and CNS-10
requires no diagnostic.

## Options

1. Access is checked from the place of the check, so the member can use
   the private member (Clang, GCC 16, EDG).
2. The requirement is false (GCC 14).

## Answer

Yes; every current compiler agrees, GCC 14 differs (option 1). Every current implementation agrees.

Settled on 2026-11-01 under decision 0051, because the newest version of each baseline family (Clang 22, GCC 16, EDG 7.0) agrees with this answer. GCC 14 still gives `false`, and the test keeps its override for GCC 14.

## Opinions

<!-- Add a subsection per opinion: ### @handle, YYYY-MM-DD -->
