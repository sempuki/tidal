# Q-0404: Does a friend declaration that names a member through a pattern such as `A<T*>::f` befriend that member of a specialization that does not match the pattern?

**Status:** provisional
**Affects:** ACC-20, `tests/classes/access/friend-member-pattern-other-specialization.cpp`, `tests/classes/access/friend-member-of-partial-specialization.cpp`
**Source:** the gap review `reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md`, section ACC-20, from the probes that a clean-room writer ran for it.
**Waits for:** the maintainer (pending.md row 4)

## Context

Class `C` declares `template <class T> friend int A<T*>::f();`. The
class template `A` has a partial specialization for pointers. The
member `f` of `A<int*>` matches the qualifier, and the member `f` of
`A<int>` does not. Both read a private member of `C`.

| Implementation | `A<int*>::f` | `A<int>::f` |
|---|---|---|
| Clang 22 | friend | friend |
| GCC 16 | friend | friend |
| EDG 7.0 | friend | not a friend |

The gap review reads the qualifier as a pattern: only the member of a
specialization that matches it is a friend, as EDG does.

## Evidence

The tests in **Affects**. The test of `A<int>` has an override for EDG.

## Options

1. The member of every specialization is a friend (Clang, GCC).
2. Only the member of a specialization that matches the pattern is a
   friend (EDG, the gap review).

## Provisional answer

Every specialization (option 1). Two of three families agree, and
`pending.md` row 4 keeps the matrix's answer where at most one newest
family follows the review.

## Opinions

### Gap review, 2026-11-05 (phase 10 text)

The source-reader gap review of the text phase 10 wrote (`reports/review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md`) names the question in its section on ACC-20. The reference reads against the answer that Tidal keeps: the member of a specialization outside the qualifier's pattern is not a friend. The question stays in `pending.md` row 4.
